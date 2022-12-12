#{
Mult:
    li t0, 1
    beq a1, zero, Mult.Ret

    Mult.Loop:
        beq a2, zero, Mult.Ret
        and t1, a2, t0
        beq zero, t1, Mult.Skip
        add a0, a1, a0
        Mult.Skip:
            slli a1, a1, 1
            srai a2, a2, 1
            beq zero, zero, Mult.Loop
    Mult.Ret:
        li a1, 0
        jalr zero, ra, 0
#}

