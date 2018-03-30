//! This library was designed to be render backend agnostic so it does
//! not draw anything to screen. Instead all drawn shapes, widgets
//! are made of, are buffered into memory and make up a command queue.
//! Each frame therefore fills the command buffer with draw commands
//! that then need to be executed by the user and his own render backend.
//! After that the command buffer needs to be cleared and a new frame can be
//! started. It is probably important to note that the command buffer is the main
//! drawing API and the optional vertex buffer API only takes this format and
//! converts it into a hardware accessible format.
//! 
//!


//! Usage
//! -------------------
//! To draw all draw commands accumulated over a frame you need your own render
//! backend able to draw a number of 2D primitives. This includes at least
//! filled and stroked rectangles, circles, text, lines, triangles and scissors.
//! As soon as this criterion is met you can iterate over each draw command
//! and execute each draw command in a interpreter like fashion:
//!
//!     const struct nk_command *cmd = 0;
//!     nk_foreach(cmd, &ctx) {
//!     switch (cmd->type) {
//!     case NK_COMMAND_LINE:
//!         your_draw_line_function(...)
//!         break;
//!     case NK_COMMAND_RECT
//!         your_draw_rect_function(...)
//!         break;
//!     case ...:
//!         [...]
//!     }
//!
//! In program flow context draw commands need to be executed after input has been
//! gathered and the complete UI with windows and their contained widgets have
//! been executed and before calling `nk_clear` which frees all previously
//! allocated draw commands.
//!
//!     struct nk_context ctx;
//!     nk_init_xxx(&ctx, ...);
//!     while (1) {
//!         Event evt;
//!         nk_input_begin(&ctx);
//!         while (GetEvent(&evt)) {
//!             if (evt.type == MOUSE_MOVE)
//!                 nk_input_motion(&ctx, evt.motion.x, evt.motion.y);
//!             else if (evt.type == [...]) {
//!                 [...]
//!             }
//!         }
//!         nk_input_end(&ctx);
//!
//!         [...]
//!
//!         const struct nk_command *cmd = 0;
//!         nk_foreach(cmd, &ctx) {
//!         switch (cmd->type) {
//!         case NK_COMMAND_LINE:
//!             your_draw_line_function(...)
//!             break;
//!         case NK_COMMAND_RECT
//!             your_draw_rect_function(...)
//!             break;
//!         case ...:
//!             [...]
//!         }
//!         nk_clear(&ctx);
//!     }
//!     nk_free(&ctx);
//!
//! You probably noticed that you have to draw all of the UI each frame which is
//! quite wasteful. While the actual UI updating loop is quite fast rendering
//! without actually needing it is not. So there are multiple things you could do.
//!
//! First is only update on input. This ofcourse is only an option if your
//! application only depends on the UI and does not require any outside calculations.
//! If you actually only update on input make sure to update the UI to times each
//! frame and call `nk_clear` directly after the first pass and only draw in
//! the second pass.
//!
//!     struct nk_context ctx;
//!     nk_init_xxx(&ctx, ...);
//!     while (1) {
//!         [...wait for input ]
//!
//!         [...do two UI passes ...]
//!         do_ui(...)
//!         nk_clear(&ctx);
//!         do_ui(...)
//!
//!         const struct nk_command *cmd = 0;
//!         nk_foreach(cmd, &ctx) {
//!         switch (cmd->type) {
//!         case NK_COMMAND_LINE:
//!             your_draw_line_function(...)
//!             break;
//!         case NK_COMMAND_RECT
//!             your_draw_rect_function(...)
//!             break;
//!         case ...:
//!             [...]
//!         }
//!         nk_clear(&ctx);
//!     }
//!     nk_free(&ctx);
//!
//! The second probably more applicable trick is to only draw if anything changed.
//! It is not really useful for applications with continous draw loop but
//! quite useful for desktop applications. To actually get nuklear to only
//! draw on changes you first have to define `NK_ZERO_COMMAND_MEMORY` and
//! allocate a memory buffer that will store each unique drawing output.
//! After each frame you compare the draw command memory inside the library
//! with your allocated buffer by memcmp. If memcmp detects differences
//! you have to copy the nuklears command buffer into the allocated buffer
//! and then draw like usual (this example uses fixed memory but you could
//! use dynamically allocated memory).
//!
//!     [... other defines ...]
//!     #define NK_ZERO_COMMAND_MEMORY
//!     #include "nuklear.h"
//!
//!     struct nk_context ctx;
//!     void *last = calloc(1,64*1024);
//!     void *buf = calloc(1,64*1024);
//!     nk_init_fixed(&ctx, buf, 64*1024);
//!     while (1) {
//!         [...input...]
//!         [...ui...]
//!
//!         void *cmds = nk_buffer_memory(&ctx.memory);
//!         if (memcmp(cmds, last, ctx.memory.allocated)) {
//!             memcpy(last,cmds,ctx.memory.allocated);
//!             const struct nk_command *cmd = 0;
//!             nk_foreach(cmd, &ctx) {
//!                 switch (cmd->type) {
//!                 case NK_COMMAND_LINE:
//!                     your_draw_line_function(...)
//!                     break;
//!                 case NK_COMMAND_RECT
//!                     your_draw_rect_function(...)
//!                     break;
//!                 case ...:
//!                     [...]
//!                 }
//!             }
//!         }
//!         nk_clear(&ctx);
//!     }
//!     nk_free(&ctx);
//!
//! Finally while using draw commands makes sense for higher abstracted platforms like
//! X11 and Win32 or drawing libraries it is often desirable to use graphics
//! hardware directly. Therefore it is possible to just define
//! `NK_INCLUDE_VERTEX_BUFFER_OUTPUT` which includes optional vertex output.
//! To access the vertex output you first have to convert all draw commands into
//! vertexes by calling `nk_convert` which takes in your prefered vertex format.
//! After successfully converting all draw commands just iterate over and execute all
//! vertex draw commands:
//!
//!     struct nk_convert_config cfg = {};
//!     static const struct nk_draw_vertex_layout_element vertex_layout[] = {
//!         {NK_VERTEX_POSITION, NK_FORMAT_FLOAT, NK_OFFSETOF(struct your_vertex, pos)},
//!         {NK_VERTEX_TEXCOORD, NK_FORMAT_FLOAT, NK_OFFSETOF(struct your_vertex, uv)},
//!         {NK_VERTEX_COLOR, NK_FORMAT_R8G8B8A8, NK_OFFSETOF(struct your_vertex, col)},
//!         {NK_VERTEX_LAYOUT_END}
//!     };
//!     cfg.shape_AA = NK_ANTI_ALIASING_ON;
//!     cfg.line_AA = NK_ANTI_ALIASING_ON;
//!     cfg.vertex_layout = vertex_layout;
//!     cfg.vertex_size = sizeof(struct your_vertex);
//!     cfg.vertex_alignment = NK_ALIGNOF(struct your_vertex);
//!     cfg.circle_segment_count = 22;
//!     cfg.curve_segment_count = 22;
//!     cfg.arc_segment_count = 22;
//!     cfg.global_alpha = 1.0f;
//!     cfg.null = dev->null;
//!
//!     struct nk_buffer cmds, verts, idx;
//!     nk_buffer_init_default(&cmds);
//!     nk_buffer_init_default(&verts);
//!     nk_buffer_init_default(&idx);
//!     nk_convert(&ctx, &cmds, &verts, &idx, &cfg);
//!     nk_draw_foreach(cmd, &ctx, &cmds) {
//!         if (!cmd->elem_count) continue;
//!         [...]
//!     }
//!     nk_buffer_free(&cms);
//!     nk_buffer_free(&verts);
//!     nk_buffer_free(&idx);
//!
//! Reference
//! -------------------
//! nk__begin           - Returns the first draw command in the context draw command list to be drawn
//! nk__next            - Increments the draw command iterator to the next command inside the context draw command list
//! nk_foreach          - Iteratates over each draw command inside the context draw command list
//!
//! nk_convert          - Converts from the abstract draw commands list into a hardware accessable vertex format
//! nk__draw_begin      - Returns the first vertex command in the context vertex draw list to be executed
//! nk__draw_next       - Increments the vertex command iterator to the next command inside the context vertex command list
//! nk__draw_end        - Returns the end of the vertex draw list
//! nk_draw_foreach     - Iterates over each vertex draw command inside the vertex draw list
//!
//!
//!
//! # Drawing your own widgets
//! To use the command queue to draw your own widgets you can access the
//! command buffer of each window by calling `nk_window_get_canvas` after
//! previously having called `nk_begin`:
//! 
//!     void draw_red_rectangle_widget(struct nk_context *ctx)
//!     {
//!         struct nk_command_buffer *canvas;
//!         struct nk_input *input = &ctx->input;
//!         canvas = nk_window_get_canvas(ctx);
//! 
//!         struct nk_rect space;
//!         enum nk_widget_layout_states state;
//!         state = nk_widget(&space, ctx);
//!         if (!state) return;
//! 
//!         if (state != NK_WIDGET_ROM)
//!             update_your_widget_by_user_input(...);
//!         nk_fill_rect(canvas, space, 0, nk_rgb(255,0,0));
//!     }
//! 
//!     if (nk_begin(...)) {
//!         nk_layout_row_dynamic(ctx, 25, 1);
//!         draw_red_rectangle_widget(ctx);
//!     }
//!     nk_end(..)
//! 
//! Important to know if you want to create your own widgets is the `nk_widget`
//! call. It allocates space on the panel reserved for this widget to be used,
//! but also returns the state of the widget space. If your widget is not seen and does
//! not have to be updated it is '0' and you can just return. If it only has
//! to be drawn the state will be `NK_WIDGET_ROM` otherwise you can do both
//! update and draw your widget. The reason for seperating is to only draw and
//! update what is actually neccessary which is crucial for performance.

