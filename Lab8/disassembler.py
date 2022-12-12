import numpy as np

# f = open(input('Enters filename.\n'))
f = open('./input.txt')
s = np.loadtxt(f) 
f.close()
f = open('./output.txt', 'w') 
    

# R-format 
def R(s) -> str:
    h = int(s)
    rd = (h & 3968) // 2 ** 7
    f3 = (h & 28672) // 2 ** 12
    r1 = (h & 1015808) // 2 ** 15
    r2 = (h & 32505856) // 2 ** 20
    f7 = (h & 4261412864) // 2 ** 25
    
    # print(bin(h & 32505856))
    
    rv = {0:dict({0:'add', 1:'sll', 2:'slt', 3:'sltu', 4:'xor', 5:'srl', 6:'or', 7:'and'}), 32:dict({0:'sub', 5:'sra'})}
    
    c = rv.get(f7, {-1:'does not exist'}).get(f3, 'does not exist')
    
    # out = ''
    if c == 'does not exist':
        return str(h) + 'does not exist'
    # out = c + ' x' + str(rd) + ', x' + str(r1) + ', x' + str(r2)
    out = f'{c} x{rd}, x{r1}, x{r2}'
    return out
    
# I-format : except loads and jalr
def I(s) -> str:
    h = int(s)
    rd = (h & 3968) // 2 ** 7
    f3 = (h & 28672) // 2 ** 12
    r1 = (h & 1015808) // 2 ** 15
    imm = (h & 4293918720) // 2 ** 20
    f6 = (h & 4227858432) // 2 ** 25
    
    
    c = ''
    rv = {0:'addi', 1:'slli', 4:'xori', 6:'ori', 7:'andi'}
    rv2 = {0:dict({2:'slti', 3:'stliu', 5:'srli'}), 32:dict({5:'srai'})}
    
    c = rv.get(f3, 'does not exist')
    if c == 'does not exist':
        c = rv2.get(f6, {-1:'does not exist'}).get(f3, 'does not exist')
        imm = imm % 2 ** 6
        
    if c == 'does not exist':
        return str(h) + f' {f6} {f3} does not exist'

    if imm >= 2 ** 11:
        imm -= 2 ** 12
    out = f'{c} x{rd}, x{r1}, {imm}'
    return out
    
# Loads 
def L(s) -> str:
    h = int(s)
    rd = (h & 3968) // 2 ** 7
    f3 = (h & 28672) // 2 ** 12
    r1 = (h & 1015808) // 2 ** 15
    imm = (h & 4293918720) // 2 ** 20 
    # sign check
    if f3 < 4 and imm >= 2 ** 11:
        imm = 2 ** 11 - imm
    c = ''
    rv = {0:'lb', 1:'lh', 2:'lw', 3:'ld', 4:'lbu', 5:'lhu', 6:'lwu', 7:'ldu'}
    c = rv.get(f3, 'does not exist')
    
    if c == 'does not exist':
        return str(h) + 'does not exist'
    
    # out = c + ' x' + str(rd) + ', ' + str(imm) + '(x' + str(r1) + ')'
    out = f'{c} x{rd}, {imm}(x{r1})'
    return out
    
# J-format : jal
def J(s) -> str:
    h = int(s)
    rd = (h & 3968) // 2 ** 7
    x = bin(h)[2:]
    x = '0' * (32 - len(x)) + x
    x = x[::-1]
    # imm = 0
    # imm += (h & 1044480)
    # imm += (h & 2145386496) // 2 ** 20
    # imm += (h & 2 ** 20) // 2 ** 9
    # imm += (h & 2 ** 31) // 2 ** 11
    
    
    imm = int('0b' + ('0' + x[21:31][::-1] + x[20] + x[12:20][::-1] + x[31])[::-1], 2)
    if imm > 2 ** 20:
        imm -= 2 ** 21
        
    out = f'jal x{rd}, {imm}'
    return out
     
