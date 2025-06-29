#![cfg_attr(not(test), no_std)]

pub mod items;

use tlvc::{ChunkHandle, TlvcRead, TlvcReadError, TlvcReader};
use zerocopy::{FromBytes, IntoBytes};

/// A vital product data item.
pub trait Item {
    /// TLV-C tag for this item.
    fn tag() -> [u8; 4];
}

/// Interpret TLV-C data from a byte slice to retreive the first instance of an
/// [`Item`].
///
/// The first and only chunk must have the tag "VPD0" with the vital product
/// data nested inside.
pub fn read_from_slice<V: IntoBytes + FromBytes + Item>(storage: &[u8]) -> Result<V, Error<&[u8]>> {
    let mut out = V::new_zeroed();

    let mut reader = TlvcReader::begin(storage).map_err(Error::Begin)?;

    let Ok(Some(chunk)) = reader.next() else {
        return Err(Error::NoRootChunk);
    };

    if chunk.header().tag != *b"VPD0" {
        return Err(Error::NoRootChunk);
    }

    let chunk = get_chunk_with_tag(chunk.read_as_chunks(), V::tag())?;

    let len = chunk.len() as usize;
    if len > out.as_mut_bytes().len() {
        return Err(Error::ChunkSize);
    }

    chunk
        .read_exact(0, &mut out.as_mut_bytes()[..len])
        .map_err(Error::Read)?;

    Ok(out)
}

/// Get the first chunk with the correct tag.
fn get_chunk_with_tag(
    mut reader: TlvcReader<&[u8]>,
    tag: [u8; 4],
) -> Result<ChunkHandle<&[u8]>, Error<&[u8]>> {
    loop {
        match reader.next() {
            Ok(Some(chunk)) => {
                let mut scratch = [0; 32];
                if chunk.header().tag == tag {
                    chunk
                        .check_body_checksum(&mut scratch)
                        .map_err(Error::Checksum)?;
                    break Ok(chunk);
                }
            }
            Ok(None) => return Err(Error::NoSuchChunk(tag)),
            Err(e) => return Err(Error::NextChunk(e)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error<R>
where
    R: TlvcRead,
{
    Begin(TlvcReadError<R::Error>),
    Read(TlvcReadError<R::Error>),
    Checksum(TlvcReadError<R::Error>),
    NextChunk(TlvcReadError<R::Error>),
    NoSuchChunk([u8; 4]),
    NoRootChunk,
    ChunkSize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_hardware_version() {
        let text = tlvc_text::from_str(
            r#"
            [("VPD0", [
                ("HW  ", [ [3, 2, 1] ]),
                ("SN  ", [ [25, 51, 0x04, 0x41] ]),
            ])]
            "#,
        )
        .unwrap();
        let packed = tlvc_text::pack(&text);

        let hw: items::HardwareVersion = read_from_slice(packed.as_slice()).unwrap();
        assert_eq!(hw.major, 3);
        assert_eq!(hw.minor, 2);
        assert_eq!(hw.patch, 1);

        let sn: items::SerialNumber = read_from_slice(packed.as_slice()).unwrap();
        assert_eq!(sn.year, 25);
        assert_eq!(sn.week, 51);
        assert_eq!(sn.sequence, 0x4104);
    }
}
