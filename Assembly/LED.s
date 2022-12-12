.data
.word 0x100
.word 100
.text

li t0, 0x10012004
sw zero, 0(t0)
li t1, 0x20
sw t1, 4(t0)
addiw t0, t0, 8
lw s0, 4(gp)

Main:
    beq zero, s0, Exit
    sw t1, 0(t0)
    jal ra, Delay
    sw zero, 0(t0)
    jal ra, Delay
    beq zero, zero, Main
    

Delay:
    lw a0, 0(gp)
    Delay.Loop:
        addiw a0, a0, -1
        bnez a0, Delay.Loop
    
    jalr zero, ra, 0
    
Exit:
    Exit.Loop:
        jal x0, Exit.Loop
