use std::{
    fs::{self, File},
    io::Read,
    str::FromStr,
};

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};

fn get_bytes_from_filepath(filepath: &str) -> Vec<u8> {
    let mut file = File::open(filepath).expect("cannot open file");
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)
        .expect("failed to read into buffer");

    buffer
}

pub fn print(filepath: &str) {
    let buffer = get_bytes_from_filepath(filepath);
    let png_file = Png::try_from(buffer.as_slice()).unwrap();
    let chunk_type = png_file
        .chunks()
        .iter()
        .map(|c| c.chunk_type().to_string())
        .collect::<Vec<String>>();

    for chunk in chunk_type {
        println!("{chunk}");
    }
}

pub fn decode(filepath: &str, chunk_type: &str) {
    let buffer = get_bytes_from_filepath(filepath);
    let png_file = Png::try_from(buffer.as_slice()).unwrap();

    let target = png_file
        .chunk_by_type(chunk_type)
        .expect("failed to locate chunk type");

    println!("Hidden message is: {}", target.data_as_string().unwrap())
}

pub fn remove(filepath: &str, chunk_type: &str) {
    let buffer = get_bytes_from_filepath(filepath);
    let mut png_file = Png::try_from(buffer.as_slice()).unwrap();

    png_file
        .remove_first_chunk(chunk_type)
        .expect("chunk type not found");
    let write_path = std::path::Path::new(filepath);
    fs::write(write_path, png_file.as_bytes()).expect("failed to wrire to file");
}

pub fn encode(filepath: &str, chunk_type: &str, message: &str) {
    let buffer = get_bytes_from_filepath(filepath);
    let mut png_file = Png::try_from(buffer.as_slice()).unwrap();

    let end = png_file
        .remove_first_chunk("IEND")
        .expect("failed to remove chunk");

    png_file.append_chunk(Chunk::new(
        ChunkType::from_str(chunk_type).unwrap(),
        message.as_bytes().into(),
    ));
    png_file.append_chunk(end);

    let write_path = std::path::Path::new(filepath);
    fs::write(write_path, png_file.as_bytes()).expect("failed to wrire to file");
    println!("message encoded!")
}
