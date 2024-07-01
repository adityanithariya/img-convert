use clap::{Arg, Command};
use image::{io::Reader as ImageReader, DynamicImage};

pub struct ImgConvert {
    file_name: String,
    extension: String,
    out: String,
    img: Option<DynamicImage>,
}

impl ImgConvert {
    pub fn new() -> Self {
        let arguments = Command::new("img-convert")
            .arg(
                Arg::new("input")
                    .required(true)
                    .short('i')
                    .long("image")
                    .help("Takes image path as input"),
            )
            .arg(
                Arg::new("out")
                    .required(true)
                    .short('o')
                    .long("out")
                    .help("Output extension for image"),
            )
            .get_matches();
        let (file_name, extension) = arguments
            .get_one::<String>("input")
            .expect("Invalid input filename")
            .rsplit_once(".")
            .unwrap();
        ImgConvert {
            file_name: file_name.to_string(),
            extension: extension.to_string(),
            out: arguments
                .get_one::<String>("out")
                .expect("Invalid output extension")
                .clone(),
            img: None,
        }
    }
    pub fn read(&mut self) {
        self.img = Some(
            ImageReader::open(format!("{}.{}", self.file_name, self.extension))
                .expect("File Not Found")
                .decode()
                .expect("Invalid Image"),
        );
    }
    pub fn convert(&self) {
        match self
            .img
            .clone()
            .unwrap()
            .save(format!("{}.{}", self.file_name, self.out))
        {
            Ok(_) => println!("Saved image successfully!"),
            Err(err) => println!("Err: {}", err.to_string()),
        };
    }
}
