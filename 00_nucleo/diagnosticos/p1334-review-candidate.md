# P1334 — candidato conforme ao recorte

Sem objeção ao candidato inspecionado. Em `calc.rs:207-257`, missing
procura a primeira ocorrência named value e usa arg-span/hint; ausência
comum usa agregado; depois do processamento do primeiro valor, primeira
sobra usa ordem conjunta e arg-span. None tem síntese ordinal detached.
Não há cast de sobra nem mutação de Args. Os braços anteriores ao missing,
incluindo fórmulas e value_span do primeiro inválido, permanecem intactos.

`call_dispatch.rs:455-488` acrescenta exclusivamente comparação por ponteiro
de calc_abs, mantendo With recursivo e atribuição incondicional do agregado.
Não examina nome, argumentos ou erro. `stdlib/mod.rs:77` acrescenta somente
o reexport pub(crate); nenhuma API pública ou lógica no hub. Args, math,
merge With, eager, nomes e trace_call não mudaram.

Auditoria reproduzível `node 00_nucleo/diagnosticos/p1334-review-candidate-audit.cjs`,
executada em `2026-09-09T16:13:58.020Z`, terminou sem findings. Restituindo
os snippets históricos, removendo apenas os dois módulos novos e recolocando
os dois trechos produtivos autorizados/reexport, os consumers coincidem com
`original_files` do baseline, exceto headers e separadores de linhas vazias.
O inventário inteiro fora dos três pares manifestados coincide. Todos os
inputs/artifacts do freeze R1 permanecem com hashes corretos; normas dos
três L0 continuam iguais ao manifesto. Não é atestação de isolamento.

Proveniência: HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree
não commitado. Baseline `6643993d43b905e878095c722c99019c5f98cdd724cddfc3b90961153b205819`
conserva diff/stat e original_files; a auditoria reconstrói explicitamente
o delta da árvore candidata. Manifesto
`2cac9aab9dc8e911e2a14e932515db3efd5be64b10b1089d06caac0b310be1da`,
freeze `684143d5fdbb4005e5d641f772e603f48da1c32c928840c89873bbcff5fa50b5`.
SHA-256 bruto dos consumers julgados:

```text
calc.rs          96196911a437819483766b9d21da08270063527721286dbf9d59c86b30fc9f61
call_dispatch.rs 1bfe1af535e7f73c90163f856f0edcbd62acce01466c4398dceeac57f673be7d
stdlib/mod.rs    a8d828535a135463106723e27013b75b89c63df4810662ff2cf35a775a100f80
```

GREEN, build/workspace, lint/linhagem e CLI normal/repetido/invertido dos
dois corpora ainda dependem dos recibos finais; este parecer é inspeção do
candidato, não fechamento. A/B sem atestação técnica, sem refinamento ou
mutation score. Revisor escreveu somente script de auditoria e parecer;
nenhuma entrada julgada foi editada. Incidente inicial de nomes restritos
permanece registrado em `p1334-review-scope.md`.
