.data
.dword 10
.text
ld s1, 0(gp)
li s0, 1
Loop:
    beq s1, zero, Exit
    #multiply s0 = s0 * s1
    #addi sp, sp, -16
    add a1, s0, zero
    add a2, s1, zero
    
    jal ra, Mult
    
    add s0, a0, zero
    li a0, 0
    
    addi s1, s1, -1
    beq zero, zero, Loop







Mult:
    li t0, 1
    beq a1, zero, Ret
    
    Mult.Loop:
        beq a2, zero, Ret
        and t1 a2, t0
        beq zero, t1, Skip
        add a0 a1, a0 
        Skip:
            slli a1, a1, 1
            srai a2, a2, 1
            beq zero, zero, Mult.Loop
    Ret:
        jalr zero, ra, 0
    
Exit:
    sd s0, 16(gp)
