---
# P580 — Estabilização de métricas verticais em sub-layouts (registo retroactivo)

> **Passo:** 580
> **Data:** 2026-07-05
> **Foco:** Este passo foi executado sem número próprio, logo depois do P579 real (correcção de `font_size_pt` estático em `flush_line`/`new_page`/`layout_sub_frame_with_width`). A correcção de P579 introduziu uma inconsistência de 0,8pt entre o `ascender` calculado com a fonte activa (`style.size`, 11pt por defeito) e o `ascender` ainda calculado com `font_size_pt` estático (12pt) em `grid.rs`, `placement.rs`, `columns.rs`, `boxed.rs`, e no desenho de caixas de destaque em `cursor.rs`. Este documento regista o que foi feito, depois do trabalho, para o passo não ficar sem identificação no histórico.
> **Tipo:** Registo retroactivo de correcção já executada.
> **Tamanho:** M (correspondente ao trabalho já feito).
> **ADR-0108 EM VIGOR.** Nota: este passo é uma excepção à ordem normal (sonda antes de código) — o código já foi escrito antes de existir um documento próprio. O registo aqui serve para não deixar a lacuna, não para justificar repetir esta ordem no futuro.

---

## O que foi encontrado

Depois da correcção de P579 (usar `self.style.size` em vez de `self.font_size_pt` estático para o cálculo de altura de linha), os testes de integração de `Grid`, `Place`, e `Align` começaram a falhar, com um desvio consistente de exactamente 0,8pt em todas as posições verticais transladadas para o frame pai.

## Causa

A inicialização de sub-layouts de células (`layout_sub_frame_with_width`) já tinha sido corrigida em P579 para usar a fonte activa (11.0pt por defeito, ascender de 8.8pt). Mas os mecanismos de translação absoluta de items nesses sub-layouts — em `grid.rs`, `placement.rs`, `columns.rs`, `boxed.rs`, e no desenho de caixas de destaque em `cursor.rs` — continuavam a calcular o ascender de referência a partir do `font_size_pt` estático (12.0pt, ascender de 9.6pt). A diferença de 0.8pt entre os dois ascenders deslocava todos os items transladados.

## Correcção

Os cálculos de `local_start_y` e `ascender_local` em `grid.rs`, `placement.rs`, `cursor.rs`, `columns.rs`, e `boxed.rs` foram actualizados para usar `self.style.size` de forma consistente com a correcção de P579.

## Validação registada

- Snapshots P307b regenerados (`02-markup-heading.pdf`, `07-multi-feature.pdf`).
- `cargo test --workspace`: 591 testes passados, 0 falhas.
- `crystalline-lint .`: zero violações.

## Nota sobre o que falta confirmar

Este registo é posterior ao trabalho. Não há garantia de que a procura por `font_size_pt` estático, feita ficheiro a ficheiro à medida que os testes falhavam, tenha coberto todos os sítios do código onde esse valor ainda é usado de forma estática. Essa procura sistemática, mais ampla do que a reacção a testes que falham, fica para o passo seguinte (P581).

---

## Critério de fecho (aplicado retroactivamente)

- [x] Causa da inconsistência de 0,8pt identificada e documentada.
- [x] Correcção aplicada aos cinco ficheiros afectados.
- [x] Snapshots regenerados.
- [x] `cargo test --workspace` e `crystalline-lint .` confirmados no momento da execução.
- [ ] Procura sistemática por outros sítios com `font_size_pt` estático ainda por fazer — ver P581.
