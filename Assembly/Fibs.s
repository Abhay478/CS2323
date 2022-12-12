li t0, 1
li t1, 1
li s0, 8
li s1, 1

sd t0, 0(gp)
addi gp, gp, 8
sd t1, 0(gp)
addi gp, gp, 8

Loop:
    beq s0, s1, Exit
    add t2, t0, t1
    sd t2, 0(gp)
    addi gp, gp, 8
    add t0, t1, zero
    add t1, t2, zero
    addi s1, s1, 1
    beq zero, zero, Loop
Exit:
    nop
