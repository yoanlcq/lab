#![warn(
    clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]
#![allow(clippy::missing_docs_in_private_items, reason = "This is a personal experiment")]
#![allow(clippy::cargo_common_metadata, reason = "This isn't meant to be published")]
#![allow(clippy::missing_errors_doc, reason = "This is a personal experiment")]
#![warn(clippy::expect_used)]
#![warn(clippy::unwrap_used)]
// Clippy "restriction" lints
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::allow_attributes)]
#![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::assertions_on_result_states)]
#![warn(clippy::disallowed_script_idents)]
#![warn(clippy::doc_include_without_cfg)]
#![warn(clippy::empty_enum_variants_with_brackets)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::exit)]
#![warn(clippy::filetype_is_file)]
#![warn(clippy::get_unwrap)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::infinite_loop)]
#![warn(clippy::let_underscore_must_use)]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::map_err_ignore)]
#![warn(clippy::map_with_unused_argument_over_ranges)]
#![warn(clippy::missing_assert_message)]
#![warn(clippy::module_name_repetitions)]
#![warn(clippy::mutex_atomic)]
#![warn(clippy::mutex_integer)]
#![warn(clippy::needless_raw_strings)]
#![warn(clippy::non_zero_suggestions)]
#![warn(clippy::panic)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::pathbuf_init_then_push)]
#![warn(clippy::pattern_type_mismatch)]
#![warn(clippy::precedence_bits)]
#![warn(clippy::rc_buffer)]
#![warn(clippy::rc_mutex)]
#![warn(clippy::redundant_test_prefix)]
#![warn(clippy::redundant_type_annotations)]
#![warn(clippy::renamed_function_params)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::return_and_then)]
#![warn(clippy::self_named_module_files)]
#![warn(clippy::semicolon_outside_block)]
#![warn(clippy::separated_literal_suffix)]
// #![warn(clippy::std_instead_of_alloc)] // TODO
// #![warn(clippy::std_instead_of_core)] // TODO
#![warn(clippy::str_to_string)]
#![warn(clippy::suspicious_xor_used_as_pow)]
#![warn(clippy::tests_outside_test_module)]
#![warn(clippy::try_err)]
// #![warn(clippy::undocumented_unsafe_blocks)] // TODO
#![warn(clippy::unnecessary_safety_comment)]
#![warn(clippy::unnecessary_safety_doc)]
#![warn(clippy::unnecessary_self_imports)]
#![warn(clippy::unused_result_ok)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::verbose_file_reads)]

use crate::window::WindowParams;

extern crate ash;
extern crate ash_window;
extern crate raw_window_handle;
extern crate windows;

pub mod gpu;
pub mod result_hole;
pub mod window;

pub fn main() -> std::io::Result<()> {
    gpu::test();

    let window_params = WindowParams {
        title: "Vulkan experiment".to_owned(),
        position: None,
        size: None,
    };

    let display = window::Display::open(&window::DisplayParams {})?;
    let window0 = display.create_window(&window_params)?;
    let window1 = display.create_window(&window_params)?;
    window0.show();
    window1.show();
    display.main_event_loop();

    Ok(())
}
