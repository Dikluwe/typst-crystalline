from pathlib import Path
import json
import gzip
import hashlib
import re
import subprocess
import datetime

root = Path.cwd()
out = root / '00_nucleo/diagnosticos/tekt-linter-update-2026-10-16'
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
before = json.load(gzip.open(out / 'before.json.gz'))
validation = json.loads((out / 'validation.json').read_text())
assert validation['project_sources_unchanged']
applied = json.loads((out / 'applied.json').read_text())
publication = json.loads((out / 'publication.json').read_text())
remote = json.loads((out / 'remote-verification.json').read_text())
assert publication['exit'] == remote['exit'] == 0
assert remote['fetched_commit'] == before['new_pin']
assert remote['fetched_tree'] == json.loads((out / 'provenance.json').read_text())['tree']
assert sorted(p for p, h in before['hashes'].items() if sha(root / p) != h) == sorted(applied['changed'])
assert all(sha(root / p) == h for p, h in applied['hashes'].items())
assert all(x['exit'] == (1 if x['label'] == 'new-architecture' else 0)
           for x in validation['commands'])
assert validation['commands'][-1]['label'] == 'project-integration'
resources = [json.loads((out / p).read_text()) for p in
             ['initial-validation/resources.json', 'resources.json']]
peak = max(int(x['memory_after']['memory.peak']) for x in resources)
for item in resources:
    assert item['memory_after']['memory.max'] == str(12 * 1024**3)
    assert item['memory_after']['memory.swap.max'] == '0'
    events = dict(line.split() for line in item['memory_after']['memory.events'].splitlines())
    assert events['oom'] == events['oom_kill'] == '0'
summaries = {}
for name in ['v19-unit', 'v19-report-cli', 'project-integration']:
    matches = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed;', (out / (name + '.log')).read_text())
    assert len(matches) == 1
    summaries[name] = {'passed': int(matches[0][0]), 'failed': int(matches[0][1])}
python_log = (out / 'ci-python.log').read_text()
assert re.search(r'\nOK\s*$', python_log)
summaries['ci-python'] = {'passed': int(re.search(r'Ran (\d+) tests', python_log)[1]), 'failed': 0}
subprocess.run(['git', 'apply', '--reverse', '--check', str(out / 'update-pin.patch')], check=True)
for rel in ['.github/workflows/ci.yml', 'tools/ci/README.md']:
    original = (out / 'before' / rel).read_text()
    candidate = (out / 'candidate' / rel).read_text()
    assert candidate == original.replace(before['old_pin'], before['new_pin'])
    assert candidate.count(before['new_pin']) == 2 and before['old_pin'] not in candidate
    assert (root / rel).read_text() == candidate
