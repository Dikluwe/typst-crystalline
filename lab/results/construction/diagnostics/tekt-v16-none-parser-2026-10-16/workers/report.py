from pathlib import Path
import json,gzip,hashlib,re,datetime,tarfile,collections,subprocess
out=Path('00_nucleo/diagnosticos/tekt-v16-none-parser-2026-10-16').resolve();root=Path.cwd()
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before=json.load(gzip.open(out/'before.json.gz'));package=json.load(open(out/'package-result.json'));consumer=json.load(open(out/'consumer-result.json'));green=json.load(open(out/'green-result.json'))
assert consumer['project_sources_unchanged'] and consumer['exact_other_diagnostics_preserved']
assert all(sha(p)==h for p,h in before['project_hashes'].items())
assert consumer['binaries']['new']['sha256']==package['binary']['sha256']
with tarfile.open(out/'candidate-source.tar.gz','r:gz') as archive:
 archived={p.name:hashlib.sha256(archive.extractfile(p).read()).hexdigest() for p in archive.getmembers() if p.isfile()}
assert archived==package['source_hashes']
suites=re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;', (out/'green-suite.log').read_text(),re.M)
totals={'passed':sum(int(x[1]) for x in suites),'failed':sum(int(x[2]) for x in suites),'ignored':sum(int(x[3]) for x in suites),'summaries':len(suites)}
assert totals['failed']==0 and totals['passed']==1146
resources={p.name:json.load(open(p)) for p in out.glob('*-resources.json')};peak=max(int(x['memory_after']['memory.peak']) for x in resources.values())
assert all(x['memory_before']['memory.max']=='12884901888' and x['memory_before']['memory.swap.max']=='0' and 'oom_kill 0' in x['memory_after']['memory.events'] for x in resources.values())
assert all(x['memory_after']['memory.events'].split('oom ')[1].startswith('0') for x in resources.values())
newrows=json.loads((out/'green-self-lint.log').read_text())['runs'][0]['results'];selfcounts=dict(collections.Counter(x['ruleId']+':'+x['level'] for x in newrows))
assert selfcounts=={'V19:note':74,'V20:note':17}
log=(out/'attempt-doc-green-doc.log').read_text();errors=[x for x in log.splitlines() if x.startswith('error:') and not x.startswith('error: could not')]
assert len(errors)==20
status=subprocess.check_output(['git','status','--porcelain=v1'],text=True);diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True)
final={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'project_head':before['project_head'],'project_source_hashes':before['project_hashes'],'project_status':status,'project_diffstat':diffstat,'candidate':package,'tests':totals,'resources':resources,'peak_bytes':peak,'peak_gib':peak/1024**3,'consumer':consumer,'self_lint':selfcounts,'archive_byte_identical_to_validated_sources':True,'public_api_preserved':True,'rustdoc_fixed_errors':20,'stage_validation_complete':True,'publication_pending':True,'active_ci_pin':package['base_commit'],'goal_scope_incomplete':True,'remaining_candidate_v16':244}
with gzip.open(out/'review-state.json.gz','wt') as f:json.dump(final,f)
observations=json.load(open(out.parent/'tekt-v16-value-projections-2026-10-16/none-parser-findings.json'))['observations']
lines=['# Lista de falsos V16 corrigidos no candidato','','Medição: mesmo working tree não commitado, HEAD `'+before['project_head']+'`; estado e diffstat em before.json.gz. O pin ativo ainda é e53de6f.','','| Consumer | Linha medida | Função |','|---|---:|---|']
for row in observations:
 lines.append('| ['+row['source']+']('+str(root/row['source'])+':'+str(row['line'])+') | '+str(row['line'])+' | `'+row['function']+'` |')
