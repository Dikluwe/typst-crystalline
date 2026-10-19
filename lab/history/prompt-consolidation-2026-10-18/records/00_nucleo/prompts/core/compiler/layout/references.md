# L0 — references
Hash do Código: f1150194

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:4e487a99ea1f1e82a67dfaced0958f98392fb26b1a7d292355230d6ffa7f2d0b
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:ff71dfc6648d6deb4b2b8054e3d58a93777d17837129f0f1b1e7e138ffb6943c
- 00_nucleo/prompts/_nuclei/references/unreferencable-label.toml sha256:5dde6e7f84150a121e671dab0d174798e8b56f24355147c1bcc471d6babd69ed

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/references.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/references.rs:77`, `01_core/src/compiler/layout/references.rs:283`, `01_core/src/compiler/layout/references.rs:30`, `01_core/src/compiler/layout/references.rs:54`, `01_core/src/compiler/layout/references.rs:249`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own Label and Ref layout, not introspection stores or language tables. Label renders its body transparently, then records the current physical page and its captured point in runtime maps. Ref to an existing bibliography key delegates to `00_nucleo/prompts/core/compiler/layout/cite.md` with the supplement.

For ordinary references, preserve current validation precedence: unreferencable kind diagnostics before generic existence checks, then heading/equation numbering checks and numeric/legacy resolution. Emit the exact current errors and hints, with detached spans because the element lacks a span. Numeric counter-key/location formatting takes precedence; legacy figure/resolved-label data remains fallback. Missing labels are diagnosed, not silently changed back to a question mark. Preserve internal destinations and observable fragment placement; collection is specified by the frontier correction below.

An explicit supplement, including empty Content, overrides the implicit one. Equation defaults consume its causal realized supplement; figure defaults and legacy figure fallback delegate to the language owner. Heading defaults first extract the causal prefix from resolved label text only when it ends in ASCII space plus the same formatted number; otherwise use the language owner's deterministic current-style fallback. Do not add bilingual tables, kind-label catalogs or infer a target language that was not carried. A non-empty prefix joins number with U+00A0 once.

For Page form, preserve existing known-label/page lookup, missing-numbering diagnostic/hint, explicit-versus-PageStore supplement precedence and internal link. Function numbering consumes PageStore.reference_numbering without Func execution. The current Pattern branch still formats the physical page index locally; its conflict with the older shared unary logical-store promise is unresolved. A not-yet-known page retains the existing linked question-mark path rather than fabricated numbering. This cleanup does not authorize changes to numbering views or labels.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Posições no referencial composto — 2026-10-14

### Medição anterior à decisão

box-contexto-2026-10-14/probe.json.gz, fontes/binários/HEAD/working tree/UTC
preservados: Box, inset2pt, prefixo e aninhamento publicam (0,0), mesmo
quando a tinta se desloca. A diferença da baseline do context direto é
uma questão separada: não copiar o vanilla apenas por diferir. mod.rs:
1131-1148/1180-1200 escreve pontos locais em runtime.positions; Box agrupa
o corpo em outro referencial; finish.rs:295-302 sela o mapa sem recompor
esses pontos. Realized escreve com or_insert; labels têm mapa paralelo.
HTML já transporta markers privados pelo frame em html_frame.rs:30-199.
O registro single-pass e sealing por sub-store são preservados, ADR-0073/
0074 §§posição/modelo híbrido; não se adota introspecção global vanilla.
A inspeção do núcleo coordinates e seus owners mostrou usos absolutos
em grid e explicitamente locais em Place/Box. A qualificação do núcleo
conserva ambos e exige transformação até a página, sem dupla origem.
Snapshot anterior: posicoes-compostas-2026-10-14/before.json.gz.
Geometria/identidade são observáveis; o carrier e seu índice são mecânica.

### Contrato deste owner

Label conserva a captura de ponto anterior ao corpo e a escrita de página após ele; delegar escrita/captura desses valores ao helper de posição. O carrier pertence ao mesmo resultado geométrico do corpo envolvente, e a projeção aplica seu referencial antes da publicação. Não alterar seleção, contadores, ordem de efeitos ou formatação de Ref.

Correção interna ADR-0127: sem API/campo público, flag/default ou mudança
de fase. L0 primeiro; RED→GREEN de coordenadas, ausência de tinta/altura
fictícia, identidade, ordem, clones visuais e rollback; revalidar os
controles HTML, geometria, contexto/queries e suíte completa.

## Captura de links através de fronteiras — 2026-10-14

### Medição anterior à decisão

lab/results/construction/diagnostics/links-paginacao-2026-10-14.md e os recibos
posicoes-compostas-2026-10-14/automatic-boundary-probes registram fonte,
HEAD/árvore suja/UTC, binários e pin vanilla. Página 160×40pt/margem 10pt:
texto comum atravessa a quebra; Link URL, Ref e Ref Page abortam com índice
antigo após esvaziamento dos buffers. references.rs:270 aparece na pilha;
link.rs:22-32 usa o mesmo mecanismo. outline.rs:28-50 também guarda índices
antes do corpo. Limitar o índice não prova preservação do prefixo, ordem,
fragmentos, destino ou geometria. Snapshot anterior deste reparo:
lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz.
Os observáveis são conteúdo, destino, regiões e bounds; limites privados
são mecânica. Correção interna ADR-0127, sem API, default ou fase nova.

### Contrato deste owner

As duas emissões de texto de Ref (normal e Page/fallback) usam o mesmo escopo privado de destino interno. Preservar resolução, precedência de erros, supplement, numeração e labels; não executar Func aqui. Remover os drains por comprimentos obsoletos, sem substituir por clamp que perca conteúdo.

Aceitação: RED→GREEN de quebras antes/dentro do corpo, preservação de
prefixo/sufixo, regiões, destinos/bounds, corpo rico/aninhamento/colunas,
conteúdo diferido, contexto/rollback e ausência de limites privados na saída.
Revalidar os controles sem quebra e a suíte, build e linhagem.
