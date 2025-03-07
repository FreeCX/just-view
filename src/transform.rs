pub fn grayscale_to_rgb(data: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len() * 3);
    for gray in data {
        result.push(gray);
        result.push(gray);
        result.push(gray);
    }
    result
}
