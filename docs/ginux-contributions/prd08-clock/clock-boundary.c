typedef unsigned long word;
struct ts {long sec,nsec;};
static long call(long n,word a,word b,word c,word d) {
 register word r10 __asm__("r10")=d;long v;
 __asm__ volatile("syscall":"=a"(v):"a"(n),"D"(a),"S"(b),"d"(c),"r"(r10):"rcx","r11","memory");return v;
}
static void text(const char *s) {word n=0;while(s[n])++n;call(1,1,(word)s,n,0);}
static void number(long v) {char b[32];word n=0;if(v<0){text("-");v=-v;}do{b[n++]=(char)('0'+v%10);v/=10;}while(v);while(n)call(1,1,(word)&b[--n],1,0);}
static word ns(struct ts t){return (word)t.sec*1000000000UL+t.nsec;}
long clock_main(word *stack) {
 char mode=stack[0]>1?((char *)stack[2])[0]:'?';
 struct ts start,real,req,end,after,bad={300,1};
 if(call(35,(word)&bad,0,0,0)!=-22)return 10;
 if(call(228,1,(word)&start,0,0)||call(228,0,(word)&real,0,0))return 11;
 req=real;req.sec+=120;
 if(mode=='F'){req.sec=300;req.nsec=0;}
 text("CLOCK ready mode=");call(1,1,(word)&mode,1,0);text(" mono_ns=");number(ns(start));text(" real_ns=");number(ns(real));text(" target_ns=");number(ns(req));text("\n");
 long result=mode=='F'?call(35,(word)&req,0,0,0):call(230,0,1,(word)&req,0);
 if(call(228,1,(word)&end,0,0)||call(228,0,(word)&after,0,0))return 12;
 word elapsed=ns(end)-ns(start);
 text("CLOCK result mode=");call(1,1,(word)&mode,1,0);text(" result=");number(result);text(" elapsed_ns=");number(elapsed);text(" real_ns=");number(ns(after));text("\n");
 if(mode=='F')return result==0 && elapsed>=300000000000UL && elapsed<303000000000UL?0:13;
 if(mode=='B')return result==-110 && elapsed>=300000000000UL && elapsed<303000000000UL && ns(after)<ns(req)?0:14;
 if(mode=='A')return result==0 && elapsed<5000000000UL && ns(after)>=ns(req)?0:15;
 return 16;
}
__asm__(".global _start\n_start:\nmov %rsp,%rdi\nand $-16,%rsp\ncall clock_main\nmov %rax,%rdi\nmov $60,%rax\nsyscall\nud2\n");
