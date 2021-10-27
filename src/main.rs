use crate::color_matching_functions_data::{CIE_XYZ, min_wavelenght};

mod color_matching_functions_data;
fn main() {
    let max_wl_ex=min_wavelenght+CIE_XYZ.len();
    let max_wl_inc=max_wl_ex-1;
    println!("Wavelength {} to {} nm.",min_wavelenght,max_wl_inc);
}
