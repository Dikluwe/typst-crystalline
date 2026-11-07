import gdb,json,os
p=os.environ['STDLIB_COUNT_OUT'];symbols=json.loads(os.environ['STDLIB_COUNT_SYMBOLS']);counts={};gdb.execute('set pagination off');gdb.execute('set confirm off');gdb.execute('set disable-randomization off');gdb.execute('set startup-with-shell off');gdb.execute('handle SIGPIPE nostop noprint pass');gdb.execute('starti')
binary=os.path.realpath(gdb.current_progspace().filename);base=None
for line in gdb.execute('info proc mappings',to_string=True).splitlines():
 c=line.split()
 if c and c[-1]==binary and len(c)>3 and c[0].startswith('0x') and int(c[3],16)==0:base=int(c[0],16);break
assert base is not None
inferior=gdb.selected_inferior().pid
def save():
 tmp=p+'.tmp';open(tmp,'w').write(json.dumps({'counts':counts,'inferior':inferior}));os.replace(tmp,p)
class Counter(gdb.Breakpoint):
 def __init__(self,address,label):super().__init__('*'+hex(base+address),internal=True);self.label=label
 def stop(self):counts[self.label]=counts.get(self.label,0)+1;save();return False
for address,label in symbols:Counter(address,label)
save()
try:gdb.execute('continue')
except BaseException:pass
save()
try:gdb.execute('kill')
except BaseException:pass
