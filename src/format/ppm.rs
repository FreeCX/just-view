use log::debug;
use zune_core::result::DecodingResult;
use zune_ppm::PPMDecoder;

use super::Loader;
use crate::image::{ColorType, Image};

pub struct Ppm;

impl Loader for Ppm {
    fn load(data: &[u8]) -> Image {
        debug!("Use ppm loader");

        // zune_ppm doesn't support P3 format
        if data.starts_with(b"P3") {
            return Ppm::decode_p3(data);
        }

        let mut decoder = PPMDecoder::new(data);
        decoder.decode_headers().unwrap();
        let (width, height) = decoder.get_dimensions().unwrap();
        let (pixels, color_type) = match decoder.decode().unwrap() {
            DecodingResult::U8(data) => (data, ColorType::RGB8),
            DecodingResult::U16(_) => panic!("format u16 not supported"),
            DecodingResult::F32(_) => panic!("format f32 not supported"),
            _ => panic!("incorrect image"),
        };

        Image { data: pixels, width: width as u32, height: height as u32, color_type }
    }
}

impl Ppm {
    // TODO: improve parsing
    fn decode_p3(data: &[u8]) -> Image {
        let to_int =
            |data: &[u8]| -> usize { data.iter().map(|&i| i as usize - '0' as usize).fold(0, |acc, x| acc * 10 + x) };
        let mut iterator = data.split(|&item| item == ' ' as u8 || item == '\n' as u8);

        let header = iterator.next().unwrap();
        assert_eq!(header, b"P3");

        let width = to_int(iterator.next().unwrap()) as u32;
        let height = to_int(iterator.next().unwrap()) as u32;

        // TODO: use to remap data
        let _max_value = to_int(iterator.next().unwrap()) as u32;

        let mut pixels = Vec::with_capacity(3 * width as usize * height as usize);
        loop {
            let red = iterator.next();
            let green = iterator.next();
            let blue = iterator.next();
            if red.is_none() || green.is_none() || blue.is_none() {
                break;
            }
            let r = to_int(red.unwrap()) as u8;
            let g = to_int(green.unwrap()) as u8;
            let b = to_int(blue.unwrap()) as u8;
            pixels.extend_from_slice(&[r, g, b]);
        }

        Image { data: pixels, width, height, color_type: ColorType::RGB8 }
    }
}
