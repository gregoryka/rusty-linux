/* Free memory management - zoned buddy allocator.  */
// #ifndef CONFIG_ARCH_FORCE_MAX_ORDER
pub const MAX_PAGE_ORDER: u32 = 10;
// #else
// #define MAX_PAGE_ORDER CONFIG_ARCH_FORCE_MAX_ORDER
// #endif
const MAX_ORDER_NR_PAGES: usize = 1 << MAX_PAGE_ORDER;
