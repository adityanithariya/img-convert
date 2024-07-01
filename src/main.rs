use img_convert::ImgConvert;

fn main() {
    let mut image = ImgConvert::new();

    image.read();
    image.convert();
}
