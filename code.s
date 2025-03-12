
strlen:
        add      t0, x0, a0
loop:
        lb       t2, 0(t0)
        add      t0, t0, 1
        bne      t2, x0, loop
        sub      a0, t0, a0
        addi     a0, a0, -1
        ret
puts:
        mv       a2, a1
        mv       a1, a0
        addi     a0, x0, 1
        addi     a7, x0, 64
        ecall
        ret

        .section .init
        .global  _start
_start:
        la       a0, helloworld
        mv       t4, a0
        call     strlen
        mv       a1, a0
        mv       a0, t4
        call     puts

        addi     a0, x0, 1
        addi     a7, x0, 93
        ecall
        .data
helloworld:
        .string  "Hello World!\n"
