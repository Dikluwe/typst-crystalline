# P1313 — execução autorizada: CSV com Bytes e cast DataSource

## Resultado funcional

CSV passa a consumir `Bytes` como dados em memória. Por exemplo,
`csv(bytes("a,b\n1,2"))` produz `[["a","b"],["1","2"]]`; entrada vazia
produz `[]`. Delimitador, quoting, Unicode e linhas dictionary usam o decoder
já existente. O ramo Bytes não resolve caminhos nem chama World.

Um primeiro argumento inválido passa a produzir
`expected path, string, or bytes, found <tipo público longo>` na origem do
valor posicional, inclusive quando há named anteriores ou o argumento foi
transportado por With/Args. Origem sintética/detached permanece detached.

A mudança produtiva limita-se ao helper privado CSV e à escolha da fonte em
`native_csv`, depois da validação das opções. Não foram alterados o decoder,
`read`, os cinco outros decoders, os encoders, entidades, traits, dispatch ou
fase do pipeline. Os testes locais antigos que exigiam rejeição de CSV Bytes
foram substituídos pelo contrato explicitamente aprovado; os controles read
continuam. Nenhum oráculo independente anterior foi reescrito.

## Limites e dívidas que continuam abertas

Não é paridade integral dos diagnósticos CSV. Bytes malformados e opções
inválidas usam mensagens e âncoras legadas. No caso de linhas desiguais,
continua `failed to parse CSV (found 1 instead of 2 fields in line 2)`, detached,
sem o complemento `at 2:1` da referência. UTF-8 inválido mantém o mapper legado.
Named desconhecido continua vencendo o cast; argumentos ausentes, excesso e
duplicatas preservam suas políticas anteriores. Não corrigir incidentalmente
essas divergências foi parte explícita da aprovação, não adaptação ao candidato.

Symbol continua rejeitado, agora com mensagem canônica e origem do valor.
Isso é efeito normativo aprovado, não paridade Symbol: o vanilla converte
Symbol para string e tenta leitura. `csv.encode` continua inexistente.

## Autorização, estado e proveniência

O dono respondeu especificamente «Autorizo» à proposta CSV do passo 1313.
O recibo anterior a qualquer ativação/patch é
`p1313-implementation-baseline.json`, SHA-256
`26d2e294e7e66a46b41f8274743adaa4bc285083a4b79a04a82e53f544bd349f`.
Todos os resultados deste relatório vêm de working tree **não commitado** sobre
HEAD `eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`; os recibos contêm horários UTC,
argv, stdout/stderr, diff integral, diff/stat e estado do índice. As alterações
P1310–P1312 preexistentes continuam presentes; não são atribuídas a este patch.

Identidades finais de produção:

- Owner `01_core/src/compiler/stdlib/loading.rs`: SHA-256
  `a8ef0db2cd9531d6f86aa5c346c3860495a002a8244b4e9594cc9fd6f15b3468`.
- L0 `00_nucleo/prompts/compiler/stdlib/loading.md`: SHA-256
  `103a2035a2c4c534e3e905bf168ec50d06dc6c2fc484cfc362a74b22f7bf9c5c`.
  Hash efetivo L0 `687015be`; Hash do Código `9316ba1b`.
- Candidato `/dev/shm/p1313-target.keFg93/release/typst`: SHA-256
  `cefb4b485cc25ae98871cfbd7a76925d0333d6d26906dc7f7426868e899285ce`.
- Baseline P1312 `/dev/shm/p1312-target.B8uLa9/release/typst`: SHA-256
  `446c3ccfa5ef7f5eaf6031543f4d28566dac6eb53dca842ec7eb16acb084fbf0`.
- Vanilla `/usr/local/bin/typst`, upstream ratificado `a51e02804`: SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

O target P1313 foi criado na RAM e recebeu cópia independente do cache;
nenhum binário baseline foi substituído. Artefatos RAM são temporários;
os comandos e saídas integrais ficam nos diagnósticos do repositório.
Neste ambiente, `/dev/shm` e `/tmp` do sandbox padrão diferem do namespace
host usado pelos testes. Para repetir os comandos RAM é preciso acessar o
mesmo namespace autorizado; ENOENT no sandbox não demonstra remoção do binário.

## Segregação proporcional e sequência causal

Regime A/B, **executado sem atestação de isolamento técnico**. Root escreveu
L0, testes locais e implementação; `/root/p1313_tests`, em contexto novo,
escreveu casos/expectativas sem ler código candidato; `/root/p1313_review`
revisou sem editar os artefatos julgados. O filesystem é compartilhado.
Não há selo completo de refinamento, mutation score ou equivalência geral.

