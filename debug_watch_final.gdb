set pagination off
target remote :1234
define hook-stop
  if $watch_set == 1
     if $pc == 0xffffffff80010093
         continue
     end
     set $val = *(unsigned long long*)$addr
     # printf "WRITE DETECTED! Val=0x%x PC=0x%x\n", $val, $pc
     if $val == 0x100b9
        printf "CRITICAL CORRUPTION FOUND! Val=0x%x PC=0x%x\n", $val, $pc
        quit
     end
     continue
  end
end

set $watch_set = 0
break syscall_handler
commands
  silent
  if $watch_set != 1
    # offset 56 relative to RDI (which is RSP after pushes)
    set $addr = $rdi + 56
    watch *(unsigned long long*)$addr
    set $watch_set = 1
  end
  continue
end
continue
