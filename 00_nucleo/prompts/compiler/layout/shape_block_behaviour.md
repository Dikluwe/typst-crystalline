# Prompt L0 — `compiler/layout/shape_block_behaviour` — `Content::Shape` como bloco que quebra parágrafo

Hash do Código: badad90c

**Camada**: L1 · **Alvo**: `01_core/src/compiler/layout/shape.rs` (com impacto no dispatch de `Content::Shape` em `compiler/layout/mod.rs`)
**Origem**: P767 — arqueologia de P763h.
**ADRs**: ADR-0107 (paridade linguagem), ADR-0108 (anti-deriva — medir antes de decidir), ADR-0109 (atomização forma B).
**Prompts relacionados**: `entities/elements/shape.md`, `entities/geometry.md`, `compiler/layout/block.md`, `compiler/atomizacao_elementos.md` (padrão de atomização da implementação).

---

## Propósito

Este prompt altera o **comportamento observável** de `Content::Shape` no cristalino para que coincida com o Typst vanilla: as primitivas de desenho (`rect`, `square`, `ellipse`, `circle`, `line`, `polygon`, `curve`) são elementos de **bloco** que **quebram o parágrafo corrente** antes e depois de si. Não se trata de alterar a geometria nem o exportador — apenas o posicionamento relativo a texto e outros conteúdos no fluxo do documento.

Base de evidência (P763h):
- Em `lab/typst-original/crates/typst-layout/src/engine.rs:765-803`, todas as primitivas de desenho do vanilla são realizadas como `BlockElem::single_layouter`.
- O vanilla emite `warning: block may not occur inside of a paragraph and was ignored` quando uma forma ocorre dentro de `#par[...]`.
- No cristalino actual, `Content::Shape` é processado sequencialmente no mesmo fluxo de texto, o que produz AE 1864–7070 em documentos que misturam texto e formas.

---

## 1. Contrato de comportamento

Qualquer `Content::Shape` (independentemente do `ShapeKind`) deve comportar-se como um bloco auto-contido, equivalente a `BlockElem::single_layouter(...)` do vanilla:

1. **Quebra de parágrafo implícita**: antes de posicionar a forma, o parágrafo/linha corrente é terminado; depois da forma, o conteúdo subsequente inicia um novo parágrafo/linha.
2. **Não participa em linhas de texto**: uma forma nunca pode aparecer na mesma linha horizontal que texto circundante. Texto antes da forma fica na(s) linha(s) anterior(es); texto depois da forma começa abaixo dela.
3. **Empilhamento vertical**: duas ou mais formas consecutivas (ou intercaladas com outros blocos) são empilhadas verticalmente, com espaçamento de bloco entre elas.
4. **Posicionamento interno inalterado**: o canto superior-esquerdo da forma mantém-se alinhado a `cursor_x` (margem esquerda do contentor) e à baseline/altura de linha corrente, conforme já estabelecido em `shape.rs` (P748/P750). A mudança é apenas o contexto de fluxo em torno da forma.
5. **Ancoramento vertical correto (P767c)**: quando uma forma sucede texto não-bloco no mesmo parágrafo, o vanilla ancora a *base* da forma em `baseline + above` e estende-a para cima; a próxima baseline do texto fica em `shape_top + below + cap_height`. Quando a forma sucede outro bloco ou é a primeira de uma Sequence sem texto antes, mantém-se o modelo P767a (`shape_base = cursor_y − cap_height`, avanço `shape_base + height + below`).

> **Decisão sobre aviso**: o vanilla emite aviso quando uma forma ocorre dentro de `#par[...]` explícito. O cristalino **não precisa de replicar essa mensagem** — basta que quebre o parágrafo silenciosamente, tal como já fazem outros elementos de bloco do cristalino quando encontrados no fluxo.

---

## 2. Ponto de intercepção

A implementação deve reutilizar a infraestrutura de blocos já existente no `Layouter`, em vez de inventar um mecanismo paralelo:

- `Content::Block` em `01_core/src/compiler/layout/block.rs` já implementa terminamento de linha (`flush_line()`), espaçamento `above`/`below` e tracking de blocos consecutivos (`block_chain_active`, `prev_block_below_pending`).
- `Content::Heading` em `01_core/src/compiler/layout/heading.rs` e `Content::ListItem`/`EnumItem` também terminam a linha em curso antes de se posicionarem.
- A mudança pode ser feita **num dos seguintes pontos** (a escolher em P767a com base no menor impacto):
  - **Opção A (preferida se viável)**: modificar `shape::layout` para invocar o mesmo protocolo de bloco que `block::layout` (flush, aplicação de `above`/`below`, flag `block_chain_active`).
  - **Opção B**: no realizador ou no braço `Content::Shape` de `layout_content`, envolver implicitamente a forma num `BlockElem` com body vazio e emitir a forma como conteúdo do bloco. Isto reaproveita toda a lógica de `block::layout`, mas pode complicar `place()` e medições.
  - **Opção C**: introduzir um estado "paragraph broken" no `Layouter` que force o início de uma nova linha/parágrafo após a forma. Só se A e B forem inviáveis.

A escolha final deve ser justificada no relatório de P767a com medição de AE antes/depois.

---

## 3. Espaçamento `above` / `below`

O vanilla usa `BlockElem` com `spacing` por defeito igual a `1.2em` (`Em::new(1.2).into()`), que se propaga para `above` e `below` quando não especificados (`container.rs:342-354`).

- As formas cristalinas devem aplicar o **mesmo espaçamento por defeito** entre si e entre texto/blocos adjacentes.
- O valor concreto (1.2em) deve ser medido contra o vanilla num documento de teste simples (ex.: `#rect(...)#rect(...)`) antes de ser hardcoded.
- Se o `Layouter` já tiver um mecanismo de `above`/`below` collapse (P250), reutilizá-lo. Não duplicar lógica de colapso.

---

## 4. Impacto em `place()`

Formas dentro de `place(...)` (posicionamento absoluto) **não são afectadas** por este L0:

- `place()` foi corrigido em P763f; o caminho absoluto não passa pelo fluxo normal de parágrafo.
- A verificação de P767a deve confirmar que documentos como `#place(top+left, rect(...))` mantêm AE baixo.

---

## 5. Impacto nos testes de regressão de texto (P745–P762)

Antes de implementar, executar uma busca nos testes de layout por documentos que misturem texto literal com formas no mesmo fluxo (ex.: `"A #rect(...) B"`).

- Não foram encontrados testes deste tipo na base actual (`01_core/src/compiler/layout/tests.rs`).
- Se algum teste futuro depender do comportamento actual (fluxo contínuo), esse teste deve ser actualizado consciente — não é uma regressão, é a correcção do modelo.
- A validação de P767a inclui `cargo test --workspace` e repetição das medições de P763h (formas isoladas e misturadas com texto) para todas as primitivas, além do checklist de sub-layouts (grid, box, columns, place).

---

## Critério de aceitação (para P767a)

- [ ] Documentos com forma isolada mantêm AE próximo do baseline (5–300).
- [ ] Documentos com texto misturado com formas passam a empilhar formas e texto como blocos, com AE próximo do baseline.
- [ ] Formas dentro de `place()` não regressam.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações (excepto V7 esperado).
- [ ] Header `@prompt` / `@prompt-hash` actualizado em `shape.rs` (via `crystalline-lint --fix-hashes .`).
