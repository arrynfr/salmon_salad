.text
.align 2
.global _start
_start:
sub sp, sp, #192
mov x0, sp
//svc 0x1
svc 0x1338
svc 0x1339
svc 0x1331
svc 0x1332
svc 0x1333
svc 0x1333
svc 0x1333
add sp, sp, #192
b .
