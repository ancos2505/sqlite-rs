#![forbid(unsafe_code, non_ascii_idents)]

//! # SQLite arquitecture
//! *Reference:* https://www.sqlite.org/arch.html

pub mod file_header;
pub mod io;
#[cfg(feature = "log")]
pub(crate) mod log;
#[macro_use]
pub(crate) mod log_macros;
pub mod pager;
pub mod result;
pub mod runtime;
pub mod traits;
#[macro_use]
pub mod macros;

#[cfg(test)]
mod tests;

use std::{fs::Metadata, sync::OnceLock};

use crate::{
  file_header::SqliteHeader, io::SqliteIoMode, result::SqliteResult, runtime::SqliteRuntime,
};

static VERSION_NUMBER: OnceLock<u32> = OnceLock::new();

pub const IN_MEMORY_URI: &str = "sqlite://:memory:";

#[derive(Debug)]
pub struct SqliteConnection {
  runtime: SqliteRuntime,
}

impl SqliteConnection {
  pub fn open(conn_str: impl AsRef<str>) -> SqliteResult<Self> {
    crate::log::EnvLogger::init();

    VERSION_NUMBER.get_or_init(|| {
      let mut s = env!("CARGO_PKG_VERSION").split('.');
      let release = s.next().and_then(|x| x.parse().ok()).unwrap_or(0u32);
      let major = s.next().and_then(|x| x.parse().ok()).unwrap_or(0u32);
      let minor = s.next().and_then(|x| x.parse().ok()).unwrap_or(0u32);

      (10_000 * release) + (100 * major) + minor
    });

    trace!("Starting SqliteRuntime...");

    let runtime = SqliteRuntime::start(conn_str)?;
    trace!("SqliteRuntime started: [{runtime:?}].");

    Ok(Self { runtime })
  }

  pub fn file_header(&self) -> &SqliteHeader {
    &self.runtime.file_header()
  }

  pub fn io_mode(&self) -> &SqliteIoMode {
    self.runtime.pager().io().mode()
  }

  pub fn file_metadata(&self) -> Option<&Metadata> {
    self.runtime.pager().io().file_metadata()
  }
}
