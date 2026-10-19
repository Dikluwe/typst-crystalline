# Auditoria dos 57 casos de too_many_arguments

**Resultado: 46 exceções pontuais justificadas e 11 correções internas prescritas, em 28 L0.** As 22 assinaturas públicas ficam preservadas. Nenhuma API/corpo ou anotação de supressão foi alterada nesta entrega.

Dos 11 casos de correção, C047 e C066 removem um argumento privado sem uso e passam de 9 para 8. O aviso residual é justificável depois dessa limpeza. A meta projetada é retirar nove ocorrências por reorganização interna e tratar 48 com expectativa pontual (46 diretas + 2 condicionadas), sem adotar sete como limite arquitetural universal. Isso ainda não foi materializado.

## Critérios e proveniência

Cada decisão abaixo combina a assinatura/corpo e chamadores com a cláusula vigente do owner, cujo hash anterior está em classification.json. O L0 foi atualizado para prescrever a correção ou a exceção nominal. O agrupamento de estado só foi escolhido onde existe um conjunto coeso repetido; um registro genérico usado apenas para baixar a contagem não é aceito. A necessidade de manutenção é julgamento estrutural, não prova de bug funcional ou ganho de desempenho.

No grupo de compatibilidade, eval mantém _routines/_traced/_route e realize mantém _registry por contrato público: os parâmetros não utilizados não são apresentados como necessidade do algoritmo. Nos seletores puros e leaf renderers, a injeção explícita evita inferência de estilo, fonte, posição ou estado global. Os três helpers exclusivos de testes têm justificativas separadas.

Snapshot inicial: HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC 2026-10-15T16:39:18.873477+00:00. before.json.gz contém fontes/L0 completos, status, diffstat e hashes; dependencies.json.gz conserva os quatro owners de chamadores lidos antes das propostas. A validação final reconfirmou exatamente os 57 spans Clippy.

## Índice individual

