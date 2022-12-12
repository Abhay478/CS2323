li t0, 11
li t1, 1
Loop:
    sd t1, 0(gp)
    addi gp, gp, 8
    addi t1, t1, 1
    blt t1, t0, Loop
Exit:
    ld a5, -8(gp)
    