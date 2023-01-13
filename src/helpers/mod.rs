use nannou::image::{load_from_memory, DynamicImage};
use reqwest;

pub fn get_image(img_name: &str, url: &str) -> Result<DynamicImage, Box<dyn std::error::Error>> {

    let target_url = url.to_string() + img_name;
    let resp_bytes = reqwest::blocking::get(&target_url)?.bytes()?;
    let dyn_image = load_from_memory(&resp_bytes)?;
 //    image::load_from_memory(&resp_bytes)?       
 //           .save_with_format("test.png", ImageFormat::Png)?;
    Ok(dyn_image)
}