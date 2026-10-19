from pathlib import Path
import json,gzip,subprocess
out=Path(__file__).resolve().parents[1]
def expected():
 b=json.load(gzip.open(out/'before.json.gz'));red=json.load(gzip.open(out/'red-state.json.gz'));plan=json.load(open(out/'projection-plan.json'));result={}
 for source in json.load(open(out/'scope.json'))['sources']:
  s=red['files'][source];assert s.startswith(b['files'][source]),source;data=s.encode()
  for t in sorted([t for t in plan['targets'] if t['source']==source],key=lambda t:t['pattern_start_byte'],reverse=True):
   a,z=t['pattern_start_byte'],t['pattern_end_byte'];assert data[a:z]==b'_',t;data=data[:a]+t['replacement'].encode()+data[z:]
  s=data.decode()
  if source.endswith('/fields.rs'):
   needle='        Content::Styled(styled) => styled_equation(&styled.body),';assert s.count(needle)==1;s=s.replace(needle,needle+'''
        Content::Source(e) => styled_equation(&e.body),
        Content::ShowRecipe(e) => styled_equation(&e.body),
        Content::Located(e) => styled_equation(&e.body),
        Content::Realized(e) => styled_equation(&e.visual),''',1)
  result[source]=subprocess.run(['rustfmt','--edition','2021','--emit','stdout'],input=s,capture_output=True,text=True,check=True).stdout
 return result
