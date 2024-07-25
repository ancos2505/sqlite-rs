mod page;

#[cfg(test)]
mod tests;

use std::fmt::Debug;

use page::PageKind;

use crate::{
  file_header::{MagicHeaderString, PageSize},
  io::SqliteIo,
  result::SqliteResult,
  traits::ParseBytes,
};

use self::page::Page;

// pub type RetrievedPage = Vec<u8>;

#[derive(Debug)]
pub struct SqlitePager {
  io: SqliteIo,
  page_size: PageSize,
  // reserved_bytes_per_page: ReservedBytesPerPage,
}

#[derive(Debug)]
pub struct RetrievedPage {
  pub(super) size: usize,
  pub(super) data: Vec<u8>,
  pub(super) kind: PageKind,
}

impl RetrievedPage {
  pub fn size(&self) -> usize {
    self.size
  }

  pub fn data(&self) -> &[u8] {
    &self.data
  }

  pub fn kind(&self) -> &PageKind {
    &self.kind
  }
}

impl<const N: usize> From<Page<N>> for RetrievedPage {
  fn from(page: Page<N>) -> Self {
    let Page { size, data, kind } = page;
    Self {
      size,
      data: data.to_vec(),
      kind,
    }
  }
}

impl SqlitePager {
  pub fn connect(mut io: SqliteIo) -> SqliteResult<Self> {
    io.rewind()?;
    const BYTES_TO_READ: usize = MagicHeaderString::LENGTH_BYTES + PageSize::LENGTH_BYTES;
    let mut buf = [0u8; BYTES_TO_READ];
    let bytes_read = io.read(&mut buf)?;

    let msg = format!(
      "[{bytes_read}] Bytes read from [{}]. File: {} at line {}",
      io.mode(),
      file!(),
      line!()
    );
    trace!("{msg}");

    let pager = if bytes_read > 0 {
      let page_size = PageSize::parse_bytes(&buf[16..=17])?;
      trace!("Parsed PageSize [{page_size:?}]");
      Self {
        io,
        page_size,
        // reserved_bytes_per_page: ReservedBytesPerPage::parse_bytes(&[buf[20]])?,
      }
    } else {
      Self {
        io,
        page_size: PageSize::default(),
        // reserved_bytes_per_page: ReservedBytesPerPage::default(),
      }
    };
    Ok(pager)
  }

  pub fn get_first_page(&mut self) -> SqliteResult<RetrievedPage> {
    self.io.rewind()?;
    // TODO
    let page_number: u32 = 1;
    {
      match self.page_size {
        PageSize::L512 => {
          const PAGE_SIZE: usize = 512;

          let mut bytes = [0u8; PAGE_SIZE];
          // TODO page_number
          let bytes_read = self.io.read(&mut bytes)?;
          dbg!(bytes_read);

          return Ok(
            Page {
              size: bytes.len(),
              data: Box::new(bytes),
              kind: PageKind::_Todo,
            }
            .into(),
          );
        }

        PageSize::L1024 => {
          const PAGE_SIZE: usize = 1024;
          let mut bytes = Box::new([0u8; PAGE_SIZE]);
          // TODO page_number
          let bytes_read = self.io.read(&mut *bytes)?;
          dbg!(bytes_read);

          return Ok(
            Page {
              size: bytes.len(),
              data: bytes,
              kind: PageKind::_Todo,
            }
            .into(),
          );
        }
        PageSize::L2048 => {
          const PAGE_SIZE: usize = 2048;
          let mut bytes = Box::new([0u8; PAGE_SIZE]);
          // TODO page_number
          let bytes_read = self.io.read(&mut *bytes)?;
          dbg!(bytes_read);

          return Ok(
            Page {
              size: bytes.len(),
              data: bytes,
              kind: PageKind::_Todo,
            }
            .into(),
          );
        }
        PageSize::L4096 => {
          const PAGE_SIZE: usize = 4096;
          let mut bytes = Box::new([0u8; PAGE_SIZE]);
          // TODO page_number
          let bytes_read = self.io.read(&mut *bytes)?;
          dbg!(bytes_read);

          return Ok(
            Page {
              size: bytes.len(),
              data: bytes,
              kind: PageKind::_Todo,
            }
            .into(),
          );
        }
        PageSize::L8192 => {
          const PAGE_SIZE: usize = 8192;
          let mut bytes = Box::new([0u8; PAGE_SIZE]);
          // TODO page_number
          let bytes_read = self.io.read(&mut *bytes)?;
          dbg!(bytes_read);

          return Ok(
            Page {
              size: bytes.len(),
              data: bytes,
              kind: PageKind::_Todo,
            }
            .into(),
          );
        }
        PageSize::L16384 => {
          const PAGE_SIZE: usize = 16384;
          let mut bytes = Box::new([0u8; PAGE_SIZE]);
          // TODO page_number
          let bytes_read = self.io.read(&mut *bytes)?;
          dbg!(bytes_read);

          return Ok(
            Page {
              size: bytes.len(),
              data: bytes,
              kind: PageKind::_Todo,
            }
            .into(),
          );
        }
        PageSize::L32768 => {
          const PAGE_SIZE: usize = 32768;
          let mut bytes = Box::new([0u8; PAGE_SIZE]);
          // TODO page_number
          let bytes_read = self.io.read(&mut *bytes)?;
          dbg!(bytes_read);

          return Ok(
            Page {
              size: bytes.len(),
              data: bytes,
              kind: PageKind::_Todo,
            }
            .into(),
          );
        }
        PageSize::L65536 => {
          const PAGE_SIZE: usize = 65536;
          let mut bytes = Box::new([0u8; PAGE_SIZE]);
          // TODO page_number
          let bytes_read = self.io.read(&mut *bytes)?;
          dbg!(bytes_read);

          return Ok(
            Page {
              size: bytes.len(),
              data: bytes,
              kind: PageKind::_Todo,
            }
            .into(),
          );
        }
      }
    }
  }

  pub fn page_size(&self) -> &PageSize {
    &self.page_size
  }

  pub fn io(&self) -> &SqliteIo {
    &self.io
  }
}

pub trait ValidPage {}

impl ValidPage for Page<512> {}
impl ValidPage for Page<1024> {}
impl ValidPage for Page<2048> {}
impl ValidPage for Page<4096> {}
impl ValidPage for Page<8192> {}
impl ValidPage for Page<16384> {}
impl ValidPage for Page<32768> {}
impl ValidPage for Page<65536> {}