use core::ptr::*;
use nuke_sys::*;
use context::*;
use clear::*;

#[must_use = "this is the entry point to reading back draw commands"]
pub struct DrawStep<'a> {
    ctx: &'a mut Context<'a>
}

/// NOTE: Should it implement ExactSizeIterator ? (Resolved: No.) 
///       - We don't actually know the exact number of items until we iterate over all of them beforehand.
///         Not sure that this matches the intent of ExactSizeIterator (compare to
///         slices iterators!);
///       - That's not cache-friendly at all.
///       - What benefits does it bring to the user anyway ?
/// NOTE: There's no Iter because Nuklear internally mutates the context with nk__begin().
/// NOTE: There's no IterMut because Nuklear internally gives back const pointers to command data,
///       and it doesn't make sense to mutate the CommandIterator anyway.
// We reference a Context that outlives us.
pub struct CommandIterator<'a, 'b:'a> {
    ctx: &'a mut Context<'b>,
    cmd_ptr: *const nk_command,
}

impl<'a,'b:'a> Iterator for CommandIterator<'a, 'b> {
    type Item = Command<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cmd_ptr == null() {
            return None;
        }
        unsafe {
            let cur = Command::from(self.cmd_ptr.as_ref().unwrap());
            self.cmd_ptr = nk__next(&mut self.ctx.raw, self.cmd_ptr);
            Some(cur)
        }
    }
}
impl<'a,'b:'a> CommandIterator<'a, 'b> {
    pub(crate) fn new(ctx: &'a mut Context<'b>) -> Self {
        let cmd_ptr = unsafe { nk__begin(&mut ctx.raw) };
        Self { ctx, cmd_ptr }
    }
}


