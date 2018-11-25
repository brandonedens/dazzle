// Copyright 2018 by Brandon Edens.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Author: Brandon Edens <brandonedens@gmail.com>
// Date: 2018-11-24

#[macro_use]
extern crate log;

use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use fern::colors::{Color, ColoredLevelConfig};

use fuse_mt::*;
use time::*;

/// Setup system logging.
fn set_up_logging() {
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::Green)
        .trace(Color::BrightBlack);
    let colors_level = colors_line.clone().info(Color::Green);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{date}][{target}][{level}{color_line}] {message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors_line.get_color(&record.level()).to_fg_str()
                ),
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message,
            ));
        })
        .level(log::LevelFilter::Warn)
        .level_for("bedfs", log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
    debug!("Logging initialized.");
}

fn main() -> std::io::Result<()> {
    set_up_logging();

    let args: Vec<OsString> = env::args_os().collect();
    if args.len() != 2 {
        println!("usage: {} <mountpoint>", &env::args().next().unwrap());
        ::std::process::exit(-1);
    }

    let filesystem = DazzleFs;
    let fuse_args: Vec<&OsStr> = vec![&OsStr::new("-o"), &OsStr::new("auto_unmount")];
    fuse_mt::mount(fuse_mt::FuseMT::new(filesystem, 1), &args[1], &fuse_args)?;
    Ok(())
}

pub struct DazzleFs;

impl FilesystemMT for DazzleFs {
    fn init(&self, _req: RequestInfo) -> ResultEmpty {
        debug!("init");
        Ok(())
    }

    fn destroy(&self, _req: RequestInfo) {
        debug!("destroy");
    }

    fn getattr(&self, _req: RequestInfo, path: &Path, fh: Option<u64>) -> ResultEntry {
        debug!("getattr: {:?}", path);
        Err(libc::ENOSYS)
    }

    fn opendir(&self, _req: RequestInfo, path: &Path, _flags: u32) -> ResultOpen {
        debug!("opendir");
        Err(libc::ENOSYS)
    }

    fn releasedir(&self, _req: RequestInfo, path: &Path, fh: u64, _flags: u32) -> ResultEmpty {
        debug!("releasedir: {:?}", path);
        Err(libc::ENOSYS)
    }

    fn readdir(&self, _req: RequestInfo, path: &Path, fh: u64) -> ResultReaddir {
        debug!("readdir: {:?}", path);
        Err(libc::ENOSYS)
    }

    fn open(&self, _req: RequestInfo, path: &Path, flags: u32) -> ResultOpen {
        debug!("open: {:?} flags={:#x}", path, flags);
        Err(libc::ENOSYS)
    }

    fn release(
        &self,
        _req: RequestInfo,
        path: &Path,
        fh: u64,
        _flags: u32,
        _lock_owner: u64,
        _flush: bool,
    ) -> ResultEmpty {
        debug!("release: {:?}", path);
        Err(libc::ENOSYS)
    }

    fn read(&self, _req: RequestInfo, path: &Path, fh: u64, offset: u64, size: u32) -> ResultData {
        debug!("read: {:?} {:#x} @ {:#x}", path, size, offset);
        Err(libc::ENOSYS)
    }

    fn write(
        &self,
        _req: RequestInfo,
        path: &Path,
        fh: u64,
        offset: u64,
        data: Vec<u8>,
        _flags: u32,
    ) -> ResultWrite {
        debug!("write: {:?} {:#x} @ {:#x}", path, data.len(), offset);
        Err(libc::ENOSYS)
    }

    fn flush(&self, _req: RequestInfo, path: &Path, fh: u64, _lock_owner: u64) -> ResultEmpty {
        debug!("flush: {:?}", path);
        Err(libc::ENOSYS)
    }

    fn fsync(&self, _req: RequestInfo, path: &Path, fh: u64, datasync: bool) -> ResultEmpty {
        debug!("fsync: {:?}, data={:?}", path, datasync);
        Err(libc::ENOSYS)
    }

    fn chmod(&self, _req: RequestInfo, path: &Path, fh: Option<u64>, mode: u32) -> ResultEmpty {
        debug!("chown: {:?} to {:#o}", path, mode);
        Err(libc::ENOSYS)
    }

    fn chown(
        &self,
        _req: RequestInfo,
        path: &Path,
        fh: Option<u64>,
        uid: Option<u32>,
        gid: Option<u32>,
    ) -> ResultEmpty {
        let uid = uid.unwrap_or(::std::u32::MAX); // docs say "-1", but uid_t is unsigned
        let gid = gid.unwrap_or(::std::u32::MAX); // ditto for gid_t
        debug!("chmod: {:?} to {}:{}", path, uid, gid);
        Err(libc::ENOSYS)
    }

    fn truncate(&self, _req: RequestInfo, path: &Path, fh: Option<u64>, size: u64) -> ResultEmpty {
        debug!("truncate: {:?} to {:#x}", path, size);
        Err(libc::ENOSYS)
    }

