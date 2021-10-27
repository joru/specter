#![allow(non_snake_case)]

use crate::color_matching_functions_data::{wavelength_to_CIE_XYZ, min_wavelenght};
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
    let max_wl_ex=min_wavelenght+wavelength_to_CIE_XYZ.len();
    let max_wl_inc=max_wl_ex-1;
    println!("Wavelength {} to {} nm.",min_wavelenght,max_wl_inc);
    println!("sRGB->CIE XYZ->sRGB:{:#?}",CIE_XYZ_to_sRGB*sRGB_to_CIE_XYZ);
    println!("CIE XYZ->sRGB->CIE XYZ:{:#?}",sRGB_to_CIE_XYZ*CIE_XYZ_to_sRGB);
    let iluminant_E_XYZ=Vec3::new(1.0,1.0,1.0);
    let iluminant_D65_sRGB=Vec3::new(1.0,1.0,1.0);
    let iluminant_E_sRGB=CIE_XYZ_to_sRGB*iluminant_E_XYZ;
    let iluminant_D65_XYZ=sRGB_to_CIE_XYZ*iluminant_D65_sRGB;
    println!("E_sRGB:{:?}, D65_XYZ:{:?}",iluminant_E_sRGB,iluminant_D65_XYZ);
    let (raw_min_R, raw_max_R, raw_min_G, raw_max_G, raw_min_B, raw_max_B) = min_max_vec3(wavelength_to_CIE_XYZ.iter().map(|&(X,Y,Z)|CIE_XYZ_to_sRGB*Vec3::new(X,Y,Z)));
    println!("{:#?}-{:#?}\t{:#?}-{:#?}\t{:#?}-{:#?}",raw_min_R,raw_max_R,raw_min_G,raw_max_G,raw_min_B,raw_max_B);
    let raw_min_min=raw_min_R.min(raw_min_G).min(raw_min_B);
    let raw_max_max=raw_max_R.max(raw_max_G).max(raw_max_B);
    let to_add_xyz=iluminant_D65_XYZ*(-raw_min_min);
    let to_mul=1.0/(raw_max_max-raw_min_min);
    let stat2=min_max_vec3(wavelength_to_CIE_XYZ.iter().map(|&(X,Y,Z)|CIE_XYZ_to_sRGB*((Vec3::new(X,Y,Z)+to_add_xyz)*to_mul)));
    println!("{:#?}",stat2);

}

fn min_max_vec3(raw_sRGBs: impl Iterator<Item=Vec3>) -> (f32, f32, f32, f32, f32, f32) {
    let mut min_R=1f32;
    let mut max_R=0f32;
    let mut min_G=1f32;
    let mut max_G=0f32;
    let mut min_B=1f32;
    let mut max_B=0f32;
    for rgb in raw_sRGBs{
        let R=rgb.X();
        let G=rgb.Y();
        let B=rgb.Z();
    
        if  R< min_R{
            min_R=R;
        }
        if R>max_R{
            max_R=R;
        }
        if  G< min_G{
            min_G=G;
        }
        if G>max_G{
            max_G=G;
        }
        if  B< min_B{
            min_B=B;
        }
        if B>max_B{
            max_B=B;
        }

    }
    (min_R, max_R, min_G, max_G, min_B, max_B)
}
