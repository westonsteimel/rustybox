use libc;
extern "C" {
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
}

use libc::gid_t;
use libc::mode_t;
use libc::off_t;
use libc::time_t;
use libc::uid_t;

use crate::archival::libarchive::bb_archive::file_header_t;

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn header_list(mut file_header: *const file_header_t) {
  //TODO: cpio -vp DIR should output "DIR/NAME", not just "NAME" */
  puts((*file_header).name);
}
