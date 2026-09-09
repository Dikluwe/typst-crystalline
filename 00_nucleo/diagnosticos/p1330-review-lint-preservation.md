# P1330 — preservação substantiva dos diagnósticos do linter

**PASS**: os diagnósticos P1329/P1330 são iguais em severidade, regra,
mensagem e caminho, inclusive multiplicidade e ordem. A única diferença
de localização disponível nos dois formatos é o avanço esperado de oito
linhas em calc.rs depois da linha 141. Não há warning/note novo ou removido.

Revisor `/root/p1330_review`, conferência em memória em
`2026-09-09T13:25:30.311Z` e consulta imediatamente seguinte.
HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
Os dois recibos guardam seus próprios UTC, HEAD, inventário e diff/stat
integrais, necessários para identificar os estados comparados.

Entradas imutáveis:

- p1329-final-lint.json SHA-256
  `d2f40499bfe78e0e55fcc3319aaf915665cee2ba2d996f5b8ddd6e3348596d1d`;
  comando `crystalline-lint .`, saída textual.
- p1330-final-lint.json SHA-256
  `e55256d2c4ae9f5c3291a3c5961b327529a98b0afe4bbdf317aeabe9cb91da28`;
  comando `crystalline-lint --format sarif .`, saída SARIF.

Método reproduzível: extrair do texto antigo todos os blocos com regex
`^(warning|info|error): ([\s\S]*?) \[(V\d+)\]\n\s+--> (.*):(\d+)\n`
em modo global/multilinha; mapear info textual para note SARIF. Extrair
do SARIF level, ruleId, message.text, artifactLocation.uri e startLine.
Todos os 1385 cabeçalhos textuais foram parseados, sem perda silenciosa.
Ambas as sequências contêm 240 warnings, 1145 notes e zero errors.

Comparar inicialmente multiconjuntos de severidade/regra/mensagem/caminho:
iguais. Em seguida, somar oito somente às linhas antigas de
`./01_core/src/compiler/stdlib/calc.rs` maiores que 141 e comparar as
sequências inteiras, incluindo linhas: igualdade exata. O deslocamento
atinge 41 diagnósticos e corresponde ao delta produtivo já revisado.
Isso sustenta preservação além das contagens agregadas.

Limite: a saída textual histórica não contém coluna, portanto não foi
possível comparar startColumn. Esta projeção de lint é documentada e não
normaliza outputs CLI da linguagem. Não repete o linter nem altera seus
recibos; não significa ausência global de warnings ou paridade geral.
