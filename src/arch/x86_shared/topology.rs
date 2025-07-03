// cfg not NUMA
pub use super::super::topology_inner::*;

#[inline]
pub fn numa_node_id() -> usize {
    0
}
