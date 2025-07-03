// #ifdef CONFIG_CPUSETS
// static inline bool cpusets_enabled(void)
// {
// 	return static_branch_unlikely(&cpusets_enabled_key);
// }
// #else /* !CONFIG_CPUSETS */
#[inline]
pub fn cpusets_enabled() -> bool {
    false
}
// #endif /* !CONFIG_CPUSETS */
