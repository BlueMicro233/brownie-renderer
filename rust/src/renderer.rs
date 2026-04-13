use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use rayon::prelude::*;

use crate::global::{clamp, update_progress};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vector::Vec3;

pub struct Renderer;

impl Renderer {
    pub fn render(&self, scene: &Scene, spp: u32) -> io::Result<()> {
        let mut framebuffer = vec![Vec3::zero(); scene.width * scene.height];

        let scale = (scene.fov.to_radians() * 0.5).tan();
        let image_aspect_ratio = scene.width as f32 / scene.height as f32;
        let eye_pos = Vec3::new(278.0, 273.0, -800.0);

        println!("SPP: {}", spp);

        let finished = AtomicUsize::new(0);

        thread::scope(|scope| {
            let progress_finished = &finished;
            scope.spawn(move || {
                while progress_finished.load(Ordering::Relaxed) < scene.height {
                    let p = progress_finished.load(Ordering::Relaxed) as f32 / scene.height as f32;
                    update_progress(p);
                    thread::sleep(Duration::from_millis(100));
                }
            });

            framebuffer
                .par_chunks_mut(scene.width)
                .enumerate()
                .for_each(|(j, row)| {
                    for (i, pixel) in row.iter_mut().enumerate() {
                        let x = (2.0 * (i as f32 + 0.5) / scene.width as f32 - 1.0)
                            * image_aspect_ratio
                            * scale;
                        let y = (1.0 - 2.0 * (j as f32 + 0.5) / scene.height as f32) * scale;

                        let dir = Vec3::new(-x, y, 1.0).normalize();
                        let ray = Ray::new(eye_pos, dir);

                        let mut color = Vec3::zero();
                        for _ in 0..spp {
                            color += scene.cast_ray(&ray, 0) / spp as f32;
                        }
                        *pixel = color;
                    }
                    finished.fetch_add(1, Ordering::Relaxed);
                });
        });

        update_progress(1.0);

        let file = File::create("binary.ppm")?;
        let mut writer = BufWriter::new(file);
        writer.write_all(format!("P6\n{} {}\n255\n", scene.width, scene.height).as_bytes())?;

        for pixel in framebuffer {
            let color = [
                (255.0 * clamp(0.0, 1.0, pixel.x).powf(0.6)) as u8,
                (255.0 * clamp(0.0, 1.0, pixel.y).powf(0.6)) as u8,
                (255.0 * clamp(0.0, 1.0, pixel.z).powf(0.6)) as u8,
            ];
            writer.write_all(&color)?;
        }

        writer.flush()?;
        Ok(())
    }
}
