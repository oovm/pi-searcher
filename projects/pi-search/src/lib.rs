// #![forbid(missing_docs)]
// #![forbid(missing_debug_implementations)]
// #![forbid(rustdoc::missing_crate_level_docs)]
// #![doc = include_str!("../Readme.md")]
// #![doc(html_logo_url = "https://avatars.githubusercontent.com/u/31191489")]
// #![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/31191489")]

mod base10;
mod base256;
pub mod utils;

pub use base10::PiBase10;
pub use base256::PiBase256;
