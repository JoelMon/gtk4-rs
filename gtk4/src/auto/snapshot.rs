// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::StyleContext;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct Snapshot(Object<ffi::GtkSnapshot, ffi::GtkSnapshotClass>) @extends gdk::Snapshot;

    match fn {
        get_type => || ffi::gtk_snapshot_get_type(),
    }
}

impl Snapshot {
    #[doc(alias = "gtk_snapshot_new")]
    pub fn new() -> Snapshot {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_snapshot_new()) }
    }

    //#[doc(alias = "gtk_snapshot_append_border")]
    //pub fn append_border(&self, outline: &gsk::RoundedRect, border_width: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 20 }; 4, border_color: /*Unimplemented*/FixedArray TypeId { ns_id: 11, id: 80 }; 4) {
    //    unsafe { TODO: call ffi:gtk_snapshot_append_border() }
    //}

    #[doc(alias = "gtk_snapshot_append_cairo")]
    pub fn append_cairo(&self, bounds: &graphene::Rect) -> Option<cairo::Context> {
        unsafe {
            from_glib_full(ffi::gtk_snapshot_append_cairo(
                self.to_glib_none().0,
                bounds.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_snapshot_append_color")]
    pub fn append_color(&self, color: &gdk::RGBA, bounds: &graphene::Rect) {
        unsafe {
            ffi::gtk_snapshot_append_color(
                self.to_glib_none().0,
                color.to_glib_none().0,
                bounds.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_inset_shadow")]
    pub fn append_inset_shadow(
        &self,
        outline: &gsk::RoundedRect,
        color: &gdk::RGBA,
        dx: f32,
        dy: f32,
        spread: f32,
        blur_radius: f32,
    ) {
        unsafe {
            ffi::gtk_snapshot_append_inset_shadow(
                self.to_glib_none().0,
                outline.to_glib_none().0,
                color.to_glib_none().0,
                dx,
                dy,
                spread,
                blur_radius,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_layout")]
    pub fn append_layout(&self, layout: &pango::Layout, color: &gdk::RGBA) {
        unsafe {
            ffi::gtk_snapshot_append_layout(
                self.to_glib_none().0,
                layout.to_glib_none().0,
                color.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_node")]
    pub fn append_node<P: IsA<gsk::RenderNode>>(&self, node: &P) {
        unsafe {
            ffi::gtk_snapshot_append_node(self.to_glib_none().0, node.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_append_outset_shadow")]
    pub fn append_outset_shadow(
        &self,
        outline: &gsk::RoundedRect,
        color: &gdk::RGBA,
        dx: f32,
        dy: f32,
        spread: f32,
        blur_radius: f32,
    ) {
        unsafe {
            ffi::gtk_snapshot_append_outset_shadow(
                self.to_glib_none().0,
                outline.to_glib_none().0,
                color.to_glib_none().0,
                dx,
                dy,
                spread,
                blur_radius,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_append_texture")]
    pub fn append_texture<P: IsA<gdk::Texture>>(&self, texture: &P, bounds: &graphene::Rect) {
        unsafe {
            ffi::gtk_snapshot_append_texture(
                self.to_glib_none().0,
                texture.as_ref().to_glib_none().0,
                bounds.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_free_to_node")]
    pub fn free_to_node(&self) -> Option<gsk::RenderNode> {
        unsafe { from_glib_full(ffi::gtk_snapshot_free_to_node(self.to_glib_full())) }
    }

    #[doc(alias = "gtk_snapshot_free_to_paintable")]
    pub fn free_to_paintable(&self, size: Option<&graphene::Size>) -> Option<gdk::Paintable> {
        unsafe {
            from_glib_full(ffi::gtk_snapshot_free_to_paintable(
                self.to_glib_full(),
                size.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_snapshot_gl_shader_pop_texture")]
    pub fn gl_shader_pop_texture(&self) {
        unsafe {
            ffi::gtk_snapshot_gl_shader_pop_texture(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_perspective")]
    pub fn perspective(&self, depth: f32) {
        unsafe {
            ffi::gtk_snapshot_perspective(self.to_glib_none().0, depth);
        }
    }

    #[doc(alias = "gtk_snapshot_pop")]
    pub fn pop(&self) {
        unsafe {
            ffi::gtk_snapshot_pop(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_push_blend")]
    pub fn push_blend(&self, blend_mode: gsk::BlendMode) {
        unsafe {
            ffi::gtk_snapshot_push_blend(self.to_glib_none().0, blend_mode.to_glib());
        }
    }

    #[doc(alias = "gtk_snapshot_push_blur")]
    pub fn push_blur(&self, radius: f64) {
        unsafe {
            ffi::gtk_snapshot_push_blur(self.to_glib_none().0, radius);
        }
    }

    #[doc(alias = "gtk_snapshot_push_clip")]
    pub fn push_clip(&self, bounds: &graphene::Rect) {
        unsafe {
            ffi::gtk_snapshot_push_clip(self.to_glib_none().0, bounds.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_push_color_matrix")]
    pub fn push_color_matrix(
        &self,
        color_matrix: &graphene::Matrix,
        color_offset: &graphene::Vec4,
    ) {
        unsafe {
            ffi::gtk_snapshot_push_color_matrix(
                self.to_glib_none().0,
                color_matrix.to_glib_none().0,
                color_offset.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_push_cross_fade")]
    pub fn push_cross_fade(&self, progress: f64) {
        unsafe {
            ffi::gtk_snapshot_push_cross_fade(self.to_glib_none().0, progress);
        }
    }

    //#[doc(alias = "gtk_snapshot_push_debug")]
    //pub fn push_debug(&self, message: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gtk_snapshot_push_debug() }
    //}

    //#[doc(alias = "gtk_snapshot_push_gl_shader")]
    //pub fn push_gl_shader(&self, shader: /*Ignored*/&gsk::GLShader, bounds: &graphene::Rect, take_args: &glib::Bytes) {
    //    unsafe { TODO: call ffi:gtk_snapshot_push_gl_shader() }
    //}

    #[doc(alias = "gtk_snapshot_push_opacity")]
    pub fn push_opacity(&self, opacity: f64) {
        unsafe {
            ffi::gtk_snapshot_push_opacity(self.to_glib_none().0, opacity);
        }
    }

    #[doc(alias = "gtk_snapshot_push_repeat")]
    pub fn push_repeat(&self, bounds: &graphene::Rect, child_bounds: Option<&graphene::Rect>) {
        unsafe {
            ffi::gtk_snapshot_push_repeat(
                self.to_glib_none().0,
                bounds.to_glib_none().0,
                child_bounds.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_push_rounded_clip")]
    pub fn push_rounded_clip(&self, bounds: &gsk::RoundedRect) {
        unsafe {
            ffi::gtk_snapshot_push_rounded_clip(self.to_glib_none().0, bounds.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_push_shadow")]
    pub fn push_shadow(&self, shadow: &gsk::Shadow, n_shadows: usize) {
        unsafe {
            ffi::gtk_snapshot_push_shadow(
                self.to_glib_none().0,
                shadow.to_glib_none().0,
                n_shadows,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_render_background")]
    pub fn render_background<P: IsA<StyleContext>>(
        &self,
        context: &P,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        unsafe {
            ffi::gtk_snapshot_render_background(
                self.to_glib_none().0,
                context.as_ref().to_glib_none().0,
                x,
                y,
                width,
                height,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_render_focus")]
    pub fn render_focus<P: IsA<StyleContext>>(
        &self,
        context: &P,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        unsafe {
            ffi::gtk_snapshot_render_focus(
                self.to_glib_none().0,
                context.as_ref().to_glib_none().0,
                x,
                y,
                width,
                height,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_render_frame")]
    pub fn render_frame<P: IsA<StyleContext>>(
        &self,
        context: &P,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        unsafe {
            ffi::gtk_snapshot_render_frame(
                self.to_glib_none().0,
                context.as_ref().to_glib_none().0,
                x,
                y,
                width,
                height,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_render_insertion_cursor")]
    pub fn render_insertion_cursor<P: IsA<StyleContext>>(
        &self,
        context: &P,
        x: f64,
        y: f64,
        layout: &pango::Layout,
        index: i32,
        direction: pango::Direction,
    ) {
        unsafe {
            ffi::gtk_snapshot_render_insertion_cursor(
                self.to_glib_none().0,
                context.as_ref().to_glib_none().0,
                x,
                y,
                layout.to_glib_none().0,
                index,
                direction.to_glib(),
            );
        }
    }

    #[doc(alias = "gtk_snapshot_render_layout")]
    pub fn render_layout<P: IsA<StyleContext>>(
        &self,
        context: &P,
        x: f64,
        y: f64,
        layout: &pango::Layout,
    ) {
        unsafe {
            ffi::gtk_snapshot_render_layout(
                self.to_glib_none().0,
                context.as_ref().to_glib_none().0,
                x,
                y,
                layout.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_snapshot_restore")]
    pub fn restore(&self) {
        unsafe {
            ffi::gtk_snapshot_restore(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_rotate")]
    pub fn rotate(&self, angle: f32) {
        unsafe {
            ffi::gtk_snapshot_rotate(self.to_glib_none().0, angle);
        }
    }

    #[doc(alias = "gtk_snapshot_rotate_3d")]
    pub fn rotate_3d(&self, angle: f32, axis: &graphene::Vec3) {
        unsafe {
            ffi::gtk_snapshot_rotate_3d(self.to_glib_none().0, angle, axis.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_save")]
    pub fn save(&self) {
        unsafe {
            ffi::gtk_snapshot_save(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_scale")]
    pub fn scale(&self, factor_x: f32, factor_y: f32) {
        unsafe {
            ffi::gtk_snapshot_scale(self.to_glib_none().0, factor_x, factor_y);
        }
    }

    #[doc(alias = "gtk_snapshot_scale_3d")]
    pub fn scale_3d(&self, factor_x: f32, factor_y: f32, factor_z: f32) {
        unsafe {
            ffi::gtk_snapshot_scale_3d(self.to_glib_none().0, factor_x, factor_y, factor_z);
        }
    }

    #[doc(alias = "gtk_snapshot_to_node")]
    pub fn to_node(&self) -> Option<gsk::RenderNode> {
        unsafe { from_glib_full(ffi::gtk_snapshot_to_node(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_snapshot_to_paintable")]
    pub fn to_paintable(&self, size: Option<&graphene::Size>) -> Option<gdk::Paintable> {
        unsafe {
            from_glib_full(ffi::gtk_snapshot_to_paintable(
                self.to_glib_none().0,
                size.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_snapshot_transform")]
    pub fn transform(&self, transform: Option<&gsk::Transform>) {
        unsafe {
            ffi::gtk_snapshot_transform(self.to_glib_none().0, transform.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_transform_matrix")]
    pub fn transform_matrix(&self, matrix: &graphene::Matrix) {
        unsafe {
            ffi::gtk_snapshot_transform_matrix(self.to_glib_none().0, matrix.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_translate")]
    pub fn translate(&self, point: &graphene::Point) {
        unsafe {
            ffi::gtk_snapshot_translate(self.to_glib_none().0, point.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_snapshot_translate_3d")]
    pub fn translate_3d(&self, point: &graphene::Point3D) {
        unsafe {
            ffi::gtk_snapshot_translate_3d(self.to_glib_none().0, point.to_glib_none().0);
        }
    }
}

impl Default for Snapshot {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Snapshot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Snapshot")
    }
}
