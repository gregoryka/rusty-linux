use core::hint::cold_path;

use crate::{
    arch::topology::{MAX_NUMNODES, Nodemask},
    mm::{gfp::GFP, mmzone::pglist_data::pglist_data},
};

use super::{zone::zone, zone_type};

/*
 * This struct contains information about a zone in a zonelist. It is stored
 * here to avoid dereferences into large structures and lookups of tables
 */
struct zoneref<'a> {
    /// TODO Pointer to actual zone
    zone: &'a zone,
    /// zone_idx(zoneref->zone)
    zone_idx: zone_type,
}

impl zoneref<'_> {
    /**
     * next_zones_zonelist - Returns the next zone at or below highest_zoneidx within the allowed nodemask using a cursor within a zonelist as a starting point
     * @z: The cursor used as a starting point for the search
     * @highest_zoneidx: The zone index of the highest zone to return
     * @nodes: An optional nodemask to filter the zonelist with
     *
     * This function returns the next zone at or below a given zone index that is
     * within the allowed nodemask using a cursor as the starting point for the
     * search. The zoneref returned is a cursor that represents the current zone
     * being examined. It should be advanced by one before calling
     * next_zones_zonelist again.
     *
     * Return: the next zone at or below highest_zoneidx within the allowed
     * nodemask using a cursor within a zonelist as a starting point
     */
    #[inline(always)]
    fn next_zones_zonelist(
        z: &[Self],
        highest_zoneidx: zone_type,
        nodes: Option<Nodemask>,
    ) -> Option<&zoneref> {
        if nodes.is_none() && z.first().unwrap().zonelist_zone_idx() <= highest_zoneidx {
            z.first()
        } else {
            cold_path();
            Self::__next_zones_zonelist(z, highest_zoneidx, nodes)
        }
    }

    /// Returns the next zone at or below highest_zoneidx in a zonelist
    fn __next_zones_zonelist(
        // &self,
        z: &[Self],
        highest_zoneidx: zone_type,
        nodes: Option<Nodemask>,
    ) -> Option<&zoneref> {
        /*
         * Find the next suitable zone to use for the allocation.
         * Only filter based on nodemask if it's set
         */
        if nodes.is_none() {
            cold_path();
            z.iter().find(|z| z.zonelist_zone_idx() < highest_zoneidx)
        } else {
            // z.iter().find(|z| z.zonelist_zone_idx() < highest_zoneidx || )
            //     while (zonelist_zone_idx(z) > highest_zoneidx ||
            //             (zonelist_zone(z) && !zref_in_nodemask(z, nodes)))
            //         z++;
            todo!()
        }
    }

    #[inline]
    fn zonelist_zone_idx(&self) -> zone_type {
        self.zone_idx
    }
}

const MAX_ZONES_PER_ZONELIST: usize = MAX_NUMNODES * zone_type::MAX_NR_ZONES;

/*
 * One allocation request operates on a zonelist. A zonelist
 * is a list of zones, the first one is the 'goal' of the
 * allocation, the other zones are fallback zones, in decreasing
 * priority.
 *
 * To speed the reading of the zonelist, the zonerefs contain the zone index
 * of the entry being read. Helper functions to access information given
 * a struct zoneref are
 *
 * zonelist_zone()    - Return the struct zone * for an entry in _zonerefs
 * zonelist_zone_idx()    - Return the index of the zone for an entry
 * zonelist_node_idx()    - Return the index of the node for an entry
 */
pub struct zonelist<'a> {
    /* Maximum number of zones on a zonelist */
    _zonerefs: [zoneref<'a>; MAX_ZONES_PER_ZONELIST + 1],
}

impl zonelist<'_> {
    /**
     * first_zones_zonelist - Returns the first zone at or below highest_zoneidx within the allowed nodemask in a zonelist
     * @zonelist: The zonelist to search for a suitable zone
     * @highest_zoneidx: The zone index of the highest zone to return
     * @nodes: An optional nodemask to filter the zonelist with
     *
     * This function returns the first zone at or below a given zone index that is
     * within the allowed nodemask. The zoneref returned is a cursor that can be
     * used to iterate the zonelist with next_zones_zonelist by advancing it by
     * one before calling.
     *
     * When no eligible zone is found, zoneref->zone is NULL (zoneref itself is
     * never NULL). This may happen either genuinely, or due to concurrent nodemask
     * update due to cpuset modification.
     *
     * Return: Zoneref pointer for the first suitable zone found
     */
    #[inline]
    pub fn first_zones_zonelist(
        &self,
        highest_zoneidx: zone_type,
        nodes: Option<Nodemask>,
    ) -> Option<&zoneref> {
        //     return next_zones_zonelist(zonelist->_zonerefs,
        //                             highest_zoneidx, nodes);
    }
}
