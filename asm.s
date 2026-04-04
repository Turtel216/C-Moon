.intel_syntax noprefix
.section .text

.globl add
.type add, @function
add:
    # --- add prologue ---
    push rbp
    mov rbp, rsp
.add_entry:
    mov rcx, rdi
    mov rdi, rcx
    add rdi, rsi
    mov rax, rdi
    jmp .add_epilogue
.add_exit:
.add_epilogue:
    mov rsp, rbp
    pop rbp
    ret

.globl main
.type main, @function
main:
    # --- main prologue ---
    push rbp
    mov rbp, rsp
.main_entry:
    mov rdi, 10
    mov rsi, 15
    call add
    mov rcx, rax
    mov rsi, rcx
    mov rax, rsi
    jmp .main_epilogue
.main_exit:
.main_epilogue:
    mov rsp, rbp
    pop rbp
    ret

