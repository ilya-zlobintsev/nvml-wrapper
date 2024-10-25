use crate::bindings::nvmlClockOffset_v1_t;
use std::mem;

const fn struct_version<T>(ver: u32) -> u32 {
    mem::size_of::<T>() as u32 | (ver << 24u32)
}

pub const NVML_CLOCK_OFFSET_V1: u32 = struct_version::<nvmlClockOffset_v1_t>(1);
