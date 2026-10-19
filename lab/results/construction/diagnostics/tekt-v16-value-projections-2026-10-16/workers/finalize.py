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
out = root / '00_nucleo/diagnosticos/tekt-v16-value-projections-2026-10-16'
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
assert red['selected_e0004'] == 0 and green['selected_e0004'] == 17
assert not red['other_errors'] and not green['other_errors']
assert green['total_e0004'] - red['total_e0004'] == 17
delta = json.loads((out / 'architecture-delta.json').read_text())
assert delta['old_v16'] == 262 and delta['new_v16'] == 253
assert len(delta['removed_selected']) == 9
assert not delta['added_non_v19'] and not delta['removed_other_non_v19']
current = json.loads((out / 'validation-architecture.log').read_text())['runs'][0]['results']
counts = dict(collections.Counter(x['ruleId'] for x in current))
resources = [json.loads((out / (p + '-resources.json')).read_text())
             for p in ['inventory', 'none-probe', 'baseline', 'implementation', 'validation']]
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
listing = ['# Correções do lote — projeções de Value', '',
           '| Função | Arquivo | Situação anterior |', '|---|---|---|']
for row in rows:
    label = 'V16 emitido' if row['kind'] == 'emitted-v16' else 'projeção latente sem V16'
    listing.append(f"| `{row['function']}` | [{row['source']}]({root / row['source']}) | {label} |")
listing += ['', 'Os dezessete pontos exigem revisão em compilação quando o enum cresce.',
            'Os grupos com o mesmo resultado compartilham corpo e conservam as notas V19.', '']
(out / 'lista-correcoes.md').write_text('\n'.join(listing))
remaining = [x for x in current if x['ruleId'] == 'V16']
per_file = collections.Counter(x['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./') for x in remaining)
pending = ['# Pendências V16 após o lote', '', '**253 erros emitidos**, sem redução de severidade.', '',
           '| Arquivo | Ocorrências |', '|---|---:|']
for source, count in sorted(per_file.items(), key=lambda p: (-p[1], p[0])):
    pending.append(f'| [{source}]({root / source}) | {count} |')
pending += ['', f'Localizações e mensagens completas: [remaining-v16.json]({out / "remaining-v16.json"}).',
            'T027 e T158/T160 continuam pendentes mesmo sem emissão; não foram corrigidos por este lote.', '']
(out / 'lista-pendencias.md').write_text('\n'.join(pending))
latent = json.loads((root / '00_nucleo/diagnosticos/tekt-v16-show-selectors-2026-10-16/latent-v16.json').read_text())
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
         'v16_before': 262, 'v16_after': 253, 'new_v19_notes': len(delta['new_v19_groups']),
         'rules_after': counts, 'suite': {k: workspace[k] for k in ['passed', 'failed', 'ignored']},
         'probe': {'selected_before': 0, 'selected_after': 17, 'total_before': red['total_e0004'], 'total_after': green['total_e0004']},
         'memory_peak_bytes': peak, 'memory_limit_bytes': 12 * 1024**3, 'swap_limit_bytes': 0,
         'oom': 0, 'oom_kill': 0, 'global_architecture_gate': 'failed: remaining V16',
         'linter': before['linter'], 'productive_files_changed': scope['sources'] + scope['owners']}
none_probe = json.loads((out / 'none-probe-result.json').read_text())
assert none_probe['explicit_none_is_reported_as_wildcard'] and none_probe['compiler_already_requires_future_variant']
findings = json.loads((out / 'none-parser-findings.json').read_text())['observations']
assert len(findings) == 9
assert all(sha(x['source']) == before['hashes'][x['source']] for x in findings)
parser_lines = ['# Emissões sobre None explícito — correção do parser pendente', '',
    'A fixture mínima compilou sem wildcard, mas o linter denunciou None; uma',
    'variante futura já produz E0004. A causa foi localizada em check_is_catchall',
    'e get_catchall_ident do parser Rust. Não houve edição do linter nesta etapa.', '',
    '| Consumer | Linha | Função |', '|---|---:|---|']
for finding in findings:
    parser_lines.append(f"| [{finding['source']}]({root / finding['source']}:{finding['line']}) | {finding['line']} | `{finding['function']}` |")
parser_lines += ['', 'O defeito confirmado é a classificação de None como binding genérico.',
    'Isso não certifica a exaustividade de braços internos como Some(_): eles',
    'precisam de análise própria. Não isentar todos os identificadores maiúsculos,',
    'pois um identificador incomum também pode ser binding verdadeiro.', '',
    f'[Reprodução]({out / "none-probe-result.json"}) · [Análise]({out / "none-parser-analysis.json"}).', '']