| ID | Decisão | Argumentos | Função e fonte | Prescrição L0 |
|---|---|---:|---|---|
| C003 | Exceção justificada | 8 | `eval_with_full_error` — [01_core/src/compiler/eval/mod.rs:682](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/mod.rs:682) | [00_nucleo/prompts/compiler/eval.md:397](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/eval.md:397) |
| C004 | Exceção justificada | 9 | `eval_with_full_error_and_target` — [01_core/src/compiler/eval/mod.rs:709](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/mod.rs:709) | [00_nucleo/prompts/compiler/eval.md:405](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/eval.md:405) |
| C005 | Exceção justificada | 10 | `eval_with_full_error_target_and_features` — [01_core/src/compiler/eval/mod.rs:734](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/mod.rs:734) | [00_nucleo/prompts/compiler/eval.md:413](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/eval.md:413) |
| C007 | Exceção justificada | 8 | `walk` — [01_core/src/compiler/introspect/walk.rs:91](/repos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/walk.rs:91) | [00_nucleo/prompts/compiler/introspect/walk.md:186](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/introspect/walk.md:186) |
| C008 | Exceção justificada | 10 | `walk_at` — [01_core/src/compiler/introspect/walk.rs:121](/repos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/walk.rs:121) | [00_nucleo/prompts/compiler/introspect/walk.md:194](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/introspect/walk.md:194) |
| C009 | Exceção justificada | 8 | `walk_heading` — [01_core/src/compiler/introspect/heading.rs:147](/repos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/heading.rs:147) | [00_nucleo/prompts/compiler/introspect/heading.md:120](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/introspect/heading.md:120) |
| C010 | Exceção justificada | 8 | `run` — [01_core/src/compiler/layout/convergence.rs:9](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/convergence.rs:9) | [00_nucleo/prompts/compiler/layout/convergence.md:43](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout/convergence.md:43) |
| C011 | Exceção justificada | 8 | `apply` — [01_core/src/compiler/layout/page_geometry.rs:9](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/page_geometry.rs:9) | [00_nucleo/prompts/compiler/layout/page_geometry.md:67](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout/page_geometry.md:67) |
| C014 | Corrigir | 13 | `layout_grid` — [01_core/src/compiler/layout/grid.rs:164](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/grid.rs:164) | [00_nucleo/prompts/compiler/layout/grid.md:154](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout/grid.md:154) |
| C015 | Exceção justificada | 8 | `line_cost` — [01_core/src/compiler/layout/paragraph_select.rs:94](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/paragraph_select.rs:94) | [00_nucleo/prompts/compiler/layout/paragraph_select.md:190](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout/paragraph_select.md:190) |
| C016 | Exceção justificada | 8 | `word` — [01_core/src/compiler/layout/break_selection.rs:27](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/break_selection.rs:27) | [00_nucleo/prompts/compiler/layout/break_selection.md:105](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout/break_selection.md:105) |
| C017 | Exceção justificada | 8 | `layout_with_introspector_and_metrics_and_callbacks_from` — [01_core/src/compiler/layout/mod.rs:1911](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/mod.rs:1911) | [00_nucleo/prompts/compiler/layout.md:726](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout.md:726) |
| C020 | Exceção justificada | 8 | `realize_document` — [01_core/src/compiler/realize/mod.rs:127](/repos/Antigravity/typst-crystalline/01_core/src/compiler/realize/mod.rs:127) | [00_nucleo/prompts/compiler/realize/mod.md:320](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/realize/mod.md:320) |
| C021 | Corrigir | 9 | `compose_outline_entry_indented` — [01_core/src/compiler/stdlib/structural/outline.rs:340](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/structural/outline.rs:340) | [00_nucleo/prompts/compiler/stdlib/structural/outline.md:385](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/stdlib/structural/outline.md:385) |
| C023 | Exceção justificada | 9 | `page_run_with_geometry` — [01_core/src/entities/content/construction/layout.rs:177](/repos/Antigravity/typst-crystalline/01_core/src/entities/content/construction/layout.rs:177) | [00_nucleo/prompts/entities/content/construction/layout.md:134](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/content/construction/layout.md:134) |
| C045 | Corrigir | 9 | `walk` — [03_infra/src/export/gradients/mod.rs:320](/repos/Antigravity/typst-crystalline/03_infra/src/export/gradients/mod.rs:320) | [00_nucleo/prompts/infra/export/gradients/mod.md:289](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/gradients/mod.md:289) |
| C047 | Corrigir + exceção residual | 9 | `render_text_shaped` — [03_infra/src/export/render.rs:349](/repos/Antigravity/typst-crystalline/03_infra/src/export/render.rs:349) | [00_nucleo/prompts/infra/export/render.md:376](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/render.md:376) |
| C048 | Exceção justificada | 8 | `render_glyph` — [03_infra/src/export/render.rs:388](/repos/Antigravity/typst-crystalline/03_infra/src/export/render.rs:388) | [00_nucleo/prompts/infra/export/render.md:384](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/render.md:384) |
| C049 | Exceção justificada | 10 | `render_shape` — [03_infra/src/export/render.rs:839](/repos/Antigravity/typst-crystalline/03_infra/src/export/render.rs:839) | [00_nucleo/prompts/infra/export/render.md:392](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/render.md:392) |
| C051 | Exceção justificada | 9 | `emit_shaped_pdf` — [03_infra/src/export/stream/text.rs:342](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:342) | [00_nucleo/prompts/infra/export/stream/text.md:248](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream/text.md:248) |
| C052 | Exceção justificada | 9 | `emit_shaped_pdf_deferred` — [03_infra/src/export/stream/text.rs:367](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:367) | [00_nucleo/prompts/infra/export/stream/text.md:256](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream/text.md:256) |
| C053 | Exceção justificada | 10 | `emit_shaped_pdf_inner` — [03_infra/src/export/stream/text.rs:392](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:392) | [00_nucleo/prompts/infra/export/stream/text.md:264](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream/text.md:264) |
| C054 | Exceção justificada | 9 | `emit_glyph_pdf_marked` — [03_infra/src/export/stream/text.rs:533](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:533) | [00_nucleo/prompts/infra/export/stream/text.md:272](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream/text.md:272) |
| C055 | Exceção justificada | 9 | `emit_shaped_pdf_verbose` — [03_infra/src/export/stream/text.rs:719](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:719) | [00_nucleo/prompts/infra/export/stream/text.md:280](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream/text.md:280) |
| C056 | Exceção justificada | 9 | `emit_shaped_pdf_verbose_deferred` — [03_infra/src/export/stream/text.rs:744](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:744) | [00_nucleo/prompts/infra/export/stream/text.md:288](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream/text.md:288) |
| C057 | Exceção justificada | 10 | `emit_shaped_pdf_verbose_inner` — [03_infra/src/export/stream/text.rs:769](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:769) | [00_nucleo/prompts/infra/export/stream/text.md:296](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream/text.md:296) |
| C058 | Exceção justificada | 9 | `emit_glyph_pdf_verbose_marked` — [03_infra/src/export/stream/text.rs:895](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:895) | [00_nucleo/prompts/infra/export/stream/text.md:304](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream/text.md:304) |
| C062 | Corrigir | 9 | `cidfont` — [03_infra/src/export/stream.rs:224](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream.rs:224) | [00_nucleo/prompts/infra/export/stream.md:244](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream.md:244) |
| C064 | Corrigir | 11 | `multifont` — [03_infra/src/export/stream.rs:255](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream.rs:255) | [00_nucleo/prompts/infra/export/stream.md:252](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream.md:252) |
| C065 | Exceção justificada | 9 | `render_shape` — [03_infra/src/export/svg/geometry.rs:31](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/geometry.rs:31) | [00_nucleo/prompts/infra/export/svg/geometry.md:104](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/svg/geometry.md:104) |
| C066 | Corrigir + exceção residual | 9 | `render_text_shaped` — [03_infra/src/export/svg/glyphs.rs:286](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/glyphs.rs:286) | [00_nucleo/prompts/infra/export/svg/glyphs.md:81](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/svg/glyphs.md:81) |
| C067 | Corrigir | 14 | `render_group` — [03_infra/src/export/svg/groups.rs:16](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/groups.rs:16) | [00_nucleo/prompts/infra/export/svg/groups.md:68](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/svg/groups.md:68) |
| C068 | Exceção justificada | 8 | `render_image` — [03_infra/src/export/svg/images.rs:14](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/images.rs:14) | [00_nucleo/prompts/infra/export/svg/images.md:59](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/svg/images.md:59) |
| C069 | Corrigir | 12 | `render_link` — [03_infra/src/export/svg/navigation.rs:83](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/navigation.rs:83) | [00_nucleo/prompts/infra/export/svg/navigation.md:68](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/svg/navigation.md:68) |
| C071 | Corrigir | 9 | `render_item` — [03_infra/src/export/svg.rs:233](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg.rs:233) | [00_nucleo/prompts/infra/export/svg.md:200](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/svg.md:200) |
| C074 | Exceção justificada | 8 | `export_pdf_with_font_and_document_id_and_tags_and_serialization_and_pages_and_standard` — [03_infra/src/export/mod.rs:446](/repos/Antigravity/typst-crystalline/03_infra/src/export/mod.rs:446) | [00_nucleo/prompts/infra/export/mod.md:190](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/mod.md:190) |
| C075 | Exceção justificada | 8 | `export_pdf_with_font_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard` — [03_infra/src/export/mod.rs:555](/repos/Antigravity/typst-crystalline/03_infra/src/export/mod.rs:555) | [00_nucleo/prompts/infra/export/mod.md:198](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/mod.md:198) |
| C082 | Exceção justificada | 8 | `export_pdf_multifont_with_document_id_and_tags_and_serialization_and_pages_and_standard` — [03_infra/src/export/mod.rs:672](/repos/Antigravity/typst-crystalline/03_infra/src/export/mod.rs:672) | [00_nucleo/prompts/infra/export/mod.md:206](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/mod.md:206) |
| C089 | Exceção justificada | 8 | `export_pdf_multifont_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard` — [03_infra/src/export/mod.rs:810](/repos/Antigravity/typst-crystalline/03_infra/src/export/mod.rs:810) | [00_nucleo/prompts/infra/export/mod.md:214](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/mod.md:214) |
| C100 | Exceção justificada | 10 | `render_bundle_document` — [03_infra/src/pipeline/bundle.rs:379](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/bundle.rs:379) | [00_nucleo/prompts/infra/pipeline/bundle.md:145](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/bundle.md:145) |
| C101 | Exceção justificada | 8 | `compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_serialization` — [03_infra/src/pipeline/pdf.rs:121](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:121) | [00_nucleo/prompts/infra/pipeline/pdf.md:178](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:178) |
| C102 | Corrigir | 12 | `compile_to_pdf_bytes_impl` — [03_infra/src/pipeline/pdf.rs:376](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:376) | [00_nucleo/prompts/infra/pipeline/pdf.md:186](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:186) |
| C103 | Exceção justificada | 9 | `compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override` — [03_infra/src/pipeline/pdf.rs:548](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:548) | [00_nucleo/prompts/infra/pipeline/pdf.md:194](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:194) |
| C104 | Exceção justificada | 10 | `compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override` — [03_infra/src/pipeline/pdf.rs:578](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:578) | [00_nucleo/prompts/infra/pipeline/pdf.md:202](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:202) |
| C105 | Exceção justificada | 8 | `compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_override` — [03_infra/src/pipeline/pdf.rs:614](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:614) | [00_nucleo/prompts/infra/pipeline/pdf.md:210](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:210) |
| C106 | Exceção justificada | 9 | `compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override` — [03_infra/src/pipeline/pdf.rs:641](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:641) | [00_nucleo/prompts/infra/pipeline/pdf.md:218](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:218) |
| C107 | Exceção justificada | 10 | `compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override` — [03_infra/src/pipeline/pdf.rs:669](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:669) | [00_nucleo/prompts/infra/pipeline/pdf.md:226](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:226) |
| C108 | Exceção justificada | 8 | `compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_override` — [03_infra/src/pipeline/pdf.rs:702](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:702) | [00_nucleo/prompts/infra/pipeline/pdf.md:234](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:234) |
| C109 | Exceção justificada | 9 | `compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_and_pages_override` — [03_infra/src/pipeline/pdf.rs:733](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:733) | [00_nucleo/prompts/infra/pipeline/pdf.md:242](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:242) |
| C110 | Exceção justificada | 10 | `compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_and_pages_and_standard_override` — [03_infra/src/pipeline/pdf.rs:760](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:760) | [00_nucleo/prompts/infra/pipeline/pdf.md:250](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:250) |
| C111 | Exceção justificada | 8 | `compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_override` — [03_infra/src/pipeline/pdf.rs:793](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:793) | [00_nucleo/prompts/infra/pipeline/pdf.md:258](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:258) |
| C112 | Exceção justificada | 8 | `compile_to_pdf_bytes_full_error_and_document_id_with_features_and_serialization` — [03_infra/src/pipeline/pdf.rs:878](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:878) | [00_nucleo/prompts/infra/pipeline/pdf.md:266](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:266) |
| C113 | Exceção justificada | 8 | `compile_to_pdf_bytes_oracle_with_features_and_serialization` — [03_infra/src/pipeline/pdf.rs:945](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:945) | [00_nucleo/prompts/infra/pipeline/pdf.md:274](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:274) |
| C117 | Exceção justificada | 8 | `try_shape_context` — [03_infra/src/shaper/shaping.rs:124](/repos/Antigravity/typst-crystalline/03_infra/src/shaper/shaping.rs:124) | [00_nucleo/prompts/infra/shaper/shaping.md:222](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/shaper/shaping.md:222) |
| C120 | Exceção justificada | 8 | `layout_matrix` — [01_core/src/compiler/math/layout/matrix.rs:22](/repos/Antigravity/typst-crystalline/01_core/src/compiler/math/layout/matrix.rs:22) | [00_nucleo/prompts/compiler/math/layout/matrix.md:99](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/math/layout/matrix.md:99) |
| C121 | Exceção justificada | 8 | `make_bib_dict_full` — [01_core/src/compiler/stdlib/tests/support.rs:228](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/tests/support.rs:228) | [00_nucleo/prompts/compiler/stdlib/tests/support.md:42](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/stdlib/tests/support.md:42) |
| C122 | Exceção justificada | 10 | `make_bib_dict_full_p159g` — [01_core/src/compiler/stdlib/tests/support.rs:272](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/tests/support.rs:272) | [00_nucleo/prompts/compiler/stdlib/tests/support.md:50](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/stdlib/tests/support.md:50) |

