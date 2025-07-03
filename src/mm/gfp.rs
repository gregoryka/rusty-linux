use core::hint::cold_path;

use bitflags::bitflags;
use static_assertions::const_assert;

use crate::{
    arch::current::get_current, mm::mmzone::{migratetype::migratetype, pglist_data::zonelists}, sched::task::flags::process_flags, utils::bitops::BITS_PER_USIZE
};

use super::mmzone::zone_type;

enum GFP_BITS {
    ___GFP_DMA_BIT,
    ___GFP_HIGHMEM_BIT,
    ___GFP_DMA32_BIT,
    ___GFP_MOVABLE_BIT,
    ___GFP_RECLAIMABLE_BIT,
    ___GFP_HIGH_BIT,
    ___GFP_IO_BIT,
    ___GFP_FS_BIT,
    ___GFP_ZERO_BIT,
    ___GFP_UNUSED_BIT, /* 0x200u unused */
    ___GFP_DIRECT_RECLAIM_BIT,
    ___GFP_KSWAPD_RECLAIM_BIT,
    ___GFP_WRITE_BIT,
    ___GFP_NOWARN_BIT,
    ___GFP_RETRY_MAYFAIL_BIT,
    ___GFP_NOFAIL_BIT,
    ___GFP_NORETRY_BIT,
    ___GFP_MEMALLOC_BIT,
    ___GFP_COMP_BIT,
    ___GFP_NOMEMALLOC_BIT,
    ___GFP_HARDWALL_BIT,
    ___GFP_THISNODE_BIT,
    ___GFP_ACCOUNT_BIT,
    ___GFP_ZEROTAGS_BIT,
    // #ifdef CONFIG_KASAN_HW_TAGS
    //     ___GFP_SKIP_ZERO_BIT,
    //     ___GFP_SKIP_KASAN_BIT,
    // #endif
    // #ifdef CONFIG_LOCKDEP
    //     ___GFP_NOLOCKDEP_BIT,
    // #endif
    // #ifdef CONFIG_SLAB_OBJ_EXT
    //     ___GFP_NO_OBJ_EXT_BIT,
    // #endif
    LAST_BIT,
}

