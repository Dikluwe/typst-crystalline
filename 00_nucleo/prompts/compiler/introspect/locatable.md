# L0 — introspect/locatable
Hash do Código: 1c8b1a6b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67

```text
```

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/locatable.rs`.
The exhaustive Content match in is_locatable at `:20` classifies the current
node, not its descendants. `01_core/src/compiler/introspect/extract_payload.rs:18`
is a separate delegated extractor with an open None fallback.
Layout and introspection share classification for causal allocation; allocation
and traversal themselves are owned by their respective productive modules.

## Contract

Preserve `pub fn is_locatable(content: &Content) -> bool` as a pure,
deterministic, stateless classification over the closed enum, without a wildcard.
A new enum variant requires an explicit classification decision; adding a
locatable producer also requires extractor/traversal review.

The native true branches are Strong, Emph, Heading, Figure, Cite, Metadata,
State, StateUpdate, StateDisplay, CounterDisplayCallback, ContextBlock,
Outline, OutlineEntry, Bibliography, Equation, CounterUpdate, Table and Footnote.
Dynamic is true only when dyn_element_kind() is present.
The remaining variants are false, including Styled, Label, Sequence, Columns,
text/spacing/flow sentinels, mathematical carriers, FigureCaption and
FootnoteEntry. A non-locatable wrapper can still have traversed locatable children;
false is not a command to discard its descendants.

Own Strong/Emph occurrences remain true even for empty or repeated bodies.
Styled surrounding them stays false. Footnote itself is true independently
of column layout. SpaceFromNewline remains false without origin normalization.
No boolean classifies an arbitrary wrapper as Heading or invents identity from
old show flags.

## Alignment and acceptance

The intended ordinary-producer invariant is
is_locatable(c) == extract_payload(c).is_some(), supporting synchronized
Location allocation rather than a second independent occurrence count.
Do not claim universal verified coverage: OutlineEntry is currently true here
but returns None from the extractor and has a post-walk synthesized query path.
Its direct-input/traversal alignment is unresolved; see
`00_nucleo/diagnosticos/sanitization-introspection-columns-history.md`.
Dynamic alignment also depends on its existing kind/payload boundary.

Cover native true branches and representative false wrappers/leaves, own
Strong/Emph, both Space variants, and delegation alignment for the ordinary
producer domain. Preserve the explicit OutlineEntry discrepancy as an audited
pending issue; fixing either consumer is outside documentation sanitization.


## Realização com avaliação única

Located é locatable sse body é; Realized e ShowRecipe são carriers sem payload próprio, preservando is_locatable ↔ extract_payload.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Source não é nova ocorrência locatable; classificar somente seu body, em paridade com extract_payload.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.


## Exaustividade de origens e Source — 2026-10-16

### Medição antes da decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status, git diff HEAD --stat e fontes/hashes integrais em `00_nucleo/diagnosticos/tekt-v16-introspect-origins-2026-10-16/before.json.gz`. Value tem 39 variantes e Content 120. eval/mod.rs:956 conserva fallback None latente T027; layout/realized.rs:14–45 usa predicado implícito e binding other para T158/T160; :77–130 usa cadeias if-let com false implícito. extract_payload.rs:18–98 despacha sem normalizar Source; locatable.rs:21 classifica Source pelo body. O núcleo content/source-origin.toml exige transparência e ocorrência única. Inferência a refutar por teste RED antes da implementação: Source(Heading) retorna None pela extração atual, e Located(Source(Heading)) perde a prova de preparação. map_content em entities/content/traversal.rs:79 já trata Source transparentemente. Não se infere intenção upstream nem nova paridade vanilla destes fatos; não mudar a pendência OutlineEntry sem investigação própria.

### Decisão exclusiva e aceitação

Preservar integralmente o match exaustivo e as decisões atuais. Atualizar somente a descrição do extractor (agora nominal, Source normalizado) e qualificar a equivalência de payload ao domínio de produtores ordinários. A discrepância OutlineEntry true/None e seu caminho sintetizado continuam auditados e pendentes; não alegar nem impor equivalência universal. Código executável deste consumer não muda.

O grupo nominal com consequência idêntica pode permanecer como V19 informativo, prescrito por responsabilidade acima, sem duplicar corpos nem supressões globais. Correções estruturais adicionam barreira local contra variantes novas; não afirmam defeito funcional em todos os fallbacks nem cobertura de execução de todas as alternativas. Preservar firmas públicas e comportamento fora da transparência Source já contratada.

Escrever testes co-localizados dos casos Source produtores/não produtores, Located/Source, coleta sem duplicação, fallbacks e predicados; confirmar RED da transparência antes do código. Controles de preservação e single_evaluation_ existentes passam antes/depois. Em cópias do workspace real, variantes sentinelas separadas em Value e Content devem produzir E0004 nos seis pontos selecionados depois da correção; guardar ausência dessas barreiras antes, demais erros e fontes. Resselo somente dos pares editados. Validar build, suíte workspace completa, Clippy all-targets com warnings/expectativas como erro, fmt, Rustdoc estrito, V1/V5/V15/V26 e reversa; SARIF remove somente V16 selecionado e mantém demais diagnósticos, admitindo apenas V19 equivalentes prescritos. Recursos: 12 GiB, swap zero, Cargo um job e comandos pesados sequenciais. Não declarar fechado o objetivo geral ou OutlineEntry por fechar este lote.


## Alinhamento OutlineEntry e walk exaustivo — 2026-10-16

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, lista exata git diff HEAD --stat, fontes/hashes e binários em `00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16/before.json.gz`. Medições CLI anteriores em measurement-before.json e measurement-fields-before.json: no pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, entry direta aparece uma vez, sintetizada uma vez e mista duas vezes; no Tekt, a direta some da query (0 contra 1), mista perde a direta (1 contra 2), e body() direto falha cannot outline heading. Fonte: entities/elements/outline_entry.rs:61 body só aceita Heading bruto; :99 fornece NativeElement; introspect/extract_payload.rs:148 descarta OutlineEntry; walk.rs:805 trata entry terminal; outline_entries.rs:38–61 sintetiza entries após walk. Generic map_content visita element/fill (traversal.rs:221); realized.rs:139–281 usa essa travessia para certificar origens, motivo para testar um heading referenciado Located sem confundi-lo com descendente renderizado. O upstream pinado declara OutlineEntry Locatable em outline.rs:472–481. Isto mede semântica/morfologia e erros observáveis da linguagem, não impõe igualdade Rust/bytes/PDF ou mecânica vanilla. A hipótese de correção é refutada por query/fields/body errados, headings fantasmas, duplicação ou regressão em síntese/ordem/labels.

### Responsabilidade exclusiva vigente

Código executável inalterado. Atualizar a documentação do alinhamento: OutlineEntry é produtor nativo delegado pelo extractor; remover a ressalva de pendência no texto vigente, preservando histórico e a dependência de Dynamic em seu contrato. Não alegar prova universal de todas as variantes; Source segue transparente. Assinatura e classificação completas intactas.

Esta revisão supersede somente as ressalvas anteriores de pendência OutlineEntry neste owner; preserva os demais contratos vigentes.

### Aceitação

L0 primeiro, testes co-localizados antes da implementação e RED funcional real; manter controles existentes e a medição CLI anterior intacta. Repetir fixtures direct/synthesized/mixed com body() e com acesso ao campo, comparar resultados de linguagem, não bytes PDF. Sonda Content sentinela separada em cópia do workspace real: os seis pontos W1–W6 devem acrescentar E0004 após a correção e não antes, sem outros erros; preservar logs/inputs. Resselo só dos cinco pares afetados e grafo V15/V26 válido. Gates: build workspace, fmt, Clippy all-targets -D warnings e unfulfilled_lint_expectations, Rustdoc -D warnings, suíte workspace completa, V1/V5/V15/V26 e reversa, diff-check e SARIF diferencial. Esperado: seis V16 removidos (239→233), seis V19 informativos novos; V19 existente do complemento do extractor perde uma alternativa e os dois grupos de needs_fresh_index migram OutlineEntry de false para true. Nenhum V20 novo ou outro diagnóstico alterado após mapear deslocamentos. V19 é grupo de consequência uniforme prescrito aqui, não cobertura de execução de cada alternativa. Sem supressões ou configurações globais. 12 GiB, swap zero, um job Cargo, comandos pesados sequenciais; estado/UTC/hashes registrados. Correção interna/paridade ADR-0127, sem API/campo/trait/default/fase ou compatibilidade novos. Não publicar, commitar ou fazer push. O objetivo geral permanece aberto com os demais V16.
