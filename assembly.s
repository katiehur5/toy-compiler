
.globl boo
boo:
pushq %rbp
movq %rsp, %rbp
subq $24, %rsp
movq %rdi, -8(%rbp)
movq %rsi, -16(%rbp)
movq -8(%rbp), %rax
movq $12, %rcx
addq %rcx, %rax
movq %rax, -24(%rbp)
movq -24(%rbp), %rax
addq $24, %rsp
popq %rbp
retq

.globl foo
foo:
pushq %rbp
movq %rsp, %rbp
subq $40, %rsp
movq %rdi, -8(%rbp)
movq %rsi, -16(%rbp)
movq -8(%rbp), %rax
movq $12, %rcx
addq %rcx, %rax
movq %rax, -24(%rbp)
movq $24, %rdi
movq -24(%rbp), %rsi
call boo
movq %rax, -32(%rbp)
movq -32(%rbp), %rax
movq -24(%rbp), %rcx
subq %rcx, %rax
movq %rax, -40(%rbp)
movq -40(%rbp), %rax
addq $40, %rsp
popq %rbp
retq
