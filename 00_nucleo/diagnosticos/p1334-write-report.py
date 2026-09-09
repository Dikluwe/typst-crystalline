"""Write the substantive final report using measured immutable receipts."""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('r',D/'p1334-record.py')
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
read=lambda n:json.loads((D/('p1334-'+n+'.json')).read_text())
m=read('metrics');c=m['counts'];s=r.state();r.verify(s)
build=read('final-build');tests=m['tests'];green=tests['unit-green'];workspace=tests['workspace-tests']
assert build['exit']==0 and green['failed']==workspace['failed']==0
rows=read('ab-cli-normal')['cases']
remaining={x['case']:x['expression'] for x in rows if x['case'] in m['remaining_cases']}
text=f'''# P1334 — argumentos de calc.abs: resultado e pendências

## Resultado

Implementados os diagnósticos que faltavam na assinatura da nativa `calc.abs`:
ausência de `value`, uso indevido de `value:` com hint e primeira sobra na ordem
conjunta de posicionais/nomeados. O erro aponta para a ocorrência inteira;
quando não há argumento, aponta para a chamada. As origens importadas e os
pré-argumentos de `with` são preservados. Um primeiro valor inválido continua
vencendo as sobras, sem alterar conversões, fórmulas ou espécies numéricas.

O dispatcher reconhece somente a identidade da nativa, inclusive através de
`With`; não decide mensagens nem inspeciona argumentos. A fachada recebeu
apenas reexport interno. Os três L0s foram atualizados antes do runtime e
resselados após a implementação; ownership 1:1 e núcleo existente preservados.

## Medição e alcance

No corpus congelado deste passo, a igualdade literal com o vanilla passou de
**{c['base_equals_vanilla']}/{c['observations']} para {c['candidate_equals_vanilla']}/{c['observations']}**:
{c['changed']} observações mudaram e {c['regressed_from_parity']} regrediram de paridade.
São 816 células principais e 12 de import real entre arquivos. As expectativas
congeladas foram satisfeitas nas três execuções (normal, repetida e invertida).
Essas células incluem controles e dívidas preservadas: passar no corpus não
significa que todas as células sejam iguais ao vanilla, nem paridade geral.

O corpo de `calc.abs` foi corrigido apenas nos guards de ausência/sobra.
`Some(occurrences)` conserva ordem, duplicatas e origem; `None` sintético usa
posicionais antes dos nomeados, com spans individuais detached. Fixtures antigas
incoerentes não são prova de paridade; sua migração está no ledger independente.
Não foi criado fallback para um `Some` incoerente.

## Validação

- RED compilado: {tests['unit-red-r1']['passed']} passaram, {tests['unit-red-r1']['failed']} falharam nos casos previstos.
- GREEN focal: {green['passed']} passaram, {green['failed']} falharam.
- Workspace: {workspace['passed']} passaram, {workspace['failed']} falharam, {workspace['ignored']} ignorados; o focal está incluído, não se soma ao total.
- `cargo build --release --locked`, formatação e `git diff --check`: exit 0.
- Linhagem dos três owners e gate estrito V5/V15/V26: sem violações.
- Linter geral: exit 0, mas mantém avisos/info preexistentes. A comparação
  integral das mensagens, descontando somente coordenadas de linha, não encontrou
  novos apontamentos (`p1334-lint-comparison.json`). Não se declara zero avisos global.

A primeira tentativa RED não executou testes: o teste novo usava `scope()` em
um `Value` retornado por `make_calc_module`. O autor B corrigiu somente a
extração `Value::Module` em sucessor, antes de C; original e falha de compilação
ficaram preservados. A evidência RED válida é `p1334-unit-red-r1.json`.

## O que ainda falta

Este passo fecha os diagnósticos da assinatura nativa no domínio coerente testado.
A resolução de `abs` importado em math permanece fora do recorte: não foi
contornada por spelling nem por mudança global no dispatcher. Persistem também
as outras dívidas carregadas como controles. Casos ainda diferentes neste corpus:

'''+''.join(f'- `{name}`: `{expr}`\n' for name,expr in sorted(remaining.items()))+f'''
Esses casos são observações específicas, não inventário exaustivo da linguagem.
O próximo recorte deve medir a rota de resolução math e o respectivo L0 antes de
decidir sua correção; nenhum passo futuro foi implementado implicitamente.

## Proveniência e reprodução

HEAD `{s['head']}`, **working tree não commitado**. Todas as medições trazem
inventário SHA-256, diff/stat, argv, início/fim UTC e estado antes/depois nos
recibos privados `p1334-*.json`; os recibos CLI públicos identificam o executável.
Build: {build['at']} → {build['end']}.
Workspace: {workspace['at']} → {workspace['end']}.
Binário candidato `{build['binary']['path']}`:
`{build['binary']['sha256']}`.
Vanilla ratificado upstream/main `a51e02804`, `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
BASE P1333 SHA-256 `79470612fc121fa42a6898846f85d0b0325edf517c90a830c01cde947f748ffe`.
As referências CLI foram executadas antes de C e reutilizadas por cache imutável,
com hashes e UTC originais verificados; cada rodada executou novamente o candidato.

Estado rastreado exato no momento da validação (inclui alterações herdadas):

```text
{s['diff_stat'].rstrip()}
```

Reprodução: usar `CARGO_TARGET_DIR=/tmp/p1334-target.Ujq0xU` nos comandos cargo;
executar `p1334-ab-cli.py` e `p1334-ab-cross-cli.py` com `--candidate` apontando
para o binário acima, `--output` novo e `--order normal` ou `reverse`.
Não sobrescrever recibos congelados. O manifesto, freezes, métricas e closure
registram os hashes necessários para verificar a mesma árvore e os artefatos.

## Independência e limites

A skill `tekt-materializacao-segregada` orientou a separação entre implementação,
testes/expectativas pré-C e revisão. Regime A/B **sem atestação técnica de isolamento**,
sem selo de refinamento e sem alegação de mutation score. Testes B não receberam
código candidato nem resultados nativos privados; o veredito CLI usa saídas públicas.
Pareceres: `p1334-review-candidate.md`, `p1334-review-final.md` e
`p1334-ab-receipt.md`. O fechamento verificável está em `p1334-closure.json`.
Histórico e alterações herdadas preservados. Sem stage, commit ou push.
'''
path=D/'p1334-final-report.md';assert not path.exists()
patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+line+'\n' for line in text.splitlines())+'*** End Patch\n'
subprocess.run(['apply_patch'],input=patch,text=True,cwd=r.ROOT,check=True)
print(path,r.sha(path))
