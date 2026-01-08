
userspace/target/x86_64-unknown-none/release/shell:     file format elf64-x86-64


Disassembly of section .text:

0000000000010000 <_start>:
   10000:	48 31 ed             	xor    %rbp,%rbp
   10003:	48 89 e7             	mov    %rsp,%rdi
   10006:	48 83 e4 f0          	and    $0xfffffffffffffff0,%rsp
   1000a:	e8 01 01 00 00       	call   10110 <shell_entry>
   1000f:	0f 0b                	ud2

0000000000010011 <_RNvCs3uGVbeP0EXQ_5shell8bytes_eq>:
   10011:	48 39 ce             	cmp    %rcx,%rsi
   10014:	75 19                	jne    1002f <_RNvCs3uGVbeP0EXQ_5shell8bytes_eq+0x1e>
   10016:	31 c9                	xor    %ecx,%ecx
   10018:	48 39 ce             	cmp    %rcx,%rsi
   1001b:	0f 94 c0             	sete   %al
   1001e:	74 0e                	je     1002e <_RNvCs3uGVbeP0EXQ_5shell8bytes_eq+0x1d>
   10020:	44 8a 04 0f          	mov    (%rdi,%rcx,1),%r8b
   10024:	44 3a 04 0a          	cmp    (%rdx,%rcx,1),%r8b
   10028:	48 8d 49 01          	lea    0x1(%rcx),%rcx
   1002c:	74 ea                	je     10018 <_RNvCs3uGVbeP0EXQ_5shell8bytes_eq+0x7>
   1002e:	c3                   	ret
   1002f:	31 c0                	xor    %eax,%eax
   10031:	c3                   	ret

0000000000010032 <_RNvCs5kVVpB1dGJU_7___rustc17rust_begin_unwind>:
   10032:	50                   	push   %rax
   10033:	e8 33 08 00 00       	call   1086b <_RNvCs1C5MKr7Wveb_10libsyscall20common_panic_handler>

0000000000010038 <_RNvNtCs1C5MKr7Wveb_10libsyscall2io5write>:
   10038:	48 89 f2             	mov    %rsi,%rdx
   1003b:	48 89 fe             	mov    %rdi,%rsi
   1003e:	6a 01                	push   $0x1
   10040:	5f                   	pop    %rdi
   10041:	6a 18                	push   $0x18
   10043:	41 58                	pop    %r8
   10045:	48 89 f8             	mov    %rdi,%rax
   10048:	0f 05                	syscall
   1004a:	48 83 f8 f5          	cmp    $0xfffffffffffffff5,%rax
   1004e:	75 07                	jne    10057 <_RNvNtCs1C5MKr7Wveb_10libsyscall2io5write+0x1f>
   10050:	4c 89 c0             	mov    %r8,%rax
   10053:	0f 05                	syscall
   10055:	eb ee                	jmp    10045 <_RNvNtCs1C5MKr7Wveb_10libsyscall2io5write+0xd>
   10057:	c3                   	ret

0000000000010058 <_RNvNvCs3uGVbeP0EXQ_5shell11shell_entry14sigint_handler>:
   10058:	c3                   	ret

0000000000010059 <_RNvNvCs3uGVbeP0EXQ_5shell11shell_entry20sigreturn_trampoline>:
   10059:	6a 0f                	push   $0xf
   1005b:	58                   	pop    %rax
   1005c:	0f 05                	syscall
   1005e:	0f 0b                	ud2

0000000000010060 <_RNvXs1i_NtCsfJBMPiLOdLr_4core3fmtReNtB6_7Display3fmtCs3uGVbeP0EXQ_5shell>:
   10060:	48 89 f2             	mov    %rsi,%rdx
   10063:	48 8b 07             	mov    (%rdi),%rax
   10066:	48 8b 77 08          	mov    0x8(%rdi),%rsi
   1006a:	48 89 c7             	mov    %rax,%rdi
   1006d:	e9 ce 19 00 00       	jmp    11a40 <_RNvXsi_NtCsfJBMPiLOdLr_4core3fmteNtB5_7Display3fmt>

0000000000010072 <_RNvYNtCs1C5MKr7Wveb_10libsyscall6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write10write_charCs3uGVbeP0EXQ_5shell>:
   10072:	50                   	push   %rax
   10073:	83 64 24 04 00       	andl   $0x0,0x4(%rsp)
   10078:	81 fe 80 00 00 00    	cmp    $0x80,%esi
   1007e:	73 09                	jae    10089 <_RNvYNtCs1C5MKr7Wveb_10libsyscall6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write10write_charCs3uGVbeP0EXQ_5shell+0x17>
   10080:	40 88 74 24 04       	mov    %sil,0x4(%rsp)
   10085:	6a 01                	push   $0x1
   10087:	eb 68                	jmp    100f1 <_RNvYNtCs1C5MKr7Wveb_10libsyscall6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write10write_charCs3uGVbeP0EXQ_5shell+0x7f>
   10089:	89 f0                	mov    %esi,%eax
   1008b:	24 3f                	and    $0x3f,%al
   1008d:	0c 80                	or     $0x80,%al
   1008f:	89 f1                	mov    %esi,%ecx
   10091:	c1 e9 06             	shr    $0x6,%ecx
   10094:	81 fe 00 08 00 00    	cmp    $0x800,%esi
   1009a:	73 0f                	jae    100ab <_RNvYNtCs1C5MKr7Wveb_10libsyscall6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write10write_charCs3uGVbeP0EXQ_5shell+0x39>
   1009c:	80 c9 c0             	or     $0xc0,%cl
   1009f:	88 4c 24 04          	mov    %cl,0x4(%rsp)
   100a3:	88 44 24 05          	mov    %al,0x5(%rsp)
   100a7:	6a 02                	push   $0x2
   100a9:	eb 46                	jmp    100f1 <_RNvYNtCs1C5MKr7Wveb_10libsyscall6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write10write_charCs3uGVbeP0EXQ_5shell+0x7f>
   100ab:	80 e1 3f             	and    $0x3f,%cl
   100ae:	80 c9 80             	or     $0x80,%cl
   100b1:	89 f2                	mov    %esi,%edx
   100b3:	c1 ea 0c             	shr    $0xc,%edx
   100b6:	81 fe ff ff 00 00    	cmp    $0xffff,%esi
   100bc:	77 13                	ja     100d1 <_RNvYNtCs1C5MKr7Wveb_10libsyscall6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write10write_charCs3uGVbeP0EXQ_5shell+0x5f>
   100be:	80 ca e0             	or     $0xe0,%dl
   100c1:	88 54 24 04          	mov    %dl,0x4(%rsp)
   100c5:	88 4c 24 05          	mov    %cl,0x5(%rsp)
   100c9:	88 44 24 06          	mov    %al,0x6(%rsp)
   100cd:	6a 03                	push   $0x3
   100cf:	eb 20                	jmp    100f1 <_RNvYNtCs1C5MKr7Wveb_10libsyscall6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write10write_charCs3uGVbeP0EXQ_5shell+0x7f>
   100d1:	80 e2 3f             	and    $0x3f,%dl
   100d4:	80 ca 80             	or     $0x80,%dl
   100d7:	c1 ee 12             	shr    $0x12,%esi
   100da:	40 80 ce f0          	or     $0xf0,%sil
   100de:	40 88 74 24 04       	mov    %sil,0x4(%rsp)
   100e3:	88 54 24 05          	mov    %dl,0x5(%rsp)
   100e7:	88 4c 24 06          	mov    %cl,0x6(%rsp)
   100eb:	88 44 24 07          	mov    %al,0x7(%rsp)
   100ef:	6a 04                	push   $0x4
   100f1:	5a                   	pop    %rdx
   100f2:	48 8d 74 24 04       	lea    0x4(%rsp),%rsi
   100f7:	e8 a7 07 00 00       	call   108a3 <_RNvXCs1C5MKr7Wveb_10libsyscallNtB2_6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_str>
   100fc:	59                   	pop    %rcx
   100fd:	c3                   	ret

00000000000100fe <_RNvYNtCs1C5MKr7Wveb_10libsyscall6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_fmtCs3uGVbeP0EXQ_5shell>:
   100fe:	48 89 d1             	mov    %rdx,%rcx
   10101:	48 89 f2             	mov    %rsi,%rdx
   10104:	48 c7 c6 e0 1d 01 00 	mov    $0x11de0,%rsi
   1010b:	e9 40 0f 00 00       	jmp    11050 <_RNvNtCsfJBMPiLOdLr_4core3fmt5write>

