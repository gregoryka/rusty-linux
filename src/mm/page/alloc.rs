use crate::{
    arch::topology::nodemask,
    cgroups::cpuset,
    mm::{
        gfp::GFP,
        mmzone::{migratetype::migratetype, pglist_data::pglist_data, zone_type, zonelist::zonelist},
    },
};

/*
 * Structure for holding the mostly immutable allocation parameters passed
 * between functions involved in allocations, including the alloc_pages*
 * family of functions.
 *
 * nodemask, migratetype and highest_zoneidx are initialized only once in
 * __alloc_pages() and then never change.
 *
 * zonelist, preferred_zone and highest_zoneidx are set first in
 * __alloc_pages() for the fast path, and might be later changed
 * in __alloc_pages_slowpath(). All other functions pass the whole structure
 * by a const pointer.
 */
pub(super) struct alloc_context {
    zonelist: &'static zonelist,
    nodemask: Option<nodemask>,
    // struct zoneref *preferred_zoneref;
    migratetype: migratetype,

    /*
     * highest_zoneidx represents highest usable zone index of
     * the allocation request. Due to the nature of the zone,
     * memory on lower zone than the highest_zoneidx will be
     * protected by lowmem_reserve[highest_zoneidx].
     *
     * highest_zoneidx is also used by reclaim/compaction to limit
     * the target zone since higher zone than this index cannot be
     * usable for this allocation request.
     */
    highest_zoneidx: zone_type,
    // bool spread_dirty_pages;
}

impl alloc_context {
    #[inline]
    fn prepare_alloc_pages(
        gfp_mask: GFP,
        order: u32,
        preferred_nid: usize,
        nodemask: Option<nodemask>,
        alloc_gfp: &mut GFP,
        alloc_flags: &mut u32,
    ) -> Option<Self> {
        let highest_zoneidx = gfp_mask.gfp_zone().unwrap();
        let zonelist = pglist_data::node_zonelist(preferred_nid, gfp_mask);
        let nodemask = nodemask;
        let migratetype = migratetype::from(gfp_mask);

        if cpuset::cpusets_enabled() {
            *alloc_gfp |= GFP::HARDWALL;
            /*
            * When we are in the interrupt context, it is irrelevant
            * to the current task context. It means that any node ok.
            */
            // if in_task() && nodemask.is_none() {
            //     ac->nodemask = &cpuset_current_mems_allowed;
            // }
            // else {
            //     *alloc_flags |= ALLOC_CPUSET;
            // }
        }

        // might_alloc(gfp_mask);

        /*
         * Don't invoke should_fail logic, since it may call
         * get_random_u32() and printk() which need to spin_lock.
         */
        // if (!(*alloc_flags & ALLOC_TRYLOCK) &&
        //     should_fail_alloc_page(gfp_mask, order))
        //     return false;

        // *alloc_flags = gfp_to_alloc_flags_cma(gfp_mask, *alloc_flags);

        /* Dirty zone balancing only done in the fast path */
        // ac->spread_dirty_pages = (gfp_mask & __GFP_WRITE);

        /*
         * The preferred zone is used for statistics but crucially it is
         * also used as the starting point for the zonelist iterator. It
         * may get reset for allocations that ignore memory policies.
         */
        // ac->preferred_zoneref = first_zones_zonelist(ac->zonelist,
        //     //                 ac->highest_zoneidx, ac->nodemask);

        Some(Self {
            zonelist: zonelist,
            nodemask: nodemask,
            migratetype: migratetype,
            highest_zoneidx: highest_zoneidx,
        })
    }
}
