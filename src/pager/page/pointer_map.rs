//! ### Pointer Map or Ptrmap Pages
//!
//!  Pointer map or ptrmap pages are extra pages inserted into the database to
//! make the operation of auto_vacuum and incremental_vacuum modes more
//! efficient. Other page types in the database typically have pointers from
//! parent to child. For example, an interior b-tree page contains pointers to
//! its child b-tree pages and an overflow chain has a pointer from earlier to
//! later links in the chain. A ptrmap page contains linkage information going
//! in the opposite direction, from child to parent.
//!
//!  Ptrmap pages must exist in any database file which has a non-zero largest
//! root b-tree page value at offset 52 in the database header. If the largest
//! root b-tree page value is zero, then the database must not contain ptrmap
//! pages.
//!
//!  In a database with ptrmap pages, the first ptrmap page is page 2. A ptrmap
//! page consists of an array of 5-byte entries. Let J be the number of 5-byte
//! entries that will fit in the usable space of a page.
//! (In other words, J=U/5.) The first ptrmap page will contain back pointer
//! information for pages 3 through J+2, inclusive. The second pointer map page
//! will be on page J+3 and that ptrmap page will provide back pointer
//! information for pages J+4 through 2*J+3 inclusive. And so forth for the
//! entire database file.
//!
//!  In a database that uses ptrmap pages, all pages at locations identified by
//! the computation in the previous paragraph must be ptrmap page and no other
//! page may be a ptrmap page. Except, if the byte-lock page happens to fall on
//! the same page number as a ptrmap page, then the ptrmap is moved to the
//! following page for that one case.
//!
//!  Each 5-byte entry on a ptrmap page provides back-link information about one
//! of the pages that immediately follow the pointer map. If page B is a ptrmap
//! page then back-link information about page B+1 is provided by the first
//! entry on the pointer map. Information about page B+2 is provided by the
//! second entry. And so forth.
//!
//!  Each 5-byte ptrmap entry consists of one byte of "page type" information
//! followed by a 4-byte big-endian page number. Five page types are recognized:
//!
//! 1. A b-tree root page. The page number should be zero.
//! 2. A freelist page. The page number should be zero.
//! 3. The first page of a cell payload overflow chain. The page number is the
//!    b-tree page that contains the cell whose content has overflowed.
//! 4. A page in an overflow chain other than the first page. The page number is
//!    the prior page of the overflow chain.
//! 5. A non-root b-tree page. The page number is the parent b-tree page.
//!
//!  In any database file that contains ptrmap pages, all b-tree root pages must
//! come before any non-root b-tree page, cell payload overflow page, or
//! freelist page. This restriction ensures that a root page will never be moved
//!  during an auto-vacuum or incremental-vacuum. The auto-vacuum logic does not
//! know how to update the root_page field of the sqlite_schema table and so it
//! is necessary to prevent root pages from being moved during an auto-vacuum in
//! order to preserve the integrity of the sqlite_schema table. Root pages are
//! moved to the beginning of the database file by the CREATE TABLE, CREATE
//! INDEX, DROP TABLE, and DROP INDEX operations.