## Correções internas prescritas — 11

### C014 — `layout_grid`

Fonte: [01_core/src/compiler/layout/grid.rs:164](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/grid.rs:164). L0: [00_nucleo/prompts/compiler/layout/grid.md:154](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout/grid.md:154).

**Medição e decisão:** Grid e Table repetem a mesma projeção de doze campos posicionais para um único motor visual. Essa operação tem um conjunto coeso de tracks, conteúdo, regras e defaults; os campos devem ser nomeados no pedido privado, preservando o motor compartilhado.

**Prescrição:** GridLayoutInput emprestado, privado ao motor grid, com os doze campos nomeados e genéricos H/V preservados; o método recebe self e input. Migrar as projeções Grid/Table sem novo owner de placement.

**Aceitação:** Grid e Table mantêm tracks/gutters, header/footer, spans, estilo herdado e semântica de tabela; verificar ordem de campos e nenhuma cópia profunda nova.

Âncoras complementares: [01_core/src/compiler/layout/grid.rs:139](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/grid.rs:139), [01_core/src/compiler/layout/table.rs:107](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/table.rs:107).

### C021 — `compose_outline_entry_indented`

Fonte: [01_core/src/compiler/stdlib/structural/outline.rs:340](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/structural/outline.rs:340). L0: [00_nucleo/prompts/compiler/stdlib/structural/outline.md:385](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/stdlib/structural/outline.md:385).

**Medição e decisão:** O chamador já calcula um plano de indentação (base/hanging/auto/chave/nível/direção) antes de compor o conteúdo. Sete valores desse plano são transportados como posicionais separados; nomear o plano privado preserva presença de prefixo e torna a operação verificável.

**Prescrição:** Plano privado de indentação com prefix_width_pt, base, hanging_pt, auto, parent_key, level e rtl; compositor recebe prefix, inner e o plano. Preservar exatamente as combinações Option e fallback atuais.

**Aceitação:** Sem prefixo, auto/fixo/callback, múltiplos níveis, parent_key e RTL mantêm conteúdo, padding e markers; não reexecutar callback nem recalcular métricas.

Âncoras complementares: [01_core/src/compiler/stdlib/structural/outline.rs:450](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/structural/outline.rs:450).

### C045 — `walk`

Fonte: [03_infra/src/export/gradients/mod.rs:320](/repos/Antigravity/typst-crystalline/03_infra/src/export/gradients/mod.rs:320). L0: [00_nucleo/prompts/infra/export/gradients/mod.md:289](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/gradients/mod.md:289).

**Medição e decisão:** O walk recursivo recebe repetidamente o mesmo mapa, duas saídas e dois contadores mutáveis que formam o estado de descoberta. O L0 exige sincronismo de índices/IDs; esse estado deve ter um dono local nomeado, separado do referencial geométrico por ramo.

**Prescrição:** Collector local de descoberta possui ptr_to_idx, refs, grad_objs, next_id e counter. A recursão recebe o collector mais items/referencial/altura/placement; não trocar a travessia nem dedup.

**Aceitação:** IDs/índices sincronizados, ordem de background/body/foreground, reutilização fill/stroke, conic em grupos/tiling e referências de página preservados.

Âncoras complementares: [03_infra/src/export/gradients/mod.rs:300](/repos/Antigravity/typst-crystalline/03_infra/src/export/gradients/mod.rs:300), [03_infra/src/export/gradients/mod.rs:351](/repos/Antigravity/typst-crystalline/03_infra/src/export/gradients/mod.rs:351).

### C047 — `render_text_shaped`

Fonte: [03_infra/src/export/render.rs:349](/repos/Antigravity/typst-crystalline/03_infra/src/export/render.rs:349). L0: [00_nucleo/prompts/infra/export/render.md:376](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/render.md:376).

**Medição e decisão:** O argumento _text não é lido pelo helper raster; o dispatcher já fornece glyphs/estilo/face, e PNG não emite camada textual. Remover esse transporte morto primeiro. Os oito argumentos restantes são inputs reais do leaf renderer e admitem exceção pontual residual.

**Prescrição:** Remover somente _text do helper privado raster e de seus chamadores. Após a redução de nove para oito argumentos, autorizar expect local com motivo dos oito inputs reais de pintura.

**Aceitação:** PNG conserva glifos/offsets/avanços/alpha/transformações/face; não adicionar texto invisível nem mudar o caminho HTML/PDF. A exceção residual só vale após remover o argumento morto.

Âncoras complementares: [03_infra/src/export/render.rs:349](/repos/Antigravity/typst-crystalline/03_infra/src/export/render.rs:349).

### C062 — `cidfont`

