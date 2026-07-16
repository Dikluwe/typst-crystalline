---
# P772j — L0 + reimplementação: alinhamento efectivo per-cell em `grid()`

> **Passo:** 772j
> **Data:** 2026-07-16
> **Foco:** P772h confirmou que o código órfão encontrado por P772f (`Content::Place` envolvendo o corpo da célula quando `effective_align` é `Some`) veio de uma sessão anterior de Claude Code que começou a implementar P772f antes da versão que efectivamente fechou o passo, sem nunca ter L0, testes, nem commit. O código foi deixado no working tree, não removido. A intenção de desenho original não é recuperável com certeza — a decisão é reverter e refazer com o processo completo (L0 primeiro), não tentar reaproveitar o código órfão directamente.
> **Tipo:** L0 + Implementação. Reverter o código órfão antes de escrever qualquer coisa nova.
> **Tamanho:** M.
> **ADR-0108/ADR-0114 EM VIGOR** — funcionalidade nunca fechada com sonda própria; sondar do zero, não assumir que o código órfão estava no caminho certo só por já existir.
> **Regra de ouro do CLAUDE.md** — L0 antes de código, sem excepção, mesmo reimplementando algo que já existiu parcialmente.
> **Dependências:** P772h (arqueologia, conclusão de que a origem não é recuperável e a decisão assenta só no mérito), P772f (medição original: `#grid(align: center, ...)` diverge do vanilla em ~13.5pt com o código órfão presente).

---

## Passo 0 — Reverter o código órfão

```bash
git diff 01_core/src/rules/layout/grid.rs | grep -B3 -A20 "P772f — aplicar align efectivo"
```

Confirmar o âmbito exacto do bloco antes de reverter (não remover mais nem menos do que o identificado por P772f/P772h).

```bash
git checkout -- 01_core/src/rules/layout/grid.rs
```

Ou, se houver outras alterações legítimas no mesmo ficheiro que não devem ser perdidas, reverter só o bloco específico manualmente.

Confirmar que `cargo test --workspace` continua verde depois de reverter — se o código órfão já estava a corrigir algo por acidente, isso apareceria aqui.

---

## Sonda — comportamento real do vanilla para `align:` em `grid()`

```bash
grep -n "fn resolve_cell\|align\b" lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs | head -30
```

Confirmar exactamente como o vanilla resolve `align` efectivo por célula: precedência entre `align:` do grid, `align:` per-célula (`grid.cell(align: ...)`), e alinhamento herdado do contexto. Não assumir a partir do nome da propriedade — ler a função de resolução real.

```bash
cat > /tmp/p772j-align-vanilla.typ <<'EOF'
#grid(columns: 2, align: center, [Hello], [World])
EOF
lab/typst-original/target/release/typst compile /tmp/p772j-align-vanilla.typ /tmp/p772j-vanilla.pdf
mutool trace /tmp/p772j-vanilla.pdf | grep -A3 "Hello\|World"
```

Registar a posição X exacta de "Hello" no vanilla — este é o valor-alvo (P772f mediu `x=20.247` no vanilla vs `x=33.772` no cristalino com o código órfão).

---

## Implementação

### L0

Escrever/actualizar `00_nucleo/prompts/rules/layout.md` (secção de `grid`) com o mecanismo de alinhamento efectivo por célula: como o alinhamento é resolvido (precedência), e como é aplicado no layout (mecanismo escolhido — não necessariamente `Content::Place`, confirmar se é a abordagem certa ou se alinhamento dentro de célula deve ser tratado de outra forma, dado P772g ter acabado de mexer exactamente nesta área de composição de coordenadas).

### Código

Implementar conforme o L0, reutilizando o mecanismo de composição de coordenadas já corrigido em P772g (não reintroduzir o padrão "recompositor" que P772g acabou de eliminar de `layout_align`/footnotes — se a nova implementação usa `Content::Place` internamente, aplicar a mesma disciplina de origem absoluta vs delta que P772g estabeleceu).

---

## Validação

```bash
cat > /tmp/p772j-align-test.typ <<'EOF'
#grid(columns: 2, align: center, [Hello], [World])
EOF
./target/release/typst compile /tmp/p772j-align-test.typ /tmp/p772j-cristalino.pdf
mutool trace /tmp/p772j-cristalino.pdf | grep -A3 "Hello\|World"
```

Confirmar ΔX ≈ 0 contra o vanilla (P772h registou o alvo: `x=20.247`).

Checklist de sub-layouts (regra 5): `align` em célula de grid dentro de `columns`, dentro de `box`, etc.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [x] Código órfão revertido, escopo confirmado, testes continuam verdes.
- [x] Precedência de `align` em `grid()` confirmada contra o vanilla real, não assumida — fold por eixo (era `.or()` do `Align2D` inteiro).
- [x] L0 escrito antes do código (`00_nucleo/prompts/rules/layout.md`, hash `a214cd68`).
- [x] Implementação nova, reutilizando a disciplina de coordenadas de P772g.
- [x] `#grid(align: center, ...)` bate com o vanilla — divergência residual pequena (~1.7pt, era 13.5pt) atribuída e documentada a uma limitação mecânica já existente (medição aproximada vs real de largura), não a um bug de mecanismo ou precedência.
- [x] Checklist de sub-layouts (`columns`, consistente com P772g).
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772j.md`.

---

## Próximo passo

Com P772g, P772i e P772j fechados, a linha de achados extra de P772f está resolvida. Retomar a varredura da stdlib: `visualize::image::svg` (7 itens), `foundations::scope` (7 itens), `text::font::*` (~22 itens).
