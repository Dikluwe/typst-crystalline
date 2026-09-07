# P1306 — `std.typ` deixa de ser chamado de `global` no erro

Implementado e verificado. Para um arquivo ordinário `std.typ` importado como
`specimen`, o acesso `specimen.nope` passou de:

```text
antes: module `global` does not contain `nope`
agora: module `std` does not contain `nope`
```

O global real, alcançado pelo binding `std` ou por aliases, continua dizendo
`module global` — com as crases da mensagem original. Ele já armazena o nome
`global` desde P1305. A correção remove somente a conversão nominal remanescente
no diagnóstico e usa o nome guardado no módulo. Não altera constructors,
imports, repr, lookup, spans ou reconhecimento especial de `pdf`/`sym`.

Veredito independente: **`P1306_PASS_NAMED_MODULE_DIAGNOSTIC_IDENTITY`**.
Certificado: `p1306-certificate.json`, SHA-256
`9c7008973799dcf02cb3f02b08a76707c5e52a0a63b7738260fad2c06664733d`,
emitido em `2026-09-07T15:31:31.034785+00:00`.

## O que poderia refutar a correção

Os testes distinguem o arquivo ordinário do global real, inclusive por alias,
sombra lexical, acesso aninhado e reexport integral `std: *`. Também cobrem
`global.typ`, `map.typ`, nome não reservado, calc/sym/color.map, campos ausentes
distintos e deslocamentos de linha/coluna. Os positivos pareados confirmam que
os imports são válidos e que campos existentes continuam com valor, kind,
chamada e repr corretos.

O RED formal falhou somente pela identidade de nome em 40 combinações
caso/perfil, repetidas nas três ordens; os dois testes de controle passaram.
Após a mudança de uma linha produtiva, os mesmos três testes passaram sem
alteração das expectativas. A matriz final executou 37 probes × quatro perfis
× três ordens: **444/444 satisfeitos**, sem `Unknown` e sem instabilidade.
Ela compara o candidato novo com os envelopes vanilla previamente congelados
e as exceções de preservação individualizadas abaixo; não são 444 alegações
de igualdade irrestrita com o vanilla.

Os **11 mutantes reais**, compilados em cópia descartável, foram rejeitados:
nome antigo, todos como global, global chamado std, nome do alias, correção
apenas de `nope` ou de um perfil, span no alvo/expressão, perda/duplicação de
diagnóstico e apagamento de lookup existente. Não houve mutante inválido,
sobrevivente ou morte por falha de instrumentação. A restauração foi conferida
após cada execução; os controles sem mutação passaram antes e depois.

## Validação e limites que continuam visíveis

- Suíte completa: **6.580 testes passaram, zero falhas, três doctests ignorados**.
  Os filtros P1290/P1300/P1301/P1303/P1305 executaram casos e passaram.
- Build fresco release, `cargo build`, formatação e `git diff --check`: passaram.
- Lint estrito V3/V4/V5/V13/V14/V15/V26: zero violações; dry-run sem reparos.
  Lint geral: **zero erros, 236 warnings e 1.133 infos**. O verificador comparou
  mensagens, severidades e paths com o recibo P1305: nenhuma adição/remoção,
  desconsiderando apenas deslocamentos de linha. Não se afirma lint geral vazio.

Permanecem as diferenças preexistentes individualizadas: warnings de
`this import has no effect` ausentes em dois casos de bare import; span total
do dicionário, preservado por contrato; rastro de importação ausente no caso
de erro dentro de arquivo importado. A âncora real desse erro continua resolvida
na fonte correta. Não se removeram warnings ou paths genericamente do comparador.

Encoders, anonimato de plugin e demais coortes continuam fora do escopo. Os
504 pares históricos de disponibilidade do P1305 não foram recontados nem
reduzidos por estas correções de diagnóstico. Não houve nova auditoria integral
dos 627 probes ou dos 15 mapas.

## Proveniência e custo

As medições usam HEAD `b303f1f15b610e09872b567027e0d806387fde8c` mais working
tree **não commitada**. Os únicos arquivos tracked alterados são:

```text
00_nucleo/prompts/compiler/eval/bindings/field_access.md
00_nucleo/prompts/compiler/eval/tests.md
01_core/src/compiler/eval/bindings/field_access.rs
01_core/src/compiler/eval/tests.rs
```

Os L0 precederam os testes/código; as cláusulas antigas P1301/P1303 foram
explicitamente revisadas sem reescrever medições históricas. Os testes anteriores
permaneceram intactos. O manifesto `p1306-manifest.json` e cada recibo registram
HEAD, status, diff/stat, horários, comandos e hashes, incluindo os diagnósticos
novos. A cadeia confere o inventário de 1.027 arquivos e preserva os demais owners.

Referência: vanilla ratificado upstream/main `a51e02804`. Candidato fresco em
`/dev/shm/p1306-target.g8kO5v1h/release/typst`, SHA-256
`945e488c53246b33fd11c56ff670c697aeaffa5a92d5c15cc8e09a76cf7cf1f3`.
O baseline certificado P1305 foi reconferido e não sobrescrito.

Custos registrados, com execuções paralelas que não devem ser somadas como
tempo de espera: build release 182,328 s; suíte workspace 269,462 s; controle
inicial dos mutantes 197,809 s; mutantes 298,222 s; controle restaurado 14,663 s.
Recibos principais: `p1306-red.json`, `p1306-green.json`, `p1306-build.json`,
`p1306-gates.json`, `p1306-matrix.json` e `p1306-mutant-ledger.json`.

A tentativa contratual com `--color`, rejeitada pelo vanilla, foi preservada
integralmente e não contou como RED. A inspeção da fonte do linter identificou
risco de metadata inversa escapar da seleção por V5; não se alegou uma falha
nova medida no executável instalado. O `Hash do Código` foi derivado dos bytes
reais e atualizado mediante reabertura explícita somente de metadata e selo
sucessor independente. Nenhum contrato semântico ou oracle mudou após o selo.

A skill de materialização segregada separou contrato/L0, oráculos/ataques,
implementação e veredito entre autoridades distintas. Regime: **executado sem
atestação de isolamento técnico**, pois o ambiente é compartilhado. O certificado
atesta somente o fragmento descrito, não equivalência geral do compilador.

Implementação concluída; **sem commit nesta execução**.