Fonte: [03_infra/src/export/stream.rs:224](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream.rs:224). L0: [00_nucleo/prompts/infra/export/stream.md:244](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream.md:244).

**Medição e decisão:** O construtor de PageContext desmonta quatro componentes de FontScenario::Cidfont apenas para remontar esse enum; os recursos de página e o modo são outro grupo. Reutilizar a fronteira FontScenario já existente na construção interna.

**Prescrição:** Compor PageContext com FontScenario já construído e os recursos de página/modo, em construtor comum interno. Migrar cidfont e seus chamadores; manter variantes e dados por empréstimo.

**Aceitação:** Sem clone de mapas/fontes; tags Disabled, UA1/oracle false antes dos setters, Type1/CIDFont/Multifont, MCIDs e recursos ficam iguais.

Âncoras complementares: [03_infra/src/export/stream.rs:64](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream.rs:64), [03_infra/src/export/builder/fonts.rs:657](/repos/Antigravity/typst-crystalline/03_infra/src/export/builder/fonts.rs:657).

### C064 — `multifont`

Fonte: [03_infra/src/export/stream.rs:255](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream.rs:255). L0: [00_nucleo/prompts/infra/export/stream.md:252](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream.md:252).

**Medição e decisão:** O construtor Multifont repete a montagem de FontScenario com seis tabelas correlatas e os mesmos recursos/modo de PageContext. A composição deve receber o cenário já tipado, sem criar outro conjunto de arrays nem prometer validação de cardinalidade inexistente.

**Prescrição:** Usar o mesmo construtor interno que recebe FontScenario, mantendo as tabelas multifont existentes e LookupCache default. Não redesenhar o enum nem expor tipo público novo.

**Aceitação:** Preservar o alinhamento de índices fornecido pelo builder, glyph reverse/bitmap/eixos e seleção de face; nenhum novo fallback ou alegação de validação de arrays.

Âncoras complementares: [03_infra/src/export/stream.rs:85](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream.rs:85), [03_infra/src/export/builder/fonts.rs:1181](/repos/Antigravity/typst-crystalline/03_infra/src/export/builder/fonts.rs:1181).

### C066 — `render_text_shaped`

Fonte: [03_infra/src/export/svg/glyphs.rs:286](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/glyphs.rs:286). L0: [00_nucleo/prompts/infra/export/svg/glyphs.md:81](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/svg/glyphs.md:81).

**Medição e decisão:** O helper SVG de contornos ignora _text; o caminho HTML textual é escolhido antes no dispatcher e usa outro writer. Remover somente o argumento morto; os oito inputs restantes conservam face efetiva, glyph definitions e escala, com exceção pontual residual.

**Prescrição:** Remover somente _text do leaf SVG de contornos e seu argumento no dispatcher. Após passar de nove para oito inputs, autorizar expect local para os oito inputs efetivos.

**Aceitação:** O caminho HTML continua emitindo o texto original antes do leaf; o SVG conserva glyph identity/escala/offsets e definições. Não remover texto do FrameItem ou da API.

Âncoras complementares: [03_infra/src/export/svg.rs:254](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg.rs:254), [03_infra/src/export/svg/glyphs.rs:286](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/glyphs.rs:286).

### C067 — `render_group`

Fonte: [03_infra/src/export/svg/groups.rs:16](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/groups.rs:16). L0: [00_nucleo/prompts/infra/export/svg/groups.md:68](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/svg/groups.md:68).

**Medição e decisão:** Group reenvia sete componentes do mesmo contexto SVG a cada filho, além da geometria local. A sessão de definições/fontes/IDs/modo deve ser emprestada como contexto privado do hub; as dimensões internas formam Size existente.

**Prescrição:** Consumir o contexto privado de sessão definido pelo owner SVG; combinar inner_width/inner_height em Size existente. A free function conserva geometria/filhos explícitos e permanece descendente estática.

**Aceitação:** Clips/matrizes aplicados uma vez, IDs únicos compartilhados com imagens, escopo local de paint_root restaurado, destinos/fontes/HTML preservados.

Âncoras complementares: [03_infra/src/export/svg/groups.rs:16](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/groups.rs:16).

### C069 — `render_link`

Fonte: [03_infra/src/export/svg/navigation.rs:83](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/navigation.rs:83). L0: [00_nucleo/prompts/infra/export/svg/navigation.md:68](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/svg/navigation.md:68).

**Medição e decisão:** Link reenvia a mesma sessão SVG usada pelo dispatcher/grupos. Manter target, área de hit e filhos explícitos, mas transportar as definições/fontes/IDs/modo no contexto privado do hub.

**Prescrição:** Receber a sessão privada do owner SVG, mantendo target, items, pos e size separados; continuar free function de navegação.

**Aceitação:** URLs, destinos presentes/ausentes, área de hit, transforms, conteúdo e modo HTML iguais; não derivar filenames ou fazer novo layout.

Âncoras complementares: [03_infra/src/export/svg/navigation.rs:83](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/navigation.rs:83).

### C071 — `render_item`

Fonte: [03_infra/src/export/svg.rs:233](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg.rs:233). L0: [00_nucleo/prompts/infra/export/svg.md:200](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/svg.md:200).

**Medição e decisão:** O dispatcher SVG cria e retransmite a mesma sessão de fontes, definições, contador de clip, destinos, glyph context e modo HTML por vários caminhos recursivos. Deve possuir o contexto privado por exportação, conservando match exaustivo e chamadas estáticas.

**Prescrição:** Definir no hub SVG uma sessão privada emprestada contendo fonts, glyph_defs, paint_defs, clip_next_id, destinations, glyph_fonts e html_frame; render_item recebe xml/item/sessão. Grupos, links e células de paint a reutilizam com reborrow.

**Aceitação:** Uma sessão por exportação, sem global/clone de caches, sem vtable ou public API nova; manter ordem das três camadas, match exaustivo e restauração dos referenciais de paint.

Âncoras complementares: [03_infra/src/export/svg.rs:233](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg.rs:233), [03_infra/src/export/svg/paints.rs:140](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/paints.rs:140).

### C102 — `compile_to_pdf_bytes_impl`

Fonte: [03_infra/src/pipeline/pdf.rs:376](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:376). L0: [00_nucleo/prompts/infra/pipeline/pdf.md:186](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:186).

**Medição e decisão:** Núcleo privado recebe nove opções de execução/exportação mais World/Source e Timings. As mesmas sequências são montadas por wrappers ordinários e oracle; o pedido privado deve nomear os eixos preservando None até a precedência documental.

**Prescrição:** Pedido privado PdfCompileRequest nomeia full_error, document_id, stream_mode, pdf_tagged_override, features, pretty_override, pages_override, standard_override e oracle; World, Source e Timings permanecem argumentos separados. Todas as fachadas públicas ficam intactas.

**Aceitação:** Preservar None/Some, precedência caller/documento/default, canais de erro/warning, timings e rotas zero/uma/várias fontes e oracle; não ampliar o suporte do oracle.

Âncoras complementares: [03_infra/src/pipeline/pdf.rs:376](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:376), [03_infra/src/pipeline/pdf.rs:578](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:578), [03_infra/src/pipeline/pdf.rs:760](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:760).


