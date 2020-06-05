use std::fs;

struct ImageBuffer {
    bytes: Vec<u8>,
    num_layers: usize,
    image_width: u32,
    image_height: u32,
    layer_length: usize,
}

impl ImageBuffer {
    fn new(input: &str, image_width: u32, image_height: u32) -> Self {
        let bytes: Vec<u8> = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let layer_length: usize = (image_width * image_height) as usize;
        let num_layers = (bytes.len() as f64 / layer_length as f64).ceil() as usize;
        Self {
            bytes,
            image_width,
            image_height,
            layer_length,
            num_layers,
        }
    }

    fn decode(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![2; self.layer_length];
        for layer in self.into_iter() {
            for (i, byte) in layer.iter().enumerate() {
                if result[i] == 2 {
                    result[i] = *byte;
                }
            }
        }
        result
    }
}

impl<'a> IntoIterator for &'a ImageBuffer {
    type Item = &'a [u8];
    type IntoIter = ImageBufferIterator<'a>;

    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        ImageBufferIterator {
            buffer: &self,
            current_layer: 0,
        }
    }
}

impl std::fmt::Display for ImageBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let decoded = self.decode();
        let mut image: String = "".to_string();
        for y in 0..self.image_height {
            let mut row: String = "".to_string();
            for x in 0..self.image_width {
                let i: usize = (y * self.image_width + x) as usize;
                let byte = decoded[i];
                row += match byte {
                    0 => " ",
                    1 => "#",
                    _ => "?",
                }
            }
            if y < self.image_height - 1 {
                row += "\n";
            }
            image += &row;
        }
        write!(f, "{}", image)
    }
}

struct ImageBufferIterator<'a> {
    buffer: &'a ImageBuffer,
    current_layer: usize,
}

impl<'a> Iterator for ImageBufferIterator<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        if self.current_layer >= self.buffer.num_layers {
            None
        } else {
            let start = self.current_layer * self.buffer.layer_length;
            let end = start + self.buffer.layer_length;
            self.current_layer += 1;
            Some(&self.buffer.bytes[start..end])
        }
    }
}

const WIDTH: u32 = 25;
const HEIGHT: u32 = 6;

pub fn solve() {
    let filename = "res/day_08.txt";
    let input = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Could not read file: {}", filename));
    let ib = ImageBuffer::new(&input, WIDTH, HEIGHT);

    let mut most_filled_layer_counts = (ib.layer_length, 0, 0);
    for layer in ib.into_iter() {
        let mut layer_counts = (0, 0, 0);
        for byte in layer {
            match byte {
                0 => layer_counts.0 += 1,
                1 => layer_counts.1 += 1,
                2 => layer_counts.2 += 1,
                _ => {}
            }
        }
        if layer_counts.0 < most_filled_layer_counts.0 {
            most_filled_layer_counts = layer_counts
        }
    }

    let part_one = most_filled_layer_counts.1 * most_filled_layer_counts.2;

    println!("{}\n{}", part_one, ib.to_string());
}

#[test]
fn can_create_an_image_buffer() {
    let ib = ImageBuffer::new("123456789012", 3, 2);
    assert_eq!(ib.bytes, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    assert_eq!(ib.layer_length, 6);
    assert_eq!(ib.num_layers, 2);
}

#[test]
fn can_iterate_the_layers_of_an_image_buffer() {
    let ib = ImageBuffer::new("123456789012", 3, 2);
    let mut it = ib.into_iter();
    assert_eq!(it.next(), Some(&ib.bytes[..6]));
    println!();
    assert_eq!(it.next(), Some(&ib.bytes[6..]));
    assert_eq!(it.next(), None);
}

#[test]
fn can_decode_an_image_from_an_image_buffer() {
    let ib = ImageBuffer::new("0222112222120000", 2, 2);
    assert_eq!(ib.decode(), vec![0, 1, 1, 0]);
}

#[test]
fn can_view_the_decoded_image() {
    let ib = ImageBuffer::new("0222112222120000", 2, 2);
    assert_eq!(ib.to_string(), " #\n# ");
}
