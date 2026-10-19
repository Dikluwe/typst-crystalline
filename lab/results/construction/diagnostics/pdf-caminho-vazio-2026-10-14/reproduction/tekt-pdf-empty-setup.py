from pathlib import Path
import json,gzip,hashlib,datetime,subprocess
out=Path('00_nucleo/diagnosticos/pdf-caminho-vazio-2026-10-14')
src='03_infra/src/export/stream/draw.rs';owner='00_nucleo/prompts/infra/export/stream/draw.md';owners={src:owner};(out/'owners.json').write_text(json.dumps(owners,indent=2))
def snapshot(name):
 r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={n:dict(text=Path(n).read_text(),sha256=hashlib.sha256(Path(n).read_bytes()).hexdigest()) for n in [src,owner]})
 with gzip.open(out/name,'wt') as f:json.dump(r,f)
snapshot('before-source.json.gz')
p=Path(owner);p.write_text(p.read_text()+'''
## Caminho de forma sem pintura — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/block-regiao-2026-10-14/open-pdf/results.json.gz`
mede um backing azul extra em Block(outset:4pt) sem fill/stroke, somente no
PDF Tekt. A inspeção visual da amostra confirma a pintura indevida. O corpus
`00_nucleo/diagnosticos/pdf-caminho-vazio-2026-10-14/before-full/results.json.gz`
conserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree sujo,
status, diffstat, UTC, fontes, comandos e hashes de binários. Rect/Ellipse/
RoundedRect invisíveis seguidos de marcador azul põem tinta desde (20,20) no
PDF Tekt, normal e compacto; SVG Tekt e vanilla pinado
9f2b6e8715237cb086899a42873660fe744622e8 só pintam o marcador (70,60).
Os dois ramos `draw.rs:513-517` e `:922-926` constroem o caminho mas nada
emitem para (fill=None,stroke=None), antes de Q. Hipótese causal: o caminho
pendente é consumido pela pintura/recorte seguinte. Refutadores: isolamento
top/local/grupo, pintura fill/stroke/ambos e máscara seguinte inalterados
apesar de encerrar o caminho. Isso é geometria visível de linguagem, não
obrigação de igualdade de bytes nem inferência de intenção upstream.
O caso polygon(stroke:none) é rejeitado antes do exportador no Tekt e fica
registrado como defeito distinto, sem crédito de paridade deste lote.

### Responsabilidade exclusiva e correção interna

Nos dispatchers top/local, terminar sem pintar o caminho construído de uma
Shape cujo fill e stroke sejam ambos ausentes. Preservar envelopes gráficos
e semânticos, ordem de pintura, resource/tag bookkeeping, transformações,
primitivas e os caminhos de fill/stroke efetivos. O ramo sem pintura não pode
contaminar nem a tinta nem a máscara da ocorrência seguinte, inclusive ao
sair de Group. Não descartar documentos, converter ausência em preto nem
alterar o layout produtor para esconder a falha do exportador.

### Aceitação

Correção interna ADR-0127, L0 primeiro e RED→GREEN sem gate público: nenhum
campo, assinatura pública, default, flag, dependência, fase ou unsafe novo.
Os testes no mesmo consumer observam quais retângulos participam da pintura
e do clip em um interpretador independente do estado de caminho PDF, com
valores esperados próprios; não exigem uma string exata ou operador específico
se outra implementação cumprir a geometria. Cobrir top/local, escape de Group,
Compact/Verbose, fill/stroke/ambos e clip posterior. Repetir PDF/SVG bilateral,
controles positivos/sem pintura, qpdf e regressões existentes, suíte completa,
build/doc/fmt/check, inventários e linhagem. Resselo somente deste owner.
Outset/overflow de Block, espessura de sublinhado e aceitação de polygon None
continuam com seus owners e suas evidências; não são fechados por esta correção.
''')
p=Path(src);s=p.read_text();i=s.rfind('\n}');p.write_text(s[:i]+Path('/tmp/tekt-pdf-empty-tests.txt').read_text()+s[i:]);subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',src],check=True)
snapshot('red-state.json.gz')
for suffix in ['before','before-full']:
 p=Path('/tmp/tekt-pdf-empty-'+suffix+'.log')
 if p.exists():(out/(suffix+'.log')).write_bytes(p.read_bytes())
