"""Summarize the independent A/B run without reading production or owner tests."""
import hashlib
import json
from pathlib import Path
import subprocess

BASE=Path(__file__).resolve().parent
def sha(p):
    with open(p,'rb') as f: return hashlib.file_digest(f,'sha256').hexdigest()
freeze_path=BASE/'p1314-ab-freeze.json'
runs_path=BASE/'p1314-ab-candidate-runs.json'
comparison_path=BASE/'p1314-ab-comparison.json'
freeze=json.loads(freeze_path.read_text())
runs=json.loads(runs_path.read_text())
comparison=json.loads(comparison_path.read_text())
assert comparison['freeze_sha256']==sha(freeze_path)
assert comparison['candidate_runs_sha256']==sha(runs_path)
assert len(runs['runs'])==2808
assert all(r['exit'] in (0,1) for r in runs['runs']), 'Unknown: abnormal process exit'
red=sum(e['baseline_red'] for e in freeze['expected'])
text=f'''# P1314 — recibo A/B independente

Resultado: **{comparison['status']}**. {comparison['comparisons']} comparações exatas,
{len(comparison['failures'])} divergências e {comparison['unknown']} Unknown.
Suite de 234 expressões em default, html, a11y e html+a11y, executada nas
ordens normal, repeat e reverse. As {red} expectativas RED anteriores ao
candidato estão incluídas no mesmo contrato de 936 expectativas.

Regime: A/B executado sem atestação de isolamento técnico. Executor
`/root/p1314_tests`; não é protocolo completo, selo de refinamento, mutation
score ou prova de equivalência funcional geral. As capacidades e a exposição
incidental ao diff histórico embutido no baseline autorizado estão declaradas
no freeze; nenhum código candidato, owner loading.rs ou testes locais foi lido.

## Entradas e proveniência

- Freeze: `p1314-ab-freeze.json`, SHA-256 `{sha(freeze_path)}`.
- L0 normativo: `{freeze['l0']['normative_sha256']}`. Todos os bytes são
  protegidos, exceto a linha canônica `Hash do Código`; pins verificados.
- Baseline: `/dev/shm/p1313-target.keFg93/release/typst`,
  SHA-256 `cefb4b485cc25ae98871cfbd7a76925d0333d6d26906dc7f7426868e899285ce`.
- Vanilla ratificado: upstream `a51e02804`, `/usr/local/bin/typst`,
  SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Candidato: `/dev/shm/p1314-target.cswujn/release/typst`,
  SHA-256 `cf9b997ac399cfb95cf15063cc417484ed28a75be18f6e7d55a28300d792114f`.
- Build recebido: `p1314-build.json`, SHA-256 `{sha(BASE/'p1314-build.json')}`.
- Runs: `p1314-ab-candidate-runs.json`, SHA-256 `{sha(runs_path)}`.
- Comparação: `p1314-ab-comparison.json`, SHA-256 `{sha(comparison_path)}`.
- Intervalo: {runs['start']['utc']} — {runs['end']['utc']}.
- HEAD: `{runs['start']['head']}`; working tree não commitado, diff/stat
  integral abaixo. UTC, argv, cwd, tempo, exit/stdout/stderr por processo e
  estado final encontram-se em runs.

```text
{runs['start']['diff_stat'].rstrip()}
```

## Execução reproduzível

```sh
python3 00_nucleo/diagnosticos/p1314-ab-runner.py --candidate /dev/shm/p1314-target.cswujn/release/typst --candidate-sha256 cf9b997ac399cfb95cf15063cc417484ed28a75be18f6e7d55a28300d792114f --orders normal,repeat,reverse --output <novo-runs.json>
python3 00_nucleo/diagnosticos/p1314-ab-freeze.py --measurement 00_nucleo/diagnosticos/p1314-ab-freeze.json --candidate-runs <novo-runs.json> --output <nova-comparacao.json>
```

RAM e fixtures são do namespace host; executar com a mesma capacidade de
acesso concedida à medição. Outputs devem ser novos; evidência histórica
nunca é sobrescrita.

## Escopo observado e limites

Os 144 casos P1313 usam expressões e cwd `/tmp/p1313-ab-fixtures` originais.
Antes do freeze, seus outputs atuais foram conferidos literalmente contra
P1313 em todos os perfis. Os 13 deltas de opções CSV estão identificados em
`historical_deltas` e `historical_case_mapping`; a ordem normal serve para
replay sem outra execução. Também há o sentinela P1312 exato
`csv("data.csv", delimiter: "ab")`.

Casos causais distinguem primeira ocorrência inválida, última válida,
delimiter inteiro antes de row-type, value span, With, Args, spread, sink,
filter e map detached. Há controles de defaults, valores CSV, unknown-named
antes de fonte/opções, cast da fonte, missing, excesso, duplicata sintática,
I/O, parsing, UTF-8 inválido, read, decoders, encoders e fields.

Delimiter Symbol continua rejeitado por obrigação normativa; a origem e
o trace medidos no vanilla são reutilizados sem alegar igualdade da coerção.
Row-type Symbol segue diagnóstico vanilla. Dívidas de unknown-named, parsing
e read encoding sobrescrito permanecem controles literais.

Ausência de arquivos discrimina erro de opção antes de leitura; zero chamadas
a World e Args sintético Rust exigem evidência do owner/revisor. O transporte
map da linguagem cobre origem detached pública. Não há teste independente
direto de carriers Rust nem atestação de isolamento do filesystem compartilhado.
'''
p=BASE/'p1314-ab-receipt.md'
assert not p.exists()
patch='*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+x+'\n' for x in text.splitlines())+'*** End Patch\n'
subprocess.run(['apply_patch'],input=patch,text=True,check=True,capture_output=True)
print(json.dumps(dict(status=comparison['status'],comparisons=comparison['comparisons'],baseline_red=red,failures=len(comparison['failures']),unknown=comparison['unknown'],receipt_sha256=sha(p))))
