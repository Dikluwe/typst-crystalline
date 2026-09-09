# P1335 — owners mínimos medidos pela revisão

Estado: baseline P1335 `0ec240607d173acd513a21d2728c66a1e514f3ad294b88bad60f3fdc8e984254`,
HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093` mais a árvore não commitada fixada
nesse recibo. São limites inferiores condicionais ao recorte causal declarado,
não seleção nem autorização de implementação.

`01_core/src/compiler/stdlib/loading.rs:1197` resolve DataSource e sua ausência;
`01_core/src/compiler/eval/call_dispatch.rs:455` controla transporte do span agregado.
O match fechado inclui CSV/encoders JSON–TOML–YAML/panic/abs, não todos os decoders.
`01_core/src/compiler/stdlib/mod.rs:135` já reexporta os ponteiros de CBOR, CSV,
JSON, read, TOML, XML e YAML. Portanto, `loader-data-source-missing` e
`loader-path-missing`, quando exigem o diagnóstico inteiro também através de With,
necessitam ao menos loading e call_dispatch. Retirar o transportador para baratear
a coorte contraria a fonte. Não se presume um owner de fachada adicional, pois os
ponteiros já estão exportados. Testemunha que implique outro transportador refuta
a suficiência desses dois; não refuta este limite inferior.

`01_core/src/compiler/eval/modules.rs:177` propaga eval_imported_file com `?`;
`lab/typst-original/crates/typst-eval/src/import.rs:36` acrescenta Tracepoint::Import.
A variante e sua formatação existem em source_result e `02_shell/src/diagnostic.rs:162`.
A hipótese estreita de trace na falha da avaliação do arquivo tem modules como owner
indispensável. Uma necessidade real de formatter/entidade refuta owner único.

L0 `compiler/eval/math.md`, integralmente lido, exige resolução pelo scope e
lookup/chamadas conformes à língua. `math.rs:517` extrai spelling de MathIdent e
entra em braços nativos antes do fallback de valor resolvido. A hipótese de shadow
de abs/sqrt por função que devolve Content exige este owner e tem prioridade 2
somente se testemunhas bilaterais atuais isolarem esse comportamento. O retorno
Content-only é explicitamente preservado pelo próprio L0 e não se funde nessa causa.
Não se infere nova representação ou mudança de fase.

CBOR merece atenção separada: `eval/mod.rs:1836` registra o nome intrínseco
`cbor.encode`. Um trace por With pode exigir corrigir também esse registro, além
da mensagem em loading e do agregado em call_dispatch. A coorte de dois owners
permanece hipótese até medir a rota; não é aceita por copiar a contagem histórica.
