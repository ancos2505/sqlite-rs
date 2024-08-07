//! ### The Lock-Byte Page
//!
//!  The lock-byte page is the **single page of the database file** that
//! contains the bytes at offsets between `1073741824` and `1073742335`,
//! inclusive. A database file that is less than or equal to `1073741824` bytes
//! in size contains **no lock-byte page**. A database file larger than
//! `1073741824` contains **exactly one lock-byte page**.
//!
//!  The lock-byte page is set aside for use by the operating-system specific
//! VFS implementation in implementing the database file locking primitives.
//! SQLite does not use the lock-byte page. The SQLite core will never read or
//! write the lock-byte page, though operating-system specific VFS
//! implementations may choose to read or write bytes on the lock-byte page
//! according to the needs and proclivities of the underlying system. The unix
//! and win32 VFS implementations that come built into SQLite do not write to
//! the lock-byte page, but third-party VFS implementations for other operating
//! systems might.
//!
//!  The lock-byte page arose from the need to support Win95 which was the
//! predominant operating system when this file format was designed and which
//! only supported mandatory file locking. All modern operating systems that
//! we know of support advisory file locking, and so the lock-byte page is not
//! really needed any more, but is retained for backwards compatibility.
