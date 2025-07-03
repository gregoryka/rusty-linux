use crate::utils::bitops;

// #ifdef CONFIG_NODES_SHIFT
// #define NODES_SHIFT     CONFIG_NODES_SHIFT
// #else
const NODES_SHIFT: usize = 0;
// #endif

pub(crate) const MAX_NUMNODES: usize = 1 << NODES_SHIFT;

const NUMA_NO_NODE: usize = usize::MAX;

pub struct nodemask {
    bits: [usize; bitops::bits_to_usize(MAX_NUMNODES)],
}
