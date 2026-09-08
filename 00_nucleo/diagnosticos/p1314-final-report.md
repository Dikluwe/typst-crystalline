# P1314 — opções CSV: validar ocorrências e mostrar a origem do erro

## O que mudou

Uma opção CSV inválida pré-ligada não pode mais desaparecer quando uma opção
válida posterior a sobrepõe. Por exemplo:

```typst
{ let f = csv.with(delimiter: "ab"); f(bytes("a,b"), delimiter: ",") }
```

No baseline P1313, isso retornava uma tabela. A correção exige o erro
`expected exactly one character` na origem de `"ab"`, antes de qualquer
tentativa de leitura de arquivo. O mesmo tratamento vale para row-type
inválido transportado por With/Args/spread.

Erros de delimiter e row-type passam a apontar para o valor que falhou,
em vez de aparecerem sem origem. Todas as ocorrências de delimiter são
validadas antes de todas as de row-type, mantendo a ordem dos parâmetros da
função; dentro de cada nome vale a ordem causal. Se todas forem válidas, a
última vence. Args sintético ou origem explicitamente detached permanece
detached, sem range inventado.

Isso não muda os tipos aceitos, defaults ou parser: delimiter continua Str
de um caractere ASCII e row-type continua tipo array/dictionary. O patch
extraiu os casts existentes para funções privadas e acrescentou um consumidor
privado de ocorrências. Não alterou entidades, assinaturas públicas, traits,
dispatch, read, outros loaders, encoders ou o decoder CSV.

## O que continua aberto

Não é paridade geral de CSV. Named desconhecido ainda vence a fonte/opções;
missing e excesso mantêm a política anterior. Duplicatas sintáticas diretas
continuam sendo rejeitadas pelo parser, antes da função — são distintas das
ocorrências trazidas por With/Args.

Parsing CSV continua com mensagens/âncoras legadas, inclusive campos desiguais
e UTF-8 inválido. Erros de I/O não foram reformulados. Symbol como fonte
continua rejeitado; delimiter Symbol continua `expected string, found symbol`,
agora na origem correta. O vanilla converte Symbol para Str: essa rejeição é
uma fronteira normativa preservada, não paridade da coerção. `csv.encode`
continua inexistente.

## Decisão e proveniência

A decisão foi precedida pela medição bilateral
`p1314-measurement.json`, SHA-256
`660fd77c95fe243e22590036b03dc5e364b0e3a107888592b35c4d166d557626`.
A intenção está explícita nos casts CSV e no consumo de todas as ocorrências
em `foundations/args.rs:218–235` do vanilla ratificado. A macro da função
confirma a ordem por parâmetro, não uma ordem intercalada conveniente.

O novo L0 substitui expressamente a preservação P1313 somente para origem dos
erros e validação de todas as ocorrências das duas opções. A revisão
`p1314-review-precontract.md` classificou o recorte como `CONTINUOUS_SCOPED`
pela ADR-0127: correção interna de paridade, sem nova API/fase/cast admitido.
Não foi necessária uma nova paragem humana após o pedido de escrever e
implementar o próximo passo.

Todas as medições são de working tree **não commitado** sobre HEAD
`eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`. Baseline anterior ao L0/patch:
`p1314-baseline.json`, SHA-256
`75cccd6e392df84001640f244facde73883fd414de045253e97e6a12bfad70c6`.
Os recibos registram UTC, argv, saídas integrais, HEAD, diff/stat, diff e índice.
As alterações anteriores P1310–P1313 continuam presentes e não são atribuídas
a esta implementação.

Identidades do candidato:

- Owner `01_core/src/compiler/stdlib/loading.rs`: SHA-256
  `1e911a53392ce900e482cf020973d5c78136977c032045fdc4f66e10ea8ec83a`.
- L0 `00_nucleo/prompts/compiler/stdlib/loading.md`: SHA-256
  `6b717ccd9ddd4784407ca2b1d052f25c63c78198062748f449538a6ff85a0097`.
  Hash efetivo `e1f22b24`; Hash do Código `03ea1fcc`.
- Binário `/dev/shm/p1314-target.cswujn/release/typst`: SHA-256
  `cf9b997ac399cfb95cf15063cc417484ed28a75be18f6e7d55a28300d792114f`.
- Baseline `/dev/shm/p1313-target.keFg93/release/typst`: SHA-256
  `cefb4b485cc25ae98871cfbd7a76925d0333d6d26906dc7f7426868e899285ce`.
- Vanilla `/usr/local/bin/typst`, upstream `a51e02804`: SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

Target RAM dedicado, com cópia independente do cache anterior; nenhum baseline
foi sobrescrito. `/dev/shm` e `/tmp` do sandbox padrão diferem do namespace
host usado pelos testes. Reproduzir os comandos RAM requer esse mesmo acesso
autorizado; ENOENT no sandbox não significa remoção do executável.

## Independência e testes

Regime A/B, **executado sem atestação de isolamento técnico**. Root escreveu
L0, testes locais e patch. O testador `/root/p1314_tests`, em contexto novo,
congelou expectativas antes de receber o candidato. O revisor
`/root/p1314_review` não edita os artefatos julgados. Filesystem compartilhado
não permite atestar isolamento. O baseline contém diff histórico dos passos
anteriores; essa exposição foi declarada pelo testador, não confundida com
leitura do patch P1314. Não há selo completo ou mutation score.

