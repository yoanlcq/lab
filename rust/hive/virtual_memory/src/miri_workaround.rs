use std::collections::BTreeMap;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};

use super::{Addr, AddrRange, PtrRange, Error, Result, ProtectionFlags};

#[derive(Debug, Clone)]
pub struct System {
    reserved_ranges: Arc<Mutex<BTreeMap<AddrRange, Vec<u8>>>>,
    page_size: NonZeroUsize,
    allocation_granularity: NonZeroUsize,
}

impl System {
    pub fn new(page_size: NonZeroUsize, allocation_granularity: NonZeroUsize) -> Self {
        Self {
            reserved_ranges: Default::default(),
            page_size,
            allocation_granularity
        }
    }
    pub fn reserve(&self, _starting_address_hint: Option<Addr>, size: usize) -> Result<AddrRange> {
        let aligned_size = size.next_multiple_of(self.page_size.get());
        let conservative_size = aligned_size + self.allocation_granularity.get() - 1;
        let v = Vec::<u8>::with_capacity(conservative_size);
        let addr = v.as_ptr().addr().next_multiple_of(self.allocation_granularity.get());
        let addr_range = AddrRange::new(Addr::new(addr), aligned_size);
        self.reserved_ranges.lock().unwrap().insert(addr_range, v);
        Ok(addr_range)
    }
    pub fn commit(&self, addr_range: AddrRange, _protection_flags: ProtectionFlags) -> Result<PtrRange> {
        let addr_range = addr_range.covering_page_size(self.page_size);
        let mut reserved_ranges = self.reserved_ranges.lock().unwrap();
        let reserved_range = Self::find_reserved_range(&mut reserved_ranges, addr_range)?;
        todo!()
    }
    pub unsafe fn decommit(&self, ptr_range: PtrRange) -> Result<()> {
        let ptr_range = ptr_range.covering_page_size(self.page_size);
        let mut reserved_ranges = self.reserved_ranges.lock().unwrap();
        let reserved_range = Self::find_reserved_range(&mut reserved_ranges, ptr_range.to_addr_range())?;
        todo!()
    }
    pub unsafe fn unreserve(&self, addr_range: AddrRange) -> Result<()> {
        match self.reserved_ranges.lock().unwrap().remove(&addr_range) {
            Some(_) => Ok(()),
            None => Err(Error::other("addr_range is not a known reserved range")),
        }
    }
    fn find_reserved_range(reserved_ranges: &mut BTreeMap<AddrRange, Vec<u8>>, addr_range: AddrRange) -> Result<(&AddrRange, &mut Vec<u8>)> {
        let addr = addr_range.addr().get();
        for it in reserved_ranges {
            let start = it.0.addr().get();
            let end = start + it.0.size();
            if start <= addr && addr < start + end {
                if !it.0.contains(addr_range) {
                    return Err(Error::other("requested range is not contained within the reserved range"));
                }
                return Ok(it);
            }
        }
        Err(Error::other("could not find a reserved range from the given address"))
    }
}