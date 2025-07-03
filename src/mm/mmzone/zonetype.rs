use core::mem;

use static_assertions::const_assert;

#[derive(Debug, PartialEq)]
pub(crate) enum zone_type {
    /*
     * ZONE_DMA and ZONE_DMA32 are used when there are peripherals not able
     * to DMA to all of the addressable memory (ZONE_NORMAL).
     * On architectures where this area covers the whole 32 bit address
     * space ZONE_DMA32 is used. ZONE_DMA is left for the ones with smaller
     * DMA addressing constraints. This distinction is important as a 32bit
     * DMA mask is assumed when ZONE_DMA32 is defined. Some 64-bit
     * platforms may need both zones as they support peripherals with
     * different DMA addressing limitations.
     */
    // #ifdef CONFIG_ZONE_DMA
    ZONE_DMA,
    // #endif
    // #ifdef CONFIG_ZONE_DMA32
    ZONE_DMA32,
    // #endif
    /*
     * Normal addressable memory is in ZONE_NORMAL. DMA operations can be
     * performed on pages in ZONE_NORMAL if the DMA devices support
     * transfers to all addressable memory.
     */
    ZONE_NORMAL,
    // #ifdef CONFIG_HIGHMEM
    //     /*
    //      * A memory area that is only addressable by the kernel through
    //      * mapping portions into its own address space. This is for example
    //      * used by i386 to allow the kernel to address the memory beyond
    //      * 900MB. The kernel will set up special mappings (page
    //      * table entries on i386) for each page that the kernel needs to
    //      * access.
    //      */
    //     ZONE_HIGHMEM,
    // #endif
    /*
     * ZONE_MOVABLE is similar to ZONE_NORMAL, except that it contains
     * movable pages with few exceptional cases described below. Main use
     * cases for ZONE_MOVABLE are to make memory offlining/unplug more
     * likely to succeed, and to locally limit unmovable allocations - e.g.,
     * to increase the number of THP/huge pages. Notable special cases are:
     *
     * 1. Pinned pages: (long-term) pinning of movable pages might
     *    essentially turn such pages unmovable. Therefore, we do not allow
     *    pinning long-term pages in ZONE_MOVABLE. When pages are pinned and
     *    faulted, they come from the right zone right away. However, it is
     *    still possible that address space already has pages in
     *    ZONE_MOVABLE at the time when pages are pinned (i.e. user has
     *    touches that memory before pinning). In such case we migrate them
     *    to a different zone. When migration fails - pinning fails.
     * 2. memblock allocations: kernelcore/movablecore setups might create
     *    situations where ZONE_MOVABLE contains unmovable allocations
     *    after boot. Memory offlining and allocations fail early.
     * 3. Memory holes: kernelcore/movablecore setups might create very rare
     *    situations where ZONE_MOVABLE contains memory holes after boot,
     *    for example, if we have sections that are only partially
     *    populated. Memory offlining and allocations fail early.
     * 4. PG_hwpoison pages: while poisoned pages can be skipped during
     *    memory offlining, such pages cannot be allocated.
     * 5. Unmovable PG_offline pages: in paravirtualized environments,
     *    hotplugged memory blocks might only partially be managed by the
     *    buddy (e.g., via XEN-balloon, Hyper-V balloon, virtio-mem). The
     *    parts not manged by the buddy are unmovable PG_offline pages. In
     *    some cases (virtio-mem), such pages can be skipped during
     *    memory offlining, however, cannot be moved/allocated. These
     *    techniques might use alloc_contig_range() to hide previously
     *    exposed pages from the buddy again (e.g., to implement some sort
     *    of memory unplug in virtio-mem).
     * 6. ZERO_PAGE(0), kernelcore/movablecore setups might create
     *    situations where ZERO_PAGE(0) which is allocated differently
     *    on different platforms may end up in a movable zone. ZERO_PAGE(0)
     *    cannot be migrated.
     * 7. Memory-hotplug: when using memmap_on_memory and onlining the
     *    memory to the MOVABLE zone, the vmemmap pages are also placed in
     *    such zone. Such pages cannot be really moved around as they are
     *    self-stored in the range, but they are treated as movable when
     *    the range they describe is about to be offlined.
     *
     * In general, no unmovable allocations that degrade memory offlining
     * should end up in ZONE_MOVABLE. Allocators (like alloc_contig_range())
     * have to expect that migrating pages in ZONE_MOVABLE can fail (even
     * if has_unmovable_pages() states that there are no unmovable pages,
     * there can be false negatives).
     */
    ZONE_MOVABLE,
    // #ifdef CONFIG_ZONE_DEVICE
    //     ZONE_DEVICE,
    // #endif
    // __MAX_NR_ZONES,
}

impl zone_type {

    pub const MAX_NR_ZONES: usize = mem::variant_count::<Self>();

    /// When a memory allocation must conform to specific limitations (such
    /// as being suitable for DMA) the caller will pass in hints to the
    /// allocator in the gfp_mask, in the zone modifier bits.  These bits
    /// are used to select a priority ordered list of memory zones which
    /// match the requested limits. See gfp_zone() in include/linux/gfp.h
    pub const ZONES_SHIFT: usize = {
        const_assert!(zone_type::MAX_NR_ZONES <= 8usize);
        match zone_type::MAX_NR_ZONES {
            0 | 1 => 0,
            2 => 1,
            3 | 4 => 2,
            5..=8 => 3,
            // irrelevant as we are already after a compile check
            _ => 0,
        }
    };
}

impl TryFrom<usize> for zone_type {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            val if val == zone_type::ZONE_DMA as usize => Ok(zone_type::ZONE_DMA),
            val if val == zone_type::ZONE_DMA32 as usize => Ok(zone_type::ZONE_DMA32),
            val if val == zone_type::ZONE_NORMAL as usize => Ok(zone_type::ZONE_NORMAL),
            val if val == zone_type::ZONE_MOVABLE as usize => Ok(zone_type::ZONE_MOVABLE),
            _ => Err("Got invalid value for zone_type")
        }
    }
}
