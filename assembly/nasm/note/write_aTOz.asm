
  ;裸机上循环输出a-z
SEGMENT writeaToz vstart=0x7c00   
  int9offset equ 9*4
  mov ax,stack                      ;设置栈
  mov ss,ax
  mov sp,10

  mov ax,data
  mov ds,ax

  mov ax,0
  mov es,ax

  ;获取中断例程9入口地址
  push word [es:int9offset]         ;偏移地址址入栈
  pop word [0]
  push word [es:int9offset+2]       ;段地址址入栈 0xf000e987
  pop word [2]                      ;将地址保存入数据区

  mov word [es:int9offset], newInt9
  mov ax,cs
  mov [es:int9offset+2],ax
  jmp start

newInt9:     
  push ax
  push bx
  push es

  in al,0x60
  pushf
  pushf 
  pop bx
  and bh,11111100b
  push bx
  popf

  call dword [ds:0]

  cmp al,1
  jne newInt9_end
  mov ax,0xb800
  mov es,ax
  inc byte [es:2001]
  mov bl,'z'
  mov [es:2002],bl,

newInt9_end:
  pop es
  pop bx
  pop ax
  iret    


start:
  mov ax,0xb800
  mov es,ax
  mov ah,'a'
show_aTOz:
  mov [es:2000],ah
  call for1qw
  inc ah
  cmp ah,'z'
  jna show_aTOz
  mov ah,'a'
  jmp show_aTOz
for1qw:
  push ax
  push dx

  mov dx,0x3000
  mov ax,0
for:
  sub ax,1
  sbb dx,0
  cmp dx,0
  jne for

  pop dx
  pop ax

  ret

stack:  times 100 db 0

data: times 100 db 0
 
times 510 - ($ - $$) db 0
                     db 0x55,0xaa