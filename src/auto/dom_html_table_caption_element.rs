// This file was generated by gir (f00d658) from gir-files (1069259)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLTableCaptionElement(Object<ffi::WebKitDOMHTMLTableCaptionElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_table_caption_element_get_type(),
    }
}

pub trait DOMHTMLTableCaptionElementExt {
    fn get_align(&self) -> Option<String>;

    fn set_align(&self, value: &str);
}

impl<O: IsA<DOMHTMLTableCaptionElement>> DOMHTMLTableCaptionElementExt for O {
    fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_caption_element_get_align(self.to_glib_none().0))
        }
    }

    fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_caption_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
