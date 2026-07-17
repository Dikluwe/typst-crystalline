---
# P772v — `table()` header/footer como row-groups (extensão do padrão de `grid()`)

> **Passo:** 772v
> **Data:** 2026-07-17
> **Foco:** P772i implementou `grid.header`/`grid.footer` como row-groups reais para `grid()`, mas registrou explicitamente que `table()` ficou fora do âmbito — `TableElem` não tem os campos `header`/`footer` que `GridElem` ganhou, e o loop de `table()` trata `Content::TableHeader`/`TableFooter` como célula normal (mesmo scope-out #16 de P772f, variante table). Este passo estende o mecanismo já validado, sem redesenhar.
> **Tipo:** Implementação directa (padrão já estabelecido e testado em P772i; extensão estrutural, não descoberta).
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — confirmar que `table.header`/`table.footer` do vanilla têm exatamente a mesma forma de API que `grid.header`/`grid.footer`, não assumir por analogia.
> **Dependências:** P772i (mecanismo original, `split_header_footer`, commit a confirmar), P772f (achado original do scope-out #16, variante table).

---

## Sonda — confirmar paridade de API entre `table` e `grid` no vanilla

```bash
grep -n "#\[elem(name = \"header\"\|#\[elem(name = \"footer\"" lab/typst-original/crates/typst-library/src/layout/table/mod.rs 2>/dev/null
```

Confirmar se `table.header(...)`/`table.footer(...)` são exatamente o mesmo mecanismo de `grid.header`/`grid.footer` (elementos-filho, mesma estrutura `Repeatable<T>`/`level`/`repeat`), ou se há diferenças específicas de `table` (ex: estilo visual padrão de header/footer em tabelas, que `grid` não tem).

```bash
grep -n "struct TableElem\|header\|footer" 01_core/src/entities/*.rs | grep -i table
```

Confirmar o estado atual de `TableElem` no cristalino.

---

## Implementação

1. Adicionar campos `header: Option<Content>`/`footer: Option<Content>` a `TableElem`, espelhando `GridElem` pós-P772i.
2. Estender `split_header_footer` (ou replicar o padrão) para `table()`, distinguindo `Content::TableHeader`/`TableFooter` no loop de resolução, em vez de caírem no braço genérico.
3. Reutilizar o motor de layout existente — as células de header/footer extraídas do `body`, preenchidas até múltiplo de `num_cols`, coladas antes/depois das células normais (mesmo padrão de P772i).
4. Confirmar se `table()` tem alguma diferença visual padrão para header/footer (ex: linha de destaque, negrito) que `grid()` não tem — não assumir que é puramente estrutural sem verificar o vanilla.

---

## Validação

```bash
cat > /tmp/p772v-table-header.typ <<'EOF'
#table(
  columns: 2,
  table.header[Nome][Idade],
  [Ana], [30],
  [Bruno], [25],
)
EOF
lab/typst-original/target/release/typst compile /tmp/p772v-table-header.typ /tmp/p772v-vanilla.pdf
./target/release/typst compile /tmp/p772v-table-header.typ /tmp/p772v-cristalino.pdf
mutool trace /tmp/p772v-vanilla.pdf > /tmp/p772v-trace-vanilla.txt
mutool trace /tmp/p772v-cristalino.pdf > /tmp/p772v-trace-cristalino.txt
```

Confirmar por coordenadas, não só ausência de erro — mesma disciplina de P772i.

```bash
# Confirmar que o bug do .first() (P772i, achado adicional) não se repete aqui
cat > /tmp/p772v-multi-celula.typ <<'EOF'
#table(columns: 2, table.header[A][B])
EOF
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Paridade de API `table.header`/`footer` vs `grid.header`/`footer` confirmada contra o vanilla.
- [ ] `TableElem` com campos `header`/`footer`.
- [ ] `Content::TableHeader`/`TableFooter` distinguidos no loop de resolução.
- [ ] Coordenadas confirmadas contra o vanilla.
- [ ] Múltiplas células em `table.header[A][B]` preservadas (não repetir o bug `.first()` de P772i).
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] L0 atualizado.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772v.md`.

---

## Próximo passo

Resíduo de risco plausível do inventário original: `foundations::target_`, `plugin_`, `image::pdf`, `layout::frame`, `math` — 23 itens em 5 módulos, conforme P772t.
