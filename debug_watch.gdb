set pagination off
target remote :1234

# Break once to set watchpoint
break syscall_handler
commands
  silent
  if $watch_set != 1
    set $addr = $rdi + 56
    printf "Setting watchpoint on RCX slot at 0x%x\n", $addr
    watch *(unsigned long long*)$addr
    set $watch_set = 1
  end
  continue
end

set $watch_set = 0
continue
