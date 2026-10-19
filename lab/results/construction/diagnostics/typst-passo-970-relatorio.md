# Relatório — Passo 970 (índice de raiz com tamanho e posição errados)

**Data:** 2026-08-05
**Proveniência**: HEAD no início = `2337dc435` (P968). Parte 1 (tamanho)
implementada no working tree e validada antes do commit; Parte 2 (posição)
**não implementada — parada no gate ADR-0127** (requer campos novos em
`MathConstants`, ver abaixo). Binários: debug ~14:42; release "antes" =
binário de P968 copiado para `temp/p970/typst-antes` antes do rebuild.

## Fase A — fórmulas reais do vanilla confirmadas

1. **Tamanho**: o vanilla fixa o índice em `MathSize::ScriptScript`
   absoluto (`resolve.rs:1235-1239` — "the index in scriptscript size and
   cramped style") e o factor é **absoluto, não cumulativo**
   (`TextSize::resolve`,
   `lab/typst-original/crates/typst-library/src/text/mod.rs:1139-1152`:
   `ScriptScript ⇒ ×script_script_percent` sobre o tamanho de texto
   declarado). Medido na fonte (fontTools, NewCMMath-Book):
   `ScriptPercentScaleDown=70`, `ScriptScriptPercentScaleDown=50` ⇒ índice
   a 5.5pt sobre 11pt — bate com a auditoria. O cristalino usava
   ×`script_percent` (0.7 ⇒ 7.7pt — também bate com a auditoria): P915
   fixou o campo `math_size` e P945 deixou o factor em scope-out explícito
   ("fica para passo dedicado").
2. **Posição** (`radical.rs:86-96,113-114`, lido e confirmado):
   `shift_up = RadicalDegreeBottomRaisePercent × (inner_ascent − descent)
   + index.descent`; `sqrt_offset = KernBefore + index.width + KernAfter`
   (o √ e o radicando deslocam-se à direita); `index_x = −min(sqrt_offset,
   0) + KernBefore`; baseline do índice em `−shift_up`. Valores da fonte:
   `RadicalKernBeforeDegree=278du`, `RadicalKernAfterDegree=−556du`,
   `RadicalDegreeBottomRaisePercent=60%`, `RadicalExtraAscender=48du`.
   Nenhuma destas constantes existe em `MathConstants`.
3. **Divergência cristalina actual** (`root.rs:120-139`): tamanho ×0.7
   (corrigido na Parte 1); posição `idx_x = 20% da largura do √`,
   `idx_dy = −total_ascent` (topo do composto) — daí o "grande e quase no
   topo, flutuando".

## Fase B — Parte 1 implementada (tamanho), Parte 2 no gate

**Parte 1 (fluxo contínuo ADR-0127 — correcção de fórmula interna sem mudar
assinaturas)**: o factor do índice passa a ser o ScriptScript absoluto por
nível corrente — `Display|Text → ×sscript`, `Script → ×sscript/script`,
`ScriptScript → ×1.0` (mesma forma dos helpers P945/P952). L0:
`root.md` §P970 (partes 1 e 2, com a divisão declarada explicitamente, per
a regra de divisão entre passos).

**Testes** (`p970_tests`, RED confirmado nos 4 — obtinham 8.400/8.400/
5.880/4.200): Text 12pt → 6.0pt; Display 12pt → 6.0pt; Script (8.4pt) →
6.0pt (propriedade absoluta); ScriptScript (6.0pt) → 6.0pt (trava).
Suite completa: **5727 testes, 0 falhas** (+4).

**Parte 2 (posição) — PARADA NO GATE**: requer 4 campos novos em
`MathConstants` (`radical_kern_before_degree`,
`radical_kern_after_degree`, `radical_degree_bottom_raise_percent`,
`radical_extra_ascender`) + leitura em `03_infra/font_metrics.rs` —
mudança de contrato público (ADR-0127 ponto 1; precedente P959). A fórmula
completa e os valores medidos estão registados em `root.md` §P970 Parte 2
para não reler a fonte. A posição actual (índice no topo) permanece até à
confirmação.

## Fase C — Revalidação (do que foi implementado)

- **End-to-end** (documento de 30 secções, `root(3, x)` da secção 1):
  índice "3" medido a **h=5.50pt — igual ao vanilla** (antes: 7.70pt;
  vanilla: 5.50pt). Distância ao topo do √: 3.96pt (antes: 2.2pt — melhorou
  porque o glifo é menor; o valor vanilla 8.4pt depende da Parte 2).
- **compare.py** secções 1/13/14: **146/242 — idêntico antes e depois**
  (a correcção muda o tamanho do glifo, não origens; a paridade posicional
  fica intacta).
- **Benchmark** (`benchmark-p970-canonical.py`, 7 cenários): 01-hello 1.013
  · 02-lorem 0.994 · 03-images 1.016 · 04-math 1.000 · 05-tables 1.020 ·
  06-long 0.988 · 07-context 0.964 — **rácio médio 0.999, zero regressão**.
- **Linter**: resselo de `root.rs` (→ `aa820449`);
  `crystalline-lint .` → 0 violations (só o V7 órfão pré-existente).

## Resultado

- Tamanho do índice corrigido para o ScriptScript absoluto do vanilla
  (5.5pt = vanilla, medido end-to-end) — metade do achado 9.1 fechada.
- Posição (Parte 2): fórmula e valores medidos e registados no L0;
  implementação pendente da confirmação do gate (4 campos em
  `MathConstants`).
- Benchmark sem regressão; suíte verde; linter limpo.

---

## Adendo — Parte 2 implementada (gate confirmado pelo dono em 2026-08-05)

**Proveniência**: HEAD no início = `225b96a42` (P969). Benchmark:
`temp/p970b/typst-antes` = release de P970-parte-1; resultados em
`tools/perf/results/p970b-canonical/`.

**Implementado** (tudo o que a Fase A mediu, `root.md` §P970 Parte 2):

- 4 campos novos em `MathConstants` (`radical_kern_before_degree`,
  `radical_kern_after_degree`, `radical_degree_bottom_raise_percent`,
  `radical_extra_ascender`) + leitura real em `font_metrics.rs`
  (`infra/font_metrics.md` §P970; percentual lido como i16 ÷100).
- `root.rs`: `sqrt_offset = kern_before + idx.width + kern_after`; √,
  overline e radicando deslocam-se `max(sqrt_offset, 0)`; índice em
  `−min(sqrt_offset,0) + kern_before` na horizontal e baseline a
  `−shift_up` (`shift_up = raise × (inner_ascent − descent_surd) +
  idx.descent`); ascent do composto = `max(inner_ascent, shift_up +
  idx.ascent)`. Sem índice: inalterado (guarda em teste).
- Oráculo (P969) ganhou `radical_degree_shift_up` e
  `radical_sqrt_offset` — os 4 testes novos (`p970b_tests`) referenciam o
  oráculo, não aritmética solta. RED confirmado nos 4 (valores antigos:
  x=1.44, y=−9.912, radicando sem deslocamento, ascent 9.912); guarda
  sem-índice verde desde o início. Suite: **5731+ testes verdes** no
  momento do GREEN (4892 core).

**Validação end-to-end** (`root(3, x)` da secção 1, doc de 30 secções):
índice a **8.93pt do topo do √** (vanilla: 8.37pt; antes: 2.2pt — "quase
no topo, flutuando"), x a 3.37pt da aresta esquerda do √ (= kern_before,
fórmula). Residual sub-ponto (0.56pt) atribuído ao ajuste de gap do
TeXbook p443 item 11 (`radical.rs:76`) não portado — registado no L0.

**Achado novo registado** (explica a diferença de altura do surd): o
cristalino usa `radical_vertical_gap` (50du) mesmo em Display; o vanilla
usa `radical_display_style_vertical_gap` (148du) em Display
(`radical.rs:32-36`) — daí o √ do vanilla chegar ~3pt mais alto. Requer
campo novo em `MathConstants` ⇒ gate de um passo futuro. Não é o achado
9.1 (índice), mas condiciona a posição absoluta do índice contra o
vanilla.

**compare.py** secções 1/13/14: 146→149 acima do limiar (sec 1: 37→40,
med|dx| 0.275→1.147) — **agravamento esperado e explicado**: o índice
agora senta-se correctamente *relativo ao surd* (a métrica da auditoria),
mas o surd cristalino é mais curto que o do vanilla (o achado novo acima),
logo em posição absoluta de página o índice fica mais longe do vanilla
do que quando estava "flutuando" por acidente perto da posição do
vanilla. A evidência fiável é a medição directa (8.93 vs 8.37 do topo do
√); compare.py mede distâncias absolutas com emparelhamento heurístico
(lição P949).

**Benchmark** (7 cenários): 01-hello 1.020 · 02-lorem 1.002 · 03-images
1.024 · 04-math 1.021 · 05-tables 0.999 · 06-long 1.000 · 07-context
0.992 — rácio médio **1.008**, dentro do ruído habitual da máquina
(spread ±2.4%; a alteração toca só o caminho de radicais com índice).
