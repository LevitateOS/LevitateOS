set pagination off
target remote :1234
# Break at sysretq in syscall_entry
break *0xffffffff800100c0
commands
  silent
  if $rcx == 0x100cf
     printf "HIT READ RETURN (0x%x). Executing sysretq.\n", $rcx
     stepi
     printf "RIP after sysretq: 0x%x\n", $pc
     if $pc == 0x100b9
        printf "CRITICAL: RIP CORRUPTED TO 0x100b9!\n"
        quit
     else
        printf "SAFE: Returned to 0x%x\n", $pc
        continue
     end
  end
  if $rcx == 0x100b9
     printf "CRITICAL: RCX CORRUPTED TO 0x100b9 BEFORE Sysretq!\n"
     quit
  end
  continue
end
continue
