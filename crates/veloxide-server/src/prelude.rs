//! Key default types for this application designed to be imported in most crate modules.
//!
//! Notes:
//!     - The best practice is to have a narrow crate prelude to normalize the key types throughout the application code.
//!     - We keep this as small as possible, and try to limit generic name beside Result and Error (which is re-exported from this module)

pub use crate::error::Error;

use color_eyre::eyre;
pub type Result<T> = eyre::Result<T, Error>;
