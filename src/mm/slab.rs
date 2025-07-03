use core::hint::cold_path;

use super::{gfp::GFP, kmem_cache::kmem_cache};

struct slab {

}

impl slab {
    fn new_slab(s: kmem_cache, flags: GFP, node: i32) -> Self {
        let flags = if flags.intersects(GFP::SLAB_BUG_MASK) {
            cold_path();
            flags.kmalloc_fix_flags()
        } else {flags};

        // WARN_ON_ONCE(s->ctor && (flags & __GFP_ZERO));

        Self::allocate_slab(s, flags & (GFP::RECLAIM_MASK | GFP::CONSTRAINT_MASK), node)
    }

    fn allocate_slab(s: kmem_cache, flags: GFP, node: i32) -> Self {
    // struct slab *slab;
    let oo = s.oo;
    // gfp_t alloc_gfp;
    // void *start, *p, *next;
    // int idx;
    // bool shuffle;

    let flags = flags.filter_allowed_mask();

    let flags = s.add_alloc_flags(flags);

    /*
     * Let the initial higher-order allocation fail under memory pressure
     * so we fall-back to the minimum order allocation.
     */
    let alloc_gfp = (flags | GFP::NOWARN | GFP::NORETRY) & !GFP::NOFAIL;
    let alloc_gfp = if (alloc_gfp.contains(GFP::DIRECT_RECLAIM)) && oo.order() > s.min.order() {
        (alloc_gfp | GFP::NOMEMALLOC) & !GFP::RECLAIM
    } else {alloc_gfp};

    // slab = alloc_slab_page(alloc_gfp, node, oo);
    // if (unlikely(!slab)) {
    //     oo = s->min;
    //     alloc_gfp = flags;
    //     /*
    //      * Allocation may have failed due to fragmentation.
    //      * Try a lower order alloc if possible
    //      */
    //     slab = alloc_slab_page(alloc_gfp, node, oo);
    //     if (unlikely(!slab))
    //         return NULL;
    //     stat(s, ORDER_FALLBACK);
    // }

    // slab->objects = oo_objects(oo);
    // slab->inuse = 0;
    // slab->frozen = 0;
    // init_slab_obj_exts(slab);

    // account_slab(slab, oo_order(oo), s, flags);

    // slab->slab_cache = s;

    // kasan_poison_slab(slab);

    // start = slab_address(slab);

    // setup_slab_debug(s, slab, start);

    // shuffle = shuffle_freelist(s, slab);

    // if (!shuffle) {
    //     start = fixup_red_left(s, start);
    //     start = setup_object(s, start);
    //     slab->freelist = start;
    //     for (idx = 0, p = start; idx < slab->objects - 1; idx++) {
    //         next = p + s->size;
    //         next = setup_object(s, next);
    //         set_freepointer(s, p, next);
    //         p = next;
    //     }
    //     set_freepointer(s, p, NULL);
    // }

    // return slab;
    todo!()
}
}
