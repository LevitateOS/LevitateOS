#define _GNU_SOURCE
#include <sys/mount.h>
#include <sys/wait.h>
#include <sys/reboot.h>
#include <sys/stat.h>
#include <poll.h>
#include <time.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
struct child {pid_t pid;int out,err,done,started,adjusted,status;char mode;char audit[65536];size_t used;};
static double mono(void){struct timespec t;clock_gettime(CLOCK_MONOTONIC,&t);return t.tv_sec+t.tv_nsec/1e9;}
static void fail(const char *s){printf("PRD08_VM FAIL %s errno=%d\n",s,errno);fflush(stdout);reboot(RB_POWER_OFF);for(;;)pause();}
static struct child spawn(char mode){
 int a[2],b[2];if(pipe(a)||pipe(b))fail("pipe");char config[32],result[32];snprintf(config,sizeof config,"/config-%c.toml",mode);snprintf(result,sizeof result,"/result-%c.json",mode);
 pid_t p=fork();if(p<0)fail("fork");if(!p){dup2(a[1],1);dup2(b[1],2);for(int fd=3;fd<256;++fd)close(fd);execl("/ginux","ginux","launch",config,"/rootfs","--result-file",result,(char*)0);_exit(126);}
 close(a[1]);close(b[1]);fcntl(a[0],F_SETFL,O_NONBLOCK);fcntl(b[0],F_SETFL,O_NONBLOCK);
 return (struct child){.pid=p,.out=a[0],.err=b[0],.mode=mode};
}
static void step(struct child *c){
 struct timespec old,next;clock_gettime(CLOCK_REALTIME,&old);next=old;next.tv_sec+=c->mode=='A'?240:-600;
 if(clock_settime(CLOCK_REALTIME,&next))fail("guest clock_settime");
 printf("PRD08_VM STEP mode=%c delta=%d before=%ld.%09ld after=%ld.%09ld mono=%.9f active_sleep=confirmed\n",c->mode,c->mode=='A'?240:-600,old.tv_sec,old.tv_nsec,next.tv_sec,next.tv_nsec,mono());fflush(stdout);c->adjusted=1;
}
static void pump(struct child *c){
 char buf[8192];ssize_t n;
 while((n=read(c->out,buf,sizeof buf))>0){fwrite(buf,1,n,stdout);fflush(stdout);}
 while((n=read(c->err,buf,sizeof buf))>0){
  size_t take=(size_t)n;if(take>sizeof c->audit-1-c->used)take=sizeof c->audit-1-c->used;
  memcpy(c->audit+c->used,buf,take);c->used+=take;c->audit[c->used]=0;
 }
 if(!c->started && strstr(c->audit,"surface=configured-global-clock-sleep")){c->started=1;printf("PRD08_VM ACTIVE mode=%c mono=%.9f\n",c->mode,mono());fflush(stdout);}
 if(!c->adjusted && (c->mode=='A'||c->mode=='B') && c->started){struct timespec t={0,200000000};nanosleep(&t,0);step(c);}
 if(!c->done && waitpid(c->pid,&c->status,WNOHANG)==c->pid){
  c->done=1;printf("PRD08_VM EXIT mode=%c status=%d mono=%.9f audit_bytes=%zu\n",c->mode,c->status,mono(),c->used);
  char path[32];snprintf(path,sizeof path,"/result-%c.json",c->mode);int f=open(path,O_RDONLY);n=f<0?-1:read(f,buf,sizeof buf-1);if(f>=0)close(f);if(n<0)fail("missing result");buf[n]=0;printf("PRD08_VM RESULT mode=%c %s\n",c->mode,buf);
  if(!WIFEXITED(c->status)||WEXITSTATUS(c->status)||!strstr(buf,"\"exited\"")||!strstr(buf,"\"code\":0")){fwrite(c->audit,1,c->used,stdout);fail("launcher result");}
  if((c->mode=='A'||c->mode=='B')&&!c->adjusted)fail("adjustment not observed");
  close(c->out);close(c->err);fflush(stdout);
 }
}
static void wait_all(struct child *c,int count){double deadline=mono()+330;for(;;){int done=0;for(int i=0;i<count;++i){pump(c+i);done+=c[i].done;}if(done==count)return;if(mono()>deadline)fail("outer monotonic timeout");struct timespec tick={0,10000000};nanosleep(&tick,0);}}
int main(void){
 if(getpid()!=1)return 125;
 mkdir("/proc",0755);if(mount("proc","/proc","proc",0,0))return 124;
 char cmd[4096];int f=open("/proc/cmdline",O_RDONLY);ssize_t n=read(f,cmd,sizeof cmd-1);close(f);if(n<=0)return 123;cmd[n]=0;if(!strstr(cmd,"prd08_isolated_clock_fixture=1"))return 122;
 f=open("/dev/console",O_RDWR);if(f>=0){dup2(f,0);dup2(f,1);dup2(f,2);if(f>2)close(f);}setvbuf(stdout,0,_IONBF,0);
 printf("PRD08_VM BEGIN pid=%d mono=%.9f guest_clock_only=1 network=none\n",getpid(),mono());
 struct child z=spawn('Z');wait_all(&z,1);
 struct child a=spawn('A');wait_all(&a,1);
 struct child pair[2]={spawn('F'),spawn('B')};wait_all(pair,2);
 puts("PRD08_VM PASS full_300=1 interrupted_relative_absolute=1 realtime_forward=1 realtime_backward_cap=1");sync();reboot(RB_POWER_OFF);return 0;
}
