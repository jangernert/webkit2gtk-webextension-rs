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
use libc;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLTableCellElement(Object<ffi::WebKitDOMHTMLTableCellElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_table_cell_element_get_type(),
    }
}

pub trait DOMHTMLTableCellElementExt {
    fn get_abbr(&self) -> Option<String>;

    fn get_align(&self) -> Option<String>;

    fn get_axis(&self) -> Option<String>;

    fn get_bg_color(&self) -> Option<String>;

    fn get_cell_index(&self) -> libc::c_long;

    fn get_ch(&self) -> Option<String>;

    fn get_ch_off(&self) -> Option<String>;

    fn get_col_span(&self) -> libc::c_long;

    fn get_headers(&self) -> Option<String>;

    fn get_height(&self) -> Option<String>;

    fn get_no_wrap(&self) -> bool;

    fn get_row_span(&self) -> libc::c_long;

    fn get_scope(&self) -> Option<String>;

    fn get_v_align(&self) -> Option<String>;

    fn get_width(&self) -> Option<String>;

    fn set_abbr(&self, value: &str);

    fn set_align(&self, value: &str);

    fn set_axis(&self, value: &str);

    fn set_bg_color(&self, value: &str);

    fn set_ch(&self, value: &str);

    fn set_ch_off(&self, value: &str);

    fn set_col_span(&self, value: libc::c_long);

    fn set_headers(&self, value: &str);

    fn set_height(&self, value: &str);

    fn set_no_wrap(&self, value: bool);

    fn set_row_span(&self, value: libc::c_long);

    fn set_scope(&self, value: &str);

    fn set_v_align(&self, value: &str);

    fn set_width(&self, value: &str);
}

impl<O: IsA<DOMHTMLTableCellElement>> DOMHTMLTableCellElementExt for O {
    fn get_abbr(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_abbr(self.to_glib_none().0))
        }
    }

    fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_align(self.to_glib_none().0))
        }
    }

    fn get_axis(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_axis(self.to_glib_none().0))
        }
    }

    fn get_bg_color(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_bg_color(self.to_glib_none().0))
        }
    }

    fn get_cell_index(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_get_cell_index(self.to_glib_none().0)
        }
    }

    fn get_ch(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_ch(self.to_glib_none().0))
        }
    }

    fn get_ch_off(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_ch_off(self.to_glib_none().0))
        }
    }

    fn get_col_span(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_get_col_span(self.to_glib_none().0)
        }
    }

    fn get_headers(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_headers(self.to_glib_none().0))
        }
    }

    fn get_height(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_height(self.to_glib_none().0))
        }
    }

    fn get_no_wrap(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_table_cell_element_get_no_wrap(self.to_glib_none().0))
        }
    }

    fn get_row_span(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_get_row_span(self.to_glib_none().0)
        }
    }

    fn get_scope(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_scope(self.to_glib_none().0))
        }
    }

    fn get_v_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_v_align(self.to_glib_none().0))
        }
    }

    fn get_width(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_width(self.to_glib_none().0))
        }
    }

    fn set_abbr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_abbr(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_axis(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_axis(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_bg_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_bg_color(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_ch(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_ch(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_ch_off(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_ch_off(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_col_span(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_col_span(self.to_glib_none().0, value);
        }
    }

    fn set_headers(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_headers(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_height(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_height(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_no_wrap(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_no_wrap(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_row_span(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_row_span(self.to_glib_none().0, value);
        }
    }

    fn set_scope(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_scope(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_v_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_v_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_width(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_width(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
