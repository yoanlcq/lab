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
// Clippy "restriction" lints
#![allow(clippy::absolute_paths, reason = "These may not be idiomatic but are fine")]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::allow_attributes)]
#![warn(clippy::allow_attributes_without_reason)]
#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "This one is way too far-reaching, but most importantly, alphabetical order of declarations is almost never the best one for \
              readers. For instance you generally want the new() method to appear first. You also generally want to keep related \
              functions close together and alphabetical order should have no say in that"
)]
#![allow(clippy::arithmetic_side_effects, reason = "TODO")]
#![allow(clippy::as_conversions, reason = "TODO")]
#![allow(clippy::as_pointer_underscore, reason = "TODO")]
#![allow(clippy::as_underscore, reason = "TODO")]
#![warn(clippy::assertions_on_result_states)]
#![allow(clippy::big_endian_bytes, reason = "TODO")]
#![allow(clippy::cfg_not_test, reason = "TODO")]
#![allow(clippy::clone_on_ref_ptr, reason = "TODO")]
#![allow(clippy::cognitive_complexity, reason = "TODO")]
#![allow(clippy::create_dir, reason = "TODO")]
#![allow(clippy::dbg_macro, reason = "TODO")]
#![allow(clippy::decimal_literal_representation, reason = "TODO")]
#![allow(clippy::default_numeric_fallback, reason = "TODO")]
#![allow(clippy::default_union_representation, reason = "TODO")]
#![allow(clippy::deref_by_slicing, reason = "TODO")]
#![warn(clippy::disallowed_script_idents)]
#![warn(clippy::doc_include_without_cfg)]
#![allow(clippy::else_if_without_else, reason = "TODO")]
#![allow(clippy::empty_drop, reason = "TODO")]
#![warn(clippy::empty_enum_variants_with_brackets)]
#![warn(clippy::empty_structs_with_brackets)]
#![allow(clippy::error_impl_error, reason = "TODO")]
#![allow(clippy::exhaustive_enums, reason = "TODO")]
#![allow(clippy::exhaustive_structs, reason = "TODO")]
#![warn(clippy::exit)]
#![allow(
    clippy::expect_used,
    reason = "Sometimes the only alternative is unsafe code. See also clippy::unwrap_used"
)]
#![allow(clippy::field_scoped_visibility_modifiers, reason = "TODO")]
#![warn(clippy::filetype_is_file)]
#![allow(clippy::float_arithmetic, reason = "TODO")]
#![allow(clippy::float_cmp_const, reason = "TODO")]
#![allow(clippy::fn_to_numeric_cast_any, reason = "TODO")]
#![warn(clippy::get_unwrap)]
#![allow(clippy::host_endian_bytes, reason = "TODO")]
#![warn(clippy::if_then_some_else_none)]
#![allow(clippy::impl_trait_in_params, reason = "TODO")]
#![allow(clippy::implicit_return, reason = "TODO")]
#![allow(clippy::indexing_slicing, reason = "TODO")]
#![warn(clippy::infinite_loop)]
#![allow(clippy::inline_asm_x86_att_syntax, reason = "TODO")]
#![allow(clippy::inline_asm_x86_intel_syntax, reason = "TODO")]
#![allow(clippy::integer_division, reason = "TODO")]
#![allow(clippy::integer_division_remainder_used, reason = "TODO")]
#![allow(clippy::iter_over_hash_type, reason = "TODO")]
#![allow(clippy::large_include_file, reason = "TODO")]
#![warn(clippy::let_underscore_must_use)]
#![allow(clippy::let_underscore_untyped, reason = "TODO")]
#![allow(clippy::little_endian_bytes, reason = "TODO")]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::map_err_ignore)]
#![warn(clippy::map_with_unused_argument_over_ranges)]
#![allow(clippy::mem_forget, reason = "TODO")]
#![allow(clippy::min_ident_chars, reason = "TODO")]
#![warn(clippy::missing_assert_message)]
#![allow(clippy::missing_asserts_for_indexing, reason = "TODO")]
#![allow(clippy::missing_docs_in_private_items, reason = "TODO")]
#![allow(clippy::missing_inline_in_public_items, reason = "TODO")]
#![allow(clippy::missing_trait_methods, reason = "TODO")]
#![allow(clippy::mixed_read_write_in_expression, reason = "TODO")]
#![allow(
    clippy::mod_module_files,
    reason = "This is the opposite of the clippy::self_named_module_files lint that we prefer because it avoids having one file and one \
              folder with the same name that must be kept in sync. Even though having multiple mod.rs tabs opened can be confusing"
)]
#![warn(clippy::module_name_repetitions)]
#![allow(clippy::modulo_arithmetic, reason = "TODO")]
#![allow(clippy::multiple_inherent_impl, reason = "TODO")]
#![allow(clippy::multiple_unsafe_ops_per_block, reason = "TODO")]
#![warn(clippy::mutex_atomic)]
#![warn(clippy::mutex_integer)]
#![warn(clippy::needless_raw_strings)]
#![allow(clippy::non_ascii_literal, reason = "TODO")]
#![warn(clippy::non_zero_suggestions)]
#![warn(clippy::panic)]
#![warn(clippy::panic_in_result_fn)]
#![allow(clippy::partial_pub_fields, reason = "TODO")]
#![warn(clippy::pathbuf_init_then_push)]
#![warn(clippy::pattern_type_mismatch)]
// #![allow(clippy::pointer_format, reason = "TODO")] // Not available in the version I'm using
#![warn(clippy::precedence_bits)]
#![allow(clippy::print_stderr, reason = "TODO")]
#![allow(clippy::print_stdout, reason = "TODO")]
#![allow(clippy::pub_use, reason = "TODO")]
#![allow(clippy::pub_with_shorthand, reason = "TODO")]
#![allow(clippy::pub_without_shorthand, reason = "TODO")]
#![allow(clippy::question_mark_used, reason = "TODO")]
#![warn(clippy::rc_buffer)]
#![warn(clippy::rc_mutex)]
#![warn(clippy::redundant_test_prefix)]
#![warn(clippy::redundant_type_annotations)]
#![allow(
    clippy::ref_patterns,
    reason = "You literally have no other choice when destructuring a non-Copy member out of a struct ref"
)]
#![warn(clippy::renamed_function_params)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::return_and_then)]
#![allow(clippy::same_name_method, reason = "TODO")]
#![warn(clippy::self_named_module_files)] // NOTE: Opposite of clippy::mod_module_files
#![allow(clippy::semicolon_inside_block, reason = "TODO")]
#![warn(clippy::semicolon_outside_block)]
#![warn(clippy::separated_literal_suffix)]
#![allow(clippy::shadow_reuse, reason = "TODO")]
#![allow(clippy::shadow_same, reason = "TODO")]
#![allow(clippy::shadow_unrelated, reason = "TODO")]
#![allow(clippy::single_call_fn, reason = "TODO")]
#![allow(clippy::single_char_lifetime_names, reason = "TODO")]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::str_to_string)]
#![allow(clippy::string_add, reason = "TODO")]
#![allow(clippy::string_lit_chars_any, reason = "TODO")]
#![allow(clippy::string_slice, reason = "TODO")]
#![warn(clippy::suspicious_xor_used_as_pow)]
#![warn(clippy::tests_outside_test_module)]
#![allow(clippy::todo, reason = "TODO")]
#![warn(clippy::try_err)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::unimplemented, reason = "TODO")]
#![warn(clippy::unnecessary_safety_comment)]
#![warn(clippy::unnecessary_safety_doc)]
#![warn(clippy::unnecessary_self_imports)]
#![allow(clippy::unneeded_field_pattern, reason = "TODO")]
#![allow(clippy::unreachable, reason = "TODO")]
#![allow(clippy::unseparated_literal_suffix, reason = "TODO")]
#![warn(clippy::unused_result_ok)]
#![allow(clippy::unused_trait_names, reason = "TODO")]
#![warn(clippy::unwrap_in_result)]
#![allow(
    clippy::unwrap_used,
    reason = "I want this, but it gets really annoying for specific frequent cases such as Mutex::lock(), Weak::upgrade(), and \
              downcast_ref. See also clippy::expect_used"
)]
#![allow(clippy::use_debug, reason = "TODO")]
#![warn(clippy::verbose_file_reads)]
#![allow(clippy::wildcard_enum_match_arm, reason = "TODO")]