bitflags! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub(super) struct GFP: u32 {
        /*
        * Physical address zone modifiers (see linux/mmzone.h - low four bits)
        */
        const DMA = 1 << 0;
        const HIGHMEM = 1 << 1;
        const DMA32 = 1 << 2;
        const MOVABLE = 1 << 3; /* ZONE_MOVABLE allowed */
        const RECLAIMABLE = 1 << 4;
        const HIGH = 1 << 5;
        const IO = 1 << 6;
        const FS = 1 << 7;
        const ZERO = 1 << 8;
        /* 0x200u unused */

        /**
         * DOC: Reclaim modifiers
         *
         * Reclaim modifiers
         * -----------------
         * Please note that all the following flags are only applicable to sleepable
         * allocations (e.g. %GFP_NOWAIT and %GFP_ATOMIC will ignore them).
         *
         * %__GFP_IO can start physical IO.
         *
         * %__GFP_FS can call down to the low-level FS. Clearing the flag avoids the
         * allocator recursing into the filesystem which might already be holding
         * locks.
         *
         * %__GFP_DIRECT_RECLAIM indicates that the caller may enter direct reclaim.
         * This flag can be cleared to avoid unnecessary delays when a fallback
         * option is available.
         *
         * %__GFP_KSWAPD_RECLAIM indicates that the caller wants to wake kswapd when
         * the low watermark is reached and have it reclaim pages until the high
         * watermark is reached. A caller may wish to clear this flag when fallback
         * options are available and the reclaim is likely to disrupt the system. The
         * canonical example is THP allocation where a fallback is cheap but
         * reclaim/compaction may cause indirect stalls.
         *
         * %__GFP_RECLAIM is shorthand to allow/forbid both direct and kswapd reclaim.
         *
         * The default allocator behavior depends on the request size. We have a concept
         * of so-called costly allocations (with order > %PAGE_ALLOC_COSTLY_ORDER).
         * !costly allocations are too essential to fail so they are implicitly
         * non-failing by default (with some exceptions like OOM victims might fail so
         * the caller still has to check for failures) while costly requests try to be
         * not disruptive and back off even without invoking the OOM killer.
         * The following three modifiers might be used to override some of these
         * implicit rules. Please note that all of them must be used along with
         * %__GFP_DIRECT_RECLAIM flag.
         *
         * %__GFP_NORETRY: The VM implementation will try only very lightweight
         * memory direct reclaim to get some memory under memory pressure (thus
         * it can sleep). It will avoid disruptive actions like OOM killer. The
         * caller must handle the failure which is quite likely to happen under
         * heavy memory pressure. The flag is suitable when failure can easily be
         * handled at small cost, such as reduced throughput.
         *
         * %__GFP_RETRY_MAYFAIL: The VM implementation will retry memory reclaim
         * procedures that have previously failed if there is some indication
         * that progress has been made elsewhere.  It can wait for other
         * tasks to attempt high-level approaches to freeing memory such as
         * compaction (which removes fragmentation) and page-out.
         * There is still a definite limit to the number of retries, but it is
         * a larger limit than with %__GFP_NORETRY.
         * Allocations with this flag may fail, but only when there is
         * genuinely little unused memory. While these allocations do not
         * directly trigger the OOM killer, their failure indicates that
         * the system is likely to need to use the OOM killer soon.  The
         * caller must handle failure, but can reasonably do so by failing
         * a higher-level request, or completing it only in a much less
         * efficient manner.
         * If the allocation does fail, and the caller is in a position to
         * free some non-essential memory, doing so could benefit the system
         * as a whole.
         *
         * %__GFP_NOFAIL: The VM implementation _must_ retry infinitely: the caller
         * cannot handle allocation failures. The allocation could block
         * indefinitely but will never return with failure. Testing for
         * failure is pointless.
         * It _must_ be blockable and used together with __GFP_DIRECT_RECLAIM.
         * It should _never_ be used in non-sleepable contexts.
         * New users should be evaluated carefully (and the flag should be
         * used only when there is no reasonable failure policy) but it is
         * definitely preferable to use the flag rather than opencode endless
         * loop around allocator.
         * Allocating pages from the buddy with __GFP_NOFAIL and order > 1 is
         * not supported. Please consider using kvmalloc() instead.
         */
        const DIRECT_RECLAIM = 1 << 10;  /* Caller can reclaim */
        const KSWAPD_RECLAIM = 1 << 11;  /* kswapd can wake */
        const WRITE = 1 << 12;
        const NOWARN = 1 << 13;
        const RETRY_MAYFAIL = 1 << 14;
        const NOFAIL= 1 << 15;
        const NORETRY= 1 << 16;
        const MEMALLOC= 1 << 17;
        const COMP= 1 << 18;
        const NOMEMALLOC= 1 << 19;
        const HARDWALL= 1 << 20;
        const THISNODE= 1 << 21;
        const ACCOUNT= 1 << 22;
        const ZEROTAGS= 1 << 23;

        const test = 0;

        const RECLAIM =((GFP::DIRECT_RECLAIM.bits() | GFP::KSWAPD_RECLAIM.bits()));

        // #ifdef CONFIG_KASAN_HW_TAGS
        // #define ___GFP_SKIP_ZERO    BIT(___GFP_SKIP_ZERO_BIT)
        // #define ___GFP_SKIP_KASAN    BIT(___GFP_SKIP_KASAN_BIT)
        // #else
        // #define ___GFP_SKIP_ZERO    0
        // #define ___GFP_SKIP_KASAN    0
        // #endif
        // #ifdef CONFIG_LOCKDEP
        // #define ___GFP_NOLOCKDEP    BIT(___GFP_NOLOCKDEP_BIT)
        // #else
        // #define ___GFP_NOLOCKDEP    0
        // #endif
        // #ifdef CONFIG_SLAB_OBJ_EXT
        // #define ___GFP_NO_OBJ_EXT       BIT(___GFP_NO_OBJ_EXT_BIT)
        // #else
        // #define ___GFP_NO_OBJ_EXT       0
        // #endif

        const ZONEMASK = GFP::DMA.bits() | GFP::HIGHMEM.bits() | GFP::DMA32.bits() | GFP::MOVABLE.bits();
        /* Do not use these with a slab allocator */
        const SLAB_BUG_MASK = GFP::DMA32.bits() | GFP::HIGHMEM.bits();

        /*
        * The set of flags that only affect watermark checking and reclaim
        * behaviour. This is used by the MM to obey the caller constraints
        * about IO, FS and watermark checking while ignoring placement
        * hints such as HIGHMEM usage.
        */
        // cfg LOCKDEP
        // let mask = mask | GFP::NOLOCKDEP.bits();
        const RECLAIM_MASK =
        GFP::RECLAIM.bits()
            | GFP::HIGH.bits()
            | GFP::IO.bits()
            | GFP::FS.bits()
            | GFP::NOWARN.bits()
            | GFP::RETRY_MAYFAIL.bits()
            | GFP::NOFAIL.bits()
            | GFP::NORETRY.bits()
            | GFP::MEMALLOC.bits()
            | GFP::NOMEMALLOC.bits();
        /* Control allocation cpuset and node placement constraints */
        const CONSTRAINT_MASK = GFP::HARDWALL.bits() | GFP::THISNODE.bits();

        /* The GFP flags allowed during early boot */
        const BOOT_MASK = (((1 << GFP_BITS::LAST_BIT as u32) - 1) & !(GFP::RECLAIM.bits()|GFP::IO.bits()|GFP::FS.bits()));
        // const __GFP_BITS_MASK is for "unknown bits" - bitflags takes care of this
    }
}

