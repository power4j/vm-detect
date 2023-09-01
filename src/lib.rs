#![no_std]
#![cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use core::arch::x86_64::{__cpuid, _rdtsc};

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Detection: u32 {
        /// Measures the time/instruction count it takes to execute the cpuid instruction
        const RDTSC = 0b00001;
        /// Checks the hypervisor present bit in ecx after running cpuid leaf 1
        const HYPERVISOR_BIT = 0b00010;
        /// Checks if the CPU manufacturer id is of a known hypervisor
        const HYPERVISOR_CPU_VENDOR = 0b00100;
        /// Checks if the CPU manufacturer id is not GenuineIntel or AuthenticAMD
        const UNEXPECTED_CPU_VENDOR = 0b01000;
    }
}

const KNOWN_HYPERVISORS: [u32; 7] = [
    fnv1(b"bhyve bhyve "),
    fnv1(b"KVMKVMKVM\0\0\0"),
    fnv1(b"TCGTCGTCGTCG"),
    fnv1(b"Microsoft Hv"),
    fnv1(b" lrpepyh  vr"),
    fnv1(b"VMwareVMware"),
    fnv1(b"XenVMMXenVMM"),
];

const EXPECTED_VENDORS: [u32; 2] = [fnv1(b"GenuineIntel"), fnv1(b"AuthenticAMD")];

/// Run basic hypervisor detection
///
/// You can inspect the returned [`Detection`] to see which checks triggered (if any).
pub fn vm_detect() -> Detection {
    let mut detection = Detection::empty();

    if rdtsc_detection() {
        detection |= Detection::RDTSC;
    }

    if hypervisor_vendor().is_some() {
        detection |= Detection::HYPERVISOR_BIT;
    }

    let vendor_hash = fnv1(&cpu_vendor());

    if KNOWN_HYPERVISORS.iter().any(|&hash| hash == vendor_hash) {
        detection |= Detection::HYPERVISOR_CPU_VENDOR;
    } else if !EXPECTED_VENDORS.iter().any(|&hash| hash == vendor_hash) {
        detection |= Detection::UNEXPECTED_CPU_VENDOR;
    }

    detection
}

#[inline(always)]
const fn fnv1(data: &[u8; 12]) -> u32 {
    let mut res: u32 = 0x811c9dc5;

    let mut i = 0;
    while i < data.len() {
        res = res.wrapping_mul(0x01000193) ^ (data[i] as u32);

        i += 1;
    }

    res
}

#[allow(dead_code)]
fn brand_name() -> Option<[u8; 16 * 3]> {
    if unsafe { __cpuid(0x80000000) }.eax < 0x80000004 {
        return None;
    }
    let cpuids: [_; 3] = unsafe {
        [
            __cpuid(0x80000002),
            __cpuid(0x80000003),
            __cpuid(0x80000004),
        ]
    };

    let mut result = [0; 16 * 3];

    for (i, cpuid) in cpuids.into_iter().enumerate() {
        for (j, data) in [cpuid.eax, cpuid.ebx, cpuid.ecx, cpuid.edx]
            .into_iter()
            .enumerate()
        {
            let index = i * 16 + j * 4;
            result[index..index + 4].copy_from_slice(&data.to_le_bytes());
        }
    }

    Some(result)
}

fn rdtsc_detection() -> bool {
    const ITERATIONS: u64 = 1000;

    let sum: u64 = (0..ITERATIONS)
        .map(|_| unsafe {
            let start = _rdtsc();
            let _ = __cpuid(0);
            let end = _rdtsc();
            end - start
        })
        .sum();

    let avg = sum / ITERATIONS;
    avg > 500
}

fn cpu_vendor() -> [u8; 12] {
    let cpuid = unsafe { __cpuid(0) };

    let mut result = [0; 12];

    for (i, data) in [cpuid.ebx, cpuid.edx, cpuid.ecx].into_iter().enumerate() {
        let index = i * 4;
        result[index..index + 4].copy_from_slice(&data.to_le_bytes());
    }

    result
}

fn hypervisor_vendor() -> Option<[u8; 12]> {
    let processor_info = unsafe { __cpuid(1) };
    // Check Hypervisor Present bit
    if processor_info.ecx >> 31 & 1 == 0 {
        return None;
    }

    let cpuid = unsafe { __cpuid(0x40000000) };

    let mut result = [0; 12];

    for (i, data) in [cpuid.ebx, cpuid.ecx, cpuid.edx].into_iter().enumerate() {
        let index = i * 4;
        result[index..index + 4].copy_from_slice(&data.to_le_bytes());
    }

    Some(result)
}