0000000000010110 <shell_entry>:
   10110:	55                   	push   %rbp
   10111:	41 57                	push   %r15
   10113:	41 56                	push   %r14
   10115:	41 55                	push   %r13
   10117:	41 54                	push   %r12
   10119:	53                   	push   %rbx
   1011a:	48 81 ec 38 04 00 00 	sub    $0x438,%rsp
   10121:	4c 8d 64 24 06       	lea    0x6(%rsp),%r12
   10126:	6a 01                	push   $0x1
   10128:	41 5e                	pop    %r14
   1012a:	4c 89 e7             	mov    %r12,%rdi
   1012d:	48 c7 c6 81 1c 01 00 	mov    $0x11c81,%rsi
   10134:	4c 89 f2             	mov    %r14,%rdx
   10137:	e8 67 07 00 00       	call   108a3 <_RNvXCs1C5MKr7Wveb_10libsyscallNtB2_6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_str>
   1013c:	6a 1c                	push   $0x1c
   1013e:	5a                   	pop    %rdx
   1013f:	4c 89 e7             	mov    %r12,%rdi
   10142:	48 c7 c6 89 1c 01 00 	mov    $0x11c89,%rsi
   10149:	e8 55 07 00 00       	call   108a3 <_RNvXCs1C5MKr7Wveb_10libsyscallNtB2_6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_str>
   1014e:	6a 1a                	push   $0x1a
   10150:	5a                   	pop    %rdx
   10151:	4c 89 e7             	mov    %r12,%rdi
   10154:	48 c7 c6 10 1e 01 00 	mov    $0x11e10,%rsi
   1015b:	e8 43 07 00 00       	call   108a3 <_RNvXCs1C5MKr7Wveb_10libsyscallNtB2_6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_str>
   10160:	4c 89 e7             	mov    %r12,%rdi
   10163:	48 c7 c6 81 1c 01 00 	mov    $0x11c81,%rsi
   1016a:	4c 89 f2             	mov    %r14,%rdx
   1016d:	e8 31 07 00 00       	call   108a3 <_RNvXCs1C5MKr7Wveb_10libsyscallNtB2_6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_str>
   10172:	6a 0d                	push   $0xd
   10174:	58                   	pop    %rax
   10175:	6a 02                	push   $0x2
   10177:	5f                   	pop    %rdi
   10178:	48 c7 c6 58 00 01 00 	mov    $0x10058,%rsi
   1017f:	48 c7 c2 59 00 01 00 	mov    $0x10059,%rdx
   10186:	0f 05                	syscall
   10188:	6a 27                	push   $0x27
   1018a:	58                   	pop    %rax
   1018b:	0f 05                	syscall
   1018d:	48 89 c7             	mov    %rax,%rdi
   10190:	b8 ea 03 00 00       	mov    $0x3ea,%eax
   10195:	0f 05                	syscall
   10197:	6a 40                	push   $0x40
   10199:	59                   	pop    %rcx
   1019a:	48 8d bc 24 38 03 00 	lea    0x338(%rsp),%rdi
   101a1:	00 
   101a2:	31 c0                	xor    %eax,%eax
   101a4:	f3 ab                	rep stos %eax,(%rdi)
   101a6:	48 8d 6c 24 07       	lea    0x7(%rsp),%rbp
   101ab:	6a 18                	push   $0x18
   101ad:	41 5d                	pop    %r13
   101af:	49 bf 00 26 00 00 01 	movabs $0x100002600,%r15
   101b6:	00 00 00 
   101b9:	4c 89 e7             	mov    %r12,%rdi
   101bc:	48 c7 c6 2a 1e 01 00 	mov    $0x11e2a,%rsi
   101c3:	6a 02                	push   $0x2
   101c5:	5a                   	pop    %rdx
   101c6:	e8 d8 06 00 00       	call   108a3 <_RNvXCs1C5MKr7Wveb_10libsyscallNtB2_6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_str>
   101cb:	31 db                	xor    %ebx,%ebx
   101cd:	c6 44 24 07 00       	movb   $0x0,0x7(%rsp)
   101d2:	31 c0                	xor    %eax,%eax
   101d4:	31 ff                	xor    %edi,%edi
   101d6:	48 89 ee             	mov    %rbp,%rsi
   101d9:	4c 89 f2             	mov    %r14,%rdx
   101dc:	0f 05                	syscall
   101de:	48 83 f8 f5          	cmp    $0xfffffffffffffff5,%rax
   101e2:	75 07                	jne    101eb <shell_entry+0xdb>
   101e4:	4c 89 e8             	mov    %r13,%rax
   101e7:	0f 05                	syscall
   101e9:	eb e7                	jmp    101d2 <shell_entry+0xc2>
   101eb:	48 85 c0             	test   %rax,%rax
   101ee:	7e dd                	jle    101cd <shell_entry+0xbd>
   101f0:	0f b6 44 24 07       	movzbl 0x7(%rsp),%eax
   101f5:	83 f8 08             	cmp    $0x8,%eax
   101f8:	74 2f                	je     10229 <shell_entry+0x119>
   101fa:	83 f8 7f             	cmp    $0x7f,%eax
   101fd:	74 2a                	je     10229 <shell_entry+0x119>
   101ff:	83 f8 0d             	cmp    $0xd,%eax
   10202:	74 3e                	je     10242 <shell_entry+0x132>
   10204:	83 f8 0a             	cmp    $0xa,%eax
   10207:	74 39                	je     10242 <shell_entry+0x132>
   10209:	48 81 fb 00 01 00 00 	cmp    $0x100,%rbx
   10210:	73 bb                	jae    101cd <shell_entry+0xbd>
   10212:	88 84 1c 38 03 00 00 	mov    %al,0x338(%rsp,%rbx,1)
   10219:	48 ff c3             	inc    %rbx
   1021c:	48 89 ef             	mov    %rbp,%rdi
   1021f:	4c 89 f6             	mov    %r14,%rsi
   10222:	e8 11 fe ff ff       	call   10038 <_RNvNtCs1C5MKr7Wveb_10libsyscall2io5write>
   10227:	eb a4                	jmp    101cd <shell_entry+0xbd>
   10229:	48 85 db             	test   %rbx,%rbx
   1022c:	74 9d                	je     101cb <shell_entry+0xbb>
   1022e:	48 ff cb             	dec    %rbx
   10231:	48 c7 c7 48 1e 01 00 	mov    $0x11e48,%rdi
   10238:	6a 03                	push   $0x3
   1023a:	5e                   	pop    %rsi
   1023b:	e8 f8 fd ff ff       	call   10038 <_RNvNtCs1C5MKr7Wveb_10libsyscall2io5write>
   10240:	eb 8b                	jmp    101cd <shell_entry+0xbd>
   10242:	48 c7 c7 81 1c 01 00 	mov    $0x11c81,%rdi
   10249:	4c 89 f6             	mov    %r14,%rsi
   1024c:	e8 e7 fd ff ff       	call   10038 <_RNvNtCs1C5MKr7Wveb_10libsyscall2io5write>
   10251:	48 85 db             	test   %rbx,%rbx
   10254:	0f 84 5f ff ff ff    	je     101b9 <shell_entry+0xa9>
   1025a:	48 81 fb 01 01 00 00 	cmp    $0x101,%rbx
   10261:	0f 83 6d 05 00 00    	jae    107d4 <shell_entry+0x6c4>
   10267:	31 ff                	xor    %edi,%edi
   10269:	48 39 fb             	cmp    %rdi,%rbx
   1026c:	74 19                	je     10287 <shell_entry+0x177>
   1026e:	0f b6 84 3c 38 03 00 	movzbl 0x338(%rsp,%rdi,1),%eax
   10275:	00 
   10276:	48 83 f8 20          	cmp    $0x20,%rax
   1027a:	77 0e                	ja     1028a <shell_entry+0x17a>
   1027c:	49 0f a3 c7          	bt     %rax,%r15
   10280:	73 08                	jae    1028a <shell_entry+0x17a>
   10282:	48 ff c7             	inc    %rdi
   10285:	eb e2                	jmp    10269 <shell_entry+0x159>
   10287:	48 89 df             	mov    %rbx,%rdi
   1028a:	48 39 df             	cmp    %rbx,%rdi
   1028d:	48 89 d9             	mov    %rbx,%rcx
   10290:	48 0f 42 cf          	cmovb  %rdi,%rcx
   10294:	48 8d 43 ff          	lea    -0x1(%rbx),%rax
   10298:	48 8d 70 01          	lea    0x1(%rax),%rsi
   1029c:	48 39 fe             	cmp    %rdi,%rsi
   1029f:	76 22                	jbe    102c3 <shell_entry+0x1b3>
   102a1:	48 39 d8             	cmp    %rbx,%rax
   102a4:	0f 83 4f 05 00 00    	jae    107f9 <shell_entry+0x6e9>
   102aa:	0f b6 94 04 38 03 00 	movzbl 0x338(%rsp,%rax,1),%edx
   102b1:	00 
   102b2:	48 83 fa 20          	cmp    $0x20,%rdx
   102b6:	77 0e                	ja     102c6 <shell_entry+0x1b6>
   102b8:	49 0f a3 d7          	bt     %rdx,%r15
   102bc:	73 08                	jae    102c6 <shell_entry+0x1b6>
   102be:	48 ff c8             	dec    %rax
   102c1:	eb d5                	jmp    10298 <shell_entry+0x188>
   102c3:	48 89 ce             	mov    %rcx,%rsi
   102c6:	49 89 f4             	mov    %rsi,%r12
   102c9:	49 29 fc             	sub    %rdi,%r12
   102cc:	0f 82 18 05 00 00    	jb     107ea <shell_entry+0x6da>
   102d2:	0f 84 3d 01 00 00    	je     10415 <shell_entry+0x305>
   102d8:	48 8d 1c 3c          	lea    (%rsp,%rdi,1),%rbx
   102dc:	48 81 c3 38 03 00 00 	add    $0x338,%rbx
   102e3:	48 89 df             	mov    %rbx,%rdi
   102e6:	4c 89 e6             	mov    %r12,%rsi
   102e9:	48 c7 c2 5c 1c 01 00 	mov    $0x11c5c,%rdx
   102f0:	6a 04                	push   $0x4
   102f2:	59                   	pop    %rcx
   102f3:	e8 19 fd ff ff       	call   10011 <_RNvCs3uGVbeP0EXQ_5shell8bytes_eq>
   102f8:	84 c0                	test   %al,%al
   102fa:	0f 85 2c 05 00 00    	jne    1082c <shell_entry+0x71c>
   10300:	48 89 df             	mov    %rbx,%rdi
   10303:	4c 89 e6             	mov    %r12,%rsi
   10306:	48 c7 c2 60 1c 01 00 	mov    $0x11c60,%rdx
   1030d:	6a 0e                	push   $0xe
   1030f:	59                   	pop    %rcx
   10310:	e8 fc fc ff ff       	call   10011 <_RNvCs3uGVbeP0EXQ_5shell8bytes_eq>
   10315:	84 c0                	test   %al,%al
   10317:	0f 85 2e 05 00 00    	jne    1084b <shell_entry+0x73b>
   1031d:	48 89 df             	mov    %rbx,%rdi
   10320:	4c 89 e6             	mov    %r12,%rsi
   10323:	48 c7 c2 50 1c 01 00 	mov    $0x11c50,%rdx
   1032a:	6a 04                	push   $0x4
   1032c:	59                   	pop    %rcx
   1032d:	e8 df fc ff ff       	call   10011 <_RNvCs3uGVbeP0EXQ_5shell8bytes_eq>
   10332:	84 c0                	test   %al,%al
   10334:	0f 85 db 00 00 00    	jne    10415 <shell_entry+0x305>
   1033a:	48 89 df             	mov    %rbx,%rdi
   1033d:	4c 89 e6             	mov    %r12,%rsi
   10340:	48 c7 c2 54 1c 01 00 	mov    $0x11c54,%rdx
   10347:	6a 04                	push   $0x4
   10349:	59                   	pop    %rcx
   1034a:	e8 c2 fc ff ff       	call   10011 <_RNvCs3uGVbeP0EXQ_5shell8bytes_eq>
   1034f:	84 c0                	test   %al,%al
   10351:	0f 84 91 00 00 00    	je     103e8 <shell_entry+0x2d8>
   10357:	48 8d 5c 24 06       	lea    0x6(%rsp),%rbx
   1035c:	48 89 df             	mov    %rbx,%rdi
   1035f:	48 c7 c6 89 1c 01 00 	mov    $0x11c89,%rsi
   10366:	6a 1c                	push   $0x1c
   10368:	5a                   	pop    %rdx
   10369:	e8 35 05 00 00       	call   108a3 <_RNvXCs1C5MKr7Wveb_10libsyscallNtB2_6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_str>
   1036e:	48 89 df             	mov    %rbx,%rdi
   10371:	48 c7 c6 a5 1c 01 00 	mov    $0x11ca5,%rsi
   10378:	6a 0a                	push   $0xa
   1037a:	5a                   	pop    %rdx
   1037b:	e8 23 05 00 00       	call   108a3 <_RNvXCs1C5MKr7Wveb_10libsyscallNtB2_6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_str>
   10380:	48 89 df             	mov    %rbx,%rdi
   10383:	48 c7 c6 af 1c 01 00 	mov    $0x11caf,%rsi
   1038a:	6a 1e                	push   $0x1e
   1038c:	5a                   	pop    %rdx
   1038d:	e8 11 05 00 00       	call   108a3 <_RNvXCs1C5MKr7Wveb_10libsyscallNtB2_6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_str>
   10392:	48 89 df             	mov    %rbx,%rdi
   10395:	48 c7 c6 cd 1c 01 00 	mov    $0x11ccd,%rsi
   1039c:	6a 22                	push   $0x22
   1039e:	5a                   	pop    %rdx
   1039f:	e8 ff 04 00 00       	call   108a3 <_RNvXCs1C5MKr7Wveb_10libsyscallNtB2_6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_str>
   103a4:	48 89 df             	mov    %rbx,%rdi
   103a7:	48 c7 c6 ef 1c 01 00 	mov    $0x11cef,%rsi
   103ae:	6a 20                	push   $0x20
   103b0:	5a                   	pop    %rdx
   103b1:	e8 ed 04 00 00       	call   108a3 <_RNvXCs1C5MKr7Wveb_10libsyscallNtB2_6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_str>
   103b6:	48 89 df             	mov    %rbx,%rdi
   103b9:	48 c7 c6 0f 1d 01 00 	mov    $0x11d0f,%rsi
   103c0:	6a 23                	push   $0x23
   103c2:	5a                   	pop    %rdx
   103c3:	e8 db 04 00 00       	call   108a3 <_RNvXCs1C5MKr7Wveb_10libsyscallNtB2_6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_str>
   103c8:	48 89 df             	mov    %rbx,%rdi
   103cb:	48 c7 c6 32 1d 01 00 	mov    $0x11d32,%rsi
   103d2:	6a 31                	push   $0x31
   103d4:	5a                   	pop    %rdx
   103d5:	e8 c9 04 00 00       	call   108a3 <_RNvXCs1C5MKr7Wveb_10libsyscallNtB2_6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_str>
   103da:	48 89 df             	mov    %rbx,%rdi
   103dd:	48 c7 c6 63 1d 01 00 	mov    $0x11d63,%rsi
   103e4:	6a 2a                	push   $0x2a
   103e6:	eb 27                	jmp    1040f <shell_entry+0x2ff>
   103e8:	48 89 df             	mov    %rbx,%rdi
   103eb:	4c 89 e6             	mov    %r12,%rsi
   103ee:	48 c7 c2 6e 1c 01 00 	mov    $0x11c6e,%rdx
   103f5:	6a 05                	push   $0x5
   103f7:	59                   	pop    %rcx
   103f8:	e8 14 fc ff ff       	call   10011 <_RNvCs3uGVbeP0EXQ_5shell8bytes_eq>
   103fd:	84 c0                	test   %al,%al
   103ff:	74 1e                	je     1041f <shell_entry+0x30f>
   10401:	48 8d 7c 24 06       	lea    0x6(%rsp),%rdi
   10406:	48 c7 c6 82 1c 01 00 	mov    $0x11c82,%rsi
   1040d:	6a 07                	push   $0x7
   1040f:	5a                   	pop    %rdx
   10410:	e8 8e 04 00 00       	call   108a3 <_RNvXCs1C5MKr7Wveb_10libsyscallNtB2_6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_str>
   10415:	4c 8d 64 24 06       	lea    0x6(%rsp),%r12
   1041a:	e9 9a fd ff ff       	jmp    101b9 <shell_entry+0xa9>
   1041f:	49 83 fc 05          	cmp    $0x5,%r12
   10423:	0f 82 87 00 00 00    	jb     104b0 <shell_entry+0x3a0>
   10429:	48 89 df             	mov    %rbx,%rdi
   1042c:	6a 05                	push   $0x5
   1042e:	59                   	pop    %rcx
   1042f:	48 89 ce             	mov    %rcx,%rsi
   10432:	48 c7 c2 73 1c 01 00 	mov    $0x11c73,%rdx
   10439:	e8 d3 fb ff ff       	call   10011 <_RNvCs3uGVbeP0EXQ_5shell8bytes_eq>
   1043e:	84 c0                	test   %al,%al
   10440:	74 6e                	je     104b0 <shell_entry+0x3a0>
   10442:	49 83 c4 fb          	add    $0xfffffffffffffffb,%r12
   10446:	48 83 c3 05          	add    $0x5,%rbx
   1044a:	48 8d 7c 24 38       	lea    0x38(%rsp),%rdi
   1044f:	48 89 de             	mov    %rbx,%rsi
   10452:	4c 89 e2             	mov    %r12,%rdx
   10455:	e8 e6 11 00 00       	call   11640 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8>
   1045a:	80 7c 24 38 00       	cmpb   $0x0,0x38(%rsp)
   1045f:	75 b4                	jne    10415 <shell_entry+0x305>
   10461:	48 8b 44 24 40       	mov    0x40(%rsp),%rax
   10466:	48 8b 4c 24 48       	mov    0x48(%rsp),%rcx
   1046b:	48 89 84 24 38 02 00 	mov    %rax,0x238(%rsp)
   10472:	00 
   10473:	48 89 8c 24 40 02 00 	mov    %rcx,0x240(%rsp)
   1047a:	00 
   1047b:	48 8d 84 24 38 02 00 	lea    0x238(%rsp),%rax
   10482:	00 
   10483:	48 89 84 24 38 01 00 	mov    %rax,0x138(%rsp)
   1048a:	00 
   1048b:	48 c7 84 24 40 01 00 	movq   $0x10060,0x140(%rsp)
   10492:	00 60 00 01 00 
   10497:	48 8d 7c 24 06       	lea    0x6(%rsp),%rdi
   1049c:	48 c7 c6 90 1a 01 00 	mov    $0x11a90,%rsi
   104a3:	48 8d 94 24 38 01 00 	lea    0x138(%rsp),%rdx
   104aa:	00 
   104ab:	e9 1a 03 00 00       	jmp    107ca <shell_entry+0x6ba>
   104b0:	48 89 df             	mov    %rbx,%rdi
   104b3:	4c 89 e6             	mov    %r12,%rsi
   104b6:	48 c7 c2 58 1c 01 00 	mov    $0x11c58,%rdx
   104bd:	6a 04                	push   $0x4
   104bf:	59                   	pop    %rcx
   104c0:	e8 4c fb ff ff       	call   10011 <_RNvCs3uGVbeP0EXQ_5shell8bytes_eq>
   104c5:	84 c0                	test   %al,%al
   104c7:	74 14                	je     104dd <shell_entry+0x3cd>
   104c9:	48 8d 7c 24 06       	lea    0x6(%rsp),%rdi
   104ce:	48 c7 c6 81 1c 01 00 	mov    $0x11c81,%rsi
   104d5:	4c 89 f2             	mov    %r14,%rdx
   104d8:	e9 33 ff ff ff       	jmp    10410 <shell_entry+0x300>
   104dd:	48 83 4c 24 10 ff    	orq    $0xffffffffffffffff,0x10(%rsp)
   104e3:	31 c0                	xor    %eax,%eax
   104e5:	48 3d 00 01 00 00    	cmp    $0x100,%rax
   104eb:	74 15                	je     10502 <shell_entry+0x3f2>
   104ed:	48 c7 44 04 38 01 00 	movq   $0x1,0x38(%rsp,%rax,1)
   104f4:	00 00 
   104f6:	48 83 64 04 40 00    	andq   $0x0,0x40(%rsp,%rax,1)
   104fc:	48 83 c0 10          	add    $0x10,%rax
   10500:	eb e3                	jmp    104e5 <shell_entry+0x3d5>
   10502:	31 ff                	xor    %edi,%edi
   10504:	48 c7 44 24 08 00 00 	movq   $0x0,0x8(%rsp)
   1050b:	00 00 
   1050d:	4c 39 e7             	cmp    %r12,%rdi
   10510:	73 71                	jae    10583 <shell_entry+0x473>
   10512:	48 83 7c 24 08 0f    	cmpq   $0xf,0x8(%rsp)
   10518:	77 69                	ja     10583 <shell_entry+0x473>
   1051a:	49 39 fc             	cmp    %rdi,%r12
   1051d:	74 64                	je     10583 <shell_entry+0x473>
   1051f:	0f b6 04 3b          	movzbl (%rbx,%rdi,1),%eax
   10523:	83 f8 09             	cmp    $0x9,%eax
   10526:	74 05                	je     1052d <shell_entry+0x41d>
   10528:	83 f8 20             	cmp    $0x20,%eax
   1052b:	75 05                	jne    10532 <shell_entry+0x422>
   1052d:	48 ff c7             	inc    %rdi
   10530:	eb e8                	jmp    1051a <shell_entry+0x40a>
   10532:	48 8d 04 3b          	lea    (%rbx,%rdi,1),%rax
   10536:	48 89 fe             	mov    %rdi,%rsi
   10539:	4c 39 e6             	cmp    %r12,%rsi
   1053c:	73 13                	jae    10551 <shell_entry+0x441>
   1053e:	0f b6 0c 33          	movzbl (%rbx,%rsi,1),%ecx
   10542:	83 f9 09             	cmp    $0x9,%ecx
   10545:	74 0d                	je     10554 <shell_entry+0x444>
   10547:	83 f9 20             	cmp    $0x20,%ecx
   1054a:	74 08                	je     10554 <shell_entry+0x444>
   1054c:	48 ff c6             	inc    %rsi
   1054f:	eb e8                	jmp    10539 <shell_entry+0x429>
   10551:	4c 89 e6             	mov    %r12,%rsi
   10554:	48 89 f1             	mov    %rsi,%rcx
   10557:	48 29 f9             	sub    %rdi,%rcx
   1055a:	0f 82 ab 02 00 00    	jb     1080b <shell_entry+0x6fb>
   10560:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
   10565:	48 89 fa             	mov    %rdi,%rdx
   10568:	48 c1 e2 04          	shl    $0x4,%rdx
   1056c:	48 89 44 14 38       	mov    %rax,0x38(%rsp,%rdx,1)
   10571:	48 89 4c 14 40       	mov    %rcx,0x40(%rsp,%rdx,1)
   10576:	48 ff c7             	inc    %rdi
   10579:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
   1057e:	48 89 f7             	mov    %rsi,%rdi
   10581:	eb 8a                	jmp    1050d <shell_entry+0x3fd>
   10583:	b9 00 01 00 00       	mov    $0x100,%ecx
   10588:	48 8d bc 24 38 02 00 	lea    0x238(%rsp),%rdi
   1058f:	00 
   10590:	48 8d 74 24 38       	lea    0x38(%rsp),%rsi
   10595:	f3 a4                	rep movsb (%rsi),(%rdi)
   10597:	48 83 7c 24 08 00    	cmpq   $0x0,0x8(%rsp)
   1059d:	48 89 5c 24 18       	mov    %rbx,0x18(%rsp)
   105a2:	0f 84 7e 01 00 00    	je     10726 <shell_entry+0x616>
   105a8:	31 c0                	xor    %eax,%eax
   105aa:	48 3d 00 01 00 00    	cmp    $0x100,%rax
   105b0:	74 1b                	je     105cd <shell_entry+0x4bd>
   105b2:	48 c7 84 04 38 01 00 	movq   $0x1,0x138(%rsp,%rax,1)
   105b9:	00 01 00 00 00 
   105be:	48 83 a4 04 40 01 00 	andq   $0x0,0x140(%rsp,%rax,1)
   105c5:	00 00 
   105c7:	48 83 c0 10          	add    $0x10,%rax
   105cb:	eb dd                	jmp    105aa <shell_entry+0x49a>
   105cd:	48 8b 5c 24 08       	mov    0x8(%rsp),%rbx
   105d2:	6a 08                	push   $0x8
   105d4:	58                   	pop    %rax
   105d5:	48 83 eb 01          	sub    $0x1,%rbx
   105d9:	72 55                	jb     10630 <shell_entry+0x520>
   105db:	48 89 5c 24 20       	mov    %rbx,0x20(%rsp)
   105e0:	48 8b b4 04 30 02 00 	mov    0x230(%rsp,%rax,1),%rsi
   105e7:	00 
   105e8:	48 8b 94 04 38 02 00 	mov    0x238(%rsp,%rax,1),%rdx
   105ef:	00 
   105f0:	48 8d 7c 24 38       	lea    0x38(%rsp),%rdi
   105f5:	48 89 c3             	mov    %rax,%rbx
   105f8:	e8 43 10 00 00       	call   11640 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8>
   105fd:	83 7c 24 38 01       	cmpl   $0x1,0x38(%rsp)
   10602:	0f 84 1e 01 00 00    	je     10726 <shell_entry+0x616>
   10608:	48 8b 44 24 40       	mov    0x40(%rsp),%rax
   1060d:	48 8b 4c 24 48       	mov    0x48(%rsp),%rcx
   10612:	48 89 84 1c 30 01 00 	mov    %rax,0x130(%rsp,%rbx,1)
   10619:	00 
   1061a:	48 89 8c 1c 38 01 00 	mov    %rcx,0x138(%rsp,%rbx,1)
   10621:	00 
   10622:	48 83 c3 10          	add    $0x10,%rbx
   10626:	48 89 d8             	mov    %rbx,%rax
   10629:	48 8b 5c 24 20       	mov    0x20(%rsp),%rbx
   1062e:	eb a5                	jmp    105d5 <shell_entry+0x4c5>
   10630:	48 8b b4 24 38 01 00 	mov    0x138(%rsp),%rsi
   10637:	00 
   10638:	48 8b 94 24 40 01 00 	mov    0x140(%rsp),%rdx
   1063f:	00 
   10640:	48 85 d2             	test   %rdx,%rdx
   10643:	74 05                	je     1064a <shell_entry+0x53a>
   10645:	80 3e 2f             	cmpb   $0x2f,(%rsi)
   10648:	74 2d                	je     10677 <shell_entry+0x567>
   1064a:	c6 05 4f 1a 00 00 2f 	movb   $0x2f,0x1a4f(%rip)        # 120a0 <_RNvNvCs3uGVbeP0EXQ_5shell7execute8PATH_BUF>
   10651:	48 83 fa 3e          	cmp    $0x3e,%rdx
   10655:	6a 3e                	push   $0x3e
   10657:	58                   	pop    %rax
   10658:	48 0f 43 d0          	cmovae %rax,%rdx
   1065c:	48 8d 5a 01          	lea    0x1(%rdx),%rbx
   10660:	48 c7 c7 a1 20 01 00 	mov    $0x120a1,%rdi
   10667:	ff 15 e3 19 00 00    	call   *0x19e3(%rip)        # 12050 <memcpy+0x5f0>
   1066d:	48 c7 c6 a0 20 01 00 	mov    $0x120a0,%rsi
   10674:	48 89 da             	mov    %rbx,%rdx
   10677:	31 c0                	xor    %eax,%eax
   10679:	48 3d 00 01 00 00    	cmp    $0x100,%rax
   1067f:	74 12                	je     10693 <shell_entry+0x583>
   10681:	48 83 64 04 40 00    	andq   $0x0,0x40(%rsp,%rax,1)
   10687:	48 83 64 04 38 00    	andq   $0x0,0x38(%rsp,%rax,1)
   1068d:	48 83 c0 10          	add    $0x10,%rax
   10691:	eb e6                	jmp    10679 <shell_entry+0x569>
   10693:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
   10698:	48 83 f8 10          	cmp    $0x10,%rax
   1069c:	6a 10                	push   $0x10
   1069e:	41 5a                	pop    %r10
   106a0:	4c 0f 42 d0          	cmovb  %rax,%r10
   106a4:	6a 08                	push   $0x8
   106a6:	58                   	pop    %rax
   106a7:	31 c9                	xor    %ecx,%ecx
   106a9:	48 39 4c 24 08       	cmp    %rcx,0x8(%rsp)
   106ae:	74 2d                	je     106dd <shell_entry+0x5cd>
   106b0:	48 83 f9 10          	cmp    $0x10,%rcx
   106b4:	0f 84 60 01 00 00    	je     1081a <shell_entry+0x70a>
   106ba:	48 8b bc 04 30 01 00 	mov    0x130(%rsp,%rax,1),%rdi
   106c1:	00 
   106c2:	4c 8b 84 04 38 01 00 	mov    0x138(%rsp,%rax,1),%r8
   106c9:	00 
   106ca:	48 ff c1             	inc    %rcx
   106cd:	48 89 7c 04 30       	mov    %rdi,0x30(%rsp,%rax,1)
   106d2:	4c 89 44 04 38       	mov    %r8,0x38(%rsp,%rax,1)
   106d7:	48 83 c0 10          	add    $0x10,%rax
   106db:	eb cc                	jmp    106a9 <shell_entry+0x599>
   106dd:	b8 e9 03 00 00       	mov    $0x3e9,%eax
   106e2:	48 89 f7             	mov    %rsi,%rdi
   106e5:	48 89 d6             	mov    %rdx,%rsi
   106e8:	48 8d 54 24 38       	lea    0x38(%rsp),%rdx
   106ed:	0f 05                	syscall
   106ef:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
   106f4:	48 85 c0             	test   %rax,%rax
   106f7:	78 2d                	js     10726 <shell_entry+0x616>
   106f9:	48 63 f8             	movslq %eax,%rdi
   106fc:	b8 ea 03 00 00       	mov    $0x3ea,%eax
   10701:	0f 05                	syscall
   10703:	83 64 24 38 00       	andl   $0x0,0x38(%rsp)
   10708:	6a 3d                	push   $0x3d
   1070a:	58                   	pop    %rax
   1070b:	48 8d 74 24 38       	lea    0x38(%rsp),%rsi
   10710:	0f 05                	syscall
   10712:	6a 27                	push   $0x27
   10714:	58                   	pop    %rax
   10715:	0f 05                	syscall
   10717:	48 89 c7             	mov    %rax,%rdi
   1071a:	b8 ea 03 00 00       	mov    $0x3ea,%eax
   1071f:	0f 05                	syscall
   10721:	e9 ef fc ff ff       	jmp    10415 <shell_entry+0x305>
   10726:	48 8d 5c 24 06       	lea    0x6(%rsp),%rbx
   1072b:	48 89 df             	mov    %rbx,%rdi
   1072e:	48 c7 c6 78 1c 01 00 	mov    $0x11c78,%rsi
   10735:	6a 09                	push   $0x9
   10737:	5a                   	pop    %rdx
   10738:	e8 66 01 00 00       	call   108a3 <_RNvXCs1C5MKr7Wveb_10libsyscallNtB2_6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_str>
   1073d:	48 8d bc 24 38 01 00 	lea    0x138(%rsp),%rdi
   10744:	00 
   10745:	48 8b 74 24 18       	mov    0x18(%rsp),%rsi
   1074a:	4c 89 e2             	mov    %r12,%rdx
   1074d:	e8 ee 0e 00 00       	call   11640 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8>
   10752:	80 bc 24 38 01 00 00 	cmpb   $0x0,0x138(%rsp)
   10759:	00 
   1075a:	74 1f                	je     1077b <shell_entry+0x66b>
   1075c:	48 8d 44 24 10       	lea    0x10(%rsp),%rax
   10761:	48 89 44 24 38       	mov    %rax,0x38(%rsp)
   10766:	48 c7 44 24 40 d0 19 	movq   $0x119d0,0x40(%rsp)
   1076d:	01 00 
   1076f:	48 89 df             	mov    %rbx,%rdi
   10772:	48 c7 c6 af 1b 01 00 	mov    $0x11baf,%rsi
   10779:	eb 4a                	jmp    107c5 <shell_entry+0x6b5>
   1077b:	48 8b 84 24 40 01 00 	mov    0x140(%rsp),%rax
   10782:	00 
   10783:	48 8b 8c 24 48 01 00 	mov    0x148(%rsp),%rcx
   1078a:	00 
   1078b:	48 89 44 24 28       	mov    %rax,0x28(%rsp)
   10790:	48 89 4c 24 30       	mov    %rcx,0x30(%rsp)
   10795:	48 8d 44 24 28       	lea    0x28(%rsp),%rax
   1079a:	48 89 44 24 38       	mov    %rax,0x38(%rsp)
   1079f:	48 c7 44 24 40 60 00 	movq   $0x10060,0x40(%rsp)
   107a6:	01 00 
   107a8:	48 8d 44 24 10       	lea    0x10(%rsp),%rax
   107ad:	48 89 44 24 48       	mov    %rax,0x48(%rsp)
   107b2:	48 c7 44 24 50 d0 19 	movq   $0x119d0,0x50(%rsp)
   107b9:	01 00 
   107bb:	48 89 df             	mov    %rbx,%rdi
   107be:	48 c7 c6 f3 1a 01 00 	mov    $0x11af3,%rsi
   107c5:	48 8d 54 24 38       	lea    0x38(%rsp),%rdx
   107ca:	e8 2f f9 ff ff       	call   100fe <_RNvYNtCs1C5MKr7Wveb_10libsyscall6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_fmtCs3uGVbeP0EXQ_5shell>
   107cf:	e9 41 fc ff ff       	jmp    10415 <shell_entry+0x305>
   107d4:	ba 00 01 00 00       	mov    $0x100,%edx
   107d9:	31 ff                	xor    %edi,%edi
   107db:	48 89 de             	mov    %rbx,%rsi
   107de:	48 c7 c1 30 1e 01 00 	mov    $0x11e30,%rcx
   107e5:	e8 56 10 00 00       	call   11840 <_RNvNtNtCsfJBMPiLOdLr_4core5slice5index16slice_index_fail>
   107ea:	48 89 da             	mov    %rbx,%rdx
   107ed:	48 c7 c1 38 1c 01 00 	mov    $0x11c38,%rcx
   107f4:	e8 47 10 00 00       	call   11840 <_RNvNtNtCsfJBMPiLOdLr_4core5slice5index16slice_index_fail>
   107f9:	48 89 c7             	mov    %rax,%rdi
   107fc:	48 89 de             	mov    %rbx,%rsi
   107ff:	48 c7 c2 20 1c 01 00 	mov    $0x11c20,%rdx
   10806:	e8 05 0a 00 00       	call   11210 <_RNvNtCsfJBMPiLOdLr_4core9panicking18panic_bounds_check>
   1080b:	4c 89 e2             	mov    %r12,%rdx
   1080e:	48 c7 c1 08 1c 01 00 	mov    $0x11c08,%rcx
   10815:	e8 26 10 00 00       	call   11840 <_RNvNtNtCsfJBMPiLOdLr_4core5slice5index16slice_index_fail>
   1081a:	6a 10                	push   $0x10
   1081c:	5f                   	pop    %rdi
   1081d:	48 89 fe             	mov    %rdi,%rsi
   10820:	48 c7 c2 c8 1d 01 00 	mov    $0x11dc8,%rdx
   10827:	e8 e4 09 00 00       	call   11210 <_RNvNtCsfJBMPiLOdLr_4core9panicking18panic_bounds_check>
   1082c:	48 8d 7c 24 06       	lea    0x6(%rsp),%rdi
   10831:	6a 13                	push   $0x13
   10833:	5a                   	pop    %rdx
   10834:	48 c7 c6 b9 1d 01 00 	mov    $0x11db9,%rsi
   1083b:	e8 be f8 ff ff       	call   100fe <_RNvYNtCs1C5MKr7Wveb_10libsyscall6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_fmtCs3uGVbeP0EXQ_5shell>
   10840:	b8 a9 00 00 00       	mov    $0xa9,%eax
   10845:	31 ff                	xor    %edi,%edi
   10847:	0f 05                	syscall
   10849:	0f 0b                	ud2
   1084b:	48 8d 7c 24 06       	lea    0x6(%rsp),%rdi
   10850:	6a 59                	push   $0x59
   10852:	5a                   	pop    %rdx
   10853:	48 c7 c6 8d 1d 01 00 	mov    $0x11d8d,%rsi
   1085a:	e8 9f f8 ff ff       	call   100fe <_RNvYNtCs1C5MKr7Wveb_10libsyscall6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_fmtCs3uGVbeP0EXQ_5shell>
   1085f:	6a 01                	push   $0x1
   10861:	5f                   	pop    %rdi
   10862:	b8 a9 00 00 00       	mov    $0xa9,%eax
   10867:	0f 05                	syscall
   10869:	0f 0b                	ud2

