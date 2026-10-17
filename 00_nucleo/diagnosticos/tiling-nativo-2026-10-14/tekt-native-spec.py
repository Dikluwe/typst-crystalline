from pathlib import Path
import re,sys,gzip,json,hashlib,datetime,subprocess
out=Path('00_nucleo/diagnosticos/tiling-nativo-2026-10-14')
source=sys.argv[1];role=sys.argv[2];p=Path(source);owner=re.search(r'^//! @prompt (.+)$',p.read_text(),re.M)[1];prompt=Path(owner)
with gzip.open(out/'before.json.gz','rt') as f:before=json.load(f)
for name in [source,owner]:
 if name not in before['files']:before['files'][name]=dict(sha256=hashlib.sha256(Path(name).read_bytes()).hexdigest(),text=Path(name).read_text())
with gzip.open(out/'before.json.gz','wt') as f:json.dump(before,f)
s=prompt.read_text();title='## Transporte preparado em Shape — T1c/T2 aprovado — 2026-10-14'
assert title not in s,owner
pin='- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee\n'
m=re.search(r'Núcleos Tekt:\n((?:- [^\n]+\n)+)',s)
if m:
 pins=sorted(set(m[1].splitlines(keepends=True)+[pin]));s=s[:m.start(1)]+''.join(pins)+s[m.end(1):]
else:
 lines=s.splitlines(keepends=True);lines.insert(2,'\nNúcleos Tekt:\n'+pin);s=''.join(lines)
s+='\n\n'+title+'''

### Medição anterior à decisão

`00_nucleo/diagnosticos/tiling-nativo-2026-10-14/before.json.gz` registra
HEAD, working tree, diffstat/UTC e fontes/owners antes desta migração.
layout_types.rs:510–511 ainda transporta Paint/Stroke; shape.rs:151–191
materializa só fill em grupos; tiling.rs:85 limita/clona a grade. O preparador
prepare_cell já retorna TilingFrame uma vez por ocorrência. As sondas
`tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
comprovam que dense-open e stroke-open ainda divergem nos três targets.
O contrato foi aprovado no manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`;
recibo em `00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A preparação/contagem de objetos é mecânica de transporte; render, contornos,
texto, geometria e diagnósticos são obrigações observáveis de linguagem.

### Responsabilidade exclusiva desta unidade

'''+role+'''

### Aceitação e continuidade

Aplicar o núcleo pinado. A ratificação supersede cláusulas antigas que
aguardavam aprovação ou exigiam Paint/Stroke declarativo dentro de Shape,
somente no âmbito do contrato aprovado. Preservar os demais comportamentos
vigentes desta unidade. L0 primeiro e testes RED→GREEN; migração explícita de
fixtures e consumidores, sem conversão total From<Paint>, sem PartialEq nos
carriers e sem fallback sólido para obter build verde. Não elevar o limite
antigo nem transferir layout para L3. Validar transporte fill/stroke, recursos,
clipping, Parent afim, alvos nativos, controles Solid/Gradient, semântica de
página e toda a suíte/build/doc/fmt/linhagem. T1c, T2 e T3 só estão concluídos
quando integrados e verificados; estado intermediário quebrado ou parcial
não é certificação de produção. T4 e publicação continuam fora deste pacote.
'''
prompt.write_text(s)
print(owner)
