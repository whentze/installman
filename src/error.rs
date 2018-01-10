pub(crate) use failure::{Error, err_msg};
pub(crate) type Result<T> = ::std::result::Result<T, Error>;
