from pathlib import Path
import subprocess,json
out=Path('00_nucleo/diagnosticos/tekt-enums-2026-10-16');scope=json.loads((out/'scope.json').read_text());before={p:Path(p).read_text() for p in scope['sources']};replay=before.copy();edits=[]
color_names=['Srgb','Luma','LinearRgb','Oklab','Oklch','Hsl','Hsv','Cmyk','Spot'];math_names=['Plain','SansSerif','Chancery','Roundhand','Fraktur','Monospace','DoubleStruck','Script','SScript','Display','Inline'];func_names=['Closure','Native','NativeWithEngine','Element','Plugin','With'];space_names=['Spot','Oklab','Oklch','Srgb','Luma','LinearRgb','Hsl','Hsv','Cmyk']
def choices(prefix,names):
 if prefix in ['Color','Self']:return ' | '.join(f'{prefix}::{n}'+('(_)' if n=='Spot' else ' { .. }') for n in names)
 if prefix=='FuncRepr':return ' | '.join(f'{prefix}::{n}(_)' for n in names)
 if prefix=='ColorSpace':return ' | '.join(f'{prefix}::{n}'+('(_)' if n=='Spot' else '') for n in names)
 return ' | '.join(f'{prefix}::{n}' for n in names)
def edit(module,name,old,new):
 p='01_core/src/entities/'+module+'.rs';assert replay[p].count(old)==1,(module,name,old);replay[p]=replay[p].replace(old,new);edits.append(dict(path=p,name=name,old=old,new=new,count=1))
def arm(module,name,fn_anchor,pattern,new_pattern):
 p='01_core/src/entities/'+module+'.rs';t=replay[p];start=t.index(fn_anchor);pos=t.index(pattern,start);old=pattern;new=new_pattern
 # Require the selected occurrence rather than relying on a global substring count.
 end=pos+len(old);segment=t[start:end];assert segment.count(old)==1,(module,name)
 edits.append(dict(path=p,name=name,old=segment,new=segment[:-len(old)]+new,count=1));assert t.count(segment)==1
 replay[p]=t[:start]+segment[:-len(old)]+new+t[end:]
edit('color','eq','_other => false, // neutro: N16[β] — variantes de cores em espaços distintos são estritamente desiguais',f'({choices("Self",color_names)}, _) => false,')
arm('color/adjustments','darken','pub fn darken','_ => self.lighten(-factor),',choices('Color',[n for n in color_names if n not in ['Spot','Cmyk']])+' => self.lighten(-factor),')
arm('color/adjustments','rotate','pub fn rotate','_ => unreachable!("to_space(hue-space) devolve sempre o espaço hue"),',choices('Color',[n for n in color_names if n not in ['Oklch','Hsl','Hsv']])+' => unreachable!("to_space(hue-space) devolve sempre o espaço hue"),')
arm('color/adjustments','saturate','pub fn saturate','_ => {',choices('Color',[n for n in color_names if n not in ['Luma','Spot','Hsl','Hsv']])+' => {')
arm('color/conversion','to_oklab','pub(super) fn to_oklab_p476','_ => {',choices('Color',[n for n in color_names if n not in ['Oklab','Luma']])+' => {')
arm('color/conversion','to_oklch','pub(super) fn to_oklch_p476','_ => {',choices('Color',[n for n in color_names if n!='Oklch'])+' => {')
arm('color/conversion','to_space_spot','pub fn to_space','_ => Err(ColorSpaceError::ProcessToSpot {',choices('Color',[n for n in color_names if n!='Spot'])+' => Err(ColorSpaceError::ProcessToSpot {')
arm('color/conversion','to_process','pub fn to_process','_ => self.clone(),',choices('Color',[n for n in color_names if n!='Spot'])+' => self.clone(),')
arm('color/mixing','mix_default','pub fn mix(','_ => ColorSpace::Oklab,',f'({choices("Color",color_names)}, _) => ColorSpace::Oklab,')
edit('color/mixing','mix_hue','_other => None, // neutro: N16[β] — espaços cromáticos sem componente angular/hue não necessitam de correcção circular',choices('ColorSpace',[n for n in space_names if n not in ['Oklch','Hsl','Hsv']])+' => None,')
arm('color/mixing','weighted_default','pub(crate) fn mix_weighted','_ => ColorSpace::Oklab,',choices('Color',color_names)+' => ColorSpace::Oklab,')
arm('color/mixing','weighted_hue','pub(crate) fn mix_weighted','_ => None,',choices('ColorSpace',[n for n in space_names if n not in ['Oklch','Hsl','Hsv']])+' => None,')
edit('math_style','map_glyph_vs','_other => None, // neutro: N16[β] — sem selector de variação correspondente para o estilo pedido,',choices('MathStyleKind',[n for n in math_names if n not in ['Chancery','Roundhand']])+' => None,')
edit('math_style','letter_base','_other => None, // neutro: N16[β] — sem base alfabética para o estilo de letra pedido,','('+choices('MathStyleKind',['Roundhand','Display','Inline'])+', _, _) => None,')
edit('math_style','digit_base','_other => None, // neutro: N16[β] — sem base numérica para o estilo de dígito pedido,','(MathStyleKind::Plain, false) | ('+choices('MathStyleKind',['Chancery','Roundhand','Fraktur','Script','SScript','Display','Inline'])+', _) => None,')
edit('math_style','bmp_exception','_other => None, // neutro: N16[β] — sem exceção BMP para a combinação de glifo e estilo,','('+choices('MathStyleKind',math_names)+', _, _, _) => None,')
arm('func','element_name','pub fn element_name','_ => None,',choices('FuncRepr',[n for n in func_names if n!='Element'])+' => None,')
arm('func','namespace','pub fn namespace','_ => None,',choices('FuncRepr',['Closure','Element','Plugin'])+' => None,')
edit('func','eq','_other => false, // neutro: N16[β] — funções com representações internas diferentes são desiguais,',f'({choices("FuncRepr",func_names)}, _) => false,')
arm('func','hash','impl Hash for Func','_ => {',choices('FuncRepr',['Closure','Element','Plugin','With'])+' => {')
edit('state_update','eq','_other => false, // neutro: N16[β] — actualizações de estado com tipos de operação distintos são desiguais,','(StateUpdate::Set(_), StateUpdate::Func(_)) | (StateUpdate::Func(_), StateUpdate::Set(_)) => false,')
assert len(edits)==21 and {e['path'] for e in edits}==set(scope['sources'])
# Every operation above replaces only a pattern/comment and retains the existing arm body.
for p,t in before.items():assert Path(p).read_text()==t
for p,t in replay.items():Path(p).write_text(t)
(out/'materialization.json').write_text(json.dumps(dict(before=before,edits=edits,unformatted=replay),ensure_ascii=False,indent=2)+'\n')
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*scope['sources']],check=True)
print(json.dumps(dict(consumers=len(scope['sources']),matches=len(edits))))
