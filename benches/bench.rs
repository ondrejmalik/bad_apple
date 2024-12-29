extern crate ratatui_ASCII;
extern crate criterion;
extern crate image;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image::imageops::resize;
use image::ImageResult;
use ratatui_ASCII::{generate_ascii, get_img, grayscale_and_resize};

fn criterion_benchmark(c: &mut Criterion) {
    let mut width: u32 = 400;
    let mut height: u32 = 110;
    let mut current_frame: u16 = 1;
    let chars: [&str; 6] = [" ", ".", ",", "*", "@", "#"];

    c.bench_function("generate_ascii_from_image", |ben| {
        ben.iter(|| {
            let _ = generate_ascii(grayscale_and_resize(match get_img(&mut current_frame) {
                Ok(img) => { img }
                Err(_) => {
                    current_frame == 1;
                    return;
                }
            }, width, height), width, height, chars);
        })
    });

    c.bench_function("grayscale_and_resize_image", |ben| {
        ben.iter(|| {
            let _ = grayscale_and_resize(match get_img(&mut current_frame) {
                Ok(img) => { img }
                Err(_) => {
                    current_frame == 1;
                    return;
                }
            }, width, height);
        });
    });

    c.bench_function("get_img", |ben| {
        ben.iter(|| {
            let _ = match get_img(&mut current_frame) {
                Ok(img) => { img }
                Err(_) => {
                    current_frame == 1;
                    return;
                }
            };
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
