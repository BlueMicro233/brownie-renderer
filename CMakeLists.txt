cmake_minimum_required(VERSION 3.10)
project(RayTracing)

set(CMAKE_CXX_STANDARD 17)

find_package(OpenMP REQUIRED)

add_executable(RayTracing src/main.cpp src/Object.hpp src/Vector.cpp src/Vector.hpp src/Sphere.hpp src/global.hpp src/Triangle.hpp src/Scene.cpp
        src/Scene.hpp src/Light.hpp src/AreaLight.hpp src/BVH.cpp src/BVH.hpp src/Bounds3.hpp src/Ray.hpp src/Material.hpp src/Intersection.hpp
        src/Renderer.cpp src/Renderer.hpp
        src/Hardware_Detector.cpp
        src/Hardware_Detector.hpp)

target_link_libraries(RayTracing OpenMP::OpenMP_CXX)