000000000001086b <_RNvCs1C5MKr7Wveb_10libsyscall20common_panic_handler>:
   1086b:	50                   	push   %rax
   1086c:	6a 02                	push   $0x2
   1086e:	5f                   	pop    %rdi
   1086f:	6a 07                	push   $0x7
   10871:	5a                   	pop    %rdx
   10872:	48 c7 c6 4b 1e 01 00 	mov    $0x11e4b,%rsi
   10879:	e8 0a 00 00 00       	call   10888 <_RNvNtCs1C5MKr7Wveb_10libsyscall2io5write>
   1087e:	6a 3c                	push   $0x3c
   10880:	58                   	pop    %rax
   10881:	6a 01                	push   $0x1
   10883:	5f                   	pop    %rdi
   10884:	0f 05                	syscall
   10886:	0f 0b                	ud2

0000000000010888 <_RNvNtCs1C5MKr7Wveb_10libsyscall2io5write>:
   10888:	6a 01                	push   $0x1
   1088a:	41 58                	pop    %r8
   1088c:	6a 18                	push   $0x18
   1088e:	41 59                	pop    %r9
   10890:	4c 89 c0             	mov    %r8,%rax
   10893:	0f 05                	syscall
   10895:	48 83 f8 f5          	cmp    $0xfffffffffffffff5,%rax
   10899:	75 07                	jne    108a2 <_RNvNtCs1C5MKr7Wveb_10libsyscall2io5write+0x1a>
   1089b:	4c 89 c8             	mov    %r9,%rax
   1089e:	0f 05                	syscall
   108a0:	eb ee                	jmp    10890 <_RNvNtCs1C5MKr7Wveb_10libsyscall2io5write+0x8>
   108a2:	c3                   	ret

00000000000108a3 <_RNvXCs1C5MKr7Wveb_10libsyscallNtB2_6StdoutNtNtCsfJBMPiLOdLr_4core3fmt5Write9write_str>:
   108a3:	50                   	push   %rax
   108a4:	6a 01                	push   $0x1
   108a6:	5f                   	pop    %rdi
   108a7:	e8 dc ff ff ff       	call   10888 <_RNvNtCs1C5MKr7Wveb_10libsyscall2io5write>
   108ac:	31 c0                	xor    %eax,%eax
   108ae:	59                   	pop    %rcx
   108af:	c3                   	ret