#[doc(hidden)]
impl<'a,'b:'a> From<&'b nk_command> for Command<'a> {
    #[allow(unreachable_patterns)] // See the last match arm.
    fn from(cmd: &'b nk_command) -> Self {
        
        use core::slice;
        use nuke_sys::nk_command_type as cmd;
        
        let cmd_type = cmd.type_;
        let cmd = cmd as *const nk_command;
        
        match cmd_type {
            cmd::NK_COMMAND_SCISSOR => {
                let cmd = unsafe { &*(cmd as *const nk_command_scissor) };
                let &nk_command_scissor { x, y, w, h, .. } = cmd;
                let rect = Rect { x, y, w, h };
                Command::Scissor { rect }
            },
            cmd::NK_COMMAND_LINE => {
                let cmd = unsafe { &*(cmd as *const nk_command_line) };
                let &nk_command_line {
                    line_thickness, begin, end, color, ..
                } = cmd;
                Command::Line {
                    thickness: line_thickness, 
                    start: begin.into(), end: end.into(), 
                    color: color.into()
                }
            },
            cmd::NK_COMMAND_CURVE => {
                let cmd = unsafe { &*(cmd as *const nk_command_curve) };
                let &nk_command_curve {
                    color, ctrl, begin, end, line_thickness, ..
                } = cmd;
                Command::Curve {
                    color: color.into(), 
                    ctrl: [ctrl[0].into(), ctrl[1].into()], 
                    end: end.into(), start: begin.into(), 
                    thickness: line_thickness
                }
            },
            cmd::NK_COMMAND_RECT => {
                let cmd = unsafe { &*(cmd as *const nk_command_rect) };
                let &nk_command_rect {
                    x, y, w, h, color, line_thickness, rounding, ..
                } = cmd;
                Command::Rect {
                    rect: Rect { x, y, w, h }, 
                    color: color.into(), 
                    line_thickness, rounding
                }
            },
            cmd::NK_COMMAND_RECT_FILLED => {
                let cmd = unsafe { &*(cmd as *const nk_command_rect_filled) };
                let &nk_command_rect_filled {
                    color, x, y, w, h, rounding, ..
                } = cmd;
                Command::FilledRect {
                    color: color.into(), 
                    rect: Rect { x, y, w, h }, 
                    rounding
                }
            },
            cmd::NK_COMMAND_RECT_MULTI_COLOR => {
                let cmd = unsafe { &*(cmd as *const nk_command_rect_multi_color) };
                let &nk_command_rect_multi_color {
                    x, y, w, h, bottom, left, right, top, ..
                } = cmd;
                Command::MultiColorRect {
                    rect: Rect { x, y, w, h }, 
                    bottom: bottom.into(), 
                    left: left.into(), 
                    right: right.into(), 
                    top: top.into()
                }
            },
            cmd::NK_COMMAND_CIRCLE => {
                let cmd = unsafe { &*(cmd as *const nk_command_circle) };
                let &nk_command_circle {
                    x, y, w, h, color, line_thickness, ..
                } = cmd;
                Command::Ellipse {
                    center: Vec2 { x, y }, size: Vec2 { x: w, y: h }, 
                    color: color.into(), 
                    line_thickness
                }
            },
            cmd::NK_COMMAND_CIRCLE_FILLED => {
                let cmd = unsafe { &*(cmd as *const nk_command_circle_filled) };
                let &nk_command_circle_filled {
                    x, y, w, h, color, ..
                } = cmd;
                Command::FilledEllipse {
                    center: Vec2 { x, y }, size: Vec2 { x: w, y: h }, 
                    color: color.into(), 
                }
            },
            cmd::NK_COMMAND_ARC => {
                let cmd = unsafe { &*(cmd as *const nk_command_arc) };
                let &nk_command_arc {
                    cx, cy, r, a, color, line_thickness, ..
                } = cmd;
                Command::Arc {
                    center: Vec2 { x: cx, y: cy }, 
                    radius: r,
                    start_angle: a[0], end_angle: a[1],
                    color: color.into(),
                    line_thickness
                }
            },
            cmd::NK_COMMAND_ARC_FILLED => {
                let cmd = unsafe { &*(cmd as *const nk_command_arc_filled) };
                let &nk_command_arc_filled {
                    cx, cy, r, a, color, ..
                } = cmd;
                Command::FilledArc {
                    center: Vec2 { x: cx, y: cy }, 
                    radius: r,
                    start_angle: a[0], end_angle: a[1],
                    color: color.into(),
                }
            },
            cmd::NK_COMMAND_TRIANGLE => {
                let cmd = unsafe { &*(cmd as *const nk_command_triangle) };
                let &nk_command_triangle {
                    color, line_thickness, a, b, c, ..
                } = cmd;
                Command::Triangle {
                    color: color.into(),
                    line_thickness,
                    vertices: [a.into(), b.into(), c.into()],
                }
            },
            cmd::NK_COMMAND_TRIANGLE_FILLED => {
                let cmd = unsafe { &*(cmd as *const nk_command_triangle_filled) };
                let &nk_command_triangle_filled {
                    color, a, b, c, ..
                } = cmd;
                Command::FilledTriangle {
                    color: color.into(),
                    vertices: [a.into(), b.into(), c.into()],
                }
            },
            cmd::NK_COMMAND_POLYGON => {
                let cmd = unsafe { &*(cmd as *const nk_command_polygon) };
                let &nk_command_polygon {
                    color, line_thickness, points, point_count, ..
                } = cmd;
                // NOTE: This requires that Vec2<i16> is #[repr(C)] to match nk_vec2i.
                // TODO: Write tests for this
                let points = points.as_ptr() as *const Vec2<i16>;
                Command::Polygon {
                    color: color.into(),
                    line_thickness,
                    vertices: unsafe { slice::from_raw_parts(points, point_count as usize) },
                }
            },
            cmd::NK_COMMAND_POLYGON_FILLED => {
                let cmd = unsafe { &*(cmd as *const nk_command_polygon_filled) };
                let &nk_command_polygon_filled {
                    color, points, point_count, ..
                } = cmd;
                // NOTE: Same as above
                let points = points.as_ptr() as *const Vec2<i16>;
                Command::FilledPolygon {
                    color: color.into(),
                    vertices: unsafe { slice::from_raw_parts(points, point_count as usize) },
                }
            },
            cmd::NK_COMMAND_POLYLINE => {
                let cmd = unsafe { &*(cmd as *const nk_command_polyline) };
                let &nk_command_polyline {
                    color, line_thickness, points, point_count, ..
                } = cmd;
                let points = points.as_ptr() as *const Vec2<i16>;
                Command::Polyline {
                    color: color.into(),
                    line_thickness,
                    vertices: unsafe { slice::from_raw_parts(points, point_count as usize) },
                }
            },
            cmd::NK_COMMAND_TEXT => {
                let cmd = unsafe { &*(cmd as *const nk_command_text) };
                let &nk_command_text {
                    font, background, foreground, x, y, w, h, height, length, string, ..
                } = cmd;
                let string = string.as_ptr() as *const u8;
                Command::Text {
                    font: unimplemented!(),
                    background_color: background.into(),
                    foreground_color: foreground.into(),
                    rect: Rect { x, y, w, h },
                    height,
                    string: unsafe { slice::from_raw_parts(string, length as usize) },
                }
            },
            cmd::NK_COMMAND_IMAGE => {
                let cmd = unsafe { &*(cmd as *const nk_command_image) };
                let &nk_command_image {
                    x, y, w, h, img, col, ..
                } = cmd;
                Command::Image {
                    rect: Rect { x, y, w, h },
                    image: img.into(),
                    color: col.into(),
                }
            },
            cmd::NK_COMMAND_CUSTOM => {
                let cmd = unsafe { &*(cmd as *const nk_command_custom) };
                let &nk_command_custom {
                    x, y, w, h, callback, callback_data, ..
                } = cmd;
                Command::Custom {
                    rect: Rect { x, y, w, h }, 
                    custom: unimplemented!(),
                }
            },
            // XXX Prove that NK_COMMAND_NOP is indeed unreachable.
            // NK_COMMAND_NOP may show up if memory was zeroed beforehand.
            // AFAIK it never happens, but it's not proven yet.
            cmd::NK_COMMAND_NOP => unreachable!(),
            // In case the underlying value is garbage, we want to panic.
            _ => unreachable!(),
        }
    }
}

