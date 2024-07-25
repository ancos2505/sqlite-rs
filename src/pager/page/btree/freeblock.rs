//! # Freeblock
//!
//!  A freeblock is a structure used to identify unallocated space within a
//! b-tree page. Freeblocks are organized as a chain. The first 2 bytes of a
//! freeblock are a big-endian integer which is the offset in the b-tree page of
//! the next freeblock in the chain, or zero if the freeblock is the last on the
//! chain. The third and fourth bytes of each freeblock form a big-endian
//! integer which is the size of the freeblock in bytes, including the 4-byte
//! header. Freeblocks are always connected in order of increasing offset. The
//! second field of the b-tree page header is the offset of the first freeblock,
//! or zero if there are no freeblocks on the page. In a well-formed b-tree
//! page, there will always be at least one cell before the first freeblock.
//!
//!  A freeblock requires at least 4 bytes of space. If there is an isolated
//! group of 1, 2, or 3 unused bytes within the cell content area, those bytes
//! comprise a fragment. The total number of bytes in all fragments is stored in
//! the fifth field of the b-tree page header. In a well-formed b-tree page, the
//! total number of bytes in fragments may not exceed 60.
//!
//!  The total amount of free space on a b-tree page consists of the size of the
//! unallocated region plus the total size of all freeblocks plus the number of
//! fragmented free bytes. SQLite may from time to time reorganize a b-tree page
//! so that there are no freeblocks or fragment bytes, all unused bytes are
//! contained in the unallocated space region, and all cells are packed tightly
//! at the end of the page. This is called "defragmenting" the b-tree page.

#[derive(Debug)]
pub struct Freeblock;
