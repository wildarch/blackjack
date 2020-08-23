use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(b"Hello World")?;
    let compressed = e.finish()?;
    eprintln!("Compressed: {:?}", compressed);
    Ok(())
}