impl GFP {
    pub fn kmalloc_fix_flags(self) -> Self {
        // gfp_t invalid_mask = flags & GFP_SLAB_BUG_MASK;
        // pr_warn("Unexpected gfp: %#x (%pGg). Fixing up to gfp: %#x (%pGg). Fix your code!\n", invalid_mask, &invalid_mask, flags, &flags);
        // dump_stack();

        self.difference(GFP::SLAB_BUG_MASK)
    }

    pub fn filter_allowed_mask(self) -> Self {
        self & gfp_allowed_mask
    }

    /*
     * Applies per-task gfp context to the given allocation flags.
     * PF_MEMALLOC_NOIO implies GFP_NOIO
     * PF_MEMALLOC_NOFS implies GFP_NOFS
     * PF_MEMALLOC_PIN  implies !GFP_MOVABLE
     */
    #[inline]
    pub fn current_gfp_context(mut self) -> Self {
        // TODO READ_ONCE(current->flags)
        let pflags = unsafe { get_current().flags };

        if pflags.intersects(
            process_flags::MEMALLOC_NOIO
                | process_flags::MEMALLOC_NOFS
                | process_flags::MEMALLOC_PIN,
        ) {
            cold_path();
            /*
             * NOIO implies both NOIO and NOFS and it is a weaker context
             * so always make sure it makes precedence
             */
            if pflags.contains(process_flags::MEMALLOC_NOIO) {
                self.remove(GFP::IO | GFP::FS);
            } else if pflags.contains(process_flags::MEMALLOC_NOFS) {
                self.remove(GFP::FS);
            }
            if pflags.contains(process_flags::MEMALLOC_PIN) {
                self.remove(GFP::MOVABLE);
            }
        }
        self
    }

