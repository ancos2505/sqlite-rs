//! ### The Freelist
//!
//!  A database file might contain one or more pages that are not in active use.
//! Unused pages can come about, for example, when information is deleted from
//! the database. Unused pages are stored on the freelist and are reused when
//! additional pages are required.
//!
//!  The freelist is organized as a linked list of freelist trunk pages with
//! each trunk page containing page numbers for zero or more freelist leaf
//! pages.
//!
//!  A freelist trunk page consists of an array of 4-byte big-endian integers.
//! The size of the array is as many integers as will fit in the usable space of
//! a page. The minimum usable space is 480 bytes so the array will always be at
//! least 120 entries in length. The first integer on a freelist trunk page is
//! the page number of the next freelist trunk page in the list or zero if this
//! is the last freelist trunk page. The second integer on a freelist trunk page
//! is the number of leaf page pointers to follow. Call the second integer on a
//! freelist trunk page L. If L is greater than zero then integers with array
//! indexes between 2 and L+1 inclusive contain page numbers for freelist leaf
//! pages.
//!
//!  Freelist leaf pages contain no information. SQLite avoids reading or
//! writing freelist leaf pages in order to reduce disk I/O.
//!
//!  A bug in SQLite versions prior to 3.6.0 (2008-07-16) caused the database to
//! be reported as corrupt if any of the last 6 entries in the freelist trunk
//! page array contained non-zero values. Newer versions of SQLite do not have
//! this problem. However, newer versions of SQLite still avoid using the last
//! six entries in the freelist trunk page array in order that database files
//! created by newer versions of SQLite can be read by older versions of SQLite.
//!
//!  The number of freelist pages is stored as a 4-byte big-endian integer in
//! the database header at an offset of 36 from the beginning of the file. The
//! database header also stores the page number of the first freelist trunk page
//! as a 4-byte big-endian integer at an offset of 32 from the beginning of the
//! file.
