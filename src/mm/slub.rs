/*
 * SLUB: A slab allocator that limits cache line use instead of queuing
 * objects in per cpu and per node lists.
 *
 * The allocator synchronizes using per slab locks or atomic operations
 * and only uses a centralized lock to manage a pool of partial slabs.
 *
 * (C) 2007 SGI, Christoph Lameter
 * (C) 2011 Linux Foundation, Christoph Lameter
 */

/*
 * Lock order:
 *   1. slab_mutex (Global Mutex)
 *   2. node->list_lock (Spinlock)
 *   3. kmem_cache->cpu_slab->lock (Local lock)
 *   4. slab_lock(slab) (Only on some arches)
 *   5. object_map_lock (Only for debugging)
 *
 *   slab_mutex
 *
 *   The role of the slab_mutex is to protect the list of all the slabs
 *   and to synchronize major metadata changes to slab cache structures.
 *   Also synchronizes memory hotplug callbacks.
 *
 *   slab_lock
 *
 *   The slab_lock is a wrapper around the page lock, thus it is a bit
 *   spinlock.
 *
 *   The slab_lock is only used on arches that do not have the ability
 *   to do a cmpxchg_double. It only protects:
 *
 *    A. slab->freelist    -> List of free objects in a slab
 *    B. slab->inuse        -> Number of objects in use
 *    C. slab->objects    -> Number of objects in slab
 *    D. slab->frozen        -> frozen state
 *
 *   Frozen slabs
 *
 *   If a slab is frozen then it is exempt from list management. It is
 *   the cpu slab which is actively allocated from by the processor that
 *   froze it and it is not on any list. The processor that froze the
 *   slab is the one who can perform list operations on the slab. Other
 *   processors may put objects onto the freelist but the processor that
 *   froze the slab is the only one that can retrieve the objects from the
 *   slab's freelist.
 *
 *   CPU partial slabs
 *
 *   The partially empty slabs cached on the CPU partial list are used
 *   for performance reasons, which speeds up the allocation process.
 *   These slabs are not frozen, but are also exempt from list management,
 *   by clearing the PG_workingset flag when moving out of the node
 *   partial list. Please see __slab_free() for more details.
 *
 *   To sum up, the current scheme is:
 *   - node partial slab: PG_Workingset && !frozen
 *   - cpu partial slab: !PG_Workingset && !frozen
 *   - cpu slab: !PG_Workingset && frozen
 *   - full slab: !PG_Workingset && !frozen
 *
 *   list_lock
 *
 *   The list_lock protects the partial and full list on each node and
 *   the partial slab counter. If taken then no new slabs may be added or
 *   removed from the lists nor make the number of partial slabs be modified.
 *   (Note that the total number of slabs is an atomic value that may be
 *   modified without taking the list lock).
 *
 *   The list_lock is a centralized lock and thus we avoid taking it as
 *   much as possible. As long as SLUB does not have to handle partial
 *   slabs, operations can continue without any centralized lock. F.e.
 *   allocating a long series of objects that fill up slabs does not require
 *   the list lock.
 *
 *   For debug caches, all allocations are forced to go through a list_lock
 *   protected region to serialize against concurrent validation.
 *
 *   cpu_slab->lock local lock
 *
 *   This locks protect slowpath manipulation of all kmem_cache_cpu fields
 *   except the stat counters. This is a percpu structure manipulated only by
 *   the local cpu, so the lock protects against being preempted or interrupted
 *   by an irq. Fast path operations rely on lockless operations instead.
 *
 *   On PREEMPT_RT, the local lock neither disables interrupts nor preemption
 *   which means the lockless fastpath cannot be used as it might interfere with
 *   an in-progress slow path operations. In this case the local lock is always
 *   taken but it still utilizes the freelist for the common operations.
 *
 *   lockless fastpaths
 *
 *   The fast path allocation (slab_alloc_node()) and freeing (do_slab_free())
 *   are fully lockless when satisfied from the percpu slab (and when
 *   cmpxchg_double is possible to use, otherwise slab_lock is taken).
 *   They also don't disable preemption or migration or irqs. They rely on
 *   the transaction id (tid) field to detect being preempted or moved to
 *   another cpu.
 *
 *   irq, preemption, migration considerations
 *
 *   Interrupts are disabled as part of list_lock or local_lock operations, or
 *   around the slab_lock operation, in order to make the slab allocator safe
 *   to use in the context of an irq.
 *
 *   In addition, preemption (or migration on PREEMPT_RT) is disabled in the
 *   allocation slowpath, bulk allocation, and put_cpu_partial(), so that the
 *   local cpu doesn't change in the process and e.g. the kmem_cache_cpu pointer
 *   doesn't have to be revalidated in each section protected by the local lock.
 *
 * SLUB assigns one slab for allocation to each processor.
 * Allocations only occur from these slabs called cpu slabs.
 *
 * Slabs with free elements are kept on a partial list and during regular
 * operations no list for full slabs is used. If an object in a full slab is
 * freed then the slab will show up again on the partial lists.
 * We track full slabs for debugging purposes though because otherwise we
 * cannot scan all objects.
 *
 * Slabs are freed when they become empty. Teardown and setup is
 * minimal so we rely on the page allocators per cpu caches for
 * fast frees and allocs.
 *
 * slab->frozen        The slab is frozen and exempt from list processing.
 *             This means that the slab is dedicated to a purpose
 *             such as satisfying allocations for a specific
 *             processor. Objects may be freed in the slab while
 *             it is frozen but slab_free will then skip the usual
 *             list operations. It is up to the processor holding
 *             the slab to integrate the slab into the slab lists
 *             when the slab is no longer needed.
 *
 *             One use of this flag is to mark slabs that are
 *             used for allocations. Then such a slab becomes a cpu
 *             slab. The cpu slab may be equipped with an additional
 *             freelist that allows lockless access to
 *             free objects in addition to the regular freelist
 *             that requires the slab lock.
 *
 * SLAB_DEBUG_FLAGS    Slab requires special handling due to debug
 *             options set. This moves    slab handling out of
 *             the fast path and disables lockless freelists.
 */
