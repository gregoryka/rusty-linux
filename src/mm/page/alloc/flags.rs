use bitflags::bitflags;

use crate::mm::gfp::GFP;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct AllocFlags: u32 {
        /// don't check watermarks at all
        const NO_WATERMARKS = 0x04;
        /// Only use spin_trylock in allocation path
        const TRYLOCK = 0x400;
        /*
        /* The ALLOC_WMARK bits are used as an index to zone->watermark */
#define ALLOC_WMARK_MIN        WMARK_MIN
#define ALLOC_WMARK_LOW        WMARK_LOW
#define ALLOC_WMARK_HIGH    WMARK_HIGH

/* Mask to get the watermark bits */
#define ALLOC_WMARK_MASK    (ALLOC_NO_WATERMARKS-1)

/*
 * Only MMU archs have async oom victim reclaim - aka oom_reaper so we
 * cannot assume a reduced access to memory reserves is sufficient for
 * !MMU
 */
#ifdef CONFIG_MMU
#define ALLOC_OOM        0x08
#else
#define ALLOC_OOM        ALLOC_NO_WATERMARKS
#endif

#define ALLOC_NON_BLOCK         0x10 /* Caller cannot block. Allow access
                       * to 25% of the min watermark or
                       * 62.5% if __GFP_HIGH is set.
                       */
#define ALLOC_MIN_RESERVE     0x20 /* __GFP_HIGH set. Allow access to 50%
                       * of the min watermark.
                       */
#define ALLOC_CPUSET         0x40 /* check for correct cpuset */
#define ALLOC_CMA         0x80 /* allow allocations from CMA areas */
#ifdef CONFIG_ZONE_DMA32
#define ALLOC_NOFRAGMENT    0x100 /* avoid mixing pageblock types */
#else
#define ALLOC_NOFRAGMENT      0x0
#endif
#define ALLOC_HIGHATOMIC    0x200 /* Allows access to MIGRATE_HIGHATOMIC */
#define ALLOC_KSWAPD        0x800 /* allow waking of kswapd, __GFP_KSWAPD_RECLAIM set */
         */
    }
}

impl AllocFlags {
    /* Must be called after current_gfp_context() which can change gfp_mask */
    #[inline]
    pub fn gfp_to_alloc_flags_cma(&mut self, _gfp_mask: GFP) {
        // #ifdef CONFIG_CMA
        //     if (gfp_migratetype(gfp_mask) == MIGRATE_MOVABLE)
        //         alloc_flags |= ALLOC_CMA;
        // #endif
    }
}