## Exceções pontuais justificadas — 46

### C003 — `eval_with_full_error`

Fonte: [01_core/src/compiler/eval/mod.rs:682](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/mod.rs:682). L0: [00_nucleo/prompts/compiler/eval.md:397](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/eval.md:397).

**Medição e decisão:** Fachada pública de compatibilidade: registry e full_error são entradas explícitas; _routines/_traced/_route são transportados ao próximo wrapper, mas não consumidos na entrada completa atual. A exceção destes parâmetros é por compatibilidade prescrita no L0, não por necessidade operacional demonstrada.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C003: Fachada pública de compatibilidade: registry e full_error são entradas explícitas; _routines/_traced/_route são transportados ao próximo wrapper, mas não consumidos na entrada completa atual")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C004 — `eval_with_full_error_and_target`

Fonte: [01_core/src/compiler/eval/mod.rs:709](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/mod.rs:709). L0: [00_nucleo/prompts/compiler/eval.md:405](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/eval.md:405).

**Medição e decisão:** Fachada pública que acrescenta target à anterior sem ativar features. Preservar a distinção e os defaults declarados no L0, sem substituir a assinatura para atender ao limiar.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C004: Fachada pública que acrescenta target à anterior sem ativar features")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C005 — `eval_with_full_error_target_and_features`

Fonte: [01_core/src/compiler/eval/mod.rs:734](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/mod.rs:734). L0: [00_nucleo/prompts/compiler/eval.md:413](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/eval.md:413).

**Medição e decisão:** Entrada pública completa de avaliação: World/Source/registry/sink e configuração são utilizados; _routines/_traced/_route não são lidos pelo corpo atual. A cláusula Entrypoints and scope manda preservar a assinatura; esses três permanecem por compatibilidade, sem alegar que são necessários ao algoritmo.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C005: Entrada pública completa de avaliação: World/Source/registry/sink e configuração são utilizados; _routines/_traced/_route não são lidos pelo corpo atual")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C007 — `walk`

Fonte: [01_core/src/compiler/introspect/walk.rs:91](/repos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/walk.rs:91). L0: [00_nucleo/prompts/compiler/introspect/walk.md:186](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/introspect/walk.md:186).

**Medição e decisão:** Travessia pura recebe os acumuladores mutáveis da compilação e o contexto lexical por empréstimo. Contador de auto-label, Locator, tags e introspector têm papéis distintos; não há estado global nem novo dono de decisões.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C007: Travessia pura recebe os acumuladores mutáveis da compilação e o contexto lexical por empréstimo")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C008 — `walk_at`

Fonte: [01_core/src/compiler/introspect/walk.rs:121](/repos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/walk.rs:121). L0: [00_nucleo/prompts/compiler/introspect/walk.md:194](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/introspect/walk.md:194).

**Medição e decisão:** A recursão adiciona forced_location e shallow_table à mesma travessia. São dados causais por ramo, que não devem ser persistidos globalmente nem fundidos ao snapshot semântico.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C008: A recursão adiciona forced_location e shallow_table à mesma travessia")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C009 — `walk_heading`

Fonte: [01_core/src/compiler/introspect/heading.rs:147](/repos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/heading.rs:147). L0: [00_nucleo/prompts/compiler/introspect/heading.md:120](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/introspect/heading.md:120).

**Medição e decisão:** Free function descendente da feature Heading sob ADR-0109: conserva os acumuladores emprestados e a chain causal, delegando a recursão ao walk. A aridade acompanha a atomização estática, sem transferir render para entidades.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C009: Free function descendente da feature Heading sob ADR-0109: conserva os acumuladores emprestados e a chain causal, delegando a recursão ao walk")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C010 — `run`

Fonte: [01_core/src/compiler/layout/convergence.rs:9](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/convergence.rs:9). L0: [00_nucleo/prompts/compiler/layout/convergence.md:43](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout/convergence.md:43).

**Medição e decisão:** Coordenador privado recebe exatamente os inputs da fachada seeded: métricas/sizer injetados, introspector, font_size e dois transcripts selados com seed. Separar os dois protocolos e repetir a mesma seed é requisito L0.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C010: Coordenador privado recebe exatamente os inputs da fachada seeded: métricas/sizer injetados, introspector, font_size e dois transcripts selados com seed")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C011 — `apply`

Fonte: [01_core/src/compiler/layout/page_geometry.rs:9](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/page_geometry.rs:9). L0: [00_nucleo/prompts/compiler/layout/page_geometry.md:67](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout/page_geometry.md:67).

**Medição e decisão:** Aplicador puro de geometria recebe apenas os seis overrides geométricos, direção e PageConfig mutável. Receber um PageRun inteiro traria corpo/numeração alheios ao owner; presença None e flip/binding precisam continuar explícitos.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C011: Aplicador puro de geometria recebe apenas os seis overrides geométricos, direção e PageConfig mutável")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C015 — `line_cost`

Fonte: [01_core/src/compiler/layout/paragraph_select.rs:94](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/paragraph_select.rs:94). L0: [00_nucleo/prompts/compiler/layout/paragraph_select.md:190](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout/paragraph_select.md:190).

**Medição e decisão:** Função matemática pura de custo: Measure, largura, tamanho, justify, last, unbreakable, penalidade e LineCosts são os operandos efetivos da fórmula compartilhada com a hifenização. Já existe o domínio LineCosts; adicionar um contexto opaco só esconderia os demais operandos.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C015: Função matemática pura de custo: Measure, largura, tamanho, justify, last, unbreakable, penalidade e LineCosts são os operandos efetivos da fórmula compartilhada com a hifenização")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C016 — `word`

Fonte: [01_core/src/compiler/layout/break_selection.rs:27](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/break_selection.rs:27). L0: [00_nucleo/prompts/compiler/layout/break_selection.md:105](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout/break_selection.md:105).

**Medição e decisão:** Seletor puro isolado do Layouter por decisão explícita do L0: recebe palavra/largura medida, métricas/estilo, cursor/borda/origem reais e política de hifenização. Inferir esses valores novamente recriaria o defeito de origem local.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C016: Seletor puro isolado do Layouter por decisão explícita do L0: recebe palavra/largura medida, métricas/estilo, cursor/borda/origem reais e política de hifenização")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C017 — `layout_with_introspector_and_metrics_and_callbacks_from`

Fonte: [01_core/src/compiler/layout/mod.rs:1911](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/mod.rs:1911). L0: [00_nucleo/prompts/compiler/layout.md:726](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout.md:726).

**Medição e decisão:** Fachada pública seeded do layout; delega ao coordenador de convergência os oito inputs completos. O L0 manda preservar caminhos e assinaturas de callbacks e LocatorSeed.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C017: Fachada pública seeded do layout; delega ao coordenador de convergência os oito inputs completos")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C020 — `realize_document`

Fonte: [01_core/src/compiler/realize/mod.rs:127](/repos/Antigravity/typst-crystalline/01_core/src/compiler/realize/mod.rs:127). L0: [00_nucleo/prompts/compiler/realize/mod.md:320](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/realize/mod.md:320).

