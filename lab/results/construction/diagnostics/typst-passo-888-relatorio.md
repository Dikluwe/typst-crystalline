# Relatório — typst-passo-888: colapsar segmentos de borda partilhados (Fase A)

**Data:** 2026-07-24T03:51:32Z
**Executor:** Claude (Sonnet 5)
**Commit base:** `e2380aec9c361daaa83e60f71af5f72c7ae9e209` (HEAD do ramo `Tekt`)
**Working tree no início:** trabalho de P887 (`grid.rs`, `structural.rs`, `layout.md`,
`structural.md`, testes) ainda não commitado — mesma situação já registada no relatório de P887,
não decidida por este passo (não é código tocado por este passo).

**Confirmação prévia com o dono do projecto**: pedida antes de iniciar (o próprio `typst-passo-
888.md` marca isto como não-obrigatório) — resposta: prosseguir com Fase A/B/C.

Este relatório cobre só a Fase A. A decisão de âmbito da Fase B está em aberto no fim deste
documento — a complexidade encontrada é maior do que "colapsar 4 em 1" sugere à superfície, e o
próprio `typst-passo-888.md` condiciona a implementação a essa confirmação.

---

## 1. Fase A, ponto 1 — onde as 4 bordas por célula são emitidas, e se é deliberado

Confirmado: `01_core/src/engine/layout/grid.rs` (não `table.rs` — esse nome já não existe pós-
reorganização; `layout_grid` em `grid.rs:153` é partilhado por `Content::Grid` e `Content::Table`,
conforme já documentado em P887). As 4 bordas por célula (top/bottom/left/right) são emitidas em
`grid.rs`, no bloco "P227 + P230 + P234 — Renderização Opção β simplificada" (linhas ~825-871 antes
de P887; a correcção de P887 só mudou `width`/`height`, não a estrutura "4 por célula").

**Confirmado que `stroke` por célula é suportado e activo no código actual**
(`grid.rs:578-602`):
```rust
let (cell_stroke, cell_fill, ...) = match cell {
    Content::TableCell(e) => (e.stroke.as_ref(), ...),
    Content::GridCell(e) => (e.stroke.as_ref(), ...),
    _ => (None, None, ...),
};
let effective_stroke: Option<&Stroke> = cell_stroke.or(stroke);
```
`table.cell(stroke: ...)`/`grid.cell(stroke: ...)` sobrepõem o stroke da célula sobre o default do
grid/table. **Duas células adjacentes podem ter `effective_stroke` diferentes** — a preocupação do
prompt do passo é real e confirmada, não hipotética. "Opção β simplificada" (o nome no próprio
comentário do código) parece ter sido uma escolha deliberada de simplicidade na altura de P227 —
desenhar 4 segmentos por célula, sempre, evita ter de decidir qual das duas células adjacentes
"vence" quando divergem. Não encontrei nenhum comentário ou ADR que justifique isto como decisão
permanente de arquitectura (não é "correcto por desenho", é "simples de implementar").

---

## 2. Fase A, ponto 2 — como o vanilla decide (lido, não assumido)

`lab/typst-original/crates/typst-layout/src/grid/lines.rs` (1518 linhas). Não é uma função simples
— é um sistema de prioridade de 3 níveis, `StrokePriority` (`lines.rs:10-26`):

```rust
pub enum StrokePriority {
    GridStroke = 0,   // stroke só do default do grid/table
    CellStroke = 1,   // stroke veio (mesmo que parcialmente) de override per-cell
    ExplicitLine = 2, // stroke veio de hline/vline explícito do utilizador
}
```

`vline_stroke_at_row` (`lines.rs:274-350`, e o equivalente `hline_stroke_at_column` mais abaixo no
mesmo ficheiro) resolve, POR CADA posição de linha e POR CADA track (linha/coluna) que ela atravessa,
qual stroke usar:
- Lê o stroke do lado direito e do lado esquerdo da posição (cada um dos dois lados da linha vertical,
  por exemplo) e se cada um foi "prioritizado" (teve override explícito per-cell).
