fn main() {
    main::main();
}

#[cfg(not(target_os = "android"))]
mod main {
    pub(super) fn main() {
        panic!("This example was written for Android");
    }
}

#[cfg(target_os = "android")]
mod main {
    extern crate android_cpufeatures_sys;
    extern crate android_glue;

    use self::android_cpufeatures_sys::*;
    use std::os::raw::c_int;
    use std::fmt::{self, Display, Formatter};


    pub(super) fn main() {
        android_glue::write_log(&format!("{}", Cpu::default()));
        loop {}
    }

    #[derive(Debug)]
    pub struct Cpu {
        pub count: c_int,
        pub features: u64,
        pub family: AndroidCpuFamily,
        #[cfg(target_arch="arm")]
        pub arm_cpuid: u32,
    }

    impl Default for Cpu {
        fn default() -> Self {
            unsafe {
                Self {
                    count: android_getCpuCount(),
                    features: android_getCpuFeatures(),
                    family: android_getCpuFamily(),
                    #[cfg(target_arch="arm")]
                    arm_cpuid: android_getCpuIdArm(),
                }
            }
        }
    }

    impl Display for Cpu {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            writeln!(f, "Cpu {{")?;
            writeln!(f, "\tcount: {},", self.count)?;
            writeln!(f, "\tfamily: {:?},", self.family)?;
            #[cfg(target_arch="arm")]
            writeln!(f, "\tarm_cpuid: {:x},", self.arm_cpuid)?;
            writeln!(f, "\tfeatures: [")?;
            match self.family {
                AndroidCpuFamily::ANDROID_CPU_FAMILY_UNKNOWN => {},
                AndroidCpuFamily::ANDROID_CPU_FAMILY_MAX => {},
                AndroidCpuFamily::ANDROID_CPU_FAMILY_ARM =>
                    write_arm_features(f, self.features, "\t\t", ",\n")?,
                AndroidCpuFamily::ANDROID_CPU_FAMILY_ARM64 =>
                    write_arm64_features(f, self.features, "\t\t", ",\n")?,
                AndroidCpuFamily::ANDROID_CPU_FAMILY_X86 =>
                    write_x86_features(f, self.features, "\t\t", ",\n")?,
                AndroidCpuFamily::ANDROID_CPU_FAMILY_X86_64 =>
                    write_x86_features(f, self.features, "\t\t", ",\n")?,
                AndroidCpuFamily::ANDROID_CPU_FAMILY_MIPS =>
                    write_mips_features(f, self.features, "\t\t", ",\n")?,
                AndroidCpuFamily::ANDROID_CPU_FAMILY_MIPS64 =>
                    write_mips_features(f, self.features, "\t\t", ",\n")?,
            }
            writeln!(f, "\t],")?;
            writeln!(f, "}}")?;
            Ok(())
        }
    }


    macro_rules! extract_and_write_flags {
        ($formatter:expr, $flags:expr, $prefix:expr, $suffix:expr, $($variant:expr,)+) => {
            $(
                if $flags & ($variant as u64) != 0 { 
                    write!($formatter, "{}{:?}{}", $prefix, $variant, $suffix)?;
                }
            )+
        }
    }

    //TODO: return fmt::Result
    fn write_arm_features(f: &mut Formatter, features: u64, prefix: &str, suffix: &str) -> fmt::Result {
        extract_and_write_flags! {
            f, features, prefix, suffix,
            ArmFeature::ANDROID_CPU_ARM_FEATURE_ARMv7,
            ArmFeature::ANDROID_CPU_ARM_FEATURE_VFPv3,
            ArmFeature::ANDROID_CPU_ARM_FEATURE_NEON,
            ArmFeature::ANDROID_CPU_ARM_FEATURE_LDREX_STREX,
            ArmFeature::ANDROID_CPU_ARM_FEATURE_VFPv2,
            ArmFeature::ANDROID_CPU_ARM_FEATURE_VFP_D32,
            ArmFeature::ANDROID_CPU_ARM_FEATURE_VFP_FP16,
            ArmFeature::ANDROID_CPU_ARM_FEATURE_VFP_FMA,
            ArmFeature::ANDROID_CPU_ARM_FEATURE_NEON_FMA,
            ArmFeature::ANDROID_CPU_ARM_FEATURE_IDIV_ARM,
            ArmFeature::ANDROID_CPU_ARM_FEATURE_IDIV_THUMB2,
            ArmFeature::ANDROID_CPU_ARM_FEATURE_iWMMXt,
            ArmFeature::ANDROID_CPU_ARM_FEATURE_AES,
            ArmFeature::ANDROID_CPU_ARM_FEATURE_PMULL,
            ArmFeature::ANDROID_CPU_ARM_FEATURE_SHA1,
            ArmFeature::ANDROID_CPU_ARM_FEATURE_SHA2,
            ArmFeature::ANDROID_CPU_ARM_FEATURE_CRC32,
        }
        Ok(())
    }
    fn write_arm64_features(f: &mut Formatter, features: u64, prefix: &str, suffix: &str) -> fmt::Result {
        extract_and_write_flags! {
            f, features, prefix, suffix,
            Arm64Feature::ANDROID_CPU_ARM64_FEATURE_FP,
            Arm64Feature::ANDROID_CPU_ARM64_FEATURE_ASIMD,
            Arm64Feature::ANDROID_CPU_ARM64_FEATURE_AES,
            Arm64Feature::ANDROID_CPU_ARM64_FEATURE_PMULL,
            Arm64Feature::ANDROID_CPU_ARM64_FEATURE_SHA1,
            Arm64Feature::ANDROID_CPU_ARM64_FEATURE_SHA2,
            Arm64Feature::ANDROID_CPU_ARM64_FEATURE_CRC32,
        }
        Ok(())
    }
    fn write_x86_features(f: &mut Formatter, features: u64, prefix: &str, suffix: &str) -> fmt::Result {
        extract_and_write_flags! {
            f, features, prefix, suffix,
            X86Feature::ANDROID_CPU_X86_FEATURE_SSSE3,
            X86Feature::ANDROID_CPU_X86_FEATURE_POPCNT,
            X86Feature::ANDROID_CPU_X86_FEATURE_MOVBE,
            X86Feature::ANDROID_CPU_X86_FEATURE_SSE4_1,
            X86Feature::ANDROID_CPU_X86_FEATURE_SSE4_2,
            X86Feature::ANDROID_CPU_X86_FEATURE_AES_NI,
            X86Feature::ANDROID_CPU_X86_FEATURE_AVX,
            X86Feature::ANDROID_CPU_X86_FEATURE_RDRAND,
            X86Feature::ANDROID_CPU_X86_FEATURE_AVX2,
            X86Feature::ANDROID_CPU_X86_FEATURE_SHA_NI,
        }
        Ok(())
    }

    fn write_mips_features(f: &mut Formatter, features: u64, prefix: &str, suffix: &str) -> fmt::Result {
        extract_and_write_flags! {
            f, features, prefix, suffix,
            MipsFeature::ANDROID_CPU_MIPS_FEATURE_R6,
            MipsFeature::ANDROID_CPU_MIPS_FEATURE_MSA,
        }
        Ok(())
    }
}

