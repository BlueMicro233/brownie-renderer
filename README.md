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

### macOS

需要 Xcode Command Line Tools 和 libomp：

```shell
brew install libomp
```

编译：

```shell
make          # Debug（build/debug/）
make release  # Release（build/release/，-O3）
make run      # Release 编译 + 运行
make clean
```

也可以直接用 CMake：

```shell
mkdir build && cd build
cmake .. \
  -DCMAKE_CXX_FLAGS="-Xpreprocessor -fopenmp -I$(brew --prefix libomp)/include" \
  -DCMAKE_EXE_LINKER_FLAGS="-L$(brew --prefix libomp)/lib -lomp"
make
```

### Linux

需要 GCC（自带 OpenMP）和 CMake：

```shell
sudo apt install g++ cmake make       # Debian / Ubuntu
sudo dnf install gcc-c++ cmake make   # Fedora
```

编译：

```shell
make          # Debug
make release  # Release（-O3）
make run      # Release 编译 + 运行
make clean
```

直接用 CMake（不需要任何特殊 flag）：

```shell
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make
```

### Docker

```shell
docker build -t brownie-renderer .
```

## 运行

```shell
make run
```

或

```shell
./build/release/RayTracing
```

程序会提示输入每像素采样数（samples per pixel, spp），输入数字后开始渲染。

## Benchmark

以下数据在 **16 spp**、Cornell Box 场景、784x784 分辨率下测得。

| 平台 | 编译 | 线程 | 墙钟 | User CPU |
|------|------|------|------|----------|
| Apple M5 Max (macOS) | Release | 18 (6P+12E) | 3.8s | 61.1s |
| Apple M5 Max (macOS) | Release | 6 (P-cores) | 4.7s | 27.8s |
| Apple M5 Max (macOS) | Debug | 18 (6P+12E) | 16.0s | 258.2s |
| Rosetta 2 x86_64 (Docker) | Release | 18 (virtual) | 4.9s | 83.7s |

> Apple Silicon 上 E-cores 对渲染加速贡献很小但功耗不小。用 `OMP_NUM_THREADS=6 make run` 仅用 P-cores 可以在几乎不损失速度的前提下大幅降低 CPU 功耗。
