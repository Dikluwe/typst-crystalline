# Relatório — Passo 427

**Título**: PDF Writer Shapes: emissão de `FrameItem::Shape` no PDF  
**Data**: 2026-06-23  
**Tipo**: Refino (S) — emissão de shapes já materializada nos passos anteriores; valor agregado na cobertura de testes E2E.

## Resumo executivo

A sonda A.0 revelou que a funcionalidade descrita no passo 427 — emitir `Rect`,
`Ellipse`, `Line` e `Polygon` de `FrameItem::Shape` no stream PDF — **já está
implementada** no cristalino sob os passos de infraestrutura de export PDF
anteriores (em especial P285/P293/P424 e o fluxo geral de `stream.rs`).

O arquivo `03_infra/src/export/stream.rs` consome `FrameItem::Shape` e emite os
operadores PDF correspondentes (`re`, `m`, `l`, `h`, `B`, `b`, `f`, `S`, `rg`,
`RG`, etc.) para todos os `ShapeKind` presentes: `Rect`, `RoundedRect`,
`Ellipse`, `Line` e `Path`.

## Estado do substrato verificado

| Critério | Estado |
|----------|--------|
| `FrameItem::Shape` em `entities/layout_types.rs` | Existe (P78) |
| `ShapeKind` (`Rect`, `RoundedRect`, `Ellipse`, `Line`, `Path`) | Existe |
| Consumo de `FrameItem::Shape` no PDF writer (`stream.rs`) | Implementado |
| Emissão de `Rect` no PDF | Operador `re` + fill/stroke |
| Emissão de `Ellipse` / `Circle` no PDF | Aproximação por 4 curvas de Bézier cúbicas |
| Emissão de `Line` no PDF | `m` + `l` + `S` |
| Emissão de `Polygon` / `Path` no PDF | `m` + `l` + `h` + `B`/`b`/`f` |
| Paint fill/stroke (`rg`/`RG`) | Aplicado em shapes |
| Transform de shape via `FrameItem::Group` | Disponível (P84.6) |

## Testes adicionados

Foram adicionados **6 testes E2E** em `03_infra/src/export/tests.rs` para
fechar a cobertura de shapes no PDF:

- `p427_pdf_rect_fill_emite_operador_re_e_fill`
- `p427_pdf_rect_stroke_emite_rg_e_s`
- `p427_pdf_rect_fill_stroke_emite_b`
- `p427_pdf_ellipse_emite_bezier_e_fill`
- `p427_pdf_line_emite_m_l_s`
- `p427_pdf_polygon_path_emite_m_l_h_b`

Todos passam.

## Decisão

Nenhuma alteração de código de produção foi necessária. O passo 427 foi
concluído como **relatório epistêmico + testes E2E**, registando que a emissão
de shapes já estava materializada e aumentando a confiança na cobertura.

A spec original propunha a opção arquitetural β de criar um módulo separado
`infra/export/shape_emit.rs`. Essa opção foi **descartada** porque o L0 vigente
`00_nucleo/prompts/infra/export/stream.md` (hash `9acca994`) determina que as
shape primitives permanecem em `stream.rs` até um passo dedicado de decomposição
se justificar. Seguindo a Regra de Ouro do projeto, não se criou novo módulo sem
atualizar o L0 correspondente.

## Validação

```bash
cargo check -p typst-infra
# → ok (28 warnings preexistentes)

cargo test -p typst-infra -- p427 --nocapture
# → 6 passed

crystalline-lint .
# → 0 errors/drift; apenas 2 warnings V7 de prompts órfãos pré-existentes
```

## Ficheiros alterados

- `00_nucleo/materialization/typst-passo-427-relatorio.md` (novo)
- `00_nucleo/materialization/typst-passo-427.md` (atualizado com nota retroativa e referência ao relatório)
- `03_infra/src/export/tests.rs` (6 testes E2E novos)

## Nota metodológica

A spec P427 foi originalmente redigida como materialização (M) antes da sonda
A.0. A sonda revelou que o trabalho já tinha sido realizado no código de
exportação PDF anterior. A correção de deriva converteu a spec em documento de
verificação retroativa, manteve a implementação no local determinado pelo L0
vigente e acrescentou este relatório e a cobertura de testes. Este caso é um dos
que motivam o gate "sonda A.0 antes da spec" formalizado na ADR-0114.
