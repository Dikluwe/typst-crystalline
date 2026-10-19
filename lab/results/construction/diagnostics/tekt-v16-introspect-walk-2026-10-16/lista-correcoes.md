# Correções e grupos V19

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; estado completo, UTC e diffstat em before.json.gz.

| Ponto | Função | Alternativas com mesma consequência |
|---|---|---:|
| W1 | [reserve_locations](/repos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/walk.rs:78) | 112 |
| W2 | [reserve_locations callback](/repos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/walk.rs:196) | 117 |
| W3 | [table_root](/repos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/walk.rs:411) | 117 |
| W4 | [walk_at carriers](/repos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/walk.rs:595) | 116 |
| W5 | [walk_at fields](/repos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/walk.rs:810) | 117 |
| W6 | [reserve_tests::old_reserve](/repos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/walk.rs:1480) | 117 |
| Receptor | [outline_entry_receiver](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/call_dispatch/content_methods.rs:130) | 118 |

W6 é o oráculo de teste old_reserve; cinco dos seis V16 removidos estavam em produção. Os sete grupos têm consequência uniforme e foram prescritos nos L0, sem supressão. Não são prova de execução de todas as alternativas.

O complemento de extract_payload perde OutlineEntry (99→98); needs_fresh_index move OutlineEntry entre true/false (6→7 e 114→113). Os demais diagnósticos ficam iguais após mapear linhas.

Correções funcionais: delegação do payload e alinhamento de classificação; preservação de Location/modo raso em Source; body e métodos da entry através de Source/Located; índice por tipo; ordem de entries sintetizadas na posição causal do outline, com prefixos de state/counter; projeção legada de level/element/fill em L2, mantendo snapshots Some autoritativos.
