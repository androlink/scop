use std::{fmt::Display, fs::File, io::BufReader};

use crate::core::{
    model::obj_model::{OBJLoader, types::OBJModel},
    traits::loader::Loader,
};

impl Loader<&str> for OBJLoader {
    type Output = Result<OBJModel, OBJLoadError>;

    fn load(&self, source: &str) -> Self::Output {
        let file = File::open(source).map_err(OBJLoadError::Io)?;
        let read_buffer = BufReader::new(file);
        self.load(read_buffer)
    }
}

#[derive(Debug)]
pub enum OBJLoadError {
    InvalidFloat,
    InvalidInteger,
    MissingField,
    InvalidFace,
    UnknownKeyword(String),
    Io(std::io::Error),
}

impl Display for OBJLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = match self {
            OBJLoadError::InvalidFloat => "invalid float".into(),
            OBJLoadError::InvalidInteger => "invalid integer".into(),
            OBJLoadError::MissingField => "missing field".into(),
            OBJLoadError::InvalidFace => "invalid face".into(),
            OBJLoadError::UnknownKeyword(k) => format!("unknown keyword: '{k}'"),
            OBJLoadError::Io(error) => format!("io error: {error}"),
        };
        write!(f, "{data}")
    }
}
