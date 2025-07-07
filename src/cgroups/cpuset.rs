// #ifdef CONFIG_CPUSETS
// static inline bool cpusets_enabled(void)
// {
// 	return static_branch_unlikely(&cpusets_enabled_key);
// }
// #define cpuset_current_mems_allowed (current->mems_allowed)
// #else /* !CONFIG_CPUSETS */
#[inline]
pub fn cpusets_enabled() -> bool {
    false
}

// #define cpuset_current_mems_allowed (node_states[N_MEMORY])
// #endif /* !CONFIG_CPUSETS */