**Medição e decisão:** Contrato público de realização reproduzido literalmente no L0: conteúdo, recursos, source, registry, snapshot anterior e configuração são explícitos. Mesmo registry atualmente sem leitura não pode ser removido da API por este lint.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C020: Contrato público de realização reproduzido literalmente no L0: conteúdo, recursos, source, registry, snapshot anterior e configuração são explícitos")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C023 — `page_run_with_geometry`

Fonte: [01_core/src/entities/content/construction/layout.rs:177](/repos/Antigravity/typst-crystalline/01_core/src/entities/content/construction/layout.rs:177). L0: [00_nucleo/prompts/entities/content/construction/layout.md:134](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/content/construction/layout.md:134).

**Medição e decisão:** Construtor público puro com argumentos correspondentes ao PageRun: ausência e numbering Option<Option<_>> têm significado próprio. O L0 exige preservar variantes, valores e compatibilidade, sem resolver geometria no construtor.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C023: Construtor público puro com argumentos correspondentes ao PageRun: ausência e numbering Option<Option<_>> têm significado próprio")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C048 — `render_glyph`

Fonte: [03_infra/src/export/render.rs:388](/repos/Antigravity/typst-crystalline/03_infra/src/export/render.rs:388). L0: [00_nucleo/prompts/infra/export/render.md:384](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/render.md:384).

**Medição e decisão:** Leaf raster recebe alvo, estado gráfico, face/GID, posição, tamanho e cor resolvidos. Todos são utilizados na pintura do mesmo glifo; agrupar em estado global ou refazer seleção/medição violaria a fronteira do renderer.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C048: Leaf raster recebe alvo, estado gráfico, face/GID, posição, tamanho e cor resolvidos")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C049 — `render_shape`

Fonte: [03_infra/src/export/render.rs:839](/repos/Antigravity/typst-crystalline/03_infra/src/export/render.rs:839). L0: [00_nucleo/prompts/infra/export/render.md:392](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/render.md:392).

**Medição e decisão:** Leaf de Shape consome geometria resolvida, fill/stroke/fill-rule, estado e fontes para textura. Os dados correspondem à operação de pintar uma Shape; não há resolução de Length/layout ou aquisição de fontes neste owner.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C049: Leaf de Shape consome geometria resolvida, fill/stroke/fill-rule, estado e fontes para textura")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C051 — `emit_shaped_pdf`

Fonte: [03_infra/src/export/stream/text.rs:342](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:342). L0: [00_nucleo/prompts/infra/export/stream/text.md:248](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream/text.md:248).

**Medição e decisão:** Emissor PDF Compact recebe texto autoral para fallback Type1, glyphs e face efetiva separados, estilo, posição PDF, escala e cenário de recursos. A aridade preserva identidade e evita reshaping; wrapper restaura o tail normalmente.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C051: Emissor PDF Compact recebe texto autoral para fallback Type1, glyphs e face efetiva separados, estilo, posição PDF, escala e cenário de recursos")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C052 — `emit_shaped_pdf_deferred`

Fonte: [03_infra/src/export/stream/text.rs:367](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:367). L0: [00_nucleo/prompts/infra/export/stream/text.md:256](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream/text.md:256).

**Medição e decisão:** Variante Compact diferida possui os mesmos inputs causais e retorna o Q pendente para fechar ActualText antes da restauração gráfica. Esse protocolo explícito está prescrito no L0.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C052: Variante Compact diferida possui os mesmos inputs causais e retorna o Q pendente para fechar ActualText antes da restauração gráfica")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C053 — `emit_shaped_pdf_inner`

Fonte: [03_infra/src/export/stream/text.rs:392](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:392). L0: [00_nucleo/prompts/infra/export/stream/text.md:264](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream/text.md:264).

**Medição e decisão:** Núcleo Compact acrescenta restore à operação shaped; o controle distingue tail normal de diferido sem duplicar emissão. FontScenario e fonte efetiva não são substituíveis por nome, nem texto é derivável de GIDs.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C053: Núcleo Compact acrescenta restore à operação shaped; o controle distingue tail normal de diferido sem duplicar emissão")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C054 — `emit_glyph_pdf_marked`

Fonte: [03_infra/src/export/stream/text.rs:533](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:533). L0: [00_nucleo/prompts/infra/export/stream/text.md:272](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream/text.md:272).

**Medição e decisão:** Emissor de glifo marcado recebe abertura e fecho causais de ActualText independentemente. Uma ocorrência Large pode abrir na primeira peça e fechar na última; não deduzir esses limites pelo GID ou stream.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C054: Emissor de glifo marcado recebe abertura e fecho causais de ActualText independentemente")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C055 — `emit_shaped_pdf_verbose`

Fonte: [03_infra/src/export/stream/text.rs:719](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:719). L0: [00_nucleo/prompts/infra/export/stream/text.md:280](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream/text.md:280).

**Medição e decisão:** Emissor Verbose preserva a mesma identidade shaped, posição PDF, escala e recursos com envelope específico. A diferença de formato pertence a este owner e não autoriza reconstruir texto ou fonte.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C055: Emissor Verbose preserva a mesma identidade shaped, posição PDF, escala e recursos com envelope específico")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C056 — `emit_shaped_pdf_verbose_deferred`

Fonte: [03_infra/src/export/stream/text.rs:744](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:744). L0: [00_nucleo/prompts/infra/export/stream/text.md:288](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream/text.md:288).

**Medição e decisão:** Variante Verbose diferida transporta os mesmos inputs e informa restauração Q pendente. O L0 exige manter ActualText e restauração nesta ordem.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C056: Variante Verbose diferida transporta os mesmos inputs e informa restauração Q pendente")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C057 — `emit_shaped_pdf_verbose_inner`

Fonte: [03_infra/src/export/stream/text.rs:769](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:769). L0: [00_nucleo/prompts/infra/export/stream/text.md:296](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream/text.md:296).

**Medição e decisão:** Núcleo Verbose recebe restore explicitamente para compartilhar a implementação normal/diferida; os demais inputs são os operandos reais do texto e seu cenário de fontes.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C057: Núcleo Verbose recebe restore explicitamente para compartilhar a implementação normal/diferida; os demais inputs são os operandos reais do texto e seu cenário de fontes")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C058 — `emit_glyph_pdf_verbose_marked`

Fonte: [03_infra/src/export/stream/text.rs:895](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:895). L0: [00_nucleo/prompts/infra/export/stream/text.md:304](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/stream/text.md:304).

**Medição e decisão:** Variante Verbose do glifo marcado conserva abertura/fecho de ocorrência Large dentro de BT/ET e o cenário nominal/remapeado. Reduzir a aridade por inferência perderia o vínculo causal.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C058: Variante Verbose do glifo marcado conserva abertura/fecho de ocorrência Large dentro de BT/ET e o cenário nominal/remapeado")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C065 — `render_shape`

Fonte: [03_infra/src/export/svg/geometry.rs:31](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/geometry.rs:31). L0: [00_nucleo/prompts/infra/export/svg/geometry.md:104](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/svg/geometry.md:104).

