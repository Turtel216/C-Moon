.intel_syntax noprefix
.section .text

.globl main
.type main, @function
main:
    # --- main prologue ---
    push rbp
    mov rbp, rsp
.main_entry:
    jmp .if_then_L1
.if_then_L1:
    mov rcx, 2
    jmp .if_end_L3
.if_end_L3:
    mov rsi, rcx
    add rsi, 10
    mov rcx, rsi
    mov rax, rcx
    jmp .main_epilogue
.main_exit:
.main_epilogue:
    mov rsp, rbp
    pop rbp
    ret

