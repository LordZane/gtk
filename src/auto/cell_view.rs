// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use CellArea;
use CellAreaContext;
use CellLayout;
use Orientable;
use TreeModel;
use TreePath;
use Widget;
use ffi;
use gdk;
use gdk_pixbuf;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct CellView(Object<ffi::GtkCellView, ffi::GtkCellViewClass>): Widget, Buildable, CellLayout, Orientable;

    match fn {
        get_type => || ffi::gtk_cell_view_get_type(),
    }
}

impl CellView {
    pub fn new() -> CellView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new()).downcast_unchecked()
        }
    }

    pub fn new_with_context<P: IsA<CellArea>>(area: &P, context: &CellAreaContext) -> CellView {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_context(area.to_glib_none().0, context.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_markup(markup: &str) -> CellView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_markup(markup.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_pixbuf(pixbuf: &gdk_pixbuf::Pixbuf) -> CellView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_pixbuf(pixbuf.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_text(text: &str) -> CellView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_text(text.to_glib_none().0)).downcast_unchecked()
        }
    }
}

impl Default for CellView {
    fn default() -> Self {
        Self::new()
    }
}

pub trait CellViewExt {
    fn get_displayed_row(&self) -> Option<TreePath>;

    fn get_draw_sensitive(&self) -> bool;

    fn get_fit_model(&self) -> bool;

    fn get_model(&self) -> Option<TreeModel>;

    fn set_background_rgba(&self, rgba: &gdk::RGBA);

    fn set_displayed_row(&self, path: &mut TreePath);

    fn set_draw_sensitive(&self, draw_sensitive: bool);

    fn set_fit_model(&self, fit_model: bool);

    fn set_model<'a, P: IsA<TreeModel> + 'a, Q: Into<Option<&'a P>>>(&self, model: Q);

    fn set_property_background(&self, background: Option<&str>);

    fn get_property_background_rgba(&self) -> Option<gdk::RGBA>;

    fn get_property_background_set(&self) -> bool;

    fn set_property_background_set(&self, background_set: bool);

    fn get_property_cell_area(&self) -> Option<CellArea>;

    fn get_property_cell_area_context(&self) -> Option<CellAreaContext>;

    fn connect_property_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_background_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_background_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cell_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cell_area_context_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_draw_sensitive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_fit_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellView> + IsA<glib::object::Object>> CellViewExt for O {
    fn get_displayed_row(&self) -> Option<TreePath> {
        unsafe {
            from_glib_full(ffi::gtk_cell_view_get_displayed_row(self.to_glib_none().0))
        }
    }

    fn get_draw_sensitive(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_view_get_draw_sensitive(self.to_glib_none().0))
        }
    }

    fn get_fit_model(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_view_get_fit_model(self.to_glib_none().0))
        }
    }

    fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(ffi::gtk_cell_view_get_model(self.to_glib_none().0))
        }
    }

    fn set_background_rgba(&self, rgba: &gdk::RGBA) {
        unsafe {
            ffi::gtk_cell_view_set_background_rgba(self.to_glib_none().0, rgba.to_glib_none().0);
        }
    }

    fn set_displayed_row(&self, path: &mut TreePath) {
        unsafe {
            ffi::gtk_cell_view_set_displayed_row(self.to_glib_none().0, path.to_glib_none_mut().0);
        }
    }

    fn set_draw_sensitive(&self, draw_sensitive: bool) {
        unsafe {
            ffi::gtk_cell_view_set_draw_sensitive(self.to_glib_none().0, draw_sensitive.to_glib());
        }
    }

    fn set_fit_model(&self, fit_model: bool) {
        unsafe {
            ffi::gtk_cell_view_set_fit_model(self.to_glib_none().0, fit_model.to_glib());
        }
    }

    fn set_model<'a, P: IsA<TreeModel> + 'a, Q: Into<Option<&'a P>>>(&self, model: Q) {
        let model = model.into();
        let model = model.to_glib_none();
        unsafe {
            ffi::gtk_cell_view_set_model(self.to_glib_none().0, model.0);
        }
    }

    fn set_property_background(&self, background: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "background".to_glib_none().0, Value::from(background).to_glib_none().0);
        }
    }

    fn get_property_background_rgba(&self) -> Option<gdk::RGBA> {
        unsafe {
            let mut value = Value::from_type(<gdk::RGBA as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "background-rgba".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_background_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "background-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_background_set(&self, background_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "background-set".to_glib_none().0, Value::from(&background_set).to_glib_none().0);
        }
    }

    fn get_property_cell_area(&self) -> Option<CellArea> {
        unsafe {
            let mut value = Value::from_type(<CellArea as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "cell-area".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_cell_area_context(&self) -> Option<CellAreaContext> {
        unsafe {
            let mut value = Value::from_type(<CellAreaContext as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "cell-area-context".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::background",
                transmute(notify_background_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_background_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::background-rgba",
                transmute(notify_background_rgba_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_background_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::background-set",
                transmute(notify_background_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_cell_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::cell-area",
                transmute(notify_cell_area_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_cell_area_context_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::cell-area-context",
                transmute(notify_cell_area_context_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_draw_sensitive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::draw-sensitive",
                transmute(notify_draw_sensitive_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_fit_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::fit-model",
                transmute(notify_fit_model_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::model",
                transmute(notify_model_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_background_trampoline<P>(this: *mut ffi::GtkCellView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellView> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_background_rgba_trampoline<P>(this: *mut ffi::GtkCellView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellView> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_background_set_trampoline<P>(this: *mut ffi::GtkCellView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellView> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_cell_area_trampoline<P>(this: *mut ffi::GtkCellView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellView> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_cell_area_context_trampoline<P>(this: *mut ffi::GtkCellView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellView> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_draw_sensitive_trampoline<P>(this: *mut ffi::GtkCellView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellView> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_fit_model_trampoline<P>(this: *mut ffi::GtkCellView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellView> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_model_trampoline<P>(this: *mut ffi::GtkCellView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellView> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellView::from_glib_borrow(this).downcast_unchecked())
}
