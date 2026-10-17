from pathlib import Path
import json
import gzip
import hashlib
import subprocess
import datetime
import difflib
import collections
import re

root = Path.cwd()
out = root / '00_nucleo/diagnosticos/tekt-v16-show-selectors-2026-10-16'
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
before = json.load(gzip.open(out / 'before.json.gz'))
assert sha(before['linter']['binary']) == before['linter']['sha256']
scope = json.loads((out / 'scope.json').read_text())
validation = json.loads((out / 'validation-result.json').read_text())
baseline = json.loads((out / 'baseline-result.json').read_text())
assert validation['commands'][-1]['label'] == 'diff'
assert all(x['exit'] == (1 if x['label'] == 'architecture' else 0) for x in validation['commands'])
assert validation['sources_unchanged_during_validation']
assert all(sha(p) == h for p, h in validation['source_hashes'].items())
subprocess.run(['python3', str(out / 'workers/guard.py')], check=True, capture_output=True)
workspace = json.loads((out / 'workspace/workspace-result.json').read_text())
assert workspace['exit'] == workspace['failed'] == 0 and workspace['sources_unchanged']
strict = json.loads((out / 'strict-details.json').read_text())
assert strict['exit'] == 0 and not strict['diagnostics']
red = json.loads((out / 'probe-baseline-result.json').read_text())
green = json.loads((out / 'probe-green-result.json').read_text())
assert red['selected_e0004'] == 0 and green['selected_e0004'] == 6
assert not red['other_errors'] and not green['other_errors']
assert green['total_e0004'] - red['total_e0004'] == 6
delta = json.loads((out / 'architecture-delta.json').read_text())
assert delta['old_v16'] == 267 and delta['new_v16'] == 262
assert len(delta['removed_selected']) == 5
assert not delta['added_non_v19'] and not delta['removed_other_non_v19']
current = json.loads((out / 'validation-architecture.log').read_text())['runs'][0]['results']
counts = dict(collections.Counter(x['ruleId'] for x in current))
resources = [json.loads((out / (p + '-resources.json')).read_text())
             for p in ['baseline', 'implementation', 'validation']]
for record in resources:
    assert record['exit'] == 0 and record['cargo_jobs'] == '1'
    m = record['memory_after']
    assert m['memory.max'] == str(12 * 1024**3) and m['memory.swap.max'] == '0'
    events = dict(line.split() for line in m['memory.events'].splitlines())
    assert events['oom'] == events['oom_kill'] == '0'
peak = max(int(r['memory_after']['memory.peak']) for r in resources)
indices = []
for crate in ['typst_core', 'typst_shell', 'typst_infra', 'typst']:
    p = root / 'target/doc' / crate / 'index.html'
    assert p.is_file()
    indices.append({'path': str(p), 'sha256': sha(p), 'mtime_ns': p.stat().st_mtime_ns})
(out / 'rustdoc-indices.json').write_text(json.dumps(indices, indent=2) + '\n')
rows = json.loads((out / 'classification.json').read_text())
listing = ['# Correções do lote — seletores de show', '',
           '| Função | Arquivo | Situação anterior |', '|---|---|---|']
for row in rows:
    label = 'V16 emitido' if row['kind'] == 'emitted-v16' else 'predicado latente sem V16'
    listing.append(f"| `{row['function']}` | [{row['source']}]({root / row['source']}) | {label} |")
listing += ['', 'Os seis pontos exigem revisão em compilação quando o enum cresce.',
            'Os grupos com o mesmo resultado compartilham corpo e conservam as notas V19.', '']
(out / 'lista-correcoes.md').write_text('\n'.join(listing))
remaining = [x for x in current if x['ruleId'] == 'V16']
per_file = collections.Counter(x['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./') for x in remaining)
pending = ['# Pendências V16 após o lote', '', '**262 erros emitidos**, sem redução de severidade.', '',
           '| Arquivo | Ocorrências |', '|---|---:|']
