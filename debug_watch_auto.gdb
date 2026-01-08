set pagination off
target remote :1234

# Hook to handle stops automatically
define hook-stop
  # We use $addr to check if watchpoint is active
  if $watch_set == 1
     set $val = *(unsigned long long*)$addr
     printf "Watchpoint/Stop: Val=0x%x PC=0x%x\n", $val, $pc
     if $val == 0x100b9
        printf "CRITICAL CORRUPTION DETECTED BY PC=0x%x !!\n", $pc
        quit
     end
     # Continue automatically
     continue
  end
end

set $watch_set = 0

# Break once to set watchpoint
break syscall_handler
commands
  silent
  if $watch_set != 1
    set $addr = $rdi + 56
    printf "Setting watchpoint on RCX slot at 0x%x\n", $addr
    watch *(unsigned long long*)$addr
    set $watch_set = 1
    # Continue to let hook-stop take over?
    # No, hook-stop runs AFTER stop.
    # But commands run after stop too.
    # If I 'continue' here, hook-stop might run again?
    # hook-stop runs before commands.
    
    # Actually, simpler: define hook-stop to print and continue.
  end
  # We must continue from the breakpoint stop
  continue
end

continue
