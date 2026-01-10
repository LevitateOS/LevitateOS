// TEAM_409: Context switch with NEON/FPU state preservation
// Context layout:
//   0-88:   x19-x30 (callee-saved GPRs)
//   96:     sp
//   104:    tpidr_el0
//   112:    fpu_state.v_regs[0-31] (512 bytes)
//   624:    fpu_state.fpcr
//   628:    fpu_state.fpsr

.global cpu_switch_to
cpu_switch_to:
    // Save callee-saved GPRs
    mov     x10, sp
    stp     x19, x20, [x0, #0]
    stp     x21, x22, [x0, #16]
    stp     x23, x24, [x0, #32]
    stp     x25, x26, [x0, #48]
    stp     x27, x28, [x0, #64]
    stp     x29, x30, [x0, #80]
    str     x10,      [x0, #96]
    mrs     x11,      tpidr_el0
    str     x11,      [x0, #104]

    // TEAM_409: Save NEON/FPU registers (V0-V31)
    add     x9, x0, #112           // x9 = &context.fpu_state.v_regs
    stp     q0,  q1,  [x9, #0]
    stp     q2,  q3,  [x9, #32]
    stp     q4,  q5,  [x9, #64]
    stp     q6,  q7,  [x9, #96]
    stp     q8,  q9,  [x9, #128]
    stp     q10, q11, [x9, #160]
    stp     q12, q13, [x9, #192]
    stp     q14, q15, [x9, #224]
    stp     q16, q17, [x9, #256]
    stp     q18, q19, [x9, #288]
    stp     q20, q21, [x9, #320]
    stp     q22, q23, [x9, #352]
    stp     q24, q25, [x9, #384]
    stp     q26, q27, [x9, #416]
    stp     q28, q29, [x9, #448]
    stp     q30, q31, [x9, #480]
    // Save FPCR and FPSR
    mrs     x10, fpcr
    mrs     x11, fpsr
    str     w10, [x0, #624]        // fpcr (32-bit)
    str     w11, [x0, #628]        // fpsr (32-bit)

    // Restore callee-saved GPRs from new context
    ldp     x19, x20, [x1, #0]
    ldp     x21, x22, [x1, #16]
    ldp     x23, x24, [x1, #32]
    ldp     x25, x26, [x1, #48]
    ldp     x27, x28, [x1, #64]
    ldp     x29, x30, [x1, #80]
    ldr     x10,      [x1, #96]
    ldr     x11,      [x1, #104]
    msr     tpidr_el0, x11
    mov     sp, x10

    // TEAM_409: Restore NEON/FPU registers (V0-V31)
    add     x9, x1, #112           // x9 = &context.fpu_state.v_regs
    ldp     q0,  q1,  [x9, #0]
    ldp     q2,  q3,  [x9, #32]
    ldp     q4,  q5,  [x9, #64]
    ldp     q6,  q7,  [x9, #96]
    ldp     q8,  q9,  [x9, #128]
    ldp     q10, q11, [x9, #160]
    ldp     q12, q13, [x9, #192]
    ldp     q14, q15, [x9, #224]
    ldp     q16, q17, [x9, #256]
    ldp     q18, q19, [x9, #288]
    ldp     q20, q21, [x9, #320]
    ldp     q22, q23, [x9, #352]
    ldp     q24, q25, [x9, #384]
    ldp     q26, q27, [x9, #416]
    ldp     q28, q29, [x9, #448]
    ldp     q30, q31, [x9, #480]
    // Restore FPCR and FPSR
    ldr     w10, [x1, #624]
    ldr     w11, [x1, #628]
    msr     fpcr, x10
    msr     fpsr, x11

    ret

.global task_entry_trampoline
task_entry_trampoline:
    bl      post_switch_hook
    mov     x0, #0
    blr     x19
    bl      task_exit
    b       .
