sub sp, sp, #192
mov x0, sp
//svc 0x1
svc 0x1338
add sp, sp, #192
b .