00000000000108b0 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral>:
   108b0:	55                   	push   %rbp
   108b1:	48 89 e5             	mov    %rsp,%rbp
   108b4:	41 57                	push   %r15
   108b6:	41 56                	push   %r14
   108b8:	41 55                	push   %r13
   108ba:	41 54                	push   %r12
   108bc:	53                   	push   %rbx
   108bd:	48 83 ec 48          	sub    $0x48,%rsp
   108c1:	4c 89 45 b8          	mov    %r8,-0x48(%rbp)
   108c5:	49 89 d6             	mov    %rdx,%r14
   108c8:	49 89 fc             	mov    %rdi,%r12
   108cb:	85 f6                	test   %esi,%esi
   108cd:	74 5e                	je     1092d <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x7d>
   108cf:	45 8b 7c 24 10       	mov    0x10(%r12),%r15d
   108d4:	44 89 fa             	mov    %r15d,%edx
   108d7:	81 e2 00 00 20 00    	and    $0x200000,%edx
   108dd:	b8 00 00 11 00       	mov    $0x110000,%eax
   108e2:	be 2b 00 00 00       	mov    $0x2b,%esi
   108e7:	0f 44 f0             	cmove  %eax,%esi
   108ea:	89 75 d4             	mov    %esi,-0x2c(%rbp)
   108ed:	c1 ea 15             	shr    $0x15,%edx
   108f0:	4c 01 ca             	add    %r9,%rdx
   108f3:	41 f7 c7 00 00 80 00 	test   $0x800000,%r15d
   108fa:	74 4a                	je     10946 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x96>
   108fc:	48 89 55 c8          	mov    %rdx,-0x38(%rbp)
   10900:	48 83 f9 20          	cmp    $0x20,%rcx
   10904:	0f 83 97 00 00 00    	jae    109a1 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0xf1>
   1090a:	48 85 c9             	test   %rcx,%rcx
   1090d:	0f 84 ab 00 00 00    	je     109be <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x10e>
   10913:	41 89 ca             	mov    %ecx,%r10d
   10916:	41 83 e2 03          	and    $0x3,%r10d
   1091a:	48 83 f9 04          	cmp    $0x4,%rcx
   1091e:	0f 83 9e 00 00 00    	jae    109c2 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x112>
   10924:	31 d2                	xor    %edx,%edx
   10926:	31 c0                	xor    %eax,%eax
   10928:	e9 e6 00 00 00       	jmp    10a13 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x163>
   1092d:	49 8d 51 01          	lea    0x1(%r9),%rdx
   10931:	45 8b 7c 24 10       	mov    0x10(%r12),%r15d
   10936:	c7 45 d4 2d 00 00 00 	movl   $0x2d,-0x2c(%rbp)
   1093d:	41 f7 c7 00 00 80 00 	test   $0x800000,%r15d
   10944:	75 b6                	jne    108fc <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x4c>
   10946:	45 31 f6             	xor    %r14d,%r14d
   10949:	45 0f b7 6c 24 14    	movzwl 0x14(%r12),%r13d
   1094f:	4c 39 ea             	cmp    %r13,%rdx
   10952:	0f 82 f3 00 00 00    	jb     10a4b <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x19b>
   10958:	4d 89 cd             	mov    %r9,%r13
   1095b:	49 89 c8             	mov    %rcx,%r8
   1095e:	49 8b 1c 24          	mov    (%r12),%rbx
   10962:	4d 8b 7c 24 08       	mov    0x8(%r12),%r15
   10967:	48 89 df             	mov    %rbx,%rdi
   1096a:	4c 89 fe             	mov    %r15,%rsi
   1096d:	8b 55 d4             	mov    -0x2c(%rbp),%edx
   10970:	4c 89 f1             	mov    %r14,%rcx
   10973:	e8 98 0f 00 00       	call   11910 <_RNvNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB7_9Formatter12pad_integral12write_prefix>
   10978:	41 b6 01             	mov    $0x1,%r14b
   1097b:	84 c0                	test   %al,%al
   1097d:	0f 85 58 02 00 00    	jne    10bdb <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x32b>
   10983:	49 8b 47 18          	mov    0x18(%r15),%rax
   10987:	48 89 df             	mov    %rbx,%rdi
   1098a:	48 8b 75 b8          	mov    -0x48(%rbp),%rsi
   1098e:	4c 89 ea             	mov    %r13,%rdx
   10991:	48 83 c4 48          	add    $0x48,%rsp
   10995:	5b                   	pop    %rbx
   10996:	41 5c                	pop    %r12
   10998:	41 5d                	pop    %r13
   1099a:	41 5e                	pop    %r14
   1099c:	41 5f                	pop    %r15
   1099e:	5d                   	pop    %rbp
   1099f:	ff e0                	jmp    *%rax
   109a1:	4c 89 f7             	mov    %r14,%rdi
   109a4:	48 89 ce             	mov    %rcx,%rsi
   109a7:	49 89 cd             	mov    %rcx,%r13
   109aa:	4c 89 cb             	mov    %r9,%rbx
   109ad:	ff 15 c5 16 00 00    	call   *0x16c5(%rip)        # 12078 <memcpy+0x618>
   109b3:	49 89 d9             	mov    %rbx,%r9
   109b6:	4c 89 e9             	mov    %r13,%rcx
   109b9:	e9 77 00 00 00       	jmp    10a35 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x185>
   109be:	31 c0                	xor    %eax,%eax
   109c0:	eb 73                	jmp    10a35 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x185>
   109c2:	89 ce                	mov    %ecx,%esi
   109c4:	83 e6 1c             	and    $0x1c,%esi
   109c7:	31 d2                	xor    %edx,%edx
   109c9:	31 c0                	xor    %eax,%eax
   109cb:	0f 1f 44 00 00       	nopl   0x0(%rax,%rax,1)
   109d0:	31 ff                	xor    %edi,%edi
   109d2:	41 80 3c 16 c0       	cmpb   $0xc0,(%r14,%rdx,1)
   109d7:	40 0f 9d c7          	setge  %dil
   109db:	48 01 c7             	add    %rax,%rdi
   109de:	31 c0                	xor    %eax,%eax
   109e0:	41 80 7c 16 01 c0    	cmpb   $0xc0,0x1(%r14,%rdx,1)
   109e6:	0f 9d c0             	setge  %al
   109e9:	45 31 c0             	xor    %r8d,%r8d
   109ec:	41 80 7c 16 02 c0    	cmpb   $0xc0,0x2(%r14,%rdx,1)
   109f2:	41 0f 9d c0          	setge  %r8b
   109f6:	49 01 c0             	add    %rax,%r8
   109f9:	49 01 f8             	add    %rdi,%r8
   109fc:	31 c0                	xor    %eax,%eax
   109fe:	41 80 7c 16 03 c0    	cmpb   $0xc0,0x3(%r14,%rdx,1)
   10a04:	0f 9d c0             	setge  %al
   10a07:	4c 01 c0             	add    %r8,%rax
   10a0a:	48 83 c2 04          	add    $0x4,%rdx
   10a0e:	48 39 d6             	cmp    %rdx,%rsi
   10a11:	75 bd                	jne    109d0 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x120>
   10a13:	4d 85 d2             	test   %r10,%r10
   10a16:	74 1d                	je     10a35 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x185>
   10a18:	4c 01 f2             	add    %r14,%rdx
   10a1b:	31 f6                	xor    %esi,%esi
   10a1d:	0f 1f 00             	nopl   (%rax)
   10a20:	31 ff                	xor    %edi,%edi
   10a22:	80 3c 32 c0          	cmpb   $0xc0,(%rdx,%rsi,1)
   10a26:	40 0f 9d c7          	setge  %dil
   10a2a:	48 01 f8             	add    %rdi,%rax
   10a2d:	48 ff c6             	inc    %rsi
   10a30:	49 39 f2             	cmp    %rsi,%r10
   10a33:	75 eb                	jne    10a20 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x170>
   10a35:	48 8b 55 c8          	mov    -0x38(%rbp),%rdx
   10a39:	48 01 c2             	add    %rax,%rdx
   10a3c:	45 0f b7 6c 24 14    	movzwl 0x14(%r12),%r13d
   10a42:	4c 39 ea             	cmp    %r13,%rdx
   10a45:	0f 83 0d ff ff ff    	jae    10958 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0xa8>
   10a4b:	41 f7 c7 00 00 00 01 	test   $0x1000000,%r15d
   10a52:	4c 89 4d b0          	mov    %r9,-0x50(%rbp)
   10a56:	48 89 55 c8          	mov    %rdx,-0x38(%rbp)
   10a5a:	75 2c                	jne    10a88 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x1d8>
   10a5c:	44 89 ee             	mov    %r13d,%esi
   10a5f:	29 d6                	sub    %edx,%esi
   10a61:	44 89 f8             	mov    %r15d,%eax
   10a64:	c1 e8 1d             	shr    $0x1d,%eax
   10a67:	83 e0 03             	and    $0x3,%eax
   10a6a:	48 8d 15 e3 13 00 00 	lea    0x13e3(%rip),%rdx        # 11e54 <memcpy+0x3f4>
   10a71:	48 63 04 82          	movslq (%rdx,%rax,4),%rax
   10a75:	48 01 d0             	add    %rdx,%rax
   10a78:	48 89 4d 98          	mov    %rcx,-0x68(%rbp)
   10a7c:	89 75 d0             	mov    %esi,-0x30(%rbp)
   10a7f:	ff e0                	jmp    *%rax
   10a81:	89 f0                	mov    %esi,%eax
   10a83:	e9 92 00 00 00       	jmp    10b1a <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x26a>
   10a88:	49 8b 44 24 10       	mov    0x10(%r12),%rax
   10a8d:	48 89 45 c0          	mov    %rax,-0x40(%rbp)
   10a91:	25 00 00 e0 9f       	and    $0x9fe00000,%eax
   10a96:	0d 30 00 00 20       	or     $0x20000030,%eax
   10a9b:	41 89 44 24 10       	mov    %eax,0x10(%r12)
   10aa0:	4d 8b 3c 24          	mov    (%r12),%r15
   10aa4:	49 89 c8             	mov    %rcx,%r8
   10aa7:	49 8b 5c 24 08       	mov    0x8(%r12),%rbx
   10aac:	4c 89 ff             	mov    %r15,%rdi
   10aaf:	48 89 de             	mov    %rbx,%rsi
   10ab2:	8b 55 d4             	mov    -0x2c(%rbp),%edx
   10ab5:	4c 89 f1             	mov    %r14,%rcx
   10ab8:	e8 53 0e 00 00       	call   11910 <_RNvNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB7_9Formatter12pad_integral12write_prefix>
   10abd:	41 b6 01             	mov    $0x1,%r14b
   10ac0:	84 c0                	test   %al,%al
   10ac2:	0f 85 13 01 00 00    	jne    10bdb <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x32b>
   10ac8:	44 2b 6d c8          	sub    -0x38(%rbp),%r13d
   10acc:	41 ff c5             	inc    %r13d
   10acf:	90                   	nop
   10ad0:	66 41 ff cd          	dec    %r13w
   10ad4:	74 14                	je     10aea <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x23a>
   10ad6:	4c 89 ff             	mov    %r15,%rdi
   10ad9:	be 30 00 00 00       	mov    $0x30,%esi
   10ade:	ff 53 20             	call   *0x20(%rbx)
   10ae1:	84 c0                	test   %al,%al
   10ae3:	74 eb                	je     10ad0 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x220>
   10ae5:	e9 f1 00 00 00       	jmp    10bdb <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x32b>
   10aea:	4c 89 ff             	mov    %r15,%rdi
   10aed:	48 8b 75 b8          	mov    -0x48(%rbp),%rsi
   10af1:	48 8b 55 b0          	mov    -0x50(%rbp),%rdx
   10af5:	ff 53 18             	call   *0x18(%rbx)
   10af8:	84 c0                	test   %al,%al
   10afa:	0f 85 db 00 00 00    	jne    10bdb <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x32b>
   10b00:	48 8b 45 c0          	mov    -0x40(%rbp),%rax
   10b04:	49 89 44 24 10       	mov    %rax,0x10(%r12)
   10b09:	45 31 f6             	xor    %r14d,%r14d
   10b0c:	e9 ca 00 00 00       	jmp    10bdb <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x32b>
   10b11:	31 c0                	xor    %eax,%eax
   10b13:	eb 05                	jmp    10b1a <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x26a>
   10b15:	0f b7 c6             	movzwl %si,%eax
   10b18:	d1 e8                	shr    $1,%eax
   10b1a:	41 81 e7 ff ff 1f 00 	and    $0x1fffff,%r15d
   10b21:	44 89 7d c0          	mov    %r15d,-0x40(%rbp)
   10b25:	4c 89 e1             	mov    %r12,%rcx
   10b28:	4d 8b 24 24          	mov    (%r12),%r12
   10b2c:	48 8b 59 08          	mov    0x8(%rcx),%rbx
   10b30:	48 89 45 a0          	mov    %rax,-0x60(%rbp)
   10b34:	44 8d 78 01          	lea    0x1(%rax),%r15d
   10b38:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   10b3f:	00 
   10b40:	66 41 ff cf          	dec    %r15w
   10b44:	74 15                	je     10b5b <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x2ab>
   10b46:	4c 89 e7             	mov    %r12,%rdi
   10b49:	8b 75 c0             	mov    -0x40(%rbp),%esi
   10b4c:	ff 53 20             	call   *0x20(%rbx)
   10b4f:	84 c0                	test   %al,%al
   10b51:	74 ed                	je     10b40 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x290>
   10b53:	41 b6 01             	mov    $0x1,%r14b
   10b56:	e9 80 00 00 00       	jmp    10bdb <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x32b>
   10b5b:	4d 89 e7             	mov    %r12,%r15
   10b5e:	4c 89 e7             	mov    %r12,%rdi
   10b61:	48 89 de             	mov    %rbx,%rsi
   10b64:	48 89 5d a8          	mov    %rbx,-0x58(%rbp)
   10b68:	8b 55 d4             	mov    -0x2c(%rbp),%edx
   10b6b:	4c 89 f1             	mov    %r14,%rcx
   10b6e:	4c 8b 45 98          	mov    -0x68(%rbp),%r8
   10b72:	e8 99 0d 00 00       	call   11910 <_RNvNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB7_9Formatter12pad_integral12write_prefix>
   10b77:	41 b6 01             	mov    $0x1,%r14b
   10b7a:	84 c0                	test   %al,%al
   10b7c:	75 5d                	jne    10bdb <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x32b>
   10b7e:	4c 89 ff             	mov    %r15,%rdi
   10b81:	48 8b 75 b8          	mov    -0x48(%rbp),%rsi
   10b85:	48 8b 55 b0          	mov    -0x50(%rbp),%rdx
   10b89:	48 8b 45 a8          	mov    -0x58(%rbp),%rax
   10b8d:	ff 50 18             	call   *0x18(%rax)
   10b90:	84 c0                	test   %al,%al
   10b92:	75 47                	jne    10bdb <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x32b>
   10b94:	4c 8b 7d a0          	mov    -0x60(%rbp),%r15
   10b98:	44 29 7d d0          	sub    %r15d,-0x30(%rbp)
   10b9c:	44 03 7d c8          	add    -0x38(%rbp),%r15d
   10ba0:	45 29 ef             	sub    %r13d,%r15d
   10ba3:	66 41 be ff ff       	mov    $0xffff,%r14w
   10ba8:	44 8b 6d c0          	mov    -0x40(%rbp),%r13d
   10bac:	0f 1f 40 00          	nopl   0x0(%rax)
   10bb0:	43 8d 04 37          	lea    (%r15,%r14,1),%eax
   10bb4:	66 83 f8 ff          	cmp    $0xffff,%ax
   10bb8:	74 12                	je     10bcc <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x31c>
   10bba:	4c 89 e7             	mov    %r12,%rdi
   10bbd:	44 89 ee             	mov    %r13d,%esi
   10bc0:	ff 53 20             	call   *0x20(%rbx)
   10bc3:	41 ff c6             	inc    %r14d
   10bc6:	84 c0                	test   %al,%al
   10bc8:	74 e6                	je     10bb0 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x300>
   10bca:	eb 06                	jmp    10bd2 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter12pad_integral+0x322>
   10bcc:	8b 45 d0             	mov    -0x30(%rbp),%eax
   10bcf:	41 89 c6             	mov    %eax,%r14d
   10bd2:	66 44 3b 75 d0       	cmp    -0x30(%rbp),%r14w
   10bd7:	41 0f 92 c6          	setb   %r14b
   10bdb:	44 89 f0             	mov    %r14d,%eax
   10bde:	48 83 c4 48          	add    $0x48,%rsp
   10be2:	5b                   	pop    %rbx
   10be3:	41 5c                	pop    %r12
   10be5:	41 5d                	pop    %r13
   10be7:	41 5e                	pop    %r14
   10be9:	41 5f                	pop    %r15
   10beb:	5d                   	pop    %rbp
   10bec:	c3                   	ret
   10bed:	cc                   	int3
   10bee:	cc                   	int3
   10bef:	cc                   	int3