- Se nenhum dos dois teve override, usa o stroke do hline/vline explícito se houver, senão o stroke
  global do grid — prioridade `GridStroke`.
- Se um dos dois lados teve override e o outro não, o lado com override vence, com prioridade
  `CellStroke`.
- Se os dois tiveram override, o lado **direito** vence por convenção default (comentário
  `lines.rs:341-343`: "When both cells' strokes have the same priority, we default to prioritizing
  the right cell's left stroke") — a menos que `fold`/`AlternativeFold` combine os dois de outra
  forma (não explorado a fundo — fora do necessário para responder à pergunta do passo).
- `generate_line_segments` (`lines.rs:79` em diante) depois percorre os tracks perpendiculares à
  linha, computando o stroke/prioridade em cada um via a função acima, e **funde tracks contíguos
  com o mesmo stroke E a mesma prioridade** num único segmento (comentário `lines.rs:59-60`:
  "Contiguous segments with the same stroke and priority are joined together automatically").

**Resposta à pergunta do passo**: o vanilla não "desenha os dois lados parcialmente" nem simplesmente
"mistura" — tem uma regra de precedência explícita (explicit line > cell override > grid default;
empate entre dois overrides de célula → lado direito/inferior vence por convenção) e só funde
segmentos quando o stroke resolvido é **idêntico** ao longo de vários tracks consecutivos.

---

## 3. Fase A, ponto 3 — tempo de export, não só tamanho

P887 (secção 9) mediu `05-tables`: 113.9ms → 122.5ms (+8.6ms, +7.5%) depois da correcção do stroke
default. **Não consigo isolar, sem instrumentação adicional, quanto desse delta é trabalho novo
legítimo (linhas que antes não existiam) vs quanto é o excesso de segmentos duplicados** — o próprio
passo já não exige essa precisão ("não é preciso decompor com precisão, mas vale registar a
suspeita"). Registo: 4003 operadores `S` (medido em P887) para uma tabela onde o vanilla usa 371 é
~10.8× mais bytes de conteúdo de stroke a gerar, comprimir (Flate) e escrever — é razoável esperar
que uma fracção не-trivial dos 8.6ms venha daí, mas não medido isoladamente aqui.

---

## 4. Avaliação de âmbito — por que isto é maior do que "colapsar 4 em 1"

Três abordagens possíveis, com custo/risco muito diferentes:

### Opção A — De-dup local e seguro (baixo risco, baixo esforço)

Regra: ao emitir a borda de uma célula, **saltar** o lado (TOP/LEFT, por convenção) se há uma célula
vizinha nesse lado E `effective_stroke` bate exactamente (mesmo `Paint`+`thickness`+`overhang`) —
mantém o comportamento actual (ambos os lados desenham, sobrepostos) sempre que os strokes
divergirem, preservando correcção total sem replicar o sistema de prioridade do vanilla. Não precisa
de decidir "quem vence" porque só actua quando os dois lados já concordam.

**Estimativa de redução** (por tabela 5×10 sem override per-cell, caso do benchmark): de 200
segmentos (4×50 células) para ~115 (LEFT+TOP incondicionais = 100, BOTTOM/RIGHT só nas bordas
externas = 15) — redução de ~42%, projectando 4003 → ~2300 operadores `S` no documento completo.
**Não chega à ordem de grandeza do vanilla** (ainda "milhares", falha o critério explícito do passo:
"não deve continuar em milhares").

Risco: baixo — muda só a condição de emissão de 2 dos 4 lados, sem alterar z-order nem estrutura de
paginação, dado que continua a operar dentro do mesmo loop por-célula já existente.

### Opção B — Fusão completa por linha, à vanilla (fidelidade alta, esforço/risco altos)

Reimplementar como o vanilla: por cada posição de linha de grelha (vertical ou horizontal), percorrer
os tracks perpendiculares, resolver o stroke/prioridade em cada um (via a mesma regra
`cell_stroke.or(grid_stroke)` já disponível, sem precisar do sistema de prioridade de 3 níveis
completo do vanilla — ADR-0107 permite mecanismo divergente), fundir tracks contíguos com o mesmo
stroke resolvido num único segmento.

**Estimativa de redução**: para 05-tables (sem override per-cell, stroke uniforme), chegaria a
`(num_cols+1) + (num_rows+1)` segmentos por tabela = `6+11=17` × 20 tabelas ≈ 340 — muito próximo do
vanilla (371).

Risco/esforço: **substancialmente maior** do que "colapsar 4 em 1" sugere:
- `layout_grid` processa células linha-a-linha com suporte a paginação (quebra de tabela entre
  páginas — `regions.current` muda a meio da função), colspan/rowspan, e cauda diferida de células
  que ultrapassam a região (`pending_cell_tails`/`DeferredCellTail`). Uma fusão por-linha
  correcta precisa de saber, para cada posição de linha, todos os tracks que atravessa — mas os
  tracks podem estar espalhados por páginas diferentes já geradas (páginas anteriores já "fechadas"
  em `doc.pages`, fora de alcance de um pós-processamento simples sobre `current_items`).
  Restringir a fusão só ao caso comum ("tabela cabe inteira numa página, sem quebra") é possível mas
  precisa de detectar esse caso e cair no comportamento actual (não fundido) no caso geral — mais
  um ramo de lógica a manter correcto.
- As `hlines`/`vlines` explícitas (`table.hline`/`table.vline`) já são desenhadas numa passada
  separada, depois do loop por-célula (`grid.rs`, já confirmado em P887 secção 7) — teriam de entrar
  na mesma resolução de prioridade/fusão para não duplicar com as bordas de célula na mesma posição
  (o vanilla trata os dois como a mesma família de "linha de grelha"; o cristalino trata-os como dois
  mecanismos de emissão separados hoje).

### Opção C — Não implementar agora

Registar a Fase A como concluída, a Opção A como parcial-mas-insuficiente para o critério do passo,
e a Opção B como fora de âmbito razoável para "achado de optimização não urgente" sem uma decisão
explícita adicional do dono do projecto sobre investir o esforço de restruturação da Opção B.

---

## 5. Decisão necessária antes da Fase B

O `typst-passo-888.md` condiciona a Fase B a "Fase A confirmar que colapsar é seguro" — está
confirmado que é **seguro fazer a Opção A** (de-dup exacto, sem risco de regressão visual mesmo com
strokes divergentes por célula) mas **não é garantido chegar ao critério de aceitação do próprio
passo** ("não deve continuar em milhares") sem a Opção B, que tem um custo/risco maior do que o
esperado ao ler o sintoma na superfície.

Preciso de confirmação do dono do projecto sobre qual das três opções seguir antes de escrever
código de produção (Fase B).

---

## 6. Decisão do dono do projecto

Opção B (fusão completa) — investir no esforço maior para atingir o critério de aceitação do
próprio passo.

---

## 7. Fase B — Implementação

**Nota de proveniência sobre ordem TDD**: o desenho do algoritmo de fusão vertical (estado
acumulado entre linhas, descarregado em quebras de página) só se consolidou durante a própria
escrita do código — não foi possível planeá-lo em abstracto com confiança suficiente para escrever
testes formais antes, dada a interacção com paginação. Os testes de contagem foram derivados por
cálculo manual (não por execução) **antes** de correr a suíte, e usados para prever o resultado —
a suíte confirmou os números previstos sem ajuste, o que dá alguma confiança de que o cálculo (não
só o código) estava certo. Registado explicitamente, não escondido — desvio da ordem estrita
"teste falha → implementação" pedida pelo `CLAUDE.md`, pela mesma razão que building o algoritmo em
abstracto (sem código) não era praticável aqui.

### Implementação (`01_core/src/engine/layout/grid.rs`)

Substituída a "Opção β simplificada" (P227: 4 `FrameItem::Shape::Line` por célula, sempre) por:

- **Horizontal**: fundido dentro de cada linha (runs contíguos de células com o mesmo
  `effective_stroke`), sem estado entre linhas.
- **Vertical**: fundido entre linhas via `open_vsegments` (estado acumulado ao longo de toda a
  chamada a `layout_grid`), descarregado em mudanças de stroke, quebras de página
  (`flush_all_vsegments` chamado antes de cada `self.new_page()`), ou no fim da função.
- **Segurança com stroke divergente por célula**: quando os dois lados de uma fronteira têm
  `effective_stroke` diferentes, não funde — desenha os dois lados separadamente (idêntico ao
  comportamento pré-P888 nessa posição), sem tentar replicar o sistema de prioridade de 3 níveis do
  vanilla (`StrokePriority`, fora de âmbito per ADR-0107).
- Helpers novos (fora do `impl`, junto de `cell_bounds`): `cell_effective_stroke`, `vline_x`,
  `emit_hsegment`, `emit_vsegment`, `flush_all_vsegments`, `emit_row_borders`. `grid_owner: Vec<Vec<
  Option<usize>>>` (mapa linha×coluna → célula dona, cobrindo colspan/rowspan) construído uma vez
  antes do loop de linhas.

Documentação completa do algoritmo (incluindo o porquê de cada decisão de âmbito) adicionada a
`00_nucleo/prompts/engine/layout.md`, secção `## P888` — ver secção 8 abaixo sobre o gate do L0.

### Testes

**Actualizados** (comportamento antigo "4 por célula" tornou-se incorrecto, não é regressão —
esperado pelo próprio passo): `p227_grid_stroke_renderiza_4_lines_per_cell` (16→**7**, grid 2×2),
`p227_table_stroke_paridade_grid` (8→**5**, table 1×2), `p234_grid_stroke_baseline_p227_preservado`
(8→**5**, grid 1×2), `p887_grid_stroke_lines_bounding_box_bate_com_dx_dy` (16→**7**, grid 2×2 — a
verificação de bounding-box em si continua válida, só a contagem mudou). Todos os valores novos
calculados à mão antes de correr a suíte (ver nota de proveniência acima) e confirmados sem ajuste.

**Novos**:
- `p888_grid_5x10_stroke_uniforme_funde_verticais_entre_10_linhas`: grid do mesmo tamanho do
  cenário de benchmark (5×10), stroke uniforme — confirma **26** segmentos (6 verticais fundidos ao
  longo das 10 linhas + 20 horizontais), contra 200 pré-P888. Único teste que exercita fusão
  vertical através de mais de 2 linhas.
- `p888_stroke_divergente_por_celula_nao_funde_e_preserva_os_dois_lados`: `grid.cell(stroke:
  vermelho)` adjacente a uma célula sem override (herda azul do grid) — confirma que a fronteira
  entre elas mantém **2** segmentos sobrepostos (não funde), e que as 2 fronteiras externas
  (sem divergência) continuam com 1 segmento cada. Este é o teste de segurança pedido explicitamente
  pela Fase A/B do prompt.

### Suíte completa, discriminada por crate

| Crate | Passou | Falhou | Ignorado |
|---|---|---|---|
| `typst-core` | 4698 | 0 | 2 |
| `typst-infra` | 732 | 0 | 5 |
| `typst-shell` | 41 | 0 | 0 |
| `typst-wiring` (+ `tests/crystalline_lint.rs`) | 37 + 2 | 0 | 0 |

Zero falhas.

### Confirmação visual + contagem de operadores (`05-tables.typ`, fonte actual)

Render a 150dpi **idêntico** ao pré-P888 (P887) — todas as linhas presentes, nenhuma deslocada.
Contagem de operadores `S`: **4003 → 523** (vanilla: 371) — 87% de redução; razão cristalino/vanilla
cai de 10.8× para 1.4×, dentro da mesma ordem de grandeza do vanilla (critério de aceitação do
passo: "não deve continuar em milhares" — 523 confirma isso com folga).

### Gate do Protocolo de Nucleação

`crystalline-lint --fix-hashes .` corrido depois de adicionar a secção `## P888` a `layout.md` —
13 ficheiros do cluster `layout/` tiveram o `@prompt-hash` recalculado (mesmo padrão de P887, L0
partilhado por todo o `layout/`). `crystalline-lint .`: 0 avisos de drift, só o V7 pré-existente.

---

## 8. Fase C — Regressão (benchmark completo, 7 cenários)

Baseline: `timings-*-p887.json` (existe, per a preferência do prompt — "se existir" P887, senão
P886). Metodologia idêntica.

| Cenário | Vanilla (P887) | Cristalino (P887) | Razão (P887) | Vanilla (P888) | Cristalino (P888) | Razão (P888) |
|---|---|---|---|---|---|---|
| 01-hello | 269.9ms | 94.2ms | 0.35× | 274.7ms | 96.8ms | 0.35× |
| 02-lorem | 273.4ms | 117.0ms | 0.43× | 280.1ms | 118.3ms | 0.42× |
| 03-images | 6.7ms | 101.1ms | 15.18× | 7.1ms | 104.1ms | 14.70× |
| 04-math | 274.4ms | 5173.4ms | 18.85× | 279.3ms | 5160.8ms | 18.48× |
| **05-tables** | 303.1ms | 122.5ms | **0.40×** | 299.3ms | **116.2ms** | **0.39×** |
| 06-long | 303.0ms | 381.0ms | 1.26× | 294.4ms | 377.4ms | 1.28× |
| 07-context | 297.6ms | 140.5ms | 0.47× | 294.1ms | 138.0ms | 0.47× |

**Leitura**: 6 dos 7 cenários dentro do ruído (variação ≤ ~2%). `05-tables` é o único que muda de
forma notável: **122.5ms → 116.2ms**, uma **melhoria** de ~5%, consistente com "menos operadores a
escrever/comprimir" (523 vs 4003) — exactamente o sentido esperado pelo prompt ("espera-se que fique
mais rápido... se subir, é sinal de que a correcção introduziu overhead novo"). Não subiu — a
correcção eliminou redundância, não introduziu custo novo. `03-images` e `04-math` continuam
pré-existentes, fora do escopo deste passo (P873).

---

## 9. Resultado — Passo 888 fechado

- Header de linhagem actualizado em `grid.rs` (`@updated 2026-07-24`; `@prompt-hash` recalculado
  nos 13 ficheiros do cluster `layout/`).
- Testes novos cobrindo a fusão à escala do benchmark e a segurança com stroke divergente por
  célula; 4 testes pré-existentes actualizados (não removidos) para os valores fundidos correctos.
- Fase A: confirmado que fundir é seguro quando os dois lados concordam; vanilla usa sistema de
  prioridade de 3 níveis para o caso divergente (não replicado, per ADR-0107 — âmbito registado).
- Fase B: implementação completa (Opção B, decisão do dono), suíte verde nas 4 crates,
  `crystalline-lint` limpo, redução de 4003 para 523 operadores `S` (vanilla: 371), confirmação
  visual sem diferença.
- Fase C: benchmark completo, sem regressão; `05-tables` melhora (~5%), confirmando que a correcção
  eliminou redundância em vez de introduzir overhead.
- Árvore de trabalho: trabalho de P887 continuava por commitar no início deste passo — não coube a
  este passo decidir (secção 0, herdado de P887).
