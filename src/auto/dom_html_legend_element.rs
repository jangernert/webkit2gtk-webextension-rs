// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_webextension_sys;
use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMHTMLFormElement;
use DOMNode;
use DOMObject;

glib_wrapper! {
    pub struct DOMHTMLLegendElement(Object<webkit2_webextension_sys::WebKitDOMHTMLLegendElement, webkit2_webextension_sys::WebKitDOMHTMLLegendElementClass, DOMHTMLLegendElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_legend_element_get_type(),
    }
}

pub const NONE_DOMHTML_LEGEND_ELEMENT: Option<&DOMHTMLLegendElement> = None;

pub trait DOMHTMLLegendElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_align(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_form(&self) -> Option<DOMHTMLFormElement>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_align(&self, value: &str);

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLLegendElement>> DOMHTMLLegendElementExt for O {
    fn get_align(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_legend_element_get_align(self.as_ref().to_glib_none().0))
        }
    }

    fn get_form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_dom_html_legend_element_get_form(self.as_ref().to_glib_none().0))
        }
    }

    fn set_align(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_legend_element_set_align(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_align_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLLegendElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLLegendElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLLegendElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::align\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_align_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_form_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLLegendElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLLegendElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLLegendElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::form\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_form_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMHTMLLegendElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLLegendElement")
    }
}
