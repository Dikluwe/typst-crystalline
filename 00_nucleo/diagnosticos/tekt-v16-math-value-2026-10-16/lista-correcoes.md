# Correções e grupos V19 preservados

Medição: working tree não commitado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`; before.json.gz conserva hashes, UTC, status e diffstat.

| Caso | Consumer / função | Linha anterior | Grupo atual | Decisão |
|---|---|---:|---:|---|
| M1 | [parse_delim_val](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/math/delimiters.rs:19) | 19 | 35 variantes | Corrigir wildcard; manter V19 informativo |
| M2 | [parse_delim_char](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/math/delimiters.rs:62) | 28 | 36 variantes | Corrigir wildcard; manter V19 informativo |
| M3 | [vec_delim_char](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/structural/math/vector.rs:47) | 47 | 36 variantes | Corrigir wildcard; manter V19 informativo |
| M4 | [math_matrix_augment](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/structural/math/matrix.rs:114) | 114 | 37 variantes | Corrigir wildcard; manter V19 informativo |

M1–M3 preservam None para os tipos atuais sem representação de delimitador. M4 preserva os bits do Stroke já validado; atalhos Length/Color continuam marcando somente seu campo. As alternativas partilham o mesmo corpo; não constituem decisões distintas. Os sete controles exercitam os casos descritos nos L0, sem alegar cobertura de execução de cada variante dos grupos.

As fronteiras existentes que devolvem erro continuam com o mesmo diagnóstico; não foram convertidas em ausência silenciosa. Nenhuma supressão/configuração foi adicionada.
