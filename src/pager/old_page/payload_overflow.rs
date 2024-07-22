//! ### Cell Payload Overflow Pages
//!
//!  When the payload of a b-tree cell is too large for the b-tree page, the
//! surplus is spilled onto overflow pages. Overflow pages form a linked list.
//! The first four bytes of each overflow page are a big-endian integer which is
//! the page number of the next page in the chain, or zero for the final page in
//! the chain. The fifth byte through the last usable byte are used to hold
//! overflow content.
