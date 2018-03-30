//! Editing text in this library is handled by either `nk_edit_string` or
//! `nk_edit_buffer`. But like almost everything in this library there are multiple
//! ways of doing it and a balance between control and ease of use with memory
//! as well as functionality controlled by flags.
//! 
//! This library generally allows three different levels of memory control:
//! First of is the most basic way of just providing a simple char array with
//! string length. This method is probably the easiest way of handling simple
//! user text input. Main upside is complete control over memory while the biggest
//! downside in comparsion with the other two approaches is missing undo/redo.
//! 
//! For UIs that require undo/redo the second way was created. It is based on
//! a fixed size nk_text_edit struct, which has an internal undo/redo stack.
//! This is mainly useful if you want something more like a text editor but don't want
//! to have a dynamically growing buffer.
//! 
//! The final way is using a dynamically growing nk_text_edit struct, which
//! has both a default version if you don't care where memory comes from and an
//! allocator version if you do. While the text editor is quite powerful for its
//! complexity I would not recommend editing gigabytes of data with it.
//! It is rather designed for uses cases which make sense for a GUI library not for
//! an full blown text editor.
 