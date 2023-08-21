#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not connect to server or server returned internal error")]
    ServerError,

    #[error("provided currency was invalid or NBP API does not provide data for it")]
    NoData,
}
