use virtual_memory::*;

fn print_page_range_info(r: &PageRangeInfo) {
    print!("{{ ");
    print!("addr: {:#018x}", r.addr().get());
    print!(", size: {:#018x}", r.size());
    print!(", state: {:?}", r.state());
    if r.state() != PageState::Free {
        print!(", allocation: ({:#018x}", r.allocation_addr().unwrap().get());
        if let Some(prot) = r.allocation_os_protection_flags() {
            print!(", prot: {:#010x} ({:?})", prot.0, ProtectionFlags::from_os_lossy(prot));
        }
        print!(")");

        if r.state() != PageState::Reserved {
            print!(", prot: {:#010x} ({:?})", r.os_protection_flags().unwrap().0, r.protection_flags_lossy().unwrap());
        }
        print!(", type: {:?}", r.type_().unwrap());
    }
    print!(" }}");
}

fn main() {
    let vms = VirtualMemorySystem::new();

    let mut num_ranges = 0;
    for it in vms.page_range_info_iter(Addr::new(0)) {
        match it {
            Ok(r) => {
                print_page_range_info(&r);
                println!();
                num_ranges += 1;
            },
            Err(e) => {
                println!("Error (this may be normal if this is the last iteration): {}", e);
            }
        }
    }
    println!("Listed {} page ranges", num_ranges);
}