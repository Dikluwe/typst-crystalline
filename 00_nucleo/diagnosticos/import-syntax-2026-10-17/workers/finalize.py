from pathlib import Path
import gzip,json,hashlib,datetime,subprocess,collections,difflib
out=Path(__file__).resolve().parents[1];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();b=json.load(gzip.open(out/'before.json.gz'));b['files'].update(json.load(gzip.open(out/'scope-extension.json.gz'))['files']);scope=json.load(open(out/'scope.json'));v=json.load(open(out/'validation-result.json'));assert v['complete'];assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==b['head'];hashes={p:sha(p) for p in set(b['hashes'])|set(scope['sources'])|set(scope['owners'].values())|set(scope['nuclei'])};assert hashes==v['source_hashes'];w=json.load(open(out/'workspace/workspace-result.json'));assert w['exit']==w['failed']==0 and w['passed']==8921 and w['ignored']==7;obs=json.load(open(out/'observables-result.json'));arch=json.load(open(out/'architecture-delta.json'));assert json.load(open(out/'public-contract-before.json'))==json.load(open(out/'public-contract-after.json'));assert json.load(open(out/'red-result.json'))['functional_red'];resources=[];seen=set();events=collections.Counter()
for p in sorted(out.rglob('*-resources.json')):
 r=json.load(open(p));key=json.dumps(r,sort_keys=True)
 if key in seen:continue
 seen.add(key);resources.append(dict(path=str(p),record=r));assert r['memory_before']['memory.max']==str(12*1024**3) and r['memory_before']['memory.high']==str(10*1024**3) and r['memory_before']['memory.swap.max']=='0';a=dict(x.split() for x in r['memory_before']['memory.events'].splitlines());z=dict(x.split() for x in r['memory_after']['memory.events'].splitlines());events.update({k:int(z[k])-int(a[k]) for k in z})
peak=max(int(r['record']['memory_after']['memory.peak']) for r in resources);assert events['max']==events['oom']==events['oom_kill']==0
measured=json.load(gzip.open(out/'after-probe.json.gz'))['binaries'];final_binaries={}
for name,info in measured.items():
 with open(info['path'],'rb') as stream:digest=hashlib.file_digest(stream,'sha256').hexdigest()
 assert digest==info['sha256'],(name,'binary changed after measurement');final_binaries[name]=dict(path=info['path'],sha256=digest)
