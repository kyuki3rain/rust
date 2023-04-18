.intel_syntax noprefix
.globl main
main:
   push 42
   push 32
   pop rdi
   pop rax
   add rax, rdi
   push rax
   push 2
   pop rdi
   pop rax
   sub rax, rdi
   push rax
   pop rax
   ret

