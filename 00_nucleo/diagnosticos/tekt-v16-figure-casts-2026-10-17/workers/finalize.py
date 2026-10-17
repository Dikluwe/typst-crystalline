from pathlib import Path
import collections,datetime,difflib,gzip,hashlib,json,subprocess
out=Path(__file__).resolve().parents[1]
def sha(p):
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  for data in iter(lambda:f.read(1048576),b''):h.update(data)
 return h.hexdigest()
b=json.load(gzip.open(out/'working-baseline.json.gz'));v=json.load(open(out/'validation-result.json'));scope=json.load(open(out/'scope.json'));assert v.get('complete') and v['sources_unchanged'];assert all(sha(p)==h for p,h in v['source_hashes'].items());assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==b['head'];changed=[p for p,h in b['hashes'].items() if sha(p)!=h]+scope.get('nuclei',[]);assert set(changed)==set(scope['sources'])|set(scope['owners'].values())|set(scope.get('nuclei',[]));prior=json.load(gzip.open('00_nucleo/diagnosticos/tekt-v16-catalogos-2026-10-17/final-state.json.gz'));assert all(v['source_hashes'][p]==h for p,h in prior['source_hashes'].items() if p not in changed)
suite=json.load(open(out/'workspace/workspace-result.json'));assert (suite['exit'],suite['passed'],suite['failed'],suite['ignored'])==(0,8836,0,7);assert '0 passed; 2 failed;' in (out/'red-symbol.log').read_text();assert '1 passed; 4 failed;' in (out/'red.log').read_text();assert '0 passed; 1 failed;' in (out/'red-located.log').read_text();assert '2 passed; 0 failed;' in (out/'validation-symbols-after.log').read_text();assert '6 passed; 0 failed;' in (out/'validation-controls-after.log').read_text();assert '1 passed; 0 failed;' in (out/'validation-cast-controls-after.log').read_text();assert len(json.load(open(out/'red-probe-result.json'))['results'])==4;assert len(json.load(open(out/'green-probe-result.json'))['results'])==4
shape=json.load(open(out/'caption-shape-observables-result.json'));assert shape['pairs']==15;assert '0 passed; 4 failed;' in (out/'red-caption-shape-all.log').read_text();assert '0 passed; 1 failed;' in (out/'red-caption-casts.log').read_text();assert '5 passed; 0 failed;' in (out/'validation-caption-shape-controls.log').read_text();assert '1 passed; 0 failed;' in (out/'validation-caption-shape-controls.log').read_text();caption=json.load(open(out/'caption-observables-result.json'));assert caption['repaired']==['empty','content','explicit-caption'];assert '0 passed; 1 failed;' in (out/'red-caption.log').read_text();assert '1 passed; 0 failed;' in (out/'validation-caption-controls.log').read_text();more=json.load(open(out/'more-observables-result.json'));assert more['pairs']==70 and more['morphology_gap_explicit'];obs=json.load(open(out/'observables-result.json'));assert obs['pairs']==130 and obs['no_unexpected_changes'];delta=json.load(open(out/'architecture-delta.json'));assert delta['after']['V16:error']==117 and delta['no_new_errors_or_warnings'];assert len(json.load(open(out/'cast-integrity-result.json'))['results'])==2
packages={'text-size-length-2026-10-17':'771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba','table-row-groups-2026-10-17':'a47fe97272fd6a5bd96022be959ba277a317a74086f5e5f4a569bba008f4fae9'}
for name,h in packages.items():
 p=Path('00_nucleo/materialization')/name;assert sha(p/'SHA256SUMS')==h
 for line in (p/'SHA256SUMS').read_text().splitlines():want,f=line.split('  ',1);assert sha(p/f)==want
resources={};resource_aliases={};unique={}
for p in sorted(out.rglob('*resources.json')):
 r=json.load(open(p));k=json.dumps(r,sort_keys=True);name=str(p.relative_to(out))
 if k in unique:resource_aliases[name]=unique[k]
 else:unique[k]=name;resources[name]=r
