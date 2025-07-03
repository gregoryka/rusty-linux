use crate::{arch::topology::MAX_NUMNODES, mm::{gfp::GFP, mmzone::pglist_data::pglist_data}};

use super::{zone::zone, zone_type};

/*
 * This struct contains information about a zone in a zonelist. It is stored
 * here to avoid dereferences into large structures and lookups of tables
 */
struct zoneref {
    /// TODO Pointer to actual zone
    zone: zone,
    /// zone_idx(zoneref->zone)
    zone_idx: u32,
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
pub struct zonelist {
    /* Maximum number of zones on a zonelist */
    _zonerefs: [zoneref; MAX_ZONES_PER_ZONELIST + 1],
}

impl zonelist {

}