    /*
     * GFP_ZONE_TABLE is a word size bitstring that is used for looking up the
     * zone to use given the lowest 4 bits of gfp_t. Entries are GFP_ZONES_SHIFT
     * bits long and there are 16 of them to cover all possible combinations of
     * __GFP_DMA, __GFP_DMA32, __GFP_MOVABLE and __GFP_HIGHMEM.
     *
     * The zone fallback order is MOVABLE=>HIGHMEM=>NORMAL=>DMA32=>DMA.
     * But GFP_MOVABLE is not only a zone specifier but also an allocation
     * policy. Therefore __GFP_MOVABLE plus another zone selector is valid.
     * Only 1 bit of the lowest 3 bits (DMA,DMA32,HIGHMEM) can be set to "1".
     *
     *       bit       result
     *       =================
     *       0x0    => NORMAL
     *       0x1    => DMA or NORMAL
     *       0x2    => HIGHMEM or NORMAL
     *       0x3    => BAD (DMA+HIGHMEM)
     *       0x4    => DMA32 or NORMAL
     *       0x5    => BAD (DMA+DMA32)
     *       0x6    => BAD (HIGHMEM+DMA32)
     *       0x7    => BAD (HIGHMEM+DMA32+DMA)
     *       0x8    => NORMAL (MOVABLE+0)
     *       0x9    => DMA or NORMAL (MOVABLE+DMA)
     *       0xa    => MOVABLE (Movable is valid only if HIGHMEM is set too)
     *       0xb    => BAD (MOVABLE+HIGHMEM+DMA)
     *       0xc    => DMA32 or NORMAL (MOVABLE+DMA32)
     *       0xd    => BAD (MOVABLE+DMA32+DMA)
     *       0xe    => BAD (MOVABLE+DMA32+HIGHMEM)
     *       0xf    => BAD (MOVABLE+DMA32+HIGHMEM+DMA)
     *
     * GFP_ZONES_SHIFT must be <= 2 on 32 bit platforms.
     */
    pub fn gfp_zone(self) -> Option<zone_type> {
        // #if defined(CONFIG_ZONE_DEVICE) && (MAX_NR_ZONES-1) <= 4
        /* ZONE_DEVICE is not a valid GFP zone specifier */
        // #define GFP_ZONES_SHIFT 2
        // #else
        const ZONES_SHIFT: usize = zone_type::ZONES_SHIFT;
        // #endif

        // #if 16 * GFP_ZONES_SHIFT > BITS_PER_LONG
        // #error GFP_ZONES_SHIFT too large to create GFP_ZONE_TABLE integer
        // #endif

        const ZONE_TABLE: usize = {
            const_assert!(16 * ZONES_SHIFT <= BITS_PER_USIZE);
            const OPT_ZONE_NORMAL: usize = zone_type::ZONE_NORMAL as usize;
            // #ifdef CONFIG_HIGHMEM
            // #define OPT_ZONE_HIGHMEM ZONE_HIGHMEM
            // #else
            const OPT_ZONE_HIGHMEM: usize = zone_type::ZONE_NORMAL as usize;
            // #endif

            // #ifdef CONFIG_ZONE_DMA
            const OPT_ZONE_DMA: usize = zone_type::ZONE_DMA as usize;
            // #else
            // #define OPT_ZONE_DMA ZONE_NORMAL
            // #endif

            // #ifdef CONFIG_ZONE_DMA32
            const OPT_ZONE_DMA32: usize = zone_type::ZONE_DMA32 as usize;
            // #else
            // #define OPT_ZONE_DMA32 ZONE_NORMAL
            // #endif
            const OPT_ZONE_MOVABLE: usize = zone_type::ZONE_MOVABLE as usize;

            (OPT_ZONE_NORMAL << 0 * ZONES_SHIFT)
                | (OPT_ZONE_DMA << GFP::DMA.bits() as usize * ZONES_SHIFT)
                | (OPT_ZONE_HIGHMEM << GFP::HIGHMEM.bits() as usize * ZONES_SHIFT)
                | (OPT_ZONE_DMA32 << GFP::DMA32.bits() as usize * ZONES_SHIFT)
                | (OPT_ZONE_NORMAL << GFP::MOVABLE.bits() as usize * ZONES_SHIFT)
                | (OPT_ZONE_DMA << (GFP::MOVABLE.bits() | GFP::DMA.bits()) as usize * ZONES_SHIFT)
                | (OPT_ZONE_MOVABLE
                    << (GFP::MOVABLE.bits() | GFP::HIGHMEM.bits()) as usize * ZONES_SHIFT)
                | (OPT_ZONE_DMA32
                    << (GFP::MOVABLE.bits() | GFP::DMA32.bits()) as usize * ZONES_SHIFT)
        };

        /*
         * GFP_ZONE_BAD is a bitmap for all combinations of __GFP_DMA, __GFP_DMA32
         * __GFP_HIGHMEM and __GFP_MOVABLE that are not permitted. One flag per
         * entry starting with bit 0. Bit is set if the combination is not
         * allowed.
         */
        const ZONE_BAD: u32 = {
            1 << (GFP::DMA.bits() | GFP::HIGHMEM.bits())
                | 1 << (GFP::DMA.bits() | GFP::DMA32.bits())
                | 1 << (GFP::DMA32.bits() | GFP::HIGHMEM.bits())
                | 1 << (GFP::DMA.bits() | GFP::DMA32.bits() | GFP::HIGHMEM.bits())
                | 1 << (GFP::MOVABLE.bits() | GFP::HIGHMEM.bits() | GFP::DMA.bits())
                | 1 << (GFP::MOVABLE.bits() | GFP::DMA32.bits() | GFP::DMA.bits())
                | 1 << (GFP::MOVABLE.bits() | GFP::DMA32.bits() | GFP::HIGHMEM.bits())
                | 1 << (GFP::MOVABLE.bits()
                    | GFP::DMA32.bits()
                    | GFP::DMA.bits()
                    | GFP::HIGHMEM.bits())
        };

        let bit: usize = self.intersection(GFP::ZONEMASK).bits() as usize;

        let z = (ZONE_TABLE >> (bit * ZONES_SHIFT)) & ((1 << ZONES_SHIFT) - 1);
        if ((ZONE_BAD >> bit) & 1) != 0 {
            None
        } else {
            z.try_into().ok()
        }
    }

