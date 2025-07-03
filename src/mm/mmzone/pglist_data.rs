use core::mem;

use crate::mm::{gfp::GFP, mmzone::zonelist::zonelist};

pub enum zonelists {
    ZONELIST_FALLBACK,    /* zonelist with fallback */
    // #ifdef CONFIG_NUMA
    /*
    * The NUMA zonelists are doubled because we need zonelists that
    * restrict the allocations to a single node for __GFP_THISNODE.
    */
    // ZONELIST_NOFALLBACK,    /* zonelist without fallback (__GFP_THISNODE) */
    // #endif
    // MAX_ZONELISTS
}

impl zonelists{
    const MAX_ZONELISTS: usize = mem::variant_count::<Self>();
}

/*
 * On NUMA machines, each NUMA node would have a pg_data_t to describe
 * it's memory layout. On UMA machines there is a single pglist_data which
 * describes the whole memory.
 *
 * Memory statistics and page replacement data structures are maintained on a
 * per-zone basis.
 */
pub struct pglist_data {
//     /*
//     * node_zones contains just the zones for THIS node. Not all of the
//     * zones may be populated, but it is the full list. It is referenced by
//     * this node's node_zonelists as well as other node's node_zonelists.
//     */
//     struct zone node_zones[MAX_NR_ZONES];

    /*
    * node_zonelists contains references to all zones in all nodes.
    * Generally the first zones will be references to this node's
    * node_zones.
    */
    node_zonelists: [zonelist; zonelists::MAX_ZONELISTS],

//     int nr_zones; /* number of populated zones in this node */
// #ifdef CONFIG_FLATMEM    /* means !SPARSEMEM */
//     struct page *node_mem_map;
// #ifdef CONFIG_PAGE_EXTENSION
//     struct page_ext *node_page_ext;
// #endif
// #endif
// #if defined(CONFIG_MEMORY_HOTPLUG) || defined(CONFIG_DEFERRED_STRUCT_PAGE_INIT)
//     /*
//     * Must be held any time you expect node_start_pfn,
//     * node_present_pages, node_spanned_pages or nr_zones to stay constant.
//     * Also synchronizes pgdat->first_deferred_pfn during deferred page
//     * init.
//     *
//     * pgdat_resize_lock() and pgdat_resize_unlock() are provided to
//     * manipulate node_size_lock without checking for CONFIG_MEMORY_HOTPLUG
//     * or CONFIG_DEFERRED_STRUCT_PAGE_INIT.
//     *
//     * Nests above zone->lock and zone->span_seqlock
//     */
//     spinlock_t node_size_lock;
// #endif
//     unsigned long node_start_pfn;
//     unsigned long node_present_pages; /* total number of physical pages */
//     unsigned long node_spanned_pages; /* total size of physical page
//                         range, including holes */
//     int node_id;
//     wait_queue_head_t kswapd_wait;
//     wait_queue_head_t pfmemalloc_wait;

//     /* workqueues for throttling reclaim for different reasons. */
//     wait_queue_head_t reclaim_wait[NR_VMSCAN_THROTTLE];

//     atomic_t nr_writeback_throttled;/* nr of writeback-throttled tasks */
//     unsigned long nr_reclaim_start;    /* nr pages written while throttled
//                     * when throttling started. */
// #ifdef CONFIG_MEMORY_HOTPLUG
//     struct mutex kswapd_lock;
// #endif
//     struct task_struct *kswapd;    /* Protected by kswapd_lock */
//     int kswapd_order;
//     enum zone_type kswapd_highest_zoneidx;

//     int kswapd_failures;        /* Number of 'reclaimed == 0' runs */

// #ifdef CONFIG_COMPACTION
//     int kcompactd_max_order;
//     enum zone_type kcompactd_highest_zoneidx;
//     wait_queue_head_t kcompactd_wait;
//     struct task_struct *kcompactd;
//     bool proactive_compact_trigger;
// #endif
//     /*
//     * This is a per-node reserve of pages that are not available
//     * to userspace allocations.
//     */
//     unsigned long        totalreserve_pages;

// #ifdef CONFIG_NUMA
//     /*
//     * node reclaim becomes active if more unmapped pages exist.
//     */
//     unsigned long        min_unmapped_pages;
//     unsigned long        min_slab_pages;
// #endif /* CONFIG_NUMA */

//     /* Write-intensive fields used by page reclaim */
//     CACHELINE_PADDING(_pad1_);

// #ifdef CONFIG_DEFERRED_STRUCT_PAGE_INIT
//     /*
//     * If memory initialisation on large machines is deferred then this
//     * is the first PFN that needs to be initialised.
//     */
//     unsigned long first_deferred_pfn;
// #endif /* CONFIG_DEFERRED_STRUCT_PAGE_INIT */

// #ifdef CONFIG_TRANSPARENT_HUGEPAGE
//     struct deferred_split deferred_split_queue;
// #endif

// #ifdef CONFIG_NUMA_BALANCING
//     /* start time in ms of current promote rate limit period */
//     unsigned int nbp_rl_start;
//     /* number of promote candidate pages at start time of current rate limit period */
//     unsigned long nbp_rl_nr_cand;
//     /* promote threshold in ms */
//     unsigned int nbp_threshold;
//     /* start time in ms of current promote threshold adjustment period */
//     unsigned int nbp_th_start;
//     /*
//     * number of promote candidate pages at start time of current promote
//     * threshold adjustment period
//     */
//     unsigned long nbp_th_nr_cand;
// #endif
//     /* Fields commonly accessed by the page reclaim scanner */

//     /*
//     * NOTE: THIS IS UNUSED IF MEMCG IS ENABLED.
//     *
//     * Use mem_cgroup_lruvec() to look up lruvecs.
//     */
//     struct lruvec        __lruvec;

//     unsigned long        flags;

// #ifdef CONFIG_LRU_GEN
//     /* kswap mm walk data */
//     struct lru_gen_mm_walk mm_walk;
//     /* lru_gen_folio list */
//     struct lru_gen_memcg memcg_lru;
// #endif

//     CACHELINE_PADDING(_pad2_);

//     /* Per-node vmstats */
//     struct per_cpu_nodestat __percpu *per_cpu_nodestats;
//     atomic_long_t        vm_stat[NR_VM_NODE_STAT_ITEMS];
// #ifdef CONFIG_NUMA
//     struct memory_tier __rcu *memtier;
// #endif
// #ifdef CONFIG_MEMORY_FAILURE
//     struct memory_failure_stats mf_stats;
// #endif
}

impl pglist_data {
    // #ifndef CONFIG_NUMA

    #[inline]
    pub fn NODE_DATA(_nid: usize) -> &'static pglist_data {
        todo!()
        // &contig_page_data
    }

    // #else /* CONFIG_NUMA */

    // #include <asm/mmzone.h>

    // #endif /* !CONFIG_NUMA */`

    /*
    * We get the zone list from the current node and the gfp_mask.
    * This zone list contains a maximum of MAX_NUMNODES*MAX_NR_ZONES zones.
    * There are two zonelists per node, one for all zones with memory and
    * one containing just zones from the node the zonelist belongs to.
    *
    * For the case of non-NUMA systems the NODE_DATA() gets optimized to
    * &contig_page_data at compile-time.
    */
    #[inline]
    pub fn node_zonelist(nid: usize, flags: GFP) -> &'static zonelist { // todo ref
        &Self::NODE_DATA(nid).node_zonelists[flags.gfp_zonelist() as usize]
    }
}

// static contig_page_data: pglist_data = pglist_data{};
