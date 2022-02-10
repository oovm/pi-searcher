// #![forbid(missing_docs)]
// #![forbid(missing_debug_implementations)]
// #![forbid(rustdoc::missing_crate_level_docs)]
// #![doc = include_str!("../Readme.md")]
// #![doc(html_logo_url = "https://avatars.githubusercontent.com/u/31191489")]
// #![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/31191489")]

mod base10;
mod base256;
mod precomputed;
pub mod utils;

pub use base10::PiBase10;
pub use base256::PiBase256;
use bincode::{config::standard, decode_from_slice};
pub use precomputed::PiComputed;

pub fn computed() -> PiComputed<PiBase10> {
    let config = standard();
    let buffer = include_bytes!("computed.bin");
    let (this, _) = decode_from_slice(&buffer[..], config).unwrap();
    this
}
