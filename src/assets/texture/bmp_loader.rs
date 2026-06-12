use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::assets::texture::{Texture, loader::TextureLoadError};

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

pub fn load_bmp(bmp_file: &str) -> Result<Texture, TextureLoadError> {
    let file = File::open(bmp_file).map_err(|_| TextureLoadError::OpenFileFailed)?;
    let mut bytes: Vec<u8> = vec![];
    BufReader::new(file)
        .read_to_end(&mut bytes)
        .map_err(|_| TextureLoadError::ReadError)?;
    let mut context: ParseContext = Default::default();
    let header = get_header(&bytes)?;
    let header_info = get_header_info(&bytes)?;
    println!("width: {}", header_info.width);
    println!("height: {}", header_info.height);
    println!("size: {}", header_info.size);
    let img_data = bytes
        .get(header.offset as usize..header_info.size as usize)
        .ok_or(TextureLoadError::BadFormating)?
        .to_vec();

    Ok(Texture {
        data: img_data,
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

fn get_header(bytes: &[u8]) -> Result<BMPHeader, TextureLoadError> {
    if bytes.len() < 14 {
        Err(TextureLoadError::BadFormating)?
    }
    let bmp_type: [u8; 2] = byte_cast!(bytes, 0x00, u16)
        .map_err(|_| TextureLoadError::BadFormating)?
        .to_le_bytes();
    if !matches!(
        bmp_type,
        [b'B', b'M'] | [b'B', b'A'] | [b'C', b'I'] | [b'C', b'P'] | [b'I', b'C'] | [b'P', b'T']
    ) {
        Err(TextureLoadError::BadFormating)?
    };
    let file_size = byte_cast!(bytes, 0x02, u32).map_err(|_| TextureLoadError::BadFormating)?;
    let reserved1 = byte_cast!(bytes, 0x06, u16).map_err(|_| TextureLoadError::BadFormating)?;
    let reserved2 = byte_cast!(bytes, 0x08, u16).map_err(|_| TextureLoadError::BadFormating)?;
    let offset = byte_cast!(bytes, 0x0A, u32).map_err(|_| TextureLoadError::BadFormating)?;
    Ok(BMPHeader {
        bmp_type,
        reserved: (reserved1, reserved2),
        offset,
        file_size,
    })
}

fn get_header_info(bytes: &[u8]) -> Result<BMPHeaderInfo, TextureLoadError> {
    if bytes.len() < 54 {
        Err(TextureLoadError::BadFormating)?
    }
    let size = byte_cast!(bytes, 0x22, u32).map_err(|_| TextureLoadError::BadFormating)?;
    let width = byte_cast!(bytes, 0x12, u32).map_err(|_| TextureLoadError::BadFormating)?;
    let height = byte_cast!(bytes, 0x16, u32).map_err(|_| TextureLoadError::BadFormating)?;
    let bits = byte_cast!(bytes, 0x1C, u16).map_err(|_| TextureLoadError::BadFormating)?;
    let size = if size == 0 {
        width * height * bits as u32 / 8
    } else {
        size
    };
    Ok(BMPHeaderInfo {
        size,
        width,
        height,
    })
}
