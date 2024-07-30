use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error{
    //auth related
    #[error("can't hash password")]
    Hash,
    #[error("bad password")]
    BadPassword
}