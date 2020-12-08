// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::DeviceTool;
use crate::Display;
use crate::InputSource;
use crate::ModifierType;
use crate::Seat;
use crate::Surface;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct Device(Object<ffi::GdkDevice>);

    match fn {
        get_type => || ffi::gdk_device_get_type(),
    }
}

impl Device {
    #[doc(alias = "gdk_device_get_caps_lock_state")]
    pub fn get_caps_lock_state(&self) -> bool {
        unsafe { from_glib(ffi::gdk_device_get_caps_lock_state(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_device_tool")]
    pub fn get_device_tool(&self) -> Option<DeviceTool> {
        unsafe { from_glib_none(ffi::gdk_device_get_device_tool(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_direction")]
    pub fn get_direction(&self) -> pango::Direction {
        unsafe { from_glib(ffi::gdk_device_get_direction(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_display")]
    pub fn get_display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_device_get_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_has_cursor")]
    pub fn get_has_cursor(&self) -> bool {
        unsafe { from_glib(ffi::gdk_device_get_has_cursor(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_modifier_state")]
    pub fn get_modifier_state(&self) -> ModifierType {
        unsafe { from_glib(ffi::gdk_device_get_modifier_state(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_name")]
    pub fn get_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gdk_device_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_num_lock_state")]
    pub fn get_num_lock_state(&self) -> bool {
        unsafe { from_glib(ffi::gdk_device_get_num_lock_state(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_num_touches")]
    pub fn get_num_touches(&self) -> u32 {
        unsafe { ffi::gdk_device_get_num_touches(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_device_get_product_id")]
    pub fn get_product_id(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gdk_device_get_product_id(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_scroll_lock_state")]
    pub fn get_scroll_lock_state(&self) -> bool {
        unsafe { from_glib(ffi::gdk_device_get_scroll_lock_state(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_seat")]
    pub fn get_seat(&self) -> Option<Seat> {
        unsafe { from_glib_none(ffi::gdk_device_get_seat(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_source")]
    pub fn get_source(&self) -> InputSource {
        unsafe { from_glib(ffi::gdk_device_get_source(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_surface_at_position")]
    pub fn get_surface_at_position(&self) -> (Option<Surface>, f64, f64) {
        unsafe {
            let mut win_x = mem::MaybeUninit::uninit();
            let mut win_y = mem::MaybeUninit::uninit();
            let ret = from_glib_none(ffi::gdk_device_get_surface_at_position(
                self.to_glib_none().0,
                win_x.as_mut_ptr(),
                win_y.as_mut_ptr(),
            ));
            let win_x = win_x.assume_init();
            let win_y = win_y.assume_init();
            (ret, win_x, win_y)
        }
    }

    #[doc(alias = "gdk_device_get_vendor_id")]
    pub fn get_vendor_id(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gdk_device_get_vendor_id(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_has_bidi_layouts")]
    pub fn has_bidi_layouts(&self) -> bool {
        unsafe { from_glib(ffi::gdk_device_has_bidi_layouts(self.to_glib_none().0)) }
    }

    pub fn get_property_has_bidi_layouts(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"has-bidi-layouts\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `has-bidi-layouts` getter")
                .unwrap()
        }
    }

    pub fn get_property_n_axes(&self) -> u32 {
        unsafe {
            let mut value = glib::Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"n-axes\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `n-axes` getter")
                .unwrap()
        }
    }

    pub fn set_property_seat(&self, seat: Option<&Seat>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"seat\0".as_ptr() as *const _,
                glib::Value::from(seat).to_glib_none().0,
            );
        }
    }

    pub fn get_property_tool(&self) -> Option<DeviceTool> {
        unsafe {
            let mut value = glib::Value::from_type(<DeviceTool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"tool\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `tool` getter")
        }
    }

    pub fn connect_changed<F: Fn(&Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<F: Fn(&Device) + 'static>(
            this: *mut ffi::GdkDevice,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_tool_changed<F: Fn(&Device, &DeviceTool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn tool_changed_trampoline<F: Fn(&Device, &DeviceTool) + 'static>(
            this: *mut ffi::GdkDevice,
            tool: *mut ffi::GdkDeviceTool,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(tool))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tool-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    tool_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_caps_lock_state_notify<F: Fn(&Device) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_caps_lock_state_trampoline<F: Fn(&Device) + 'static>(
            this: *mut ffi::GdkDevice,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::caps-lock-state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_caps_lock_state_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_direction_notify<F: Fn(&Device) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_direction_trampoline<F: Fn(&Device) + 'static>(
            this: *mut ffi::GdkDevice,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::direction\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_direction_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_has_bidi_layouts_notify<F: Fn(&Device) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_bidi_layouts_trampoline<F: Fn(&Device) + 'static>(
            this: *mut ffi::GdkDevice,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-bidi-layouts\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_bidi_layouts_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_modifier_state_notify<F: Fn(&Device) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_modifier_state_trampoline<F: Fn(&Device) + 'static>(
            this: *mut ffi::GdkDevice,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::modifier-state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_modifier_state_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_n_axes_notify<F: Fn(&Device) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_axes_trampoline<F: Fn(&Device) + 'static>(
            this: *mut ffi::GdkDevice,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::n-axes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_n_axes_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_num_lock_state_notify<F: Fn(&Device) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_num_lock_state_trampoline<F: Fn(&Device) + 'static>(
            this: *mut ffi::GdkDevice,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::num-lock-state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_num_lock_state_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_scroll_lock_state_notify<F: Fn(&Device) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_scroll_lock_state_trampoline<F: Fn(&Device) + 'static>(
            this: *mut ffi::GdkDevice,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scroll-lock-state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scroll_lock_state_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_seat_notify<F: Fn(&Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_seat_trampoline<F: Fn(&Device) + 'static>(
            this: *mut ffi::GdkDevice,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::seat\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_seat_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_tool_notify<F: Fn(&Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tool_trampoline<F: Fn(&Device) + 'static>(
            this: *mut ffi::GdkDevice,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tool\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tool_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Device")
    }
}
