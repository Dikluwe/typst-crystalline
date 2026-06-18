# Relatório P363 — F-5a fatiado: o introspect-chain (infra aditiva)

> **Desfecho.** A Fase A mediu que **introspect-chain + de-bake não cabem num lote** → **válvula
> acionada**: **P363 = só o introspect-chain** (a infra; o de-bake dos 3 numbering é o **P364**). A
> infra está **feita**: o `walk` do introspect threada uma `StyleChain` (empurrada ao descer num
> `Content::Styled`, espelho de `layout/mod.rs:1248`), de modo que um heading/equation/figure dentro
> do `Content::Styled` que o `#set …(numbering:)` embrulha tem o gate `X.numbering` **disponível na
> chain** — a fonte única que o de-bake (P364) vai ler. **Aditivo / content-preserving**: nenhum read
> de gate mudou (continuam no campo assado até o P364); suíte **2737 → 2738** (+1 teste de prova),
> **perf idêntica** (0.6809s antes/depois), lint **0/0**, lente **66/0**.

**HEAD**: pós-P359 (8916829c2). **Branch**: Tekt. Lente `98d8f9e`. Justificativa = **princípio
(fonte única / atomização)**, decidido pelo dono (P362), não demanda da lente.

---

## Fase A (medida; `file:line`)

Os 3 gates de numbering vivem **em dobro** (campo assado **e** chain). Consumidores:
- **heading**: layout `mod.rs:714` (`h.numbering_active`); introspect walk `:823` passa
  `h.numbering_active` a `compute_heading_auto_toc`.
- **equation**: layout `mod.rs:812`→`equation.rs:28`; introspect `:663` gateia o counter da equation
  por `numbering_active` (do payload).
- **figure**: layout `mod.rs:860` (`e.numbering`); payload da figura.
- **Layout já lê a chain** (`self.chain`); **introspect NÃO tinha chain no walk** (P353) → daí a
  infra deste lote.
- **Separação gate-vs-número (lição DEBT-60/P359):** o número (`formatted_counter_at`) e o avanço do
  **contador de heading** (`introspect.rs` `apply_hierarchical_at`, **incondicional** P335) **não são
  tocados**. O de-bake (P364) lê o **gate**, não o número.

**Válvula (medida):** o introspect-chain (threading por ~40 sítios de `walk` + push em `Styled`) e o
de-bake (remover 3 campos assados → cascata em construtores/`to_payload`/fixtures) são **duas mudanças
largas** → **fatiadas** (P363 infra / P364 de-bake). O ponto 4 (`Text` `TextStyle`) já é o F-5b.

## Estágio L0 (Trava aprovada pelo dono)

`introspect.md`: secção "introspect-chain (P363, F-5a)" — o walk ganha a `StyleChain`, empurrada em
`Content::Styled`; os 3 gates lerão `chain.custom("X.numbering")` no de-bake; separação gate-vs-número
explícita; aditivo e verificável sem o de-bake. Hash sincronizado (`introspect.rs`).

## Estágio 1 — o introspect-chain (`file:line`)

- **`walk` ganha `chain: &StyleChain`** (`introspect.rs`, assinatura) — threadada por **todos** os
  ~40 sítios recursivos + as entradas (`introspect_with_introspector`, `fixpoint.rs:95`, helpers de
  teste), raiz = `StyleChain::default_chain()` (espelha o root do layout).
- **Arm `Content::Styled`**: `let pushed = chain.push_styles(styles); walk(body, …, &pushed, …)` —
  empurra os styles ao descer (escopado à subárvore), espelho de `layout/mod.rs:1248`.
- **Aditivo**: nenhum read de gate mudou — `compute_heading_auto_toc` ainda recebe
  `h.numbering_active`; `introspect.rs:663` ainda lê o payload. A chain está **disponível**, ainda
  não **consumida** (isso é o de-bake P364).
- **Probe de verificação** (`#[cfg(test)] mod introspect_chain_probe`): o arm Heading captura, sob
  `cfg(test)`, o gate **lido da chain** (`chain.custom("heading.numbering")`). Não muda produção.

## Estágio Teste

- **Novo**: `introspect_chain_threada_gate_de_numbering_ate_o_heading` — um heading sob
  `Content::Styled{heading.numbering=true}` vê o gate na chain (**true**); um heading fora vê
  **false**. Captura `[true, false]` → prova que a chain foi threadada até o heading. **Única adição.**
- **Content-preserving**: a rede de caracterização (+11) e toda a suíte passam **sem alteração** —
  nenhum read de gate mudou. **Atomização ainda incompleta** (o campo assado coexiste) — isso fecha
  no de-bake (P364); aqui só se ergue a infra.

## Gates (todos verdes)

```
build: workspace limpo. suíte (RUST_MIN_STACK=33554432): 2737 → 2738 (+1 teste de prova; ZERO
  asserção existente alterada — aditivo). Demais crates: 472/24/2/21, 0 falhas.
lint: crystalline-lint . = 0/0 (sem drift).
ACEITAÇÃO: a chain no walk tem o custom certo num heading sob #set numbering (teste); nenhum
  comportamento mudou (infra aditiva).
INTACTOS: o NÚMERO (formatted_counter_at) e o contador P335 (incondicional) — não tocados; α/caso 2,
  caso 4, morph ==/morph_canon, flag P350c, Marco G — não tocados.
lente (instrumento): content→elements = 66 INALTERADO; elemento→elemento = 0 (a chain adiciona
  introspect→style_chain, não content→elements).
perf (mesma sessão): antes 0.6809 s ± 0.0051; depois 0.6809 s ± 0.0079 → IDÊNTICA (a infra é
  behavior-neutral; push_styles por `Styled` é negligível; a probe é cfg(test)).
L0 (critério 5): introspect.md (introspect-chain) + hash sincronizado ANTES do código; Trava aprovada.
```

## Estado / próximo

Tocados: `introspect.md` (L0), `introspect.rs` (chain threadada + probe + teste), `fixpoint.rs` (1
call threadada). Nenhum outro. Árvore limpa fora de docs.

**Próximo (P364, o de-bake — decisão do dono):** sobre esta infra, os 3 gates passam a ler
`chain.custom("X.numbering")` e os campos assados (`HeadingElem.numbering_active`,
`EquationElem.numbering_active`, `FigureElem.numbering`) são removidos → **fonte única**. *Risco
medido a confirmar no P364:* fixtures que constroem elementos numerados **sem** o transporte
`Content::Styled` (ex. `Content::heading_numbered` direto) não têm o custom na chain → o de-bake
precisa que passem pelo transporte, ou as asserções viram (declaradas, S5b). O ponto 4 (`TextStyle`)
é o F-5b.

## Fora de escopo (confirmado)

O **de-bake** (P364); o ponto 4 / `TextStyle` (F-5b); o contador/P335; F-6; o mapa aberto p/ `#set`
de user-props; Marco G; qualquer toque no α / `morph_canon` / `==` ou na flag.