impl<'a> DrawStep<'a> {
    pub fn draw<F: FnMut(CommandIterator)>(&mut self, f: F) -> ClearStep<'a> {
        unimplemented!()
    }
    /*
    pub fn draw_vertices<F: FnMut(VertexCommands)>(&mut self, f: F) -> ClearGuiStep<'a> {
        unimplemented!()
    }
    */
}

use math::*;
use color::*;
use font::*;
use libc::*;

#[allow(missing_docs)]
pub enum Command<'a> {
    // NK_COMMAND_NOP is never set by Nuklear, so it doesn't make sense to include it here.
    // It makes even less sense to wrap `Command`s into an Option because of it.
    // Users shouldn't be allowed to push Nops either - doing user-specific stuff is what the Custom
    // value is for.
    // ---
    // Nop,
    
    /// Defines a screen-space rectangle beyond which nothing is drawn. Similar to OpenGL's `glScissor()`.
    Scissor { rect: Rect<i16,u16> },
    Line { thickness: u16, start: Vec2<i16>, end: Vec2<i16>, color: Rgba<u8> },
    /// A curve, defined by Nuklear as a line with a pair of "control" points in-between.
    Curve { thickness: u16, start: Vec2<i16>, end: Vec2<i16>, ctrl: [Vec2<i16>; 2], color: Rgba<u8> },
    Rect { rounding: u16, line_thickness: u16, rect: Rect<i16,u16>, color: Rgba<u8> },
    FilledRect { rounding: u16, rect: Rect<i16,u16>, color: Rgba<u8> },
    MultiColorRect { rect: Rect<i16,u16>, left: Rgba<u8>, top: Rgba<u8>, bottom: Rgba<u8>, right: Rgba<u8> },
    Triangle { line_thickness: u16, vertices: [Vec2<i16>; 3], color: Rgba<u8> },
    FilledTriangle { vertices: [Vec2<i16>; 3], color: Rgba<u8> },
    Ellipse { center: Vec2<i16>, size: Vec2<u16>, color: Rgba<u8>, line_thickness: u16, },
    FilledEllipse { center: Vec2<i16>, size: Vec2<u16>, color: Rgba<u8> },
    /// TODO: is it clockwise or counterclockwise ?
    Arc { center: Vec2<i16>, radius: u16, line_thickness: u16, start_angle: f32, end_angle: f32, color: Rgba<u8> },
    /// TODO: is it clockwise or counterclockwise ?
    FilledArc { center: Vec2<i16>, radius: u16, start_angle: f32, end_angle: f32, color: Rgba<u8> },
    Polygon { color: Rgba<u8>, line_thickness: u16, vertices: &'a [Vec2<i16>] },
    FilledPolygon { color: Rgba<u8>, vertices: &'a [Vec2<i16>] },
    Polyline { color:Rgba<u8>, line_thickness: u16, vertices: &'a [Vec2<i16>] },
    Image { rect: Rect<i16,u16>, image: Image, color: Rgba<u8> },
    // XXX string should be an `&str` instead of an `&[u8]`.
    // That would require completely preventing the user from being able to feed non-UTF8 text to the input step.
    // As it is, the user has to manually use `core::str::from_utf8()`.
    // While it relieves us from the responsibility, Nuke as an indiomatic Rust binding should be able 
    // to enforce UTF8 and therefore fearlessly hand back an `&str` which is more comfortable.
    Text { font: &'a VirtualFont, background_color: Rgba<u8>, foreground_color: Rgba<u8>, rect: Rect<i16,u16>, height: f32, string: &'a [u8] },
    Custom { rect: Rect<i16,u16>, custom: &'a mut CustomCommandCallback },
}
pub trait CustomCommandCallback {
    // FIXME Don't use *mut c_void
    fn callback(&mut self, canvas: *mut c_void, rect: Rect<i16,u16>);
}
// TODO Figure out what this is
pub struct Image {}

#[doc(hidden)]
impl From<nk_image> for Image {
    fn from(img: nk_image) -> Self {
        unimplemented!()
    }
}
