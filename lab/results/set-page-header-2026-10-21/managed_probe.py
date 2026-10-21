"""Launch one serial resource-limited evidence worker; retain exact argv."""
from pathlib import Path
import datetime,json,os,subprocess,sys
P=Path(__file__).resolve().parent
name=sys.argv[1];command=sys.argv[2:]
assert name.startswith('tekt-set-page-header-') and command
out=P/(name.removeprefix('tekt-set-page-header-')+'-launch.json');assert not out.exists()
argv=['systemd-run','--user','--wait','--pipe','--collect','--unit='+name,'-p','MemoryMax=1G','-p','MemoryHigh=768M','-p','MemorySwapMax=0','--setenv=PATH='+os.environ['PATH'],'--working-directory='+str(Path.cwd()),*command]
r={'utc_start':datetime.datetime.now(datetime.timezone.utc).isoformat(),'argv':argv,'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()}
out.write_text(json.dumps(r,indent=2)+'\n');code=subprocess.call(argv)
r.update(utc_end=datetime.datetime.now(datetime.timezone.utc).isoformat(),exit=code);out.write_text(json.dumps(r,indent=2)+'\n');raise SystemExit(code)
