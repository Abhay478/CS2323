.data
.dword 13
.text
ld s1, 0(gp)
li s0, 1
Loop:
    beq s1, zero, Exit
    
    sd s1, 0(sp)
    addi sp, sp, -8
    #No need to store s0, since we are addding anew.
    
    #args
    add a1, s0, zero
    add a2, s1, zero
    li s0, 0
    li s1, 0
    
    #multiply s0 = s0 * s1
    jal ra, Mult

    #restore
    add s0, a0, zero
    li a0, 0
    ld s1, 8(sp)
    addi sp, sp, 8
    
    #decrement
    addi s1, s1, -1
    beq zero, zero, Loop


#{
Mult:
    li s0, 1
    beq a1, zero, Mult.Ret

    Mult.Loop:
        beq a2, zero, Mult.Ret
        and s1, a2, s0
        beq zero, s1, Mult.Skip
        add a0, a1, a0
        Mult.Skip:
            slli a1, a1, 1
            srai a2, a2, 1
            beq zero, zero, Mult.Loop
    Mult.Ret:
        li a1, 0
        jalr zero, ra, 0
#}

Exit:
    sd s0, 16(gp)


