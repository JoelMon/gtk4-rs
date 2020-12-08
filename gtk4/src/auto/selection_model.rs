// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Bitset;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct SelectionModel(Interface<ffi::GtkSelectionModel>) @requires gio::ListModel;

    match fn {
        get_type => || ffi::gtk_selection_model_get_type(),
    }
}

pub const NONE_SELECTION_MODEL: Option<&SelectionModel> = None;

pub trait SelectionModelExt: 'static {
    #[doc(alias = "gtk_selection_model_get_selection")]
    fn get_selection(&self) -> Bitset;

    #[doc(alias = "gtk_selection_model_get_selection_in_range")]
    fn get_selection_in_range(&self, position: u32, n_items: u32) -> Bitset;

    #[doc(alias = "gtk_selection_model_is_selected")]
    fn is_selected(&self, position: u32) -> bool;

    #[doc(alias = "gtk_selection_model_select_all")]
    fn select_all(&self) -> bool;

    #[doc(alias = "gtk_selection_model_select_item")]
    fn select_item(&self, position: u32, unselect_rest: bool) -> bool;

    #[doc(alias = "gtk_selection_model_select_range")]
    fn select_range(&self, position: u32, n_items: u32, unselect_rest: bool) -> bool;

    #[doc(alias = "gtk_selection_model_selection_changed")]
    fn selection_changed(&self, position: u32, n_items: u32);

    #[doc(alias = "gtk_selection_model_set_selection")]
    fn set_selection(&self, selected: &Bitset, mask: &Bitset) -> bool;

    #[doc(alias = "gtk_selection_model_unselect_all")]
    fn unselect_all(&self) -> bool;

    #[doc(alias = "gtk_selection_model_unselect_item")]
    fn unselect_item(&self, position: u32) -> bool;

    #[doc(alias = "gtk_selection_model_unselect_range")]
    fn unselect_range(&self, position: u32, n_items: u32) -> bool;

    fn connect_selection_changed<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SelectionModel>> SelectionModelExt for O {
    fn get_selection(&self) -> Bitset {
        unsafe {
            from_glib_full(ffi::gtk_selection_model_get_selection(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_selection_in_range(&self, position: u32, n_items: u32) -> Bitset {
        unsafe {
            from_glib_full(ffi::gtk_selection_model_get_selection_in_range(
                self.as_ref().to_glib_none().0,
                position,
                n_items,
            ))
        }
    }

    fn is_selected(&self, position: u32) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_model_is_selected(
                self.as_ref().to_glib_none().0,
                position,
            ))
        }
    }

    fn select_all(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_model_select_all(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn select_item(&self, position: u32, unselect_rest: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_model_select_item(
                self.as_ref().to_glib_none().0,
                position,
                unselect_rest.to_glib(),
            ))
        }
    }

    fn select_range(&self, position: u32, n_items: u32, unselect_rest: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_model_select_range(
                self.as_ref().to_glib_none().0,
                position,
                n_items,
                unselect_rest.to_glib(),
            ))
        }
    }

    fn selection_changed(&self, position: u32, n_items: u32) {
        unsafe {
            ffi::gtk_selection_model_selection_changed(
                self.as_ref().to_glib_none().0,
                position,
                n_items,
            );
        }
    }

    fn set_selection(&self, selected: &Bitset, mask: &Bitset) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_model_set_selection(
                self.as_ref().to_glib_none().0,
                selected.to_glib_none().0,
                mask.to_glib_none().0,
            ))
        }
    }

    fn unselect_all(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_model_unselect_all(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn unselect_item(&self, position: u32) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_model_unselect_item(
                self.as_ref().to_glib_none().0,
                position,
            ))
        }
    }

    fn unselect_range(&self, position: u32, n_items: u32) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_model_unselect_range(
                self.as_ref().to_glib_none().0,
                position,
                n_items,
            ))
        }
    }

    fn connect_selection_changed<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn selection_changed_trampoline<P, F: Fn(&P, u32, u32) + 'static>(
            this: *mut ffi::GtkSelectionModel,
            position: libc::c_uint,
            n_items: libc::c_uint,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<SelectionModel>,
        {
            let f: &F = &*(f as *const F);
            f(
                &SelectionModel::from_glib_borrow(this).unsafe_cast_ref(),
                position,
                n_items,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"selection-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    selection_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SelectionModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SelectionModel")
    }
}
