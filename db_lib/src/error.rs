use core::fmt;
use image::ImageError;
use std::error::Error;

pub type DbError = diesel::result::Error;

#[derive(Debug)]
pub enum DbOrImageError {
    Db(DbError),
    Image(ImageError),
}

impl From<ImageError> for DbOrImageError {
    fn from(error: ImageError) -> Self {
        Self::Image(error)
    }
}

impl From<DbError> for DbOrImageError {
    fn from(error: DbError) -> Self {
        Self::Db(error)
    }
}

#[derive(Debug)]
pub enum InsertImageFromUrlError {
    Db(DbError),
    Image(ImageError),
    Network(reqwest::Error),
}

impl From<DbError> for InsertImageFromUrlError {
    fn from(err: DbError) -> Self {
        InsertImageFromUrlError::Db(err)
    }
}

impl From<ImageError> for InsertImageFromUrlError {
    fn from(err: ImageError) -> Self {
        InsertImageFromUrlError::Image(err)
    }
}

impl From<reqwest::Error> for InsertImageFromUrlError {
    fn from(err: reqwest::Error) -> Self {
        InsertImageFromUrlError::Network(err)
    }
}

impl From<ImageFromUrlError> for InsertImageFromUrlError {
    fn from(err: ImageFromUrlError) -> Self {
        match err {
            ImageFromUrlError::Image(err) => InsertImageFromUrlError::Image(err),
            ImageFromUrlError::Network(err) => InsertImageFromUrlError::Network(err),
        }
    }
}

impl From<DbOrImageError> for InsertImageFromUrlError {
    fn from(err: DbOrImageError) -> Self {
        match err {
            DbOrImageError::Db(err) => InsertImageFromUrlError::Db(err),
            DbOrImageError::Image(err) => InsertImageFromUrlError::Image(err),
        }
    }
}

#[derive(Debug)]
pub enum ImageFromUrlError {
    Image(ImageError),
    Network(reqwest::Error),
}

impl From<ImageError> for ImageFromUrlError {
    fn from(err: ImageError) -> Self {
        ImageFromUrlError::Image(err)
    }
}

impl From<reqwest::Error> for ImageFromUrlError {
    fn from(err: reqwest::Error) -> Self {
        ImageFromUrlError::Network(err)
    }
}

impl fmt::Display for DbOrImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Db(e) => write!(f, "{}", e),
            Self::Image(e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Display for InsertImageFromUrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Db(e) => write!(f, "{}", e),
            Self::Image(e) => write!(f, "{}", e),
            Self::Network(e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Display for ImageFromUrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Network(e) => write!(f, "{}", e),
            Self::Image(e) => write!(f, "{}", e),
        }
    }
}

impl Error for DbOrImageError {}
impl Error for InsertImageFromUrlError {}
impl Error for ImageFromUrlError {}
