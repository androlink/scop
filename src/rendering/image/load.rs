use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::rendering::image::Texture;

#[derive(Default)]
struct BMPHeader {
    bmp_type: [u8; 2],
    file_size: u32,
    reserved: (u16, u16),
    offset: u32,
}

#[derive(Default)]
struct BMPHeaderInfo {
    size: u32,
    width: u32,
    height: u32,
}

#[derive(Default)]
struct ParseContext {
    header: BMPHeader,
    header_info: BMPHeaderInfo,
}

pub fn load_bmp(bmp_file: &str) -> Option<Texture> {
    let file = File::open(bmp_file)
        .inspect_err(|e| eprintln!("{e}"))
        .ok()?;
    let mut bytes: Vec<u8> = vec![];
    BufReader::new(file)
        .read_to_end(&mut bytes)
        .inspect_err(|e| eprintln!("{e}"))
        .ok()?;
    let mut context: ParseContext = Default::default();
    let header = get_header(&bytes)?;
    let header_info = get_header_info(&bytes)?;
    println!("width: {}", header_info.width);
    println!("height: {}", header_info.height);
    println!("size: {}", header_info.size);
    let Some(img_data) = bytes.get(header.offset as usize..header_info.size as usize) else {
        eprintln!("texture: Bad formating");
        None?
    };

    Some(Texture {
        data: img_data.to_vec(),
        width: header_info.width,
        height: header_info.height,
    })
}

macro_rules! byte_cast {
    ($buf:ident, $offset:literal, $type:ident) => {
        match $buf[$offset..$offset + size_of::<$type>()].try_into() {
            Ok(e) => Ok($type::from_le_bytes(e)),
            Err(e) => Err(e),
        }
    };
}

fn get_header(bytes: &[u8]) -> Option<BMPHeader> {
    if bytes.len() < 14 {
        eprintln!("texture: bad formating");
        None?;
    }
    let bmp_type: [u8; 2] = byte_cast!(bytes, 0x00, u16)
        .inspect_err(|_| eprintln!("texture: bad formating"))
        .ok()?
        .to_le_bytes();
    if !matches!(
        bmp_type,
        [b'B', b'M'] | [b'B', b'A'] | [b'C', b'I'] | [b'C', b'P'] | [b'I', b'C'] | [b'P', b'T']
    ) {
        eprintln!("texture: bad formating");
        None?
    };
    let file_size = byte_cast!(bytes, 0x02, u32)
        .inspect_err(|_| eprintln!("texture: bad formating"))
        .ok()?;
    let reserved1 = byte_cast!(bytes, 0x06, u16)
        .inspect_err(|_| eprintln!("texture: bad formating"))
        .ok()?;
    let reserved2 = byte_cast!(bytes, 0x08, u16)
        .inspect_err(|_| eprintln!("texture: bad formating"))
        .ok()?;
    let offset = byte_cast!(bytes, 0x0A, u32)
        .inspect_err(|_| eprintln!("texture: bad formating"))
        .ok()?;
    Some(BMPHeader {
        bmp_type,
        reserved: (reserved1, reserved2),
        offset,
        file_size,
    })
}

fn get_header_info(bytes: &[u8]) -> Option<BMPHeaderInfo> {
    if bytes.len() < 54 {
        eprintln!("texture: bad formating");
        None?;
    }
    let size = byte_cast!(bytes, 0x22, u32)
        .inspect_err(|_| eprintln!("texture: bad formating"))
        .ok()?;
    let width = byte_cast!(bytes, 0x12, u32)
        .inspect_err(|_| eprintln!("texture: bad formating"))
        .ok()?;
    let height = byte_cast!(bytes, 0x16, u32)
        .inspect_err(|_| eprintln!("texture: bad formating"))
        .ok()?;
    let bits = byte_cast!(bytes, 0x1C, u16)
        .inspect_err(|_| eprintln!("texture: bad formating"))
        .ok()?;
    let size = if size == 0 {
        width * height * bits as u32 / 8
    } else {
        size
    };
    Some(BMPHeaderInfo {
        size,
        width,
        height,
    })
}
