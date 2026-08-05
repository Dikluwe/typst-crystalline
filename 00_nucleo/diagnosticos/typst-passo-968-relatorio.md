# Relatório — Passo 968 (74% do texto com `Tr 2`/faux-bold indevido)

**Data:** 2026-08-05
**Proveniência das medições**: HEAD no início do passo = `bba00cc6d`
(P967). Alterações de P968 **não commitadas** durante as medições —
ficheiros alterados (`git diff HEAD --stat`):
`00_nucleo/prompts/entities/layout_types.md`,
`01_core/src/entities/layout_types.rs`. Binários: debug construído ~12:10,
release reconstruído ~12:15 (o release anterior — binário de P967 — foi
copiado para `temp/p968/typst-antes` antes do rebuild).

## Fase A — causa confirmada: sobre-disparo do gate de faux-bold

**Reprodução do achado** (mesmo método da auditoria, `mutool clean -d` +
contagem de blocos `BT…ET` com operador `2 Tr`): **1541 de 2074 blocos
(74.30%)** com `Tr 2` — bate com os 1541/74% da auditoria.

As quatro confirmações pedidas pelo passo:

1. **Negrito genuíno no documento: 0%.** Nenhum `*...*`/`#strong` no fonte
   (a única ocorrência de `*` é `x^*`, expoente na secção 28). 74% de
   faux-bold contra 0% de negrito real ⇒ sobre-disparo, não caso legítimo.
2. **Condição exacta** (`TextStyle::faux_bold_stroke_pt`,
   `01_core/src/entities/layout_types.rs:279`, P139):
   `((weight − 400)/300).max(0) × size × k` — dispara para **qualquer**
   weight > 400. P944 fixa `weight: Some(450)` no `math_style` de toda a
   equação (`engine/layout/equation.rs:90`) — paridade de língua correcta
   com o show_set do vanilla (`TextElem::weight = 450`,
   `lab/typst-original/crates/typst-library/src/math/equation.rs:197`).
   Os dois `w` medidos pela auditoria batem exactamente: 450 → 50/300 ×
   size × 0.04 = **0.073pt a 11pt** e **0.051pt a 7.7pt** (script). Todo o
   texto math levava contorno — daí os 74% (o documento é maioritariamente
   math) e a impressão de "texto meio negrito".
3. **Os 26% em `Tr 0`**: prosa em Libertinus Serif (weight `None`→400 →
   stroke 0) mais blocos math sem operador `Tr` explícito (herança de
   estado gráfico — a contagem por bloco, minha e da auditoria, atribui
   esses blocos ao modo herdado; a conclusão não muda).
4. **Mecanismo do vanilla**: não existe faux-bold em lado nenhum do código
   vanilla (zero ocorrências de faux/embolden/synthetic-bold nos crates) —
   a selecção de variante real via fontdb satisfaz todos os pesos, sempre
   `Tr 0`. O weight 450 selecciona NewCMMath-Book, sem contorno.

## Fase B — limiar de intenção bold (weight ≥ 600)

**L0 primeiro**: `prompts/entities/layout_types.md` §P968. ADR-0127:
correcção interna de paridade (não é contrato público nem comportamento por
defeito novo do produto) — fluxo contínuo, sem paragem de gate.

**Testes** (`layout_types.rs`, RED confirmado nos dois primeiros):
- `…_450_zero_passo_968` — Book do math não é intenção de negrito → 0
  (falhava: 0.0733).
- `…_500_zero_passo_968` — medium → 0 (falhava: 0.1467).
- `…_600_positivo_passo_968` — fronteira semibold/bold → 0.293 @ 11pt.
- Guardas P139 inalteradas e verdes: 400→0, 100→0, None→0, 700→0.44,
  escala com size.

**Implementação**: `if w < 600 { return 0.0; }` no início de
`faux_bold_stroke_pt`. Fórmula de P139 inalterada acima do limiar; pontos
de emissão (`stream.rs` Type1 + envelope verbose P956) consomem o mesmo
helper, sem alteração. Suíte completa: **5723 testes, 0 falhas** (+3).

## Fase C — Revalidação

1. **Recontagem no documento de 30 secções**: `Tr 2` **1541 → 0 (0.0%)** —
   bate com o conteúdo genuinamente negrito (0%).
2. **Guarda do negrito genuíno** (`temp/p968/bold.typ`: `*negrito de
   verdade* e texto normal $ x+y=3 $ fim`): o cristalino compõe o negrito
   com a **variante bold real** (`LibertinusSerif-Bold`, F2) e `Tr 0` —
   exactamente como o vanilla (que também usa a bold real, `Tr 0`). O
   faux-bold legítimo (weight ≥ 600 sem variante bold carregada) fica
   mantido — divergência de mecânica consciente registada em P956, não
   removida às cegas.
3. **Confirmação visual**: zoom 600 DPI na secção 8 (`temp/p968/zoom-antes`
   vs `zoom-depois`) — o traço engrossado de "dado" e dos glifos math é
   visível no antes e ausente no depois; crop a 100 DPI do depois é
   indistinguível do vanilla em peso de traço.
4. **Posições inalteradas**: `compare.py` no documento completo dá
   **1735/2722** glifos acima do limiar antes e depois do fix — idêntico
   (o fix remove contorno, não move glifos).
5. **Benchmark** (`tools/perf/benchmark-p968-canonical.py`, hyperfine,
   7 cenários, `tools/perf/results/p968-canonical/`):
   01-hello 0.998 · 02-lorem 1.010 · 03-images 0.997 · 04-math 1.008 ·
   05-tables 1.018 · 06-long 0.996 · 07-context 1.006 — rácio médio
   **1.005**, zero regressão (a alteração é um early-return num helper de
   render; o trabalho de layout é inalterado; o spread ±1.8% é o ruído
   habitual da máquina).
6. **Linter**: resselo de `layout_types.rs` (→ `1896783e`);
   `crystalline-lint .` → **0 violations** (permanece só o V7 órfão
   pré-existente de `package_version_resolution.md`, alheio a este passo).

## Resultado

- Causa exacta confirmada em código: gate de P139 (`weight > 400`) a
  disparar para o weight 450 que P944 (correctamente, por paridade com o
  show_set do vanilla) fixa em toda a matemática.
- Proporção de `Tr 2` reduzida de 74.30% para **0.0%**, a bater com o
  negrito genuíno do documento (0%).
- Faux-bold legítimo mantido para weight ≥ 600 sem variante real; quando a
  variante bold real existe, ela é seleccionada (medido com Libertinus
  Bold) — nenhum dos dois caminhos regrediu.
- Benchmark sem regressão; suíte verde; linter limpo.
