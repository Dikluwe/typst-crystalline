# Relatório — Passo 994: `$...$` aninhado em funções de layout (Opção β, `layout_external`)

**Estado do código das medições**: HEAD `cd94b5338` (P993) + alterações deste
passo.
**Commit final**: registado no fim deste ficheiro.
**Gate ADR-0127**: desenho (Opção β) aprovado pelo dono no passo
(`typst-passo-994.md`); a conclusão da Fase A (sem mudança de contrato
público — fluxo contínuo) foi apresentada e **aprovada pelo dono**
(2026-08-11, AskUserQuestion) antes da Fase B.

## Fase A — respostas às 3 perguntas (registadas no L0 antes de código)

1. `Layouter` é genérico — `self.metrics` coagido a `&dyn FontMetrics`
   (impl existente, `metrics.rs:369`); geometria real, não `FixedMetrics`.
2. `layout_sub_frame` devolve `(height, items, …)`; região ilimitada
   (precedente `measure_content_real`).
3. `FrameItem::Group` chega como está; `MathBox.items` já o aceita — sem
   mudança de contrato.

## Fase B — dois agentes (per P898, como o passo manda)

**Agente A**: 6 testes em `p994_tests` (`engine/layout/tests.rs`) — 5 RED com
a assinatura exacta do bug (texto literal/`^`/sem caixa/"center" vazado) + 1
guarda de não-regressão verde.

**Agente B**: implementação em 3 rondas, com 2 bloqueios reportados e 3
desvios medidos — todos registados no L0 (`_comum.md` §P994 + 3 adendas):

- **Bloqueio 1 (visibilidade)**: `layout_sub_frame`/`SubLayoutRegion`
  alargados ao mínimo que alcança `engine::math` (`pub(in crate::engine)`;
  `pub(crate)` disparava `private_interfaces`).
- **Bloqueio 2 (Fase A errada no consumidor final)**: `equation.rs:241`
  descartava `FrameItem::Group` — braços novos `Group`/`Shape` na integração
  da equação na página.
- **Desvio 1 (deny-list, não delegação cega)**: refutado por medição —
  markup `Content::Text` em math ficava centrado no eixo (−4.5pt);
  `needs_external_layout` sobe só `Equation`/`Boxed`/`Align`/`Pad`/`Block`
  (recursivo via `Sequence`/`Styled`), mais próximo do vanilla
  (`ir/resolve.rs:127-146`).
- **Desvio 2 (fix de baseline, revisão do orquestrador)**: âncora = baseline
  declarada do frame (o vanilla só usa `height/2+axis` quando o frame NÃO
  declara baseline — `math/mod.rs:596`), via `scan_external_verticals`;
  embutimento **achatado** em vez de `Group` (a via PDF de `Group`+texto tem
  incoerência de convenção Y pré-existente, provada fora de math —
  `box(clip:)` perde texto; **candidata a passo próprio**).
- **Desvio 3**: typo de codepoint do Agente A (U+1D45F=𝑟 → U+1D44F=𝑏),
  verificado via unicodedata.
- **Causa secundária (align)**: `eval_math_arg_value` — ident que resolve no
  scope para valor não-Content/Func/Module desvia para `eval_math_callee`
  (`center` volta a ser alinhamento; bónus: `delim: none`, `auto`, cores em
  args math). Bónus medido: também corrige `auto`/`none` como args.

**Revisão do orquestrador**: caso composto (`text()`∋`box()`∋`$v^2$`) falhou
visualmente na 1ª ronda (conteúdo fora da caixa) → fix de baseline acima; na
2ª ronda, correto.

## Fase C — Revalidação

- **6 casos do diagnóstico** (`p993-minimos/`, renders + `pdftotext -bbox`):
  e1 **idêntico ao vanilla ao centésimo de ponto** (𝑏 yMin 26.108 vs
  26.1065); e3 com caixas + letras itálicas dentro, mesma baseline; e2/e4 com
  sobescritos reais; e4 sem vazamento de "center".
- **Documento estendido** (`.typ/typst-math-extended-test.typ`): secção 35
  agora idêntica ao vanilla (tamanhos + itálico + sobrescritos). O texto
  disperso na secção 36 (caixas com `rotate`/`scale`) **é pré-existente** —
  verificado com build do HEAD pré-P994 (`e3513451e`, worktree): já lá
  estava e não é do escopo deste passo.
- **Suíte**: **5823 testes verdes, 0 falhas** (4954+787+41+2+37+2).
- **Lint**: 0 violations (só V7 órfão pré-existente).
- **Benchmark** (`benchmark-p994-canonical.py`, antes = release pré-P994):
  ratios 0.991–1.005, **média 0.998 — sem regressão**.

## Achados registados para passos futuros

1. **Exportador PDF de `FrameItem::Group` com filhos de texto é incoerente**
   (o `cm` não inverte Y com matriz identidade; erro medido = 2×y_local;
   o raster assume o contrário) — pré-existente, afeta `box(clip:)` fora de
   math. Provado por medição no content stream.
2. Altura do box inline (`boxed.rs:186-196`, inset só horizontal) — igual
   fora de math; débito de `boxed.rs`.
3. Secção 36 do estendido (transformações `rotate`/`scale` em math) —
   posicionamento do conteúdo, pré-existente.
4. `Styled`/`Sequence` de markup puro sem equação dentro em math (ex.:
   `#text(fill: red)[nota]`) mantém o caminho de texto fundido — candidato
   se aparecer em documento real.

**Commit final**: `git log` após este relatório (mensagem `fix(math):
math aninhado em funções de layout — layout_external (Opção β) — Passo 994`).
