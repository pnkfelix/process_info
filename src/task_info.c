// Copyright 2015 Felix S. Klock II
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Adapted from http://stackoverflow.com/a/1911863/36585
//
// See also servo/support/rust-task_info/

#include <mach/mach.h>

int task_info_self_init(int flavor, task_info_t info) {
    mach_msg_type_number_t t_info_count;

#define ARM(K) case K: t_info_count = K##_COUNT; break;

    switch (flavor) {
        // these were extracted by inspecting task_info.h and
        // finding the definitions of *_COUNT.

        ARM(TASK_BASIC_INFO_32);
        ARM(TASK_BASIC_INFO_64);
        ARM(TASK_EVENTS_INFO);
        ARM(TASK_THREAD_TIMES_INFO);
        ARM(TASK_ABSOLUTETIME_INFO);
        ARM(TASK_KERNELMEMORY_INFO);
        ARM(TASK_SECURITY_TOKEN);
        ARM(TASK_AUDIT_TOKEN);
        ARM(TASK_AFFINITY_TAG_INFO);
        ARM(TASK_DYLD_INFO);
        ARM(TASK_EXTMOD_INFO);
        ARM(MACH_TASK_BASIC_INFO);
        ARM(TASK_POWER_INFO);
        ARM(TASK_VM_INFO);
        // ARM(TASK_VM_INFO_REV0);
        ARM(TASK_TRACE_MEMORY_INFO);
        ARM(TASK_WAIT_STATE_INFO);
        ARM(TASK_POWER_INFO_V2);
        ARM(TASK_FLAGS_INFO);

    default:
        // oops.
        return -1;
    }

    if (KERN_SUCCESS != task_info(mach_task_self(),
                                  flavor,
                                  info,
                                  &t_info_count))
    {
        return -2;
    }

    return 0;
}

size_t task_info_sizeof_struct_for_flavor(int flavor) {

#define ARM(K, T) case K: return sizeof(T)

    switch (flavor) {
        ARM(TASK_BASIC_INFO_32,     struct task_basic_info_32);
        ARM(TASK_BASIC_INFO_64,     struct task_basic_info_64);
        ARM(TASK_EVENTS_INFO,       struct task_events_info);
        ARM(TASK_THREAD_TIMES_INFO, struct task_thread_times_info);
        ARM(TASK_ABSOLUTETIME_INFO, struct task_absolutetime_info);
        ARM(TASK_KERNELMEMORY_INFO, struct task_kernelmemory_info);
        ARM(TASK_SECURITY_TOKEN,    security_token_t);
        ARM(TASK_AUDIT_TOKEN,       audit_token_t);
        ARM(TASK_AFFINITY_TAG_INFO, struct task_affinity_tag_info);
        ARM(TASK_DYLD_INFO,         struct task_dyld_info);
        ARM(TASK_EXTMOD_INFO,       struct task_extmod_info);
        ARM(MACH_TASK_BASIC_INFO,   struct mach_task_basic_info);
        ARM(TASK_POWER_INFO,        struct task_power_info);
        ARM(TASK_VM_INFO,           struct task_vm_info);
        // ARM(TASK_VM_INFO_REV0,      struct task_vm_info_rev0);
        ARM(TASK_TRACE_MEMORY_INFO, struct task_trace_memory_info);
        ARM(TASK_WAIT_STATE_INFO,   struct task_wait_state_info);
        ARM(TASK_POWER_INFO_V2,     struct task_power_info_v2);
        ARM(TASK_FLAGS_INFO,        struct task_flags_info);

    default:
        return 0;
    }
}

int task_info_value_for_flavor(char *name) {

#define ARM(K) if (strcmp(name, #K) == 0) { return K; }

    ARM(TASK_BASIC_INFO_32);
    ARM(TASK_BASIC_INFO_64);
    ARM(TASK_EVENTS_INFO);
    ARM(TASK_THREAD_TIMES_INFO);
    ARM(TASK_ABSOLUTETIME_INFO);
    ARM(TASK_KERNELMEMORY_INFO);
    ARM(TASK_SECURITY_TOKEN);
    ARM(TASK_AUDIT_TOKEN);
    ARM(TASK_AFFINITY_TAG_INFO);
    ARM(TASK_DYLD_INFO);
    ARM(TASK_EXTMOD_INFO);
    ARM(MACH_TASK_BASIC_INFO);
    ARM(TASK_POWER_INFO);
    ARM(TASK_VM_INFO);
    // ARM(TASK_VM_INFO_REV0);
    ARM(TASK_TRACE_MEMORY_INFO);
    ARM(TASK_WAIT_STATE_INFO);
    ARM(TASK_POWER_INFO_V2);
    ARM(TASK_FLAGS_INFO);

    return -1;
}

void task_info_print_field_sizes(int flavor) {

#define HEADER(N) struct N s; printf("%s (%d) {\n", #N, sizeof(struct N));
#define FIELD(N) printf("    %s: %d,\n", #N, sizeof(s.N));
#define FOOTER() printf("}\n"); return;

    switch (flavor) {
    case TASK_BASIC_INFO_32: {
        HEADER(task_basic_info_32);
        FIELD(suspend_count);
        FIELD(virtual_size);
        FIELD(resident_size);
        FIELD(user_time);
        FIELD(system_time);
        FIELD(policy);
        FOOTER();
    }
    case TASK_BASIC_INFO_64: {
        HEADER(task_basic_info_64);
        FIELD(suspend_count);
        FIELD(virtual_size);
        FIELD(resident_size);
        FIELD(user_time);
        FIELD(system_time);
        FIELD(policy);
        FOOTER();
    }
    default:
        printf("unhandled flavor: %d\n", flavor);
        return;
    }
}
