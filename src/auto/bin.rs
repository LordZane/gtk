// This file was generated by gir (https://github.com/gtk-rs/gir @ ea993ed)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use Buildable;
use Container;
use Widget;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Bin(Object<ffi::GtkBin, ffi::GtkBinClass>): Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_bin_get_type(),
    }
}

pub trait BinExt {
    fn get_child(&self) -> Option<Widget>;
}

impl<O: IsA<Bin>> BinExt for O {
    fn get_child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_bin_get_child(self.to_glib_none().0))
        }
    }
}
