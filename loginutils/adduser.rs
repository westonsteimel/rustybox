use crate::libbb::ptr_to_globals::bb_errno;
use crate::libpwdgrp::pwd_grp::bb_internal_getgrgid;
use crate::libpwdgrp::pwd_grp::bb_internal_getpwnam;
use crate::librb::smallint;
use libc;
use libc::chmod;
use libc::chown;
use libc::geteuid;
use libc::gid_t;
use libc::mode_t;
use libc::passwd;
use libc::time;
use libc::time_t;
use libc::uid_t;
use libc::umask;
extern "C" {

  #[no_mangle]
  fn execlp(__file: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn mkdir(__path: *const libc::c_char, __mode: mode_t) -> libc::c_int;

  /* Search for an entry with a matching user ID.  */

  /* Search for an entry with a matching username.  */

  /* Search for an entry with a matching group ID.  */

  /* Search for an entry with a matching group name.  */

  #[no_mangle]
  static mut option_mask32: u32;

  #[no_mangle]
  static mut logmode: smallint;

  #[no_mangle]
  static bb_msg_perm_denied_are_you_root: [libc::c_char; 0];
}

pub type C2RustUnnamed = libc::c_int;
pub const FILEUTILS_IGNORE_CHMOD_ERR: C2RustUnnamed = -2147483648;
pub const FILEUTILS_REFLINK_ALWAYS: C2RustUnnamed = 262144;
pub const FILEUTILS_REFLINK: C2RustUnnamed = 131072;
pub const FILEUTILS_RMDEST: C2RustUnnamed = 32768;
pub const FILEUTILS_NO_TARGET_DIR: C2RustUnnamed = 16384;
pub const FILEUTILS_UPDATE: C2RustUnnamed = 8192;
pub const FILEUTILS_VERBOSE: C2RustUnnamed = 4096;
pub const FILEUTILS_DEREFERENCE_L0: C2RustUnnamed = 256;
pub const FILEUTILS_DEREF_SOFTLINK: C2RustUnnamed = 128;
pub const FILEUTILS_MAKE_SOFTLINK: C2RustUnnamed = 64;
pub const FILEUTILS_MAKE_HARDLINK: C2RustUnnamed = 32;
pub const FILEUTILS_INTERACTIVE: C2RustUnnamed = 16;
pub const FILEUTILS_FORCE: C2RustUnnamed = 8;
pub const FILEUTILS_RECUR: C2RustUnnamed = 4;
pub const FILEUTILS_DEREFERENCE: C2RustUnnamed = 2;
pub const FILEUTILS_PRESERVE_STATUS: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed_0 = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed_0 = 2;
pub const LOGMODE_STDIO: C2RustUnnamed_0 = 1;
pub const LOGMODE_NONE: C2RustUnnamed_0 = 0;
/* remix */
/* recoded such that the uid may be passed in *p */
unsafe fn passwd_study(mut p: *mut passwd) {
  let mut max: libc::c_int = 60000i32;
  if !bb_internal_getpwnam((*p).pw_name).is_null() {
    crate::libbb::verror_msg::bb_error_msg_and_die(
      b"%s \'%s\' in use\x00" as *const u8 as *const libc::c_char,
      b"user\x00" as *const u8 as *const libc::c_char,
      (*p).pw_name,
    );
    /* this format string is reused in adduser and addgroup */
  }
  if option_mask32 & (1i32 << 7i32) as libc::c_uint == 0 {
    if option_mask32 & (1i32 << 5i32) as libc::c_uint != 0 {
      (*p).pw_uid = 100i32 as uid_t;
      max = 999i32
    } else {
      (*p).pw_uid = (999i32 + 1i32) as uid_t
    }
  }
  /* check for a free uid (and maybe gid) */
  while !crate::libpwdgrp::pwd_grp::bb_internal_getpwuid((*p).pw_uid).is_null()
    || (*p).pw_gid == -1i32 as gid_t && !bb_internal_getgrgid((*p).pw_uid).is_null()
  {
    if option_mask32 & (1i32 << 7i32) as libc::c_uint != 0 {
      /* -u N, cannot pick uid other than N: error */
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"%s \'%s\' in use\x00" as *const u8 as *const libc::c_char,
        b"uid\x00" as *const u8 as *const libc::c_char,
        crate::libbb::xfuncs::itoa((*p).pw_uid as libc::c_int),
      );
      /* this format string is reused in adduser and addgroup */
    }
    if (*p).pw_uid == max as libc::c_uint {
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"no %cids left\x00" as *const u8 as *const libc::c_char,
        'u' as i32,
      );
      /* this format string is reused in adduser and addgroup */
    } /* new gid = uid */
    (*p).pw_uid = (*p).pw_uid.wrapping_add(1)
  }
  if (*p).pw_gid == -1i32 as gid_t {
    (*p).pw_gid = (*p).pw_uid;
    if !crate::libpwdgrp::pwd_grp::bb_internal_getgrnam((*p).pw_name).is_null() {
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"%s \'%s\' in use\x00" as *const u8 as *const libc::c_char,
        b"group\x00" as *const u8 as *const libc::c_char,
        (*p).pw_name,
      );
      /* this format string is reused in adduser and addgroup */
    }
  };
}
unsafe fn addgroup_wrapper(mut p: *mut passwd, mut group_name: *const libc::c_char) -> libc::c_int {
  let mut argv: [*mut libc::c_char; 6] = [0 as *mut libc::c_char; 6];
  argv[0] = b"addgroup\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  if !group_name.is_null() {
    /* Add user to existing group */
    argv[1] = b"--\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[2] = (*p).pw_name;
    argv[3] = group_name as *mut libc::c_char;
    argv[4] = std::ptr::null_mut::<libc::c_char>()
  } else {
    /* Add user to his own group with the first free gid
     * found in passwd_study.
     */
    argv[1] = b"--gid\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[2] = crate::libbb::xfuncs::utoa((*p).pw_gid);
    argv[3] = b"--\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[4] = (*p).pw_name;
    argv[5] = std::ptr::null_mut::<libc::c_char>()
  }
  return crate::libbb::vfork_daemon_rexec::spawn_and_wait(argv.as_mut_ptr());
}
unsafe fn passwd_wrapper(mut login_name: *const libc::c_char) -> ! {
  execlp(
    b"passwd\x00" as *const u8 as *const libc::c_char,
    b"passwd\x00" as *const u8 as *const libc::c_char,
    b"--\x00" as *const u8 as *const libc::c_char,
    login_name,
    0 as *mut libc::c_void,
  );
  crate::libbb::verror_msg::bb_simple_error_msg_and_die(
    b"can\'t execute passwd, you must set password manually\x00" as *const u8
      as *const libc::c_char,
  );
}
//FIXME: upstream adduser has no short options! NOT COMPATIBLE!
static mut adduser_longopts: [libc::c_char; 110] = [
  104, 111, 109, 101, 0, 1, 104, 103, 101, 99, 111, 115, 0, 1, 103, 115, 104, 101, 108, 108, 0, 1,
  115, 105, 110, 103, 114, 111, 117, 112, 0, 1, 71, 100, 105, 115, 97, 98, 108, 101, 100, 45, 112,
  97, 115, 115, 119, 111, 114, 100, 0, 0, 68, 101, 109, 112, 116, 121, 45, 112, 97, 115, 115, 119,
  111, 114, 100, 0, 0, 68, 115, 121, 115, 116, 101, 109, 0, 0, 83, 110, 111, 45, 99, 114, 101, 97,
  116, 101, 45, 104, 111, 109, 101, 0, 0, 72, 117, 105, 100, 0, 1, 117, 115, 107, 101, 108, 0, 1,
  107, 0,
];
/*
 * adduser will take a login_name as its first parameter.
 * home, shell, gecos:
 * can be customized via command-line parameters.
 */
