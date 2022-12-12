.data
.dword 5
.text
ld s3, 0(gp)
addi s4, s4, s3
addi s5, s5, s3


Loop:
    #use s3, s4, s5
    #multiply s3 = s3 * s4
    #decrement s5
    beq s5, zero, Exit










Mult:
    li t0, 1
    beq a1, zero, Mult.Ret

    Mult.Loop:
        beq a2, zero, Mult.Ret
        and t1 a2, t0
        beq zero, t1, Skip
        add a0 a1, a0
        Skip:
            slli a1, a1, 1
            srai a2, a2, 1
            beq zero, zero, Mult.Loop
    Mult.Ret:
        jalr zero, ra, 0



Exit:
    
