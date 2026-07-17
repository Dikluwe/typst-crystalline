# Relatório — Passo 416

**Título**: Footnote nota rodapé real — body renderizado no rodapé  
**Data**: 2026-06-22  
**Tipo**: Materialização (M) — redundante; funcionalidade já implementada nos passos P304/P305.

## Resumo executivo

A sonda A.0 revelou que a funcionalidade descrita no passo 416 — renderizar o
body de `Content::Footnote` no rodapé da página — **já está implementada** no
cristalino sob os passos **P304** (footnote body renderizado no rodapé) e
**P305** (overflow multi-página + bug fixes).

O arquivo `01_core/src/engine/layout/footnote.rs` emite o marker `[N]` inline e
difere o body para `pending_footnote_bodies`. O método
`flush_pending_footnote_bodies` em `01_core/src/engine/layout/cursor.rs` faz o
layout bottom-up no rodapé, com greedy fit e defer para próxima página quando
não cabe.

## Estado do substrato verificado

| Critério | Estado |
|----------|--------|
| `Content::Footnote { body: Box<Content> }` | Existe (P295/P326) |
| `Layouter::footnote_counter` | Existe (P295) |
| `pending_footnote_bodies` | Existe (P304) |
| `flush_pending_footnote_bodies` | Implementado (P304/P305) |
| `new_page()` / `finish()` chamam flush | Sim (P304) |
| `Regions` / `clip_mask` infra | Disponível (P242/P243) |

## Testes existentes

Foram encontrados **17 testes** diretamente relacionados a footnote, dos quais
**11** cobrem P304/P305:

- `p304_footnote_body_presente_no_documento`
- `p304_footnote_body_no_rodape_y_alto`
- `p304_marker_inline_acima_do_body`
- `p304_multiplos_footnotes_bodies_empilhados`
- `p304_documento_sem_footnote_sem_impacto`
- `p304_footnote_body_complex_content_renderizado`
- `p305_overflow_body_grande_distribui_no_documento`
- `p305_overflow_multiplos_bodies_todos_preservados`
- `p305_regressao_p304_single_page_preservado`
- `p305_regressao_documento_sem_footnote_bit_exact`
- `p305_body_gigante_nao_loop_infinito`
- `p305_bug_fix_overflow_sem_overlap_no_top`

Todos passam.

## Decisão

Nenhuma alteração de código foi necessária. O passo 416 foi concluído como
**relatório epistêmico**, registando que o trabalho já tinha sido realizado
nos passos P304/P305.

## Validação

```bash
cargo test --workspace -- --skip p350c_flag_on_nao_convergente_classifica
# → todos verdes

crystalline-lint .
# → 0 drift; 2 warnings de prompt órfão pré-existentes (adr-stub-vs-fallback.md, show-regex.md)
```

## Ficheiros alterados

- `00_nucleo/materialization/typst-passo-416-relatorio.md` (novo; único artefacto deste passo)

## Nota metodológica

A spec P416 foi originalmente redigida como materialização antes da sonda A.0. A sonda revelou
que o trabalho já tinha sido feito nos Passos P304/P305. A correção de deriva converteu a spec em
documento de verificação retroativa e acrescentou este relatório. Este caso é um dos cinco
(P388, P409, P413, P416, P421) que motivam o gate "sonda A.0 antes da spec" formalizado na
ADR metodológica deste plano de correção.
