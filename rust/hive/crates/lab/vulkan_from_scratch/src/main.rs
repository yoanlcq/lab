#![expect(unused_crate_dependencies, reason = "This is essentially the same as the library")]

fn main() -> vulkan_from_scratch_lib::gpu::Result<()> {
    vulkan_from_scratch_lib::main()
}
