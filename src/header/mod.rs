//! Reference: https://www.sqlite.org/fileformat2.html

mod application_id;
mod database_text_encoding;
mod db_filesize_in_pages;
mod file_change_counter;
mod file_format_version_numbers;
mod freelist_pages;
mod incremental_vacuum_settings;
mod magic_header_string;
mod page_size;
mod payload_fractions;
mod reserved_bytes_per_page;
mod reserved_for_expansion;
mod schema_cookie;
mod schema_format;
mod suggested_cache_size;

mod user_version;
mod version_valid_for;
mod write_library_version;

use crate::traits::{ParseBytes, ValidateParsed};
use crate::{
  impl_name,
  result::{SqliteError, SqliteResult},
};

pub use self::{
  application_id::ApplicationId,
  database_text_encoding::DatabaseTextEncoding,
  db_filesize_in_pages::DatabaseFileSizeInPages,
  file_change_counter::FileChangeCounter,
  file_format_version_numbers::{
    FileFormatReadVersion, FileFormatVersionNumbers, FileFormatWriteVersion,
  },
  freelist_pages::FreeListPages,
  incremental_vacuum_settings::IncrementalVacuumSettings,
  magic_header_string::MagicHeaderString,
  page_size::PageSize,
  payload_fractions::{
    LeafPayloadFraction, MaximumEmbeddedPayloadFraction,
    MinimumEmbeddedPayloadFraction, PayloadFractions,
  },
  reserved_bytes_per_page::ReservedBytesPerPage,
  reserved_for_expansion::ReservedForExpansion,
  schema_cookie::SchemaCookie,
  schema_format::SchemaFormat,
  suggested_cache_size::SuggestedCacheSize,
  user_version::UserVersion,
  version_valid_for::VersionValidFor,
  write_library_version::WriteLibraryVersion,
};

