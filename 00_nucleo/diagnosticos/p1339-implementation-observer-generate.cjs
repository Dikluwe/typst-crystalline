// Mechanical source-declaration expansion. No fixture expectations are read.
// Prints an apply_patch addition of a reviewed Rust fragment; never writes files.
const fs = require('fs');
const inventory = JSON.parse(fs.readFileSync('00_nucleo/diagnosticos/p1339-mutant-closed-state-inventory.json'));
const decls = inventory.declarations.map(d => ({...d, text: d.declaration.map(l=>l.text).join('\n')}));
function extract(path, names) {
  const text = fs.readFileSync(path,'utf8').replace(/\/\/[^\n]*/g,'').replace(/#\[[\s\S]*?\]/g,'');
  for(const name of names) {
    const re=new RegExp('pub (struct|enum) '+name+'\\b'); const m=re.exec(text); if(!m) throw name;
    let start=m.index, i=start, n=0, began=false;
    for(;i<text.length;i++) { if(text[i]==='{'){n++;began=true;} if(text[i]==='}'&&!--n&&began){i++;break;} if(text[i]===';'&&!began){i++;break;} }
    decls.push({name,path,kind:m[1],text:text.slice(start,i)});
  }
}
extract('01_core/src/entities/value.rs',['Type']);
extract('01_core/src/entities/gradient.rs',['GradientStop','RelativeTo','Linear','Radial','Conic','Gradient']);
extract('01_core/src/entities/tiling.rs',['Tiling','TilingBody','TilingRelative']);
extract('01_core/src/entities/counter_update.rs',['CounterUpdate']);
extract('01_core/src/entities/duration.rs',['Duration']);
extract('01_core/src/entities/version.rs',['Version']);
extract('01_core/src/entities/symbol.rs',['Symbol']);
extract('01_core/src/entities/layout_types.rs',['Ratio','Angle']);
extract('01_core/src/entities/color.rs',['ColorSpace']);
extract('01_core/src/entities/axes.rs',['Axes']);
extract('01_core/src/entities/layout_types.rs',['Size']);
function split(s) { let a=[],p=0,n=0; for(let i=0;i<s.length;i++){if('<({['.includes(s[i]))n++;if('>)}]'.includes(s[i]))n--;if(s[i]===','&&!n){a.push(s.slice(p,i).trim());p=i+1;}}a.push(s.slice(p).trim());return a.filter(Boolean); }
const special = new Set(['ModuleInner','Module','Styles','IntrospectedContent','RootedPath','VirtualPath','FontList','Lang','Datetime','Engine','Route','FileId','StyleChain','StyleNode','Scope','Binding','NativeFunc','NativeFuncWithEngine','ClosureRepr','ClosureParam','Func','FuncRepr','ElementFunc','PluginFunc','SyntaxNode','SyntaxText','LeafNode','InnerNode','ErrorNode','SyntaxError','Span','Location','Bytes','Decimal','Regex','Version','PluginModuleId']);
const out=[];const seen=new Set();
special.add('Paper'); special.add('Sink'); special.add('Angle');
for(const d of decls) {
  if(special.has(d.name)||d.kind==='type'||!d.text.startsWith('pub ')||d.path==='01_core/src/entities/syntax_node.rs') continue;
  const path='crate::'+d.path.replace(/^01_core\/src\//,'').replace(/\.rs$/,'').replaceAll('/','::')+'::'+d.name;
  if(seen.has(path))continue;seen.add(path);
  const generic=/^(?:pub )?(?:struct|enum) \w+<T/.test(d.text);
  const target=path+(generic?'<T>':'');
  const cleaned=d.text.replace(/#\[[\s\S]*?\]/g,'');
  const body=cleaned.slice(cleaned.indexOf('{')+1,cleaned.lastIndexOf('}')).trim();
  if(d.kind==='struct') {
    if(d.text.includes('{')) {
      const fields=split(body).map(s=>{const m=/^pub (\w+)\s*:/.exec(s);if(!m)throw 'private/unparsed '+d.name+' '+s;return m[1];});
      out.push(`impl${generic?'<T: ObservationEq>':''} ObservationEq for ${target} { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { ${fields.map(f=>f+': _').join(', ')} } = self; ObservationRelation::Same${fields.map(f=>`.and(self.${f}.observation_relation(&other.${f}))`).join('')} } }`);
    } else if(d.text.includes('(')) {
      const fields=split(d.text.slice(d.text.indexOf('(')+1,d.text.lastIndexOf(')')));
      if(!fields.every(f=>f.startsWith('pub '))) throw 'private tuple '+d.name;
      out.push(`impl${generic?'<T: ObservationEq>':''} ObservationEq for ${target} { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self(${fields.map(()=> '_').join(', ')}) = self; ObservationRelation::Same${fields.map((f,i)=>`.and(self.${i}.observation_relation(&other.${i}))`).join('')} } }`);
    } else out.push(`impl ObservationEq for ${target} { fn observation_relation(&self, _: &Self) -> ObservationRelation { let Self = self; ObservationRelation::Same } }`);
  } else {
    let arms=[];
    let variants=split(body);
    if(d.name==='Selector'&&d.path.endsWith('/selector.rs')&&!variants.some(v=>v.startsWith('Element')))variants.push('Element { function: Func, fields: EcoVec<(EcoString, Value)> }');
    for(const variant of variants) {
      const name=/^\w+/.exec(variant)[0];let suffix=variant.slice(name.length).trim();let pattern=name,patternB=name,relations=[];
      if(suffix.startsWith('(')) { const fields=split(suffix.slice(1,-1));pattern+=`(${fields.map((_,i)=>'a'+i).join(', ')})`;patternB+=`(${fields.map((_,i)=>'b'+i).join(', ')})`;relations=fields.map((_,i)=>`a${i}.observation_relation(b${i})`); }
      else if(suffix.startsWith('{')) { const fields=split(suffix.slice(1,-1)).map(f=>f.split(':')[0].trim());pattern+=` { ${fields.map((f,i)=>f+': a'+i).join(', ')} }`;patternB+=` { ${fields.map((f,i)=>f+': b'+i).join(', ')} }`;relations=fields.map((_,i)=>`a${i}.observation_relation(b${i})`); }
      const relation='ObservationRelation::Same'+relations.map(r=>`.and(${r})`).join('');
      arms.push(`Self::${pattern} => match other { Self::${patternB} => ${relation}, _ => ObservationRelation::Different },`);
    }
    out.push(`impl${generic?'<T: ObservationEq>':''} ObservationEq for ${target} { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {\n${arms.join('\n')}\n} } }`);
  }
}
const result='// Mechanical declaration expansion: exhaustive matches and all-field destructuring.\n'+out.join('\n');
console.log('*** Begin Patch\n*** Add File: /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/p1339-implementation-observer-structural.rs\n'+result.split('\n').map(l=>'+'+l).join('\n')+'\n*** End Patch');