(out / 'parser-pendencias.md').write_text('\n'.join(parser_lines))
state['none_parser_findings'] = 9
state['goal_scope_incomplete'] = True
(out / 'final-state.json').write_text(json.dumps(state, ensure_ascii=False, indent=2) + '\n')
report = f'''# V16 — lote de projeções de Value concluído

**Nove V16 corrigidos: 262 → 253 erros emitidos.** Oito projeções latentes
adicionais foram fechadas no mesmo owner; 17 decisões em quatro consumers e seus
quatro L0. O objetivo global de corrigir todas as pendências continua aberto.

## Correção e preservação

A entidade Value mantém suas 39 variantes e assinaturas. is_none, truthy e os
doze casts passam a nomear as variantes atuais que chegam ao resultado genérico.
As três projeções de diagnóstico em callee_error, format_options e document
preservam seus contratos próprios: document continua usando str; as outras
rotas conservam string onde já aplicável. Content/LocatedContent continuam
identificando o elemento no erro de callee, sem perder a projeção consultada.

Não foram alteradas fórmulas, unidades, limites, sinais, empréstimos, identidade
de Arc Version, payloads, chamadas ou mensagens. A conversão Float de Duration
continua em segundos e a de Int em nanossegundos. Esta etapa não corrige nem
certifica como paridade as bordas numéricas não medidas, incluindo NaN nessa
conversão. O enum produtivo, Type, igualdade/hash, representação dos dados e pipeline permanecem.
Não houve contrato público, flag, default ou quebra de compatibilidade novos.

Os grupos equivalentes compartilham um corpo. As **{len(delta['new_v19_groups'])}
notas V19 novas** estão mantidas e prescritas nos L0, sem copiar corpos nem
confundir alternativas escritas com cobertura de testes. Sem allow/expect,
exceção ou relaxamento da política V16. A comparação remove apenas os nove
V16 previstos, sem diagnóstico novo fora de V19.

## L0, testes e prova estrutural

L0 atualizado antes do código/testes, medição antes de decisão e resselo
bilateral restrito. before.json.gz, classification.json e l0-before-code.json
registram ownership e fontes; materialization.json conserva cada complemento
nominal e replay de edição. stage.diff abrange apenas os oito arquivos previstos.

Os sete testes novos passaram antes da alteração dos padrões e na validação
final. Exercitam verdade/vazio, casts estritos, empréstimos, coerções, sinal,
overflow, inválidos de Decimal/Version/Regex e nomes de diagnóstico. São controles
do contrato vigente, não prova nova de paridade universal nem cobertura de cada
payload das 39 variantes. Não houve RED funcional; a manutenção preserva resultados.

O RED→GREEN é estrutural: em cópias reais do workspace, uma variante sentinela
adicionada apenas ao Value copiado passou de **0 para 17 E0004 nos pontos alvo**.
O total da cópia foi {red['total_e0004']} → {green['total_e0004']}, sem outro tipo
de erro, usando cargo check do core real e sem stubs. Recibos/fontes em
probe-*-result.json e probe-*-input.json.gz. Nenhuma variante foi adicionada à
fonte produtiva. code-guard.json verifica escopo e preservação dos corpos,
com exceção da expansão prescrita de matches! em is_none para match exaustivo.

## Defeito do parser identificado e ainda pendente

O inventário atual encontrou nove emissões V16 cujo padrão é None explícito.
A fixture mínima em none-probe-result.json compila sem wildcard, mas o linter
pinado denuncia None. Acrescentar uma variante ao enum interno já produz E0004
nessa fixture antes de qualquer reparo: é um falso positivo do parser, que não
distingue esse constructor de binding. Os nove locais do projeto estão em
none-parser-findings.json, com os contextos AST de origem. Seus consumers não
foram reescritos para esconder a emissão. A correção do linter é uma próxima
ação concreta para o objetivo global, não foi declarada concluída nesta etapa.

## Validação final

**{workspace['passed']} testes aprovados, {workspace['failed']} falhas,
{workspace['ignored']} ignorados** na suíte completa. Build workspace, formato,
Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations,
V1/V5/V15/V26 estritos, linhagem reversa, Rustdoc -D warnings e git diff --check
passaram. Os quatro índices HTML foram conferidos em rustdoc-indices.json.

O SARIF completo continua em exit 1 pelos 253 V16 emitidos, incluindo os nove
casos None acima. V19 tem {counts.get('V19',0)} notas e V20 tem
{counts.get('V20',0)} notas. A comparação por regra/nível/arquivo/mensagem
ignora deslocamentos de linhas e conserva todos os diagnósticos externos;
architecture-delta.json e remaining-v16.json guardam os dados. T027 e T158/T160
continuam rastreados sem emissão, com suas fontes intactas.

## RAM e proveniência

Execuções pesadas sequenciais em cgroup: MemoryMax=12 GiB, MemorySwapMax=0,
CARGO_BUILD_JOBS=1. Maior pico: {peak} bytes ({peak/1024**3:.3f} GiB), zero OOM
e OOM-kill. O pico permaneceu abaixo do teto configurado. Os recibos
*-resources.json preservam também os contadores memory.events.

HEAD `{before['head']}`, working tree não commitado. Snapshot inicial UTC
`{before['utc']}`; validação final UTC `{validation['start']}`–`{validation['end']}`.
Status completo, git diff HEAD --stat, fontes e hashes antes/depois em
before.json.gz e final-state.json. Fontes/L0/configuração ficaram congelados
durante a validação. Linter pin `{before['linter']['commit']}`, SHA-256 do binário
`{before['linter']['sha256']}`. Suíte com vanilla pin
9f2b6e8715237cb086899a42873660fe744622e8 e binário SHA-256 ratificado
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea;
ambiente/receita em workspace/workspace-pre.json.gz.
Nenhum commit, push ou release nesta etapa.

[Correções]({out / 'lista-correcoes.md'}) · [Pendências]({out / 'lista-pendencias.md'}) · [Parser]({out / 'parser-pendencias.md'}).
'''
(out / 'relatorio.md').write_text(report)
manifest = [f'{sha(p)}  {p.relative_to(out)}' for p in sorted(out.rglob('*'))
            if p.is_file() and p.name != 'manifest.sha256' and '__pycache__' not in p.parts]
(out / 'manifest.sha256').write_text('\n'.join(manifest) + '\n')
print(json.dumps({'closed': True, 'v16': [262,253], 'new_v19_notes': len(delta['new_v19_groups']),
                  'suite': state['suite'], 'peak_bytes': peak, 'manifest_files': len(manifest)}))
