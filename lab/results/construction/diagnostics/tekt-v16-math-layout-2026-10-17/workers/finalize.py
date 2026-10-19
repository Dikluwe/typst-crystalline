from pathlib import Path
import collections, datetime, difflib, gzip, hashlib, json, re, subprocess

root = Path.cwd()
out = Path(__file__).resolve().parents[1]
now = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(p):
    h = hashlib.sha256()
    with Path(p).open('rb') as f:
        for block in iter(lambda: f.read(4 * 1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()

b = json.load(gzip.open(out / 'before.json.gz'))
scope = json.load(open(out / 'scope.json'))
v = json.load(open(out / 'validation-result.json'))
w = json.load(open(out / 'workspace/workspace-result.json'))
delta = json.load(open(out / 'architecture-delta.json'))
red = json.load(open(out / 'probe-baseline-result.json'))
green = json.load(open(out / 'probe-green-result.json'))
guard = json.load(open(out / 'guard-result.json'))
previous = json.load(open(out.parent / 'tekt-v16-eval-math-2026-10-17/workspace/workspace-result.json'))
assert v['validation_complete'] and v['sources_unchanged_during_validation']
assert all(sha(p) == h for p, h in v['source_hashes'].items())
assert all(c['exit'] == (1 if c['label'] == 'architecture' else 0) for c in v['commands'])
assert subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip() == b['head']
assert subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip() == 'Tekt'
changed = sorted(p for p, h in b['hashes'].items() if sha(p) != h)
assert changed == sorted(scope['existing_sources'] + scope['existing_owners'])
assert guard['new_concrete_tests'] == 29 and guard['public_declarations_preserved']
assert guard['replay_all_authorized_edits'] and guard['unrelated_sources_L0_configuration_unchanged']
assert red['selected'] == 0 and green['selected'] == 37
assert green['total_e0004'] == red['total_e0004'] + 37
assert not red['other_errors'] and not green['other_errors']
assert red['sources_unchanged'] and green['sources_unchanged']
assert '5 passed; 24 failed;' in (out / 'baseline-functional-red.log').read_text()
assert json.load(open(out / 'controls-resources.json'))['exit'] == 0
assert w['exit'] == 0 and w['failed'] == 0 and w['sources_unchanged']
assert w['passed'] == previous['passed'] + 29 and w['ignored'] == previous['ignored']
test_lines = re.findall(r'^test .*v16_math_source_.* \.\.\. ok$', (out / 'workspace/workspace.log').read_text(), re.M)
assert len(test_lines) == 29, test_lines
(out / 'controls-workspace-proof.json').write_text(json.dumps({'source': 'workspace/workspace.log', 'tests': test_lines, 'passed': len(test_lines), 'baseline': '5 passed; 24 failed;', 'separate_initial_green_receipt': 'controls-resources.json', 'initial_green_stdout_not_retained': True}, indent=2) + '\n')
assert delta['after'] == {'V16:error': 152, 'V19:note': 1028, 'V20:note': 869}
assert delta['all_other_diagnostics_preserved'] and len(delta['new_V19']) == 39
assert sha(b['linter']['binary']) == b['linter']['sha256']

probes = {}
for name in ['attach', 'family', 'family-v2', 'lines', 'markers']:
    r = json.load(open(out / ('source-after-' + name + '.json')))
    assert r['sources_unchanged'] and r['exit'] == r['build_exit'] == 0
    probes[name] = r
for name in ['attach', 'lines']:
    lines = probes[name]['stdout'].splitlines()
    assert len(lines) == 2 and lines[0].removeprefix('plain ') == lines[1].removeprefix('source ')
assert probes['lines']['stdout'].count('lines=2') == 2
lines = probes['markers']['stdout'].splitlines()
assert len(lines) == 4
assert all(a.replace('source=false', 'source=*') == z.replace('source=true', 'source=*') for a, z in zip(lines[::2], lines[1::2]))
assert len(probes['family-v2']['pairs']) == 23 and all(p['same'] for p in probes['family-v2']['pairs'])
docs = {str(root / 'target/doc' / crate / 'index.html'): sha(root / 'target/doc' / crate / 'index.html') for crate in ['typst', 'typst_core', 'typst_shell', 'typst_infra']}
resources = {str(p.relative_to(out)): json.load(open(p)) for p in out.rglob('*-resources.json')}
assert resources['validation-resources.json']['exit'] == 0
peak = max(int(r['memory_after']['memory.peak']) for r in resources.values())
assert peak <= 12 * 1024**3
pressure = 0
for r in resources.values():
    assert r['memory_before']['memory.max'] == '12884901888'
    assert r['memory_before']['memory.swap.max'] == '0' and r['cargo_jobs'] == '1'
    events = dict(line.split() for line in r['memory_after']['memory.events'].splitlines())
    assert events['oom'] == events['oom_kill'] == '0'
    pressure = max(pressure, int(events['max']))

patch = []
for p in scope['sources'] + scope['owners']:
    old = b['files'].get(p, '')
    patch.extend(difflib.unified_diff(old.splitlines(keepends=True), Path(p).read_text().splitlines(keepends=True), fromfile='a/' + p, tofile='b/' + p))
(out / 'stage.patch').write_text(''.join(patch))
probe_input = json.load(gzip.open(out / 'probe-green-input.json.gz'))
target_by_id = {t['id']: t for t in green['targets']}
cases = '# Casos do lote de layout matemático\n\n'
cases += 'Proveniência: before.json.gz/final-state.json.gz e probes baseline/green; 36 decisões originais, 18 emitidas e 18 latentes. Os L0 prescrevem o complemento nominal por caso antes do código. A linha abaixo aponta para o braço corrigido no estado final, não para o HEAD limpo.\n\n'
cases += '| ID | Consumer / função | Enum | Estado anterior | L0 proprietário |\n|---|---|---|---|---|\n'
for r in scope['targets']:
    t = target_by_id[r['id']]
    ast = probe_input['AST'][r['source']]
    a, z = ast['before'][t['match_index']], ast['after'][t['match_index']]
    index = [i for i, arm in enumerate(a['arms']) if arm['pattern'] == '_']
    assert len(index) == 1
    line = z['arms'][index[0]]['line']
    label = r['source'].split('/')[-1] + ':' + str(line)
    status = 'V16 emitido' if r['status'] == 'emitted-warning' else 'fallback latente'
    cases += f"| {r['id']} | [{label}]({root / r['source']}:{line}) · `{r['function']}` | {r['enum']} | {status} | [{Path(r['owner']).name}]({root / r['owner']}) |\n"
cases += '\nO encaminhamento auxiliar de partition_grid mantém o clone original e sua origem, com complemento nominal; ele é o 37º local da sonda. Os dois grupos do observador novo completam as 39 notas V19 novas. As notas continuam visíveis e não alegam cobertura de execução por alternativa. A projeção Option de attach é rasa e remove um V20 antigo. A lista/mensagem de cada diagnóstico está em architecture-delta.json.\n'
(out / 'casos.md').write_text(cases)
remaining = json.load(open(out / 'remaining-v16.json'))
counts = collections.Counter(r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./') for r in remaining)
assert sum(counts.values()) == 152
pending = '# Pendências após o lote de layout matemático\n\n'
pending += 'Proveniência: final-state.json.gz, HEAD e working tree não commitado, UTC/status/diffstat/hashes exatos. O gate global continua com exit 1 pelos 152 V16 abaixo. Detalhes e mensagens em remaining-v16.json.\n\n'
pending += '| Consumer | V16 |\n|---|---:|\n' + ''.join(f'| `{p}` | {n} |\n' for p, n in counts.most_common())
pending += '\n## Ordem de continuidade\n\n1. Auditar os próximos consumers de V16, com medição, L0 e regressões próprias; preservar grupos V19 que partilham uma decisão legítima. A tabela está ordenada por quantidade para inventário, não autoriza prioridade arquitetural nem alteração de API.\n2. Completar separadamente **math-stretch-composite**: o contrato vigente inclui eixo nativo, anexos e corpos compostos. Este lote só restaura a transparência de Source na elegibilidade atual; não fecha a implementação integral de stretch nem ratifica seu fallback parcial.\n3. Reavaliar as notas V19/V20 por decisão e cobertura, sem supressão global. As 39 notas V19 deste lote estão justificadas nos owners; executar um braço não prova exercitar cada alternativa.\n\nNenhuma alteração de contrato público está autorizada por esta lista. Continuam aplicáveis os gates ADR-0127, a linhagem 1:1 e o limite de 12 GiB. Sem release nesta etapa.\n'
(out / 'pendencias.md').write_text(pending)

report = f'''# V16 — layout matemático e transparência de Source

Etapa concluída em {now()}. **V16 170 → 152**, **V19 989 → 1028 notas**, **V20 870 → 869 notas**. Os consumers deste lote ficaram sem V16 emitido. O gate global continua com exit 1 pelos **152 V16 restantes**; o objetivo geral permanece aberto.

## O que foi corrigido

Fechados **18 V16 emitidos e 18 fallbacks latentes**, mais um encaminhamento nominal auxiliar em partition_grid. Foram auditados Content, Value, MathStyleKind e FrameItem em 12 consumers e seus 12 owners, incluindo um consumer/owner novo de testes. A [lista dos 36 casos](casos.md) aponta para código e L0; guards, complementos e motivos individuais estão em [scope.json](scope.json). Campos e assinaturas públicas foram preservados. Os grupos V19 mantêm o corpo anterior; variantes com guards recusados continuam chegando ao complemento. Nenhuma supressão foi adicionada.

As medições anteriores mostravam defeitos reais ao acrescentar Source ao mesmo conteúdo matemático. A largura de limits inline mudava de **7.2 para 12.912 pt**, as linhas de uma equação mudavam de **2 para 1** e a última baseline de **16.2 para 0 pt**. A matriz complementar tinha **19 diferenças em 23 pares**. Source agora é normalizado por empréstimo na classificação de limites, scripts/kerns, texto de operadores, acentos/dotless, suportes, alinhamento de matrizes, espaçamento, sequências, stretch e classes. Quebras e pontos de alinhamento também são reconhecidos sob Source. Conteúdo derivado preserva a primeira origem causal pelos helpers existentes; o teste de flatten verifica a precedência entre origem do filho e origem herdada.

Após o build, os mesmos programas públicos e métricas foram repetidos com a biblioteca reconstruída: **todos os 23 pares coincidem**, limits retorna **7.2/7.2 pt**, metadados da equação retornam **2/2 linhas e baseline 16.2/16.2 pt**. As posições dos marcadores de alinhamento também coincidem. Os stdout, fontes da sonda, rlib, comandos e hashes anteriores/posteriores estão preservados nos source-*.json. São provas dessas geometrias e metadados; não alegam paridade integral de matemática com o vanilla.

A projeção Option de attach foi decomposta em `?` e match nominal raso de Content. Isso remove o V20 antigo desse padrão, sem introduzir outro. São **39 V19 novos**: 36 complementos originais, um encaminhamento de grid e dois grupos do observador de testes. A nota V19 da projeção foi atualizada para sua forma rasa. Todos os demais diagnósticos foram preservados, incluindo seus campos e posições mapeadas por AST/linhas. [architecture-delta.json](architecture-delta.json).

## Validação

O RED funcional teve **5 testes passando e 24 falhando**. O GREEN inicial terminou com exit 0; seu recibo de recursos foi preservado, mas o stdout dessa chamada não ficou retido. A suíte completa posterior registra nominalmente os **29 controles passando**, sem relaxar as expectativas. Foram acrescentados 29 testes concretos (sete definições anotadas, incluindo macro com 23 casos). O observador rejeita itens não suportados e compara texto, posições, dimensões, metadados de linhas e substituições lógicas. [controls-workspace-proof.json](controls-workspace-proof.json).

A suíte workspace terminou com **{w['passed']} passando, zero falhas e {w['ignored']} ignorados**, acréscimo de 29 sobre a entrada. A sonda descartável adicionando variantes aos quatro enums passou de **0 para 37 E0004 nos locais selecionados**; total **{red['total_e0004']} → {green['total_e0004']}**, sem outro erro. Os 36 locais originais e o encaminhamento auxiliar agora exigem revisão do compilador quando o domínio cresce. As fontes reais não foram modificadas pela sonda.

Passaram build workspace, fmt, Clippy workspace/all-targets com `-D warnings` e `-D unfulfilled_lint_expectations`, Rustdoc com `-D warnings` e quatro índices HTML, V1/V5/V15/V26 estrito, linhagem reversa, replay das alterações e diff-check. [validation-result.json](validation-result.json), [workspace/workspace-result.json](workspace/workspace-result.json), [guard-result.json](guard-result.json), [stage.patch](stage.patch).

## Recursos e proveniência

Comandos pesados sequenciais sob **MemoryMax=12 GiB**, **swap zero** e **Cargo um job**. Pico do cgroup: **{peak} bytes ({peak / 1024**3:.3f} GiB)**, sem OOM/OOM kill. Maior memory.events/max: **{pressure}**. Esse dado mede o cgroup; não prova ganho de desempenho nem representa RSS isolado.

HEAD `{b['head']}`, branch Tekt, **working tree não commitado**. [before.json.gz](before.json.gz) e [final-state.json.gz](final-state.json.gz) registram UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat exato. Fontes ficaram congeladas nos gates. Resselo limitado aos 12 pares, com grafo de ownership/núcleos validado. Linter `{b['linter']['commit']}`, SHA-256 `{b['linter']['sha256']}`. Sem alteração de pin CI, commit, push, merge ou release do Typst.

Foram preservados dois problemas de montagem das provas: contagem inicial do analisador suplementar (24 em vez dos 23 pares já medidos) e uso de Point::new inexistente no teste. Foram corrigidos sem contar erro de compilação/análise como RED funcional; recibos em source-family-analysis-repair.json, point-repair.json e attempt-point/.

O primeiro Clippy estrito apontou uma clonagem usada apenas para montar uma fatia de um item no teste novo. O L0 do teste foi atualizado antes da troca para `std::slice::from_ref`, mantendo dados e expectativas; o candidato anterior e sua falha estão em attempt-clippy/, e o replay inclui esse único refinamento. Todos os gates foram reexecutados sobre o estado final.

## Continuidade

[Lista organizada dos 152 V16 pendentes](pendencias.md), [locais e mensagens completos](remaining-v16.json). **math-stretch-composite continua aberto**: o contrato já aprovado exige anexos/corpos compostos/eixo nativo e não é estreitado pelo reparo atual de Source. V19 permanece informativo e visível; ausência de V16 não demonstra cobertura de cada alternativa.
'''
(out / 'relatorio.md').write_text(report)
record = dict(utc=now(), head=b['head'], status=subprocess.check_output(['git', 'status', '--porcelain=v1'], text=True), diffstat=subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True), source_hashes=v['source_hashes'], changed_existing=changed, added=scope['new_sources'] + scope['new_owners'], scope=scope, tests=w, architecture=delta, probes={'before': red, 'after': green}, resources=resources, peak_bytes=peak, docs=docs, linter=b['linter'], new_tests=29, stage_complete=True, goal_scope_incomplete=True, remaining_v16=152, open_named_debt=['math-stretch-composite'], no_commit_push_release=True)
with gzip.open(out / 'final-state.json.gz', 'wt') as f:
    json.dump(record, f)
manifest = {str(p.relative_to(out)): sha(p) for p in sorted(out.rglob('*')) if p.is_file() and p.name != 'manifest-sha256.json'}
(out / 'manifest-sha256.json').write_text(json.dumps(manifest, indent=2) + '\n')
assert all(sha(out / p) == h for p, h in manifest.items())
print(json.dumps(dict(stage_complete=True, remaining_V16=152, V19=1028, V20=869, passed=w['passed'], failed=0, ignored=w['ignored'], peak_gib=peak / 1024**3, manifest_files=len(manifest), goal_scope_incomplete=True)), flush=True)
