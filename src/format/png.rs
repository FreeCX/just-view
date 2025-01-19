use png::Decoder;

use super::Loader;
use crate::image::Image;

pub struct Png;

impl Loader for Png {
    fn load(data: &[u8]) -> Image {
        let decoder = Decoder::new(data);
        let mut reader = decoder.read_info().unwrap();
        let mut pixels = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut pixels).unwrap();
        // buf.shrink_to(info.buffer_size());

        Image {
            data: pixels,
            width: info.width,
            height: info.height,
            // TODO: normal cast
            depth: info.bit_depth as u8,
            color_type: info.color_type as u8,
        }
    }
}
