# P1234 — auditoria corrigida de precisão e serialização

**Proveniência:** HEAD `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`,
working tree não commitada em `2026-08-27T21:58:24-03:00`. A lista exata de
paths, `git diff HEAD --stat`, HEAD e horário foi preservada em
`p1234-execution-provenance.txt`, SHA-256
`57bfcda15393d7e7b184180fa352ae9f20f323a71cd5121e4bbcdceb7c43ea26`.

**Veredito:** `ACCEPTED_DIAGNOSTIC` — sem edição de produção.

O probe saneado derivou 30 probes (dois meshes para cada uma das 15 fixtures
falhas do P1233) e emitiu 630 linhas B01–B07. Duas execuções completas foram
byte-idênticas. Os checks estruturais de população, inputs, identidade do
binário, opacidade e causalidade foram aplicados; não houve mutation testing e
nenhum mutation score é alegado.

## Resultado causal

- Os 30 probes, incluindo as seis fixtures `linear/oklab`, têm B01/B02
  publicamente iguais na precisão exposta pelo binário atual. As 12
  divergências Oklab do ensaio anterior não foram reproduzidas; o binário
  histórico não foi preservado e não pode sustentar uma decisão presente.
- B03 (`to_rgba_f32`) e B04 (tupla pré-quantizada da métrica) não são expostos
  pelo CLI e permanecem `Unknown`.
- Nenhum probe atribuiu causa primária a L3. B05–B07 foram medidos a partir dos
  SVGs congelados, com `stop-color` e `stop-opacity` separados, mas não podem
  absolver fronteiras anteriores opacas.

Isto não autoriza correção L1 ou L3 para nenhum cluster. Os 30 probes precisam
de uma sonda interna legitimada ou diagnóstico posterior que atravesse B03/B04
e demonstre ligação quantitativa com as falhas P1233.

Artefatos centrais: `p1234-precision-probes.tsv`,
`p1234-precision-boundaries.tsv`, `p1234-precision-owners.tsv`,
`p1234-precision-clusters.tsv`, `p1234-precision-commands.tsv`,
`p1234-precision-ataques.tsv` e `p1234-tekt-certificado.tsv`.

Regime: reprodução diagnóstica, não preseal ou protocolo completo. O ambiente
partilha filesystem e contexto; portanto o resultado é `EXECUTADO SEM
ATESTAÇÃO DE ISOLAMENTO`.
