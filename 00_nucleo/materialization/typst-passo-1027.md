# Passo 1027 — `#text(size:)` ignorado em math bare: regressão de âmbito do P994

**Tipo**: Investigar → classificar gate → corrigir se aprovado.
**Achado (Passo 1026, ao construir régua contínua)**: `needs_external_layout`
(`compiler/math/layout/mod.rs:249-262`) só sobe para `layout_external` conteúdo que
contém `Equation`/`Boxed`/`Align`/`Pad`/`Block`. `Styled(Text, …)` não está na lista —
`$ #text(size: 40pt)[x] $` (markup simples, sem `$...$` aninhado) fica no caminho de texto
de math antigo, que descarta o override de tamanho. Medido: `5,52 × 4,80pt` produzido
contra `20,16 × 17,28pt` esperado (vanilla).
**Contexto histórico**: o critério original do P994 era "tamanhos preservados em
`text()`". Uma "adenda 2" (não identificada por número neste passo — localizar na Fase A)
estreitou a deny-list para a lista actual, sem registar que isso partia o critério
original para o caso de markup simples sem equação aninhada. Os testes-guarda de P994
não apanharam isto porque testam `#text(size: …)[$…$]` (com equação aninhada, que entra
na allow-list), não o caso bare.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1026.

---

## Fase A — Reconstruir o porquê da adenda 2, antes de mexer

1. Localizar o passo/adenda que estreitou `needs_external_layout` de "qualquer `Styled`"
   (ou o que fosse antes) para a lista actual de 4 variantes. Ler a razão registada — não
   presumir que foi arbitrário.
2. **Hipótese a confirmar**: a lista foi estreitada porque incluir `Styled(Text, …)` sem
   condição causava outro problema (ex.: markup de texto puro dentro de math, tipo
   `#text(fill: red)[nota]`, a entrar por engano no caminho external e perder algo que o
   caminho de texto de math tinha). Se esta hipótese (ou outra) se confirmar, a correcção
   não pode ser simplesmente "adicionar `Styled` de volta à lista" — precisa de
   distinguir os dois casos.
3. Verificar se `Content::Styled(Text, [Size(_)])` (só tamanho, sem outros estilos) é
   distinguível de `Styled(Text, [outros estilos])` de forma barata, e se essa distinção é
   o que falta.

## Fase B — Medir o alcance real do problema

Repetir a tabela do P1026 (Achado A) para outras propriedades de `text()` além de `size`
— `fill`, `weight`, `style` (itálico) — em markup bare dentro de `$...$`, para confirmar
se é só `size` que se perde ou se é qualquer propriedade de `text()` aplicada sem equação
aninhada.

## Fase C — Classificar o gate

Mudança de output visual em qualquer documento com `text()` bare dentro de math → gate
ADR-0127, categoria 2/3. L0 antes de código.

```
Dado $ #text(size: 40pt)[x] $ (markup bare, sem $...$ aninhado)
Quando renderizado
Então o tamanho é aplicado, batendo com vanilla (per medição da Fase B, todas as
  propriedades relevantes de text(), não só size)

Dado $ #text(fill: red)[nota] $ (só estilo, não é o caso do P994 original)
Quando renderizado
Então [comportamento a confirmar na Fase A — pode ser correcto manter caminho antigo se
  a hipótese da adenda 2 se confirmar para este caso]
```

Não-regressão: todos os testes de P994 (`#text(size: …)[$…$]` com equação aninhada) e os
5 casos do diagnóstico original de P993 continuam a passar.

## Fase D — Implementar (só após gate) e validar

```
crystalline-lint .
cargo test --workspace
```
Decalque visual contra o corpus canónico e o corpus `00_nucleo/corpus-docs/math/`
(P998) — é mudança de aparência, confirmar visualmente, não só por `pdftotext`.

---

## Resultado esperado

`text()` bare dentro de math aplica as propriedades relevantes (tamanho pelo menos,
outras conforme a Fase B revelar), sem reabrir o problema que a adenda 2 tinha resolvido
(se a Fase A confirmar que havia um problema real por trás do estreitamento).
