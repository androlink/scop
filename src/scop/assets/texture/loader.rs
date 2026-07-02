use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureLoadError {
    OpenFileFailed,
    ReadError,
    UnrecognizedCharacter,
    BadFormating,
}

impl Display for TextureLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = match self {
            TextureLoadError::OpenFileFailed => "fail to open file",
            TextureLoadError::ReadError => "read error",
            TextureLoadError::UnrecognizedCharacter => "unrecognized char",
            TextureLoadError::BadFormating => "bad formating",
        };
        f.write_str(data)
    }
}
