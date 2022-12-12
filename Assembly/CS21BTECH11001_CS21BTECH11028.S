.data
L1: .word 0 #error flag
D1:.word 4, 4 #dimensions
M1:.word 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16 #matrix

D2:.word 4, 1 #dimensions
M2:.word 1, 1, 1, 1 #matrix

D3:.word 0, 0
M3:.word 0
.global main
.text


main:
    la s0, D1
    lw s1, 0(s0) #rows, do not change
    lw s2, 4(s0) #columns, do not change
    la s0, D2
    lw s10, 0(s0) #rows, do not change
    lw s11, 4(s0) #columns, do not change
    
    bne s2, s10, Exit
    
    la s8, M1 #matrix addresses, do not change
    la s9, M2
    
    li s0, 4 #dummy, do not change. When ld-->lw, change to 4
    mul a5, s1, s2
    mul a5, a5, s0 #byte offset for size of M1
    mul a6, s10, s11
    mul a6, a6, s0 #byte offset for size of M2
    
    jal Transpose
    add a0, zero, s8
    addi a1, s9, 0
    #addi a2, s2, 0
    jal Mult
    
    la gp, D3
    sw s1, 0(gp)
    add gp, gp, s0
    sw s11, 0(gp)
    add gp, gp, s0 
    #addi gp, gp, 0xf0
    Loop:
        beq tp, sp, Ret
        lw t6, 0(tp)
        sw t6, 0(gp)
        sub tp, tp, s0
        add gp, gp, s0
        beq zero, zero, Loop
    
    Ret:
        jal zero, Ret
    
    
    
Mult:
    
    mul s6, s0, s10 #byte offset to shift to next row of transposed M2
    mul s7, s0, s2 #byte offset to shift to next row of M1
    
    #takes addresses of two matrices in a0, a1, stores to data segment
    add tp, zero, sp
    Mult.Outer:
        beq a3, a5, Mult.Ret
        Mult.Inner:
            beq t3, a6, Mult.Inner.End
            add a0, a3, s8 #mat1
            add a1, t3, s9 #mat2
            addi a2, s10, 0
            
            jal gp, Dot
            
            sw a0, 0(sp)
            sub sp, sp, s0
            add t3, t3, s6
            beq zero, zero, Mult.Inner
        Mult.Inner.End:
            add a3, a3, s7
            li t3, 0
            beq zero, zero, Mult.Outer
            
    Mult.Ret:
        lui gp, 0x10000
        ret
            
            
            
Transpose:
    #takes no arguments, very specific to this program.
    #minor changes should make it general
    
    
    mul a1, s11, s0 #24 bytes
    #mul s3, s2, s0
    addi tp, sp, 0
    Transpose.Loop:
        beq t1, a1, Transpose.After
        addi a2, s10, 0 #rows
        add a0, zero, s9
        add a0, a0, t1
        
        jal gp, Range
        
        add t1, t1, s0
        beq zero, zero, Transpose.Loop
        
    Transpose.After:
        addi t1, tp, 0
        addi t2, s9, 0
        Transpose.After.Loop:
            beq t1, sp, Transpose.Ret
            lw t6, 0(t1)
            sw t6, 0(t2)
            
            sub t1, t1, s0
            add t2, t2, s0
            beq zero, zero, Transpose.After.Loop
    Transpose.Ret:
        addi sp, tp, 0
        lui gp, 0x10000
        ret
            
    
Range:
    #uses sp, a0, a1, a2, t6
    #very general

    #address of initial position in a0
    #stride in a1 (in bytes)
    #limit in a2
    
    Range.Loop:
        beq a2, zero, Range.Ret
        
        lw t6, 0(a0)
        sw t6, 0(sp)
        sub sp, sp, s0
        add a0, a0, a1
        
        addi a2, a2, -1
        beq zero, zero, Range.Loop
    
    Range.Ret:
        jalr x0, gp, 0    
        
        
Dot: #uses s4, s5, a0, a1, t4, t5, t6

    #address of one vector at a0
    #address of other is in a1
    #limit in a2
    #takes dot product, returns value in a0
    Dot.Loop:
        beq a2, zero, Dot.Ret
        lw t6, 0(a1)
        lw t5, 0(a0)
        
        mul t4, t5, t6
        add s5, s5, t4
        
        add a1, a1, s0
        add a0, a0, s0
        
        addi a2, a2, -1
        beq zero, zero, Dot.Loop
    
    Dot.Ret:
        addi a0, s5, 0
        li s5, 0
        jalr zero, gp, 0
    
  Exit:
      la t0, L1
      li t1, 1
      sw t1, 0(t0)  