for source, count in sorted(per_file.items(), key=lambda p: (-p[1], p[0])):
    pending.append(f'| [{source}]({root / source}) | {count} |')
pending += ['', f'Localizações e mensagens completas: [remaining-v16.json]({out / "remaining-v16.json"}).',
            'T027 e T158/T160 continuam pendentes mesmo sem emissão; não foram corrigidos por este lote.', '']
(out / 'lista-pendencias.md').write_text('\n'.join(pending))
latent = json.loads((root / '00_nucleo/diagnosticos/tekt-v12-2026-10-16/latent-v16.json').read_text())
assert all(sha(row['source']) == before['hashes'][row['source']] for row in latent)
(out / 'latent-v16.json').write_text(json.dumps(latent, ensure_ascii=False, indent=2) + '\n')
diff = []
for path in [*scope['sources'], *scope['owners']]:
    diff.extend(difflib.unified_diff(before['files'][path].splitlines(keepends=True),
        Path(path).read_text().splitlines(keepends=True), fromfile='a/' + path, tofile='b/' + path))
(out / 'stage.diff').write_text(''.join(diff))
utc = datetime.datetime.now(datetime.timezone.utc).isoformat()
state = {'utc': utc, 'head': before['head'], 'working_tree': 'não commitado',
         'status': subprocess.check_output(['git', 'status', '--porcelain=v1'], text=True),
         'diffstat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True),
         'source_hashes': validation['source_hashes'], 'scope': scope,
         'v16_before': 267, 'v16_after': 262, 'new_v19_notes': len(delta['new_v19_groups']),
         'rules_after': counts, 'suite': {k: workspace[k] for k in ['passed', 'failed', 'ignored']},
         'probe': {'selected_before': 0, 'selected_after': 6, 'total_before': red['total_e0004'], 'total_after': green['total_e0004']},
         'memory_peak_bytes': peak, 'memory_limit_bytes': 12 * 1024**3, 'swap_limit_bytes': 0,
         'oom': 0, 'oom_kill': 0, 'global_architecture_gate': 'failed: remaining V16',
         'linter': before['linter'], 'productive_files_changed': scope['sources'] + scope['owners']}
