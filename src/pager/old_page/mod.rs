
use crate::{header::PageSize, result::SqliteResult};

#[derive(Debug)]
pub struct Page {
  size: PageSize,
  data: PageData,
  kind: PageKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PageData(Vec<u8>);
impl PageData {
  pub fn raw_data(&self) -> &Vec<u8> {
    &self.0
  }
}
impl Page {
  pub const MAX_LENGTH: usize = PageSize::MAX.as_usize();

  pub fn parse<const N: usize>(size: PageSize, raw_data: [u8; N]) -> SqliteResult<Self> {
    let data: PageData = {
      match (&size, raw_data.len()) {
        (&PageSize::L512, 512)
        | (&PageSize::L1024, 1024)
        | (&PageSize::L2048, 2048)
        | (&PageSize::L4096, 4096)
        | (&PageSize::L8192, 8192)
        | (&PageSize::L16384, 16384)
        | (&PageSize::L32768, 32768)
        | (&PageSize::L65536, 65536) => PageData(raw_data.to_vec()),
        _ => {
          return Err(crate::result::SqliteError::Custom(
            "Error on parsing Page. Bytes length does no match with PageSize".into(),
          ))
        }
      }
    };
    // TODO
    Ok(Self {
      size,
      data,
      kind: PageKind::_Todo,
    })
  }

  pub fn data(&self) -> &PageData {
    &self.data
  }
}

