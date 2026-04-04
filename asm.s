.intel_syntax noprefix
.section .text

.globl main
.type main, @function
main:
    # --- main prologue ---
    push rbp
    mov rbp, rsp
.main_entry:
    mov r10, 1
    test r10, r10
    je .if_else_L2
    jmp .if_then_L1
.if_else_L2:
    mov rcx, 100
    jmp .if_end_L3
.if_end_L3:
    mov rsi, rcx
    add rsi, 10
    mov rdi, rsi
    mov rax, rdi
    jmp .main_epilogue
.if_then_L1:
    mov rcx, 2
    jmp .if_end_L3
.main_exit:
.main_epilogue:
    mov rsp, rbp
    pop rbp
    ret