# jalr
def jalr(s) -> str:
    h = int(s)
    rd = (h & 3968) // 2 ** 7
    # f3 = (h & 28672) // 2 ** 12
    r1 = (h & 1015808) // 2 ** 15
    imm = (h & 4293918720) // 2 ** 20
    if imm >= 2 ** 11:
        imm = imm - 2 ** 12
    out = f'jalr x{rd}, x{r1}, {imm}'
    return out

# S-format
def S(s) -> str:
    h = int(s)
    imm = (h & 3968) // 2 ** 7
    f3 = (h & 28672) // 2 ** 12
    r1 = (h & 1015808) // 2 ** 15
    r2 = (h & 32505856) // 2 ** 20
    imm += (h & 4261412864) // 2 ** 20
    
    if imm >= 2 ** 11:
        imm -= 2 ** 12
    
    rv = {0:'sb', 1:'sh', 2:'sw', 3:'sd'}
    c = rv.get(f3, 'does not exist')
    
    if c == 'does not exist':
        return str(h) + 'does not exist'
    
    out = f'{c} x{r1}, {imm}(x{r2})'
    return out

# B-format
def B(s) -> str:
    h = int(s)
    x = bin(h)[2:]
    x = '0' * (32 - len(x)) + x
    x = x[::-1]
    f3 = (h & 28672) // 2 ** 12
    r1 = (h & 1015808) // 2 ** 15
    r2 = (h & 32505856) // 2 ** 20
    
    imm = int('0b' + ('0' + x[8:12] + x[25:31] + x[7] + x[31])[::-1], 2)
    if imm >= 2 ** 12:
        imm -= 2 ** 13
    
    rv = {0:'beq', 1:'bne', 4:'blt', 5:'bge', 6:'bltu', 7:'bgeu'}
    c = rv.get(f3, 'does not exist')
    
    if c == 'does not exist':
        return str(h) + 'does not exist'
    
    out = f'{c} x{r1}, x{r2}, {imm}'
    return out

# U-format : lui
def lui(s) -> str:
    h = int(s)
    imm = (h & 4294963200) // 2 ** 12
    rd = (h & 3968) // 2 ** 7
    
    if imm > 2 ** 20:
        imm = 2 ** 2 - imm
    out = f'lui x{rd}, {imm}'
    return out

# U-format : auipc
def auipc(s) -> str:
    h = int(s)
    imm = (h & 4294963200) // 2 ** 12
    rd = (h & 3968) // 2 ** 7
    
    if imm > 2 ** 20:
        imm = 2 ** 2 - imm
    out = f'auipc x{rd}, {imm}'
    return out

# Invalid opcode
def dummy(s):
    h = int(s)
    return f'Invalid opcode: {h & 127}'
         
ops = [] # contains opcodes of each instruction 

# weird loadtxt format. If only one instruction, you just get a number, not a list of length 1. Converts it.
if s.ndim == 0:
    s = [s]

# opcode evaluation
ops = [int(k) & 127 for k in s]

# dict of valid opcodes
codes = {51:R, 19:I, 3:L, 103:jalr, 35:S, 99:B, 111:J, 55:lui, 23:auipc}
temp = [''] # will contain assembly code, without labels

# Calls disassembling functions, appends assembly code obtained to temp 
for i in range(len(s)):
    x = codes.get(ops[i], dummy)
    temp += [x(s[i])]
    
# actual output, with labels
out = [''] * (len(temp))
# branch count, used in label name
count = '0'
temp.remove('')

# adds labels while copying temp to out
for n in range(len(temp)):
    out[n] += temp[n]
    if ops[n] in [103, 99]:
        off = (int(temp[n][temp[n].rfind(' '):])) // 4
        out[n + off] = 'L' + count + ': ' + out[n + off]
        out[n] = temp[n][:temp[n].rfind(' ') + 1] + 'L' + count
        count = str(1 + int(count))
        
# prints output. 
for i in out:
    print(i)
    f.write(i)