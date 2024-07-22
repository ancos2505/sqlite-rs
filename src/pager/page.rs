use std::fmt::Debug;

use crate::result::SqliteResult;

pub struct Page<const N: usize> {
  size: usize,
  data: [u8; N],
}

impl<const N: usize> Page<N> {
  pub fn parse(input: &[u8]) -> SqliteResult<Box<dyn ValidPageSize>>
  where
    Page<N>: ValidPageSize,
  {
    let page = input.try_into().map(|data: [u8; N]| {
      Box::new(Page {
        size: data.len(),
        data,
      })
    })?;
    Ok(page)
  }

  pub fn size(&self) -> usize {
    self.size
  }

  pub fn data(&self) -> [u8; N] {
    self.data
  }
}

impl<const N: usize> Debug for Page<N> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Page")
      .field("size", &self.size)
      .field("data", &"[..]")
      .finish()
  }
}

pub trait ValidPageSize {}
impl ValidPageSize for Page<512> {}
impl ValidPageSize for Page<1024> {}
impl ValidPageSize for Page<2048> {}
impl ValidPageSize for Page<4096> {}
impl ValidPageSize for Page<8192> {}
impl ValidPageSize for Page<16384> {}
impl ValidPageSize for Page<32768> {}
impl ValidPageSize for Page<65536> {}
