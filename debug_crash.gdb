target remote :1234
set pagination off
set language c

# Break at sys_read ENTRY
break *0xffffffff800385b0
command 1
  echo ------------------------------------------\n
  echo [sys_read Entry]\n
  echo [RSP]\n
  p/x $rsp
  echo [Return Address]\n
  p/x *(unsigned long long*)$rsp
  
  # Watch Return Address
  set $ret_addr = (unsigned long long*)$rsp
  watch *$ret_addr
  
  continue
end

continue
quit

continue
quit
