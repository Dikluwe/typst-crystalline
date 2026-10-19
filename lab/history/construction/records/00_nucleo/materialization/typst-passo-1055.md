# Passo 1055 — Corrigir os 4 achados de layout do P1053: `raw`, `divider`, `quote`, `term_item`

**Tipo**: Gate (`ADR-0127`, categoria 2/3 — muda output visual por defeito) → corrigir.
Investigação e medição **já feitas no P1053** — este passo materializa a correcção, não
reabre a investigação, salvo confirmação pontual de fase A abaixo.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1054.

---

## Fase 0 — Confirmar o estado de `divider.rs` antes de mais nada

`divider.rs:44` (`cursor_y += style.size * 0.6`) não apareceu nomeado na Categoria 3 do
P1054, apesar de catalogado no P1053. Confirmar:
```
crystalline-lint --checks v21 . | grep -A2 divider.rs
```
Se aparecer sob outra categoria (ex.: "Outros Escalares de Domínio"): prosseguir
normalmente, é só classificação da lista, não afecta este passo. Se **não** aparecer em
lado nenhum: confirmar se o predicado de V21 falha em reconhecer `cursor_y +=` como
sumidouro geométrico (candidato a extensão do predicado, per Passo 0066 Parte 3 —
registar como classe nova a adicionar ao `tekt-linter` se for o caso, não deixar
silenciosamente por fora).

---

## Achado 1 — `raw.rs:111` (`prev.size.0 * 0.9`)

**Medição do P1053**:
- Inline: `Δw = -8.05pt` (vanilla preserva 100% do `size`, só troca família para
  monospace; cristalino contrai para 0.9×).
- Bloco: `Δx = +11.00pt`, `Δw = -22.61pt` (vanilla não aplica indent de `1em` sem
  `inset`; cristalino injecta `+1em` e reduz tamanho) — "Gravidade Alta".

**Proveniência vanilla**: `typst-library/src/text/raw.rs:360-390` (tamanho) e `:400-430`
(indentação de bloco).

**Correcção**:
1. Remover a contracção `× 0.9` — `raw` preserva 100% do `size`, só muda família de
   fonte para monospace.
2. Remover a indentação de `1em` aplicada por defeito em bloco sem `inset` explícito.

## Achado 2 — `divider.rs:44` (`style.size * 0.6`)

**Medição do P1053**: `Δy = -6.05pt` no traço, `-12.10pt` cumulativo na linha seguinte —
vanilla usa `BlockElem::spacing = 1.2em` acima/abaixo (13.2pt), cristalino usa `size ×
0.6` (6.6pt).

**Proveniência vanilla**: `typst-library/src/layout/container.rs:342`.

**Correcção**: substituir `style.size * 0.6` por `block.spacing` (`1.2em`, mesma
constante já materializada em `equation.rs` no P1054 — reaproveitar a citação, não
reescrever).

## Achado 3 — `quote.rs:37/47` (`style.size * 1.5`)

**Medição do P1053**: `Δx = +5.50pt` (vanilla usa `indent = 1.0em` e suprime aspas
inteligentes em modo bloco; cristalino usa `1.5em` e mantém aspas).

**Proveniência vanilla**: `typst-library/src/model/quote.rs:75-95`.

**Correcção**: `indent` de `1.0em` (não `1.5em`); suprimir aspas inteligentes quando
`block: true`.

## Achado 4 — `term_item.rs:27` (`style.size * 1.5`)

Mesmo padrão do Achado 3 (mesma indentação hardcoded). **Fase A obrigatória**: confirmar
a proveniência vanilla específica de `terms`/`term_item` (não presumir que é a mesma
citação de `quote.rs` só porque o valor numérico é igual) — procurar
`typst-library/src/model/terms.rs` ou equivalente, `file:line` do default de indentação.

---

## Critérios de verificação (os 4 achados)

```
Dado ```code``` inline ou bloco, sem overrides
Quando renderizado
Então largura/posição batem com vanilla (Δw=0, Δx=0 nos casos medidos)

Dado --- (divider) entre parágrafos
Quando renderizado
Então espaçamento acima/abaixo bate com block.spacing=1.2em, não size*0.6

Dado #quote(block: true)[...] sem indent explícito
Quando renderizado
Então indent=1.0em, sem aspas inteligentes

Dado /Termo: Descrição (term_item) sem indent explícito
Quando renderizado
Então indent bate com o default vanilla confirmado na Fase A
```

Não-regressão: todos os testes existentes de `raw`/`divider`/`quote`/`terms`.

## Implementar e validar

```
crystalline-lint --checks v21 .
cargo test --workspace
```
Decalque contra corpus canónico — os 4 mudam output visual por defeito.

---

## Resultado esperado

Os 4 achados do P1053 corrigidos, com citação `file:line` real no código (não só no
relatório de diagnóstico). V21 reduz de 62 para ~58 (ou menos, se `divider.rs` estava
escondido nos "outros"). Fica para depois: Categoria 1 (30, centragem — provavelmente
legítima em bloco, a confirmar como classe antes de tratar item a item) e Categoria 2
(17, testes — fora do âmbito de correcção de produção).
