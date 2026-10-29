<!-- BEGIN-ZH -->
# __DEVICE_NAME_CN__内核

这是一个面向 **__DEVICE_NAME_CN__** 的自定义内核项目，重点围绕稳定性、性能调优与日常可用性进行构建。项目当前维护 `LKM` 与 `BakaSU` 两种发行模式，以满足不同用户对纯净环境和高级功能的需求。

## 项目概览

* **性能优化**：提供面向日常使用与游戏场景的调度与性能调优。
* **精简发行模式**：围绕 `LKM` 与 `BakaSU` 两种模式持续维护，减少历史分支带来的维护复杂度。
* **扩展特性支持**：可按构建配置集成 `SuSFS` 与 `BBG`。
* **版本标识**：`__LOCALVERSION_BASE__`

## 发行版本说明

* **LKM (Loadable Kernel Module)**
  * 不内置 Root 方案，适合希望保持内核环境尽可能精简的使用方式。
  * 如需 Root，需要通过 KernelSU Manager App 手动修补并刷入设备的 `init_boot` 镜像。

* **BakaSU**
  * 集成 BakaSU，适合有模块扩展、隐藏能力或进阶调试需求的用户。

## 安装指南

1. **解锁 Bootloader**：请确保设备已完成 Bootloader 解锁。
2. **准备 Recovery**：建议使用较新的 `TWRP` 或 `OrangeFox Recovery`。
3. **刷入内核**：从本项目 [Releases 页面](https://github.com/__PROJECT_REPO__/releases) 下载对应版本，并在 Recovery 中刷入内核 `zip` 包。
4. **仅 LKM 版本需要：修补 `init_boot`**
   使用前请先备份当前设备的 `init_boot.img`，再通过 KernelSU Manager App 选择并修补该镜像，随后使用 Fastboot 或 Recovery 将修补后的镜像刷入 `init_boot` 分区。
5. **重启设备**：完成刷入后重启系统并确认功能状态。

## 下载

所有最新构建版本均发布于 [**Releases 页面**](https://github.com/__PROJECT_REPO__/releases)。

## 反馈与支持

请通过本仓库的 [Issues](https://github.com/__PROJECT_REPO__/issues) 反馈问题。

## 来源与致谢

本项目基于以下开源项目构建：

* [BakaSU](https://github.com/Baka-SU/BakaSU) — 内核 Root 方案
* [YuzakiKokuban 内核项目](https://github.com/YuzakiKokuban/android_kernel_xiaomi_sm8850) — 设备内核与构建参考
* [KernelSU](https://github.com/tiann/KernelSU)

## 免责声明

刷机有风险，操作需谨慎。在进行任何操作前，请务必完整备份您的个人数据。因刷入此内核而导致的任何设备损坏或数据丢失，本人概不负责。
<!-- END-ZH -->

<!-- BEGIN-EN -->
# __DEVICE_NAME_EN__ Kernel

This is a custom kernel project for the **__DEVICE_NAME_EN__**, built with a focus on stability, performance tuning, and day-to-day usability. The project currently maintains two release tracks, `LKM` and `BakaSU`, to serve both clean setups and advanced power-user workflows.

## Overview

* **Performance Tuned**: Includes targeted scheduling and performance optimizations for smoother daily use and gaming.
* **Streamlined Release Model**: Focuses on the actively maintained `LKM` and `BakaSU` variants to keep maintenance predictable and transparent.
* **Optional Feature Integration**: Matching builds may include `SuSFS` and `BBG`.
* **Version Identifier**: `__LOCALVERSION_BASE__`

## Release Variants

* **LKM (Loadable Kernel Module)**
  * Does not include a built-in root solution and is intended for users who prefer a cleaner kernel environment.
  * If root access is required, patch and flash the device `init_boot` image manually through the KernelSU Manager App.

* **BakaSU**
  * Ships with BakaSU integration, recommended for users who need module extensibility, root hiding, or other advanced workflows.

## Installation

1. **Unlock the Bootloader**: Make sure the device bootloader is already unlocked.
2. **Prepare a Recovery Environment**: A recent version of `TWRP` or `OrangeFox Recovery` is recommended.
3. **Flash the Kernel**: Download the appropriate package from the [Releases page](https://github.com/__PROJECT_REPO__/releases) and flash the kernel `zip` through Recovery.
4. **LKM Builds Only: Patch `init_boot`**
   Back up your current `init_boot.img`, patch it with the KernelSU Manager App, and flash the patched image back to the `init_boot` partition using Fastboot or Recovery.
5. **Reboot the Device**: Restart the system and verify the kernel is running as expected.

## Downloads

All current builds are published on the [**Releases Page**](https://github.com/__PROJECT_REPO__/releases).

## Support

Please report issues via [Issues](https://github.com/__PROJECT_REPO__/issues) in this repository.

## Credits

This project is built on top of these open-source projects:

* [BakaSU](https://github.com/Baka-SU/BakaSU) — kernel-side root solution
* [YuzakiKokuban kernel](https://github.com/YuzakiKokuban/android_kernel_xiaomi_sm8850) — device kernel and build reference
* [KernelSU](https://github.com/tiann/KernelSU)

## Disclaimer

Flashing custom software carries inherent risks. Please make a full backup of your personal data before proceeding. I am not responsible for any damage to your device or data loss that may occur as a result of flashing this kernel.
<!-- END-EN -->