Freeze independente `p1313-ab-freeze.json`, SHA-256
`0f7ba52eb0ec820f53754274d51cb7ff4d20aab8d284d3c56161ff3d70b3e998`,
fixou casos, políticas, baseline/vanilla, fixtures, step e L0 antes do patch.
O hash normativo L0 permanece
`553294f33c131121926207fb203c1e4a4174b03e653afe39e5827b3a773fead3`;
somente o metadado Hash do Código foi atualizado depois. O parecer
`p1313-review-prepatch.md` autorizou o patch no recorte congelado.

RED inicial e RED-r1 preservados em `p1313-unit-red*.json`: seis falhas reais,
sem erro de compilação. R1 acrescentou a cobertura UTF-8 antes do patch.
O GREEN local verificou seis testes. Ele precede o resselo e teve ajuste
apenas de comentário durante a execução; não é usado como identidade do estado
final. A suíte completa final é o gate daquele estado.

O passo congelado mantém o status que tinha na entrada da materialização;
esta entrega registra o resultado da execução sem invalidar seu pin.
`p1313-final-report.md` é o relatório histórico da proposta e permanece
inalterado. Este arquivo é o relatório da implementação autorizada.

## Validação final

Todos os números abaixo se referem ao candidato e ao working tree identificados
acima. Fonte da recontagem: `p1313-gates.json`, SHA-256
`1d5ea157e0dc195fc62c3aaea574844eb021464e5d0e630f0c91657422872f1c`;
os recibos de cada linha contêm proveniência e saídas integrais.

| Gate / corpus | Resultado | Recibo em diagnósticos |
|---|---|---|
| Testes locais RED → GREEN | 6 falhas esperadas → 6 passam | `p1313-unit-red-r1.json`, `p1313-unit-green.json` |
| Build workspace release | exit 0 | `p1313-build.json` |
| Testes workspace release | 6.639 passam, 0 falham, 3 ignorados | `p1313-workspace-tests.json` |
| A/B independente | 1.728/1.728 comparações passam; 0 Unknown | `p1313-ab-candidate-runs.json`, `p1313-ab-comparison.json` |
| Lint | 0 erros, 240 warnings, 1.138 infos | `p1313-lint.json` |
| Fmt / diff-check / resselo | exit 0; Nothing to fix | `p1313-fmt.json`, `p1313-diff-check.json`, `p1313-lineage-preview-final.json` |
| Replay P1310 contra P1312 | 1.032 preservados, 4 deltas CSV esperados | `p1313-p1310-replay.json` |
| Replay P1311 contra P1312 | 220 preservados, 0 deltas | `p1313-p1311-replay.json` |
| Replay P1312 | 264 preservados, 16 deltas CSV esperados | `p1313-p1312-replay.json` |
| Replay P1308 contra P1312 | 1.974 preservados, 8 deltas CSV esperados | `p1313-p1308-delta.json` |

Nenhum Unknown nos replays. Deltas de cast/sucesso CSV dos replays P1310/P1308
coincidem com vanilla prépatch. Os quatro casos CSV de P1312 usam expectativas
independentes congeladas, incluindo Symbol normativo separado. Contra o oráculo
P1308 antigo, o resultado continua **1.938 Preserved, 44 Violated, 0 Unknown**:
36 deltas aprovados anteriores mais oito novos. Não é um replay histórico
integralmente verde, nem se atualizou seu oráculo para aparentar isso.

O incidente intermitente do workspace P1312 não ocorreu nesta execução;
não foi necessária repetição. A causa histórica continua não estabelecida.
Os três ignorados são os doctests antigos de layout/introspector, não novos
testes desativados. Lint não significa ausência de warnings: mantém os avisos
acima, com zero erros de gravidade/linhagem.

Comparação de 8.083 arquivos de baseline: somente owner CSV e L0 correspondente
mudaram neste passo. Os 211 artefatos anteriores pinados estão intactos,
assim como o baseline executável e os arquivos P1311. Sem stage, commit ou push.

A/B executou 144 casos em quatro perfis, nas ordens normal/repeat/reverse,
sem instabilidade nem falha. O resultado independente `p1313-ab-comparison.json`
tem SHA-256 `773f2ffa166980fe9f3267a33a3a386c4eda27cdfed993e2d8d832e5a23da12d`;
as execuções brutas têm SHA-256
`5b75c1a7346fb92519bcb1dff11cd99c34ef567a7ceb050f2cf6801569c9ad4b`.
As limitações CLI (ausência de I/O, Args sintético e UTF-8 inválido) são cobertas
pelos testes locais e inspeção do código, não atribuídas à observação CLI.

**Fechamento: PASS_SCOPED**, sem achados pendentes no recorte, conforme
`p1313-review-final.md`. Recontagem independente reproduzível em
`p1313-review-check.cjs`; evidência `p1313-review-evidence.json`, SHA-256
`d5f631589e068734db35d3afc05358a0dd08c9e38ee528d8c83a1f103a634add`.
O parecer cobre esta materialização CSV e as preservações declaradas;
não encerra as dívidas de diagnóstico e Symbol descritas acima.
