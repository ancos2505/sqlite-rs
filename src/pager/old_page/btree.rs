//! ### B-tree Pages
//!  The b-tree algorithm provides key/data storage with unique and ordered keys
//! on page-oriented storage devices. For background information on b-trees, see
//! Knuth, The Art Of Computer Programming, Volume 3 "Sorting and Searching",
//! pages 471-479. Two variants of b-trees are used by SQLite. "Table b-trees"
//! use a 64-bit signed integer key and store all data in the leaves.
//! "Index b-trees" use arbitrary keys and store no data at all.
//!
//!  A b-tree page is either an interior page or a leaf page. A leaf page
//! contains keys and in the case of a table b-tree each key has associated
//! data. An interior page contains K keys together with K+1 pointers to child
//! b-tree pages. A "pointer" in an interior b-tree page is just the 32-bit
//! unsigned integer page number of the child page.
//!
//!  The number of keys on an interior b-tree page, K, is almost always at least
//! 2 and is usually much more than 2. The only exception is when page 1 is an
//! interior b-tree page. Page 1 has 100 fewer bytes of storage space available,
//! due to the presence of the database header at the beginning of that page,
//! and so sometimes (rarely) if page 1 is an interior b-tree page, it can end
//! up holding just a single key. In all other cases, K is 2 or more. The upper
//! bound on K is as many keys as will fit on the page. Large keys on index
//! b-trees are split up into overflow pages so that no single key uses more
//! than one fourth of the available storage space on the page and hence every
//! internal page is able to store at least 4 keys. The integer keys of table
//! b-trees are never large enough to require overflow, so key overflow only
//! occurs on index b-trees.
//!
//!  Define the depth of a leaf b-tree to be 1 and the depth of any interior
//! b-tree to be one more than the maximum depth of any of its children. In a
//! well-formed database, all children of an interior b-tree have the same
//! depth.
//!
//!  In an interior b-tree page, the pointers and keys logically alternate with
//! a pointer on both ends. (The previous sentence is to be understood
//! conceptually - the actual layout of the keys and pointers within the page is
//! more complicated and will be described in the sequel.) All keys within the
//! same page are unique and are logically organized in ascending order from
//! left to right. (Again, this ordering is logical, not physical. The actual
//! location of keys within the page is arbitrary.) For any key X, pointers to
//! the left of a X refer to b-tree pages on which all keys are less than or
//! equal to X. Pointers to the right of X refer to pages where all keys are
//! greater than X.
//!
//!  Within an interior b-tree page, each key and the pointer to its immediate
//! left are combined into a structure called a "cell". The right-most pointer
//! is held separately. A leaf b-tree page has no pointers, but it still uses
//! the cell structure to hold keys for index b-trees or keys and content for
//! table b-trees. Data is also contained in the cell.
//!
//!  Every b-tree page has at most one parent b-tree page. A b-tree page without
//! a parent is called a root page. A root b-tree page together with the closure
//! of its children form a complete b-tree. It is possible (and in fact rather
//! common) to have a complete b-tree that consists of a single page that is
//! both a leaf and the root. Because there are pointers from parents to
//! children, every page of a complete b-tree can be located if only the root
//! page is known. Hence, b-trees are identified by their root page number.
//!
//!  A b-tree page is either a table b-tree page or an index b-tree page. All
//! pages within each complete b-tree are of the same type: either table or
//! index. There is one table b-trees in the database file for each rowid table
//! in the database schema, including system tables such as sqlite_schema. There
//! is one index b-tree in the database file for each index in the schema,
//! including implied indexes created by uniqueness constraints. There are no
//! b-trees associated with virtual tables. Specific virtual table
//! implementations might make use of shadow tables for storage, but those
//! shadow tables will have separate entries in the database schema. WITHOUT
//! ROWID tables use index b-trees rather than a table b-trees, so there is one
//! index b-tree in the database file for each WITHOUT ROWID table. The b-tree
//! corresponding to the sqlite_schema table is always a table b-tree and always
//! has a root page of 1. The sqlite_schema table contains the root page number
//! for every other table and index in the database file.
//!
//!  Each entry in a table b-tree consists of a 64-bit signed integer key and up
//! to 2147483647 bytes of arbitrary data. (The key of a table b-tree
//! corresponds to the rowid of the SQL table that the b-tree implements.)
//! Interior table b-trees hold only keys and pointers to children. All data is
//! contained in the table b-tree leaves.
//!
//!  Each entry in an index b-tree consists of an arbitrary key of up to
//! 2147483647 bytes in length and no data.
//!
//!  Define the "payload" of a cell to be the arbitrary length section of the
//! cell. For an index b-tree, the key is always arbitrary in length and hence
//! the payload is the key. There are no arbitrary length elements in the cells
//! of interior table b-tree pages and so those cells have no payload. Table
//! b-tree leaf pages contain arbitrary length content and so for cells on those
//! pages the payload is the content.
//!
//!  When the size of payload for a cell exceeds a certain threshold (to be
//! defined later) then only the first few bytes of the payload are stored on
//! the b-tree page and the balance is stored in a linked list of content
//! overflow pages.
//!
//!  A b-tree page is divided into regions in the following order:
//!
//! 1. The 100-byte database file header (found on page 1 only)
//! 2. The 8 or 12 byte b-tree page header
//! 3. The cell pointer array
//! 4. Unallocated space
//! 5. The cell content area
//! 6. The reserved region.
//!
//!  The 100-byte database file header is found only on page 1, which is always
//! a table b-tree page. All other b-tree pages in the database file omit this
//! 100-byte header.
//!
//!  The reserved region is an area of unused space at the end of every page
//! (except the locking page) that extensions can use to hold per-page
//! information. The size of the reserved region is determined by the one-byte
//! unsigned integer found at an offset of 20 into the database file header. The
//! size of the reserved region is usually zero.
