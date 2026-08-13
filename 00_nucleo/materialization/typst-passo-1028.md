# Passo 1028 — `BoxedElem.width` não chega à largura desenhada

**Tipo**: Investigar → classificar gate → corrigir se aprovado. Independente do Passo
1027 (fora de math, sem relação de código).
**Achado (Passo 1026)**: `#box(width: 40pt, ...)` não aplica a largura pedida — a caixa
sai com a largura do conteúdo mais inset, ignorando `width`. `height` funciona
correctamente (confirmado pelo caso de controlo). Reproduz-se em texto corrido, não é
específico de math nem de `layout_external`.

| documento (11pt) | vanilla | cristalino |
|---|---:|---:|
| `#box(height: 40pt, width: 40pt, stroke: 1pt)` (vazio) | 40,80 × 40,80pt | 1,92 × 38,40pt |
| `#box(height: 40pt, width: 40pt, stroke: 1pt)[a]` | 40,80 × 40,80pt | 6,96 × 42,00pt |
| `#box(width: 40pt, stroke: 1pt)[a]` | 40,80 × 8,16pt | 6,96 × 14,64pt |
| `#box(height: 40pt, stroke: 1pt)[a]` (controlo, sem `width`) | 6,00 × 40,80pt | 6,96 × 42,00pt |

**Pré-condição**: `git status` limpo. Ponto de partida indicado: `boxed.rs:133` e `:225`.

---

## Fase A — Localizar a causa

1. Ler `boxed.rs:133` e `:225` — confirmar por que `width` não participa no cálculo de
   largura final, enquanto `height` participa correctamente no caminho paralelo.
2. Confirmar se é omissão simples (campo lido mas nunca aplicado) ou se há lógica
   condicional que devia aplicar `width` nalgum caso e falha silenciosamente.
3. Verificar o vanilla (`typst_library::layout::box` ou equivalente) — confirmar o
   mecanismo esperado antes de escrever o fix, não presumir "aplicar width directamente"
   sem confirmar se há regras de precedência (ex.: `width` vs conteúdo maior que `width`).

## Fase B — Alcance: só `box`, ou também `block`/outros com `width`?

Verificar rapidamente se `block(width:)` ou outros elementos com campo `width` explícito
têm o mesmo problema, ou se é isolado a `BoxedElem`. Não presumir isolado sem checar.

## Fase C — Classificar o gate

Mudança de output visual em qualquer documento com `box(width:)` → gate ADR-0127,
categoria 2/3, mesmo tratamento do Achado A. L0 antes de código.

```
Dado #box(width: 40pt, stroke: 1pt)[a]
Quando renderizado
Então largura final é 40pt + o que o vanilla adicionar (stroke/inset, confirmar
  exactamente per medição vanilla), não 6,96pt

Dado #box(height: 40pt, stroke: 1pt)[a] (sem width — controlo)
Quando renderizado
Então comportamento inalterado (guarda de não-regressão directa dos números já medidos)

Dado #box(width: 40pt)[conteúdo mais largo que 40pt]
Quando renderizado
Então [comportamento a confirmar na Fase A — vanilla estica, corta, ou ignora? decidir
  com medição, não suposição]
```

## Fase D — Implementar (só após gate) e validar

```
crystalline-lint .
cargo test --workspace
```
Decalque visual — mudança de aparência em qualquer documento com `box(width:)`.

---

## Resultado esperado

`BoxedElem.width` aplicado correctamente, com o caso de conteúdo maior que `width`
resolvido conforme a regra vanilla confirmada, não presumida.
