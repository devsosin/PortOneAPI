use serde::Deserialize;

use crate::errors::ClientError;

pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Debug, Deserialize)]
pub struct PortOneResponse<T> {
    #[serde(flatten)]
    data: Option<T>,
}