(out / 'final-state.json').write_text(json.dumps(state, ensure_ascii=False, indent=2) + '\n')
report = f'''# V16 — lote de seletores de show concluído

**Cinco V16 corrigidos: 267 → 262 erros emitidos.** Um sexto ponto, o predicado
is_text_rule antes expresso por matches!, também passa a exigir uma decisão
quando o enum cresce. O lote abrange quatro consumers e seus quatro L0.

## Comportamento preservado

- native_element_base recorre apenas por Where; And/Or não extraem uma base nativa.
- O caminho textual dedicado aceita somente Text/Regex; preserva offsets UTF-8,
  ordem e exclusão de matches vazios. O grupo vazio de textual_match_ranges inclui Text
  porque o guard do braço anterior pode falhar.
- A captura de equação e tabela conserva identidade executável, aliases reais,
  rejeição de funções homônimas falsas e a política any em And/Or. Filtros Where
  não são avaliados para decidir a necessidade de captura contextual.

As alternativas com comportamento comum permanecem agrupadas, com um único
corpo. Há **{len(delta['new_v19_groups'])} notas V19 novas**, mantidas como informação.
Elas registram alternativas escritas, não déficit de testes nem combinações
alcançáveis. Não houve supressão, alteração de severidade, corpo duplicado ou
helper genérico para contornar o linter. O gate global continua vermelho pelos
262 V16 externos ao lote. T027 e T158/T160 continuam rastreados sem emissão.

Não houve nova API, variante/campo produtivo, default de produto, mudança de fase
ou quebra de compatibilidade. O fallback interno de FuncRepr em table_cells e decisões de
Content/query não fazem parte desta correção. O lote não certifica paridade geral
ou ganho de leitura/performance.

## L0, testes e prova estrutural

Os quatro L0 foram atualizados antes dos testes/código, sob fluxo contínuo
ADR-0127, com ownership/núcleos validados e resselo restrito bilateral. A medição
e decisão por função estão em classification.json, scope.json e
l0-before-code.json. stage.diff inclui somente os oito arquivos declarados;
pattern-changes.diff separa a nominalização dos novos controles de teste.

Os quatro testes novos passaram no código anterior. Eles verificam as nove
variantes de show, identidade/aliases, composições e seus vazios, Where aninhado,
UTF-8, literal vazio e regex de match vazio. Não há alegação de RED funcional:
o comportamento vigente é preservado. O RED→GREEN desta manutenção é estrutural.

Em cópias descartáveis do workspace real, acrescentar uma variante sentinela
somente ao Selector copiado produziu **0 → 6 erros E0004 nos seis pontos alvo**.
O total da cópia foi {red['total_e0004']} → {green['total_e0004']}, sem outro tipo de
erro. A prova compila o core real com cargo check, sem stubs. O enum produtivo
permanece idêntico; os recibos e fontes mutadas estão em probe-*-result.json e
probe-*-input.json.gz. code-guard.json verifica escopo, corpos esperados e
preservação dos arquivos externos.

## Validação final

**{workspace['passed']} testes aprovados, {workspace['failed']} falhas,
{workspace['ignored']} ignorados** na suíte workspace, incluindo os quatro novos
controles. Passaram build workspace, formato, Clippy workspace/all-targets com
-D warnings e -D unfulfilled_lint_expectations, gates V1/V5/V15/V26 estritos,
linhagem reversa, Rustdoc -D warnings e git diff --check. Os quatro índices HTML
foram verificados e registrados em rustdoc-indices.json.

O SARIF completo retorna exit 1 pelos V16 restantes. A comparação conserva regra,
nível, arquivo e mensagem, desconsiderando deslocamentos de linhas: removeu apenas
os cinco V16 previstos; não houve diagnóstico novo fora de V19. O inventário
posterior tem {counts.get('V19',0)} notas V19 e {counts.get('V20',0)} notas V20.
architecture-delta.json e remaining-v16.json contêm a evidência completa.

## RAM e proveniência

Execuções pesadas sequenciais em cgroup, MemoryMax=12 GiB, MemorySwapMax=0 e
CARGO_BUILD_JOBS=1. Maior pico observado: {peak} bytes
({peak/1024**3:.3f} GiB), zero OOM e OOM-kill; recibos *-resources.json.
O pico atingiu o teto configurado; a execução não teve folga de memória nesse
momento. Os contadores memory.events, inclusive max, foram preservados nos recibos.

HEAD `{before['head']}`, working tree não commitado.
Snapshot inicial UTC `{before['utc']}`; validação final UTC
`{validation['start']}`–`{validation['end']}`. before.json.gz e final-state.json
registram hashes, arquivos/fontes, status completo e git diff HEAD --stat.
Fontes/L0/configuração ficaram congelados durante a validação final.
Linter pin `{before['linter']['commit']}`, binário SHA-256
`{before['linter']['sha256']}`. A suíte usa o vanilla pin
9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 ratificado
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea;
o recibo workspace/workspace-pre.json.gz conserva o ambiente e a receita.
Nenhum commit, push ou release nesta etapa.

[Lista de correções]({out / 'lista-correcoes.md'}) · [Pendências]({out / 'lista-pendencias.md'}).
'''
(out / 'relatorio.md').write_text(report)
manifest = [f'{sha(p)}  {p.relative_to(out)}' for p in sorted(out.rglob('*'))
            if p.is_file() and p.name != 'manifest.sha256' and '__pycache__' not in p.parts]
(out / 'manifest.sha256').write_text('\n'.join(manifest) + '\n')
print(json.dumps({'closed': True, 'v16': [267,262], 'new_v19_notes': len(delta['new_v19_groups']),
                  'suite': state['suite'], 'peak_bytes': peak, 'manifest_files': len(manifest)}))
