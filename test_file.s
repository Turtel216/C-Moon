.intel_syntax noprefix
.section .text

.globl main
.type main, @function
main:
    push rbp
    mov rbp, rsp
.main_entry:
    mov rcx, 2
    jmp .while_cond_L1
.while_cond_L1:
    cmp rcx, 2
    setne al
    movzx esi, al
    test rsi, rsi
    je .while_end_L3
    jmp .while_body_L2
.while_end_L3:
    mov rsi, rcx
    add rsi, 1
    mov rax, rsi
    jmp .main_epilogue
.while_body_L2:
    mov rsi, rcx
    add rsi, 1
    mov rcx, rsi
    jmp .while_cond_L1
.main_exit:
.main_epilogue:
    mov rsp, rbp
    pop rbp
    ret

