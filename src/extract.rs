// Raw is a simple package manager written in rust, it aims to be compatible with the Pkgfiles written that works with pkgmk from pkgutils/cards
//    Copyright (C) 2026  Alexis/Delta-Azura

//    This program is free software; you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation; either version 2 of the License, or
//    (at your option) any later version.

//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.

//    You should have received a copy of the GNU General Public License along
//    with this program; if not, write to the Free Software Foundation, Inc.,
//    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.



use tar::Archive;
use bzip2::read::BzDecoder;
use xz2::read::XzDecoder;
use flate2::read::GzDecoder;
use std::fs::File;


pub fn extract(tarball: &String) {
    let source = File::open(tarball).unwrap();
    if tarball.ends_with(".tar.gz") || tarball.ends_with(".tgz") {
        let mut archive = Archive::new(GzDecoder::new(source));
        archive.unpack(".").unwrap();
    } else if tarball.ends_with(".tar.xz") {
        let mut archive = Archive::new(XzDecoder::new(source));
        archive.unpack(".").unwrap();
    } else if tarball.ends_with(".tar.bz2") {
        let mut archive = Archive::new(BzDecoder::new(source));
        archive.unpack(".").unwrap();
    } else if tarball.ends_with(".tar.zst") {
        let decoder = zstd::stream::read::Decoder::new(source).unwrap();
        let mut archive = Archive::new(decoder);
        archive.unpack(".").unwrap();
    }
}