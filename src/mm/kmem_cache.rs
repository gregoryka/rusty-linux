use super::gfp::GFP;

/// Word size structure that can be atomically updated or read and that
/// contains both the order and the number of objects that a slab of the
/// given order would contain.
#[derive(Clone, Copy)]
pub(super) struct kmem_cache_order_objects {
    x: u32,
}

impl kmem_cache_order_objects {
    const SHIFT: u32 = 16;

    #[inline]
    pub fn order(&self) -> u32 {
        return self.x >> Self::SHIFT;
    }
}

/// Slab cache management.
pub(super) struct kmem_cache {
    // #ifndef CONFIG_SLUB_TINY
    //     struct kmem_cache_cpu __percpu *cpu_slab;
    // #endif
    //     /* Used for retrieving partial slabs, etc. */
    //     slab_flags_t flags;
    //     unsigned long min_partial;
    //     unsigned int size;		/* Object size including metadata */
    //     unsigned int object_size;	/* Object size without metadata */
    //     struct reciprocal_value reciprocal_size;
    //     unsigned int offset;		/* Free pointer offset */
    // #ifdef CONFIG_SLUB_CPU_PARTIAL
    //     /* Number of per cpu partial objects to keep around */
    //     unsigned int cpu_partial;
    //     /* Number of per cpu partial slabs to keep around */
    //     unsigned int cpu_partial_slabs;
    // #endif
    pub oo: kmem_cache_order_objects,

    /// Allocation and freeing of slabs
    pub min: kmem_cache_order_objects,
    /// gfp flags to use on each alloc
    allocflags: GFP,
    //     int refcount;			/* Refcount for slab cache destroy */
    //     void (*ctor)(void *object);	/* Object constructor */
    //     unsigned int inuse;		/* Offset to metadata */
    //     unsigned int align;		/* Alignment */
    //     unsigned int red_left_pad;	/* Left redzone padding size */
    //     const char *name;		/* Name (only for display!) */
    //     struct list_head list;		/* List of slab caches */
    // #ifdef CONFIG_SYSFS
    //     struct kobject kobj;		/* For sysfs */
    // #endif
    // #ifdef CONFIG_SLAB_FREELIST_HARDENED
    //     unsigned long random;
    // #endif

    // #ifdef CONFIG_NUMA
    //     /*
    //     * Defragmentation by allocating from a remote node.
    //     */
    //     unsigned int remote_node_defrag_ratio;
    // #endif

    // #ifdef CONFIG_SLAB_FREELIST_RANDOM
    //     unsigned int *random_seq;
    // #endif

    // #ifdef CONFIG_KASAN_GENERIC
    //     struct kasan_cache kasan_info;
    // #endif

    // #ifdef CONFIG_HARDENED_USERCOPY
    //     unsigned int useroffset;	/* Usercopy region offset */
    //     unsigned int usersize;		/* Usercopy region size */
    // #endif

    //     struct kmem_cache_node *node[MAX_NUMNODES];
}

impl kmem_cache {
    pub fn add_alloc_flags(&self, flags: GFP) -> GFP {
        flags | self.allocflags
    }
}
