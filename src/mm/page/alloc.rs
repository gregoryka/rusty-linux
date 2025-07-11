mod flags;

use crate::{
    arch::topology::Nodemask,
    cgroups::cpuset,
    mm::{
        gfp::GFP,
        mmzone::{
            migratetype::migratetype, pglist_data::pglist_data, zone_type, zonelist::zonelist,
        },
        page::alloc::flags::AllocFlags,
    },
};

// #ifdef CONFIG_FAIL_PAGE_ALLOC
// bool should_fail_alloc_page(gfp_t gfp_mask, unsigned int order);
// #else
#[inline]
fn should_fail_alloc_page(_gfp_mask: GFP, _order: u32) -> bool {
    false
}
// #endif /* CONFIG_FAIL_PAGE_ALLOC */

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
    nodemask: Option<Nodemask>,
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
    spread_dirty_pages: bool,
}

impl alloc_context {
    #[inline]
    fn prepare_alloc_pages(
        gfp_mask: GFP,
        order: u32,
        preferred_nid: usize,
        nodemask: Option<Nodemask>,
        alloc_gfp: &mut GFP,
        alloc_flags: &mut AllocFlags,
    ) -> Option<Self> {
        let highest_zoneidx = gfp_mask.gfp_zone().unwrap();
        let zonelist = pglist_data::node_zonelist(preferred_nid, gfp_mask);
        let nodemask = nodemask;
        let migratetype = migratetype::from(gfp_mask);

        if cpuset::cpusets_enabled() {
            // TODO CONFIG_CPUSETS
            // *alloc_gfp |= GFP::HARDWALL;
            // /*
            // * When we are in the interrupt context, it is irrelevant
            // * to the current task context. It means that any node ok.
            // */
            // if in_task() && nodemask.is_none() {
            //     ac->Nodemask = &cpuset_current_mems_allowed;
            // }
            // // else {
            // //     *alloc_flags |= ALLOC_CPUSET;
            // // }
        }

        // might_alloc(gfp_mask);

        /*
         * Don't invoke should_fail logic, since it may call
         * get_random_u32() and printk() which need to spin_lock.
         */
        if !(alloc_flags.contains(AllocFlags::TRYLOCK)) && should_fail_alloc_page(gfp_mask, order) {
            return None;
        }

        alloc_flags.gfp_to_alloc_flags_cma(gfp_mask);

        /* Dirty zone balancing only done in the fast path */
        let spread_dirty_pages = gfp_mask.intersects(GFP::WRITE);

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
            spread_dirty_pages: spread_dirty_pages,
        })
    }
}
