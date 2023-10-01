ld 1, :0x5456
st 1, :0x0001

fn 0x0001 2 1 2 ; piss
	add 1 2 3
	ret
endfn

ld 1, 2
ld 2, 2
call 0x0001 
; r3 = 4

fn 0x0003 