/// # Database File Format
///
/// |Offset | Size  | Description|
/// |-------|-------|------------|
/// |  0    | 16    | The header string: "Sqlite format 3\000" |
/// | 16    |  2    | The database page size in bytes. Must be a power of two between 512 and 32768 inclusive, or the bytes 1 representing a page size of 65536. |
/// | 18    |  1    | File format write version. 1 for legacy; 2 for WAL. |
/// | 19    |  1    | File format read version. 1 for legacy; 2 for WAL. |
/// | 20    |  1    | Bytes of unused "reserved" space at the end of each page. Usually 0. |
/// | 21    |  1    | Maximum embedded payload fraction. Must be 64. |
/// | 22    |  1    | Minimum embedded payload fraction. Must be 32. |
/// | 23    |  1    | Leaf payload fraction. Must be 32. |
/// | 24    |  4    | File change counter. |
/// | 28    |  4    | Size of the database file in pages. The "in-header database size". |
/// | 32    |  4    | Page number of the first freelist trunk page. |
/// | 36    |  4    | Total number of freelist pages. |
/// | 40    |  4    | The schema cookie. |
/// | 44    |  4    | The schema format number. Supported schema formats are 1, 2, 3, and 4. |
/// | 48    |  4    | Default page cache size. |
/// | 52    |  4    | The page number of the largest root b-tree page when in auto-vacuum or incremental-vacuum modes, or zero otherwise. |
/// | 56    |  4    | The database text encoding. A bytes of 1 means UTF-8. A bytes of 2 means UTF-16le. A bytes of 3 means UTF-16be. |
/// | 60    |  4    | The "user version" as read and set by the user_version pragma. |
/// | 64    |  4    | True (non-zero) for incremental-vacuum mode. False (zero) otherwise. |
/// | 68    |  4    | The "Application ID" set by PRAGMA application_id. |
/// | 72    | 20    | Reserved for expansion. Must be zero. |
/// | 92    |  4    | The version-valid-for number. |
/// | 96    |  4    | SQLITE_VERSION_NUMBER |
#[derive(Debug, Default)]
pub struct SqliteHeader {
  /// The header string: "`Sqlite format 3\000`".
  magic_header_string: MagicHeaderString,
  /// The database page size in bytes.
  ///  Must be a power of two between 512 and 32768 inclusive,
  /// or the bytes 1 representing a page size of 65536.
  page_size: PageSize,
  /// File format version numbers.
  file_format_version_numbers: FileFormatVersionNumbers,
  /// Bytes of unused "reserved" space at the end of each page. Usually 0.
  reserved_bytes_per_page: ReservedBytesPerPage,
  /// Payload Fractions.
  payload_fractions: PayloadFractions,
  /// File change counter.
  file_change_counter: FileChangeCounter,
  /// Size of the database file in pages. The "in-header database size".
  db_filesize_in_pages: DatabaseFileSizeInPages,
  /// Unused pages in the database file are stored on a freelist.
  freelist_pages: FreeListPages,
  /// The schema cookie.
  schema_cookie: SchemaCookie,
  /// The schema format number.
  schema_format: SchemaFormat,
  /// Default page cache size.
  suggested_cache_size: SuggestedCacheSize,
  /// Incremental vacuum settings.
  incremental_vacuum_settings: IncrementalVacuumSettings,
  /// The database text encoding. A value of 1 means UTF-8. A value of 2 means UTF-16le. A value of 3 means UTF-16be.
  database_text_encoding: DatabaseTextEncoding,
  /// The "user version" as read and set by the user_version pragma.
  user_version: UserVersion,
  /// The "Application ID" set by PRAGMA application_id.
  application_id: ApplicationId,
  /// Reserved for expansion. Must be zero.
  reserved_for_expansion: ReservedForExpansion,
  /// Version-valid-for number
  version_valid_for: VersionValidFor,
  /// Write library version number
  write_library_version: WriteLibraryVersion,
}
/*

[./data/mydatabase.db]:
database page size:  4096
write format:        1
read format:         1
reserved bytes:      0
file change counter: 5
database page count: 6
freelist page count: 0
schema cookie:       3
schema format:       4
default cache size:  0
autovacuum top root: 0
incremental vacuum:  0
text encoding:       1 (utf8)
user version:        0
application id:      0
software version:    3041002

*/
impl SqliteHeader {
  pub const LENGTH_BYTES: usize = 100;
  pub fn magic_header_string(&self) -> &MagicHeaderString {
    &self.magic_header_string
  }

  pub fn page_size(&self) -> &PageSize {
    &self.page_size
  }

  pub fn file_format_version_numbers(&self) -> &FileFormatVersionNumbers {
    &self.file_format_version_numbers
  }

  pub fn reserved_bytes_per_page(&self) -> &ReservedBytesPerPage {
    &self.reserved_bytes_per_page
  }

  pub fn payload_fractions(&self) -> &PayloadFractions {
    &self.payload_fractions
  }

  pub fn file_change_counter(&self) -> &FileChangeCounter {
    &self.file_change_counter
  }

  pub fn db_filesize_in_pages(&self) -> &DatabaseFileSizeInPages {
    &self.db_filesize_in_pages
  }

  pub fn freelist_pages(&self) -> &FreeListPages {
    &self.freelist_pages
  }

  pub fn schema_cookie(&self) -> &SchemaCookie {
    &self.schema_cookie
  }

  pub fn schema_format(&self) -> &SchemaFormat {
    &self.schema_format
  }

  pub fn suggested_cache_size(&self) -> &SuggestedCacheSize {
    &self.suggested_cache_size
  }

  pub fn incremental_vacuum_settings(&self) -> &IncrementalVacuumSettings {
    &self.incremental_vacuum_settings
  }

  pub fn database_text_encoding(&self) -> &DatabaseTextEncoding {
    &self.database_text_encoding
  }

  pub fn user_version(&self) -> &UserVersion {
    &self.user_version
  }

