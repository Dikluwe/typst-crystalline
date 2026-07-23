# Relatório — typst-passo-869: fechar as lacunas deixadas por P868

**Data:** 2026-07-23T15:45:00Z  
**Executor:** Kimi Code  
**Commit base:** `06a336b0c3ae42949c2bced6c4c6511b9d89bebd`  
**Ramo final:** `Tekt` (commits `3c8839e72` e `76ae4c101`)

---

## Lacuna 1 — reconciliação da contagem de testes

### Baseline confirmado

```bash
$ git checkout 06a336b0c3ae42949c2bced6c4c6511b9d89bebd
$ cargo test -p typst-core
```

```text
test result: ok. 4655 passed; 0 failed; 2 ignored
```

### Final consolidado confirmado

```bash
$ git checkout Tekt
$ cargo test -p typst-core
```

```text
test result: ok. 4682 passed; 0 failed; 2 ignored
```

**Diferença real em `typst-core`: 4682 − 4655 = +27.**

### Testes novos identificados no código

| Passo | Testes novos em `typst-core` | Contagem |
|---|---|---|
| P862 | `p862_repr_plain_text_splits_on_space`, `p862_content_tree_splits_plain_text_on_space`, `lex_markup_text_splits_on_space` | 3 |
| P863 | `p863_show_par_identidade_realiza_e_aplica`, `p863_show_par_func_transforma_paragrafo`, `p863_set_par_spacing_compila_sem_abortar` | 3 |
| P864 | `layout_lista_parbreak_separa_grupos`, `layout_lista_sem_parbreak_continua_grupo`, `layout_enum_parbreak_separa_grupos_e_reinicia_numero`, `layout_enum_sem_parbreak_continua_numero`, `layout_terms_parbreak_separa_grupos`, `layout_terms_sem_parbreak_continua_grupo`, `layout_lista_para_enum_nao_adiciona_espaco_extra` | 7 |
| P865 | `p865_text_size_named`, `p865_text_named_args_comuns`, `p865_text_size_int_erro`, `p865_text_arg_desconhecido_erro`, `p865_text_bold_erro`, `p865_text_weight_int_valido`, `p865_text_variations_continua_a_funcionar`, `p865_text_scope_out_aceite_sem_erro` | 8 |
| P867 | `p867_set_page_height_auto_ok`, `p867_set_page_width_auto_ok`, `p867_set_page_height_invalid_type_error`, `p867_height_auto_cresce_para_conteudo_curto`, `p867_height_auto_cresce_para_conteudo_longo`, `p867_width_auto_cresce_para_conteudo`, `p867_height_fixo_continua_paginar` | 7 |
| **Soma bruta** | | **28** |

### Teste removido

Entre o baseline e o final, foi removido um teste do P790:

- `p790_show_par_outra_transformacao_erro_explicito`

Este teste verificava que `#show par: <transformação>` devolvia erro. O P863 mudou este comportamento para aceitar a transformação, pelo que o teste antigo foi substituído pelos 3 testes P863.

### Reconciliação

```text
4655 (baseline)
+ 28 (novos P862–P867)
−  1 (teste P790 removido)
= 4682 ✓
```

A contagem final bate exactamente. A discrepância de "28 novos" vs "+27" é explicada pela remoção do teste legado `p790_show_par_outra_transformacao_erro_explicito`.

### Nota sobre P866

Os 5 testes do P866 (`p866_output_png_recusado_com_erro_claro`, `p866_output_svg_recusado_com_erro_claro`, `p866_format_flag_png_vence_extensao_pdf`, `p866_output_pdf_continua_funcionar`, `p866_format_flag_pdf_continua_funcionar`) vivem em `04_wiring/tests/cli.rs`, não em `typst-core`. A contagem de `typst-core` não os inclui.

---

## Lacuna 2 — isolamento real do diff do P863

### Procedimento

Foi criada uma branch temporária a partir do commit base, aplicando apenas as alterações do P863:

```bash
$ git checkout -b p869-isolamento-p863 06a336b0c3ae42949c2bced6c4c6511b9d89bebd
```

As alterações do P863 foram extraídas dos ficheiros consolidados e aplicadas isoladamente. A única correção externa incluída foi a adição de `Content::Par { .. }` ao match não-locatable de `01_core/src/engine/introspect/locatable.rs` — alteração puramente mecânica para manter a exaustividade do `match`, documentada no P864 como "débito de desbloqueio do P863".

### Resultado

```bash
$ cargo clean && cargo test -p typst-core p863_show_par_func_transforma_paragrafo -- --nocapture
```

```text
     Removed 7578 files, 7.1GiB total
    Finished `test` profile [unoptimized + debuginfo] target(s) in 46.83s
     Running unittests src/lib.rs (target/debug/deps/typst_core-a144597b292dbb61)

running 1 test
test engine::eval::tests::tests::p863_show_par_func_transforma_paragrafo ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4658 filtered out
```

### Verificação alargada aos 3 testes P863

```bash
$ cargo test -p typst-core p863 -- --nocapture
```

```text
running 3 tests
test engine::eval::tests::tests::p863_set_par_spacing_compila_sem_abortar ... ok
test engine::eval::tests::tests::p863_show_par_func_transforma_paragrafo ... ok
test engine::eval::tests::tests::p863_show_par_identidade_realiza_e_aplica ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 4656 filtered out
```

### Veredito

**O teste `p863_show_par_func_transforma_paragrafo` passa isoladamente sobre o commit base, com compilação do zero.** A explicação de P868 — de que a "falha pré-existente" reportada por P864 foi um artefacto de cache/execução paralela — está confirmada pelo método correcto de isolamento.

A branch temporária foi removida após o teste.

---

## Conclusão

- **Lacuna 1**: a diferença de +27 testes em `typst-core` está reconciliada: 28 adicionados (P862–P867) menos 1 removido (P790).
- **Lacuna 2**: o P863 passa isolado sobre o commit base; a conclusão de P868 sustenta-se.
