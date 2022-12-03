; things ill forget

; gdb -q ./program
; disassemble _start

; 0xA new lyne

; syscall result always in rax

%macro sys_write 0
    mov     rax, 1
    mov     rdi, 1
    syscall
%endmacro

%macro sys_read 3 ; buffer <- fd <- count
    sub     rax, rax
    mov     rdi, %2
    mov     rsi, %1
    mov     rdx, %3
    syscall
%endmacro

%macro sys_exit 1
    mov     rax, 60
    mov     rdi, %1
    syscall
%endmacro

%macro sys_open 1
    mov     rax, 2
    mov     rdi, %1 ; filename
    sub     rsi, rsi
    sub     rdx, rdx
    syscall
%endmacro

%macro push_numbered 0
push r8
push r9
push r10
push r11
push r12
push r13
push r14
push r15
%endmacro

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;; END MACROS ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
section .rodata
msg     db      "Give one argument pls", 0xA, 0x0
debug   db      "Debug", 0xA, 0x0
section .data

line1   db      "012345678901234567890123456789012345678901234567890123456789"
line2   db      "012345678901234567890123456789012345678901234567890123456789"
line3   db      "012345678901234567890123456789012345678901234567890123456789"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;; END DATA ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
section .text

global  _start
_start:
sub     r15, r15
pop     rsi
cmp     rsi, 2
jne     cry_and_exit

normal:
pop     rsi ; ignore program name
pop     rsi ; get file name


call    set_len
sys_open    rsi
mov     r9, rax ; r9 is the fd forever
push    r9

sub     r12, r12
mainl: ; outer loop scanning through the lines of the file


mov     r10, line1
call    read_line
mov     rsi, line1
call    set_len

;rdx has length

shr     rdx, 1
sub     r13, r13
xi: ; second loop scanning through the first half of the string

sub     r14, r14
xj: ; inner loop scanning through the second half

movzx   r8, byte[line1 + r13]
movzx   r10, byte[line1 + rdx + r14]
cmp     r8, r10
jne     xjc

cmp     r8, 0x60

jl     upper

lower:
; a - 0x61
sub     r8, 0x60
add     r15, r8
jmp     next_str

upper:
; A - 0x41
sub     r8, 0x40
add     r15, 26
add     r15, r8
jmp     next_str

xjc:
inc     r14
cmp     r14, rdx
jl      xj

inc     r13
jmp      xi

next_str:
inc     r12
cmp     r12, 300
jl     mainl

mov     rsi, debug
call set_len

sub     r9, r9
c:
sys_write
inc     r9
cmp     r9, r15
jl      c

sys_exit 69


global set_len

set_len:
    sub     rdx, rdx

    loop:
    inc     rdx
    cmp     byte[rsi + rdx], 0
    jne     loop

    inc     rdx
    ret

global read_line
read_line:
    sub     rdx, rdx

    blah:
    sys_read r10, r9, 1
    cmp     byte[r10], 0xA
    je      zzz
    inc     r10
    inc     rdx
    jmp     blah

    zzz:
    mov     byte[r10], 0x0
    ret

global cry_and_exit
cry_and_exit:
    mov     rsi, msg
    call    set_len
    sys_write
    sys_exit 0
    ret