  pub fn application_id(&self) -> &ApplicationId {
    &self.application_id
  }

  pub fn reserved_for_expansion(&self) -> &ReservedForExpansion {
    &self.reserved_for_expansion
  }

  pub fn version_valid_for(&self) -> &VersionValidFor {
    &self.version_valid_for
  }

  pub fn write_library_version(&self) -> &WriteLibraryVersion {
    &self.write_library_version
  }
}

impl_name! {SqliteHeader}

impl ParseBytes for SqliteHeader {
  const LENGTH_BYTES: usize = Self::LENGTH_BYTES;

  fn parsing_handler(bytes: &[u8]) -> crate::result::SqliteResult<Self> {
    let magic_header_string = MagicHeaderString::parse_bytes(&bytes[0..=15])?;
    let page_size = PageSize::parse_bytes(&bytes[16..=17])?;
    let file_format_version_numbers =
      FileFormatVersionNumbers::parse_bytes(&bytes[18..=19])?;
    let reserved_bytes_per_page =
      ReservedBytesPerPage::parse_bytes(&[bytes[20]])?;
    let payload_fractions = PayloadFractions::parse_bytes(&bytes[21..=23])?;

    let file_change_counter = FileChangeCounter::parse_bytes(&bytes[24..=27])?;
    let db_filesize_in_pages =
      DatabaseFileSizeInPages::parse_bytes(&bytes[28..=31])?;

    let freelist_pages = FreeListPages::parse_bytes(&bytes[32..=39])?;

    let schema_cookie = SchemaCookie::parse_bytes(&bytes[40..=43])?;

    let schema_format = SchemaFormat::parse_bytes(&bytes[44..=47])?;

    let suggested_cache_size =
      SuggestedCacheSize::parse_bytes(&bytes[48..=51])?;

    let largest_root_btree_page =
      incremental_vacuum_settings::LargestRootBtreePage::parse_bytes(
        &bytes[52..=55],
      )?;

    let database_text_encoding =
      DatabaseTextEncoding::parse_bytes(&bytes[56..=59])?;

    let user_version = UserVersion::parse_bytes(&bytes[60..=63])?;

    let incremental_vacuum_mode =
      incremental_vacuum_settings::IncrementalVacuumMode::parse_bytes(
        &bytes[64..=67],
      )?;

    let application_id = ApplicationId::parse_bytes(&bytes[68..=71])?;

    let reserved_for_expansion =
      ReservedForExpansion::parse_bytes(&bytes[72..=91])?;

    let version_valid_for = VersionValidFor::parse_bytes(&bytes[92..=95])?;

    let write_library_version =
      WriteLibraryVersion::parse_bytes(&bytes[96..=99])?;

    Ok(Self {
      magic_header_string,
      page_size,
      file_format_version_numbers,
      reserved_bytes_per_page,
      payload_fractions,
      file_change_counter,
      db_filesize_in_pages,
      freelist_pages,
      schema_cookie,
      schema_format,
      suggested_cache_size,
      incremental_vacuum_settings: IncrementalVacuumSettings {
        largest_root_btree_page,
        incremental_vacuum_mode,
      },
      database_text_encoding,
      user_version,
      application_id,
      reserved_for_expansion,
      version_valid_for,
      write_library_version,
    })
  }
}

