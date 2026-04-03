.intel_syntax noprefix
.section .text

.globl main
.type main, @function
main:
    # --- main prologue ---
    push rbp
    mov rbp, rsp
.main_entry:
    mov rax, 21
    jmp .main_epilogue
.main_exit:
.main_epilogue:
    mov rsp, rbp
    pop rbp
    ret