0000000000010bf0 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad>:
   10bf0:	55                   	push   %rbp
   10bf1:	48 89 e5             	mov    %rsp,%rbp
   10bf4:	41 57                	push   %r15
   10bf6:	41 56                	push   %r14
   10bf8:	41 55                	push   %r13
   10bfa:	41 54                	push   %r12
   10bfc:	53                   	push   %rbx
   10bfd:	48 83 ec 28          	sub    $0x28,%rsp
   10c01:	49 89 d5             	mov    %rdx,%r13
   10c04:	49 89 f6             	mov    %rsi,%r14
   10c07:	44 8b 7f 10          	mov    0x10(%rdi),%r15d
   10c0b:	41 f7 c7 00 00 00 18 	test   $0x18000000,%r15d
   10c12:	0f 84 f4 00 00 00    	je     10d0c <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x11c>
   10c18:	41 f7 c7 00 00 00 10 	test   $0x10000000,%r15d
   10c1f:	75 2d                	jne    10c4e <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x5e>
   10c21:	49 83 fd 20          	cmp    $0x20,%r13
   10c25:	0f 83 85 00 00 00    	jae    10cb0 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0xc0>
   10c2b:	4d 85 ed             	test   %r13,%r13
   10c2e:	0f 84 a3 01 00 00    	je     10dd7 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x1e7>
   10c34:	44 89 e8             	mov    %r13d,%eax
   10c37:	83 e0 03             	and    $0x3,%eax
   10c3a:	49 83 fd 04          	cmp    $0x4,%r13
   10c3e:	0f 83 9b 01 00 00    	jae    10ddf <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x1ef>
   10c44:	31 c9                	xor    %ecx,%ecx
   10c46:	45 31 e4             	xor    %r12d,%r12d
   10c49:	e9 e9 01 00 00       	jmp    10e37 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x247>
   10c4e:	44 0f b7 67 16       	movzwl 0x16(%rdi),%r12d
   10c53:	4d 85 e4             	test   %r12,%r12
   10c56:	74 6f                	je     10cc7 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0xd7>
   10c58:	4d 01 f5             	add    %r14,%r13
   10c5b:	31 d2                	xor    %edx,%edx
   10c5d:	4c 89 f1             	mov    %r14,%rcx
   10c60:	4c 89 e0             	mov    %r12,%rax
   10c63:	eb 20                	jmp    10c85 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x95>
   10c65:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   10c6c:	00 00 00 00 
   10c70:	4c 8d 41 01          	lea    0x1(%rcx),%r8
   10c74:	4c 89 c2             	mov    %r8,%rdx
   10c77:	48 29 ca             	sub    %rcx,%rdx
   10c7a:	48 01 f2             	add    %rsi,%rdx
   10c7d:	4c 89 c1             	mov    %r8,%rcx
   10c80:	48 ff c8             	dec    %rax
   10c83:	74 44                	je     10cc9 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0xd9>
   10c85:	48 89 d6             	mov    %rdx,%rsi
   10c88:	4c 39 e9             	cmp    %r13,%rcx
   10c8b:	74 40                	je     10ccd <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0xdd>
   10c8d:	0f b6 11             	movzbl (%rcx),%edx
   10c90:	84 d2                	test   %dl,%dl
   10c92:	79 dc                	jns    10c70 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x80>
   10c94:	80 fa e0             	cmp    $0xe0,%dl
   10c97:	72 0b                	jb     10ca4 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0xb4>
   10c99:	80 fa f0             	cmp    $0xf0,%dl
   10c9c:	72 0c                	jb     10caa <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0xba>
   10c9e:	4c 8d 41 04          	lea    0x4(%rcx),%r8
   10ca2:	eb d0                	jmp    10c74 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x84>
   10ca4:	4c 8d 41 02          	lea    0x2(%rcx),%r8
   10ca8:	eb ca                	jmp    10c74 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x84>
   10caa:	4c 8d 41 03          	lea    0x3(%rcx),%r8
   10cae:	eb c4                	jmp    10c74 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x84>
   10cb0:	48 89 fb             	mov    %rdi,%rbx
   10cb3:	4c 89 f7             	mov    %r14,%rdi
   10cb6:	4c 89 ee             	mov    %r13,%rsi
   10cb9:	ff 15 b9 13 00 00    	call   *0x13b9(%rip)        # 12078 <memcpy+0x618>
   10cbf:	48 89 df             	mov    %rbx,%rdi
   10cc2:	49 89 c4             	mov    %rax,%r12
   10cc5:	eb 0f                	jmp    10cd6 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0xe6>
   10cc7:	31 d2                	xor    %edx,%edx
   10cc9:	31 c0                	xor    %eax,%eax
   10ccb:	eb 03                	jmp    10cd0 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0xe0>
   10ccd:	48 89 f2             	mov    %rsi,%rdx
   10cd0:	49 29 c4             	sub    %rax,%r12
   10cd3:	49 89 d5             	mov    %rdx,%r13
   10cd6:	0f b7 47 14          	movzwl 0x14(%rdi),%eax
   10cda:	49 39 c4             	cmp    %rax,%r12
   10cdd:	73 2d                	jae    10d0c <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x11c>
   10cdf:	48 89 45 c0          	mov    %rax,-0x40(%rbp)
   10ce3:	89 c2                	mov    %eax,%edx
   10ce5:	44 29 e2             	sub    %r12d,%edx
   10ce8:	44 89 f8             	mov    %r15d,%eax
   10ceb:	c1 e8 1d             	shr    $0x1d,%eax
   10cee:	83 e0 03             	and    $0x3,%eax
   10cf1:	48 8d 0d 6c 11 00 00 	lea    0x116c(%rip),%rcx        # 11e64 <memcpy+0x404>
   10cf8:	48 63 04 81          	movslq (%rcx,%rax,4),%rax
   10cfc:	48 01 c8             	add    %rcx,%rax
   10cff:	4c 89 6d b0          	mov    %r13,-0x50(%rbp)
   10d03:	89 55 cc             	mov    %edx,-0x34(%rbp)
   10d06:	ff e0                	jmp    *%rax
   10d08:	31 c0                	xor    %eax,%eax
   10d0a:	eb 2d                	jmp    10d39 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x149>
   10d0c:	48 8b 07             	mov    (%rdi),%rax
   10d0f:	48 8b 4f 08          	mov    0x8(%rdi),%rcx
   10d13:	48 8b 49 18          	mov    0x18(%rcx),%rcx
   10d17:	48 89 c7             	mov    %rax,%rdi
   10d1a:	4c 89 f6             	mov    %r14,%rsi
   10d1d:	4c 89 ea             	mov    %r13,%rdx
   10d20:	48 83 c4 28          	add    $0x28,%rsp
   10d24:	5b                   	pop    %rbx
   10d25:	41 5c                	pop    %r12
   10d27:	41 5d                	pop    %r13
   10d29:	41 5e                	pop    %r14
   10d2b:	41 5f                	pop    %r15
   10d2d:	5d                   	pop    %rbp
   10d2e:	ff e1                	jmp    *%rcx
   10d30:	89 d0                	mov    %edx,%eax
   10d32:	eb 05                	jmp    10d39 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x149>
   10d34:	0f b7 c2             	movzwl %dx,%eax
   10d37:	d1 e8                	shr    $1,%eax
   10d39:	41 81 e7 ff ff 1f 00 	and    $0x1fffff,%r15d
   10d40:	4c 8b 2f             	mov    (%rdi),%r13
   10d43:	48 8b 4f 08          	mov    0x8(%rdi),%rcx
   10d47:	48 89 4d d0          	mov    %rcx,-0x30(%rbp)
   10d4b:	48 89 45 b8          	mov    %rax,-0x48(%rbp)
   10d4f:	8d 58 01             	lea    0x1(%rax),%ebx
   10d52:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   10d59:	1f 84 00 00 00 00 00 
   10d60:	4c 89 ef             	mov    %r13,%rdi
   10d63:	66 ff cb             	dec    %bx
   10d66:	74 15                	je     10d7d <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x18d>
   10d68:	44 89 fe             	mov    %r15d,%esi
   10d6b:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
   10d6f:	ff 50 20             	call   *0x20(%rax)
   10d72:	84 c0                	test   %al,%al
   10d74:	74 ea                	je     10d60 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x170>
   10d76:	b0 01                	mov    $0x1,%al
   10d78:	e9 f7 00 00 00       	jmp    10e74 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x284>
   10d7d:	4c 89 f6             	mov    %r14,%rsi
   10d80:	48 8b 55 b0          	mov    -0x50(%rbp),%rdx
   10d84:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
   10d88:	ff 50 18             	call   *0x18(%rax)
   10d8b:	89 c1                	mov    %eax,%ecx
   10d8d:	b0 01                	mov    $0x1,%al
   10d8f:	84 c9                	test   %cl,%cl
   10d91:	0f 85 dd 00 00 00    	jne    10e74 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x284>
   10d97:	8b 5d cc             	mov    -0x34(%rbp),%ebx
   10d9a:	48 8b 45 b8          	mov    -0x48(%rbp),%rax
   10d9e:	29 c3                	sub    %eax,%ebx
   10da0:	44 01 e0             	add    %r12d,%eax
   10da3:	49 89 c4             	mov    %rax,%r12
   10da6:	44 2b 65 c0          	sub    -0x40(%rbp),%r12d
   10daa:	66 41 be ff ff       	mov    $0xffff,%r14w
   10daf:	90                   	nop
   10db0:	43 8d 04 34          	lea    (%r12,%r14,1),%eax
   10db4:	66 83 f8 ff          	cmp    $0xffff,%ax
   10db8:	0f 84 ac 00 00 00    	je     10e6a <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x27a>
   10dbe:	4c 89 ef             	mov    %r13,%rdi
   10dc1:	44 89 fe             	mov    %r15d,%esi
   10dc4:	48 8b 45 d0          	mov    -0x30(%rbp),%rax
   10dc8:	ff 50 20             	call   *0x20(%rax)
   10dcb:	41 ff c6             	inc    %r14d
   10dce:	84 c0                	test   %al,%al
   10dd0:	74 de                	je     10db0 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x1c0>
   10dd2:	e9 96 00 00 00       	jmp    10e6d <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x27d>
   10dd7:	45 31 e4             	xor    %r12d,%r12d
   10dda:	e9 f7 fe ff ff       	jmp    10cd6 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0xe6>
   10ddf:	44 89 ea             	mov    %r13d,%edx
   10de2:	83 e2 1c             	and    $0x1c,%edx
   10de5:	31 c9                	xor    %ecx,%ecx
   10de7:	45 31 e4             	xor    %r12d,%r12d
   10dea:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   10df0:	31 f6                	xor    %esi,%esi
   10df2:	41 80 3c 0e c0       	cmpb   $0xc0,(%r14,%rcx,1)
   10df7:	40 0f 9d c6          	setge  %sil
   10dfb:	4c 01 e6             	add    %r12,%rsi
   10dfe:	45 31 c0             	xor    %r8d,%r8d
   10e01:	41 80 7c 0e 01 c0    	cmpb   $0xc0,0x1(%r14,%rcx,1)
   10e07:	41 0f 9d c0          	setge  %r8b
   10e0b:	45 31 c9             	xor    %r9d,%r9d
   10e0e:	41 80 7c 0e 02 c0    	cmpb   $0xc0,0x2(%r14,%rcx,1)
   10e14:	41 0f 9d c1          	setge  %r9b
   10e18:	4d 01 c1             	add    %r8,%r9
   10e1b:	49 01 f1             	add    %rsi,%r9
   10e1e:	45 31 e4             	xor    %r12d,%r12d
   10e21:	41 80 7c 0e 03 c0    	cmpb   $0xc0,0x3(%r14,%rcx,1)
   10e27:	41 0f 9d c4          	setge  %r12b
   10e2b:	4d 01 cc             	add    %r9,%r12
   10e2e:	48 83 c1 04          	add    $0x4,%rcx
   10e32:	48 39 ca             	cmp    %rcx,%rdx
   10e35:	75 b9                	jne    10df0 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x200>
   10e37:	48 85 c0             	test   %rax,%rax
   10e3a:	0f 84 96 fe ff ff    	je     10cd6 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0xe6>
   10e40:	4c 01 f1             	add    %r14,%rcx
   10e43:	31 d2                	xor    %edx,%edx
   10e45:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   10e4c:	00 00 00 00 
   10e50:	31 f6                	xor    %esi,%esi
   10e52:	80 3c 11 c0          	cmpb   $0xc0,(%rcx,%rdx,1)
   10e56:	40 0f 9d c6          	setge  %sil
   10e5a:	49 01 f4             	add    %rsi,%r12
   10e5d:	48 ff c2             	inc    %rdx
   10e60:	48 39 d0             	cmp    %rdx,%rax
   10e63:	75 eb                	jne    10e50 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0x260>
   10e65:	e9 6c fe ff ff       	jmp    10cd6 <_RNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB5_9Formatter3pad+0xe6>
   10e6a:	41 89 de             	mov    %ebx,%r14d
   10e6d:	66 41 39 de          	cmp    %bx,%r14w
   10e71:	0f 92 c0             	setb   %al
   10e74:	48 83 c4 28          	add    $0x28,%rsp
   10e78:	5b                   	pop    %rbx
   10e79:	41 5c                	pop    %r12
   10e7b:	41 5d                	pop    %r13
   10e7d:	41 5e                	pop    %r14
   10e7f:	41 5f                	pop    %r15
   10e81:	5d                   	pop    %rbp
   10e82:	c3                   	ret
   10e83:	cc                   	int3
   10e84:	cc                   	int3
   10e85:	cc                   	int3
   10e86:	cc                   	int3
   10e87:	cc                   	int3
   10e88:	cc                   	int3
   10e89:	cc                   	int3
   10e8a:	cc                   	int3
   10e8b:	cc                   	int3
   10e8c:	cc                   	int3
   10e8d:	cc                   	int3
   10e8e:	cc                   	int3
   10e8f:	cc                   	int3

0000000000010e90 <_RNvMsf_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impy10__fmt_inner>:
   10e90:	55                   	push   %rbp
   10e91:	48 89 e5             	mov    %rsp,%rbp
   10e94:	41 57                	push   %r15
   10e96:	41 56                	push   %r14
   10e98:	41 54                	push   %r12
   10e9a:	53                   	push   %rbx
   10e9b:	48 89 d1             	mov    %rdx,%rcx
   10e9e:	48 89 fa             	mov    %rdi,%rdx
   10ea1:	49 89 c8             	mov    %rcx,%r8
   10ea4:	48 81 ff e8 03 00 00 	cmp    $0x3e8,%rdi
   10eab:	0f 82 d6 00 00 00    	jb     10f87 <_RNvMsf_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impy10__fmt_inner+0xf7>
   10eb1:	4c 8d 5e ff          	lea    -0x1(%rsi),%r11
   10eb5:	45 31 c0             	xor    %r8d,%r8d
   10eb8:	48 bb 4b 59 86 38 d6 	movabs $0x346dc5d63886594b,%rbx
   10ebf:	c5 6d 34 
   10ec2:	4c 8d 35 ab 0f 00 00 	lea    0xfab(%rip),%r14        # 11e74 <memcpy+0x414>
   10ec9:	49 89 f9             	mov    %rdi,%r9
   10ecc:	0f 1f 40 00          	nopl   0x0(%rax)
   10ed0:	4e 8d 14 01          	lea    (%rcx,%r8,1),%r10
   10ed4:	49 83 c2 fc          	add    $0xfffffffffffffffc,%r10
   10ed8:	4c 89 c8             	mov    %r9,%rax
   10edb:	48 f7 e3             	mul    %rbx
   10ede:	49 39 ca             	cmp    %rcx,%r10
   10ee1:	0f 83 3c 01 00 00    	jae    11023 <_RNvMsf_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impy10__fmt_inner+0x193>
   10ee7:	48 c1 ea 0b          	shr    $0xb,%rdx
   10eeb:	69 c2 10 27 00 00    	imul   $0x2710,%edx,%eax
   10ef1:	45 89 ca             	mov    %r9d,%r10d
   10ef4:	41 29 c2             	sub    %eax,%r10d
   10ef7:	45 69 fa 7b 14 00 00 	imul   $0x147b,%r10d,%r15d
   10efe:	41 c1 ef 13          	shr    $0x13,%r15d
   10f02:	45 0f b7 e7          	movzwl %r15w,%r12d
   10f06:	43 0f b6 04 66       	movzbl (%r14,%r12,2),%eax
   10f0b:	41 88 44 0b fd       	mov    %al,-0x3(%r11,%rcx,1)
   10f10:	4a 8d 04 01          	lea    (%rcx,%r8,1),%rax
   10f14:	48 83 c0 fd          	add    $0xfffffffffffffffd,%rax
   10f18:	48 39 c8             	cmp    %rcx,%rax
   10f1b:	0f 83 ef 00 00 00    	jae    11010 <_RNvMsf_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impy10__fmt_inner+0x180>
   10f21:	43 0f b6 44 66 01    	movzbl 0x1(%r14,%r12,2),%eax
   10f27:	41 88 44 0b fe       	mov    %al,-0x2(%r11,%rcx,1)
   10f2c:	4a 8d 04 01          	lea    (%rcx,%r8,1),%rax
   10f30:	48 83 c0 fe          	add    $0xfffffffffffffffe,%rax
   10f34:	48 39 c8             	cmp    %rcx,%rax
   10f37:	0f 83 d3 00 00 00    	jae    11010 <_RNvMsf_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impy10__fmt_inner+0x180>
   10f3d:	41 6b c7 64          	imul   $0x64,%r15d,%eax
   10f41:	41 29 c2             	sub    %eax,%r10d
   10f44:	45 0f b7 d2          	movzwl %r10w,%r10d
   10f48:	43 0f b6 04 56       	movzbl (%r14,%r10,2),%eax
   10f4d:	41 88 44 0b ff       	mov    %al,-0x1(%r11,%rcx,1)
   10f52:	4a 8d 04 01          	lea    (%rcx,%r8,1),%rax
   10f56:	48 ff c8             	dec    %rax
   10f59:	48 39 c8             	cmp    %rcx,%rax
   10f5c:	0f 83 ae 00 00 00    	jae    11010 <_RNvMsf_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impy10__fmt_inner+0x180>
   10f62:	43 0f b6 44 56 01    	movzbl 0x1(%r14,%r10,2),%eax
   10f68:	41 88 04 0b          	mov    %al,(%r11,%rcx,1)
   10f6c:	49 83 c0 fc          	add    $0xfffffffffffffffc,%r8
   10f70:	49 83 c3 fc          	add    $0xfffffffffffffffc,%r11
   10f74:	49 81 f9 7f 96 98 00 	cmp    $0x98967f,%r9
   10f7b:	49 89 d1             	mov    %rdx,%r9
   10f7e:	0f 87 4c ff ff ff    	ja     10ed0 <_RNvMsf_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impy10__fmt_inner+0x40>
   10f84:	49 01 c8             	add    %rcx,%r8
   10f87:	48 83 fa 09          	cmp    $0x9,%rdx
   10f8b:	76 56                	jbe    10fe3 <_RNvMsf_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impy10__fmt_inner+0x153>
   10f8d:	49 8d 40 fe          	lea    -0x2(%r8),%rax
   10f91:	48 39 c8             	cmp    %rcx,%rax
   10f94:	73 7a                	jae    11010 <_RNvMsf_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impy10__fmt_inner+0x180>
   10f96:	41 89 d1             	mov    %edx,%r9d
   10f99:	41 c1 e9 02          	shr    $0x2,%r9d
   10f9d:	45 0f b7 c9          	movzwl %r9w,%r9d
   10fa1:	45 69 c9 7b 14 00 00 	imul   $0x147b,%r9d,%r9d
   10fa8:	41 c1 e9 11          	shr    $0x11,%r9d
   10fac:	45 6b d1 64          	imul   $0x64,%r9d,%r10d
   10fb0:	44 29 d2             	sub    %r10d,%edx
   10fb3:	0f b7 d2             	movzwl %dx,%edx
   10fb6:	4c 8d 15 b7 0e 00 00 	lea    0xeb7(%rip),%r10        # 11e74 <memcpy+0x414>
   10fbd:	45 0f b6 1c 52       	movzbl (%r10,%rdx,2),%r11d
   10fc2:	46 88 5c 06 fe       	mov    %r11b,-0x2(%rsi,%r8,1)
   10fc7:	49 ff c8             	dec    %r8
   10fca:	49 39 c8             	cmp    %rcx,%r8
   10fcd:	73 67                	jae    11036 <_RNvMsf_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impy10__fmt_inner+0x1a6>
   10fcf:	41 0f b6 54 52 01    	movzbl 0x1(%r10,%rdx,2),%edx
   10fd5:	42 88 14 06          	mov    %dl,(%rsi,%r8,1)
   10fd9:	4c 89 ca             	mov    %r9,%rdx
   10fdc:	48 85 ff             	test   %rdi,%rdi
   10fdf:	75 0a                	jne    10feb <_RNvMsf_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impy10__fmt_inner+0x15b>
   10fe1:	eb 0d                	jmp    10ff0 <_RNvMsf_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impy10__fmt_inner+0x160>
   10fe3:	4c 89 c0             	mov    %r8,%rax
   10fe6:	48 85 ff             	test   %rdi,%rdi
   10fe9:	74 05                	je     10ff0 <_RNvMsf_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impy10__fmt_inner+0x160>
   10feb:	48 85 d2             	test   %rdx,%rdx
   10fee:	74 17                	je     11007 <_RNvMsf_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impy10__fmt_inner+0x177>
   10ff0:	48 ff c8             	dec    %rax
   10ff3:	48 39 c8             	cmp    %rcx,%rax
   10ff6:	73 18                	jae    11010 <_RNvMsf_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impy10__fmt_inner+0x180>
   10ff8:	48 8d 0d 75 0e 00 00 	lea    0xe75(%rip),%rcx        # 11e74 <memcpy+0x414>
   10fff:	0f b6 4c 51 01       	movzbl 0x1(%rcx,%rdx,2),%ecx
   11004:	88 0c 06             	mov    %cl,(%rsi,%rax,1)
   11007:	5b                   	pop    %rbx
   11008:	41 5c                	pop    %r12
   1100a:	41 5e                	pop    %r14
   1100c:	41 5f                	pop    %r15
   1100e:	5d                   	pop    %rbp
   1100f:	c3                   	ret
   11010:	48 8d 15 71 10 00 00 	lea    0x1071(%rip),%rdx        # 12088 <memcpy+0x628>
   11017:	48 89 c7             	mov    %rax,%rdi
   1101a:	48 89 ce             	mov    %rcx,%rsi
   1101d:	ff 15 35 10 00 00    	call   *0x1035(%rip)        # 12058 <memcpy+0x5f8>
   11023:	48 8d 15 5e 10 00 00 	lea    0x105e(%rip),%rdx        # 12088 <memcpy+0x628>
   1102a:	4c 89 d7             	mov    %r10,%rdi
   1102d:	48 89 ce             	mov    %rcx,%rsi
   11030:	ff 15 22 10 00 00    	call   *0x1022(%rip)        # 12058 <memcpy+0x5f8>
   11036:	48 8d 15 4b 10 00 00 	lea    0x104b(%rip),%rdx        # 12088 <memcpy+0x628>
   1103d:	4c 89 c7             	mov    %r8,%rdi
   11040:	48 89 ce             	mov    %rcx,%rsi
   11043:	ff 15 0f 10 00 00    	call   *0x100f(%rip)        # 12058 <memcpy+0x5f8>
   11049:	cc                   	int3
   1104a:	cc                   	int3
   1104b:	cc                   	int3
   1104c:	cc                   	int3
   1104d:	cc                   	int3
   1104e:	cc                   	int3
   1104f:	cc                   	int3