use std::time::Instant;

use crate::gpu::{ApiArc, ApiParams, ApiSpec, DeviceParams};
use crate::windowing::WindowParams;

extern crate alloc;
extern crate ash;
extern crate ash_window;
extern crate raw_window_handle;
extern crate windows;

pub mod as_any;
pub mod debugger;
pub mod delegates;
pub mod gpu;
pub mod result_hole;
pub mod weak_self;
pub mod windowing;

struct StartupProfiler {
    startup_instant: Instant,
}

impl StartupProfiler {
    pub fn new() -> Self {
        let this = Self {
            startup_instant: Instant::now(),
        };
        Self::log_exe_startup_time();
        this
    }

    fn log_exe_startup_time() {
        match std::env::current_exe() {
            Err(e) => eprintln!("Getting current exe path failed: {e}"),
            Ok(exe_path) => {
                match std::fs::metadata(exe_path) {
                    Err(e) => eprintln!("Getting current exe metadata failed: {e}"),
                    Ok(metadata) => {
                        match metadata.accessed() {
                            Err(e) => eprintln!("Last access time is not available on this file system: {e}"),
                            Ok(last_access_time) => {
                                match std::time::SystemTime::now().duration_since(last_access_time) {
                                    Err(e) => eprintln!("Failed to get elapsed time between exe last access and current time: {e}"),
                                    Ok(duration) => println!("[Startup] Time since exe last access: {:.3} seconds", duration.as_secs_f32()),
                                }
                            },
                        }
                    },
                }
            }
        }
    }

    pub fn log_step(&self, description: &str) {
        let elapsed = Instant::now().duration_since(self.startup_instant).as_secs_f32();
        println!("[Startup][{elapsed:.3}] {description}");
    }
}

pub fn main() -> gpu::Result<()> {
    let startup_profiler = StartupProfiler::new();

    let window_params = WindowParams {
        title: "Vulkan experiment".to_owned(),
        position: None,
        size: None,
    };

    let display = windowing::DisplayArc::open(&windowing::DisplayParams {})?;
    let window0 = display.create_window(&window_params)?;
    let window1 = display.create_window(&window_params)?;
    window0.show()?;
    window1.show()?;
    startup_profiler.log_step("Window showed");

    let api = ApiArc::create(&ApiParams { spec: ApiSpec::Vulkan })?;
    startup_profiler.log_step("GPU API created");
    let _device = api.create_device(&DeviceParams {})?;
    startup_profiler.log_step("GPU Device created");
    // let _swapchain0 = device.create_swap_chain(&SwapChainParams { window: &window0 })?;
    // let _swapchain1 = device.create_swap_chain(&SwapChainParams { window: &window1 })?;

    startup_profiler.log_step("Main event loop starting");
    display.main_event_loop();

    Ok(())
}
