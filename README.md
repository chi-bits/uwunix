# uwunix
fantasy unix computer thing
(i do not know what i am doing)

## instructions
```
; memory
ld
st ; store to ram (is that how things work?)
write ; write to disk
read ; read from disk
swap ; swap regs

; math
add
sub 
mul 
div
inc
dec

; logic
and 
or 
xor 
not 

; thing
fn ; thing im gonna try

; uhm uhh umm
jmp
call

edi ; end instr
edf ; end func

```

## functions
so i think im gonna add functions because i actually dont fucking know i suppose itll make programming easier
so uh heres how youll define one:
```
fn [fn id (word (or long?))] [# of args] [regs ...]
    [code]
    [return reg] or [return from func (no reg)]
```
and like to call functions ya know you go
```
call [function id]
```

## flags
equality,
greater than,
less than,
zero,
parity,
carry,
sign,
overflow,

## opcodes

```
   00  01  02  03  04  05  06  07  08  09  0A  0B  0C  0D  0E  0F
00 edi 
10 edf
20
30
40
50
60
70
80
90
A0
B0
C0
D0
E0
F0
```