0000000000011050 <_RNvNtCsfJBMPiLOdLr_4core3fmt5write>:
   11050:	55                   	push   %rbp
   11051:	48 89 e5             	mov    %rsp,%rbp
   11054:	41 57                	push   %r15
   11056:	41 56                	push   %r14
   11058:	41 55                	push   %r13
   1105a:	41 54                	push   %r12
   1105c:	53                   	push   %rbx
   1105d:	48 83 ec 38          	sub    $0x38,%rsp
   11061:	48 89 cb             	mov    %rcx,%rbx
   11064:	49 89 d5             	mov    %rdx,%r13
   11067:	49 89 fe             	mov    %rdi,%r14
   1106a:	f6 c3 01             	test   $0x1,%bl
   1106d:	0f 85 68 01 00 00    	jne    111db <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x18b>
   11073:	41 0f b6 45 00       	movzbl 0x0(%r13),%eax
   11078:	84 c0                	test   %al,%al
   1107a:	0f 84 7b 01 00 00    	je     111fb <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x1ab>
   11080:	48 89 75 b0          	mov    %rsi,-0x50(%rbp)
   11084:	48 8b 4e 18          	mov    0x18(%rsi),%rcx
   11088:	48 89 4d a8          	mov    %rcx,-0x58(%rbp)
   1108c:	48 c7 45 d0 00 00 00 	movq   $0x0,-0x30(%rbp)
   11093:	00 
   11094:	eb 1a                	jmp    110b0 <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x60>
   11096:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   1109d:	00 00 00 
   110a0:	41 0f b6 04 24       	movzbl (%r12),%eax
   110a5:	4d 89 e5             	mov    %r12,%r13
   110a8:	84 c0                	test   %al,%al
   110aa:	0f 84 4b 01 00 00    	je     111fb <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x1ab>
   110b0:	4d 8d 65 01          	lea    0x1(%r13),%r12
   110b4:	44 0f b6 f8          	movzbl %al,%r15d
   110b8:	84 c0                	test   %al,%al
   110ba:	78 24                	js     110e0 <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x90>
   110bc:	4c 89 f7             	mov    %r14,%rdi
   110bf:	4c 89 e6             	mov    %r12,%rsi
   110c2:	4c 89 fa             	mov    %r15,%rdx
   110c5:	ff 55 a8             	call   *-0x58(%rbp)
   110c8:	84 c0                	test   %al,%al
   110ca:	0f 85 2f 01 00 00    	jne    111ff <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x1af>
   110d0:	4d 01 fc             	add    %r15,%r12
   110d3:	eb cb                	jmp    110a0 <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x50>
   110d5:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   110dc:	00 00 00 00 
   110e0:	3c 80                	cmp    $0x80,%al
   110e2:	74 21                	je     11105 <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0xb5>
   110e4:	41 81 ff c0 00 00 00 	cmp    $0xc0,%r15d
   110eb:	75 3f                	jne    1112c <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0xdc>
   110ed:	4c 8b 7d d0          	mov    -0x30(%rbp),%r15
   110f1:	4c 89 f8             	mov    %r15,%rax
   110f4:	48 c1 e0 04          	shl    $0x4,%rax
   110f8:	48 c7 45 c8 20 00 00 	movq   $0x60000020,-0x38(%rbp)
   110ff:	60 
   11100:	e9 9d 00 00 00       	jmp    111a2 <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x152>
   11105:	45 0f b7 7d 01       	movzwl 0x1(%r13),%r15d
   1110a:	49 8d 75 03          	lea    0x3(%r13),%rsi
   1110e:	4c 89 f7             	mov    %r14,%rdi
   11111:	4c 89 fa             	mov    %r15,%rdx
   11114:	ff 55 a8             	call   *-0x58(%rbp)
   11117:	84 c0                	test   %al,%al
   11119:	0f 85 e0 00 00 00    	jne    111ff <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x1af>
   1111f:	4f 8d 24 2f          	lea    (%r15,%r13,1),%r12
   11123:	49 83 c4 03          	add    $0x3,%r12
   11127:	e9 74 ff ff ff       	jmp    110a0 <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x50>
   1112c:	b9 20 00 00 60       	mov    $0x60000020,%ecx
   11131:	a8 01                	test   $0x1,%al
   11133:	74 0b                	je     11140 <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0xf0>
   11135:	41 8b 4d 01          	mov    0x1(%r13),%ecx
   11139:	49 83 c5 05          	add    $0x5,%r13
   1113d:	4d 89 ec             	mov    %r13,%r12
   11140:	a8 02                	test   $0x2,%al
   11142:	75 19                	jne    1115d <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x10d>
   11144:	31 d2                	xor    %edx,%edx
   11146:	4c 8b 7d d0          	mov    -0x30(%rbp),%r15
   1114a:	a8 04                	test   $0x4,%al
   1114c:	74 20                	je     1116e <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x11e>
   1114e:	41 0f b7 34 24       	movzwl (%r12),%esi
   11153:	49 83 c4 02          	add    $0x2,%r12
   11157:	a8 08                	test   $0x8,%al
   11159:	75 19                	jne    11174 <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x124>
   1115b:	eb 20                	jmp    1117d <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x12d>
   1115d:	41 0f b7 14 24       	movzwl (%r12),%edx
   11162:	49 83 c4 02          	add    $0x2,%r12
   11166:	4c 8b 7d d0          	mov    -0x30(%rbp),%r15
   1116a:	a8 04                	test   $0x4,%al
   1116c:	75 e0                	jne    1114e <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0xfe>
   1116e:	31 f6                	xor    %esi,%esi
   11170:	a8 08                	test   $0x8,%al
   11172:	74 09                	je     1117d <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x12d>
   11174:	45 0f b7 3c 24       	movzwl (%r12),%r15d
   11179:	49 83 c4 02          	add    $0x2,%r12
   1117d:	a8 10                	test   $0x10,%al
   1117f:	75 49                	jne    111ca <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x17a>
   11181:	a8 20                	test   $0x20,%al
   11183:	74 0b                	je     11190 <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x140>
   11185:	0f b7 c6             	movzwl %si,%eax
   11188:	c1 e0 04             	shl    $0x4,%eax
   1118b:	0f b7 74 03 08       	movzwl 0x8(%rbx,%rax,1),%esi
   11190:	4c 89 f8             	mov    %r15,%rax
   11193:	48 c1 e0 04          	shl    $0x4,%rax
   11197:	89 4d c8             	mov    %ecx,-0x38(%rbp)
   1119a:	66 89 55 cc          	mov    %dx,-0x34(%rbp)
   1119e:	66 89 75 ce          	mov    %si,-0x32(%rbp)
   111a2:	4c 89 75 b8          	mov    %r14,-0x48(%rbp)
   111a6:	48 8b 4d b0          	mov    -0x50(%rbp),%rcx
   111aa:	48 89 4d c0          	mov    %rcx,-0x40(%rbp)
   111ae:	48 8b 3c 03          	mov    (%rbx,%rax,1),%rdi
   111b2:	48 8d 75 b8          	lea    -0x48(%rbp),%rsi
   111b6:	ff 54 03 08          	call   *0x8(%rbx,%rax,1)
   111ba:	84 c0                	test   %al,%al
   111bc:	75 41                	jne    111ff <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x1af>
   111be:	49 ff c7             	inc    %r15
   111c1:	4c 89 7d d0          	mov    %r15,-0x30(%rbp)
   111c5:	e9 d6 fe ff ff       	jmp    110a0 <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x50>
   111ca:	0f b7 d2             	movzwl %dx,%edx
   111cd:	c1 e2 04             	shl    $0x4,%edx
   111d0:	0f b7 54 13 08       	movzwl 0x8(%rbx,%rdx,1),%edx
   111d5:	a8 20                	test   $0x20,%al
   111d7:	75 ac                	jne    11185 <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x135>
   111d9:	eb b5                	jmp    11190 <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x140>
   111db:	48 d1 eb             	shr    $1,%rbx
   111de:	48 8b 46 18          	mov    0x18(%rsi),%rax
   111e2:	4c 89 f7             	mov    %r14,%rdi
   111e5:	4c 89 ee             	mov    %r13,%rsi
   111e8:	48 89 da             	mov    %rbx,%rdx
   111eb:	48 83 c4 38          	add    $0x38,%rsp
   111ef:	5b                   	pop    %rbx
   111f0:	41 5c                	pop    %r12
   111f2:	41 5d                	pop    %r13
   111f4:	41 5e                	pop    %r14
   111f6:	41 5f                	pop    %r15
   111f8:	5d                   	pop    %rbp
   111f9:	ff e0                	jmp    *%rax
   111fb:	31 c0                	xor    %eax,%eax
   111fd:	eb 02                	jmp    11201 <_RNvNtCsfJBMPiLOdLr_4core3fmt5write+0x1b1>
   111ff:	b0 01                	mov    $0x1,%al
   11201:	48 83 c4 38          	add    $0x38,%rsp
   11205:	5b                   	pop    %rbx
   11206:	41 5c                	pop    %r12
   11208:	41 5d                	pop    %r13
   1120a:	41 5e                	pop    %r14
   1120c:	41 5f                	pop    %r15
   1120e:	5d                   	pop    %rbp
   1120f:	c3                   	ret

0000000000011210 <_RNvNtCsfJBMPiLOdLr_4core9panicking18panic_bounds_check>:
   11210:	55                   	push   %rbp
   11211:	48 89 e5             	mov    %rsp,%rbp
   11214:	48 83 ec 30          	sub    $0x30,%rsp
   11218:	48 8d 45 f8          	lea    -0x8(%rbp),%rax
   1121c:	48 89 38             	mov    %rdi,(%rax)
   1121f:	48 8d 4d f0          	lea    -0x10(%rbp),%rcx
   11223:	48 89 31             	mov    %rsi,(%rcx)
   11226:	48 8d 75 d0          	lea    -0x30(%rbp),%rsi
   1122a:	48 89 0e             	mov    %rcx,(%rsi)
   1122d:	48 8b 0d 34 0e 00 00 	mov    0xe34(%rip),%rcx        # 12068 <memcpy+0x608>
   11234:	48 89 4e 08          	mov    %rcx,0x8(%rsi)
   11238:	48 89 46 10          	mov    %rax,0x10(%rsi)
   1123c:	48 89 4e 18          	mov    %rcx,0x18(%rsi)
   11240:	48 8d 3d 75 08 00 00 	lea    0x875(%rip),%rdi        # 11abc <memcpy+0x5c>
   11247:	ff 15 13 0e 00 00    	call   *0xe13(%rip)        # 12060 <memcpy+0x600>
   1124d:	cc                   	int3
   1124e:	cc                   	int3
   1124f:	cc                   	int3

0000000000011250 <_RNvNtCsfJBMPiLOdLr_4core9panicking9panic_fmt>:
   11250:	55                   	push   %rbp
   11251:	48 89 e5             	mov    %rsp,%rbp
   11254:	48 83 ec 30          	sub    $0x30,%rsp
   11258:	48 89 7d f0          	mov    %rdi,-0x10(%rbp)
   1125c:	48 89 75 f8          	mov    %rsi,-0x8(%rbp)
   11260:	48 8d 45 f0          	lea    -0x10(%rbp),%rax
   11264:	48 89 45 d8          	mov    %rax,-0x28(%rbp)
   11268:	48 89 55 e0          	mov    %rdx,-0x20(%rbp)
   1126c:	66 c7 45 e8 01 00    	movw   $0x1,-0x18(%rbp)
   11272:	48 8d 7d d8          	lea    -0x28(%rbp),%rdi
   11276:	ff 15 cc 0d 00 00    	call   *0xdcc(%rip)        # 12048 <memcpy+0x5e8>
   1127c:	cc                   	int3
   1127d:	cc                   	int3
   1127e:	cc                   	int3
   1127f:	cc                   	int3

