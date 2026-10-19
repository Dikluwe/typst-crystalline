from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,re
out=Path('00_nucleo/diagnosticos/block-regiao-2026-10-14');out.mkdir(exist_ok=False)
names=['mod','checkpoint','sub_frame','region_progress','place','placement','block','tests'];owners={f'01_core/src/compiler/layout/{n}.rs':re.search(r'^//! @prompt (.+)$',Path(f'01_core/src/compiler/layout/{n}.rs').read_text(),re.M).group(1) for n in names}
paths=list(owners)+list(owners.values());r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={n:dict(text=Path(n).read_text(),sha256=hashlib.sha256(Path(n).read_bytes()).hexdigest()) for n in paths})
with gzip.open(out/'before-source.json.gz','wt') as f:json.dump(r,f)
(out/'owners.json').write_text(json.dumps(owners,indent=2))
p=Path('00_nucleo/prompts/_nuclei/block-place-region.toml');assert not p.exists();p.write_text('''tekt = 1
kind = "nucleus"
id = "block-place-region"
title = "Região geométrica local de Block para Place"

[[claims]]
id = "local-region"
level = "must"
statement = "Block instala região privada para Place com origem interna após inset e dimensão declarada menos os dois insets. Altura auto é resolvida pelo fluxo causal do próprio Block; overlay não aumenta a altura. Não usar a página como tamanho final do container."

[[claims]]
id = "flow-origin"
level = "must"
statement = "Place recebe opcionalmente a origem de fluxo já certificada; somente o eixo vertical omitido a usa. Alinhamento explícito usa a região local. O body recebe a largura local e é composto uma vez. Itens, decoração e metadados recebem a mesma translação."

[[claims]]
id = "scope"
level = "must"
statement = "A região privada é guardada e restaurada por Block e checkpoint, isolada em subframes e invalidada na transição física de região. Não muda API pública ou fase. Um Block fragmentado requer regiões por fragmento antes de ser certificado universalmente."

[[claims]]
id = "clip"
level = "must"
statement = "O recorte usa dimensões alocadas do container, incluindo inset, e filhos locais ao Group; não pode usar avanço zero de Place como altura explícita nem aplicar origem duas vezes. Correções diferidas do container são resolvidas antes do encapsulamento; caminhos remanescentes acompanham a estrutura."
''');pin=hashlib.sha256(p.read_bytes()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest();entry=f'- {p} sha256:{pin}'
roles={
'mod':'Armazenar block_place_region:Option<Rect> privado, inicial None; configuração nova limpa. Nenhuma entidade pública nova.',
'checkpoint':'Capturar/restaurar block_place_region no checkpoint do sufixo.',
'sub_frame':'Retirar a região de Block durante sublayout e restaurá-la em LIFO; filhos constroem a própria região.',
'region_progress':'Invalidar block_place_region ao fechar página/coluna, sem restaurar limites do fragmento anterior.',
'place':'Passar flow_y opcional para o helper privado de Place. Remover empréstimo de margin.top e pós-translação de subframe; o helper aplica a origem na fórmula comum, preservando cursor e floats.',
'placement':'Resolver região local de Block antes de compor o body, sem herdar a região no sublayout do body. Aplicar flow_y somente quando v ausente. Preservar seleção cell/subframe/page fora de Block. Altura auto mantém fixup para resolução pelo owner Block; translação de itens/decorações/metadados é única.',
'block':'Instalar região interna com as duas bordas de inset; largura declarada é total com inset, não largura do body acrescida de inset. Conservar certificado do body; resolver alinhamentos Y auto contra sua altura causal antes de agrupar. Clip usa origem externa e altura alocada/causal, rebaseia filhos e caminhos uma vez. Restaurar região pai somente se página/coluna sobrevivem. Preservar fluxo de margem existente; tamanho/desenho/outset e fragmentação ainda precisam controles adicionais B2b.',
 'tests':'Regressões root/Box para inset simétrico/assimétrico, Top/Bottom/Horizon, aninhamento, texto, auto e clip. Verificar dimensões da máscara e coordenadas dos filhos, não apenas existência. Preservar controles anteriores, checkpoint e região após o Block.'}
for src,owner in owners.items():
 p=Path(owner);s=p.read_text();m=re.search(r'Núcleos Tekt:\n((?:- [^\n]+\n)+)',s)
 if m:s=s[:m.start(1)]+'\n'.join(sorted(set(m.group(1).splitlines()+[entry])))+'\n'+s[m.end(1):]
 else:
  i=s.index('\n',s.index('Hash do Código:'));s=s[:i+1]+'\nNúcleos Tekt:\n'+entry+'\n'+s[i+1:]
 s+='''
## Block: região local e recorte — B2a — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/place-fluxo-2026-10-14/block-regions-before/results.json.gz` preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore suja, diffstat, UTC, fontes e binários. Em 24 pares contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, só 2 bboxes coincidem: inset3/top Tekt(20,15), vanilla(20,18); bottom/right Tekt(71,37), vanilla(62,44); clip apaga o marcador Tekt. block.rs:90 ignora inset.right; :220 instala largura como body, :275 usa avanço zero de Place como altura de clip, :290 agrupa filhos absolutos sem rebase. placement.rs:505 usa página/subframe sem região Block. Fonte pinada typst-layout/src/flow/block.rs:66-89 e :223-234 força tamanho interno, cresce inset e só então recorta; :278-280 desconta inset da região. Geometria é observável de linguagem; buffers Rust são mecânica. A hipótese de região local é refutada por aninhamento, texto, auto, clipping ou vazamento aos irmãos.

### Responsabilidade exclusiva

'''+roles[Path(src).stem]+'''

### Aceitação e incompletude explícita

Correção interna/de paridade ADR-0127, sem campo público, modo ou fase. Este adendo substitui somente as fórmulas incompatíveis de região/width/clip anteriores; não legitima outros reparos sem medição. L0 primeiro, RED→GREEN, validação bilateral, suíte e resselo restrito. B2a cobre região local e clipping na região sobrevivente. B2b completa fragmentação de Block, dimensões/outset e integração dos metadados/decoradores que controles adicionais refutarem; B3 continua responsável por baseline tardia de inline Box/Transform e produtores ainda não certificados. Não declarar objetivo completo com essas frentes abertas.
''';p.write_text(s)
print(pin)
