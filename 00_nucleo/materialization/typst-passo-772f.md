---
# P772f — Varredura de `typst_library::layout::grid::resolve`

> **Passo:** 772f (continuação da série P765a→P772e)
> **Data:** 2026-07-16
> **Foco:** `layout::grid::resolve` é o maior módulo restante na lista `lacuna-inventario` (24 itens) e tem alto potencial de efeito observável: resolve alinhamento, spanning, e dimensionamento de células em `#grid` e `#table`. Divergências aqui alteram o layout renderizado.
> **Tipo:** Sonda + Implementação directa para achados confirmados.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0107** — efeitos observáveis = layout renderizado, mensagens de erro, e semântica da linguagem (`#grid`/`#table`).
> **Dependências:** P772e (decisão de continuar varredura selectiva).

---

## Sonda — classificar os 24 itens

```bash
awk -F'\t' '$1=="lacuna-inventario" && $5 ~ /typst_library::layout::grid::resolve/' \
  00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt | cut -f5 | sort
```

Para cada item:

```bash
grep -n "<item>" lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs 2>/dev/null
```

E o estado no cristalino:

```bash
grep -rn "<item>" 01_core/src/rules/layout/grid* 02_shell/src/* 03_infra/src/* 2>/dev/null
```

### Casos de teste prioritários

Focar em diferenças que sejam observáveis:

1. **Células vazias / spanning:** `#grid(columns: 3, rows: 2, ..)` com células que se estendem para além da grid.
2. **Dimensionamento automático:** `auto` vs `1fr` vs tamanhos fixos em combinações complexas.
3. **Alinhamento de conteúdo:** `align` em células individuais vs grid global.
4. **Mensagens de erro:** grid malformada (ex: colspan excessivo) deve produzir mensagens idênticas ao vanilla.

```bash
cat > /tmp/p772f-grid.typ <<'EOF'
#grid(
  columns: (1fr, 1fr, 1fr),
  rows: (1cm, 1cm),
  align: center + horizon,
  [A], [B], [C],
  [D], grid.cell(colspan: 2, [EF]),
)
EOF
lab/typst-original/target/release/typst compile /tmp/p772f-grid.typ /tmp/p772f-grid-vanilla.pdf
./target/release/typst /tmp/p772f-grid.typ -o /tmp/p772f-grid-crys.pdf
```

Comparar visualmente (ou via extração de texto) se o layout diverge.

---

## Implementação

Só para achados confirmados como bug real (layout divergente ou mensagem de erro observávelmente diferente).

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Os 24 itens de `layout::grid::resolve` classificados item a item.
- [ ] Casos de grid/table com spanning e dimensionamento testados contra vanilla.
- [ ] Bugs reais corrigidos com teste (layout ou mensagem exacta).
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772f.md`.

---

## Próximo passo

Após P772f, avaliar resultados e decidir se continua para `image::svg`, `foundations::scope`, `text::font::*`, ou encerra a série.
