use core::{hint::cold_path, mem};

use static_assertions::const_assert;

use crate::mm::gfp::GFP;

#[derive(Debug,PartialEq)]
pub enum migratetype {
    MIGRATE_UNMOVABLE,
    MIGRATE_MOVABLE,
    MIGRATE_RECLAIMABLE,
    MIGRATE_HIGHATOMIC, /* the number of types on the pcp lists */
    // MIGRATE_HIGHATOMIC = MIGRATE_PCPTYPES,
    // #ifdef CONFIG_CMA
    /*
    * MIGRATE_CMA migration type is designed to mimic the way
    * ZONE_MOVABLE works.  Only movable pages can be allocated
    * from MIGRATE_CMA pageblocks and page allocator never
    * implicitly change migration type of MIGRATE_CMA pageblock.
    *
    * The way to use it is to change migratetype of a range of
    * pageblocks to MIGRATE_CMA which can be done by
    * __free_pageblock_cma() function.
    */
    // MIGRATE_CMA,
    // #endif
    // #ifdef CONFIG_MEMORY_ISOLATION
    // MIGRATE_ISOLATE,	/* can't allocate from here */
    // #endif
    // MIGRATE_TYPES
}

static page_group_by_mobility_disabled: bool = false;

impl From<GFP> for migratetype {
    fn from(value: GFP) -> Self {
        const MOVABLE_SHIFT: u32 = 3;
        const MOVABLE_MASK: u32 = GFP::RECLAIMABLE.bits() | GFP::MOVABLE.bits();
        /* Convert GFP flags to their corresponding migrate type */
        // VM_WARN_ON((gfp_flags & GFP_MOVABLE_MASK) == GFP_MOVABLE_MASK);
        const_assert!((1u32 << MOVABLE_SHIFT) == GFP::MOVABLE.bits());
        const_assert!(
            (GFP::MOVABLE.bits() >> MOVABLE_SHIFT) == migratetype::MIGRATE_MOVABLE as u32
        );
        const_assert!(
            (GFP::RECLAIMABLE.bits() >> MOVABLE_SHIFT) == migratetype::MIGRATE_RECLAIMABLE as u32
        );
        const_assert!(
            ((GFP::MOVABLE.bits() | GFP::RECLAIMABLE.bits()) >> MOVABLE_SHIFT)
                == migratetype::MIGRATE_HIGHATOMIC as u32
        );

        if page_group_by_mobility_disabled {
            cold_path();
            migratetype::MIGRATE_UNMOVABLE
        } else {
            /* Group based on mobility */
            unsafe { mem::transmute(((value.bits() & MOVABLE_MASK) >> MOVABLE_SHIFT) as u8) }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::mm::{gfp::GFP, mmzone::migratetype::migratetype};

    #[test]
    fn test_migrate_from_gfp() {
        let gfp = GFP::empty();
        assert_eq!(migratetype::from(gfp), migratetype::MIGRATE_UNMOVABLE);
        let gfp = GFP::MOVABLE;
        assert_eq!(migratetype::from(gfp), migratetype::MIGRATE_MOVABLE);
        let gfp = GFP::RECLAIMABLE;
        assert_eq!(migratetype::from(gfp), migratetype::MIGRATE_RECLAIMABLE);
        let gfp = GFP::MOVABLE | GFP::RECLAIMABLE;
        assert_eq!(migratetype::from(gfp), migratetype::MIGRATE_HIGHATOMIC);
        let gfp = GFP::all();
        assert_eq!(migratetype::from(gfp), migratetype::MIGRATE_HIGHATOMIC);
    }
}