Freeze `p1314-ab-freeze.json`, SHA-256
`41ac0730e01cd31cd9660b0e229e37d5e486a2c114927f3ac49c3475733756b9`.
O L0 normativo permanece
`21231170b7092331ae09fafc656210bb94829658d822a15291de6ea0eb28dbfe`;
somente o metadado Hash do Código foi atualizado. O passo é coordenação,
não fonte normativa congelada. `p1314-review-prepatch.md` autorizou o patch
depois de conferir expectativas, identidade e preservações.

RED local: quatro falhas reais e dois controles passam; uma falha flagra
World::resolve_path indevido depois de ignorar a opção inválida anterior.
GREEN: seis testes passam, já no source final resselado. Não houve mudança
no código durante essa execução. Uma tentativa operacional de apply_patch
falhou por formato do cabeçalho de hunk, sem alterar arquivos; a aplicação
seguinte corrigiu o comando, não o contrato ou o resultado de um teste.

## Resultados finais

Os resultados abaixo se referem ao candidato e working tree identificados
acima; horários, comandos, diff/stat e saídas completas estão nos recibos.

| Validação | Resultado | Recibo em diagnósticos |
|---|---|---|
| RED → GREEN local | 4 falhas + 2 controles passam → 6 passam | `p1314-unit-red.json`, `p1314-unit-green.json` |
| Build workspace release | exit 0 | `p1314-build.json` |
| Testes workspace release | 6.645 passam, 0 falham, 3 ignorados | `p1314-workspace-tests.json` |
| A/B independente | 2.808 comparações passam; 0 falhas/Unknown | `p1314-ab-comparison.json` |
| Lint | 0 erros, 240 warnings, 1.136 infos | `p1314-lint.json` |
| Fmt, diff-check, resselo | exit 0; Nothing to fix | `p1314-fmt.json`, `p1314-diff-check.json`, `p1314-lineage-preview-final.json` |
| Replay P1310 contra P1313 | 1.036 preservados, 0 deltas | `p1314-p1310-replay.json` |
| Replay P1311 contra P1313 | 220 preservados, 0 deltas | `p1314-p1311-replay.json` |
| Replay P1312 contra P1313 | 276 preservados, 4 deltas de delimiter | `p1314-p1312-replay.json` |
| P1313, dentro do A/B | 524 preservados, 52 deltas previstos | `p1314-p1313-replay.json` |
| Replay P1308 contra P1313 | 1.982 preservados, 0 deltas novos | `p1314-p1308-delta.json` |

A/B contém 234 expressões nos quatro perfis, normal/repeat/reverse, sem
instabilidade. Inclui os 144 casos P1313, executados no cwd original. O
replay P1313 acima é a recomparação dessas linhas na ordem normal, **não uma
execução adicional**; os 52 deltas são os 13 casos de opções predeclarados
nos quatro perfis. O restante preserva literalmente as observações P1313.
Contra o oráculo P1308 antigo continuam 1.938 Preserved, 44 Violated e zero
Unknown, sem nenhuma mudança nova. Não foram reescritos oráculos históricos.

Comparação independente `p1314-ab-comparison.json`: SHA-256
`c04d16bed5b824dc52db6dfb383d21fecb0d56bdd8fdaf9e1c981515a545f0da`.
Execuções brutas `p1314-ab-candidate-runs.json`: SHA-256
`e1aa3bf594dbcd95c2ed818ec3747e8cd10193ee341f00dc79e51250e1bf1155`.
Recibo independente `p1314-ab-receipt.md`: SHA-256
`a0d310891f913942183d89d479a7aa84fc81887f62bbf662842ea68dcdf22417`.
Os controles de ausência de chamadas World e Args sintético Rust pertencem
aos testes locais/revisão de código, não são atribuídos à observação CLI.

Revisão já recontou 8.083 arquivos: só o owner e o L0 correspondente mudaram
contra o baseline deste passo; os 252 artefatos anteriores pinados permanecem
intactos. Não houve stage, commit, push, alteração de testes anteriores ou
limpeza dos baselines.

Suíte workspace completa: `p1314-workspace-tests.json`, SHA-256
`6d85a954e73742a97c1d9927d0cb99f10f546939d9e099272b3a976078330ac1`.
Não houve falha intermitente nem repetição necessária. Os três ignorados são
doctests antigos de layout/introspector; nenhum teste foi desativado.
Recontagem dos gates `p1314-gates.json`, SHA-256
`f760bcd10839bb8996623f1ce4b4978a453d5095fb91f64f9355c5ef24526e6a`:
todas as verificações passaram. Lint sem erros não significa sem avisos;
os warnings e infos estão explicitados acima.

**Fechamento: PASS_SCOPED**, sem achados pendentes, conforme
`p1314-review-final.md` (SHA-256
`c4eb2fea310d55e5f3c9ae1053f8e0664f3df2a6c2eeffd9787fff344ac08fd5`).
Evidência independente `p1314-review-evidence.json`, SHA-256
`6c0f091da3eef84294af36a6b8b0dd47027a86ff2d410080e3b983a697067751`,
reproduzível por `p1314-review-check.cjs`. O parecer cobre as opções CSV e
as preservações declaradas; não encerra as demais dívidas listadas acima.
