li t1, 1
li s0, 10
li s1, 1

Loop:
    beq s0, s1, Exit
    add t0, t1, t0
    sd t0, 0(gp)
    addi gp, gp, 8
    addi s1, s1, 1
    addi t1, t1, 2
    beq zero, zero, Loop
Exit:
    nop