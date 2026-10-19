# L0 — introspect/extract_payload
Hash do Código: dc399222

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/extract_payload.rs`.
The function at `:18` delegates to element-owned to_payload implementations;
it ends in `_ => None`. The parent walk at
`01_core/src/compiler/introspect.rs:1915` consumes its result before recursion.
The current function is not compiler-exhaustive merely because its dispatch is
static. The exhaustive classification owner is
`00_nucleo/prompts/compiler/introspect/locatable.md`.

## Contract

Preserve `pub fn extract_payload(content: &Content) -> Option<ElementPayload>`.
It is pure, deterministic for the same modeled input, independent of counters,
walk context, Locator and Engine. Cloning domain data is permitted; do not
perform I/O, walk descendants or execute callbacks.

Delegate directly to the native element for Strong, Emph, Heading, Figure,
Cite, Metadata, State, StateUpdate, StateDisplay, CounterDisplayCallback,
Outline, Bibliography, Equation, Table, Footnote, CounterUpdate and ContextBlock.
Dynamic uses its existing dyn_to_payload boundary, not a new dispatch mechanism.
All remaining variants return None in this dispatcher.

Payload formulas and domain fields belong to their entity L0s, not to a
duplicated table in this module. Effective lexical numbering/supplement/alt
capture belongs to `00_nucleo/prompts/compiler/introspect.md`, after extraction.
Do not invent payloads from bold/italic flags or descend through Styled:
the child's normal walk discovers its own occurrence.

## Acceptance and alignment boundary

Check native delegate equivalence, own Strong/Emph nodes including empty and
nested bodies, and None for Text, Empty, both Space variants, Sequence and
Styled. A new locatable producer requires an explicit coordinated review of
classification, extraction and causal traversal.

The intended classification/extraction alignment is not proven universal:
OutlineEntry is classified true but falls through this dispatcher, while its
queriable entries are synthesized after the walk. This observed discrepancy is
recorded in `lab/results/construction/diagnostics/sanitization-introspection-columns-history.md`.
Do not silently add an arm, fabricate payloads or remove its classification
under this documentation-only contract.


## Realização com avaliação única

Located delega payload ao corpo; Realized e ShowRecipe não emitem payload próprio. Walk trata proveniência e corpo separadamente.


## Exaustividade de origens e Source — 2026-10-16

### Medição antes da decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status, git diff HEAD --stat e fontes/hashes integrais em `lab/results/construction/diagnostics/tekt-v16-introspect-origins-2026-10-16/before.json.gz`. Value tem 39 variantes e Content 120. eval/mod.rs:956 conserva fallback None latente T027; layout/realized.rs:14–45 usa predicado implícito e binding other para T158/T160; :77–130 usa cadeias if-let com false implícito. extract_payload.rs:18–98 despacha sem normalizar Source; locatable.rs:21 classifica Source pelo body. O núcleo content/source-origin.toml exige transparência e ocorrência única. Inferência a refutar por teste RED antes da implementação: Source(Heading) retorna None pela extração atual, e Located(Source(Heading)) perde a prova de preparação. map_content em entities/content/traversal.rs:79 já trata Source transparentemente. Não se infere intenção upstream nem nova paridade vanilla destes fatos; não mudar a pendência OutlineEntry sem investigação própria.

### Decisão exclusiva e aceitação

Normalizar Source com o helper existente content.source_body() antes do match, sem novo payload, Location, callback ou travessia de descendentes. Isso restaura a transparência já exigida pelo núcleo content-source-origin e pelo classifier Source. Preservar Located e todas as delegações to_payload/dyn_to_payload atuais; não descer por Styled nem inventar payload para OutlineEntry. Substituir o fallback aberto por complemento nominal das 120 variantes atuais, mantendo None para as restantes e o grupo existente ShowRecipe/Realized. O caso Source do complemento é inalcançável após normalização e não representa produtor próprio. A antiga ressalva de fallback aberto é supersedida por esta revisão interna. Ajustar os comentários antigos que afirmavam não-locatabilidade universal ou evolução aberta; OutlineEntry permanece discrepância explícita pendente, não uma equivalência universal provada.

O grupo nominal com consequência idêntica pode permanecer como V19 informativo, prescrito por responsabilidade acima, sem duplicar corpos nem supressões globais. Correções estruturais adicionam barreira local contra variantes novas; não afirmam defeito funcional em todos os fallbacks nem cobertura de execução de todas as alternativas. Preservar firmas públicas e comportamento fora da transparência Source já contratada.

Escrever testes co-localizados dos casos Source produtores/não produtores, Located/Source, coleta sem duplicação, fallbacks e predicados; confirmar RED da transparência antes do código. Controles de preservação e single_evaluation_ existentes passam antes/depois. Em cópias do workspace real, variantes sentinelas separadas em Value e Content devem produzir E0004 nos seis pontos selecionados depois da correção; guardar ausência dessas barreiras antes, demais erros e fontes. Resselo somente dos pares editados. Validar build, suíte workspace completa, Clippy all-targets com warnings/expectativas como erro, fmt, Rustdoc estrito, V1/V5/V15/V26 e reversa; SARIF remove somente V16 selecionado e mantém demais diagnósticos, admitindo apenas V19 equivalentes prescritos. Recursos: 12 GiB, swap zero, Cargo um job e comandos pesados sequenciais. Não declarar fechado o objetivo geral ou OutlineEntry por fechar este lote.


## Alinhamento OutlineEntry e walk exaustivo — 2026-10-16

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, lista exata git diff HEAD --stat, fontes/hashes e binários em `lab/results/construction/diagnostics/tekt-v16-introspect-walk-2026-10-16/before.json.gz`. Medições CLI anteriores em measurement-before.json e measurement-fields-before.json: no pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, entry direta aparece uma vez, sintetizada uma vez e mista duas vezes; no Tekt, a direta some da query (0 contra 1), mista perde a direta (1 contra 2), e body() direto falha cannot outline heading. Fonte: entities/elements/outline_entry.rs:61 body só aceita Heading bruto; :99 fornece NativeElement; introspect/extract_payload.rs:148 descarta OutlineEntry; walk.rs:805 trata entry terminal; outline_entries.rs:38–61 sintetiza entries após walk. Generic map_content visita element/fill (traversal.rs:221); realized.rs:139–281 usa essa travessia para certificar origens, motivo para testar um heading referenciado Located sem confundi-lo com descendente renderizado. O upstream pinado declara OutlineEntry Locatable em outline.rs:472–481. Isto mede semântica/morfologia e erros observáveis da linguagem, não impõe igualdade Rust/bytes/PDF ou mecânica vanilla. A hipótese de correção é refutada por query/fields/body errados, headings fantasmas, duplicação ou regressão em síntese/ordem/labels.

### Responsabilidade exclusiva vigente

Adicionar somente a delegação Content::OutlineEntry(e) => e.to_payload(), retirando OutlineEntry do complemento nominal None; a entidade já fornece NativeElement. Preservar Source normalizado, Located e todas as delegações. Isso corrige o alinhamento ordinário previamente pendente; não criar payload nem Location especial e não fabricar descendentes do campo element. O teste anterior que documentava a pendência passa explicitamente a exigir Some(NativeElement) direto e por Source; demais esperados anteriores permanecem. A síntese pós-walk não é substituída; medir os controles de não duplicação e refinar antes de codificar qualquer exceção adicional se forem refutados.

Esta revisão supersede somente as ressalvas anteriores de pendência OutlineEntry neste owner; preserva os demais contratos vigentes.

### Aceitação

L0 primeiro, testes co-localizados antes da implementação e RED funcional real; manter controles existentes e a medição CLI anterior intacta. Repetir fixtures direct/synthesized/mixed com body() e com acesso ao campo, comparar resultados de linguagem, não bytes PDF. Sonda Content sentinela separada em cópia do workspace real: os seis pontos W1–W6 devem acrescentar E0004 após a correção e não antes, sem outros erros; preservar logs/inputs. Resselo só dos cinco pares afetados e grafo V15/V26 válido. Gates: build workspace, fmt, Clippy all-targets -D warnings e unfulfilled_lint_expectations, Rustdoc -D warnings, suíte workspace completa, V1/V5/V15/V26 e reversa, diff-check e SARIF diferencial. Esperado: seis V16 removidos (239→233), seis V19 informativos novos; V19 existente do complemento do extractor perde uma alternativa e os dois grupos de needs_fresh_index migram OutlineEntry de false para true. Nenhum V20 novo ou outro diagnóstico alterado após mapear deslocamentos. V19 é grupo de consequência uniforme prescrito aqui, não cobertura de execução de cada alternativa. Sem supressões ou configurações globais. 12 GiB, swap zero, um job Cargo, comandos pesados sequenciais; estado/UTC/hashes registrados. Correção interna/paridade ADR-0127, sem API/campo/trait/default/fase ou compatibilidade novos. Não publicar, commitar ou fazer push. O objetivo geral permanece aberto com os demais V16.
