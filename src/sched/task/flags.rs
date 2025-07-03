use bitflags::bitflags;

/*
* Per process flags
*/
bitflags! {
    #[derive(Clone, Copy)]
    pub struct process_flags: u32 {
        /// I'm a virtual CPU
        const VCPU = 0x00000001;
        /// I am an IDLE thread
        const IDLE = 0x00000002;
        /// Getting shut down
        const EXITING = 0x00000004;
        /// Coredumps should ignore this task
        const POSTCOREDUMP = 0x00000008;
        /// Task is an IO worker
        const IO_WORKER = 0x00000010;
        /// I'm a workqueue worker
        const WQ_WORKER = 0x00000020;
        /// Forked but didn't exec
        const FORKNOEXEC = 0x00000040;
        /// Process policy on mce errors
        const MCE_PROCESS = 0x00000080;
        /// Used super-user privileges
        const SUPERPRIV = 0x00000100;
        /// Dumped core
        const DUMPCORE = 0x00000200;
        /// Killed by a signal
        const SIGNALED = 0x00000400;
        /// Allocating memory to free memory. See memalloc_noreclaim_save()
        const MEMALLOC = 0x00000800;
        /// set_user() noticed that RLIMIT_NPROC was exceeded
        const NPROC_EXCEEDED = 0x00001000;
        /// If unset the fpu must be initialized before use
        const USED_MATH = 0x00002000;
        /// Kernel thread cloned from userspace thread */
        const USER_WORKER = 0x00004000;
        /// This thread should not be frozen
        const NOFREEZE = 0x00008000;
        /// I am kcompactd
        const KCOMPACTD = 0x00010000;
        /// I am kswapd
        const KSWAPD = 0x00020000;
        /// All allocations inherit GFP_NOFS. See memalloc_nfs_save()
        const MEMALLOC_NOFS = 0x00040000;
        /// All allocations inherit GFP_NOIO. See memalloc_noio_save()
        const MEMALLOC_NOIO = 0x00080000;
        /// Throttle writes only against the bdi I write to,
        /// I am cleaning dirty pages from some other bdi. */
        const LOCAL_THROTTLE = 0x00100000;
        /// I am a kernel thread
        const KTHREAD = 0x00200000;
        /// Randomize virtual address space
        const RANDOMIZE = 0x00400000;
        /// Userland is not allowed to meddle with cpus_mask
        const NO_SETAFFINITY = 0x04000000;
        /// Early kill for mce process policy
        const MCE_EARLY = 0x08000000  ;
        /// Allocations constrained to zones which allow long term pinning.
        /// See memalloc_pin_save()
        const MEMALLOC_PIN = 0x10000000;
        /// plug has ts that needs updating
        const BLOCK_TS = 0x20000000;
        /// This thread called freeze_processes() and should not be frozen
        const SUSPEND_TASK = 0x80000000;
    }
}