pub unsafe fn adduser_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut pw: passwd = passwd {
    pw_name: std::ptr::null_mut::<libc::c_char>(),
    pw_passwd: std::ptr::null_mut::<libc::c_char>(),
    pw_uid: 0,
    pw_gid: 0,
    pw_gecos: std::ptr::null_mut::<libc::c_char>(),
    pw_dir: std::ptr::null_mut::<libc::c_char>(),
    pw_shell: std::ptr::null_mut::<libc::c_char>(),
  };
  let mut usegroup: *const libc::c_char = std::ptr::null();
  let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut opts: libc::c_uint = 0;
  let mut uid: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut skel: *const libc::c_char = b"/etc/skel\x00" as *const u8 as *const libc::c_char;
  /* got root? */
  if geteuid() != 0 {
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(bb_msg_perm_denied_are_you_root.as_ptr());
  }
  pw.pw_gecos = b"Linux User,,,\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  /* We assume that newly created users "inherit" root's shell setting */
  pw.pw_shell =
    crate::libbb::xfuncs_printf::xstrdup(crate::libbb::get_shell_name::get_shell_name()); /* might come from getpwnam(), need to make a copy */
  pw.pw_dir = std::ptr::null_mut::<libc::c_char>();
  opts = crate::libbb::getopt32::getopt32long(
    argv,
    b"^h:g:s:G:DSHu:k:\x00-1:?2:SD\x00" as *const u8 as *const libc::c_char,
    adduser_longopts.as_ptr(),
    &mut pw.pw_dir as *mut *mut libc::c_char,
    &mut pw.pw_gecos as *mut *mut libc::c_char,
    &mut pw.pw_shell as *mut *mut libc::c_char,
    &mut usegroup as *mut *const libc::c_char,
    &mut uid as *mut *mut libc::c_char,
    &mut skel as *mut *const libc::c_char,
  );
  if opts & (1i32 << 7i32) as libc::c_uint != 0 {
    pw.pw_uid = crate::libbb::xatonum::xatou_range(uid, 0 as libc::c_uint, 60000i32 as libc::c_uint)
  }
  argv = argv.offset(optind as isize);
  pw.pw_name = *argv.offset(0);
  if opts == 0 && !(*argv.offset(1)).is_null() {
    /* if called with two non-option arguments, adduser
     * will add an existing user to an existing group.
     */
    return addgroup_wrapper(&mut pw, *argv.offset(1));
  }
  /* fill in the passwd struct */
  if pw.pw_dir.is_null() {
    /* create string for $HOME if not specified already */
    pw.pw_dir = crate::libbb::xfuncs_printf::xasprintf(
      b"/home/%s\x00" as *const u8 as *const libc::c_char,
      *argv.offset(0),
    )
  } /* exits on failure */
  pw.pw_passwd = b"x\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  if opts & (1i32 << 5i32) as libc::c_uint != 0 {
    if usegroup.is_null() {
      usegroup = b"nogroup\x00" as *const u8 as *const libc::c_char
    }
    if opts & (1i32 << 2i32) as libc::c_uint == 0 {
      pw.pw_shell = b"/bin/false\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
  }
  pw.pw_gid = if !usegroup.is_null() {
    crate::libbb::bb_pwd::xgroup2gid(usegroup)
  } else {
    -1i32 as libc::c_long
  } as gid_t;
  /* make sure everything is kosher and setup uid && maybe gid */
  passwd_study(&mut pw);
  p = crate::libbb::xfuncs_printf::xasprintf(
    b"x:%u:%u:%s:%s:%s\x00" as *const u8 as *const libc::c_char,
    pw.pw_uid,
    pw.pw_gid,
    pw.pw_gecos,
    pw.pw_dir,
    pw.pw_shell,
  );
  if crate::libbb::update_passwd::update_passwd(
    b"/etc/passwd\x00" as *const u8 as *const libc::c_char,
    pw.pw_name,
    p,
    0 as *const libc::c_char,
  ) < 0
  {
    return 1i32;
  }
  /* /etc/shadow fields:
   * 1. username
   * 2. encrypted password
   * 3. last password change (unix date (unix time/24*60*60))
   * 4. minimum days required between password changes
   * 5. maximum days password is valid
   * 6. days before password is to expire that user is warned
   * 7. days after password expires that account is disabled
   * 8. unix date when login expires (i.e. when it may no longer be used)
   */
  /* fields:     2 3  4 5     6 78 */
  p = crate::libbb::xfuncs_printf::xasprintf(
    b"!:%u:0:99999:7:::\x00" as *const u8 as *const libc::c_char,
    (time(0 as *mut time_t) as libc::c_uint).wrapping_div((24i32 * 60i32 * 60i32) as libc::c_uint),
  );
  /* ignore errors: if file is missing we suppose admin doesn't want it */
  crate::libbb::update_passwd::update_passwd(
    b"/etc/shadow\x00" as *const u8 as *const libc::c_char,
    pw.pw_name,
    p,
    0 as *const libc::c_char,
  );
  /* add to group */
  addgroup_wrapper(&mut pw, usegroup);
  /* clear the umask for this process so it doesn't
   * screw up the permissions on the mkdir and chown. */
  umask(0i32 as mode_t);
  if opts & (1i32 << 6i32) as libc::c_uint == 0 {
    /* set the owner and group so it is owned by the new user,
     * then fix up the permissions to 2755. Can't do it before
     * since chown will clear the setgid bit */
    let mut mkdir_err: libc::c_int = mkdir(pw.pw_dir, 0o755i32 as mode_t);
    if mkdir_err == 0 {
      /* New home. Copy /etc/skel to it */
      let mut args: [*const libc::c_char; 5] = [
        b"chown\x00" as *const u8 as *const libc::c_char,
        b"-R\x00" as *const u8 as *const libc::c_char,
        crate::libbb::xfuncs_printf::xasprintf(
          b"%u:%u\x00" as *const u8 as *const libc::c_char,
          pw.pw_uid as libc::c_int,
          pw.pw_gid as libc::c_int,
        ) as *const libc::c_char,
        pw.pw_dir as *const libc::c_char,
        0 as *const libc::c_char,
      ];
      /* Be silent on any errors (like: no /etc/skel) */
      if opts & (1i32 << 8i32) as libc::c_uint == 0 {
        logmode = LOGMODE_NONE as libc::c_int as smallint
      }
      crate::libbb::copy_file::copy_file(skel, pw.pw_dir, FILEUTILS_RECUR as libc::c_int);
      logmode = LOGMODE_STDIO as libc::c_int as smallint;
      crate::coreutils::chown::chown_main(4i32, args.as_mut_ptr() as *mut *mut libc::c_char);
    }
    if mkdir_err != 0 && *bb_errno != 17i32
      || chown(pw.pw_dir, pw.pw_uid, pw.pw_gid) != 0
      || chmod(pw.pw_dir, 0o2755i32 as mode_t) != 0
    {
      /* set setgid bit on homedir */
      crate::libbb::perror_msg::bb_simple_perror_msg(pw.pw_dir);
    }
  }
  if opts & (1i32 << 4i32) as libc::c_uint == 0 {
    /* interactively set passwd */
    passwd_wrapper(pw.pw_name);
  }
  return 0;
}
