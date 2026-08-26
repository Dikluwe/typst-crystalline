# Prompt L0 — `compiler/stdlib/structural/table_lines` — linhas de tabela e grelha
Hash do Código: ba8a5360

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/table_lines.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada destas nativas (marcos P69…P962). Este L0
especifica **a superfície do nó**; o detalhe por marco vive no pai.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/layout/grid/` — as linhas fazem parte do resolve do grid. Co-mudança: P512+P513 e P739 movem as quatro nativas como bloco, nunca com as células.

---

## Contexto

As linhas explícitas de `grid` e `table`. Este L0 **absorve** os quatro prompts órfãos
escritos no Passo 512 e nunca activados como donos — `grid_hline.md`, `grid_vline.md`,
`table_hline.md`, `table_vline.md`. O conteúdo deles está aqui; os ficheiros originais
foram removidos e as suas excepções de órfão retiradas do `crystalline.toml`.

## Instrução

### Assinaturas

```typst
grid.hline(start: int = 0, end: int = auto, stroke: stroke = 1pt,
           position: auto | top | bottom = auto) -> content
grid.vline(start: int = 0, end: int = auto, stroke: stroke = 1pt,
           position: auto | left | right = auto) -> content
table.hline(…)  // idêntico a grid.hline
table.vline(…)  // idêntico a grid.vline
```

### Semântica

A linha é posicionada pela **ordem dos children**, não por coordenadas absolutas:
- `hline` com `position: auto` assume `top` se for a primeira coisa numa linha de
  children, senão `bottom`.
- `vline` com `position: auto` assume `left`.

O `row`/`column` efectivo é calculado por `native_table`/`native_grid` durante a
separação dos children, com base na ordem de aparecimento e no número de colunas.

### `stroke`

**P739A** — `stroke: none` é aceite (paridade vanilla, medido: compila, linha não
desenhada). A entidade guarda `Option<Stroke>`; o render salta a linha quando `None`.
Zero-thickness continua **proibido** (hairline em PDF — P726). O traço por omissão é
`default_hline_stroke()`, partilhado com `table_grid` (única aresta entre os nós).

### Validação

- `position` tem de estar no conjunto do eixo (`top`/`bottom`/`auto` para h,
  `left`/`right`/`auto` para v); fora disso, erro com a mensagem verbatim.
- `start >= 0`; `end` opcional (`auto`).
- Erros via `SourceDiagnostic::error(Span::detached(), …)`.
- `extract_line_range` é o helper comum de `start`/`end` (P512).

## Restrições Estruturais

- L1 puro.
- As quatro nativas são estruturalmente idênticas duas a duas (grid ↔ table); qualquer
  correcção numa aplica-se às restantes.

## Critérios de Verificação

```
#grid.hline()                        → linha no topo da primeira linha de células
#grid.hline(stroke: 2pt, position: bottom) → linha grossa na base
#grid.hline(start: 0, end: 2)        → linha de 0 até 2 colunas
#grid.hline(stroke: none)            → aceite, linha não desenhada (P739A)
#grid.vline(position: top)           → Err (eixo errado, mensagem verbatim)
#table.vline(position: bottom)       → Err
#table.hline() / #table.vline()      → mesmos defaults que os pares de grid
```
