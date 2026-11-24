//! Platform-level data formats, shared by every Xbox Series title.
//!
//! Each module is pure (bytes in, bytes out, no I/O): [`texture`] (the
//! GPU's tiled texture layouts and copies to and from linear rows),
//! [`bcpack`] (BCPack texture decompression) and [`xma`] (the XMA2 audio
//! codec). Opus is decoded in `victory-hle` with `opus_pure`, and the zlib
//! streams of hardware decompression are inflated there too.

pub mod bcpack;
pub mod texture;
pub mod xma;
