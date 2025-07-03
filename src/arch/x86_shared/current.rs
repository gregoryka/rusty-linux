use core::ptr;
use crate::sched::task::task;

// TODO DECLARE_PER_CPU_CACHE_HOT(struct task_struct *, current_task);
// static current_task: task = task{flags=GFP::E};

#[inline(always)]
pub unsafe fn get_current() -> &'static task {
    // if (IS_ENABLED(CONFIG_USE_X86_SEG_SUPPORT))
    //     return this_cpu_read_const(const_current_task);
    todo!()
    // &current_task
}
