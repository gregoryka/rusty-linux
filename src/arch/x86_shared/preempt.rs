// TODO DECLARE_PER_CPU_CACHE_HOT(int, __preempt_count);
static __preempt_count: u32 = 0;
/* We use the MSB mostly because its available */
const PREEMPT_NEED_RESCHED: u32 = 0x80000000u32;

/*
 * We mask the PREEMPT_NEED_RESCHED bit so as not to confuse all current users
 * that think a non-zero value indicates we cannot preempt.
 */
#[inline(always)]
pub fn preempt_count() -> u32 {
    // TODO raw_cpu_read_4(__preempt_count) & !PREEMPT_NEED_RESCHED
    __preempt_count & !PREEMPT_NEED_RESCHED
}
