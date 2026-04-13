mod bounds3;
mod bvh;
mod global;
mod hardware_detector;
mod intersection;
mod material;
mod object;
mod ray;
mod renderer;
mod scene;
mod triangle;
mod vector;

use std::io;
use std::sync::Arc;
use std::time::Instant;

use hardware_detector::CpuInfo;
use material::{Material, MaterialType};
use renderer::Renderer;
use scene::Scene;
use triangle::MeshTriangle;
use vector::Vec3;

fn main() -> Result<(), String> {
    let cpu = CpuInfo::detect();

    println!("==========================================================");
    println!("            ***** Brownie Renderer *****                 ");
    println!("* A Parallelized Path Tracing Light Transport Simulator *");
    println!("==========================================================");
    println!("                  *** CPU Info ***                       ");
    println!("Model: {}", cpu.model);
    println!("Threads: {}", cpu.logical_cores);
    println!("AVX support: {}", if cpu.has_avx { "Yes" } else { "No" });
    println!("AVX2 support: {}", if cpu.has_avx2 { "Yes" } else { "No" });
    println!("==========================================================");
    println!("Input sample per pixel:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("failed to read spp from stdin: {e}"))?;

    let spp: u32 = input
        .trim()
        .parse()
        .map_err(|e| format!("invalid spp value: {e}"))?;

    if spp < 1 {
        return Err("sample per pixel must >= 1".to_string());
    }

    let mut scene = Scene::new(784, 784);

    let red = {
        let mut m = Material::new(MaterialType::Diffuse, Vec3::zero());
        m.kd = Vec3::new(0.63, 0.065, 0.05);
        Arc::new(m)
    };

    let green = {
        let mut m = Material::new(MaterialType::Diffuse, Vec3::zero());
        m.kd = Vec3::new(0.14, 0.45, 0.091);
        Arc::new(m)
    };

    let white = {
        let mut m = Material::new(MaterialType::Diffuse, Vec3::zero());
        m.kd = Vec3::new(0.725, 0.71, 0.68);
        Arc::new(m)
    };

    let light = {
        let mut m = Material::new(
            MaterialType::Diffuse,
            8.0 * Vec3::new(0.747 + 0.058, 0.747 + 0.258, 0.747)
                + 15.6 * Vec3::new(0.740 + 0.287, 0.740 + 0.160, 0.740)
                + 18.4 * Vec3::new(0.737 + 0.642, 0.737 + 0.159, 0.737),
        );
        m.kd = Vec3::splat(0.65);
        Arc::new(m)
    };

    let bunny_mat = {
        let mut m = Material::new(MaterialType::Diffuse, Vec3::zero());
        m.kd = Vec3::new(0.54, 0.54, 0.54);
        Arc::new(m)
    };

    let bunny = Arc::new(
        MeshTriangle::new("../models/cornellbox/bunny.obj", bunny_mat)
            .map_err(|e| format!("failed to load bunny mesh: {e}"))?,
    );
    let floor = Arc::new(
        MeshTriangle::new("../models/cornellbox/floor.obj", white.clone())
            .map_err(|e| format!("failed to load floor mesh: {e}"))?,
    );
    let shortbox = Arc::new(
        MeshTriangle::new("../models/cornellbox/shortbox.obj", white.clone())
            .map_err(|e| format!("failed to load shortbox mesh: {e}"))?,
    );
    let tallbox = Arc::new(
        MeshTriangle::new("../models/cornellbox/tallbox.obj", white.clone())
            .map_err(|e| format!("failed to load tallbox mesh: {e}"))?,
    );
    let left = Arc::new(
        MeshTriangle::new("../models/cornellbox/left.obj", red)
            .map_err(|e| format!("failed to load left wall mesh: {e}"))?,
    );
    let right = Arc::new(
        MeshTriangle::new("../models/cornellbox/right.obj", green)
            .map_err(|e| format!("failed to load right wall mesh: {e}"))?,
    );
    let light_obj = Arc::new(
        MeshTriangle::new("../models/cornellbox/light.obj", light)
            .map_err(|e| format!("failed to load light mesh: {e}"))?,
    );

    scene.add(bunny);
    scene.add(floor);
    scene.add(shortbox);
    scene.add(tallbox);
    scene.add(left);
    scene.add(right);
    scene.add(light_obj);

    scene.build_bvh();

    let renderer = Renderer;
    let start = Instant::now();
    renderer
        .render(&scene, spp)
        .map_err(|e| format!("render failed: {e}"))?;

    let elapsed = start.elapsed();
    println!("\nRender complete:");
    println!("Time taken: {} hours", elapsed.as_secs() / 3600);
    println!("          : {} minutes", elapsed.as_secs() / 60);
    println!("          : {} seconds", elapsed.as_secs());

    Ok(())
}