    /*
    * There is only one page-allocator function, and two main namespaces to
    * it. The alloc_page*() variants return 'struct page *' and as such
    * can allocate highmem pages, the *get*page*() variants return
    * virtual kernel addresses to the allocated page(s).
    */
    #[inline]
    pub fn gfp_zonelist(self) -> zonelists {
    // #ifdef CONFIG_NUMA
    //     if (unlikely(flags & __GFP_THISNODE))
    //         return ZONELIST_NOFALLBACK;
    // #endif
        zonelists::ZONELIST_FALLBACK
    }
}

#[cfg(test)]
mod test {
    use crate::mm::mmzone::zone_type;

    use super::GFP;

    #[test]
    fn test_kmalloc_fix_flags() {
        let test_flag = GFP::SLAB_BUG_MASK;
        assert_eq!(test_flag.kmalloc_fix_flags(), GFP::empty());

        let test_flag = GFP::DMA | GFP::DMA32;
        assert_eq!(test_flag.kmalloc_fix_flags(), GFP::DMA);

        let test_flag = GFP::ZONEMASK;
        assert_eq!(test_flag.kmalloc_fix_flags(), GFP::DMA | GFP::MOVABLE);
    }

    #[test]
    fn test_gfp_zone() {
        for i in 0..16u32 {
            let flags = GFP::from_bits_truncate(i);
            let expected = match i {
                0 | 8 => Some(zone_type::ZONE_NORMAL),
                1 | 9 => Some(zone_type::ZONE_DMA),
                2 => Some(zone_type::ZONE_NORMAL),
                4 | 12 => Some(zone_type::ZONE_DMA32),
                10 => Some(zone_type::ZONE_MOVABLE),
                _ => None
            };
            assert_eq!(expected, flags.gfp_zone());
        }
    }
}

/*
 * gfp_allowed_mask is set to GFP_BOOT_MASK during early boot to restrict what
 * GFP flags are used before interrupts are enabled. Once interrupts are
 * enabled, it is set to __GFP_BITS_MASK while the system is running. During
 * hibernation, it is used by PM to avoid I/O during memory allocation while
 * devices are suspended.
 */
// TODO __read_mostly
static gfp_allowed_mask: GFP = GFP::BOOT_MASK;