lines+=['','Todos são braços explícitos `None`, já fechados no domínio externo Option. Outros padrões internos como `Some(_)` não ficam certificados por esta correção. Nenhum consumer Typst foi reescrito para esconder o erro do parser.']
(out/'lista-correcoes.md').write_text('\n'.join(lines)+'\n')
report=f'''# V16 — None/prelude e validação do linter

Estado: candidato implementado e validado; publicação e pin aguardam autorização.

## Resultado

O candidato `{package['commit']}` (árvore `{package['tree']}`, branch `{package['branch']}`) corrige a classificação de `None`/`r#None` como binding genérico. No mesmo snapshot Typst, V16 **253 → 244**, com nove remoções e zero alterações nos demais diagnósticos: V19 permanece **888 notas** e V20 **871 notas**. O gate global continua falhando pelos 244 V16 restantes. O pin ativo ainda é `{package['base_commit']}` e continua emitindo 253; o efeito acima foi medido com o candidato, ainda local.

A [lista dos nove casos](lista-correcoes.md), os SARIF [anterior](consumer-old.log) e [novo](consumer-new.log), e [consumer-result.json](consumer-result.json) conservam mensagens, níveis e localizações.

## Correções

- Parser: extrair o binding uma vez; reconhecer None do prelude sem isentar nomes maiúsculos arbitrários. `_`, bindings comuns/raw e desativação local do prelude continuam V16. Atributos internos/externos, herança no mesmo arquivo, cfg_attr conservador, fronteiras de módulos, comentários/literais e guards são testados. Helpers privados, sem contrato público, dependência, I/O ou nova supressão.
- V19: um grupo novo legítimo (`line_comment | block_comment`) preservado e classificado individualmente no manifesto 0037; ambas as alternativas são exercitadas. O ratchet exato permanece ativo, zero V16/V17 no próprio linter. Auto-lint integral: 74 V19 e 17 V20, somente notas.
- Fixtures: quatro `.gitkeep` conservam diretórios vazios necessários em export/checkout novo. A falha anterior foi reproduzida também com o binário base; harness e expectativas preservados.
- Rustdoc: 20 erros de apresentação corrigidos em 11 consumers e respectivos L0. Somente doc-comments e metadados mudaram nesses consumers; código executável foi comparado e preservado. Nomes de tabelas, exemplos de citações e tipos usam código literal, sem links/HTML falsos.

O [patch completo](candidate.patch) inclui 30 arquivos: 12 consumers, seus 12 owners L0, dois documentos do inventário e quatro marcadores de diretório. [package-result.json](package-result.json) lista todos os caminhos/hashes e comprova o escopo. Não houve alteração no código/configuração/prompt produtivo do Typst nesta etapa, nem no checkout canônico do linter.

## Validação e recursos

- RED: seis falhas reproduzidas e três controles negativos passaram antes da implementação; [red-tests.log](red-tests.log).
- GREEN: nove testes específicos; suíte completa do linter **{totals['passed']} passaram, zero falhas, {totals['ignored']} ignorados preexistentes**; [green-suite.log](green-suite.log).
- Build, Clippy all-targets com `-D warnings`, Rustdoc com `-D warnings`, fmt e V1/V5/V15/V26 estritos passaram; [green-result.json](green-result.json). Rustdoc tinha 20 erros antes, preservados em [attempt-doc-green-doc.log](attempt-doc-green-doc.log), e agora passa.
- Consumidor: 16 testes Python do CI e dois testes Rust de integração passaram com o candidato no PATH; linhagem direta estrita/reversa e diff-check passaram. Comparação SARIF integral removeu somente nove V16 e manteve exatamente todos os outros resultados.
- Limite verificado por cgroup: **12 GiB**, swap **0**, Cargo **1 job**; pico registrado da etapa **{peak} bytes ({peak/1024**3:.3f} GiB)**. Sem OOM, OOM kill ou eventos de atingir o teto. [Registros de recursos](green-resources.json) e todos os ensaios em [review-state.json.gz](review-state.json.gz).

## Proveniência e entrega

Typst: HEAD `{before['project_head']}`, **working tree não commitado**. [before.json.gz](before.json.gz) conserva a lista exata `git diff HEAD --stat`, status, hashes completos de fontes/prompts/configuração e UTC; [review-state.json.gz](review-state.json.gz) conserva o estado após validação. As fontes permaneceram congeladas durante a medição. A data do diretório é 16/10 no fuso America/Sao_Paulo; horários UTC nos registros podem ser 17/10.

Linter base: `{package['base_commit']}`. Binário base SHA-256 `{before['old_binary']['sha256']}`. Candidato SHA-256 `{package['binary']['sha256']}`. Fontes compiladas são byte a byte iguais ao [arquivo exportado](candidate-source.tar.gz) e ao commit local; [bundle](candidate.bundle) conserva o commit com o base como pré-requisito. Os hashes bidirecionais dos owners editados foram verificados.

[proposed-pin.patch](proposed-pin.patch) prepara as quatro referências em `.github/workflows/ci.yml` e `tools/ci/README.md`, sem aplicar pin não publicado. A autorização anterior cobria somente publicar e53de6f; falta autorizar esta nova branch/commit. Não houve push, merge ou release. Após autorização, publicar só a branch, confirmar o commit remoto e então aplicar o pin; não fazer merge/release nem commit/push do Typst.

## Limites e próximos lotes

O parser continua local ao SourceFile: não expande macros nem resolve aliases/tipos ou ancestrais de módulos em arquivos externos. O snapshot Typst foi conferido sem no_implicit_prelude. Não é prova geral de resolução de nomes Rust. Some(_) e outros catchalls internos exigem análise própria.

O objetivo geral segue aberto: 244 V16 reais/pendentes no candidato, além da auditoria dos casos latentes. O próximo lote já foi medido: quatro decisões de Value em delimitadores matemáticos/vetores e presença de stroke de matriz, três owners, sem mudança pública. As listas não autorizam supressões globais nem a remoção de V19 informativo.
'''
(out/'relatorio.md').write_text(report)
(out/'review.md').write_text(f'''# Revisão para publicação — linter V16

Publicar somente `{package['branch']}` de Dikluwe/tekt-linter no commit `{package['commit']}`, filho direto de `{package['base_commit']}`, e depois aplicar o [pin preparado](proposed-pin.patch) no Typst. Sem merge ou release; sem commit/push do Typst.

[Relatório completo](relatorio.md) · [patch do linter](candidate.patch) · [lista dos nove falsos positivos](lista-correcoes.md).

Resultado validado: 253 → 244 V16, outros diagnósticos idênticos; 1146 testes do linter passaram (7 ignorados), nove regressões específicas, 16 testes Python e dois Rust de integração; Clippy, Rustdoc estrito, formato e linhagem passaram. Pico {peak/1024**3:.3f} GiB sob teto 12 GiB, swap zero, um job e sem OOM.

Inclui correção None/prelude, um grupo V19 justificado/testado, empacotamento de quatro fixtures e 20 erros documentais Rustdoc em 11 consumers com seus L0. Não altera APIs públicas, severidades ou configuração para mascarar V16. O pin atual continua no base; a publicação anterior autorizada não abrangia este novo commit.
''')
manifest={str(p.relative_to(out)):sha(p) for p in sorted(out.rglob('*')) if p.is_file() and p.name!='manifest-sha256.json'}
(out/'manifest-sha256.json').write_text(json.dumps(manifest,indent=2)+'\n')
assert all(sha(out/p)==h for p,h in manifest.items())
print(json.dumps({'review_commit':package['commit'],'tests':totals,'peak_gib':peak/1024**3,'manifest_files':len(manifest),'publication_pending':True}),flush=True)
