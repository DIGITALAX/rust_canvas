// use std::{sync::Arc, error::Error};

// pub fn text_2_image (prompt: &str) -> Result<(), Box<dyn Error>> {
//     let environment = Arc::new(OrtEnvironment::builder().build()?);
//     let mut scheduler = EulerDiscreteScheduler::stable_diffusion_v1_optimized_default()?;
//     let pipeline = StableDiffusionPipeline::new(&environment, "./pyke-diffusers-sd15", StableDiffusionOptions::default()).expect("pipeline error");

//     print!("after pipeline");
    
//     let imgs = pipeline.txt2img(prompt, &mut scheduler, StableDiffusionTxt2ImgOptions::default()).expect("file error");
//     println!("imags {:?}", imgs);
//     // imgs[0].clone().into_rgb8().save("result.png")?;

//     Ok(())
// }

