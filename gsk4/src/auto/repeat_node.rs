// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RenderNode;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct RepeatNode(Object<ffi::GskRepeatNode>) @extends RenderNode;

    match fn {
        get_type => || ffi::gsk_repeat_node_get_type(),
    }
}

impl RepeatNode {
    #[doc(alias = "gsk_repeat_node_new")]
    pub fn new<P: IsA<RenderNode>>(
        bounds: &graphene::Rect,
        child: &P,
        child_bounds: Option<&graphene::Rect>,
    ) -> RepeatNode {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_repeat_node_new(
                bounds.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                child_bounds.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_repeat_node_get_child")]
    pub fn get_child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(ffi::gsk_repeat_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_repeat_node_peek_child_bounds")]
    pub fn peek_child_bounds(&self) -> Option<graphene::Rect> {
        unsafe {
            from_glib_none(ffi::gsk_repeat_node_peek_child_bounds(
                self.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for RepeatNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RepeatNode")
    }
}
