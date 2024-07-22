mod page;

#[cfg(test)]
mod tests;

use std::fmt::Debug;

use crate::header::PageSize;
use crate::{
  header::{
    FileFormatReadVersion, FileFormatWriteVersion, MagicHeaderString, ReservedBytesPerPage,
  },
  io::SqliteIo,
  result::SqliteResult,
  traits::ParseBytes,
};

use self::page::Page;

#[derive(Debug)]
pub struct SqlitePager {
  io: SqliteIo,
  page_size: PageSize,
  // reserved_bytes_per_page: ReservedBytesPerPage,
}

impl SqlitePager {
  pub fn connect(mut io: SqliteIo) -> SqliteResult<Self> {
    io.rewind()?;
    const BYTES_TO_READ: usize = MagicHeaderString::LENGTH_BYTES
      + PageSize::LENGTH_BYTES
      + FileFormatWriteVersion::LENGTH_BYTES
      + FileFormatReadVersion::LENGTH_BYTES
      + ReservedBytesPerPage::LENGTH_BYTES;
    let mut buf = [0u8; BYTES_TO_READ];

    let bytes_read = io.read(&mut buf)?;
    trace!("[{bytes_read}] Bytes read from [{}]", io.mode());
    let pager = if bytes_read > 0 {
      Self {
        io,
        page_size: PageSize::parse_bytes(&buf[16..=17])?,
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

  pub fn get_first_page(&mut self) -> SqliteResult<()> {
    let page_number: u32 = 1;
    {
      match self.page_size {
        PageSize::L512 => {
          const PAGE_SIZE: usize = 512;
          let mut bytes = [0u8; PAGE_SIZE];
          let bytes_read = self.io.read(&mut bytes)?;
          dbg!(bytes_read);
          let page = Page::<PAGE_SIZE>::parse(&bytes)?;
        }
        PageSize::L1024 => {
          const PAGE_SIZE: usize = 1024;
          let mut bytes = [0u8; PAGE_SIZE];
          let bytes_read = self.io.read(&mut bytes)?;
          dbg!(bytes_read);
          let page = Page::<PAGE_SIZE>::parse(&bytes)?;
        }
        PageSize::L2048 => {
          const PAGE_SIZE: usize = 2048;
          let mut bytes = [0u8; PAGE_SIZE];
          let bytes_read = self.io.read(&mut bytes)?;
          dbg!(bytes_read);
          let page = Page::<PAGE_SIZE>::parse(&bytes)?;
        }
        PageSize::L4096 => {
          const PAGE_SIZE: usize = 4096;
          let mut bytes = [0u8; PAGE_SIZE];
          let bytes_read = self.io.read(&mut bytes)?;
          dbg!(bytes_read);
          let page = Page::<PAGE_SIZE>::parse(&bytes)?;
        }
        PageSize::L8192 => {
          const PAGE_SIZE: usize = 8192;
          let mut bytes = [0u8; PAGE_SIZE];
          let bytes_read = self.io.read(&mut bytes)?;
          dbg!(bytes_read);
          let page = Page::<PAGE_SIZE>::parse(&bytes)?;
        }
        PageSize::L16384 => {
          const PAGE_SIZE: usize = 16384;
          let mut bytes = [0u8; PAGE_SIZE];
          let bytes_read = self.io.read(&mut bytes)?;
          dbg!(bytes_read);
          let page = Page::<PAGE_SIZE>::parse(&bytes)?;
        }
        PageSize::L32768 => {
          const PAGE_SIZE: usize = 32768;
          let mut bytes = [0u8; PAGE_SIZE];
          let bytes_read = self.io.read(&mut bytes)?;
          dbg!(bytes_read);
          let page = Page::<PAGE_SIZE>::parse(&bytes)?;
        }
        PageSize::L65536 => {
          const PAGE_SIZE: usize = 65536;
          let mut bytes = [0u8; PAGE_SIZE];
          let bytes_read = self.io.read(&mut bytes)?;
          dbg!(bytes_read);
          let page = Page::<PAGE_SIZE>::parse(&bytes)?;
        }
      }
    }

    Ok(())
  }

  pub fn page_size(&self) -> &PageSize {
    &self.page_size
  }
  
  pub fn io(&self) -> &SqliteIo {
        &self.io
    }
}
