// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_webextension_sys;
use DOMDOMWindow;
use DOMEvent;
use DOMObject;

glib_wrapper! {
    pub struct DOMUIEvent(Object<webkit2_webextension_sys::WebKitDOMUIEvent, webkit2_webextension_sys::WebKitDOMUIEventClass, DOMUIEventClass>) @extends DOMEvent, DOMObject;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_ui_event_get_type(),
    }
}

pub const NONE_DOMUI_EVENT: Option<&DOMUIEvent> = None;

pub trait DOMUIEventExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_char_code(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_detail(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_key_code(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_layer_x(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_layer_y(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_page_x(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_page_y(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_view(&self) -> Option<DOMDOMWindow>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn init_ui_event<P: IsA<DOMDOMWindow>>(&self, type_: &str, canBubble: bool, cancelable: bool, view: &P, detail: libc::c_long);

    fn connect_property_char_code_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_detail_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_key_code_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_layer_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_layer_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_page_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_page_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMUIEvent>> DOMUIEventExt for O {
    fn get_char_code(&self) -> libc::c_long {
        unsafe {
            webkit2_webextension_sys::webkit_dom_ui_event_get_char_code(self.as_ref().to_glib_none().0)
        }
    }

    fn get_detail(&self) -> libc::c_long {
        unsafe {
            webkit2_webextension_sys::webkit_dom_ui_event_get_detail(self.as_ref().to_glib_none().0)
        }
    }

    fn get_key_code(&self) -> libc::c_long {
        unsafe {
            webkit2_webextension_sys::webkit_dom_ui_event_get_key_code(self.as_ref().to_glib_none().0)
        }
    }

    fn get_layer_x(&self) -> libc::c_long {
        unsafe {
            webkit2_webextension_sys::webkit_dom_ui_event_get_layer_x(self.as_ref().to_glib_none().0)
        }
    }

    fn get_layer_y(&self) -> libc::c_long {
        unsafe {
            webkit2_webextension_sys::webkit_dom_ui_event_get_layer_y(self.as_ref().to_glib_none().0)
        }
    }

    fn get_page_x(&self) -> libc::c_long {
        unsafe {
            webkit2_webextension_sys::webkit_dom_ui_event_get_page_x(self.as_ref().to_glib_none().0)
        }
    }

    fn get_page_y(&self) -> libc::c_long {
        unsafe {
            webkit2_webextension_sys::webkit_dom_ui_event_get_page_y(self.as_ref().to_glib_none().0)
        }
    }

    fn get_view(&self) -> Option<DOMDOMWindow> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_ui_event_get_view(self.as_ref().to_glib_none().0))
        }
    }

    fn init_ui_event<P: IsA<DOMDOMWindow>>(&self, type_: &str, canBubble: bool, cancelable: bool, view: &P, detail: libc::c_long) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_ui_event_init_ui_event(self.as_ref().to_glib_none().0, type_.to_glib_none().0, canBubble.to_glib(), cancelable.to_glib(), view.as_ref().to_glib_none().0, detail);
        }
    }

    fn connect_property_char_code_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_char_code_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMUIEvent, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMUIEvent>
        {
            let f: &F = &*(f as *const F);
            f(&DOMUIEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::char-code\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_char_code_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_detail_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_detail_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMUIEvent, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMUIEvent>
        {
            let f: &F = &*(f as *const F);
            f(&DOMUIEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::detail\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_detail_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_key_code_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_key_code_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMUIEvent, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMUIEvent>
        {
            let f: &F = &*(f as *const F);
            f(&DOMUIEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::key-code\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_key_code_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_layer_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_layer_x_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMUIEvent, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMUIEvent>
        {
            let f: &F = &*(f as *const F);
            f(&DOMUIEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::layer-x\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_layer_x_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_layer_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_layer_y_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMUIEvent, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMUIEvent>
        {
            let f: &F = &*(f as *const F);
            f(&DOMUIEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::layer-y\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_layer_y_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_page_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_page_x_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMUIEvent, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMUIEvent>
        {
            let f: &F = &*(f as *const F);
            f(&DOMUIEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::page-x\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_page_x_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_page_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_page_y_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMUIEvent, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMUIEvent>
        {
            let f: &F = &*(f as *const F);
            f(&DOMUIEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::page-y\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_page_y_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_view_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMUIEvent, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMUIEvent>
        {
            let f: &F = &*(f as *const F);
            f(&DOMUIEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::view\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_view_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMUIEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMUIEvent")
    }
}
