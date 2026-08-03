# Passo 948 — Relatório (ferramenta `tools/geometry/compare.py` + primeiro relatório de 30 secções)

**Data**: 2026-08-01
**Proveniência**: `test_crystalline.pdf` e `test_vanilla.pdf` (2026-08-01 16:29, gerados
pelo dono após o commit `4e69ec1ad` — P946/P947 commitados). HEAD na execução:
`4e69ec1ad`. Ferramenta: `tools/geometry/compare.py` (Python, `lab/.venv`, pikepdf).
Relatório completo: `temp/p948-report.json`.

---

## 1. O que foi construído

`tools/geometry/compare.py` (+ `README.md`) — comparação geométrica glifo a glifo entre
dois PDFs do mesmo `.typ`:

- **Extração sem proxy**: caminhada directa do content stream (pikepdf) — operadores
  `Tj`/`TJ`/`cm`/`Tm`/`Td`/`Tf`, larguras `/W` dos CIDFonts (para posições glifo-a-glifo
  dentro de runs multi-glifo, caso vanilla), `ToUnicode` (bfchar/bfrange). Posições
  absolutas normalizadas para y-para-baixo (orientação de cada PDF detectada por
  heurística sobre a MediaBox).
- **Secções**: cabeçalhos `N.` na margem esquerda — ambos os formatos (run único à
  vanilla; dígitos glifo-a-glifo à cristalino, incluindo números de 2 dígitos sem
  re-detectar o `0` de `10` — bug apanhado e corrigido durante a Fase B).
- **Emparelhamento**: `difflib.SequenceMatcher` sobre a sequência de codepoints em
  ordem de leitura (linhas por y, x dentro da linha) — robusto a ordens de emissão
  diferentes no stream (peças de assembly) e a codepoints divergentes (extensores
  não mapeados do vanilla entram como `∅`).
- **Deltas por construção**: cada par medido relativamente à origem do seu cluster de
  linha — imune à diferença de largura de página `width: auto` (536.03 vs 500.09pt —
  artefacto de ~18pt em x, medido e eliminado por esta normalização).

## 2. Auto-testes (resultado)

- **Delta zero** (auto-comparação): 30 secções, 3025 glifos emparelhados, **0 acima do
  limiar**, todos os deltas 0.0. ✓
- **Divergência conhecida** (`temp/p944/out-p943.pdf`, render pré-P944 com os defeitos
  originais, vs vanilla): secções defeituosas sinalizadas com desvios grandes (max|dx|
  de 98–178pt nas secções 21/23/29/30) e secções sãs próximas de zero. ✓
- **Bug da origem dupla** (`oxb/oyb` calculados do lado errado — apanhado na Fase B ao
  inspeccionar pares reais): corrigido antes do primeiro relatório.

## 3. Relatório das 30 secções (par actual, limiar 0.5pt)

Triagem por **mediana de |Δx|** (a mediana é o sinal robusto; max|Δx| tem artefactos de
emparelhamento/origem de cluster — ver §4):

| Rank | Secção | med\|Δx\| (pt) | med\|Δy\| | emparelhados | s/par A+B |
|---|---|---|---|---|---|
| 1 | 22 (Delimitadores Escaláveis) | 14.10 | 0.0 | 38 | 102 |
| 2 | 21 (Matrizes Avançadas) | 10.17 | 0.0 | 89 | 25 |
| 3 | 9 (Funções por Partes) | 8.45 | 0.0 | 69 | 13 |
| 4 | 8 (Alinhamento) | 8.25 | 0.0 | 125 | 5 |
| 5 | 15 (Séries) | 7.41 | 1.26 | 106 | 33 |
| 6 | 27 (Relatividade) | 6.66 | 0.0 | 81 | 27 |
| 7 | 25 (Análise Complexa) | 6.45 | 2.78 | 105 | 85 |
| 8 | 28 (Grafos) | 5.88 | 0.0 | 79 | 23 |
| 9 | 30 (Otimização) | 5.74 | 2.70 | 122 | 166 |
| 10 | 12 (Probabilidade) | 4.68 | 0.01 | 113 | 49 |
| 11 | 18 (Aninhadas) | 4.59 | 0.0 | 66 | 14 |
| 12 | 29 (Info) | 4.57 | 2.74 | 97 | 29 |
| 13 | 26 (Quantica) | 3.95 | 0.01 | 90 | 34 |
| 14 | 20 (Refs/Numeração) | 3.36 | 0.0 | 70 | 31 |
| 15 | 16 (Fontes/Estilos) | 3.29 | 0.0 | 44 | 5 |
| 16 | 17 (Sub/Sobrescritos) | 3.13 | 0.0 | 77 | 6 |
| 17 | 4 (Cálculo) | 2.76 | 0.0 | 120 | 25 |
| 18 | 10 (Acentos) | 2.52 | 0.0 | 72 | 19 |
| 19 | 23 (Ops Personalizados) | 2.00 | 2.75 | 90 | 32 |
| 20–30 | 19, 1, 14, 13, 11, 2, 24, 7, 5, 6, 3 | ≤1.90 | ≤2.39 | — | — |

