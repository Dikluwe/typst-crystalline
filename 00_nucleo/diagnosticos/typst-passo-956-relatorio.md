# Passo 956 — Relatório PARCIAL (Fase 0 + Fase A; Fases B–D à espera da confirmação do dono)

**Data**: 2026-08-03
**Estado da árvore**: commit base `4f113026b` (P955); este passo até aqui só
documentação (ADR emendada + 6 L0s + resselo de hashes) — **zero código**.
**Gate**: Fase A.4 do passo — **PARADO antes da Fase B** para confirmação do
dono (mudança de comportamento por defeito do compilador).

---

## 1. Fase 0 — ADR-0126 emendada (inversão verboso/compacto)

Correcção do dono aplicada **como emenda datada, texto original preservado**:

- §1: o formato Passo 20 (um `BT…ET` por item, `Td`) é o modo **compacto**;
  o modo **verboso** é o modo NOVO que espelha a semântica do vanilla (`Tm`,
  `q`/`cm`/`Q` por bloco, `cs`/`scn`, `Tr` explícito). Texto original (P954)
  preservado em bloco quotado marcado "rótulos INVERTIDOS".
- Destino final registado: verboso validado **directamente contra o vanilla**
  vira o **caminho de produção padrão**; compacto passa a **flag opcional**,
  validada por decalque contra o verboso, não removida.
- §2 (complemento P955): o parágrafo que identificava o formato Passo 20 como
  "verboso" ganhou marcador de emenda inline; a conclusão de não-tensão
  mantém-se com os rótulos corrigidos.
- §5: consequências reescritas na versão corrigida (incl. custo novo: PDF por
  defeito fica MAIOR — a troca é medida na Fase D); texto original preservado.
- §4 (alternativas) sobrevive intacto à troca de rótulos — não foi tocado.
- `adr/README.md`: linha da tabela e ledger actualizados.

## 2. Fase A.1 — mapeamento (código actual + vanilla medido)

**Caminho de emissão actual (compacto)**: `build_page_stream` →
`draw_item_top` (calcula `pdf_y = page_height − pos.y`) / `draw_item_local`
(y directo, dentro de Group com `cm`) → `emit_text_pdf` / `emit_shaped_pdf` /
`emit_glyph_pdf` (`03_infra/src/export/stream.rs:190/303/467`), cada um
emitindo `{rg}BT /F{n} {size} Tf [{Tc}][{2 Tr w}] {x} {y} Td … ET`. Fill via
`fill_rg_prefix` (`r g b rg`) antes do bloco. Recursos por página construídos
em 3 sítios de `builder.rs` (`/Font << … >>` + xobj + patterns; sem
`/ColorSpace`).

**Padrão vanilla medido** (typst 0.15.1, `temp/p956/min-vanilla.pdf`,
descomprimido com `mutool clean -d`):

```pdf
q 1 0 0 -1 70.86614 763.78564 cm
/c0 cs 0 scn
BT 0 Tr /f0 11 Tf 1 0 0 -1 0 0 Tm [(…)] TJ
ET
Q
```

com `/ColorSpace << /c0 [ICCBased gray] /c1 [ICCBased sRGB] >>` nos recursos e
`/Span<</MCID n>>BDC … EMC` à volta de cada bloco (eixo acessibilidade —
**scope-out mantido**, ADR-0126 §1.3).

## 3. Fase A.2/A.3 — desenho (6 L0s emendados, secção §P956 em cada)

- **`infra/export/stream.md`** — o desenho principal: envelope verbose
  `q 1 0 0 -1 x y cm` + `/c0 cs r g b scn` + `BT 0 Tr /F{n} {size} Tf [Tc]
  1 0 0 -1 0 0 Tm […] TJ ET` + `Q`; posição toda no `cm` (sem
  `page_height − y`), o que **unifica** `draw_item_top`/`draw_item_local`;
  array TJ partilhado entre modos (P485/486/520/548 intocados); faux-bold com
  `2 Tr` no mesmo envelope; escopo = texto (`Text`/`TextShaped`/`Glyph`) —
  Line/Image/Shape/Group inalterados; regra "todo o teste declara o modo".
- **`infra/export/builder.md`** — `/ColorSpace << /c0 ICC-sRGB >>` nos 3
  pontos de recursos (só verbose); perfil ICC sRGB passa a ser sempre embutido
  em verbose; `PageContext::type1/cidfont/multifont` ganham `mode`. Nuance
  registada: vanilla usa ICC **gray** para acromáticos; v1 cristalina usa sRGB
  único (3 componentes) — mesma cor, operadores iguais; refinamento futuro.
- **`infra/export/mod.md`** — `pub enum StreamMode { Verbose, Compact }`
  (Default = Verbose); `export_pdf*` ganham `stream_mode` como último
  parâmetro — **quebra de assinatura deliberada** (precedente P113) para o
  compilador forçar modo explícito em todos os callers/testes.
- **`infra/pipeline.md`** — as 6 variantes `compile_to_pdf_bytes*` ganham
  `stream_mode` trailing; PNG/SVG intocados.
- **`shell/cli.md`** — flag `--compact` (bool; ausente → verbose);
  `RunIntent.compact: bool` (cru, L2 não importa L3).
- **`wiring.md`** — main.rs traduz bool → `StreamMode` e passa nas duas
  chamadas PDF.

**Resposta a Fase A.3**: sim, é mudança de interface pública (flag CLI nova +
comportamento por defeito novo + assinaturas L3). Documentação: docstring da
flag (cli.md), ADR-0126 §5, e Fase D.3 (docs user-facing da flag).

## 4. Estado do gate

- `crystalline-lint --fix-hashes .` → 10 ficheiros resselados (só headers;
  zero código alterado); `crystalline-lint .` → zero violations (resta o V7
  órfão pré-existente alheio).
- **Fases B–D NÃO iniciadas** — à espera da confirmação do dono sobre:
  1. nome/semântica da flag (`--compact`, bool, ausente = verbose);
  2. quebra de assinatura `export_pdf*`/`compile_to_pdf_bytes*` (parâmetro
     trailing) em vez de variantes aditivas;
  3. sRGB único na v1 (vs gray-space do vanilla) — nuance registada;
  4. escopo verbose = só emissão de texto (Line/Image/Shape/Group ficam).
