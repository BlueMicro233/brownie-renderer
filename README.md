# Brownie Renderer
**A parallelized path tracing light transport simulator.**

布朗尼渲染器：高质量多核路径追踪渲染器。

本渲染器的目标是成为一个轻量化、可扩展的基于物理的高性能渲染器，在实时渲染的低采样率下达到最佳效果。

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