0000000000011280 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars>:
   11280:	4c 8d 4f 07          	lea    0x7(%rdi),%r9
   11284:	49 83 e1 f8          	and    $0xfffffffffffffff8,%r9
   11288:	4c 89 c8             	mov    %r9,%rax
   1128b:	48 29 f8             	sub    %rdi,%rax
   1128e:	48 89 f1             	mov    %rsi,%rcx
   11291:	48 29 c1             	sub    %rax,%rcx
   11294:	73 19                	jae    112af <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x2f>
   11296:	48 85 f6             	test   %rsi,%rsi
   11299:	74 55                	je     112f0 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x70>
   1129b:	89 f1                	mov    %esi,%ecx
   1129d:	83 e1 03             	and    $0x3,%ecx
   112a0:	48 83 fe 04          	cmp    $0x4,%rsi
   112a4:	73 4d                	jae    112f3 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x73>
   112a6:	31 d2                	xor    %edx,%edx
   112a8:	31 c0                	xor    %eax,%eax
   112aa:	e9 91 00 00 00       	jmp    11340 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0xc0>
   112af:	48 83 f9 08          	cmp    $0x8,%rcx
   112b3:	72 e1                	jb     11296 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x16>
   112b5:	41 57                	push   %r15
   112b7:	41 56                	push   %r14
   112b9:	41 55                	push   %r13
   112bb:	41 54                	push   %r12
   112bd:	53                   	push   %rbx
   112be:	89 ca                	mov    %ecx,%edx
   112c0:	83 e2 07             	and    $0x7,%edx
   112c3:	49 39 f9             	cmp    %rdi,%r9
   112c6:	75 07                	jne    112cf <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x4f>
   112c8:	31 f6                	xor    %esi,%esi
   112ca:	e9 18 01 00 00       	jmp    113e7 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x167>
   112cf:	41 89 c0             	mov    %eax,%r8d
   112d2:	41 83 e0 03          	and    $0x3,%r8d
   112d6:	48 89 fe             	mov    %rdi,%rsi
   112d9:	4c 29 ce             	sub    %r9,%rsi
   112dc:	48 83 fe fc          	cmp    $0xfffffffffffffffc,%rsi
   112e0:	0f 86 80 00 00 00    	jbe    11366 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0xe6>
   112e6:	45 31 c9             	xor    %r9d,%r9d
   112e9:	31 f6                	xor    %esi,%esi
   112eb:	e9 d4 00 00 00       	jmp    113c4 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x144>
   112f0:	31 c0                	xor    %eax,%eax
   112f2:	c3                   	ret
   112f3:	48 83 e6 fc          	and    $0xfffffffffffffffc,%rsi
   112f7:	31 d2                	xor    %edx,%edx
   112f9:	31 c0                	xor    %eax,%eax
   112fb:	0f 1f 44 00 00       	nopl   0x0(%rax,%rax,1)
   11300:	45 31 c0             	xor    %r8d,%r8d
   11303:	80 3c 17 c0          	cmpb   $0xc0,(%rdi,%rdx,1)
   11307:	41 0f 9d c0          	setge  %r8b
   1130b:	49 01 c0             	add    %rax,%r8
   1130e:	31 c0                	xor    %eax,%eax
   11310:	80 7c 17 01 c0       	cmpb   $0xc0,0x1(%rdi,%rdx,1)
   11315:	0f 9d c0             	setge  %al
   11318:	45 31 c9             	xor    %r9d,%r9d
   1131b:	80 7c 17 02 c0       	cmpb   $0xc0,0x2(%rdi,%rdx,1)
   11320:	41 0f 9d c1          	setge  %r9b
   11324:	49 01 c1             	add    %rax,%r9
   11327:	4d 01 c1             	add    %r8,%r9
   1132a:	31 c0                	xor    %eax,%eax
   1132c:	80 7c 17 03 c0       	cmpb   $0xc0,0x3(%rdi,%rdx,1)
   11331:	0f 9d c0             	setge  %al
   11334:	4c 01 c8             	add    %r9,%rax
   11337:	48 83 c2 04          	add    $0x4,%rdx
   1133b:	48 39 d6             	cmp    %rdx,%rsi
   1133e:	75 c0                	jne    11300 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x80>
   11340:	48 85 c9             	test   %rcx,%rcx
   11343:	74 20                	je     11365 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0xe5>
   11345:	48 01 d7             	add    %rdx,%rdi
   11348:	31 d2                	xor    %edx,%edx
   1134a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   11350:	31 f6                	xor    %esi,%esi
   11352:	80 3c 17 c0          	cmpb   $0xc0,(%rdi,%rdx,1)
   11356:	40 0f 9d c6          	setge  %sil
   1135a:	48 01 f0             	add    %rsi,%rax
   1135d:	48 ff c2             	inc    %rdx
   11360:	48 39 d1             	cmp    %rdx,%rcx
   11363:	75 eb                	jne    11350 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0xd0>
   11365:	c3                   	ret
   11366:	41 89 c2             	mov    %eax,%r10d
   11369:	41 83 e2 04          	and    $0x4,%r10d
   1136d:	45 31 c9             	xor    %r9d,%r9d
   11370:	31 f6                	xor    %esi,%esi
   11372:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   11379:	1f 84 00 00 00 00 00 
   11380:	45 31 db             	xor    %r11d,%r11d
   11383:	42 80 3c 0f c0       	cmpb   $0xc0,(%rdi,%r9,1)
   11388:	41 0f 9d c3          	setge  %r11b
   1138c:	49 01 f3             	add    %rsi,%r11
   1138f:	31 f6                	xor    %esi,%esi
   11391:	42 80 7c 0f 01 c0    	cmpb   $0xc0,0x1(%rdi,%r9,1)
   11397:	40 0f 9d c6          	setge  %sil
   1139b:	31 db                	xor    %ebx,%ebx
   1139d:	42 80 7c 0f 02 c0    	cmpb   $0xc0,0x2(%rdi,%r9,1)
   113a3:	0f 9d c3             	setge  %bl
   113a6:	48 01 f3             	add    %rsi,%rbx
   113a9:	4c 01 db             	add    %r11,%rbx
   113ac:	31 f6                	xor    %esi,%esi
   113ae:	42 80 7c 0f 03 c0    	cmpb   $0xc0,0x3(%rdi,%r9,1)
   113b4:	40 0f 9d c6          	setge  %sil
   113b8:	48 01 de             	add    %rbx,%rsi
   113bb:	49 83 c1 04          	add    $0x4,%r9
   113bf:	4d 39 ca             	cmp    %r9,%r10
   113c2:	75 bc                	jne    11380 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x100>
   113c4:	4d 85 c0             	test   %r8,%r8
   113c7:	74 1e                	je     113e7 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x167>
   113c9:	49 01 f9             	add    %rdi,%r9
   113cc:	45 31 d2             	xor    %r10d,%r10d
   113cf:	90                   	nop
   113d0:	45 31 db             	xor    %r11d,%r11d
   113d3:	43 80 3c 11 c0       	cmpb   $0xc0,(%r9,%r10,1)
   113d8:	41 0f 9d c3          	setge  %r11b
   113dc:	4c 01 de             	add    %r11,%rsi
   113df:	49 ff c2             	inc    %r10
   113e2:	4d 39 d0             	cmp    %r10,%r8
   113e5:	75 e9                	jne    113d0 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x150>
   113e7:	48 01 c7             	add    %rax,%rdi
   113ea:	48 85 d2             	test   %rdx,%rdx
   113ed:	0f 84 95 00 00 00    	je     11488 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x208>
   113f3:	49 b8 f8 ff ff ff ff 	movabs $0x7ffffffffffffff8,%r8
   113fa:	ff ff 7f 
   113fd:	49 21 c8             	and    %rcx,%r8
   11400:	31 c0                	xor    %eax,%eax
   11402:	42 80 3c 07 c0       	cmpb   $0xc0,(%rdi,%r8,1)
   11407:	0f 9d c0             	setge  %al
   1140a:	83 fa 01             	cmp    $0x1,%edx
   1140d:	74 7b                	je     1148a <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x20a>
   1140f:	45 31 c9             	xor    %r9d,%r9d
   11412:	42 80 7c 07 01 c0    	cmpb   $0xc0,0x1(%rdi,%r8,1)
   11418:	41 0f 9d c1          	setge  %r9b
   1141c:	4c 01 c8             	add    %r9,%rax
   1141f:	83 fa 02             	cmp    $0x2,%edx
   11422:	74 66                	je     1148a <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x20a>
   11424:	45 31 c9             	xor    %r9d,%r9d
   11427:	42 80 7c 07 02 c0    	cmpb   $0xc0,0x2(%rdi,%r8,1)
   1142d:	41 0f 9d c1          	setge  %r9b
   11431:	4c 01 c8             	add    %r9,%rax
   11434:	83 fa 03             	cmp    $0x3,%edx
   11437:	74 51                	je     1148a <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x20a>
   11439:	45 31 c9             	xor    %r9d,%r9d
   1143c:	42 80 7c 07 03 c0    	cmpb   $0xc0,0x3(%rdi,%r8,1)
   11442:	41 0f 9d c1          	setge  %r9b
   11446:	4c 01 c8             	add    %r9,%rax
   11449:	83 fa 04             	cmp    $0x4,%edx
   1144c:	74 3c                	je     1148a <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x20a>
   1144e:	45 31 c9             	xor    %r9d,%r9d
   11451:	42 80 7c 07 04 c0    	cmpb   $0xc0,0x4(%rdi,%r8,1)
   11457:	41 0f 9d c1          	setge  %r9b
   1145b:	4c 01 c8             	add    %r9,%rax
   1145e:	83 fa 05             	cmp    $0x5,%edx
   11461:	74 27                	je     1148a <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x20a>
   11463:	45 31 c9             	xor    %r9d,%r9d
   11466:	42 80 7c 07 05 c0    	cmpb   $0xc0,0x5(%rdi,%r8,1)
   1146c:	41 0f 9d c1          	setge  %r9b
   11470:	4c 01 c8             	add    %r9,%rax
   11473:	83 fa 06             	cmp    $0x6,%edx
   11476:	74 12                	je     1148a <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x20a>
   11478:	31 d2                	xor    %edx,%edx
   1147a:	42 80 7c 07 06 c0    	cmpb   $0xc0,0x6(%rdi,%r8,1)
   11480:	0f 9d c2             	setge  %dl
   11483:	48 01 d0             	add    %rdx,%rax
   11486:	eb 02                	jmp    1148a <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x20a>
   11488:	31 c0                	xor    %eax,%eax
   1148a:	48 01 f0             	add    %rsi,%rax
   1148d:	48 c1 e9 03          	shr    $0x3,%rcx
   11491:	49 b8 01 01 01 01 01 	movabs $0x101010101010101,%r8
   11498:	01 01 01 
   1149b:	48 be ff 00 ff 00 ff 	movabs $0xff00ff00ff00ff,%rsi
   114a2:	00 ff 00 
   114a5:	48 ba 01 00 01 00 01 	movabs $0x1000100010001,%rdx
   114ac:	00 01 00 
   114af:	eb 42                	jmp    114f3 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x273>
   114b1:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   114b8:	0f 1f 84 00 00 00 00 
   114bf:	00 
   114c0:	31 db                	xor    %ebx,%ebx
   114c2:	4c 01 cf             	add    %r9,%rdi
   114c5:	4c 29 d1             	sub    %r10,%rcx
   114c8:	45 89 d3             	mov    %r10d,%r11d
   114cb:	41 83 e3 03          	and    $0x3,%r11d
   114cf:	49 89 de             	mov    %rbx,%r14
   114d2:	49 21 f6             	and    %rsi,%r14
   114d5:	48 c1 eb 08          	shr    $0x8,%rbx
   114d9:	48 21 f3             	and    %rsi,%rbx
   114dc:	4c 01 f3             	add    %r14,%rbx
   114df:	48 0f af da          	imul   %rdx,%rbx
   114e3:	48 c1 eb 30          	shr    $0x30,%rbx
   114e7:	48 01 d8             	add    %rbx,%rax
   114ea:	4d 85 db             	test   %r11,%r11
   114ed:	0f 85 b6 00 00 00    	jne    115a9 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x329>
   114f3:	48 85 c9             	test   %rcx,%rcx
   114f6:	0f 84 2e 01 00 00    	je     1162a <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x3aa>
   114fc:	49 89 f9             	mov    %rdi,%r9
   114ff:	48 81 f9 c0 00 00 00 	cmp    $0xc0,%rcx
   11506:	41 ba c0 00 00 00    	mov    $0xc0,%r10d
   1150c:	4c 0f 42 d1          	cmovb  %rcx,%r10
   11510:	42 8d 3c d5 00 00 00 	lea    0x0(,%r10,8),%edi
   11517:	00 
   11518:	41 89 fb             	mov    %edi,%r11d
   1151b:	41 81 e3 e0 07 00 00 	and    $0x7e0,%r11d
   11522:	74 9c                	je     114c0 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x240>
   11524:	4d 01 cb             	add    %r9,%r11
   11527:	31 db                	xor    %ebx,%ebx
   11529:	4d 89 ce             	mov    %r9,%r14
   1152c:	0f 1f 40 00          	nopl   0x0(%rax)
   11530:	4d 8b 3e             	mov    (%r14),%r15
   11533:	4d 8b 66 08          	mov    0x8(%r14),%r12
   11537:	4d 89 fd             	mov    %r15,%r13
   1153a:	49 f7 d5             	not    %r13
   1153d:	49 c1 ed 07          	shr    $0x7,%r13
   11541:	49 c1 ef 06          	shr    $0x6,%r15
   11545:	4d 09 ef             	or     %r13,%r15
   11548:	4d 21 c7             	and    %r8,%r15
   1154b:	49 01 df             	add    %rbx,%r15
   1154e:	4c 89 e3             	mov    %r12,%rbx
   11551:	48 f7 d3             	not    %rbx
   11554:	48 c1 eb 07          	shr    $0x7,%rbx
   11558:	49 c1 ec 06          	shr    $0x6,%r12
   1155c:	49 09 dc             	or     %rbx,%r12
   1155f:	4d 21 c4             	and    %r8,%r12
   11562:	4d 8b 6e 10          	mov    0x10(%r14),%r13
   11566:	4c 89 eb             	mov    %r13,%rbx
   11569:	48 f7 d3             	not    %rbx
   1156c:	48 c1 eb 07          	shr    $0x7,%rbx
   11570:	49 c1 ed 06          	shr    $0x6,%r13
   11574:	49 09 dd             	or     %rbx,%r13
   11577:	4d 21 c5             	and    %r8,%r13
   1157a:	4d 01 e5             	add    %r12,%r13
   1157d:	4d 01 fd             	add    %r15,%r13
   11580:	49 8b 5e 18          	mov    0x18(%r14),%rbx
   11584:	49 89 df             	mov    %rbx,%r15
   11587:	49 f7 d7             	not    %r15
   1158a:	49 c1 ef 07          	shr    $0x7,%r15
   1158e:	48 c1 eb 06          	shr    $0x6,%rbx
   11592:	4c 09 fb             	or     %r15,%rbx
   11595:	4c 21 c3             	and    %r8,%rbx
   11598:	4c 01 eb             	add    %r13,%rbx
   1159b:	49 83 c6 20          	add    $0x20,%r14
   1159f:	4d 39 de             	cmp    %r11,%r14
   115a2:	75 8c                	jne    11530 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x2b0>
   115a4:	e9 19 ff ff ff       	jmp    114c2 <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x242>
   115a9:	41 81 e2 fc 00 00 00 	and    $0xfc,%r10d
   115b0:	44 89 d1             	mov    %r10d,%ecx
   115b3:	49 8b 0c c9          	mov    (%r9,%rcx,8),%rcx
   115b7:	48 89 cf             	mov    %rcx,%rdi
   115ba:	48 f7 d7             	not    %rdi
   115bd:	48 c1 ef 07          	shr    $0x7,%rdi
   115c1:	48 c1 e9 06          	shr    $0x6,%rcx
   115c5:	48 09 f9             	or     %rdi,%rcx
   115c8:	4c 21 c1             	and    %r8,%rcx
   115cb:	41 83 fb 01          	cmp    $0x1,%r11d
   115cf:	74 3e                	je     1160f <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x38f>
   115d1:	4b 8b 7c d1 08       	mov    0x8(%r9,%r10,8),%rdi
   115d6:	48 89 fb             	mov    %rdi,%rbx
   115d9:	48 f7 d3             	not    %rbx
   115dc:	48 c1 eb 07          	shr    $0x7,%rbx
   115e0:	48 c1 ef 06          	shr    $0x6,%rdi
   115e4:	48 09 df             	or     %rbx,%rdi
   115e7:	4c 21 c7             	and    %r8,%rdi
   115ea:	48 01 f9             	add    %rdi,%rcx
   115ed:	41 83 fb 02          	cmp    $0x2,%r11d
   115f1:	74 1c                	je     1160f <_RNvNtNtCsfJBMPiLOdLr_4core3str5count14do_count_chars+0x38f>
   115f3:	4b 8b 7c d1 10       	mov    0x10(%r9,%r10,8),%rdi
   115f8:	49 89 f9             	mov    %rdi,%r9
   115fb:	49 f7 d1             	not    %r9
   115fe:	49 c1 e9 07          	shr    $0x7,%r9
   11602:	48 c1 ef 06          	shr    $0x6,%rdi
   11606:	4c 09 cf             	or     %r9,%rdi
   11609:	4c 21 c7             	and    %r8,%rdi
   1160c:	48 01 f9             	add    %rdi,%rcx
   1160f:	48 89 cf             	mov    %rcx,%rdi
   11612:	48 21 f7             	and    %rsi,%rdi
   11615:	48 c1 e9 08          	shr    $0x8,%rcx
   11619:	48 21 f1             	and    %rsi,%rcx
   1161c:	48 01 f9             	add    %rdi,%rcx
   1161f:	48 0f af ca          	imul   %rdx,%rcx
   11623:	48 c1 e9 30          	shr    $0x30,%rcx
   11627:	48 01 c8             	add    %rcx,%rax
   1162a:	5b                   	pop    %rbx
   1162b:	41 5c                	pop    %r12
   1162d:	41 5d                	pop    %r13
   1162f:	41 5e                	pop    %r14
   11631:	41 5f                	pop    %r15
   11633:	c3                   	ret
   11634:	cc                   	int3
   11635:	cc                   	int3
   11636:	cc                   	int3
   11637:	cc                   	int3
   11638:	cc                   	int3
   11639:	cc                   	int3
   1163a:	cc                   	int3
   1163b:	cc                   	int3
   1163c:	cc                   	int3
   1163d:	cc                   	int3
   1163e:	cc                   	int3
   1163f:	cc                   	int3

0000000000011640 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8>:
   11640:	41 57                	push   %r15
   11642:	41 56                	push   %r14
   11644:	53                   	push   %rbx
   11645:	48 89 f8             	mov    %rdi,%rax
   11648:	31 ff                	xor    %edi,%edi
   1164a:	48 89 d1             	mov    %rdx,%rcx
   1164d:	48 83 e9 0f          	sub    $0xf,%rcx
   11651:	48 0f 43 f9          	cmovae %rcx,%rdi
   11655:	48 85 d2             	test   %rdx,%rdx
   11658:	0f 84 a9 01 00 00    	je     11807 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1c7>
   1165e:	4c 8d 46 07          	lea    0x7(%rsi),%r8
   11662:	49 83 e0 f8          	and    $0xfffffffffffffff8,%r8
   11666:	49 29 f0             	sub    %rsi,%r8
   11669:	31 c9                	xor    %ecx,%ecx
   1166b:	4c 8d 15 ca 08 00 00 	lea    0x8ca(%rip),%r10        # 11f3c <memcpy+0x4dc>
   11672:	49 bb 80 80 80 80 80 	movabs $0x8080808080808080,%r11
   11679:	80 80 80 
   1167c:	eb 11                	jmp    1168f <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x4f>
   1167e:	66 90                	xchg   %ax,%ax
   11680:	49 ff c6             	inc    %r14
   11683:	4c 89 f1             	mov    %r14,%rcx
   11686:	48 39 d1             	cmp    %rdx,%rcx
   11689:	0f 83 78 01 00 00    	jae    11807 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1c7>
   1168f:	0f b6 1c 0e          	movzbl (%rsi,%rcx,1),%ebx
   11693:	84 db                	test   %bl,%bl
   11695:	78 59                	js     116f0 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0xb0>
   11697:	45 89 c1             	mov    %r8d,%r9d
   1169a:	41 29 c9             	sub    %ecx,%r9d
   1169d:	41 f6 c1 07          	test   $0x7,%r9b
   116a1:	74 11                	je     116b4 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x74>
   116a3:	48 ff c1             	inc    %rcx
   116a6:	eb de                	jmp    11686 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x46>
   116a8:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   116af:	00 
   116b0:	48 83 c1 10          	add    $0x10,%rcx
   116b4:	48 39 f9             	cmp    %rdi,%rcx
   116b7:	73 0e                	jae    116c7 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x87>
   116b9:	4c 8b 4c 0e 08       	mov    0x8(%rsi,%rcx,1),%r9
   116be:	4c 0b 0c 0e          	or     (%rsi,%rcx,1),%r9
   116c2:	4d 85 d9             	test   %r11,%r9
   116c5:	74 e9                	je     116b0 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x70>
   116c7:	48 39 d1             	cmp    %rdx,%rcx
   116ca:	73 ba                	jae    11686 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x46>
   116cc:	0f 1f 40 00          	nopl   0x0(%rax)
   116d0:	80 3c 0e 00          	cmpb   $0x0,(%rsi,%rcx,1)
   116d4:	78 b0                	js     11686 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x46>
   116d6:	48 ff c1             	inc    %rcx
   116d9:	48 39 ca             	cmp    %rcx,%rdx
   116dc:	75 f2                	jne    116d0 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x90>
   116de:	e9 24 01 00 00       	jmp    11807 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1c7>
   116e3:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   116ea:	84 00 00 00 00 00 
   116f0:	46 0f b6 34 13       	movzbl (%rbx,%r10,1),%r14d
   116f5:	41 b1 01             	mov    $0x1,%r9b
   116f8:	41 83 fe 04          	cmp    $0x4,%r14d
   116fc:	74 5d                	je     1175b <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x11b>
   116fe:	41 83 fe 03          	cmp    $0x3,%r14d
   11702:	74 29                	je     1172d <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0xed>
   11704:	41 83 fe 02          	cmp    $0x2,%r14d
   11708:	0f 85 0a 01 00 00    	jne    11818 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1d8>
   1170e:	4c 8d 71 01          	lea    0x1(%rcx),%r14
   11712:	49 39 d6             	cmp    %rdx,%r14
   11715:	0f 83 f8 00 00 00    	jae    11813 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1d3>
   1171b:	42 80 3c 36 bf       	cmpb   $0xbf,(%rsi,%r14,1)
   11720:	b3 01                	mov    $0x1,%bl
   11722:	0f 8e 58 ff ff ff    	jle    11680 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x40>
   11728:	e9 f1 00 00 00       	jmp    1181e <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1de>
   1172d:	4c 8d 71 01          	lea    0x1(%rcx),%r14
   11731:	49 39 d6             	cmp    %rdx,%r14
   11734:	0f 83 d9 00 00 00    	jae    11813 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1d3>
   1173a:	46 0f b6 34 36       	movzbl (%rsi,%r14,1),%r14d
   1173f:	48 81 fb e0 00 00 00 	cmp    $0xe0,%rbx
   11746:	74 41                	je     11789 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x149>
   11748:	81 fb ed 00 00 00    	cmp    $0xed,%ebx
   1174e:	75 54                	jne    117a4 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x164>
   11750:	41 80 fe 9f          	cmp    $0x9f,%r14b
   11754:	7e 66                	jle    117bc <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x17c>
   11756:	e9 bd 00 00 00       	jmp    11818 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1d8>
   1175b:	4c 8d 71 01          	lea    0x1(%rcx),%r14
   1175f:	49 39 d6             	cmp    %rdx,%r14
   11762:	0f 83 ab 00 00 00    	jae    11813 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1d3>
   11768:	46 0f b6 34 36       	movzbl (%rsi,%r14,1),%r14d
   1176d:	48 81 fb f0 00 00 00 	cmp    $0xf0,%rbx
   11774:	74 22                	je     11798 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x158>
   11776:	81 fb f4 00 00 00    	cmp    $0xf4,%ebx
   1177c:	75 54                	jne    117d2 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x192>
   1177e:	41 80 fe 8f          	cmp    $0x8f,%r14b
   11782:	7e 5c                	jle    117e0 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1a0>
   11784:	e9 8f 00 00 00       	jmp    11818 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1d8>
   11789:	41 80 e6 e0          	and    $0xe0,%r14b
   1178d:	41 80 fe a0          	cmp    $0xa0,%r14b
   11791:	74 29                	je     117bc <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x17c>
   11793:	e9 80 00 00 00       	jmp    11818 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1d8>
   11798:	41 80 c6 70          	add    $0x70,%r14b
   1179c:	41 80 fe 30          	cmp    $0x30,%r14b
   117a0:	72 3e                	jb     117e0 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1a0>
   117a2:	eb 74                	jmp    11818 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1d8>
   117a4:	44 8d 7b 1f          	lea    0x1f(%rbx),%r15d
   117a8:	41 80 ff 0c          	cmp    $0xc,%r15b
   117ac:	72 08                	jb     117b6 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x176>
   117ae:	80 e3 fe             	and    $0xfe,%bl
   117b1:	80 fb ee             	cmp    $0xee,%bl
   117b4:	75 62                	jne    11818 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1d8>
   117b6:	41 80 fe c0          	cmp    $0xc0,%r14b
   117ba:	7d 5c                	jge    11818 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1d8>
   117bc:	4c 8d 71 02          	lea    0x2(%rcx),%r14
   117c0:	49 39 d6             	cmp    %rdx,%r14
   117c3:	73 4e                	jae    11813 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1d3>
   117c5:	42 80 3c 36 bf       	cmpb   $0xbf,(%rsi,%r14,1)
   117ca:	0f 8e b0 fe ff ff    	jle    11680 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x40>
   117d0:	eb 4a                	jmp    1181c <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1dc>
   117d2:	80 c3 0f             	add    $0xf,%bl
   117d5:	80 fb 02             	cmp    $0x2,%bl
   117d8:	77 3e                	ja     11818 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1d8>
   117da:	41 80 fe c0          	cmp    $0xc0,%r14b
   117de:	7d 38                	jge    11818 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1d8>
   117e0:	48 8d 59 02          	lea    0x2(%rcx),%rbx
   117e4:	48 39 d3             	cmp    %rdx,%rbx
   117e7:	73 2a                	jae    11813 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1d3>
   117e9:	80 3c 1e bf          	cmpb   $0xbf,(%rsi,%rbx,1)
   117ed:	7f 2d                	jg     1181c <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1dc>
   117ef:	4c 8d 71 03          	lea    0x3(%rcx),%r14
   117f3:	49 39 d6             	cmp    %rdx,%r14
   117f6:	73 1b                	jae    11813 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1d3>
   117f8:	42 80 3c 36 c0       	cmpb   $0xc0,(%rsi,%r14,1)
   117fd:	0f 8c 7d fe ff ff    	jl     11680 <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x40>
   11803:	b3 03                	mov    $0x3,%bl
   11805:	eb 17                	jmp    1181e <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1de>
   11807:	48 89 70 08          	mov    %rsi,0x8(%rax)
   1180b:	48 89 50 10          	mov    %rdx,0x10(%rax)
   1180f:	31 c9                	xor    %ecx,%ecx
   11811:	eb 1b                	jmp    1182e <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1ee>
   11813:	45 31 c9             	xor    %r9d,%r9d
   11816:	eb 06                	jmp    1181e <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1de>
   11818:	b3 01                	mov    $0x1,%bl
   1181a:	eb 02                	jmp    1181e <_RNvNtNtCsfJBMPiLOdLr_4core3str8converts9from_utf8+0x1de>
   1181c:	b3 02                	mov    $0x2,%bl
   1181e:	48 89 48 08          	mov    %rcx,0x8(%rax)
   11822:	44 88 48 10          	mov    %r9b,0x10(%rax)
   11826:	88 58 11             	mov    %bl,0x11(%rax)
   11829:	b9 01 00 00 00       	mov    $0x1,%ecx
   1182e:	48 89 08             	mov    %rcx,(%rax)
   11831:	5b                   	pop    %rbx
   11832:	41 5e                	pop    %r14
   11834:	41 5f                	pop    %r15
   11836:	c3                   	ret
   11837:	cc                   	int3
   11838:	cc                   	int3
   11839:	cc                   	int3
   1183a:	cc                   	int3
   1183b:	cc                   	int3
   1183c:	cc                   	int3
   1183d:	cc                   	int3
   1183e:	cc                   	int3
   1183f:	cc                   	int3