**Medição e decisão:** Leaf SVG de Shape usa a geometria resolvida, fill/stroke/fill-rule e PaintDefs para referências locais. São inputs da mesma operação, e o L0 já separa a decisão de paint/layout; não há estado de sessão recursivo inteiro sendo retransmitido aqui.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C065: Leaf SVG de Shape usa a geometria resolvida, fill/stroke/fill-rule e PaintDefs para referências locais")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C068 — `render_image`

Fonte: [03_infra/src/export/svg/images.rs:14](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/images.rs:14). L0: [00_nucleo/prompts/infra/export/svg/images.md:59](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/svg/images.md:59).

**Medição e decisão:** Leaf de imagem recebe bytes, caixa física, orientação e clip já resolvidos, writer e contador de clip IDs. O contador compartilhado é uma capacidade explícita; a imagem não precisa receber a sessão inteira de fontes/destinos.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C068: Leaf de imagem recebe bytes, caixa física, orientação e clip já resolvidos, writer e contador de clip IDs")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C074 — `export_pdf_with_font_and_document_id_and_tags_and_serialization_and_pages_and_standard`

Fonte: [03_infra/src/export/mod.rs:446](/repos/Antigravity/typst-crystalline/03_infra/src/export/mod.rs:446). L0: [00_nucleo/prompts/infra/export/mod.md:190](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/mod.md:190).

**Medição e decisão:** Fachada pública single-font com oito eixos já publicados, incluindo tags, serialização, pages e standard independentes. O L0 proíbe trocar ordem/assinatura e exige encaminhamento literal ao builder.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C074: Fachada pública single-font com oito eixos já publicados, incluindo tags, serialização, pages e standard independentes")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C075 — `export_pdf_with_font_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard`

Fonte: [03_infra/src/export/mod.rs:555](/repos/Antigravity/typst-crystalline/03_infra/src/export/mod.rs:555). L0: [00_nucleo/prompts/infra/export/mod.md:198](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/mod.md:198).

**Medição e decisão:** Variante pública single-font com tempos conserva os mesmos oito inputs e devolve subset timing. É um adapter de compatibilidade, sem novo algoritmo ou resolução de opções.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C075: Variante pública single-font com tempos conserva os mesmos oito inputs e devolve subset timing")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C082 — `export_pdf_multifont_with_document_id_and_tags_and_serialization_and_pages_and_standard`

Fonte: [03_infra/src/export/mod.rs:672](/repos/Antigravity/typst-crystalline/03_infra/src/export/mod.rs:672). L0: [00_nucleo/prompts/infra/export/mod.md:206](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/mod.md:206).

**Medição e decisão:** Fachada pública multifont preserva a coleção completa com variantes/eixos e opções PDF explícitas. A assinatura está congelada pelo L0; não colapsar serialização com StreamMode ou tags com standard.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C082: Fachada pública multifont preserva a coleção completa com variantes/eixos e opções PDF explícitas")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C089 — `export_pdf_multifont_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard`

Fonte: [03_infra/src/export/mod.rs:810](/repos/Antigravity/typst-crystalline/03_infra/src/export/mod.rs:810). L0: [00_nucleo/prompts/infra/export/mod.md:214](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/export/mod.md:214).

**Medição e decisão:** Variante pública multifont com tempos preserva os mesmos eixos e o retorno de timing. A aridade é da superfície de compatibilidade; eventual deduplicação dos corpos dos exporters é outra auditoria e não exige trocar esta assinatura.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C089: Variante pública multifont com tempos preserva os mesmos eixos e o retorno de timing")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C100 — `render_bundle_document`

Fonte: [03_infra/src/pipeline/bundle.rs:379](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/bundle.rs:379). L0: [00_nucleo/prompts/infra/pipeline/bundle.md:145](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/bundle.md:145).

**Medição e decisão:** Fronteira por documento do bundle recebe corpo/formato/metadados e o snapshot global, seed/offset causais, serviços/opções e warnings. Esses dados têm escopos distintos e o L0 exige não misturar documentos ou snapshots; não há necessidade demonstrada de novo agregador.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C100: Fronteira por documento do bundle recebe corpo/formato/metadados e o snapshot global, seed/offset causais, serviços/opções e warnings")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C101 — `compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_serialization`

Fonte: [03_infra/src/pipeline/pdf.rs:121](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:121). L0: [00_nucleo/prompts/infra/pipeline/pdf.md:178](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:178).

**Medição e decisão:** Wrapper público converte PdfTags/serialization explícitos em overrides Some, preservando features, ID, modo e retorno de tempos. Essa distinção de presença é normativa no L0.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C101: Wrapper público converte PdfTags/serialization explícitos em overrides Some, preservando features, ID, modo e retorno de tempos")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C103 — `compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override`

Fonte: [03_infra/src/pipeline/pdf.rs:548](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:548). L0: [00_nucleo/prompts/infra/pipeline/pdf.md:194](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:194).

**Medição e decisão:** Wrapper público com tempos conserva pages e fornece standard None ao mais completo. A API e a diferença entre ausência e override explícito são de compatibilidade.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C103: Wrapper público com tempos conserva pages e fornece standard None ao mais completo")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C104 — `compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override`

Fonte: [03_infra/src/pipeline/pdf.rs:578](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:578). L0: [00_nucleo/prompts/infra/pipeline/pdf.md:202](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:202).

**Medição e decisão:** Wrapper público completo com tempos fornece as opções ordinárias e oracle=false ao núcleo. Preservar assinatura/retorno e construir o futuro pedido interno com campos nomeados.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C104: Wrapper público completo com tempos fornece as opções ordinárias e oracle=false ao núcleo")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C105 — `compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_override`

Fonte: [03_infra/src/pipeline/pdf.rs:614](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:614). L0: [00_nucleo/prompts/infra/pipeline/pdf.md:210](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:210).

**Medição e decisão:** Wrapper público com tempos e pretty sem pages delega pages None. A assinatura legada é explícita no transporte e deve continuar disponível.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C105: Wrapper público com tempos e pretty sem pages delega pages None")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C106 — `compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override`

Fonte: [03_infra/src/pipeline/pdf.rs:641](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:641). L0: [00_nucleo/prompts/infra/pipeline/pdf.md:218](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:218).

**Medição e decisão:** Wrapper público sem tempos com pages delega standard None. Preservar warnings/resultado e não resolver o default do standard antes do documento.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C106: Wrapper público sem tempos com pages delega standard None")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C107 — `compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override`

Fonte: [03_infra/src/pipeline/pdf.rs:669](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:669). L0: [00_nucleo/prompts/infra/pipeline/pdf.md:226](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:226).

**Medição e decisão:** Wrapper público ordinário completo sem tempos cria Timings local e passa oracle=false. A reorganização do núcleo privado não muda sua API.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C107: Wrapper público ordinário completo sem tempos cria Timings local e passa oracle=false")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C108 — `compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_override`

Fonte: [03_infra/src/pipeline/pdf.rs:702](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:702). L0: [00_nucleo/prompts/infra/pipeline/pdf.md:234](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:234).

