use crate::arch::preempt;

// #ifdef CONFIG_PREEMPT_RT
// # define in_task()        (!((preempt_count() & (NMI_MASK | HARDIRQ_MASK)) | in_serving_softirq()))
// #else
pub fn in_task() -> bool {
    // !(preempt::preempt_count() & (NMI_MASK | HARDIRQ_MASK | SOFTIRQ_OFFSET))
    todo!()
}
// #endif
