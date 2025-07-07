use crate::arch::preempt;

/*
 * We put the hardirq and softirq counter into the preemption
 * counter. The bitmask has the following meaning:
 *
 * - bits 0-7 are the preemption count (max preemption depth: 256)
 * - bits 8-15 are the softirq count (max # of softirqs: 256)
 *
 * The hardirq count could in theory be the same as the number of
 * interrupts in the system, but we run all interrupt handlers with
 * interrupts disabled, so we cannot have nesting interrupts. Though
 * there are a few palaeontologic drivers which reenable interrupts in
 * the handler, so we need more than one bit here.
 *
 *         PREEMPT_MASK:	0x000000ff
 *         SOFTIRQ_MASK:	0x0000ff00
 *         HARDIRQ_MASK:	0x000f0000
 *             NMI_MASK:	0x00f00000
 * PREEMPT_NEED_RESCHED:	0x80000000
 */
const PREEMPT_BITS: u32 = 8;
const SOFTIRQ_BITS: u32 = 8;
const HARDIRQ_BITS: u32 = 4;
const NMI_BITS: u32 = 4;


const PREEMPT_SHIFT: u32 = 0;
const SOFTIRQ_SHIFT: u32 = PREEMPT_SHIFT + PREEMPT_BITS;
const HARDIRQ_SHIFT: u32 = SOFTIRQ_SHIFT + SOFTIRQ_BITS;
const NMI_SHIFT: u32 = HARDIRQ_SHIFT + HARDIRQ_BITS;

const fn IRQ_MASK(bits: u32, shift : u32) -> u32 {
    ((1 << bits) - 1) << shift
}

const PREEMPT_MASK: u32 = IRQ_MASK(PREEMPT_BITS, PREEMPT_SHIFT);
const SOFTIRQ_MASK: u32 = IRQ_MASK(SOFTIRQ_BITS, SOFTIRQ_SHIFT);
const HARDIRQ_MASK: u32 = IRQ_MASK(HARDIRQ_BITS, HARDIRQ_SHIFT);
const NMI_MASK: u32 = IRQ_MASK(NMI_BITS, NMI_SHIFT);

const PREEMPT_OFFSET: u32 = 1 << PREEMPT_SHIFT;
const SOFTIRQ_OFFSET: u32 = 1 << SOFTIRQ_SHIFT;
const HARDIRQ_OFFSET: u32 = 1 << HARDIRQ_SHIFT;
const NMI_OFFSET: u32 = 1 << NMI_SHIFT;


// #ifdef CONFIG_PREEMPT_RT
// # define in_task()        (!((preempt_count() & (NMI_MASK | HARDIRQ_MASK)) | in_serving_softirq()))
// #else
pub fn in_task() -> bool {
    (preempt::preempt_count() & (NMI_MASK | HARDIRQ_MASK | SOFTIRQ_OFFSET)) == 0
}
// #endif
