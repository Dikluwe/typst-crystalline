from pathlib import Path
import json,datetime,subprocess
out=Path('00_nucleo/diagnosticos/clippy-enums-2026-10-16');tests=json.loads((out/'traits-tests-materialization.json').read_text());record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),before=tests['after'],edits=[],unformatted={})
assert json.loads((out/'traits-red-result.json').read_text())['exit']==101
for p,text in record['before'].items():assert Path(p).read_text()==text

def edit(name,old,new):
 p=f'01_core/src/entities/{name}.rs';s=Path(p).read_text();assert s.count(old)==1,(name,old);s=s.replace(old,new,1);Path(p).write_text(s);record['edits'].append(dict(path=p,old=old,new=new))
edit('decimal','    /// Representação textual decimal pura (sem notação científica).\n    pub fn to_string(&self) -> String {\n        self.0.to_string()\n    }\n\n','')
for name in ['duration','version']:edit(name,'    pub fn to_string(&self) -> String {','    fn canonical_string(&self) -> String {')
for name in ['decimal','duration','version']:
 typ=name.title();body='std::fmt::Display::fmt(&self.0, f)' if name=='decimal' else 'f.write_str(&self.canonical_string())'
 impl=f'impl std::fmt::Display for {typ} {{\n    fn fmt(&self, f: &mut std::fmt::Formatter<\'_>) -> std::fmt::Result {{\n        {body}\n    }}\n}}\n\n'
 edit(name,'#[cfg(test)]\nmod tests {',impl+'#[cfg(test)]\nmod tests {')
fold='''        let mut delta = StyleDelta::empty();
        for style in iter {
            style.fold_into(&mut delta);
        }
        Self { delta }'''
edit('style','    pub fn from_iter<I: IntoIterator<Item = Style>>(iter: I) -> Self {\n'+fold+'\n    }','    pub fn from_iter<I: IntoIterator<Item = Style>>(iter: I) -> Self {\n        <Self as FromIterator<Style>>::from_iter(iter)\n    }')
impl='impl FromIterator<Style> for Styles {\n    fn from_iter<I: IntoIterator<Item = Style>>(iter: I) -> Self {\n'+fold+'\n    }\n}\n\n'
edit('style','#[cfg(test)]\nmod tests {',impl+'#[cfg(test)]\nmod tests {')
for p in record['before']:record['unformatted'][p]=Path(p).read_text()
(out/'traits-materialization.json').write_text(json.dumps(record,indent=2)+'\n')
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*record['before']],check=True)
print('Approved four implementations materialized; exact edit transcript saved.')
