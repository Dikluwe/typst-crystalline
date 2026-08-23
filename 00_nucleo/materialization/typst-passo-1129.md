# L0 — Passo 1129: Largura de Glifo Errada em Texto de Legenda em Tamanho Reduzido (`underbrace`)

**Gate**: `ADR-0127` se confirmar necessidade de correcção. Este passo é
investigação.

**Base**: nota externa (secção 40, reteste pós-P1127). Altura da fonte
correcta (`7.70pt` nos dois); largura de cada glifo sistematicamente
maior no cristalino, razão média `1.146` (próxima de `8/7≈1.143`), só no
texto de legenda de `underbrace(..., "cinco estrelas")` — texto normal da
secção (título) não é afectado.

---

## 1. Isolar antes de ler código — a variável causal é "tamanho reduzido" ou "contexto de `underbrace`"?

Testar:
1. `text(size:7.7pt)[cinco estrelas]` **fora** de qualquer `underbrace`/
   contexto matemático — se a largura já divergir na mesma proporção,
   a causa é genérica a texto em tamanho reduzido (qualquer caminho),
   não específica a `underbrace`.
2. `underbrace(x, "cinco estrelas")` isolado, sem mais nada na página —
   confirmar se a razão `1.146` se mantém exactamente ou varia com
   outro conteúdo à volta.
3. Se possível, `overbrace(...)` com o mesmo texto de legenda — confirmar
   se o bug é específico a `underbrace` ou partilhado com `overbrace`
   (mesmo mecanismo de legenda, presumivelmente).

## 2. Ler o código real de largura de glifo em tamanho reduzido

Não presumir a partir da hipótese da nota (upem trocado) — confirmar:

- `03_infra/src/font_metrics.rs` — caminho real usado para medir avanço
  de glifo de texto dentro de legenda de `underbrace`/`overbrace`.
- Confirmar se este caminho é o mesmo usado para texto normal (que já
  bate exacto) ou um caminho separado — se for separado, aí está a
  causa mais provável (dois caminhos, um correcto, um com upem/escala
  errada).

## 3. Testar a hipótese numérica da nota — `8/7` como pista

A nota já identificou que `1.146≈8/7`. Não ignorar esta pista, mas
também não presumir que é a explicação sem confirmar contra valores
reais de fonte — `upem` de fontes comuns são tipicamente `1000`ou
potências de `2` (`2048`, etc.), não valores que dão `8/7` directamente
de forma óbvia. Pode ser:
- Um valor de `upem` genuinamente errado no código (confirmar qual
  fonte/variante está a ser lida para este contexto específico).
- Uma composição de duas escalas (`size_factor` aplicado duas vezes, ou
  aplicado com um factor ligeiramente diferente do esperado) que só
  coincide com `8/7` por aproximação, não por ser literalmente essa
  fracção no código.

## Critérios de verificação

1. §1: variável causal isolada — tamanho reduzido em geral, ou
   `underbrace`/`overbrace` especificamente.
2. §2: caminho de código real identificado, confirmado como distinto
   (ou não) do caminho de texto normal já correcto.
3. §3: causa numérica exacta confirmada com código, não só a
   coincidência de `8/7`.
4. Largura de "cinco estrelas" convergindo de `50.64pt` para `44.58pt`
   (±0.0005pt).
5. Re-rodar P1086-1128 — zero regressão.
6. `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- §1 executado antes de §2 — isolamento antes de leitura de código.
- Causa numérica exacta (não só "próxima de 8/7"), com código citado.
- Confirmar se `overbrace` também é afectado, não presumir.
