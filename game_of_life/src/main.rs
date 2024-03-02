use rand::Rng;
use image::{ImageBuffer, Luma, Rgba, GenericImageView};
use std::fs::File;
use std::io::BufWriter;
use gif::{Encoder, Frame, Repeat};
use image::io::Reader as ImageReader;
use std::fs;
use std::path::Path;

fn main() {
    // simulation structure
    let iterations = 1000;
    let size: usize = 300;
    let mut matrix = vec![vec![2; size]; size];
    fill_randomly(& mut matrix);
    matrix_to_image(&matrix, "frame00.png");

    // gif initialization
    let output_file = File::create("output.gif").unwrap();
    let output_writer = BufWriter::new(output_file);

    // Get the dimensions of the image
    let img = image::open("frame00.png").expect("Failed to open image");
    let dimensions = img.width() as u16;

    // Create a GIF encoder
    let mut encoder = Encoder::new(output_writer, dimensions, dimensions, &[]).unwrap(); // Replace 100, 100 with your image dimensions
    encoder.set_repeat(Repeat::Infinite).unwrap();

    // Create a temporary directory
    let temp_dir = "temp_images";
    match fs::create_dir(temp_dir) {
        Ok(()) => println!("Temporary directory created"),
        Err(e) => println!("Error creating temporary directory: {:?}", e),
    }

    for i in 0..iterations {
        // filepath
        let file_path = format!("temp_images/frame{}.png", i);
        // create image
        matrix_to_image(&matrix, &file_path);
        
        let img = ImageReader::open(&file_path).unwrap().decode().unwrap().to_luma8();
        
        let mut rgba_img = ImageBuffer::new(dimensions.into(), dimensions.into());

        for (x, y, pixel) in img.enumerate_pixels() {
            let luma = pixel.0[0];
            let rgba_pixel = Rgba([luma, luma, luma, 255]);
            rgba_img.put_pixel(x, y, rgba_pixel);
        }
        
        // Create a frame
        let mut frame = Frame::from_rgba_speed(rgba_img.width() as u16, rgba_img.height() as u16, &mut rgba_img.into_raw(), 10);
        frame.delay = 5; // 100 centiseconds = 1 second
        
        // Add the frame to the GIF
        encoder.write_frame(&frame).unwrap();

        iterater(& mut matrix);
    }
    if Path::new(temp_dir).exists() {
        match fs::remove_dir_all(temp_dir) {
            Ok(()) => println!("Temporary directory deleted"),
            Err(e) => println!("Error deleting temporary directory: {:?}", e),
        }
    }
}

fn neighbors(matrix: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut neighbor_count = 0;
    if x < matrix.len() && y < matrix[x].len() {
        if matrix[x][y] == 1 {
            neighbor_count -= 1;
        }
        //count neighbors of a normal square:
        for i in 0..3 {
            for k in 0..3 {
                if matrix[x-1+i][y-1+k] == 1 {
                    neighbor_count += 1;
                }
            }
        }
        return neighbor_count;
    } else {
        println!("too far out bitch.. know your matrix");
        return 404;
    }
}

fn fill_randomly(matrix: & mut Vec<Vec<i32>>) {
    let mut rng = rand::thread_rng();
    for i in 1..matrix.len()-1 {
        for k in 1..matrix[0].len()-1 {
            matrix[i][k] = rng.gen_range(0..2);
        }
    }
}

fn iterater(matrix: & mut Vec<Vec<i32>>) {
    //copy matrix
    let mut temp_matrix = matrix.clone();
    // convert to neighbors
    for i in 1..matrix.len()-1 {
        for k in 1..matrix[1].len()-1 {
            temp_matrix[i][k] = neighbors(&matrix, i, k);
        }
    }
    //convert to bits
    for i in 1..matrix.len()-1 {
        for k in 1..matrix[1].len()-1 {
            if temp_matrix[i][k] < 2 || temp_matrix[i][k] > 3 {
                matrix[i][k] = 0;
            }
            if temp_matrix[i][k] == 3 {
                matrix[i][k] = 1;
            }
        }
    }
}

fn matrix_to_image(matrix: &Vec<Vec<i32>>, output_path: &str) {
    let width = (matrix[0].len()-2 as usize).try_into().unwrap();
    let height = (matrix.len()-2 as usize).try_into().unwrap();
    let mut imgbuf = ImageBuffer::<Luma<u8>, Vec<u8>>::new(width, height);
    for i in 1..matrix.len()-1 {
        for k in 1..matrix[1].len()-1 {
            let pixelvalue = if matrix[i][k] == 1 {255} else {0};
            imgbuf.put_pixel((i-1) as u32, (k-1) as u32, Luma([pixelvalue]));
        }
    }
    imgbuf.save(output_path).unwrap();
}