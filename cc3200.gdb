target extended-remote :3333

# print demangled symbols
set print asm-demangle on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

# detect unhandled exceptions, hard faults and panics
break DefaultHandler
break HardFault
break rust_begin_unwind

# *try* to stop at the user entry point (it might be gone due to inlining)
break main

# enable semihosting
monitor arm semihosting enable
monitor arm semihosting_fileio enable

# load the program
load

# load the PC and SP manually
# this is required if you've formatted the CC3200 or if the current image is
# bad. If it's the case, the bootloader fails to init and ends up in a loop
# By setting those values manually, we skip the bootloader entirely
set $sp = 0x20040000
set $pc = __RESET_VECTOR

# start the process but halt the processor on main
continue
