//! The input API is responsible for holding the current input state composed of
//! mouse, key and text input states.
//! It is worth noting that no direct os or window handling is done in nuklear.
//! Instead all input state has to be provided by platform specific code. This in one hand
//! expects more work from the user and complicates usage but on the other hand
//! provides simple abstraction over a big number of platforms, libraries and other
//! already provided functionality.
//!
//! Usage
//! -------------------
//! Input state needs to be provided to nuklear by first calling `nk_input_begin`
//! which resets internal state like delta mouse position and button transistions.
//! After `nk_input_begin` all current input state needs to be provided. This includes
//! mouse motion, button and key pressed and released, text input and scrolling.
//! Both event- or state-based input handling are supported by this API
//! and should work without problems. Finally after all input state has been
//! mirrored `nk_input_end` needs to be called to finish input process.
//!
//!     struct nk_context ctx;
//!     nk_init_xxx(&ctx, ...);
//!     while (1) {
//!         Event evt;
//!         nk_input_begin(&ctx);
//!         while (GetEvent(&evt)) {
//!             if (evt.type == MOUSE_MOVE)
//!                 nk_input_motion(&ctx, evt.motion.x, evt.motion.y);
//!             else if (evt.type == ...) {
//!                 ...
//!             }
//!         }
//!         nk_input_end(&ctx);
//!         [...]
//!         nk_clear(&ctx);
//!     }
//!     nk_free(&ctx);
//!
//! Reference
//! -------------------
//! nk_input_begin      - Begins the input mirroring process. Needs to be called before all other `nk_input_xxx` calls
//! nk_input_motion     - Mirrors mouse cursor position
//! nk_input_key        - Mirrors key state with either pressed or released
//! nk_input_button     - Mirrors mouse button state with either pressed or released
//! nk_input_scroll     - Mirrors mouse scroll values
//! nk_input_char       - Adds a single ASCII text character into an internal text buffer
//! nk_input_glyph      - Adds a single multi-byte UTF-8 character into an internal text buffer
//! nk_input_unicode    - Adds a single unicode rune into an internal text buffer
//! nk_input_end        - Ends the input mirroring process by calculating state changes. Don't call any `nk_input_xxx` function referenced above after this call

use core;
use libc;
use nuke_sys::*;
use context::*;
use gui::*;
use math::*;

impl<'a,'b:'a> Context<'b> {
    pub fn input<F: FnMut(InputApi<'a,'b>)>(&'b mut self, mut f: F) -> GuiStep<'b> {
        unsafe {
            let slf = self as *mut Context;
            let raw = &mut self.raw;
            nk_input_begin(raw);
            f(InputApi::new(slf.as_mut().unwrap()));
            nk_input_end(raw);
            GuiStep::new(slf.as_mut().unwrap())
        }
    }
}


// We reference a Context that outlives us
pub struct InputApi<'a,'b:'a> {
    ctx: &'a mut Context<'b>,
}

pub type MouseScroll = Vec2<f32>;
pub type MousePosition = Vec2<i32>;

// FIXME Use the names directly when constant evaluation errors get fixed
#[allow(missing_docs)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Shift = 1,
    Ctrl = 2,
    Del = 3,
    Enter = 4,
    Tab = 5,
    Backspace = 6,
    Copy = 7,
    Cut = 8,
    Paste = 9,
    Up = 10,
    Down = 11,
    Left = 12,
    Right = 13,
    TextInsertMode = 14,
    TextReplaceMode = 15,
    TextResetMode = 16,
    TextLineStart = 17,
    TextLineEnd = 18,
    TextStart = 19,
    TextEnd = 20,
    TextUndo = 21,
    TextRedo = 22,
    TextSelectAll = 23,
    TextWordLeft = 24,
    TextWordRight = 25,
    ScrollStart = 26,
    ScrollEnd = 27,
    ScrollDown = 28,
    ScrollUp = 29,
}
impl Key {
    pub(crate) fn to_nk(self) -> nk_keys {
        unsafe {
            core::mem::transmute(self)
        }
    }
}
#[allow(missing_docs)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left = 0,
    Middle = 1,
    Right = 2,
}
impl MouseButton {
    pub(crate) fn to_nk(self) -> nk_buttons {
        unsafe {
            core::mem::transmute(self)
        }
    }
}

impl<'a,'b:'a> InputApi<'a,'b> {
    pub(crate) fn new(ctx: &'a mut Context<'b>) -> Self {
        Self { ctx }
    }
    /// Mirrors mouse cursor position
    pub fn mouse_position(&mut self, position: MousePosition) {
        unsafe {
            nk_input_motion(&mut self.ctx.raw, position.x, position.y)
        }
    }
    /// Mirrors a pressed key
    pub fn key_pressed(&mut self, key: Key) {
        unsafe {
            nk_input_key(&mut self.ctx.raw, key.to_nk(), 1)
        }
    }
    /// Mirrors a released key
    pub fn key_released(&mut self, key: Key) {
        unsafe {
            nk_input_key(&mut self.ctx.raw, key.to_nk(), 0)
        }
    }
    /// Mirrors a pressed mouse button
    pub fn mouse_button_pressed(&mut self, btn: MouseButton, pos: MousePosition) {
        unsafe {
            nk_input_button(&mut self.ctx.raw, btn.to_nk(), pos.x, pos.y, 1)
        }
    }
    /// Mirrors a pressed mouse button
    pub fn mouse_button_released(&mut self, btn: MouseButton, pos: MousePosition) {
        unsafe {
            nk_input_button(&mut self.ctx.raw, btn.to_nk(), pos.x, pos.y, 0)
        }
    }
    /// Mirrors mouse scroll values
    pub fn mouse_scroll(&mut self, scroll: MouseScroll) { 
        let MouseScroll { x, y } = scroll;
        unsafe {
            nk_input_scroll(&mut self.ctx.raw, nk_vec2 { x, y })
        }
    }
    // XXX: Shouldn't this be forbidden ? This would ensure that we can
    // safely get back an `str` slice from `draw::Command::Text::string`
    // instead of an [u8] slice.
    /// Adds a single ASCII text character into an internal text buffer
    pub fn ascii(&mut self, ascii: u8) { 
        unsafe {
            nk_input_char(&mut self.ctx.raw, ascii as libc::c_char)
        }
    }
    /// Adds a single multi-byte UTF-8 character into an internal text buffer
    pub fn char(&mut self, char: char) {
        unsafe {
            // This is correct. The last argument is actually expected
            // to be a fixed-size 4 bytes array.
            nk_input_glyph(&mut self.ctx.raw, &char as *const _ as *mut _)
        }
    }
    /// Adds a single unicode rune into an internal text buffer
    pub fn unicode_rune(&mut self, char: char) {
        unsafe {
            nk_input_unicode(&mut self.ctx.raw, char as nk_rune)
        }
    }
}