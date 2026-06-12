# DEBT — layout no-op do `Content::Dynamic` (fronteira E1)

**Estado**: aberto
**Data de abertura**: 2026-06-12 (carona C2, P335)
**Origem**: P334 (Lote F-1, ambiguidade de Fase A **A2**).
**Magnitude**: S (um arm; buraco declarado e limitado).
**Pré-condição de fecho**: o **lote de realização (F-3)** — quando a
realização/`#show` (S2–S6) der layout real ao nó dinâmico.

---

## O buraco

`rules/layout/mod.rs` arm `Content::Dynamic(_) => {}` (no-op). Um elemento de
utilizador (`Content::Dynamic`) **não renderiza** — emite zero `FrameItem`.

## Por que é limitado (não é bug solto)

O `layout_content` é **exaustivo** (sem `_ =>`); adicionar a variante `Dynamic`
(F-1, aditivo) obrigou a um arm. Em F-1 **nenhum documento real produz
`Content::Dynamic`** — só as fixtures de teste (`test_callout`). O caminho de
render do elemento dinâmico é a **realização** (a passagem nova em `rules/` que
F-3 traz, L0 `f_fronteira_e1.md` §3a.7): é lá que o nó dinâmico recebe a chain,
os guards de `#show` e o layout. Até F-3, o no-op é correto-por-construção
(content-preserving: a suíte 2720 não exercita o caminho).

## Critério de fecho

DEBT fecha quando, em F-3:
1. A realização materializar o layout do nó dinâmico (não no-op).
2. Um teste E2E render de um elemento de utilizador produzir `FrameItem`s
   observáveis (texto/forma), provando que o no-op deixou de existir.
3. O teste da **condição Trava-Q1** (transparência do invólucro de guard) passar.

## Referências

- ADR-0106 §Aprovação da Trava (Trava-Q1: guard na camada de realização).
- L0 `f_fronteira_e1.md` §3a.7 (realização + guards).
- `f-plano-lotes-passo-333.md` (F-3 = realização/`#show`, sequência emendada na
  carona C3 do P335).
- O arm: `01_core/src/rules/layout/mod.rs` (`Content::Dynamic(_) => {}`).
