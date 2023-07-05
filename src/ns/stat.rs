// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `POSIX stat` vocabulary
//!
//! This module requires `ns-stat` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|POSIX stat|
//! |**Prefix**|stat|
//! |**Namespace base IRI**|<http://www.w3.org/ns/posix/stat#>|
//! |**Description**|Describes terms for POSIX-like files and directory listings|
//! |**Is defined by**|<http://www.w3.org/ns/posix/stat>|
//!

use crate::namespace;

namespace!(
    base: "http://www.w3.org/ns/posix/stat#",

    terms: [
        /// `atime`: time of last access
        (atime, "atime"),
        /// `blksize`: blocksize for file system I/O
        (blksize, "blksize"),
        /// `blocks`: number of 512B blocks allocated
        (blocks, "blocks"),
        /// `ctime`: time of last status change
        (ctime, "ctime"),
        /// `dev`: ID of device containing file
        (dev, "dev"),
        /// `gid`: group ID of owner
        (gid, "gid"),
        /// `ino`: inode number
        (ino, "ino"),
        /// `mode`: protection
        (mode, "mode"),
        /// `mtime`: time of last modification
        (mtime, "mtime"),
        /// `nlink`: number of hard links
        (nlink, "nlink"),
        /// `rdev`: device ID (if special file)
        (rdev, "rdev"),
        /// `size`: total size, in bytes
        (size, "size"),
        /// `uid`: user ID of owner
        (uid, "uid")    ]
);