impl ValidateParsed for SqliteHeader {
  fn validate_parsed(&self) -> SqliteResult<()> {
    {
      //  The usable size is not allowed to be less than 480. In other words, if
      // the page size is 512, then the reserved space size cannot exceed 32.
      const MINIMUM_USABLE_SIZE: u32 = 480;
      if (u32::from(self.page_size())
        - u32::from(**self.reserved_bytes_per_page()))
        < MINIMUM_USABLE_SIZE
      {
        return Err(SqliteError::HeaderValidationError(
          "The usable size is not allowed to be less than 480.".into(),
        ));
      }
    }

    {
      //  The in-header database size is only considered to be valid if it is
      // non-zero and if the 4-byte change counter at offset 24 exactly matches
      // the 4-byte version-valid-for number at offset 92. The in-header database
      // size is always valid when the database is only modified using recent
      // versions of Sqlite, versions 3.7.0 (2010-07-21) and later.
      if **self.db_filesize_in_pages() < 1 {
        return Err(SqliteError::HeaderValidationError(
          "The in-header database size is not valid".into(),
        ));
      }
      if **self.file_change_counter() != **self.version_valid_for() {
        return Err(SqliteError::HeaderValidationError(
        "The change counter must exactly matches the version-valid-for number".into(),
      ));
      }
      //  If a legacy version of Sqlite writes to the database, it will not know
      // to update the in-header database size and so the in-header database
      // size could be incorrect. But legacy versions of Sqlite will also leave
      // the version-valid-for number at offset 92 unchanged so it will not
      // match the change-counter. Hence, invalid in-header database sizes can
      // be detected (and ignored) by observing when the change-counter does not
      // match the version-valid-for number.}
    }
    // TODO: Free page list

    // TODO: Schema Cookie

    {
      //  New database files created by Sqlite use format 4 by default. The
      // legacy_file_format pragma can be used to cause Sqlite to create new
      // database files using format 1. The format version number can be made to
      // default to 1 instead of 4 by setting SQLITE_DEFAULT_FILE_FORMAT=1 at
      // compile-time.
      if *self.schema_format() != SchemaFormat::Format4 {
        return Err(SqliteError::HeaderValidationError(
          "Only Schema format 4 is supported".into(),
        ));
      }
    }

    {
      //  Unused pages in the database file are stored on a freelist. The 4-byte
      // big-endian integer at offset 32 stores the page number of the first
      // page of the freelist, or zero if the freelist is empty. The 4-byte
      // big-endian integer at offset 36 stores the total number of pages on the
      // freelist.
      let freelist_pages = self.freelist_pages();
      if (**freelist_pages.total() == 0) && (**freelist_pages.first() != 0) {
        return Err(SqliteError::HeaderValidationError(
          "Free list settings may be corrupted".into(),
        ));
      }
    }
    {
      //  If the integer at offset 52 is non-zero then it is the page number of
      // the largest root page in the database file, the database file will
      // contain ptrmap pages, and the mode must be either auto_vacuum or
      // incremental_vacuum. In this latter case, the integer at offset 64 is
      // true for incremental_vacuum and false for auto_vacuum. If the integer
      // at offset 52 is zero then the integer at offset 64 must also be zero.
      let incremental_vacuum_mode =
        u32::from(self.incremental_vacuum_settings.incremental_vacuum_mode());
      let largest_root_btree_page =
        **self.incremental_vacuum_settings.largest_root_btree_page();
      if incremental_vacuum_mode == 0 && largest_root_btree_page != 0 {
        return Err(SqliteError::HeaderValidationError(
          "Incremental vacuum settings is zero but corrupted".into(),
        ));
      }
    }
    {
      //  The 4-byte big-endian integer at offset 92 is the value of the change
      // counter when the version number was stored. The integer at offset 92
      // indicates which transaction the version number is valid for and is
      // sometimes called the "version-valid-for number".
      if **self.file_change_counter() < 1 {
        return Err(SqliteError::HeaderValidationError(
          "File change counter maybe corrupted".into(),
        ));
      }
      if **self.file_change_counter() < **self.version_valid_for() {
        return Err(SqliteError::HeaderValidationError(
          "The version-valid-for number or the change counter maybe corrupted"
            .into(),
        ));
      }
    }

    Ok(())
  }
}

impl TryFrom<&[u8]> for SqliteHeader {
  type Error = SqliteError;

  fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
    let parsed = Self::parse_bytes(bytes)?;
    parsed.validate_parsed()?;
    Ok(parsed)
  }
}