    fn utimens(
        &self,
        _req: RequestInfo,
        path: &Path,
        fh: Option<u64>,
        atime: Option<Timespec>,
        mtime: Option<Timespec>,
    ) -> ResultEmpty {
        debug!("utimens: {:?}: {:?}, {:?}", path, atime, mtime);
        Err(libc::ENOSYS)
    }

    fn readlink(&self, _req: RequestInfo, path: &Path) -> ResultData {
        debug!("readlink: {:?}", path);
        Err(libc::ENOSYS)
    }

    fn statfs(&self, _req: RequestInfo, path: &Path) -> ResultStatfs {
        debug!("statfs: {:?}", path);
        Err(libc::ENOSYS)
    }

    fn fsyncdir(&self, _req: RequestInfo, path: &Path, fh: u64, datasync: bool) -> ResultEmpty {
        debug!("fsyncdir: {:?} (datasync = {:?})", path, datasync);
        Err(libc::ENOSYS)
    }

    fn mknod(
        &self,
        _req: RequestInfo,
        parent_path: &Path,
        name: &OsStr,
        mode: u32,
        rdev: u32,
    ) -> ResultEntry {
        debug!(
            "mknod: {:?}/{:?} (mode={:#o}, rdev={})",
            parent_path, name, mode, rdev
        );
        Err(libc::ENOSYS)
    }

    fn mkdir(&self, _req: RequestInfo, parent_path: &Path, name: &OsStr, mode: u32) -> ResultEntry {
        debug!("mkdir {:?}/{:?} (mode={:#o})", parent_path, name, mode);
        Err(libc::ENOSYS)
    }

    fn unlink(&self, _req: RequestInfo, parent_path: &Path, name: &OsStr) -> ResultEmpty {
        debug!("unlink {:?}/{:?}", parent_path, name);
        Err(libc::ENOSYS)
    }

    fn rmdir(&self, _req: RequestInfo, parent_path: &Path, name: &OsStr) -> ResultEmpty {
        debug!("rmdir: {:?}/{:?}", parent_path, name);
        Err(libc::ENOSYS)
    }

    fn symlink(
        &self,
        _req: RequestInfo,
        parent_path: &Path,
        name: &OsStr,
        target: &Path,
    ) -> ResultEntry {
        debug!("symlink: {:?}/{:?} -> {:?}", parent_path, name, target);
        Err(libc::ENOSYS)
    }

    fn rename(
        &self,
        _req: RequestInfo,
        parent_path: &Path,
        name: &OsStr,
        newparent_path: &Path,
        newname: &OsStr,
    ) -> ResultEmpty {
        debug!(
            "rename: {:?}/{:?} -> {:?}/{:?}",
            parent_path, name, newparent_path, newname
        );
        Err(libc::ENOSYS)
    }

    fn link(
        &self,
        _req: RequestInfo,
        path: &Path,
        newparent: &Path,
        newname: &OsStr,
    ) -> ResultEntry {
        debug!("link: {:?} -> {:?}/{:?}", path, newparent, newname);
        Err(libc::ENOSYS)
    }

    fn create(
        &self,
        _req: RequestInfo,
        parent: &Path,
        name: &OsStr,
        mode: u32,
        flags: u32,
    ) -> ResultCreate {
        debug!(
            "create: {:?}/{:?} (mode={:#o}, flags={:#x})",
            parent, name, mode, flags
        );
        Err(libc::ENOSYS)
    }

    fn listxattr(&self, _req: RequestInfo, path: &Path, size: u32) -> ResultXattr {
        debug!("listxattr: {:?}", path);
        Err(libc::ENOSYS)
    }

    fn getxattr(&self, _req: RequestInfo, path: &Path, name: &OsStr, size: u32) -> ResultXattr {
        debug!("getxattr: {:?} {:?} {}", path, name, size);
        Err(libc::ENOSYS)
    }

    fn setxattr(
        &self,
        _req: RequestInfo,
        path: &Path,
        name: &OsStr,
        value: &[u8],
        flags: u32,
        position: u32,
    ) -> ResultEmpty {
        debug!(
            "setxattr: {:?} {:?} {} bytes, flags = {:#x}, pos = {}",
            path,
            name,
            value.len(),
            flags,
            position
        );
        Err(libc::ENOSYS)
    }

    fn removexattr(&self, _req: RequestInfo, path: &Path, name: &OsStr) -> ResultEmpty {
        debug!("removexattr: {:?} {:?}", path, name);
        Err(libc::ENOSYS)
    }

    #[cfg(target_os = "macos")]
    fn setvolname(&self, _req: RequestInfo, name: &OsStr) -> ResultEmpty {
        info!("setvolname: {:?}", name);
        Err(libc::ENOTSUP)
    }

    #[cfg(target_os = "macos")]
    fn getxtimes(&self, _req: RequestInfo, path: &Path) -> ResultXTimes {
        debug!("getxtimes: {:?}", path);
        Err(libc::ENOSYS)
    }
}
