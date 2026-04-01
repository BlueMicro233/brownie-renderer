# Brownie Renderer
**A parallelized path tracing light transport simulator.**

布朗尼渲染器：高质量多核路径追踪渲染器。

本渲染器的目标是成为一个轻量化、可扩展的基于物理的高性能渲染器，在接近实时渲染的低采样率下达到最佳效果。

| ![](https://github.com/BlueMicro233/brownie-renderer/blob/main/eyecandy/2spp.png) | ![](https://github.com/BlueMicro233/brownie-renderer/blob/main/eyecandy/2spp.png) | ![](https://github.com/BlueMicro233/brownie-renderer/blob/main/eyecandy/2spp.png) |
|:--:|:--:|:--:|
| 2 spp | 4 spp | 16 spp |

## Features
- **无限次反弹路径追踪**
- **低采样高收敛**
- 完美的漫反射表面渲染
- 自定义渲染质量
- 多核心并行加速

## Future Work
- 进一步的采样优化
- ReSTIR 蓄水池时空重要性重采样算法
- 多种材质支持
...等等。

## 构建
### on Windows 11 (x86-64)
**你需要安装**：
- Windows Subsystem for Linux 2 (WSL 2)
  - 🐧安装一个 Linux 发行版（<img src="https://cdn.simpleicons.org/archlinux/1793D1" height="25"/>Arch Linux, <img src="https://cdn.simpleicons.org/ubuntu/E95420" height="25"/>Ubuntu, etc.）
- <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/clion/clion-original.svg" height="25"/> JetBrains CLion

接着，在 Linux 下安装必要的 C++ 开发工具（CMake, 编译器等）：
```shell
sudo apt install build-essential
unzip cmake
```

在 CLion 里进行如下配置：
<p align="center">
  <img src="https://github.com/BlueMicro233/GAMES101-projects/blob/main/assets/toolchain.png" width="737" height="550">
</p>

