// This file was generated by gir (https://github.com/gtk-rs/gir @ ea993ed)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use Widget;
use Window;
use ffi;
use gdk;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct WindowGroup(Object<ffi::GtkWindowGroup, ffi::GtkWindowGroupClass>);

    match fn {
        get_type => || ffi::gtk_window_group_get_type(),
    }
}

impl WindowGroup {
    pub fn new() -> WindowGroup {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_window_group_new())
        }
    }
}

impl Default for WindowGroup {
    fn default() -> Self {
        Self::new()
    }
}

pub trait WindowGroupExt {
    fn add_window<P: IsA<Window>>(&self, window: &P);

    fn get_current_device_grab<P: IsA<gdk::Device>>(&self, device: &P) -> Option<Widget>;

    fn get_current_grab(&self) -> Option<Widget>;

    fn list_windows(&self) -> Vec<Window>;

    fn remove_window<P: IsA<Window>>(&self, window: &P);
}

impl<O: IsA<WindowGroup>> WindowGroupExt for O {
    fn add_window<P: IsA<Window>>(&self, window: &P) {
        unsafe {
            ffi::gtk_window_group_add_window(self.to_glib_none().0, window.to_glib_none().0);
        }
    }

    fn get_current_device_grab<P: IsA<gdk::Device>>(&self, device: &P) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_window_group_get_current_device_grab(self.to_glib_none().0, device.to_glib_none().0))
        }
    }

    fn get_current_grab(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_window_group_get_current_grab(self.to_glib_none().0))
        }
    }

    fn list_windows(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_window_group_list_windows(self.to_glib_none().0))
        }
    }

    fn remove_window<P: IsA<Window>>(&self, window: &P) {
        unsafe {
            ffi::gtk_window_group_remove_window(self.to_glib_none().0, window.to_glib_none().0);
        }
    }
}
