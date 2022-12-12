.data
.dword 20
.text
ld s1, 0(gp)
li s0, 1
li t6, 0
Loop:
    beq s1, zero, Exit
    #multiply s0 = s0 * s1
    #addi sp, sp, -16
    add a1, s0, zero
    add a2, s1, zero
    beq t6, zero, NoSave_1
    jal ra, Store
    NoSave_1:
    jal ra, Mult
    
    beq t6, zero, NoSave_2
    ld s0, 16(sp)
    ld s1, 8(sp)
    addi sp, sp, 16
    NoSave_2:
    add s0, a0, zero
    
    li a0, 0
    
    addi s1, s1, -1
    beq zero, zero, Loop







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
        
        
        
Store:
    addi sp, sp, -16
    sd s0, 16(sp)
    sd s1, 8(sp)
    Store.Ret:
        jalr zero, ra, 0
    
Exit:
    sd s0, 16(gp)