As 10 últimas (19, 1, 14, 13, 11, 2, 24, 7, 5, 6, 3) estão com mediana ≤1.9pt —
essencialmente paridade geométrica (inclui as secções 4 e 5, corrigidas em P944/P945 —
a ferramenta confirma objectivamente o fecho dessas frentes).

**Leitura da lista priorizada** (mapeamento com achados já catalogados):

- **22** — `lr(...)` literal (P944 relatório §8.3.1); conteúdo extra desloca tudo.
- **21** — linha vertical de `augment:` ausente (achado novo de P945 §9) + resíduos de
  assembly.
- **9 / 24 / 8** — texto citado em math itálico vs upright (P944 §8.3.4) e `\\` literal
  em equações multilinha (P944 §8.3.3).
- **15 / 25 / 26 / 28 / 23** — gregos literais (`zeta`/`Gamma`/`Psi`/`chi`/`omega`…,
  P944 §8.3.2) e resíduos de posicionamento de limites (scope-outs de P944 §4).
- **12 / 18** — sizing de fracções/aninhadas (scope-out de `frac.rs`, P944 §8.3.6).
- **30** — cobertura de emparelhamento baixa (160 glifos sem par do lado A: números de
  equação `(N)` — vanilla posiciona-os diferente; ver também §4).
- **27 / 29** — resíduos de limits/ops (mesma família do grupo 15/25).

## 4. Limitações conhecidas da ferramenta (para quem ler o JSON)

- **max|Δx| não é sinal limpo**: os maiores desvios individuais (ex.: 161pt na sec. 27)
  são artefactos de origem de cluster (clusters com composição diferente entre
  compiladores deslocam a origem, não o glifo) ou pares espúrios por ordem (`√↔(`).
  **A mediana é o sinal de triagem**; o `detalhe` do JSON deve ser confirmado
  manualmente antes de abrir passo.
- Glifos não mapeados do vanilla (`∅`) emparelham só por ordem.
- Secção 30 tem 160 glifos sem par do lado A (números de equação) — limitação do
  emparelhamento por sequência quando a estrutura de linhas diverge muito.

## 5. Decisões da Fase A (registadas)

1. **Extração**: pikepdf com walker próprio (não `mutool trace` — o trace reporta
   coordenadas relativas ao span para o vanilla, misturadas com absolutas; o walker dá
   controlo uniforme dos dois lados).
2. **Emparelhamento**: por sequência de leitura + difflib (não por posição absoluta) —
   a normalização por cluster de linha cobre os deslocamentos legítimos; emparelhamento
   linha-a-linha foi prototipado e **rejeitado** (frágil quando os compiladores agrupam
   as mesmas construções em números de linhas diferentes — medido na secção 30: 47 vs 7
   linhas).
3. **Formato**: tabela stdout triada + JSON completo (`--json`), limiar configurável
   (`--limiar`, default 0.5pt), filtro `--secoes`.
4. **Secções**: depende dos cabeçalhos numerados do documento de teste (a generalização
   por geometria pura fica para depois, como permitido pelo passo); fallback para
   documento inteiro como secção 0.

## 6. Próximo passo sugerido pela lista

Investigação da secção **22** (`lr` literal — já catalogado, confirmação objectiva de
que é a maior divergência medida), depois **21** (`augment:` sem linha vertical — achado
novo de P945 ainda sem passo próprio). Nada foi corrigido neste passo, conforme o
escopo — o resultado é a lista.
