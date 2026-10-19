# Correções e decisões V19

Medição: HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; hashes, UTC e diffstat em before.json.gz.

| Ponto | Função | Decisão | Grupos V19 novos |
|---|---|---|---:|
| I1 | [eval_with_full_error_target_and_features](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/mod.rs:956) | fechar o domínio nominal preservando as consequências atuais | [38] |
| I2 | [needs_fresh_index](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/realized.rs:16) | fechar o domínio nominal preservando as consequências atuais | [6, 114] |
| I3 | [prepared_locations](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/realized.rs:155) | fechar o domínio nominal preservando as consequências atuais | [118] |
| I4 | [block_equation_origin](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/realized.rs:314) | fechar o domínio nominal preservando as consequências atuais | [115] |
| I5 | [textual_product](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/realized.rs:435) | fechar o domínio nominal preservando as consequências atuais | [5, 109] |
| I6 | [extract_payload](/repos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/extract_payload.rs:97) | normalizar Source e fechar ausência de payload por nomes | [99] |

Os oito grupos partilham uma consequência ou efeito atual. V19 permanece informativo e prescrito em seus owners, sem supressão. Isso não comprova execução de todas as alternativas nos testes.

I1 fecha T027; I3 fecha o ponto associado a T158/T160. I2, I4 e I5 fecham fallbacks implícitos. I6 corrige Source por normalização existente, preserva as delegações de elementos e fecha o único V16 emitido do lote. is_locatable recebe apenas qualificação documental; seu código executável não mudou.