0000000000011840 <_RNvNtNtCsfJBMPiLOdLr_4core5slice5index16slice_index_fail>:
   11840:	55                   	push   %rbp
   11841:	48 89 e5             	mov    %rsp,%rbp
   11844:	48 83 ec 30          	sub    $0x30,%rsp
   11848:	48 39 d7             	cmp    %rdx,%rdi
   1184b:	76 3b                	jbe    11888 <_RNvNtNtCsfJBMPiLOdLr_4core5slice5index16slice_index_fail+0x48>
   1184d:	48 89 7d f8          	mov    %rdi,-0x8(%rbp)
   11851:	48 89 55 f0          	mov    %rdx,-0x10(%rbp)
   11855:	48 8d 45 f8          	lea    -0x8(%rbp),%rax
   11859:	48 89 45 d0          	mov    %rax,-0x30(%rbp)
   1185d:	48 8b 05 04 08 00 00 	mov    0x804(%rip),%rax        # 12068 <memcpy+0x608>
   11864:	48 89 45 d8          	mov    %rax,-0x28(%rbp)
   11868:	48 8d 55 f0          	lea    -0x10(%rbp),%rdx
   1186c:	48 89 55 e0          	mov    %rdx,-0x20(%rbp)
   11870:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
   11874:	48 8d 3d 4e 03 00 00 	lea    0x34e(%rip),%rdi        # 11bc9 <memcpy+0x169>
   1187b:	48 8d 75 d0          	lea    -0x30(%rbp),%rsi
   1187f:	48 89 ca             	mov    %rcx,%rdx
   11882:	ff 15 d8 07 00 00    	call   *0x7d8(%rip)        # 12060 <memcpy+0x600>
   11888:	48 39 d6             	cmp    %rdx,%rsi
   1188b:	77 40                	ja     118cd <_RNvNtNtCsfJBMPiLOdLr_4core5slice5index16slice_index_fail+0x8d>
   1188d:	48 39 f7             	cmp    %rsi,%rdi
   11890:	76 3b                	jbe    118cd <_RNvNtNtCsfJBMPiLOdLr_4core5slice5index16slice_index_fail+0x8d>
   11892:	48 89 7d f8          	mov    %rdi,-0x8(%rbp)
   11896:	48 89 75 f0          	mov    %rsi,-0x10(%rbp)
   1189a:	48 8d 45 f8          	lea    -0x8(%rbp),%rax
   1189e:	48 89 45 d0          	mov    %rax,-0x30(%rbp)
   118a2:	48 8b 05 bf 07 00 00 	mov    0x7bf(%rip),%rax        # 12068 <memcpy+0x608>
   118a9:	48 89 45 d8          	mov    %rax,-0x28(%rbp)
   118ad:	48 8d 55 f0          	lea    -0x10(%rbp),%rdx
   118b1:	48 89 55 e0          	mov    %rdx,-0x20(%rbp)
   118b5:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
   118b9:	48 8d 3d d4 01 00 00 	lea    0x1d4(%rip),%rdi        # 11a94 <memcpy+0x34>
   118c0:	48 8d 75 d0          	lea    -0x30(%rbp),%rsi
   118c4:	48 89 ca             	mov    %rcx,%rdx
   118c7:	ff 15 93 07 00 00    	call   *0x793(%rip)        # 12060 <memcpy+0x600>
   118cd:	48 89 75 f8          	mov    %rsi,-0x8(%rbp)
   118d1:	48 89 55 f0          	mov    %rdx,-0x10(%rbp)
   118d5:	48 8d 45 f8          	lea    -0x8(%rbp),%rax
   118d9:	48 89 45 d0          	mov    %rax,-0x30(%rbp)
   118dd:	48 8b 05 84 07 00 00 	mov    0x784(%rip),%rax        # 12068 <memcpy+0x608>
   118e4:	48 89 45 d8          	mov    %rax,-0x28(%rbp)
   118e8:	48 8d 55 f0          	lea    -0x10(%rbp),%rdx
   118ec:	48 89 55 e0          	mov    %rdx,-0x20(%rbp)
   118f0:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
   118f4:	48 8d 3d 7d 02 00 00 	lea    0x27d(%rip),%rdi        # 11b78 <memcpy+0x118>
   118fb:	48 8d 75 d0          	lea    -0x30(%rbp),%rsi
   118ff:	48 89 ca             	mov    %rcx,%rdx
   11902:	ff 15 58 07 00 00    	call   *0x758(%rip)        # 12060 <memcpy+0x600>
   11908:	cc                   	int3
   11909:	cc                   	int3
   1190a:	cc                   	int3
   1190b:	cc                   	int3
   1190c:	cc                   	int3
   1190d:	cc                   	int3
   1190e:	cc                   	int3
   1190f:	cc                   	int3

0000000000011910 <_RNvNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB7_9Formatter12pad_integral12write_prefix>:
   11910:	55                   	push   %rbp
   11911:	48 89 e5             	mov    %rsp,%rbp
   11914:	41 57                	push   %r15
   11916:	41 56                	push   %r14
   11918:	41 54                	push   %r12
   1191a:	53                   	push   %rbx
   1191b:	4c 89 c3             	mov    %r8,%rbx
   1191e:	49 89 ce             	mov    %rcx,%r14
   11921:	49 89 f7             	mov    %rsi,%r15
   11924:	81 fa 00 00 11 00    	cmp    $0x110000,%edx
   1192a:	74 14                	je     11940 <_RNvNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB7_9Formatter12pad_integral12write_prefix+0x30>
   1192c:	49 89 fc             	mov    %rdi,%r12
   1192f:	89 d6                	mov    %edx,%esi
   11931:	41 ff 57 20          	call   *0x20(%r15)
   11935:	4c 89 e7             	mov    %r12,%rdi
   11938:	89 c1                	mov    %eax,%ecx
   1193a:	b0 01                	mov    $0x1,%al
   1193c:	84 c9                	test   %cl,%cl
   1193e:	75 1b                	jne    1195b <_RNvNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB7_9Formatter12pad_integral12write_prefix+0x4b>
   11940:	4d 85 f6             	test   %r14,%r14
   11943:	74 14                	je     11959 <_RNvNvMsa_NtCsfJBMPiLOdLr_4core3fmtNtB7_9Formatter12pad_integral12write_prefix+0x49>
   11945:	49 8b 47 18          	mov    0x18(%r15),%rax
   11949:	4c 89 f6             	mov    %r14,%rsi
   1194c:	48 89 da             	mov    %rbx,%rdx
   1194f:	5b                   	pop    %rbx
   11950:	41 5c                	pop    %r12
   11952:	41 5e                	pop    %r14
   11954:	41 5f                	pop    %r15
   11956:	5d                   	pop    %rbp
   11957:	ff e0                	jmp    *%rax
   11959:	31 c0                	xor    %eax,%eax
   1195b:	5b                   	pop    %rbx
   1195c:	41 5c                	pop    %r12
   1195e:	41 5e                	pop    %r14
   11960:	41 5f                	pop    %r15
   11962:	5d                   	pop    %rbp
   11963:	c3                   	ret
   11964:	cc                   	int3
   11965:	cc                   	int3
   11966:	cc                   	int3
   11967:	cc                   	int3
   11968:	cc                   	int3
   11969:	cc                   	int3
   1196a:	cc                   	int3
   1196b:	cc                   	int3
   1196c:	cc                   	int3
   1196d:	cc                   	int3
   1196e:	cc                   	int3
   1196f:	cc                   	int3

0000000000011970 <_RNvXsd_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impyNtB9_7Display3fmt>:
   11970:	55                   	push   %rbp
   11971:	48 89 e5             	mov    %rsp,%rbp
   11974:	41 56                	push   %r14
   11976:	53                   	push   %rbx
   11977:	48 83 ec 20          	sub    $0x20,%rsp
   1197b:	48 89 f3             	mov    %rsi,%rbx
   1197e:	48 8b 3f             	mov    (%rdi),%rdi
   11981:	48 8d 75 dc          	lea    -0x24(%rbp),%rsi
   11985:	41 be 14 00 00 00    	mov    $0x14,%r14d
   1198b:	ba 14 00 00 00       	mov    $0x14,%edx
   11990:	e8 fb f4 ff ff       	call   10e90 <_RNvMsf_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impy10__fmt_inner>
   11995:	49 29 c6             	sub    %rax,%r14
   11998:	4c 8d 04 28          	lea    (%rax,%rbp,1),%r8
   1199c:	49 83 c0 dc          	add    $0xffffffffffffffdc,%r8
   119a0:	ba 01 00 00 00       	mov    $0x1,%edx
   119a5:	48 89 df             	mov    %rbx,%rdi
   119a8:	be 01 00 00 00       	mov    $0x1,%esi
   119ad:	31 c9                	xor    %ecx,%ecx
   119af:	4d 89 f1             	mov    %r14,%r9
   119b2:	ff 15 b8 06 00 00    	call   *0x6b8(%rip)        # 12070 <memcpy+0x610>
   119b8:	48 83 c4 20          	add    $0x20,%rsp
   119bc:	5b                   	pop    %rbx
   119bd:	41 5e                	pop    %r14
   119bf:	5d                   	pop    %rbp
   119c0:	c3                   	ret
   119c1:	cc                   	int3
   119c2:	cc                   	int3
   119c3:	cc                   	int3
   119c4:	cc                   	int3
   119c5:	cc                   	int3
   119c6:	cc                   	int3
   119c7:	cc                   	int3
   119c8:	cc                   	int3
   119c9:	cc                   	int3
   119ca:	cc                   	int3
   119cb:	cc                   	int3
   119cc:	cc                   	int3
   119cd:	cc                   	int3
   119ce:	cc                   	int3
   119cf:	cc                   	int3

00000000000119d0 <_RNvXse_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impxNtB9_7Display3fmt>:
   119d0:	55                   	push   %rbp
   119d1:	48 89 e5             	mov    %rsp,%rbp
   119d4:	41 57                	push   %r15
   119d6:	41 56                	push   %r14
   119d8:	53                   	push   %rbx
   119d9:	48 83 ec 18          	sub    $0x18,%rsp
   119dd:	48 8b 07             	mov    (%rdi),%rax
   119e0:	48 89 c7             	mov    %rax,%rdi
   119e3:	48 f7 df             	neg    %rdi
   119e6:	48 0f 48 f8          	cmovs  %rax,%rdi
   119ea:	48 89 f3             	mov    %rsi,%rbx
   119ed:	45 31 f6             	xor    %r14d,%r14d
   119f0:	48 85 c0             	test   %rax,%rax
   119f3:	41 0f 99 c6          	setns  %r14b
   119f7:	48 8d 75 d4          	lea    -0x2c(%rbp),%rsi
   119fb:	41 bf 14 00 00 00    	mov    $0x14,%r15d
   11a01:	ba 14 00 00 00       	mov    $0x14,%edx
   11a06:	e8 85 f4 ff ff       	call   10e90 <_RNvMsf_NtNtNtCsfJBMPiLOdLr_4core3fmt3num3impy10__fmt_inner>
   11a0b:	49 29 c7             	sub    %rax,%r15
   11a0e:	4c 8d 04 28          	lea    (%rax,%rbp,1),%r8
   11a12:	49 83 c0 d4          	add    $0xffffffffffffffd4,%r8
   11a16:	ba 01 00 00 00       	mov    $0x1,%edx
   11a1b:	48 89 df             	mov    %rbx,%rdi
   11a1e:	44 89 f6             	mov    %r14d,%esi
   11a21:	31 c9                	xor    %ecx,%ecx
   11a23:	4d 89 f9             	mov    %r15,%r9
   11a26:	ff 15 44 06 00 00    	call   *0x644(%rip)        # 12070 <memcpy+0x610>
   11a2c:	48 83 c4 18          	add    $0x18,%rsp
   11a30:	5b                   	pop    %rbx
   11a31:	41 5e                	pop    %r14
   11a33:	41 5f                	pop    %r15
   11a35:	5d                   	pop    %rbp
   11a36:	c3                   	ret
   11a37:	cc                   	int3
   11a38:	cc                   	int3
   11a39:	cc                   	int3
   11a3a:	cc                   	int3
   11a3b:	cc                   	int3
   11a3c:	cc                   	int3
   11a3d:	cc                   	int3
   11a3e:	cc                   	int3
   11a3f:	cc                   	int3

0000000000011a40 <_RNvXsi_NtCsfJBMPiLOdLr_4core3fmteNtB5_7Display3fmt>:
   11a40:	48 89 d0             	mov    %rdx,%rax
   11a43:	48 89 f2             	mov    %rsi,%rdx
   11a46:	48 89 fe             	mov    %rdi,%rsi
   11a49:	48 89 c7             	mov    %rax,%rdi
   11a4c:	ff 25 2e 06 00 00    	jmp    *0x62e(%rip)        # 12080 <memcpy+0x620>
   11a52:	cc                   	int3
   11a53:	cc                   	int3
   11a54:	cc                   	int3
   11a55:	cc                   	int3
   11a56:	cc                   	int3
   11a57:	cc                   	int3
   11a58:	cc                   	int3
   11a59:	cc                   	int3
   11a5a:	cc                   	int3
   11a5b:	cc                   	int3
   11a5c:	cc                   	int3
   11a5d:	cc                   	int3
   11a5e:	cc                   	int3
   11a5f:	cc                   	int3

0000000000011a60 <memcpy>:
   11a60:	48 89 f8             	mov    %rdi,%rax
   11a63:	41 89 c0             	mov    %eax,%r8d
   11a66:	41 f7 d8             	neg    %r8d
   11a69:	41 83 e0 07          	and    $0x7,%r8d
   11a6d:	4c 39 c2             	cmp    %r8,%rdx
   11a70:	4c 0f 42 c2          	cmovb  %rdx,%r8
   11a74:	4c 89 c1             	mov    %r8,%rcx
   11a77:	f3 a4                	rep movsb (%rsi),(%rdi)
   11a79:	4c 29 c2             	sub    %r8,%rdx
   11a7c:	48 89 d1             	mov    %rdx,%rcx
   11a7f:	48 c1 e9 03          	shr    $0x3,%rcx
   11a83:	f3 48 a5             	rep movsq (%rsi),(%rdi)
   11a86:	83 e2 07             	and    $0x7,%edx
   11a89:	48 89 d1             	mov    %rdx,%rcx
   11a8c:	f3 a4                	rep movsb (%rsi),(%rdi)
   11a8e:	c3                   	ret
