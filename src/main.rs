use std::{error::Error, fs, path::PathBuf};
use bitvec::prelude::*;

use clap::{Parser, Subcommand};
use image::{DynamicImage, GenericImage, GenericImageView, ImageError, Rgba};


#[derive(Parser)]
#[clap(version)]
struct Arguments {
    #[command(subcommand)]
    cmd: Commands,
}    

#[derive(Subcommand)]
enum Commands {
    Decode {
        #[clap(short='p', long)]
        path: PathBuf,
        #[clap(short='o', long)]
        output_path: Option<PathBuf>,
    },
    Encode {
        #[clap(short='p', long)]
        path: PathBuf,
        #[clap(short='o', long)]
        output_path: PathBuf,
        #[clap(short='c', long)]
        content_to_write: String,
    }
}

fn modify_pixels(img: &DynamicImage, content: Vec<u8>) -> Result<DynamicImage, Box<dyn Error>> {
    let mut new_img: DynamicImage = img.clone();
    let img_dimensions: (u32, u32) = img.dimensions();
    let total_bits: u32 = img_dimensions.0*img_dimensions.1;
    assert!(content.len() as u32 * 8 <= total_bits);
    
    let bit_buff: BitVec = content.into_iter()
        .flat_map(|byte| 
            (0..8).rev()
                .map(move |i| 
                    (byte >> i) &  1 !=  0)).collect();

    for (i, bits) in bit_buff.chunks(3).enumerate() {
        let x = i as u32 % img_dimensions.0;
        let y = i as u32 / img_dimensions.0;
        let mut pixel = img.get_pixel(x, y).0;
        for (pos, bit) in bits.iter().by_vals().enumerate() {
            if bit {
                pixel[pos] |= 1;
            } else {
                pixel[pos] &= 254;
            }
        }
        new_img.put_pixel(x, y, Rgba::<u8>(pixel))
    }
    Ok(new_img)
}

fn write_content(file: PathBuf, output_path: PathBuf, content: String) -> Result<bool, Box<dyn Error>> {
    let img = image::open(file)?;
    let modified_img = modify_pixels(&img, content.into_bytes())?;
    modified_img.save(output_path)?;
    Ok(true)
}

fn redirect_output(content:String, output_path: Option<PathBuf>) -> Result<bool, Box<dyn Error>>{
    match output_path {
        Some(output) => {
            fs::write(output, content)?;
        }
        None => {
            println!("Extracted the following : \n{}\n", content);
        }
    }
    Ok(true)
}

fn read_hidden_content(file: PathBuf) -> Result<Vec<u8>, ImageError> {
    let img = image::open(file)?;
    let mut bitstream: Vec<bool> = Vec::<bool>::new();
    for (_, _, pixel) in img.pixels() {
        bitstream.extend([pixel.0[0]&1!=0, pixel.0[1]&1!=0, pixel.0[2]&1!=0].iter());
    }
    let bytes: Vec<u8> = bitstream.chunks_exact(8).map(|chunk| {
        let mut byte =  0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            if bit {
                byte |=  1 << i;
            }
        }
        byte
    }).collect();
    Ok(bytes)
}

fn read_to_file(file: PathBuf, output_path: Option<PathBuf>) -> Result<bool, Box<dyn Error>> {
    let extraction = read_hidden_content(file)?;
    let success: bool;
    match String::from_utf8(extraction.clone()) {
        Ok(s) => {
            success = redirect_output(s, output_path)?;
        },
        Err(e) => {
            let valid_up_to = e.utf8_error().valid_up_to();
            let s = String::from_utf8_lossy(&extraction[..valid_up_to]).into_owned();
            success = redirect_output(s, output_path)?;
        }
    }
    Ok(success)
}

fn main() -> Result<(), Box<dyn Error>>{
    let args = Arguments::parse();
    match args.cmd {
        Commands::Decode {path, output_path} => {
            read_to_file(path, output_path)?;
        }
        Commands::Encode { path, output_path, content_to_write } => {
            write_content(path, output_path, content_to_write)?;
        }
    }
    Ok(())
}
