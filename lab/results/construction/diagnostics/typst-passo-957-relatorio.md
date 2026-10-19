# Passo 957 — Relatório (peça inferior do assembly: baseline no topo do slot → sequência de passos rodada)

**Data**: 2026-08-03
**Estado da árvore**: commit base `00cd5bc56`; a correcção é 1 linha em
`assembly.rs` + shift simplificado + L0 §P957 + 3 testes novos.

---

## 1. Fase A — causa exacta (medida, glifo a glifo)

**Sintoma (dono)**: em delimitadores de 3+ linhas (caminho de assembly), a
peça de cima tem a forma certa e a de baixo sai como canto reto tipo `⌊`,
em três tipos de delimitador (`{`, `(` de matriz aumentada, `(` de matriz de
reticências).

**Cadeia de evidência** (toda a análise intermédia em `temp/p957/`):

1. **Identidade das peças correcta no subset**: os charstrings CFF embutidos
   no PDF cristalino têm hash idêntico aos do subset vanilla para todas as
   peças de `(` e `{` (topo/extensor/fundo). Não era fallback partilhado nem
   glifo errado no subset.
2. **Posições das peças divergiam do vanilla**: passos entre baselines
   consecutivas (valores reais, `(` de matriz aumentada, 4 peças):
   cristalino **[429, 376, 1426]du** vs vanilla **[1426, 376, 429]du** —
   exactamente rodados de uma posição. Chave (5 peças): cristalino
   [555, 1308, 556, 557] vs vanilla [558, 556, 1308, 556] — mesma rotação.
   Os passos do vanilla batem a fórmula do seu `assemble`
   (`glyph.rs:631-641`, `advance_i − overlap_i + ratio×(overlap_i −
   min_overlap)`) com r=0.788 (parêntese) / r=0.513 (chave) — fit perfeito.
3. **Causa**: `layout_assembly` emitia
   `y_in_box = total_height − y_from_bottom − advance_pt` — baseline da peça
   no **topo** do seu slot. O passo entre baselines consecutivas saía
   `advance_{i+1} − overlap_i` (advance da peça ERRADA — a seguinte) em vez
   de `advance_i − overlap_i`. Cada peça era desenhada `advance_i` acima do
   sítio certo: a peça inferior (gancho curvo) ficava coberta pelo extensor
   e o fundo do delimitador mostrava a haste recta — o "canto reto".
4. **Pistas falsas descartadas por medição** (registadas para não serem
   re-percorridas): (a) uma sonda com `with_system_fonts` mostrou gids de
   DejaVu Sans (3507+) — artefacto da sonda sem as fontes embutidas; o
   caminho de produção (`with_fonts_and_system`) resolve NewCMMath-Book com
   os gids correctos; (b) suspeita de subset/remap — refutada pelos hashes
   dos charstrings (ponto 1).
5. **Vanilla confirmado** a usar as mesmas peças com posições diferentes por
   tipo de delimitador (stream do vanilla decomposto por Tm/Tj por peça).

## 2. Fase B — correcção (TDD directo — correcção pontual de fórmula)

- **L0 primeiro**: `assembly.md` §P957 (medição → causa → correcção, com os
  números).
- **Testes RED** (`01_core/src/engine/math/layout/tests.rs`, valores reais de
  NewCMMath-Book): `p957_assembly_paren_passos_na_ordem_vanilla` (identidade
  [uni239D, ext, ext, uni239B] + passos [1444.17, 413.65, 447.17]du com
  alvo 3800du/r=0.8654), `p957_assembly_brace_passo_grande_apos_peca_do_meio`
  (5 peças, passo grande no 3º intervalo), `p957_assembly_tinta_centrada_no_eixo`
  (guarda: centro da tinta em −axis + tinta == [−ascent, descent]).
  Confirmados RED nos dois de passos (rotação medida no sintético:
  [447, 414, 1444] obteve-se antes da correcção).
- **Correcção** (`assembly.rs`): `y_in_box = total_height − y_from_bottom`
  (baseline no fundo do slot — o termo `− advance_pt` removido) e
  `shift_y = −axis_pt − total/2` (com baselines no fundo, a tinta ocupa
  exactamente `[0, total]` — a fórmula de P952b era a compensação da
  convenção errada e deixa de ser usada; registado no L0).
- **Suite**: `cargo test --workspace` verde — 4840 core (incl. guardas
  P945/P952 que sobrevivem à mudança) + restantes crates, 0 falhas.
- **Visual** (`temp/p957/`, 400dpi): `{`, `(`, `)` e `[` (generalização) com
  ganchos/curvas correctos em cima e em baixo, lado a lado com o vanilla.

## 3. Fase C — revalidação

- **30/30 secções revistas lado a lado** (`temp/p957/batch-*.png`, cristalino
  | vanilla ancorado por cabeçalho de secção): sem regressão noutros
  delimitadores. Divergências pré-existentes mantêm-se como estavam
  (conteúdo `lr` literal na sec 22, decoradores da sec 10, etc.).
- **Diff pré/pós-P957 confinado**: pixel-diff entre o render pós-P956 e
  pós-P957 mostra alterações **só em 3 regiões** — sec 5 (matrizes), sec
  7-9 (binómio/cases), sec 21 (matrizes avançadas) — todas assemblies de
  delimitadores, todas corrigidas para a forma vanilla. Nenhuma outra zona
  do documento mudou.
- **`compare.py`** (cristalino vs vanilla): mediana das medianas |dx| =
  **1.903pt** (era 1.925 em P956 — dentro do ruído), med|dy| = 0 em todas
  as secções. Sec 21 (matrizes com assembly): 9.331 → **1.100**. As secções
  ainda elevadas são as divergências de conteúdo já catalogadas.
- **Benchmark**: ver tabela (hyperfine, "antes" = release do estado pós-P956;
  JSONs em `tools/perf/results/p957-canonical/`).

| Cenário | antes (ms) | depois (ms) | ratio |
|---|---|---|---|
| 01-hello | 94.95 | 93.31 | 0.983 |
| 02-lorem | 116.20 | 116.36 | 1.001 |
| 03-images | 104.20 | 97.37 | 0.935 |
| 04-math | 125.75 | 121.20 | 0.964 |
| 05-tables | 91.45 | 103.77 | 1.135 |
| 06-long | 330.47 | 357.44 | 1.082 |
| 07-context | 143.46 | 139.04 | 0.969 |

Ratio médio **1.010**. O spread não reproduz por fase (`--timings-json`,
06-long, 3 runs por binário): `layout_ms`/`render_ms`/`total` com intervalos
sobrepostos (totais 226-258 antes vs 228-248 depois) — ruído de ambiente, o
mesmo padrão de P956. A correcção é uma subtracção a menos por peça de
assembly; 04-math (o cenário que exercita assemblies): 0.964. Sem regressão
atribuível.

## 4. Achados de margem (registados, não corrigidos neste passo)

- **A linha vertical de `mat(augment: #N)` não é desenhada** pelo cristalino
  (ausente do content stream — sem operador `re`/`l S` na zona); o vanilla
  desenha-a. Pré-existente (a correcção deste passo só tocou em
  `layout_assembly`). Candidato a passo próprio.
- A lição metodológica já registada em P956/P952 reconfirmou-se: sondas com
  fontes diferentes das de produção (`with_system_fonts` vs
  `with_fonts_and_system`) produzem gids diferentes — qualquer sonda de
  glifos tem de usar o caminho de fontes de produção.
