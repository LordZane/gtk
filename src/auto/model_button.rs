// This file was generated by gir (https://github.com/gtk-rs/gir @ ea993ed)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Button;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use ButtonRole;
use Container;
use Widget;
use ffi;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use gio;
use glib;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::StaticType;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ModelButton(Object<ffi::GtkModelButton>): Button, Bin, Container, Widget, Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_model_button_get_type(),
    }
}

impl ModelButton {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn new() -> ModelButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_model_button_new()).downcast_unchecked()
        }
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
impl Default for ModelButton {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ModelButtonExt {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_property_active(&self) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_property_active(&self, active: bool);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_property_centered(&self) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_property_centered(&self, centered: bool);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_property_icon(&self) -> Option<gio::Icon>;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_property_icon<P: IsA<gio::Icon> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, icon: Option<&P>);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_property_iconic(&self) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_property_iconic(&self, iconic: bool);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_property_inverted(&self) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_property_inverted(&self, inverted: bool);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_property_menu_name(&self) -> Option<String>;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_property_menu_name(&self, menu_name: Option<&str>);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_property_role(&self) -> ButtonRole;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_property_role(&self, role: ButtonRole);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_property_text(&self) -> Option<String>;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_property_text(&self, text: Option<&str>);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_centered_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_iconic_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_menu_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_role_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ModelButton> + IsA<glib::object::Object>> ModelButtonExt for O {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_property_active(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "active".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_property_active(&self, active: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "active".to_glib_none().0, Value::from(&active).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_property_centered(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "centered".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_property_centered(&self, centered: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "centered".to_glib_none().0, Value::from(&centered).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_property_icon(&self) -> Option<gio::Icon> {
        unsafe {
            let mut value = Value::from_type(<gio::Icon as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "icon".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_property_icon<P: IsA<gio::Icon> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, icon: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon".to_glib_none().0, Value::from(icon).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_property_iconic(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "iconic".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_property_iconic(&self, iconic: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "iconic".to_glib_none().0, Value::from(&iconic).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_property_inverted(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "inverted".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_property_inverted(&self, inverted: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "inverted".to_glib_none().0, Value::from(&inverted).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_property_menu_name(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "menu-name".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_property_menu_name(&self, menu_name: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "menu-name".to_glib_none().0, Value::from(menu_name).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_property_role(&self) -> ButtonRole {
        unsafe {
            let mut value = Value::from_type(<ButtonRole as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "role".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_property_role(&self, role: ButtonRole) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "role".to_glib_none().0, Value::from(&role).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_property_text(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "text".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_property_text(&self, text: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "text".to_glib_none().0, Value::from(text).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::active",
                transmute(notify_active_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_centered_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::centered",
                transmute(notify_centered_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::icon",
                transmute(notify_icon_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_iconic_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::iconic",
                transmute(notify_iconic_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::inverted",
                transmute(notify_inverted_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_menu_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::menu-name",
                transmute(notify_menu_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_role_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::role",
                transmute(notify_role_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::text",
                transmute(notify_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_active_trampoline<P>(this: *mut ffi::GtkModelButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ModelButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ModelButton::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_centered_trampoline<P>(this: *mut ffi::GtkModelButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ModelButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ModelButton::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_icon_trampoline<P>(this: *mut ffi::GtkModelButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ModelButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ModelButton::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_iconic_trampoline<P>(this: *mut ffi::GtkModelButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ModelButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ModelButton::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_inverted_trampoline<P>(this: *mut ffi::GtkModelButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ModelButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ModelButton::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_menu_name_trampoline<P>(this: *mut ffi::GtkModelButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ModelButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ModelButton::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_role_trampoline<P>(this: *mut ffi::GtkModelButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ModelButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ModelButton::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_text_trampoline<P>(this: *mut ffi::GtkModelButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ModelButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ModelButton::from_glib_borrow(this).downcast_unchecked())
}
