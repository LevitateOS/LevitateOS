from pathlib import Path
import gzip,stat,hashlib,json,shutil
root=Path('/home/vince/Projects/LevitateOS');out=root/'.artifacts/ginux-contribution-preparation/prd08'
repo=Path('/home/vince/levitate-development/ginux-prd08')
entries=[]
for name in ['dev','proc','lib64','rootfs','rootfs/bin','rootfs/work','rootfs/dev']:entries.append((name,stat.S_IFDIR|0o755,b'',0,0))
entries += [('dev/console',stat.S_IFCHR|0o600,b'',5,1),('dev/null',stat.S_IFCHR|0o666,b'',1,3)]
files={'init':out/'vm-init','ginux':repo/'target/x86_64-unknown-linux-gnu/release/ginux','rootfs/bin/clock':out/'clock-boundary','rootfs/bin/tasks':repo/'.artifacts/launch/launch-task-fixture','lib64/libc.so.6':Path('/lib64/libc.so.6'),'lib64/ld-linux-x86-64.so.2':Path('/lib64/ld-linux-x86-64.so.2')}
for name,path in files.items():entries.append((name,stat.S_IFREG|0o755,path.read_bytes(),0,0))
template=(repo/'config/instance-configured.toml').read_text()
for mode in 'ZAFB':
 cfg=template.replace('init = "/bin/bash"','init = "/bin/tasks"' if mode=='Z' else 'init = "/bin/clock"').replace('args = []','args = ["Z-clocks"]' if mode=='Z' else f'args = ["{mode}"]')
 if mode=='Z':cfg=cfg.replace('[storage]\nassignments = []','[storage]\nassignments = ["memory:/work"]').replace('[devices]\nassignments = []','[devices]\nassignments = ["null"]')
 (out/f'config-{mode}.toml').write_text(cfg);entries.append((f'config-{mode}.toml',stat.S_IFREG|0o644,cfg.encode(),0,0))
buf=bytearray()
for ino,(name,mode,data,major,minor) in enumerate(entries+[('TRAILER!!!',0,b'',0,0)],1):
 name=name.encode()+b'\0';fields=[ino,mode,0,0,1,0,len(data),0,0,major,minor,len(name),0]
 buf += b'070701'+''.join(f'{v:08x}' for v in fields).encode()+name
 buf+=b'\0'*(-len(buf)%4);buf+=data;buf+=b'\0'*(-len(buf)%4)
(out/'clock-initramfs.cpio.gz').write_bytes(gzip.compress(bytes(buf),mtime=0))
kernel=Path('/boot/vmlinuz-6.12.0-211.74.1.el10_2.x86_64');shutil.copyfile(kernel,out/'vmlinuz')
xml=f'''<domain type="kvm">
<name>acorn-prd08-clocks</name><memory unit="MiB">512</memory><vcpu>2</vcpu>
<os><type arch="x86_64" machine="q35">hvm</type><kernel>{out}/vmlinuz</kernel><initrd>{out}/clock-initramfs.cpio.gz</initrd><cmdline>console=ttyS0 rdinit=/init panic=-1 loglevel=3 selinux=0 prd08_isolated_clock_fixture=1</cmdline></os>
<features><acpi/><apic/></features><cpu mode="host-passthrough"/><clock offset="utc"/>
<on_poweroff>destroy</on_poweroff><on_reboot>destroy</on_reboot><on_crash>destroy</on_crash>
<devices><emulator>/usr/local/bin/qemu-system-x86_64</emulator><serial type="file"><source path="{out}/vm-console.log"/><target port="0"/></serial></devices>
</domain>'''
(out/'clock-vm.xml').write_text(xml)
manifest={name:{'path':str(path),'sha256':hashlib.sha256(path.read_bytes()).hexdigest()} for name,path in files.items()}
manifest['kernel']={'path':str(kernel),'sha256':hashlib.sha256(kernel.read_bytes()).hexdigest()}
(out/'vm-inputs.json').write_text(json.dumps(manifest,indent=2)+'\n')
print('Prepared isolated VM:',out/'clock-vm.xml')
