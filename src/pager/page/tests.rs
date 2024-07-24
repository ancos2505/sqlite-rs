//! Tests for Pager
//!
//! To run: `cargo test pager::page`

use std::any::Any;

use super::Page;
use crate::{header::PageSize, pager::page::ValidPageSize};

#[test]
fn ok_on_parse_valid_pagesize() {
  const BUF_SIZE: usize = 512;
  
  assert_eq!(PageSize::L512, BUF_SIZE);

  let buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
  let page = Page::<BUF_SIZE>::parse(&buf).unwrap();

  let page_any = Box::new(page) as Box<dyn Any>;

  // let valid_pagesize = Box::new(page) as Box<dyn ValidPageSize>;

  let maybe_valid_pagesize =
    Page::<BUF_SIZE>::from_dyn(page_any).map(|data| -> Box<dyn ValidPageSize> { Box::new(data) });

  assert!(maybe_valid_pagesize.is_some())
}

/// ### Why Page was implemented in that way?
/// Because of compile-time checks
///
///
/// ```
/// cargo test pager::page --features show-ct-checks
/// ```
#[test]
fn show_the_power_of_ct_checks() {
  const BUF_SIZE: usize = 800;
  let buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
  let res = Page::<BUF_SIZE>::parse(&buf);
  dbg!(&res);
  assert!(res.is_ok());
  let page = res.unwrap();

  #[cfg(feature = "show-ct-checks")]
  let valid_pagesize = Box::new(page) as Box<dyn ValidPageSize>;

  // * should warn an error in compile-time

  #[cfg(feature = "show-ct-checks")]
  {
    let maybe_valid_pagesize =
      Page::<BUF_SIZE>::from_dyn(page_any).map(|data| -> Box<dyn ValidPageSize> { Box::new(data) });

    assert!(maybe_valid_pagesize.is_some())
  }

  let page_any = Box::new(page) as Box<dyn Any>;
  assert!(page_any.is::<Page<BUF_SIZE>>());
}
