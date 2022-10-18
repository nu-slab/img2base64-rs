use std::io::Cursor;

pub fn to_base64(img: &image::DynamicImage, quality: u8) -> String {
    let mut buf: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Jpeg(quality)).unwrap();
    base64::encode(&buf)
}

pub fn to_image(data: &String) -> image::DynamicImage {
    let imdata = base64::decode(data).unwrap();
    image::load_from_memory(&imdata).unwrap()
}


#[cfg(test)]
mod tests {
    use image::Pixel;

    use super::*;
    use imageproc::drawing;

    #[test]
    fn it_works() {
        let mut img = image::DynamicImage::new_rgb8(1280, 720);
        drawing::draw_filled_circle_mut(&mut img, (500, 500), 100, *image::Rgba::from_slice(&[255, 255, 0, 0]));
        img.save("out.png").unwrap();
        let data = to_base64(&img, 50);
        //println!("{}", data);
        let img2 = to_image(&data);
        img2.save("out2.png").unwrap();
    }
}
