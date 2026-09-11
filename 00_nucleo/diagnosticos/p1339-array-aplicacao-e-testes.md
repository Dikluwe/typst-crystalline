# P1339 — aplicação de Array←Bytes e correção dos testes

## Resultado parcial

Implementado `array(bytes)` no owner dedicado, com duas reexportações internas
e seleção tipada no dispatcher. A conversão conserva os octetos sem sinal,
ordem, multiplicidade e vazio; rejeita sobras pela origem do argumento e usa
a origem agregada apenas quando aquela está detached. O dispatcher conserva
o histórico da chamada quando a origem do erro vem de um spread externo.

Não foi implementado um constructor geral de Array. Missing, named-only,
Array, Version e outros primeiros posicionais não Bytes conservam a rota
legada. Não houve alteração de expectativas, commit ou conclusão geral do P1339.

O selo estreito é
[`p1339-verifier-array-successor-seal-r1.json`](p1339-verifier-array-successor-seal-r1.json),
SHA `cb368ce0be06d2b955dd0e213256f09778197ae04256679ba7a0d4d907ef2c5b`.
A autorização e os quatro L0 precederam o código produtivo. O RED local
registrou duas falhas de constructor ausente e um controle legado verde;
o RED independente de E05 revelou depois o trace ausente, corrigido sem
mudar L0 normativo nem oráculo.

## Evidência e proveniência

HEAD de todas estas medições: `2f42d64253547734564513a1159ee6b584c1c4b4`,
**working tree não commitado**. Cada recibo indicado contém UTC, comandos,
canais integrais, hashes e a lista exata de alterações (`status`, `diff_stat`
e hashes de arquivos em `before`/`after`). Não confundir esse estado com HEAD
limpo nem usar `--version` para identificar o executável.

| Verificação | Resultado medido | Recibo |
|---|---|---|
| RED local antes da implementação | 2 falhas causais; 1 controle verde | [local-red-r1](p1339-array-local-red-r1.json) |
| Testes locais após correção de trace | 4 passaram | [local-green-r2](p1339-array-local-green-r2.json) |
| Harness independente no owner real | 1 teste executado, contendo 10 vetores | [candidate-owner-r2](p1339-array-candidate-owner-r2.json) |
| Estilos: cadeia realmente usada no replay | 12 células; teste passou | [candidate-style-r2](p1339-array-candidate-style-r2.json) |
| Build CLI final | exit 0 | [candidate-build-r2](p1339-array-candidate-build-r2.json) |
| V5/V14/V15/V26 estritos | exit 0, sem violações desses checks | [lineage-critical-r2](p1339-array-final-lineage-critical-r2.json) |
| Resselo dry-run | `Nothing to fix` | [lineage-dry-run-r2](p1339-array-final-lineage-dry-run-r2.json) |
| Lint completo padrão | exit 0; 0 erros, 249 avisos | [final-lint-r2](p1339-array-final-lint-r2.json) |

Binário CLI final: `/tmp/p1339-target.UD8gh7/release/typst`, SHA-256
`ec13271aa5984c2eda3115c6e6988d210eeb08432e09f94c0e5343258c4d2dc9`.
A proveniência do testbin instrumentado é explicitamente um pin **pós-run**,
não uma prova inventada de pin antes/depois da execução:
[instrumented-post-run-pin-r2](p1339-array-instrumented-post-run-pin-r2.json).

Comparação pública final de Array: **aguardando focal r2 e expansão**.

## Falhas que continuam abertas

A última execução dos 70 programas originais, no candidato anterior à
correção exclusiva de trace (SHA `b689f3fe…`), mediu **68 preservados e
2 diferenças**, sem Unknown. Fontes e expectativas permaneceram intactas:
[raw](p1339-array-originals-focal-runs-r1.json),
[julgamento](p1339-array-originals-focal-judgment-r1.json) e
[revisão independente](p1339-verifier-array-candidate-focal-review-r1.json).

Os casos `historical-bytes-rounding-1e-45-little` e
`historical-bytes-rounding-1e-45-big` esperam `1.401298464324817e-45`;
o candidato representa o número em decimal extenso e isso também força a
tupla a quebrar linha. A causa está em `compiler/eval/repr.rs:1094`:
`repr_float` usa Display. A referência ratificada a51e02804,
`foundations/float.rs:191` e `foundations/repr.rs:95-119`, usa a opção que
preserva o separador decimal e admite notação científica.

Essa dimensão precisa de especificação e validação próprias no owner de
repr; não foi corrigida incidentalmente no selo Array de quatro owners.
Os dois testes continuam **FAILED**, não foram reclassificados como sucesso
ou retirados do contrato. A expansão dos 840 casos originais não foi
executada. O restante do pipeline e os gates F globais do P1339 continuam
abertos. Os avisos globais do lint também impedem alegar “zero violations”.

## Ajustes mecânicos e limites de evidência

- Os testes de estilos usam reborrow com o ABI correto e observam a cadeia
  real retida pela transação, não a cadeia esperada pelo teste. O JSON
  congelado passou a ser lido por `from_slice(include_bytes!(...))`, já
  permitido em L1; fixture, expectativas e whitelist não mudaram.
- O script histórico de cobertura com erro de sintaxe foi preservado byte
  a byte. Apenas seu path exato foi classificado como não executável em
  `excluded_files`, conforme
  [aceite independente](p1339-verifier-lint-history-acceptance-r1.json).
  A falha de parsing na configuração anterior continua registrada.
- Quatro campos canônicos `Hash do Código` foram derivados nos seis owners
  Array/Style, sem mudar bytes produtivos ou SHA normativo:
  [linhagem antes/depois](p1339-array-style-derived-lineage-r1.json).
- O protótipo Node do runner instrumental abortou no primeiro snapshot Git
  com `spawnSync EPERM`, antes de compilar ou testar. Foi preservado e
  sucedido pelo recorder Python, que reutiliza o transporte congelado e
  registra as entradas de teste transitivas. Não recebeu crédito de teste.
- E08 continua sendo um probe exploratório inconclusivo, sem crédito. A
  obrigação de origem detached é exercitada pelos vetores privados reais;
  não se converteu o Unknown público em sucesso.

Regime: autoridades de contrato/oráculos, implementação, ataques e veredito
separadas, **executado sem atestação de isolamento**. Execuções mecânicas
finais foram chamadas diretamente por scripts, sem nova delegação para
cada rodada. Nenhuma equivalência geral foi inferida deste recorte.
