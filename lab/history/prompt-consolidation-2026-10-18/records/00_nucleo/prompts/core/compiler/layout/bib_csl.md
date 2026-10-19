# L0 — bib csl
Hash do Código: 91e99dd9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/bibliography-plan.toml sha256:cec5b93a8888658ac90461deea96799dfd56393be3d63c874e6cf93f4d57032c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/bib_csl.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/bib_csl.rs:39`, `01_core/src/compiler/layout/bib_csl.rs:47`, `01_core/src/compiler/layout/bib_csl.rs:58`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Construct pure citation/bibliography caches from supplied BibEntry values, built-in styles or a supplied parsed IndependentStyle, locale and optional citation order. Preserve the Normal/Prose/Author/Year mappings and the existing unsupported-form behavior. Built-in style lookup uses the archive and the cache builder's `ieee` default; the hub decides whether a cache is requested at all. Parse supplied custom CSL XML without filesystem or URL access. Convert rendered CSL nodes into Content while preserving supported links, strong/emphasis, small caps, underline and block breaks. Serialize textual YAML scalars through total JSON-string escaping so controls, Unicode, empty and YAML-looking strings remain text. Normalize a maximal run of at least two ASCII hyphens in the pages field to one hyphen; single hyphens, Unicode dashes, and other fields are unchanged. Preserve diagnostics for malformed styles/entry conversion and the existing locale fallback.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## A2-Structure — entradas CSL e colunas (2026-10-18)

### Medição anterior à decisão

lab/results/construction/diagnostics/bibliography-structure-2026-10-18/before.json.gz registra
12 pares, fontes/argv/UTC/HEAD/status/diff HEAD --stat, textos/caixas/binários;
before-sources.json.gz conserva as fontes e L0s anteriores. HEAD
b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, árvore não commitada, Tekt SHA-256
c2b1b2e93cf34f3c597a0a0fbbd51c8f8ac96a3c6bc0a4d9bd72373599acf899;
oracle bd5ebec285505889e8ff1d4f68804ab147a699f3, árvore
cac836c07e4dec2a89219370428292393a3216b5, binário SHA-256
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9.
IEEE12 põe A. em x25.124 contra29.924 e a segunda entrada em y89.936
contra112.232. IEEE18 usa x33.686 contra40.886. No CSL sem prefixo, a segunda
entrada começa em y184.424 contra191.024. As sondas com par.spacing0/2em
separam margens de espaçamento dentro das linhas. Há também diferença de
agrupamento de citações adjacentes (MLA Silva/Doe), fora da composição da lista;
manter o RED explícito, não chamar igualdade só da lista de paridade geral.

bib_csl.rs:342–355 concatena first_field, espaço literal, content e Linebreak,
perdendo a fronteira de cada entrada e o hanging_indent. convergence.rs:58–82
transporta só BibRenderCache público; bibliography.rs:107–151 emite um Block
único. O pin typst-layout/src/rules.rs:463–531 compõe duas colunas Auto com
gutter0.65em e row-gutter par.spacing, ou um Block por entrada sem prefixo;
o formato autoral do código Rust não é requisito. Observáveis são posição,
ordem, conteúdo/estilo e escopo. Hipótese: transportar fronteiras reais e
prefixos corrige a coluna e a separação sem reformatar; controles de fontes,
ordem, campos vazios, sem prefixo, zero/2em, show-grid e estilo podem refutá-la.

### Classe e escopo explícito

Correção interna/de paridade ADR-0127 em fluxo contínuo, dentro do layout,
sem API pública, tipo público novo, flag/default, compatibilidade ou fase nova.
L0 antes de testes/código. A2-Structure preserva hanging no discriminante do
plano, mas sua aplicação física, overrides por ocorrência e composição
simple/optimized/RTL continuam em A2-HangingTransport/A2-ParIndent. Não forçar
fallback greedy para contornar o suporte de avanço inicial ainda ausente da
preparação. Margens de título, CSL completos, atribuição A2-Multi e agrupamento
de citações adjacentes não ficam certificados por esta etapa.

### Responsabilidade exclusiva

Declarar submódulo privado plan. Um builder privado prepara simultaneamente BibRenderCache e Option<Arc<plan::BibliographyPlan>>, usando a mesma execução CSL já existente. As duas fachadas públicas mantêm assinatura, diagnósticos, citações e Content legado, extraindo somente o cache do resultado privado. A conversão do RenderedBibliography para plano tipado pertence exclusivamente a 00_nucleo/prompts/core/compiler/layout/bib_csl/plan.md; delegar conversores existentes de ElemChild/ElemChildren por descendência. Não renderizar CSL novamente nem converter strings para deduzir fronteiras.

### Aceitação

RED funcional de coluna com gutter relativo e margem entre entradas antes de
implementar. Controles de cache público (Content anterior e assinaturas), CSL
sem prefixo, preservação de modo hanging/plain, vazio, ordenação/citações e
rollback. PDF bilateral com caixas e texto da lista, conservando divergências
de recuo e agrupamento de citações no relatório. Build/workspace/doctests,
Clippy e Rustdoc estritos, fmt, V1/V5/V15/V26 e reversa; SARIF sem erro/warning
novo. Resselo só após ownership/grafo válido. MemoryMax12GiB/High8GiB/swap0,
jobs1/debug0/incremental0, comandos pesados sequenciais; sem commit/push/release.
