"""Build a diskless session-libvirt recstrap fixture; never starts QEMU directly."""
from pathlib import Path
import argparse,gzip,hashlib,json,re,stat,subprocess

p=argparse.ArgumentParser()
p.add_argument('--output',type=Path,required=True)
p.add_argument('--image',type=Path,required=True)
p.add_argument('--recstrap',type=Path,required=True)
p.add_argument('--controller',type=Path,required=True)
p.add_argument('--kernel',type=Path,required=True)
p.add_argument('--name',required=True)
a=p.parse_args();out=a.output.resolve();out.mkdir(parents=True,exist_ok=False)
files={'init':a.controller.resolve(),'recstrap':a.recstrap.resolve(),'payload/target.erofs':a.image.resolve()}
for tool in ['mount','umount','cp','sh','sha256sum']:
 files['usr/bin/'+tool]=Path('/usr/bin/'+tool).resolve()
for source in list(files.values()):
 if source==a.image.resolve():continue
 libraries=subprocess.check_output(['ldd',str(source)],text=True)
 for path in re.findall(r'(/[^\s()]+)',libraries):files[path.lstrip('/')]=Path(path)
entries={}
for name in list(files)+['dev/console','dev/null','proc/.placeholder','sys/.placeholder','target/.placeholder','run/.placeholder','tmp/.placeholder']:
 for parent in Path(name).parents:
  if str(parent)!='.':entries[str(parent)]=(stat.S_IFDIR|0o755,b'',0,0)
entries['dev/console']=(stat.S_IFCHR|0o600,b'',5,1)
entries['dev/null']=(stat.S_IFCHR|0o666,b'',1,3)
manifest={}
for name,source in files.items():
 data=source.read_bytes();entries[name]=(stat.S_IFREG|0o755,data,0,0)
 manifest[name]={'source':str(source),'sha256':hashlib.sha256(data).hexdigest(),'bytes':len(data)}
archive=bytearray()
for ino,(name,(mode,data,major,minor)) in enumerate(list(sorted(entries.items()))+[('TRAILER!!!',(0,b'',0,0))],1):
 name=name.encode()+b'\0'
 fields=[ino,mode,0,0,1,0,len(data),0,0,major,minor,len(name),0]
 archive+=b'070701'+''.join(f'{v:08x}' for v in fields).encode()+name
 archive+=b'\0'*(-len(archive)%4);archive+=data;archive+=b'\0'*(-len(archive)%4)
initrd=gzip.compress(bytes(archive),mtime=0)
(out/'initramfs.cpio.gz').write_bytes(initrd)
kernel=a.kernel.resolve().read_bytes();(out/'vmlinuz').write_bytes(kernel)
manifest['kernel']={'source':str(a.kernel.resolve()),'sha256':hashlib.sha256(kernel).hexdigest()}
manifest['initramfs']={'sha256':hashlib.sha256(initrd).hexdigest()}
(out/'inputs.json').write_text(json.dumps(manifest,indent=2)+'\n')
xml=f'''<domain type="kvm"><name>{a.name}</name><memory unit="MiB">4096</memory><vcpu>2</vcpu>
<os><type arch="x86_64" machine="q35">hvm</type><kernel>{out}/vmlinuz</kernel><initrd>{out}/initramfs.cpio.gz</initrd><cmdline>console=ttyS0 rdinit=/init panic=-1 loglevel=3 selinux=0 prd12_deployment_fixture=1</cmdline></os>
<features><acpi/><apic/></features><cpu mode="host-passthrough"/><clock offset="utc"/>
<on_poweroff>destroy</on_poweroff><on_reboot>destroy</on_reboot><on_crash>destroy</on_crash>
<devices><emulator>/usr/local/bin/qemu-system-x86_64</emulator><serial type="file"><source path="{out}/console.log"/><target port="0"/></serial></devices></domain>'''
(out/'domain.xml').write_text(xml)
print(out/'domain.xml')
