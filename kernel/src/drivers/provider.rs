use crate::memory::PAGE_SIZE;
use isomorphic_drivers::provider;
pub struct Provider;

impl provider::Provider for Provider {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_dma(size: usize) -> (usize, usize) {
        // let paddr = virtio_dma_alloc(size / PAGE_SIZE);
        // let vaddr = phys_to_virt(paddr);
        // (vaddr, paddr)
        (1 as usize, 1 as usize)
    }

    fn dealloc_dma(vaddr: usize, size: usize) {
        // let paddr = virt_to_phys(vaddr);
        // for i in 0..size / PAGE_SIZE {
        //     dealloc_frame(paddr + i * PAGE_SIZE);
        // }
    }
}

//todo ?