**Medição e decisão:** Wrapper público sem tempos e sem pages fornece ausência de seleção. O eixo tagged/pretty não equivale a pages; preservar o adapter.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C108: Wrapper público sem tempos e sem pages fornece ausência de seleção")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C109 — `compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_and_pages_override`

Fonte: [03_infra/src/pipeline/pdf.rs:733](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:733). L0: [00_nucleo/prompts/infra/pipeline/pdf.md:242](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:242).

**Medição e decisão:** Wrapper público oracle com pages fornece standard None; a rota diagnóstica é independente da comum. Preservar os limites atuais do oracle em vez de expandi-los para eliminar o aviso.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C109: Wrapper público oracle com pages fornece standard None; a rota diagnóstica é independente da comum")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C110 — `compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_and_pages_and_standard_override`

Fonte: [03_infra/src/pipeline/pdf.rs:760](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:760). L0: [00_nucleo/prompts/infra/pipeline/pdf.md:250](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:250).

**Medição e decisão:** Wrapper público oracle completo fornece oracle=true e Timings local. Preservar a API e a distinção entre validações de opções e os limites da emissão oracle.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C110: Wrapper público oracle completo fornece oracle=true e Timings local")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C111 — `compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_override`

Fonte: [03_infra/src/pipeline/pdf.rs:793](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:793). L0: [00_nucleo/prompts/infra/pipeline/pdf.md:258](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:258).

**Medição e decisão:** Wrapper público oracle sem pages fornece pages None. A compatibilidade dos defaults é verificável pelo adapter existente e prescrita no L0.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C111: Wrapper público oracle sem pages fornece pages None")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C112 — `compile_to_pdf_bytes_full_error_and_document_id_with_features_and_serialization`

Fonte: [03_infra/src/pipeline/pdf.rs:878](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:878). L0: [00_nucleo/prompts/infra/pipeline/pdf.md:266](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:266).

**Medição e decisão:** Wrapper público sem tempos transforma tags/serialization explícitos em Some para o núcleo com precedência documental. Manter a conversão e a assinatura publicada.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C112: Wrapper público sem tempos transforma tags/serialization explícitos em Some para o núcleo com precedência documental")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C113 — `compile_to_pdf_bytes_oracle_with_features_and_serialization`

Fonte: [03_infra/src/pipeline/pdf.rs:945](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:945). L0: [00_nucleo/prompts/infra/pipeline/pdf.md:274](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/pipeline/pdf.md:274).

**Medição e decisão:** Wrapper público oracle com serialization conserva as conversões explícitas e a seleção da rota diagnóstica, sem antecipar resolução documental.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C113: Wrapper público oracle com serialization conserva as conversões explícitas e a seleção da rota diagnóstica, sem antecipar resolução documental")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C117 — `try_shape_context`

Fonte: [03_infra/src/shaper/shaping.rs:124](/repos/Antigravity/typst-crystalline/03_infra/src/shaper/shaping.rs:124). L0: [00_nucleo/prompts/infra/shaper/shaping.md:222](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/infra/shaper/shaping.md:222).

**Medição e decisão:** Owner do shaping recebe dois caches com lifetimes próprios, serviços e dados da ocorrência, mais BidiSpan causal e view resolvida compartilhada. O L0 proíbe redetectar direção do fragmento ou pôr contexto em TextStyle; manter a injeção explícita.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C117: Owner do shaping recebe dois caches com lifetimes próprios, serviços e dados da ocorrência, mais BidiSpan causal e view resolvida compartilhada")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C120 — `layout_matrix`

Fonte: [01_core/src/compiler/math/layout/matrix.rs:22](/repos/Antigravity/typst-crystalline/01_core/src/compiler/math/layout/matrix.rs:22). L0: [00_nucleo/prompts/compiler/math/layout/matrix.md:99](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/math/layout/matrix.md:99).

**Medição e decisão:** Compatibilidade exclusivamente cfg(test): o L0 manda conservar assinatura/corpo e defaults históricos do wrapper, enquanto produção usa layout_matrix_elem com carrier completo. Não refatorar a entrada produtiva por um aviso do helper de teste.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C120: Compatibilidade exclusivamente cfg(test): o L0 manda conservar assinatura/corpo e defaults históricos do wrapper, enquanto produção usa layout_matrix_elem com carrier completo")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C121 — `make_bib_dict_full`

Fonte: [01_core/src/compiler/stdlib/tests/support.rs:228](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/tests/support.rs:228). L0: [00_nucleo/prompts/compiler/stdlib/tests/support.md:42](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/stdlib/tests/support.md:42).

**Medição e decisão:** Factory cfg(test) constrói um único dicionário bibliográfico com os oito campos fornecidos, sem política de produção. Exceção limitada à função preserva fixtures e as expectativas independentes existentes.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C121: Factory cfg(test) constrói um único dicionário bibliográfico com os oito campos fornecidos, sem política de produção")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### C122 — `make_bib_dict_full_p159g`

Fonte: [01_core/src/compiler/stdlib/tests/support.rs:272](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/tests/support.rs:272). L0: [00_nucleo/prompts/compiler/stdlib/tests/support.md:50](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/stdlib/tests/support.md:50).

**Medição e decisão:** Factory cfg(test) da extensão bibliográfica mantém dez campos explícitos e valores literais dos testes. Um novo builder apenas para satisfazer aridade aumentaria a mecânica dos testes sem corrigir responsabilidade produtiva.

**Prescrição:** Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C122: Factory cfg(test) da extensão bibliográfica mantém dez campos explícitos e valores literais dos testes")], somente nesta função.

**Aceitação:** Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

## Política de silêncio

Aplicar futuramente `#[expect(clippy::too_many_arguments, reason = "motivo específico")]` apenas sobre a função nominal justificada. A expectativa torna a exceção revisável se o aviso desaparecer. Não aplicar allow/expect em módulo, impl ou crate; não elevar o threshold; não suprimir unfulfilled_lint_expectations ou outros diagnósticos. Não autorizar supressão dos nove casos destinados à reorganização interna. Os dois casos de argumento morto só recebem a exceção após a limpeza.

Os allow já existentes fora dos 57 avisos abertos não foram ratificados por esta auditoria.

## Validação desta entrega

Build, formato, Clippy workspace/all-targets para inventário, linhagem estrita V1/V5/V15/V26, linhagem reversa e diff-check passaram. O Clippy permanece com **174 avisos**, incluindo estes **57**; Tekt conserva **380 avisos e 1.668 notas**. Nenhum silêncio foi aplicado para reduzir artificialmente esse inventário.

As fontes Rust foram comparadas byte a byte com o snapshot após retirar apenas a linha @prompt-hash: APIs, cfg, atributos, assinaturas e corpos são idênticos. Mudaram 28 L0 e 28 selos. Pico agregado dos serviços: **4.84 GiB**, sob teto de 12 GiB, swap zero, um job Cargo, sem OOM. Não se repetiu a suíte funcional nesta etapa documental; ela será exigida na materialização das correções, e a última execução permanece evidência histórica separada.

O gate Clippy estrito da CI continua pendente. As prescrições não são implementação concluída; qualquer futura necessidade de mudar contrato público deverá ser apresentada antes pelo gate ADR-0127.
