#![allow(non_snake_case)]

use crate::color_matching_functions_data::{min_wavelenght, wavelength_to_CIE_XYZ};
mod color_matching_functions_data;
mod tensors;
use tensors::{Mat, Vec3};

#[allow(non_upper_case_globals)]
pub const sRGB_to_CIE_XYZ: Mat = Mat::new([
    Vec3::new(0.41239080, 0.35758434, 0.18048079),
    Vec3::new(0.21263901, 0.71516868, 0.07219232),
    Vec3::new(0.01933082, 0.11919478, 0.95053215),
]);
#[allow(non_upper_case_globals)]
pub const CIE_XYZ_to_sRGB: Mat = Mat::new([
    Vec3::new(3.24096994, -1.53738318, -0.49861076),
    Vec3::new(-0.96924364, 1.8759675, 0.04155506),
    Vec3::new(0.05563008, -0.20397696, 1.05697251),
]);

fn main() {
    let max_wl_ex = min_wavelenght + wavelength_to_CIE_XYZ.len();
    let max_wl_inc = max_wl_ex - 1;
    println!("Wavelength {} to {} nm.", min_wavelenght, max_wl_inc);
    println!(
        "sRGB->CIE XYZ->sRGB:{:#?}",
        CIE_XYZ_to_sRGB * sRGB_to_CIE_XYZ
    );
    println!(
        "CIE XYZ->sRGB->CIE XYZ:{:#?}",
        sRGB_to_CIE_XYZ * CIE_XYZ_to_sRGB
    );
    let iluminant_E_XYZ = Vec3::new(1.0, 1.0, 1.0);
    let iluminant_D65_sRGB = Vec3::new(1.0, 1.0, 1.0);
    let iluminant_E_sRGB = CIE_XYZ_to_sRGB * iluminant_E_XYZ;
    let iluminant_D65_XYZ = sRGB_to_CIE_XYZ * iluminant_D65_sRGB;
    println!(
        "E_sRGB:{:?}, D65_XYZ:{:?}",
        iluminant_E_sRGB, iluminant_D65_XYZ
    );
    let (raw_min_R, raw_max_R, raw_min_G, raw_max_G, raw_min_B, raw_max_B) = min_max_vec3(
        wavelength_to_CIE_XYZ
            .iter()
            .map(|&(X, Y, Z)| CIE_XYZ_to_sRGB * Vec3::new(X, Y, Z)),
    );
    println!(
        "{:#?}-{:#?}\t{:#?}-{:#?}\t{:#?}-{:#?}",
        raw_min_R, raw_max_R, raw_min_G, raw_max_G, raw_min_B, raw_max_B
    );
    let raw_min_min = raw_min_R.min(raw_min_G).min(raw_min_B);
    let better_raw_min_min = -f32::from_bits((-raw_min_min).to_bits() + 2);
    let raw_max_max = raw_max_R.max(raw_max_G).max(raw_max_B);
    let to_add_xyz = iluminant_D65_XYZ * (-better_raw_min_min);
    let to_mul = 1.0 / (raw_max_max - better_raw_min_min);
    let converter = Converter {
        mul: to_mul,
        add: to_add_xyz,
    };
    println!(
        "\n{} {} {} {}\n{:?}\n",
        raw_min_min, better_raw_min_min, raw_max_max, to_mul, to_add_xyz
    );
    let stat2 = min_max_vec3(
        wavelength_to_CIE_XYZ
            .iter()
            .map(|&(X, Y, Z)| converter.convert(X, Y, Z)),
    );
    println!("{:#?}", stat2);
    let image_width = 640;
    let image_height = 360;
    let mut buffer = vec![0u8; image_width * image_height * 3];
    let path = format!("out/out.png");
    println!("{}", path);
    let bg = converter.to_bytes(0.0, 0.0, 0.0);
    for y in 0..image_height {
        for x in 0..image_width {
            let idx = (y * image_width + x) * 3;
            buffer[idx + 0] = bg.0;
            buffer[idx + 1] = bg.1;
            buffer[idx + 2] = bg.2;
        }
    }
    let rainbow_start_x = (image_width - wavelength_to_CIE_XYZ.len()) / 2;
    let rainbow_start_y = 16;
    let rainbow_height = 64;
    for i in 0..wavelength_to_CIE_XYZ.len() {
        let idx = (rainbow_start_x + i + rainbow_start_y * image_width) * 3;
        let xyz = wavelength_to_CIE_XYZ[i];
        let rgb = converter.to_bytes(xyz.0, xyz.1, xyz.2);
        buffer[idx + 0] = rgb.0;
        buffer[idx + 1] = rgb.1;
        buffer[idx + 2] = rgb.2;
    }
    let rainbow_copy_src_start = (rainbow_start_y * image_width + rainbow_start_x) * 3;
    let rainbow_copy_len = wavelength_to_CIE_XYZ.len() * 3;
    for y in 1..rainbow_height {
        let rainbow_copy_dst_start = ((rainbow_start_y + y) * image_width + rainbow_start_x) * 3;
        buffer.copy_within(
            rainbow_copy_src_start..(rainbow_copy_src_start + rainbow_copy_len),
            rainbow_copy_dst_start,
        );
    }

    println!("Saving...");
    image::save_buffer(
        path,
        &buffer,
        image_width as u32,
        image_height as u32,
        image::ColorType::Rgb8,
    )
    .expect("error saving image");
    println!("Done.")
}
#[derive(Debug)]
struct Converter {
    pub mul: f32,
    pub add: Vec3,
}

impl Converter {
    pub fn convert(&self, X: f32, Y: f32, Z: f32) -> Vec3 {
        CIE_XYZ_to_sRGB * ((Vec3::new(X, Y, Z) + self.add) * self.mul)
    }
    pub fn to_bytes(&self, X: f32, Y: f32, Z: f32) -> (u8, u8, u8) {
        let sRGBf = self.convert(X, Y, Z);
        (
            Self::to_byte(sRGBf.X()),
            Self::to_byte(sRGBf.Y()),
            Self::to_byte(sRGBf.Z()),
        )
    }
    fn to_byte(f: f32) -> u8 {
        ((f.powf(1f32 / 2.2) * 255.99) as i32).clamp(0, 255) as u8
    }
}

fn min_max_vec3(raw_sRGBs: impl Iterator<Item = Vec3>) -> (f32, f32, f32, f32, f32, f32) {
    let mut min_R = 1f32;
    let mut max_R = 0f32;
    let mut min_G = 1f32;
    let mut max_G = 0f32;
    let mut min_B = 1f32;
    let mut max_B = 0f32;
    for rgb in raw_sRGBs {
        let R = rgb.X();
        let G = rgb.Y();
        let B = rgb.Z();

        if R < min_R {
            min_R = R;
        }
        if R > max_R {
            max_R = R;
        }
        if G < min_G {
            min_G = G;
        }
        if G > max_G {
            max_G = G;
        }
        if B < min_B {
            min_B = B;
        }
        if B > max_B {
            max_B = B;
        }
    }
    (min_R, max_R, min_G, max_G, min_B, max_B)
}
