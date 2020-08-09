use crate::error::ImageFromUrlError;
use image;
use image::{DynamicImage, GenericImageView, ImageError};
use img_hash::HasherConfig;

pub struct ImageInfo<'a> {
    pub url: &'a str,
    format: &'a str,
    blob: Vec<u8>,
    image_data: DynamicImage,
}

impl ImageInfo<'_> {
    pub fn create_from_url(url: &str) -> Result<ImageInfo, ImageFromUrlError> {
        let blob = reqwest::blocking::get(url)?.bytes()?.to_vec();
        Ok(ImageInfo::create(blob, url)?)
    }

    pub fn create_from_vec<'a>(blob: Vec<u8>) -> Result<ImageInfo<'a>, ImageError> {
        ImageInfo::create(blob, &"")
    }

    fn create(blob: Vec<u8>, url: &str) -> Result<ImageInfo, ImageError> {
        let image_data = image::load_from_memory(&blob)?;
        let format = image::guess_format(&*blob)?
            .extensions_str()
            .first()
            .unwrap();
        Ok(ImageInfo {
            url,
            format,
            blob,
            image_data,
        })
    }

    pub fn get_blob(self: &Self) -> &Vec<u8> {
        &self.blob
    }

    pub fn get_width(self: &Self) -> u32 {
        self.image_data.dimensions().0
    }

    pub fn get_height(self: &Self) -> u32 {
        self.image_data.dimensions().1
    }

    pub fn get_format(self: &Self) -> &str {
        self.format
    }

    pub fn get_file_name(self: &Self) -> &str {
        &self
            .url
            .split("/")
            .last()
            .unwrap()
            .split("?")
            .next()
            .unwrap()
    }

    pub fn get_perceptual_hash(self: &Self) -> Vec<u8> {
        let hasher = HasherConfig::with_bytes_type::<[u8; 32]>()
            .hash_size(16, 16)
            .to_hasher();
        hasher.hash_image(&self.image_data).as_bytes().to_vec()
    }

    pub fn get_thumbnail(self: &Self, width: u32, height: u32) -> Result<Vec<u8>, ImageError> {
        let mut result: Vec<u8> = Vec::new();
        self.image_data
            .thumbnail(width, height)
            .write_to(&mut result, image::ImageOutputFormat::Jpeg(85))?;
        Ok(result)
    }
}
