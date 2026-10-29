use anyhow::{Context, Result, anyhow};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::config::{AnyKernelConfig, AnyKernelConfigMap, ProjectConfig, ProjectsMap};

pub fn get_root_dir() -> PathBuf {
    if let Ok(path) = env::var("CI_CENTRAL_ROOT") {
        return PathBuf::from(path);
    }

    if let Ok(current_dir) = env::current_dir() {
        for candidate in current_dir.ancestors() {
            if candidate.join("configs/projects.json").is_file() {
                return candidate.to_path_buf();
            }
        }
    }

    PathBuf::from(".")
}

pub fn get_config_path() -> PathBuf {
    get_root_dir().join("configs/projects.json")
}

pub fn get_anykernel_config_path() -> PathBuf {
    get_root_dir().join("configs/anykernel_configs.json")
}

pub fn get_upstream_path() -> PathBuf {
    get_root_dir().join("configs/upstream_commits.json")
}

pub fn get_workspace_dir() -> PathBuf {
    get_root_dir().join("kernel_workspace")
}

pub fn get_template_path(name: &str) -> PathBuf {
    get_root_dir().join("templates").join(name)
}

pub fn command_exists(command: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {command} >/dev/null 2>&1"))
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub fn env_flag(name: &str) -> bool {
    env::var(name)
        .map(|value| {
            matches!(
                value.to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

pub fn url_file_name(url: &str) -> Result<String> {
    url.rsplit('/')
        .next()
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(ToOwned::to_owned)
        .ok_or_else(|| anyhow!("Could not determine filename from URL: {}", url))
}

fn lower_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

pub fn cache_file_name(url: &str) -> Result<String> {
    let digest = Sha256::digest(url.as_bytes());
    Ok(format!(
        "{}-{}",
        lower_hex(digest.as_ref()),
        url_file_name(url)?
    ))
}

pub fn file_sha256(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 1024 * 1024];

    loop {
        let len = file.read(&mut buffer)?;
        if len == 0 {
            break;
        }
        hasher.update(&buffer[..len]);
    }

    let digest = hasher.finalize();
    Ok(lower_hex(digest.as_ref()))
}

pub fn load_projects() -> Result<ProjectsMap> {
    let path = get_config_path();
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read projects.json at {:?}", path))?;
    serde_json::from_str(&content).context("Failed to parse projects.json")
}

pub fn load_project(project_key: &str) -> Result<ProjectConfig> {
    let projects = load_projects()?;
    let proj_val = projects
        .get(project_key)
        .ok_or_else(|| anyhow!("Project not found: {}", project_key))?;
    serde_json::from_value(proj_val.clone()).context("Failed to parse project config")
}

pub fn load_anykernel_configs() -> Result<AnyKernelConfigMap> {
    let path = get_anykernel_config_path();
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read anykernel_configs.json at {:?}", path))?;
    serde_json::from_str(&content).context("Failed to parse anykernel_configs.json")
}

pub fn load_anykernel_config(config_key: &str) -> Result<AnyKernelConfig> {
    let configs = load_anykernel_configs()?;
    configs
        .get(config_key)
        .cloned()
        .ok_or_else(|| anyhow!("AnyKernel config not found: {}", config_key))
}

pub fn normalize_variant_name(variant: &str) -> String {
    match variant {
        "resukisu" | "sukisuultra" => "bakasu".to_string(),
        _ => variant.to_string(),
    }
}

pub fn variant_suffix(variant: &str) -> String {
    match normalize_variant_name(variant).as_str() {
        "main" | "lkm" => "LKM".to_string(),
        "bakasu" => "BakaSU".to_string(),
        _ => variant.to_uppercase(),
    }
}

pub fn is_bakasu_variant(variant: &str) -> bool {
    normalize_variant_name(variant) == "bakasu"
}

pub fn save_json<T: serde::Serialize>(path: &Path, data: &T) -> Result<()> {
    let content = serde_json::to_string_pretty(data)?;
    fs::write(path, content + "\n")?;
    Ok(())
}

fn write_github_kv(var_name: &str, key: &str, value: &str) -> Result<()> {
    if let Ok(path) = env::var(var_name) {
        let mut file = OpenOptions::new().append(true).create(true).open(path)?;
        writeln!(file, "{}={}", key, value)?;
    }
    Ok(())
}

pub fn set_github_env(key: &str, value: &str) -> Result<()> {
    write_github_kv("GITHUB_ENV", key, value)
}

pub fn set_github_output(key: &str, value: &str) -> Result<()> {
    write_github_kv("GITHUB_OUTPUT", key, value)
}

pub fn run_cmd(cmd: &[&str], cwd: Option<&Path>, capture: bool) -> Result<Option<String>> {
    let mut command = Command::new(cmd[0]);
    command.args(&cmd[1..]);

    if let Some(dir) = cwd {
        command.current_dir(dir);
    }

    if capture {
        let output = command.output()?;
        if !output.status.success() {
            return Err(anyhow!(
                "Command failed: {:?} Stderr: {}",
                cmd,
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Ok(Some(
            String::from_utf8_lossy(&output.stdout).trim().to_string(),
        ))
    } else {
        let status = command.status()?;
        if !status.success() {
            return Err(anyhow!("Command failed: {:?}", cmd));
        }
        Ok(None)
    }
}

pub fn run_cmd_with_env(
    cmd: &[&str],
    cwd: Option<&Path>,
    envs: &HashMap<String, String>,
) -> Result<()> {
    let mut command = Command::new(cmd[0]);
    command.args(&cmd[1..]);

    if let Some(dir) = cwd {
        command.current_dir(dir);
    }

    command.envs(envs);

    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());

    let status = command.status()?;
    if !status.success() {
        return Err(anyhow!("Command failed: {:?}", cmd));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_path(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        env::temp_dir().join(format!("{name}-{unique}"))
    }

    #[test]
    fn cache_file_name_is_stable_and_keeps_filename() {
        let name = cache_file_name("https://example.com/toolchain.tar.gz").unwrap();
        assert_eq!(
            name,
            "74caec161bcb4f71d8eb363cf7709f7e2132a464e2ef488b473eec8f0ce43249-toolchain.tar.gz"
        );
        assert_eq!(
            name,
            cache_file_name("https://example.com/toolchain.tar.gz").unwrap()
        );
    }

    #[test]
    fn file_sha256_hashes_file_content() {
        let path = unique_temp_path("kokuban-sha-test");
        fs::write(&path, b"abc").unwrap();
        assert_eq!(
            file_sha256(&path).unwrap(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn bakasu_and_legacy_variants_share_release_identity() {
        for variant in ["bakasu", "resukisu", "sukisuultra"] {
            assert_eq!(normalize_variant_name(variant), "bakasu");
            assert_eq!(variant_suffix(variant), "BakaSU");
            assert!(is_bakasu_variant(variant));
        }
        assert_eq!(variant_suffix("main"), "LKM");
        assert_eq!(variant_suffix("lkm"), "LKM");
        assert!(!is_bakasu_variant("main"));
        assert_eq!(
            normalize_variant_name("feature-resukisu"),
            "feature-resukisu"
        );
    }
}
