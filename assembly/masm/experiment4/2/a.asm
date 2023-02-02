assume cs:code

code segment 
    mov ax,20h
    mov ds,ax
    mov bx,0
    mov cx,40h
do: mov [bx],bl
    inc bx
    loop do

    mov ax, 4c00h
    int 21h    
code ends
end