state=dict(utc=now(),head=b['head'],binaries=final_binaries,source_hashes=hashes,status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),scope=scope,workspace=w,observables=obs,architecture=arch,memory=dict(peak_bytes=peak,peak_gib=peak/1024**3,events=dict(events),executions=resources),objective='active')
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(state,f)
(out/'changes.diff').write_text(''.join(''.join(difflib.unified_diff(b['files'].get(p,'').splitlines(True),Path(p).read_text().splitlines(True),fromfile='before/'+p,tofile='after/'+p)) for p in sorted(set(scope['sources'])|set(scope['owners'].values())|set(scope['nuclei']))))
(out/'follow-ups.md').write_text('''# Frentes posteriores

- Prioridade de continuação: retomar os 117 V16 por lotes, seguindo Clippy local → decisões Tekt → estrutura e contratos e preservando os complementos V19 uniformes que têm prescrição e cobertura. Clippy está limpo; esta etapa não reduziu a contagem V16.
- O corpus mantém três classes residuais: string sem fechamento (cinco rotas), diagnóstico hexadecimal inválido (cinco rotas) e morfologia do include válido (duas rotas). Os 12 resultados estão discriminados em observables-result.json. Não retirar espaços/erros do observável para declarar paridade.
- O contrato de document(format: format.html) permanece separado; context-format-function-attempt preserva a tentativa rejeitada pelo pin, e a prova válida deste lote usa document("child.html") com formato automático.
- O inventário global de V16/V19/V20 e RG1, FG1, RM1, SY1, igualdade de std entre arquivos e demais frentes anteriores continuam abertos. RG1 precisa de aprovação própria e rebase sobre o estado final atual.
- Três negativos matemáticos de âncora, medidos no lote call-once, permanecem abertos; não foram corrigidos por este trabalho de import/include.
- Não houve commit, push, merge, release ou supressão de lint. O objetivo global permanece ativo.
''')
(out/'relatorio.md').write_text(f'''# Relatório — sintaxe de import/include e contexto herdado

Etapa concluída. Fontes importadas e incluídas são validadas antes da avaliação: sintaxe inválida produz diagnóstico causal em vez de panic ou execução parcial. Módulos importados conservam as features e o target do chamador.

## Proveniência

HEAD `{b['head']}`, working tree não commitado. Baseline `{b['utc']}`, fechamento `{state['utc']}`. before.json.gz, red-state.json.gz, validation-result.json e final-state.json.gz preservam fontes/L0, hashes, UTC, status e git diff HEAD --stat. Os {len(b['hashes'])} hashes da etapa anterior foram conferidos no bootstrap; {len(hashes)} estão congelados no fechamento. Arquivos de sondas preservam argv, fixtures, saídas completas e SHA-256 dos binários. Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

## Alterações e L0

Quatro pares consumer/L0: compiler/eval/mod.rs, modules.rs, novo syntax_diagnostics.rs e tests.rs para a expectativa legada. O núcleo eval/source-validation.toml está pinado nos quatro owners. L0 foi escrito antes dos testes e da implementação; correção interna/de paridade em fluxo ADR-0127. Assinaturas, reexports e definições públicas foram comparados com o baseline; sem nova API Rust pública, default ou mudança de fase. Integridade dos arquivos fora do escopo foi conferida.

O helper puro converte erros sintáticos em SourceDiagnostic, classifica explicitamente os cinco SyntaxErrorKind existentes e mantém mensagens e todos os hints em ordem. NoTextWithinStars/Underscores continuam avisos não fatais, com span do markup delimitador e deduplicação pelo Sink existente. Documento principal, expressão, import e include compartilham essa política; nenhum catch_unwind produtivo foi adicionado.

Import e include validam a fonte depois da resolução e da verificação de ciclo, antes de executar código. Erros preservam a fonte real e acumulam a trilha de importação/inclusão aninhada. O contexto local do import herda full_error, features e target; escopo, estilos e show-rules continuam isolados conforme o contrato anterior. A construção de documento em módulo importado passa a funcionar no target Bundle, sem habilitá-la nos targets Paged/Html.

## Validação

- RED funcional: os oito testes novos falharam antes da implementação, sem erro de compilação. GREEN cobre sintaxe inválida nos três modos de import, include string/path, fontes de pacote em World de memória, aninhamento, prioridade do erro sintático sobre efeitos, avisos/deduplicação, oito perfis de features, três targets e erro de execução com trace.
- Corpus bilateral: 58 casos principais e 25 de contexto, total 83. Classificação observada: {obs['counts']}; {len(obs['changes'])} resultados Tekt mudaram. Resultados brutos e lista individual em observables-result.json. Os 12 resíduos são três classes: diagnóstico de string não fechada (cinco rotas), diagnóstico de hexadecimal inválido (cinco rotas) e morfologia de include válido (duas rotas). Rejeição com feature desligada é contabilizada à parte da execução com feature ligada.
- Comparação mantém mensagens, hints, spans e traces completos; desconta apenas o aviso Tekt preexistente de arquivo vazio por literal exato. Diferenças residuais de parser/morfologia permanecem explícitas. Bundle compara presença de child.html e todos os eventos HTML (elementos, atributos e texto); bytes e hashes das saídas também são preservados como evidência auxiliar.
- Suíte workspace: {w['passed']} aprovados, {w['failed']} falhas, {w['ignored']} ignorados. Build, formato, Clippy all-targets estrito, Rustdoc estrito/quatro índices, V1/V5/V15/V26, reverse lineage e diff --check passaram.
- Tekt global: {arch['after']}. Nenhum erro/aviso novo ou severidade relaxada. A nota V19 da classificação dos três tipos de erro migrou do hub para o novo helper com a mesma mensagem; uma nota V19 nova está apenas no helper de teste que confere o caminho comum a Import/Include, ambos exercitados pelos testes e pelas sondas de stderr. Essa cobertura decorre do L0 modules.md, que exige traces nos dois caminhos, pacote/path e aninhamento. Não há supressão. Os demais diagnósticos foram preservados por arquivo, regra, severidade e mensagem. O gate global continua bloqueado pelos V16 já existentes.
- RAM: MemoryMax 12 GiB, MemoryHigh 10 GiB, swap zero e Cargo jobs=1, execuções sequenciais. Pico medido {peak/1024**3:.6f} GiB ({peak} bytes); eventos {dict(events)}. Logs de recursos acompanham cada comando.

## Limites e continuação

A primeira suíte completa encontrou uma expectativa antiga de trace vazio. A fixture exata confirmou no pin, nos quatro perfis, que ela deve exigir Import("routes/error.typ") com span 2..36 da expressão. A expectativa foi atualizada preservando todas as demais assertivas e passou a conferir também caminho e span do trace. O L0 tests.md foi atualizado antes do teste; scope-extension.json.gz registra os textos/hashes originais. A primeira execução e seu source freeze estão em legacy-expected-attempt.

A comparação inicial e o primeiro diferencial ficaram preservados em initial-comparison. A primeira comparação só descontava o aviso de arquivo vazio no fim do stderr; a revisão retira exatamente um bloco literal desse aviso em qualquer posição e mantém todos os demais bytes. O primeiro diferencial exigia apenas a nota movida; a revisão identificou e justificou a nota adicional no teste, sem alterar fonte, severidade ou configuração. observables-reviewed-resources.json registra essa conferência separada.

Este lote fecha a validação de fontes antes de avaliar e a herança de contexto medidas. Não afirma paridade universal de parser, imports, HTML ou bundle. As diferenças remanescentes estão em observables-result.json e follow-ups.md. O objetivo global continua ativo. Os pacotes TS1 aprovado e RG1 pendente são verificados sem alteração na conferência de entrega. Sem commit/push/merge/release.
''');print('Closed',w['passed'],obs['counts'],len(obs['changes']),peak)
