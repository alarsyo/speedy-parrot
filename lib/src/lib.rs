use image::{GrayImage, Luma};

use rand::distributions::{Distribution, Uniform};

fn compute_energy_delta_data(
    img: &GrayImage,
    colors: &[u8],
    labels: &[Vec<u8>],
    x: usize,
    y: usize,
    candidate: u8,
) -> f64 {
    let current = labels[y][x];
    let pixel = img.get_pixel(x as u32, y as u32);
    let Luma(pixel) = pixel;
    let pixel = pixel[0];

    (colors[candidate as usize] as f64 - pixel as f64).abs()
        - (colors[current as usize] as f64 - pixel as f64).abs()
}

fn compute_energy_delta_neighbor(
    img: &GrayImage,
    labels: &[Vec<u8>],
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    candidate: u8,
) -> f64 {
    let width = img.width();
    let height = img.height();
    let nx = x + dx;
    let ny = y + dy;

    if nx < 0 || nx >= width as i32 || ny < 0 || ny >= height as i32 {
        return 0.0;
    }

    let current = labels[y as usize][x as usize];
    let neighbor = labels[ny as usize][nx as usize];
    let lambda = 15.0;

    match dy {
        // Left or right
        0 => lambda * 2.0 * ((candidate != neighbor) as i32 - (current != neighbor) as i32) as f64,

        // Up or down
        _ => lambda * 2.0 * ((candidate == neighbor) as i32 - (current == neighbor) as i32) as f64,
    }
}

fn compute_energy_delta(
    img: &GrayImage,
    colors: &[u8],
    labels: &[Vec<u8>],
    x: usize,
    y: usize,
    candidate: u8,
) -> f64 {
    let mut energy_delta = 0.0;

    energy_delta +=
        compute_energy_delta_neighbor(img, labels, x as i32, y as i32, -1, 0, candidate);
    energy_delta += compute_energy_delta_neighbor(img, labels, x as i32, y as i32, 1, 0, candidate);
    energy_delta += compute_energy_delta_neighbor(img, labels, x as i32, y as i32, 0, 1, candidate);
    energy_delta +=
        compute_energy_delta_neighbor(img, labels, x as i32, y as i32, 0, -1, candidate);

    energy_delta += compute_energy_delta_data(img, colors, labels, x, y, candidate);

    energy_delta
}

fn compute(img: &GrayImage, colors: &[u8], mut temperature: f64, alpha: f64) -> Vec<Vec<u8>> {
    let width = img.width();
    let height = img.height();

    let uniform = Uniform::from(0..=1);
    let uniform_float = Uniform::from(0.0..=1.0);
    let mut rng = rand::thread_rng();
    let mut res = vec![];
    for _ in 0..img.height() {
        let mut row = vec![];
        for _ in 0..img.width() {
            row.push(uniform.sample(&mut rng));
        }
        res.push(row);
    }

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut round: usize = 0;
    let mut last_negative_delta: i32 = 0;

    while last_negative_delta <= 10000 {
        if (x + y) % 2 == round {
            let candidate = uniform.sample(&mut rng);

            let mut energy_delta = compute_energy_delta(img, colors, &res, x, y, candidate) as f64;

            if energy_delta < 0.0 {
                last_negative_delta = 0;
                res[y as usize][x as usize] = candidate;
            } else {
                last_negative_delta += 1;
                energy_delta /= temperature;
                let probability: f64 = (-energy_delta).exp();

                if uniform_float.sample(&mut rng) < probability {
                    res[y as usize][x as usize] = candidate;
                }
            }

            temperature *= alpha;
        }

        x += 1;
        if x == width as usize {
            x = 0;
            y += 1;
            if y == height as usize {
                y = 0;
                round = (round + 1) % 2;
            }
        }
    }

    res
}

pub fn to_black_and_white(img: GrayImage) -> GrayImage {
    let colors: [u8; 2] = [0, 255];

    let labels = compute(&img, &colors, 51., 0.99999);

    let mut result = GrayImage::new(img.width(), img.height());

    for (i, row) in labels.iter().enumerate() {
        for (j, pixel) in row.iter().enumerate() {
            if *pixel != 0 {
                *result.get_pixel_mut(j as u32, i as u32) = Luma([255]);
            }
        }
    }

    result
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use image::{DynamicImage, ImageBuffer};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::Clamped;
    use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

    #[wasm_bindgen]
    pub fn to_black_and_white_from_js(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) {
        let width = canvas.width();
        let height = canvas.height();

        let data: ImageData = ctx
            .get_image_data(0.0, 0.0, width as f64, height as f64)
            .unwrap();
        let raw_pixels = data.data().to_vec();
        let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();

        let grayscale_image = DynamicImage::ImageRgba8(img_buffer).to_luma8();

        let new_image =
            DynamicImage::ImageLuma8(crate::to_black_and_white(grayscale_image)).to_rgba8();

        let raw_pixels = new_image.into_raw();

        let new_img_data =
            ImageData::new_with_u8_clamped_array_and_sh(Clamped(&raw_pixels), width, height);

        ctx.put_image_data(&new_img_data.unwrap(), 0.0, 0.0)
            .expect("Should put image data on Canvas");
    }
}
