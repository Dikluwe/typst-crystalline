---
# P647 — Grid inválida produz erro, em vez de renderizar vazio em silêncio

> **Passo:** 647
> **Data:** 2026-07-09
> **Foco:** P633 confirmou (caso 7) que `place_cells(cells, num_cols).unwrap_or_default()` descarta o erro de grid inválida (colspan em conflito, célula fora dos limites) e renderiza um vector vazio. P638 confirmou que o vanilla também não reporta este problema — mas por um mecanismo diferente (não detecta o conflito, em vez de detectar e descartar). Isto é melhoria além do vanilla, não correcção de paridade: o critério não é bater com o vanilla, é produzir um erro claro.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** Sem paridade a seguir aqui — a decisão de que erro produzir, e quando, é do cristalino, não copiada do vanilla.

---

## Contexto

`01_core/src/rules/layout/grid.rs:322`:

```rust
let cells = place_cells(cells, num_cols).unwrap_or_default();
```

Se `place_cells` encontrar um conflito (duas células a reclamar a mesma posição por causa de `colspan`/`rowspan`, ou uma célula com posição fora dos limites da grid), o erro é descartado e a grid renderiza vazia — sem aviso, sem indicação de qual célula causou o problema.

---

## Sonda

### Confirmar os tipos de erro que `place_cells` pode devolver

```bash
grep -n "fn place_cells\|Err(" 01_core/src/rules/layout/grid.rs | head -20
```

Listar os diferentes tipos de conflito que a função já detecta, para decidir a mensagem de erro certa para cada um.

### Confirmar o que faz mais sentido como comportamento do cristalino

Dado que não há vanilla a seguir aqui, decidir: erro que pára a compilação (mais rigoroso, mas pode ser demasiado disruptivo para um documento que só tem um problema pequeno numa grid) ou aviso que continua com a célula problemática omitida, mas avisando qual foi (mais tolerante, mas o utilizador ainda sabe o que aconteceu).

### Critério de fecho da sonda

- [ ] Tipos de conflito já detectados por `place_cells` confirmados, com `file:line`.
- [ ] Decisão registada: erro que pára, ou aviso que continua — com razão escrita, não assumida.

---

## Implementação

Depende da decisão da sonda. Se for erro: propagar via `SourceResult`, com mensagem que identifique a célula ou posição em conflito. Se for aviso: usar `engine.sink.warn(...)`, mantendo o comportamento de renderizar o resto da grid, mas avisando qual célula foi omitida e porquê.

### Critério de fecho da implementação

- [ ] Conflito de grid produz erro ou aviso claro (conforme decidido), identificando a célula/posição problemática.
- [ ] Grid válida sem regressão.
- [ ] Testado com pelo menos dois tipos diferentes de conflito (colspan em conflito, posição fora dos limites), não só um.

---

## Validação

```bash
cat > /tmp/p647-grid-conflito.typ <<'EOF'
#grid(
  columns: 2,
  grid.cell(colspan: 2)[A],
  grid.cell(colspan: 2)[B],
  [C], [D],
)
EOF
./target/release/typst /tmp/p647-grid-conflito.typ /tmp/p647.pdf
echo "Exit code: $?"
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, decisão de comportamento (erro vs. aviso) registada com razão.
- [ ] Conflitos de grid produzem diagnóstico claro, identificando a célula.
- [ ] Grid válida sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p647.md`, com hash do commit.
- [ ] Registado explicitamente como melhoria além do vanilla, não correcção de paridade.

---

## Estado da sequência de falhas silenciosas

Com este passo, os sete casos originais de "perda ou corrupção de conteúdo/estado" (secção 2.1 de P633) ficam todos tratados. Falta só o débito do parser/lexer, já registado por P634 e reconfirmado por P643, como o último item pendente desta linha de trabalho.
