#[cfg(target_os = "android")]
mod main {
    extern crate android_cpufeatures_sys as cpu;
    extern crate android_glue;

    use std::os::raw::c_int;

    #[derive(Debug)]
    pub struct Cpu {
        pub count: c_int,
        pub features: u64,
        pub family: cpu::AndroidCpuFamily,
        #[cfg(target_arch="arm")]
        pub arm_cpuid: u32,
    }

    impl Default for Cpu {
        fn default() -> Self {
            unsafe {
                Self {
                    count: cpu::android_getCpuCount(),
                    features: cpu::android_getCpuFeatures(),
                    family: cpu::android_getCpuFamily(),
                    #[cfg(target_arch="arm")]
                    arm_cpuid: cpu::android_getCpuIdArm(),
                }
            }
        }
    }

    pub(super) fn main() {
        android_glue::write_log(&format!("{:?}", Cpu::default()));
        loop {}
    }
}
#[cfg(not(target_os = "android"))]
mod main {
    pub(super) fn main() {
        panic!("This example was written for Android");
    }
}

fn main() {
    main::main();
}
