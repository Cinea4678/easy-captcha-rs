//! This create is used for preserve this name before this project is finished developing.

#![warn(missing_docs)]
#![allow(dead_code)]

pub(crate) mod base;
pub mod captcha;
pub mod extension;
mod utils;

pub use base::captcha::*;

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//
//     }
// }
