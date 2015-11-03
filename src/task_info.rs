// Copyright 2015 Felix S. Klock II
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_camel_case_types)]

use libc::{self, c_int, uint32_t, uint64_t};
#[cfg(test)]
use libc::{c_char, size_t};
use std::mem;

// transcribed from vm_types.h
pub type mach_vm_address_t = uint64_t;
pub type mach_vm_size_t = uint64_t;
pub type integer_t = libc::c_int;
pub type natural_t = libc::c_uint;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct time_value { seconds: integer_t, microseconds: integer_t, }
pub type time_value_t = time_value;
pub type policy_t = libc::c_int;

/// Note: Comments in task_info.h say to use MachTaskBasicInfo instead.
/// (This is mainly put here for completeness.)
#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct TaskBasicInfo32 {
    /// suspend count for task
    pub suspend_count: integer_t,
    /// virtual memory size (bytes)
    pub virtual_size: natural_t,
    /// resident memory size (bytes)
    pub resident_size: natural_t,
    /// total user run time for terminated threads
    pub user_time: time_value_t,
    /// total system run time for terminated threads
    pub system_time: time_value_t,
    /// default policy for new threads
    pub policy: policy_t,
}

/// Note: Comments in task_info.h say to use MachTaskBasicInfo instead.
/// (This is mainly put here for completeness.)
#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct TaskBasicInfo64 {
    /// suspend count for task
    pub suspend_count: integer_t,
    /// virtual memory size (bytes)
    pub virtual_size: mach_vm_size_t,
    /// resident memory size (bytes)
    pub resident_size: mach_vm_size_t,
    /// total user run time for terminated threads
    pub user_time: time_value_t,
    /// total system run time for terminated threads
    pub system_time: time_value_t,
    /// default policy for new threads
    pub policy: policy_t,
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct TaskEventsInfo {
    /// number of page faults
    pub faults: integer_t,
    /// number of actual pageins
    pub pageins: integer_t,
    /// number of copy-on-write faults
    pub cow_faults: integer_t,
    /// number of messages sent
    pub messages_sent: integer_t,
    /// number of messages received
    pub messages_received: integer_t,
    /// number of mach system calls
    pub syscalls_mach: integer_t,
    /// number of unix system calls
    pub syscalls_unix: integer_t,
    /// number of context switches
    pub csw: integer_t,
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct TaskThreadTimesInfo {
    /// total user run time for live threads
    pub user_time: time_value_t,
    /// total system run time for live threads
    pub system_time: time_value_t,
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct TaskAbsolutetimeInfo {
    pub total_user: uint64_t,
    pub total_system: uint64_t,
    /// existing threads only
    pub threads_user: uint64_t,
    pub threads_system: uint64_t,
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct TaskKernelmemoryInfo {
    /// private kernel mem alloc'ed
    pub total_palloc: uint64_t,
    /// private kernel mem freed
    pub total_pfree: uint64_t,
    /// shared kernel mem alloc'ed
    pub total_salloc: uint64_t,
    /// shared kernel mem freed
    pub total_sfree: uint64_t,
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct TaskAffinityTagInfo {
    pub set_count: integer_t,
    pub min: integer_t,
    pub max: integer_t,
    pub task_count: integer_t,
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct TaskDyldInfo {
    pub all_image_info_addr: mach_vm_address_t,
    pub all_image_info_size: mach_vm_size_t,
    pub all_image_info_format: integer_t,
}

// #[repr(C)]
// struct TaskExtmodInfo {
//     task_uuid: [libc::c_uchar; 16],
//     extmod_statistics: vm_extmod_statistics_data_t,
// }

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct MachTaskBasicInfo {
    /// virtual memory size (bytes)
    pub virtual_size: mach_vm_size_t,
    /// resident memory size (bytes)
    pub resident_size: mach_vm_size_t,
    /// maximum resident memory size (bytes)
    pub resident_size_max: mach_vm_size_t,
    /// total user run time for terminated threads
    pub user_time: time_value_t,
    /// total system run time for terminated threads
    pub system_time: time_value_t,
    /// default policy for new threads
    pub policy: policy_t,
    /// suspend count for task
    pub suspend_count: integer_t,
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct TaskPowerInfo {
    pub total_user: uint64_t,
    pub total_system: uint64_t,
    pub task_interrupt_wakeups: uint64_t,
    pub task_platform_idle_wakeups: uint64_t,
    pub task_timer_wakeups_bin_1: uint64_t,
    pub task_timer_wakeups_bin_2: uint64_t,
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct TaskVmInfo {
    /// virtual memory size (bytes)
    pub virtual_size: mach_vm_size_t,
    /// number of memory regions
    pub region_count: integer_t,
    pub page_size: integer_t,
    /// resident memory size (bytes)
    pub resident_size: mach_vm_size_t,
    /// peak resident size (bytes)
    pub resident_size_peak: mach_vm_size_t,

    pub device: mach_vm_size_t,
    pub device_peak: mach_vm_size_t,
    pub internal: mach_vm_size_t,
    pub internal_peak: mach_vm_size_t,
    pub external: mach_vm_size_t,
    pub external_peak: mach_vm_size_t,
    pub reusable: mach_vm_size_t,
    pub reusable_peak: mach_vm_size_t,
    pub purgeable_volatile_pmap: mach_vm_size_t,
    pub purgeable_volatile_resident: mach_vm_size_t,
    pub purgeable_volatile_virtual: mach_vm_size_t,
    pub compressed: mach_vm_size_t,
    pub compressed_peak: mach_vm_size_t,
    pub compressed_lifetime: mach_vm_size_t,

    /* added for rev1 */
    pub phys_footprint: mach_vm_size_t,
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct TaskVmInfoPurgeable(TaskVmInfo);

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct TaskTraceMemoryInfo {
    /// address of start of trace memory buffer
    pub user_memory_address: uint64_t,
    /// size of buffer in bytes
    pub buffer_size: uint64_t,
    /// size of mailbox area in bytes
    pub mailbox_array_size: uint64_t,
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct TaskWaitStateInfo {
    /// Time that all threads past and present have been in a wait state
    pub total_wait_state_time: uint64_t,
    /// Time that threads have been in SFI wait (should be a subset of total wait state time
    pub total_wait_sfi_state_time: uint64_t,
    _reserved: [uint32_t; 4],
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct GpuEnergyData {
    pub task_gpu_utilisation: uint64_t,
    pub task_gpu_stat_reserved0: uint64_t,
    pub task_gpu_stat_reserved1: uint64_t,
    pub task_gpu_stat_reserved2: uint64_t,
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct TaskPowerInfoV2 {
    pub cpu_energy: TaskPowerInfo,
    pub gpu_energy: GpuEnergyData,
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct TaskFlagsInfo {
    /// task flags
    pub flags: uint32_t,
}

extern {
    fn task_info_self_init(flavor: c_int, info: *mut c_int) -> c_int;
    #[cfg(test)]
    fn task_info_sizeof_struct_for_flavor(flavor: c_int) -> libc::size_t;
    #[cfg(test)]
    fn task_info_value_for_flavor(name: *const c_char) -> libc::c_int;
    #[cfg(test)]
    fn task_info_print_field_sizes(flavor: c_int);
}

pub trait Flavor: Sized {
    fn variant() -> FlavorVariant;
    fn c_name() -> &'static str;
    unsafe fn init_self(&mut self) {
        let ret =
            task_info_self_init(Self::variant() as c_int,
                                self as *mut Self as *mut c_int);
        if ret < 0 {
            panic!("init_self failed for {:?}", Self::c_name());
        }
    }

    fn new() -> Self {
        unsafe {
            let mut info: Self = mem::uninitialized();
            info.init_self();
            info
        }
    }

    // for debugging the self-consistency checking; if overridden,
    // should print size info for struct and its fields to stdout.
    fn describe() { }
}

impl Flavor for TaskBasicInfo32 {
    fn variant() -> FlavorVariant { FlavorVariant::TaskBasicInfo32 }
    fn c_name() -> &'static str { "TASK_BASIC_INFO_32\0" }
}

impl Flavor for TaskBasicInfo64 {
    fn variant() -> FlavorVariant { FlavorVariant::TaskBasicInfo64 }
    fn c_name() -> &'static str { "TASK_BASIC_INFO_64\0" }

    fn describe() {
println!(r#"\
pub struct TaskBasicInfo64 {{
    /// suspend count for task
    pub suspend_count: integer_t = {integer_t},
    /// virtual memory size (bytes)
    pub virtual_size: mach_vm_size_t = {mach_vm_size_t},
    /// resident memory size (bytes)
    pub resident_size: mach_vm_size_t = {mach_vm_size_t},
    /// total user run time for terminated threads
    pub user_time: time_value_t = {time_value_t},
    /// total system run time for terminated threads
    pub system_time: time_value_t = {time_value_t},
    /// default policy for new threads
    pub policy: policy_t = {policy_t},
}}\
"#,
         integer_t=mem::size_of::<integer_t>(),
         mach_vm_size_t=mem::size_of::<mach_vm_size_t>(),
         time_value_t=mem::size_of::<time_value_t>(),
         policy_t=mem::size_of::<policy_t>(),
         );
    }
}

impl Flavor for TaskEventsInfo {
    fn variant() -> FlavorVariant { FlavorVariant::TaskEventsInfo }
    fn c_name() -> &'static str { "TASK_EVENTS_INFO\0" }
}

impl Flavor for TaskThreadTimesInfo {
    fn variant() -> FlavorVariant { FlavorVariant::TaskThreadTimesInfo }
    fn c_name() -> &'static str { "TASK_THREAD_TIMES_INFO\0" }
}

impl Flavor for TaskAbsolutetimeInfo {
    fn variant() -> FlavorVariant { FlavorVariant::TaskAbsolutetimeInfo }
    fn c_name() -> &'static str { "TASK_ABSOLUTETIME_INFO\0" }
}

impl Flavor for TaskKernelmemoryInfo {
    fn variant() -> FlavorVariant { FlavorVariant::TaskKernelmemoryInfo }
    fn c_name() -> &'static str { "TASK_KERNELMEMORY_INFO\0" }
}

impl Flavor for TaskAffinityTagInfo {
    fn variant() -> FlavorVariant { FlavorVariant::TaskAffinityTagInfo }
    fn c_name() -> &'static str { "TASK_AFFINITY_TAG_INFO\0" }
}

impl Flavor for TaskDyldInfo {
    fn variant() -> FlavorVariant { FlavorVariant::TaskDyldInfo }
    fn c_name() -> &'static str { "TASK_DYLD_INFO\0" }
}

// impl Flavor for TaskExtmodInfo {
//     fn variant() -> FlavorVariant { FlavorVariant::TaskExtmodInfo }
//     fn c_name() -> &'static str { "TASK_EXTMOD_INFO\0" }
// }

impl Flavor for MachTaskBasicInfo {
    fn variant() -> FlavorVariant { FlavorVariant::MachTaskBasicInfo }
    fn c_name() -> &'static str { "MACH_TASK_BASIC_INFO\0" }
}

impl Flavor for TaskPowerInfo {
    fn variant() -> FlavorVariant { FlavorVariant::TaskPowerInfo }
    fn c_name() -> &'static str { "TASK_POWER_INFO\0" }
}

impl Flavor for TaskVmInfo {
    fn variant() -> FlavorVariant { FlavorVariant::TaskVmInfo }
    fn c_name() -> &'static str { "TASK_VM_INFO\0" }
}

impl Flavor for TaskVmInfoPurgeable {
    fn variant() -> FlavorVariant { FlavorVariant::TaskVmInfoPurgeable }
    fn c_name() -> &'static str { "TASK_VM_INFO_PURGEABLE\0" }
}

impl Flavor for TaskTraceMemoryInfo {
    fn variant() -> FlavorVariant { FlavorVariant::TaskTraceMemoryInfo }
    fn c_name() -> &'static str { "TASK_TRACE_MEMORY_INFO\0" }
}

impl Flavor for TaskWaitStateInfo {
    fn variant() -> FlavorVariant { FlavorVariant::TaskWaitStateInfo }
    fn c_name() -> &'static str { "TASK_WAIT_STATE_INFO\0" }
}

impl Flavor for TaskPowerInfoV2 {
    fn variant() -> FlavorVariant { FlavorVariant::TaskPowerInfoV2 }
    fn c_name() -> &'static str { "TASK_POWER_INFO_V2\0" }
}

impl Flavor for TaskFlagsInfo {
    fn variant() -> FlavorVariant { FlavorVariant::TaskFlagsInfo }
    fn c_name() -> &'static str { "TASK_FLAGS_INFO\0" }
}

#[repr(C)]
pub enum FlavorVariant {
    TaskBasicInfo32 = 4,
    TaskBasicInfo64 = 5,
    TaskEventsInfo = 2,
    TaskThreadTimesInfo = 3,
    TaskAbsolutetimeInfo = 1,

    TaskKernelmemoryInfo = 7,

    // flavor defined in task_info.h but not struct
    // TaskSecurityToken = 13,
    // TaskAuditToken = 15,

    TaskAffinityTagInfo = 16,
    TaskDyldInfo = 17,

    // struct defined in vm_statistics.h uses aligned directive;
    // too risky to try to support for now.
    // TaskExtmodInfo = 19,

    MachTaskBasicInfo = 20,
    TaskPowerInfo = 21,
    TaskVmInfo = 22,
    TaskVmInfoPurgeable = 23,

    TaskTraceMemoryInfo = 24,
    TaskWaitStateInfo = 25,
    TaskPowerInfoV2 = 26,
    TaskFlagsInfo = 28,
}

#[allow(dead_code)]
const FLAVORS: [FlavorVariant; 16] = [
    FlavorVariant::TaskBasicInfo32,
    FlavorVariant::TaskBasicInfo64,
    FlavorVariant::TaskEventsInfo,
    FlavorVariant::TaskThreadTimesInfo,
    FlavorVariant::TaskAbsolutetimeInfo,

    FlavorVariant::TaskKernelmemoryInfo,
    FlavorVariant::TaskAffinityTagInfo,
    FlavorVariant::TaskDyldInfo,
    FlavorVariant::MachTaskBasicInfo,
    FlavorVariant::TaskPowerInfo,

    FlavorVariant::TaskVmInfo,
    FlavorVariant::TaskVmInfoPurgeable,
    FlavorVariant::TaskTraceMemoryInfo,
    FlavorVariant::TaskWaitStateInfo,
    FlavorVariant::TaskPowerInfoV2,

    FlavorVariant::TaskFlagsInfo,
    ];

macro_rules! for_each_flavor {
    ($id:ident) => {
        $id!(TaskBasicInfo32);
        $id!(TaskBasicInfo64);
        $id!(TaskEventsInfo);
        $id!(TaskThreadTimesInfo);
        $id!(TaskAbsolutetimeInfo);
        $id!(TaskKernelmemoryInfo);
        $id!(TaskAffinityTagInfo);
        $id!(TaskDyldInfo);
        $id!(MachTaskBasicInfo);
        $id!(TaskPowerInfo);
        $id!(TaskVmInfo);
        $id!(TaskVmInfoPurgeable);
        $id!(TaskTraceMemoryInfo);
        $id!(TaskWaitStateInfo);
        $id!(TaskPowerInfoV2);
        $id!(TaskFlagsInfo);
    }
}

#[test]
fn check_flavor_values() {
    macro_rules! check { ($id:ident) => { check_value::<$id>(); } }
    for_each_flavor!(check);
}

#[test]
fn check_sizes() {
    macro_rules! check { ($id:ident) => { check_size::<$id>(); } }
    for_each_flavor!(check);
}

#[cfg(test)]
fn check_value<F:Flavor>() {
    let r_val = F::variant() as c_int;
    let c_val = unsafe {
        task_info_value_for_flavor(F::c_name().as_ptr() as *const c_char)
    };
    assert!(c_val >= 0, "Flavor {} val: {} lookup negative {}",
            F::c_name(), r_val, c_val);
    assert!(r_val == c_val, "Flavor {} val: {} != C {}",
            F::c_name(), r_val, c_val);
}

#[cfg(test)]
fn check_size<F:Flavor>() {
    unsafe {
        let flavor = F::variant() as c_int;
        let expect = task_info_sizeof_struct_for_flavor(flavor);
        let actual = mem::size_of::<F>() as libc::size_t;

        if actual != expect {
            task_info_print_field_sizes(flavor);
            F::describe();
        }

        assert!(actual == expect,
                "{name:?} mismatch actual: {actual} expect: {expect}",
                name=F::c_name(), actual=actual, expect=expect
                );
    }
}