peak=max(int(r['memory_after']['memory.peak']) for r in resources.values());events={n:sum(int(next(l.split()[1] for l in r['memory_after']['memory.events'].splitlines() if l.startswith(n+' '))) for r in resources.values()) for n in ['high','max','oom','oom_kill']}
for r in resources.values():
 m=r['memory_after'];assert (m['memory.max'],m['memory.high'],m['memory.swap.max'])==('12884901888','10737418240','0')
assert events['oom']==events['oom_kill']==0
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],working_tree='uncommitted',status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=v['source_hashes'],changed_since_previous=changed,tests={k:suite[k] for k in ['passed','failed','ignored']},observables=obs,architecture=delta,peak_bytes=peak,memory_events=events,resources=resources,resource_aliases=resource_aliases,public_api_changes=0,global_goal='active: V16, figure kind/synthesis, relative measures and independent parity debts remain',proposal_manifests_preserved=packages)
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(state,f)
(out/'patch.diff').write_text(''.join(''.join(difflib.unified_diff(b['files'].get(p,'').splitlines(True),Path(p).read_text().splitlines(True),fromfile='before/'+p,tofile='after/'+p)) for p in changed))
rows=json.load(open(out/'validation-architecture.log'))['runs'][0]['results'];remaining=[r for r in rows if r['ruleId']=='V16'];(out/'remaining-v16.json').write_text(json.dumps(remaining,indent=2)+'\n');groups=collections.defaultdict(list)
for r in remaining:
 loc=r['locations'][0]['physicalLocation'];groups[loc['artifactLocation']['uri']].append(loc['region']['startLine'])
