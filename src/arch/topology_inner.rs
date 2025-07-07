use core::mem;

use bitmaps::Bitmap;

// #ifdef CONFIG_NODES_SHIFT
// #define NODES_SHIFT     CONFIG_NODES_SHIFT
// #else
const NODES_SHIFT: usize = 0;
// #endif

pub(crate) const MAX_NUMNODES: usize = 1 << NODES_SHIFT;

const NUMA_NO_NODE: usize = usize::MAX;

/// Bitmasks that are kept for all the nodes.
enum node_states {
    /// The node could become online at some point
    N_POSSIBLE,
    /// The node is online
    N_ONLINE,
    /// The node has regular memory
    N_NORMAL_MEMORY,
// #ifdef CONFIG_HIGHMEM
    /// The node has regular or high memory
    // N_HIGH_MEMORY,
// #else
    // N_HIGH_MEMORY = N_NORMAL_MEMORY,
// #endif
    /// The node has memory(regular, high, movable)
    N_MEMORY,
    /// The node has one or more cpus
    N_CPU,
    /// The node has one or more Generic Initiators
    N_GENERIC_INITIATOR,
}


pub struct Nodemask(Bitmap<MAX_NUMNODES>);
// {
//     bits: [usize; bitops::bits_to_usize(MAX_NUMNODES)],
// }

impl Nodemask {
    pub fn NODE_MASK_ALL() -> Self {
        Self(Bitmap::<MAX_NUMNODES>::mask(MAX_NUMNODES))
    }

}



// TODO __read_mostly
// /// Array of node states.
// static node_states: [Nodemask; mem::variant_count::<node_states>()] = [
//     [N_POSSIBLE] = NODE_MASK_ALL,
//     //     [N_ONLINE] = { { [0] = 1UL } },
//     // #ifndef CONFIG_NUMA
//     //     [N_NORMAL_MEMORY] = { { [0] = 1UL } },
//     // #ifdef CONFIG_HIGHMEM
//     //     [N_HIGH_MEMORY] = { { [0] = 1UL } },
//     // #endif
//     //     [N_MEMORY] = { { [0] = 1UL } },
//     //     [N_CPU] = { { [0] = 1UL } },
//     // #endif    /* NUMA */
// ];
// nodemask_t []  = {
// };
// EXPORT_SYMBOL(node_states);
