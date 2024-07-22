//! Tests for Pager
//!
//! To run: `cargo test pager::page`

use super::Page;
use crate::header::PageSize;

#[test]

fn ok_on_parse_page() {
  let page_size = PageSize::L512;

  const BUF_SIZE: usize = 512;
  let buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
  let page = Page::parse(page_size, buf);

  assert!(page.is_ok());
}
