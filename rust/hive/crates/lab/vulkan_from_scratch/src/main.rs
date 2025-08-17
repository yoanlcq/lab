#![expect(unused_crate_dependencies, reason = "This is essentially the same as the library")]

#[global_allocator]
static GLOBAL_ALLOCATOR: tracing_facade::GlobalAllocatorWrapper<std::alloc::System> =
    tracing_facade::GlobalAllocatorWrapper::new(std::alloc::System, 100);

fn main() -> vulkan_from_scratch_lib::gpu::Result<()> {
    vulkan_from_scratch_lib::main()
}