state = {
    'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'project_head': before['head'], 'working_tree': 'não commitado',
    'status': 'updated',
    'old_pin': before['old_pin'], 'candidate_pin': before['new_pin'],
    'active_ci_pin': before['new_pin'],
    'project_sources_unchanged': True,
    'new_binary_sha256': validation['new_binary_sha256'],
    'candidate_patch_sha256': sha(out / 'update-pin.patch'),
    'tests': summaries, 'peak_bytes': peak, 'memory_max_bytes': 12 * 1024**3,
    'swap_max_bytes': 0, 'oom': 0, 'oom_kill': 0,
    'architecture_global_exit': 1,
    'v16_errors': 267,
    'publication': 'https://github.com/Dikluwe/tekt-linter/tree/codex/p0119-safe-config-init',
    'remote_commit_and_tree_verified': True,
    'changed': applied['changed'],
    'final_hashes': {p: sha(root / p) for p in before['hashes']},
    'final_status': subprocess.check_output(['git', 'status', '--porcelain=v1'], text=True),
    'final_diffstat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True),
}
(out / 'final-state.json').write_text(json.dumps(state, ensure_ascii=False, indent=2) + '\n')
report = f'''# Atualização do linter — concluída

Pin anterior: `{before['old_pin']}`. Pin atual: `{before['new_pin']}`.
A branch `codex/p0119-safe-config-init` do tekt-linter foi publicada com
autorização humana explícita. `git fetch --depth=1` do commit exato confirmou
acesso remoto e identidade da árvore com a fonte validada.
Recibos: `approval-receipt.json`, `publication.json` e `remote-verification.json`.

## Alteração aplicada

`update-pin.patch` troca exclusivamente os dois pins em `.github/workflows/ci.yml`
e as duas referências de reprodução em `tools/ci/README.md`. A aplicação foi
verificada antes com `git apply --check` e depois com o check reverso e a
identidade integral com os dois arquivos candidatos. Os arquivos ativos usam
o novo pin publicado. `applied.json` contém hashes e escopo exato.
Não houve edição de fontes produtivas, L0 ou configuração de severidade.

O patch tem SHA-256 `{state['candidate_patch_sha256']}`.
`publication-scope.json` contém todos os commits locais ancestrais incluídos na
publicação autorizada. Não houve merge em master nem criação de release.
O primeiro download havia falhado com `not our ref`, preservado em
`provenance.json`; a publicação foi feita somente após concluir os testes e obter
a aprovação. `before-publication/` conserva o relatório e o estado revisados.

## Validação local

Compilação offline/locked a partir de `git archive` do commit candidato,
em pasta temporária, sem alterações no checkout do linter. A identidade da árvore,
hash do archive, fontes congeladas, comandos e SHA-256 do binário estão em
`provenance.json` e `validation.json`.

- Testes unitários selecionados por V19: {summaries['v19-unit']['passed']} aprovados.
- Integração do relatório V19: {summaries['v19-report-cli']['passed']} aprovados.
- Auxiliares Python do CI: {summaries['ci-python']['passed']} aprovados.
- Integração crystalline_lint do typst-wiring com o executável novo no PATH:
  {summaries['project-integration']['passed']} aprovados.
- Gates V1/V5/V15/V26 estritos e linhagem reversa: aprovados.

## Impacto real no gate global

A versão nova aplica a política já existente `V16 = {{ level = "error" }}`.
Assim, os 242 warnings e 25 notes V16 anteriores passam a 267 erros.
O gate global retorna exit 1. Não foram encontrados diagnósticos não-V19 novos
ou removidos ao comparar regra, mensagem e localização sem a severidade.
`architecture-comparison.json` conserva as duas comparações, com e sem nível;
`remaining-v16.json` registra cada V16 e seu nível anterior.

V19 mantém 864 notas informativas, com mensagem observacional e contagem revista;
V20 mantém 871 notas. A atualização do pin preserva a falha global
decorrente dos V16 existentes; esta etapa não corrige esses casos nem relaxa o gate.
A interrupção inicial na arquitetura foi preservada em `initial-validation/`;
a retomada verificou explicitamente essa mudança de severidade e concluiu os
demais testes. Não há alegação de CI global verde.

## RAM e proveniência

Execuções sequenciais em cgroup com MemoryMax=12 GiB, MemorySwapMax=0 e
CARGO_BUILD_JOBS=1. Maior pico: {peak} bytes ({peak / 1024**3:.3f} GiB),
zero OOM e OOM-kill. Recibos em `resources.json` e
`initial-validation/resources.json`.

Projeto HEAD `{before['head']}`, working tree não commitado.
Snapshot inicial UTC `{before['utc']}`; validação UTC
`{validation['start']}`–`{validation['end']}`. Lista exata de fontes e hashes,
status completo e git diff HEAD --stat em `before.json.gz`. Fontes, L0,
configuração e arquivos operacionais monitorados permaneceram idênticos durante
a validação. Depois dela, somente os dois arquivos operacionais declarados foram
alterados. `final-state.json` contém hashes finais, status e diffstat completos.
Não houve commit ou push no typst-crystalline. O único push foi a branch do
linter autorizada acima; sem merge e sem release. O workflow remoto do
typst-crystalline não foi executado nesta sessão; o gate global local fica vermelho
pelas pendências V16 existentes, e não há alegação de CI global verde.
'''
(out / 'relatorio.md').write_text(report)
entries = [f'{sha(p)}  {p.relative_to(out)}' for p in sorted(out.rglob('*'))
           if p.is_file() and p.name != 'manifest.sha256' and '__pycache__' not in p.parts]
(out / 'manifest.sha256').write_text('\n'.join(entries) + '\n')
print(json.dumps(state, ensure_ascii=False))
