use serde::Deserialize;

use crate::errors::Error;

pub type ClientResult<T> = Result<T, Error>;

#[derive(Debug, Deserialize)]
pub struct PortOneResponse<T> {
    #[serde(flatten)]
    data: Option<T>,
}

impl<T> PortOneResponse<T> {
    pub fn get_data(self) -> Option<T> {
        self.data
    }
}
