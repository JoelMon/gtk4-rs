// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::FrameClockPhase;
use crate::FrameTimings;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    pub struct FrameClock(Object<ffi::GdkFrameClock, ffi::GdkFrameClockClass>);

    match fn {
        type_ => || ffi::gdk_frame_clock_get_type(),
    }
}

impl FrameClock {
    #[doc(alias = "gdk_frame_clock_begin_updating")]
    pub fn begin_updating(&self) {
        unsafe {
            ffi::gdk_frame_clock_begin_updating(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_frame_clock_end_updating")]
    pub fn end_updating(&self) {
        unsafe {
            ffi::gdk_frame_clock_end_updating(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_frame_clock_get_current_timings")]
    pub fn current_timings(&self) -> Option<FrameTimings> {
        unsafe {
            from_glib_none(ffi::gdk_frame_clock_get_current_timings(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_frame_clock_get_fps")]
    pub fn fps(&self) -> f64 {
        unsafe { ffi::gdk_frame_clock_get_fps(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_frame_clock_get_frame_counter")]
    pub fn frame_counter(&self) -> i64 {
        unsafe { ffi::gdk_frame_clock_get_frame_counter(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_frame_clock_get_frame_time")]
    pub fn frame_time(&self) -> i64 {
        unsafe { ffi::gdk_frame_clock_get_frame_time(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_frame_clock_get_history_start")]
    pub fn history_start(&self) -> i64 {
        unsafe { ffi::gdk_frame_clock_get_history_start(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_frame_clock_get_refresh_info")]
    pub fn refresh_info(&self, base_time: i64) -> (i64, i64) {
        unsafe {
            let mut refresh_interval_return = mem::MaybeUninit::uninit();
            let mut presentation_time_return = mem::MaybeUninit::uninit();
            ffi::gdk_frame_clock_get_refresh_info(
                self.to_glib_none().0,
                base_time,
                refresh_interval_return.as_mut_ptr(),
                presentation_time_return.as_mut_ptr(),
            );
            let refresh_interval_return = refresh_interval_return.assume_init();
            let presentation_time_return = presentation_time_return.assume_init();
            (refresh_interval_return, presentation_time_return)
        }
    }

    #[doc(alias = "gdk_frame_clock_get_timings")]
    pub fn timings(&self, frame_counter: i64) -> Option<FrameTimings> {
        unsafe {
            from_glib_none(ffi::gdk_frame_clock_get_timings(
                self.to_glib_none().0,
                frame_counter,
            ))
        }
    }

    #[doc(alias = "gdk_frame_clock_request_phase")]
    pub fn request_phase(&self, phase: FrameClockPhase) {
        unsafe {
            ffi::gdk_frame_clock_request_phase(self.to_glib_none().0, phase.to_glib());
        }
    }

    pub fn connect_after_paint<F: Fn(&FrameClock) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn after_paint_trampoline<F: Fn(&FrameClock) + 'static>(
            this: *mut ffi::GdkFrameClock,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"after-paint\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    after_paint_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_before_paint<F: Fn(&FrameClock) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn before_paint_trampoline<F: Fn(&FrameClock) + 'static>(
            this: *mut ffi::GdkFrameClock,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"before-paint\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    before_paint_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_flush_events<F: Fn(&FrameClock) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn flush_events_trampoline<F: Fn(&FrameClock) + 'static>(
            this: *mut ffi::GdkFrameClock,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"flush-events\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    flush_events_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_layout<F: Fn(&FrameClock) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn layout_trampoline<F: Fn(&FrameClock) + 'static>(
            this: *mut ffi::GdkFrameClock,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"layout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    layout_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_paint<F: Fn(&FrameClock) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn paint_trampoline<F: Fn(&FrameClock) + 'static>(
            this: *mut ffi::GdkFrameClock,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"paint\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    paint_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_resume_events<F: Fn(&FrameClock) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn resume_events_trampoline<F: Fn(&FrameClock) + 'static>(
            this: *mut ffi::GdkFrameClock,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"resume-events\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    resume_events_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_update<F: Fn(&FrameClock) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn update_trampoline<F: Fn(&FrameClock) + 'static>(
            this: *mut ffi::GdkFrameClock,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"update\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    update_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FrameClock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FrameClock")
    }
}