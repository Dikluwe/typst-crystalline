# Prompt L0 — `entities/page_geometry`
Hash do Código: 2136ff3d

**Camada:** L1  
**Alvo:** `01_core/src/entities/page_geometry.rs`  
**Origem:** P1140.20–P1140.20.1  
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127  
**Estado:** especificado; implementação condicionada ao gate P1140.20.1

## Medição anterior à decisão

No vanilla ratificado `a51e02804`, paper é shorthand de width/height
(`layout/page.rs:54-103`); flipped troca eixos após resolução
(`typst-layout/src/pages/run.rs:105-110`); binding auto deriva de `text.dir`
(`run.rs:149-155`); inside/outside troca por binding e paridade
(`layout/page.rs:724-756`).

A tabela normativa está em `layout/page.rs:818-1032`, em milímetros. O
cristalino gravou `595.28`/`841.89` em `PageConfig::default`
(`layout_types.rs:694-695`): aproximações, não origem normativa. Margens usam
`rest → x/y → lado`, rejeitam lados físicos com lógicos e preservam
`two_sided` (`page.rs:613-718`). O default é
`(2.5 / 21) × min(width, height)` (`pages/run.rs:112-130`).

Proveniência: HEAD `45b547073d7686cdd5d3e3030c82de3e22ec395f`, working tree
não commitada; matriz em `diagnosticos/typst-p1140.20-propriedades-page.md`.

## Decisão

Criar `entities/page_geometry.rs`, domínio puro sem `Value`, layout, frames ou
exporters.

### `Paper`

Valor fechado com nome canónico, largura e altura normativas em milímetros,
campos privados. API: `Paper::A4`, `from_name`, `name`, `width_pt`,
`height_pt`. Tabela exaustiva de `page.rs:886-1032`, inclusive Unicode;
parsing case-insensitive como a fonte. Conversão: `pt = mm × 72 / 25.4`.
Nunca copiar valores arredondados de render/export.

### `PageBinding`

Enum `Auto | Left | Right`. Auto resolve LTR→Left e RTL→Right. `swap` recebe
número físico one-based não zero: Left troca em pares; Right em ímpares.

### `PageMarginSpec`

Estender com `two_sided: Option<bool>`: None preserva modo lateral;
Some(false) torna slots físicos left/right; Some(true), lógicos
inside/outside. Os quatro `Option<f64>` continuam. Parse produz delta com
precedência específico > eixo > rest; fold aplica só componentes/modo
presentes. Não armazenar seis lados nem resolver paridade no eval.

`PageMargins` é snapshot físico. Resolve especificação folded, margem auto,
binding e número físico; só então troca lados. Auto permanece marcado por lado.

## Defaults e ordem

1. Default: A4, flipped false, binding Auto, margem auto.
2. Paper fornece só eixos sem override no mesmo delta; Auto explícito vence.
3. Flip troca o par resolvido.
4. Margem auto usa menor eixo finito orientado; um infinito usa o outro; dois
   infinitos usam `Paper::A4.width_pt()`.
5. Binding auto usa direção raiz efetiva.
6. Inside/outside resolve ao fechar cada página, pois paridade varia no run.

## Incompletude

Bleed/fill/layers, running matter e supplement pertencem a P1140.20.2–.4.
`page` público permanece ausente até P1140.21.

## Aceitação

Tabela completa e inválido; zero A4 literal em pontos; precedência e flip;
binding LTR/RTL/paridade; fold parcial, auto por lado e conflito; pureza L1.
