.intel_syntax noprefix
.section .text

.globl main
.type main, @function
main:
    push rbp
    mov rbp, rsp
.main_entry:
    mov rax, 25
    jmp .main_epilogue
.main_exit:
.main_epilogue:
    mov rsp, rbp
    pop rbp
    ret

