from pathlib import Path
import json,gzip,hashlib,datetime,re,subprocess
out=Path('00_nucleo/diagnosticos/place-origem-local-2026-10-14');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
sources=['01_core/src/compiler/layout/'+n+'.rs' for n in ['mod','sub_frame','checkpoint','placement','place','tests']];owners={n:re.search(r'^//! @prompt (.+)$',Path(n).read_text(),re.M).group(1) for n in sources};paths=sources+list(owners.values())
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={n:dict(text=Path(n).read_text(),sha256=sha(n)) for n in paths})
with gzip.open(out/'before-source.json.gz','wt') as f:json.dump(r,f)
(out/'owners.json').write_text(json.dumps(owners,indent=2))
nucleus='''tekt = 1
kind = "nucleus"
id = "place-coordinate-origin"
title = "Origem geométrica do body de Place e referencial do pai"

[[claims]]
id = "body-origin"
level = "must"
statement = "O corpo de Place não flutuante é composto uma vez com origem geométrica; texto instala sua própria baseline e forma não adquire strut de fonte. A translação ao pai ocorre uma vez em itens, decoração, posições e fixups."

[[claims]]
id = "parent-origin"
level = "must"
statement = "O subframe conserva explicitamente a origem vertical de recomposição esperada pelo chamador: zero no caminho geométrico, start_y no legado de baseline. Place usa essa origem ao ancorar no subframe, sem inferi-la da fonte corrente ou do estado mutável de baseline pendente."

[[claims]]
id = "pending-flow"
level = "must"
statement = "Sem alinhamento vertical explícito, uma linha pendente ancora Place na sua baseline física corrente; o corpo geométrico acrescenta somente sua própria geometria. Top explícito ancora no topo da região e não segue a linha pendente."

[[claims]]
id = "lifo-checkpoint"
level = "must"
statement = "Origem do subframe é estado interno puro salvo/restaurado em LIFO e capturado/restaurado com o checkpoint da cauda; não vaza entre irmãos ou tentativas. Não muda fase, API pública, default ou topologia."
'''
np=Path('00_nucleo/prompts/_nuclei/place-coordinate-origin.toml');assert not np.exists();np.write_text(nucleus);pin=hashlib.sha256(np.read_bytes()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest()
roles={
'mod':'Adicionar estado pub(super) sub_frame_origin_y:f64, inicial zero, representando somente a origem de recomposição vertical do subframe. Não alterar assinatura pública, entidade nem política de layout no hub.',
'sub_frame':'Salvar sub_frame_origin_y no protocolo isolado; após calcular start_y instalar start_y; restaurar o anterior antes de retornar. Geometric origin usa zero; legado usa a baseline instalada. Não mudar composição dos demais callers.',
'checkpoint':'Capturar e restaurar sub_frame_origin_y junto de is_sub_frame. Manter prefixo confirmado, ordinal de callbacks e demais campos.',
'placement':'Compor todo body de Place não flutuante por layout_sub_frame_at_geometric_origin. Não subtrair ascender nem cell_origin_y do body local zero. No pai subframe, origin_y é sub_frame_origin_y; raiz e célula conservam regiões existentes. Cálculo de size/align/diferimento e translações de metadados permanecem; nenhum teste de presença de texto decide origem do body.',
'place':'O adapter segue linha pendente somente quando effective_alignment.v é None. Na raiz, emprestar margin.top=cursor_y para a chamada; no subframe, transladar a ocorrência por cursor_y - sub_frame_origin_y. Não somar text top-edge: o body geométrico já instala o seu. Restaurar margin, não avançar cursor e aplicar mesma delta a itens/decoração/fixups. Floats não mudam.',
 'tests':'Adicionar testes de linguagem/geométricos com layout_test: Place de forma em Box fixo/auto/inset/aninhado/outer Place; VAlign Bottom/Horizon; variação de fonte; linha pendente shape/text e Top explícito; controles de Align/Grid/Table. Usar composição afim real para medir posição, sem aparar ou normalizar deslocamento errado. Preservar testes P1292 e de paginação existentes.'}
for source,owner in owners.items():
 p=Path(owner);s=p.read_text();entry='- '+str(np)+' sha256:'+pin;m=re.search(r'Núcleos Tekt:\n((?:- [^\n]+\n)+)',s)
 if m:s=s[:m.start(1)]+'\n'.join(sorted(set(m.group(1).splitlines()+[entry])))+'\n'+s[m.end(1):]
 else:
  i=s.index('\n',s.index('Hash do Código:'));s=s[:i+1]+'\nNúcleos Tekt:\n'+entry+'\n'+s[i+1:]
 s+='''\n## Origem geométrica de Place em região local — 2026-10-14

### Medição anterior à decisão

place-origem-local-2026-10-14/{before,extra-before}/results.json.gz registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, diffstat/UTC/fontes/binários/argv e pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Box-place publica shape em y37.75px/4 contra28pt no canvas480×400 (bbox raster aproximado); no SVG fonte, box sem margem do corpus tiling tem y14.834 local contra5. O deslocamento cresce com fonte. placement.rs:463 veta origem geométrica dentro de qualquer subframe e :533 conserva o ascender local; place.rs:96–131 soma text top-edge mesmo para Top explícito. Root-text shape diverge29.5/22.25pt; Box-text32/22.25; Grid e Align são controles positivos. Fonte pinada flow/distribute.rs:703–715 separa align_y explícito de offset de fluxo. Isso é geometria de linguagem; não obrigação de buffers Rust. A hipótese é refutada se texto, VAlign, scopes, fixups ou checkpoint perderem coordenadas após origem única.

### Responsabilidade exclusiva

'''+roles[Path(source).stem]+'''

### Aceitação e etapas

Esta seção supersede a restrição histórica de body geométrico apenas na raiz de Place. L0 primeiro e RED→GREEN em fluxo interno ADR-0127: sem campo público/trait/assinatura/fase/default novos. Comparar antes/depois SVG/PDF e preservar corpos textuais, mistura, páginas automáticas, clipping, posições observáveis e floats; build/workspace/rustdoc/fmt/linhagem/inventário.
A origem do body e a âncora após fluxo comitado são defeitos distintos. Esta etapa A não fecha âncora depois de Shape/Parbreak/Linebreak nem inset de Block, já refutados pelos controles. A etapa B de place-origem-local completa o fim causal do fluxo e a região de Block; não declarar Place universal pronto enquanto B estiver aberta. O transporte público de Tiling permanece pendente de aprovação em seu pacote, sem alteração por esta correção.
'''
 s=s.replace('y37.75px/4 contra28pt','y37.75pt contra28pt')
 p.write_text(s)
print(pin)
