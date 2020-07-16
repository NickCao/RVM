use crate::{HostPhysAddr, HostVirtAddr};

pub fn alloc_frame() -> Option<HostPhysAddr> {
    unsafe { rvm_alloc_frame() }
}

pub fn dealloc_frame(paddr: HostPhysAddr) {
    unsafe { rvm_dealloc_frame(paddr) }
}

/// Convert physical address to virtual address
pub fn phys_to_virt(paddr: HostPhysAddr) -> HostVirtAddr {
    unsafe { rvm_phys_to_virt(paddr) }
}

/// The address where the hardware jumps to when an interrupt occurs, only used on x86.
#[cfg(target_arch = "x86_64")]
pub fn x86_all_traps_handler_addr() -> usize {
    unsafe { rvm_x86_all_traps_handler_addr() }
}

extern "Rust" {
    fn rvm_alloc_frame() -> Option<HostPhysAddr>;
    fn rvm_dealloc_frame(_paddr: HostPhysAddr);
    fn rvm_phys_to_virt(_paddr: HostPhysAddr) -> HostVirtAddr;
    #[cfg(target_arch = "x86_64")]
    fn rvm_x86_all_traps_handler_addr() -> usize;
}
