# Brownie Renderer
$$
L_o(x_0, \omega_0) = \sum_{n=1}^{\infty} \int_{\Omega} \cdots \int_{\Omega} L_e(x_n \rightarrow x_{n-1}) \prod_{i=1}^{n} f_r(x_i, \omega_{i+1}, \omega_i)\ G(x_i \leftrightarrow x_{i-1})\ d\omega_1 \cdots d\omega_n
$$

<p align="center">
  <em>Brownie Renderer: A Parallelized Path Tracing Light Transport Simulator</em>
</p>

<p align="center">
  <img src="eyecandy/2048spp.png" width="500"/>
</p>

布朗尼渲染器：高质量多核路径追踪渲染器。

本渲染器的目标是成为一个轻量化、可扩展、跨平台的基于物理的高性能渲染器，在接近实时渲染的低采样率下达到最佳效果。

## Features
- **无限次反弹路径追踪**
- **低采样高收敛**
- 完美的漫反射表面渲染
- 自定义渲染质量
- 多核心并行加速（OpenMP）
- 跨平台：macOS（Apple Silicon + Intel）、Linux（x86-64 + ARM64）

## 准备做的
- 进一步的采样优化
- ReSTIR 蓄水池时空重要性重采样算法
- 自定义场景
- 多种材质支持
...等等。

## 当前的低采样效果
| ![](eyecandy/2spp.png) | ![](eyecandy/4spp.png) | ![](eyecandy/16spp.png) |
|:--:|:--:|:--:|
| 2 spp | 4 spp | 16 spp |

## 构建

### 前置条件

| 平台 | 编译器 | OpenMP |
|------|--------|--------|
| macOS (Apple Silicon) | Xcode CLT (Apple Clang) | `brew install libomp` |
| macOS (Intel) | Xcode CLT (Apple Clang) | `brew install libomp` |
| Linux (x86-64) | GCC / Clang | 系统自带（`gcc` 已包含） |
| Linux (ARM64) | GCC / Clang | 系统自带（`gcc` 已包含） |

### 使用 Makefile（推荐）

```shell
make          # 编译
make run      # 编译并运行
make clean    # 清理构建产物
```

### 使用 CMake

```shell
mkdir build && cd build
cmake .. -DCMAKE_CXX_FLAGS="-Xpreprocessor -fopenmp" -DCMAKE_EXE_LINKER_FLAGS="-L$(brew --prefix libomp)/lib -lomp"
make
```

> Linux 下不需要 `-Xpreprocessor` 和 libomp 路径，直接 `cmake .. && make` 即可。

## 运行

```shell
./build/RayTracing
```

或

```shell
make run
```

程序会提示输入每像素采样数（samples per pixel, spp），输入数字后开始渲染。

## 项目结构

```
├── Makefile                  # 跨平台构建文件
├── CMakeLists.txt            # CMake 构建配置（备选）
├── src/
│   ├── main.cpp              # 入口
│   ├── Renderer.cpp/hpp      # 渲染器主循环
│   ├── Scene.cpp/hpp         # 场景管理
│   ├── BVH.cpp/hpp           # 层次包围盒加速结构
│   ├── Vector.cpp/hpp        # 向量数学库
│   ├── Triangle.hpp          # 三角形网格
│   ├── Sphere.hpp            # 球体
│   ├── Material.hpp          # 材质（漫反射）
│   ├── Light.hpp             # 光照
│   ├── AreaLight.hpp         # 面光源
│   ├── Object.hpp            # 物体基类
│   ├── Ray.hpp               # 光线
│   ├── Intersection.hpp      # 交点
│   ├── Bounds3.hpp           # 包围盒
│   ├── global.hpp            # 全局常量
│   ├── OBJ_Loader.hpp        # OBJ 文件加载器
│   └── Hardware_Detector.cpp/hpp  # CPU 信息检测（跨平台）
├── models/                   # 模型文件
├── eyecandy/                 # 渲染结果截图
└── README.md
```
