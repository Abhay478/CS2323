




.text
li s2, 1
Fact:
    ld s0, 0(gp)
    blt s0, s2, Def # Zero
    addi s1, s0, -1
    #s0 will hold the result, s1 the counter to be decremented
    Loop:
        #saving
        addi sp, sp, -16
        sd s0, 16(sp)
        sd s1, 8(sp)
        #args
        add a1, a1, s0
        add a2, a2, s1
        #call
        jal ra, Mult
        #restored
        addi s0, 
        addi s1, s1, -1
        blt s1, zero, Exit
        beq zero, zero, Loop

Mult:
    #args in a1, a2
    #return value in a0
    
    
Exit:
    sd s0, 16(gp)
    
Def:
    li s0, 1
    beq zero, zero, Exit