(out/'inventario-v16-restante.md').write_text('# V16 após validação de figura e casts\n\n117 erros em '+str(len(groups))+' consumers. Proveniência: final-state.json.gz e SARIF integral.\n\n| Consumer | Casos | Linhas |\n|---|---:|---|\n'+''.join(f'| `{p}` | {len(ns)} | {", ".join(map(str,ns))} |\n' for p,ns in sorted(groups.items())))
(out/'pendencias.json').write_text(json.dumps(dict(V16=117,FG1='Typed kind/synthesis and traversal; infer_kind V16 remains visible. Prepare and approve public proposal before implementation.',RM1='Relative measure carriers and per-consumer casts; percentages are still truncated in extract_length. Exhaustive coverage is not parity closure.',RG1='Pending existing approval, independent of FG1/RM1.',SY1='SymbolElem carrier/reflection/morphology pending public proposal; current casts preserve glyph as Text.',native_identity_audit='Audit all registry element callbacks against kind/where metadata before claiming a complete catalogue; caption row corrected in this stage.',caption_shape_limits=shape['partial_and_pending'],caption_presence_limits=caption['pending_presence_and_diagnostic_gaps'],diagnostic_gaps=obs['remaining_diagnostic_gaps'],preserved_prior_debts=['18 catalogue diagnostics','Text Paint Gradient/Tiling and relative:self diagnostic','show text selector','Heading defaults when replaced by show function','Script size/offset parity','Custom-only constructor presence outside equality projection','Fractional Box1fr, multi-page SVG, integer literal minimum']),indent=2)+'\n')
repairs=len(obs['repairs']);gaps=len(obs['remaining_diagnostic_gaps'])
(out/'relatorio.md').write_text(f'''# Relatório — rejeições de figura e cobertura dos casts

Etapa concluída com correções funcionais em `figure` e **três V16 corrigidos, de 120 para 117**. O V16 da inferência parcial de kind continua visível: não foi silenciado com uma lista de containers neutros. O objetivo global permanece aberto. Sem nova API pública, mudança de fase, commit/push do Typst, merge ou release.

## O que mudou

`native_figure` agora rejeita body inválido em vez de criar conteúdo vazio, caption inválida em vez de pintar o nome do tipo, e kind inválido em vez de cair em inferência. Funções arbitrárias, With e valores que só imitam o nome de um elemento são rejeitados como `expected element`; a classificação usa a tabela de identidade nativa existente. Tipos com construtor são recusados como elemento e tipos sem construtor preservam seu diagnóstico específico. Símbolos conhecidos como acento/LR produzem expected element, outros produzem symbol {{valor}} is not callable, usando helpers privados nos owners de acento/LR e bridges diretos no hub. None é corpo vazio e ausência de caption, mas é inválido como kind. Strings, conteúdo e nomes explícitos válidos são preservados. Ausência de body usa `missing argument: body`.

Símbolos em body/caption preservam seu grapheme efetivo como Text. Isso melhora a preservação do conteúdo; SY1 permanece aberto para SymbolElem/reflexão/morfologia mediante revisão pública. LocatedContent é aceito como conteúdo em body e caption, conservando o conteúdo e sua origem Source. Os erros seguem body→caption→kind, mesmo se a ordem dos argumentos nomeados for invertida. A âncora vem do value_span da ocorrência correspondente, com fallback explícito; não é deduzida por igualdade Value. Os argumentos não são reavaliados.

Nos dois conversores privados, os cinco RHS existentes são preservados. As outras 34 variantes Value de cada projeção são explícitas e retornam None para o caller validar. Isso elimina dois V16 e exige classificação de uma variante futura, sem confundir a cobertura estrutural com semântica correta de percentuais. Os grupos V19 têm essa responsabilidade local prescrita no L0 e permanecem como notas, sem supressão.

A projeção de legenda presente agora devolve o carrier FigureCaption existente, conservando seu corpo e deixando defaults não resolvidos. A lógica fica no elemento; Content::get_field apenas delega. Seis sondas de reflexão corrigem três casos (legenda vazia, conteúdo e constructor explícito), preservam body e registram as duas lacunas de omissão/none. Não foram adicionados campos ou métodos de trait; o override usa a assinatura já existente.

## Verificação

Houve **14 RED funcionais reais**: quatro grupos de rejeição/precedência, um caso de conteúdo localizado, dois casos simbólicos, uma projeção de legenda, cinco casos de getters/metadados/encoders/CLI e um caso de casts do constructor de caption. Os **16 testes novos** passaram: esses 14 RED e dois controles funcionais. As sondas estruturais dos dois matches aceitam os enums atuais e rejeitam variante futura com E0004; os padrões antigos aceitavam a variante. Elas usam padrões produtivos, nomes/aridades reais e payloads stub/RHS apagado, sem mutar o workspace Cargo; não são prova de execução equivalente.

**130 pares bilaterais repetidos**: {repairs} casos de diagnóstico foram corrigidos e coincidem com o pin em código de saída, mensagem principal, primeira âncora e hints medidos. O complemento mede outros 70 pares: {len(more['correct_type_or_symbol_diagnostics'])} diagnósticos Type/Symbol coincidem com o pin, {len(more['symbol_text_preserved'])} controles conservam o texto simbólico e três construtores de símbolo inválido são preservados. O tipo de nó Text versus Symbol continua diferente e registrado.

Os outros {obs['unchanged_tekt_cases']} casos conservam exatamente stdout, stderr e código de saída dentro do cristalino; o oráculo também permaneceu idêntico. Há {gaps} diferenças de diagnóstico anteriores restantes neste corpus; JSON/fields, reflexão, síntese e medidas relativas continuam sem certificação global de paridade. Consulte observables-result.json para cada caso.

**Suíte completa: 8836 aprovados, zero falhas e sete ignorados.** Build workspace, fmt, Clippy all-targets com -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito, reverse lineage e diff passaram. Resselo restrito aos doze owners após V15/V26 válido. Nenhum novo erro/warning; todos os diagnósticos fora dos doze consumers permanecem idênticos. SARIF: **{delta['after']}**. O gate global continua exit 1 pelos 117 V16 restantes.

A projeção dos campos já transportados pela caption também corrige body/position/separator conhecidos, callback nativo refletido, metadados da função de elemento, JSON/YAML CLI e json.encode. As 15 sondas adicionais têm {len(shape['exact_language_values'])} valores completos coincidentes com o pin e {len(shape['partial_and_pending'])} limites explícitos em caption-shape-observables-result.json; os limites não contam como paridade aprovada. O constructor qualificado aceita as coerções de conteúdo medidas, sem representar separator:auto com um sentinel.

## Limites preservados e próximos passos

**FG1 permanece incompleto:** FigureElem.kind:Option<String> não distingue uma função de elemento da string homônima. Funções de elemento válidas continuam no caminho histórico; sua identidade, síntese, presença de caption omitida/none, metadados de position/separator, contadores e seletores exigem revisão pública própria. A inferência atual também não percorre todos os wrappers/containers. Não foi movida de fase nem declarada correta nesta etapa.

**SY1 permanece incompleto:** o modelo de Content ainda não distingue o carrier SymbolElem. Preservar o texto não prova igualdade de função/reflexão/morfologia com o pin.

**RM1 permanece incompleto:** extract_length ainda reduz Relative à parte absoluta e Ratio a zero; números são aceitos em alguns callers onde o pin rejeita. Essa aceitação consta de contratos históricos e não é automaticamente um defeito: RM1 deverá distinguir extensão intencional de perda de informação antes de mudar compatibilidade. Spacing e dimensões de Box possuem carriers públicos de Length; Block já conserva Rel e não foi regredido. A correção completa exige medir cada consumer e preparar contratos, migrações e testes antes do gate ADR-0127. A aprovação TS1 não cobre essas alterações.

[FG1/RM1](follow-ups.md) nomeiam os passos que completam os subconjuntos; os L0 declaram a incompletude. contract-dependencies-before.json registra a busca lexical de dependências, incluindo testes, como ponto de partida, não como lista completa de migração autorizada. RG1 exige rebase sobre a árvore atual, inclusive as novas delegações de Content::get_field e a projeção CLI; não sobrescrever por candidatos antigos. O novo núcleo caption-projection compartilha quatro invariantes entre oito owners com pins completos; código continua apontando somente aos seus owners 1:1. As 18 diferenças de diagnóstico do lote de catálogos continuam independentes, assim como RG1 pendente, Paint textual, seletor show text e demais débitos em pendencias.json. [V16 restantes](inventario-v16-restante.md).

## RAM e proveniência

Comandos pesados sequenciais sob **12 GiB rígidos, high 10 GiB, swap zero e Cargo jobs=1**. Pico **{peak} bytes ({peak/1024**3:.3f} GiB)**; eventos agregados de execuções únicas {events}. Sem OOM/OOM-kill. Memória de cgroup inclui cache e não é RSS nem prova de otimização.

HEAD **{b['head']}**, working tree não commitado. before/final-state.json.gz, logs, RED/GREEN, recursos e corpora fixam UTC, status/diffstat, fontes/L0/config/binários e comandos. [Patch desta etapa](patch.diff). Mudaram doze consumers, seus doze L0 e o novo núcleo caption-projection desde o checkpoint anterior; TS1, igualdade Styled e lote de catálogos foram preservados por hash. Manifestos TS1/RG1 conferidos integralmente e preservados.

Vanilla pin **9f2b6e8715237cb086899a42873660fe744622e8**, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. Linter pin **12797073972e375d9ab078cf6750c8b30b71e2b9**, binário SHA-256 {b['linter']['sha256']}. Fluxo contínuo ADR-0127 para as correções internas entregues; nenhuma aprovação de FG1/RM1 foi inferida.
''')
print(json.dumps(dict(tests=suite['passed'],v16=117,repairs=repairs,peak_bytes=peak,changed=changed)))
