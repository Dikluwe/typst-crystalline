from pathlib import Path
import gzip,json
script=(Path(__file__).parent/'measure.py').read_text();prefix=script[:script.index('cases={}')];runner=script[script.index('for name,(source,args) in cases.items():'):]
before=json.loads(gzip.decompress((Path(__file__).parent/'complete.json.gz').read_bytes()))
cases={n:(c['source'],c['runs']['tekt']['command'][1:]) for n,c in before['cases'].items() if n.startswith(('fresh-isolated','fresh-table','fresh-growing','fresh-block','fresh-styled','multi-fresh'))}
exec(prefix+runner,globals())
