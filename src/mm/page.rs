mod alloc;

use crate::arch::{
    mm::MAX_PAGE_ORDER,
    topology::{self, Nodemask},
};

use super::gfp::GFP;
use alloc::alloc_context;

struct page {}

impl page {
    // #ifdef CONFIG_NUMA
    // struct page *alloc_frozen_pages_noprof(gfp_t, unsigned int order);
    // #else
    #[inline]
    pub fn alloc_frozen_pages_noprof(gfp: GFP, order: u32) -> Option<Self> {
        Self::__alloc_frozen_pages_noprof(gfp, order, topology::numa_node_id(), None)
    }
    // #endif

    /*
     * This is the 'heart' of the zoned buddy allocator.
     */
    fn __alloc_frozen_pages_noprof(
        gfp: GFP,
        order: u32,
        preferred_nid: usize,
        Nodemask: Option<Nodemask>,
    ) -> Option<Self> {
        //     struct page *page;
        //     unsigned int alloc_flags = ALLOC_WMARK_LOW;
        //     gfp_t alloc_gfp; /* The gfp_t that was actually used for allocation */
        //     struct alloc_context ac = { };

        /*
         * There are several places where we assume that the order value is sane
         * so bail out early if the request is out of bound.
         */
        if order > MAX_PAGE_ORDER {
            // TODO WARN_ON_ONCE_GFP(order > MAX_PAGE_ORDER, gfp)
            return None;
        }

        let gfp = gfp.filter_allowed_mask();
        /*
         * Apply scoped allocation constraints. This is mainly about GFP_NOFS
         * resp. GFP_NOIO which has to be inherited for all allocation requests
         * from a particular context which has been marked by
         * memalloc_no{fs,io}_{save,restore}. And PF_MEMALLOC_PIN which ensures
         * movable zones are not used during allocation.
         */
        let gfp = gfp.current_gfp_context();
        let alloc_gfp = gfp;
        //     if (!prepare_alloc_pages(gfp, order, preferred_nid, nodemask, &ac,
        //             &alloc_gfp, &alloc_flags))
        //         return NULL;

        //     /*
        //      * Forbid the first pass from falling back to types that fragment
        //      * memory until all local zones are considered.
        //      */
        //     alloc_flags |= alloc_flags_nofragment(zonelist_zone(ac.preferred_zoneref), gfp);

        //     /* First allocation attempt */
        //     page = get_page_from_freelist(alloc_gfp, order, alloc_flags, &ac);
        //     if (likely(page))
        //         goto out;

        //     alloc_gfp = gfp;
        //     ac.spread_dirty_pages = false;

        //     /*
        //      * Restore the original nodemask if it was potentially replaced with
        //      * &cpuset_current_mems_allowed to optimize the fast-path attempt.
        //      */
        //     ac.nodemask = nodemask;

        //     page = __alloc_pages_slowpath(alloc_gfp, order, &ac);

        // out:
        //     if (memcg_kmem_online() && (gfp & __GFP_ACCOUNT) && page &&
        //         unlikely(__memcg_kmem_charge_page(page, gfp, order) != 0)) {
        //         free_frozen_pages(page, order);
        //         page = NULL;
        //     }

        //     trace_mm_page_alloc(page, order, alloc_gfp, ac.migratetype);
        //     kmsan_alloc_page(page, order, alloc_gfp);

        //     return page;
        todo!()
    }
}
