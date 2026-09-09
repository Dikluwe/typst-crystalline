# P1319 — recibo A/B independente

PASS no fragmento público congelado: 7740 comparações, zero falhas e zero
Unknown, nas ordens normal/repeat/reverse. Regime A/B executado sem atestação
técnica de isolamento. Não constitui selo de refinamento, mutation score ou
paridade geral CSV/Path.

## Autoridade e entradas

Testador `/root/p1319_tests`, contexto limitado à tarefa e ao L0 loading inteiro,
skill `tekt-materializacao-segregada` e suas duas referências, ADRs 0107/0108/
0127/0129, fonte vanilla loading/csv.rs, loading/mod.rs, diag.rs e
foundations/path.rs, artefatos p1318-ab e fixtures históricas, executáveis e
git HEAD/status/diff STAT. Owner cristalino, testes locais, diff produtivo e
recibos do root com source não foram lidos. Nenhum acesso a materialization ou
context foi realizado; nomes que aparecem no git status não foram abertos.
Escritas limitadas a p1319-ab-* em diagnósticos e /tmp/p1319-ab-fixtures.
O filesystem é compartilhado, portanto as restrições declaradas não provam
isolamento técnico.

L0 normativo SHA-256
`da59f9964566dc99d3341145fe8200df15ff8df172158c79d91542d47015e823`,
excluindo exclusivamente uma linha canônica `Hash do Código`. A alteração
inicial ae096892→5249d235 nessa metadata foi conferida: repor a linha antiga
reproduzia raw SHA-256
`5060ce50ec107b7921bea31e18630bda3138b0e15eaa2a14285a9073c973e318`.
Nenhuma obrigação normativa mudou. Freeze gerado em
2026-09-08T16:09:27.147983+00:00, finalizado e comunicado pelo SHA abaixo
antes da implementação candidata e de sua liberação pelo root.

| Entrada | SHA-256 |
|---|---|
| Baseline /tmp/p1318-target.eAgQwp/release/typst | 0bdb7c2ca80d7be17775d03d3fc7ac83ba4401bf4468dd84ad7711a93b5a585c |
| Vanilla /usr/local/bin/typst, upstream ratificado a51e02804 | 7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8 |
| Candidato /tmp/p1319-target.VqXtmj/release/typst | 37a8a23d6e2b5daf355d90d510bca7efb399547730e1d807e8ae871be75748bd |
| p1319-ab-runner.py | e78afd3095f6b254efb2c396c292fa6a6f6660e64f512914df8096cda8f165dc |
| p1319-ab-cases.json | 0edd17be383ac7033401bb5d05c1d3050e538f82ac5b3731d04eb2daf54b10b5 |
| p1319-ab-freeze.json | 61d7c5005e889ae0857230f27a4ca8db5ecc38734abfe8699418459c37a8908f |
| p1319-ab-baseline-runs-r1.json | 6c8d67329108be7869375e4ee073d14e06e91217ed5e4291cbc1e7c8fa2eb04d |
| p1319-ab-candidate-runs.json | 903ad38bc4e1306e8637d2e7f801ae044d6f65fe5c3a25ce02d2c46c92ace31d |
| p1319-ab-comparison.json | 69219e4762054c05b3f04a861b4edca450e5f2e69aad67e044579b64ce44d5dc |

## Proveniência e abrangência congelada

As contagens desta seção pertencem ao baseline executado sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado,
de 2026-09-08T16:04:47.934032+00:00 até
2026-09-08T16:07:06.087292+00:00. O diff/stat foi:

```text
00_nucleo/prompts/compiler/stdlib/loading.md |  92 ++++++++++++++++-
01_core/src/compiler/stdlib/loading.rs       | 146 ++++++++++++++++++++++++++-
2 files changed, 232 insertions(+), 6 deletions(-)
```

Os JSONs conservam argv/cwd, horários por processo, exit/stdout/stderr
integrais, perfis e estado anterior/posterior. O binário baseline é o candidato
P1318 fixado por SHA, não o target de build do root.

São 645 casos: 513 históricos P1318 e 132 novos, nos perfis default, html,
a11y e html+a11y. Os 2052 replays históricos foram confrontados integralmente
com a execução P1318 antes de qualquer transformação, inclusive expressão,
cwd e observação; não houve deriva. Oito casos históricos de Path/Str com
buffer inválido recebem o delta P1319 explicitado antes do patch.

O freeze contém 2580 expectativas: 432 RED e 2148 preservações literais.
Das RED, 400 exigem diagnóstico vanilla integral. As outras 32 são normativas:
detached conserva o stderr baseline acrescentando o sufixo; excesso usa o
diagnóstico vanilla da sonda sem excesso com a expressão original restaurada
na linha exibida. Essas sondas separadas têm argv e saídas registrados. O
vanilla original dá resolução detached ou excesso, respectivamente; não se
alega paridade desses casos. A causa do baseline deve coincidir integralmente
com a causa da sonda antes do sufixo, preservando seu erro vencedor.

Cobertura inclui Utf8 no cabeçalho e nos dados, ambos os row-types,
UnequalLengths anterior ao byte inválido e no mesmo registro, LF/CRLF/CR,
separador Unicode, caracteres multibyte, campos citados multilinha, BOM,
linhas iniciais vazias, Path/Str normalizados e absolutos, vpath com
subdiretório, With/Args/spread/sink/alias/named anterior, detached e excesso.
Controles preservam Bytes/P1318, texto válido, valores, opções/casts/unknown,
I/O/resolução, read, outros loaders e encoders históricos.

Exemplos congelados: data-lf.csv dá 2:1; data-crlf.csv dá 1:1. O arquivo
unequal-ls.csv conserva `found 1 instead of 2 fields in line 2` e recebe
`in unequal-ls.csv:1:1`, embora o byte inválido ocorra depois do erro vencedor.
O caminho `path("sub/data.csv")` recebe `in sub/data.csv:2:1`, conservando o
diretório. Esses números são observações diagnósticas, não uma promessa de
posição intuitiva do byte inválido nem comparação de estrutura Rust.

## Calibração e orçamento

Uma execução baseline/vanilla completa produziu 3028 processos. O primeiro
freeze bloqueou quatro sondas exploratórias de import via eval: o baseline
dá `current file is outside its sandbox root` antes de chegar ao CSV; o
vanilla chega ao parsing. Isso não foi contado como RED P1319. O erro já
conhecido permaneceu como controle literal, explicitamente sem cobertura da
base capturada entre arquivos. Quatro sondas diretas `sub/data.csv` foram
adicionadas e executadas focalmente: 32 processos, 16 novas observações RED.
Os 16 observáveis de sandbox passam a ter expectativa de preservação
justificada pelo estrato medido; nenhum deles prova parsing ou paridade.

O bruto p1319-ab-baseline-runs.json e runner/cases r0 permanecem intactos.
p1319-ab-calibration.py registra a hipótese, causa
EVAL_IMPORT_SANDBOX_BEFORE_CSV, novos casos, ausência de regressões e
proveniência focal; p1319-ab-baseline-runs-r1.json só agrega essa evidência.
Não houve segundo corpus baseline completo. Custo agregado dos dois lotes:
3060 processos e 138,627 segundos registrados nos recibos, não benchmark.
O contador de revisões do freeze foi preenchido com 1 e com o detalhe do
recibo canônico antes da comunicação do hash final; o default zero do gerador
é explicitamente substituído por essa metadata. Nenhuma expectativa mudou
nessa correção de contador.

O runner reutiliza as funções de execução/comparação P1318 como ferramenta,
mantendo os artefatos antigos imutáveis. Por isso os schemas de runs/compare
mantêm a identificação histórica; paths, hashes, freeze e casos identificam
inequivocamente esta execução P1319. O writer usa split('\n'), preservando
separadores Unicode; o incidente histórico de splitlines não foi repetido.

## Verificação candidata

O candidato foi liberado pelo root com path/SHA acima, após o freeze e sua
revisão independente. Este testador iniciou o lote em
2026-09-08T16:25:01.381970+00:00 e encerrou em
2026-09-08T16:29:50.461642+00:00, no mesmo HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado:

```text
00_nucleo/prompts/compiler/stdlib/loading.md |  92 +++++++++-
01_core/src/compiler/stdlib/loading.rs       | 247 +++++++++++++++++++++++----
2 files changed, 306 insertions(+), 33 deletions(-)
```

Uma única execução candidata produziu as três ordens, 7740 processos e
289,096 segundos registrados. Todas as 2580 expectativas passaram nas três
ordens, sem instabilidade. O gate comparou exit/stdout/stderr integrais,
expressão, cwd, cardinalidade e ausência de duplicatas. Revalidou os hashes
de todos os inputs e L0 normativo; checks adicionais antes/depois confirmaram
o SHA do próprio freeze, baseline e vanilla. A identidade candidata foi
verificada antes/depois da execução. Nenhum caso ou expectativa foi alterado
depois do freeze, e não houve novo corpus integral após PASS.

O total A/B foi 10800 processos e 427,723 segundos de duração agregada dos
lotes, não benchmark. O PASS limita-se às versões, casos, perfis e observáveis
registrados; o veredito arquitetural pertence à autoridade revisora.

## Reprodução sem sobrescrever evidência

Na raiz do repositório, com os executáveis e fixtures preservados no host,
usar nomes de saída novos. Os sufixos reproduction-01 abaixo devem estar
livres; escolher outro sufixo se já existirem. Não rodar prepare/freeze de novo
sobre os artefatos canônicos.

```bash
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1319-ab-runner.py run \
  --binary /tmp/p1319-target.VqXtmj/release/typst \
  --binary-sha256 37a8a23d6e2b5daf355d90d510bca7efb399547730e1d807e8ae871be75748bd \
  --orders normal,repeat,reverse \
  --output 00_nucleo/diagnosticos/p1319-ab-reproduction-01-runs.json
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1319-ab-runner.py compare \
  --freeze 00_nucleo/diagnosticos/p1319-ab-freeze.json \
  --measurement 00_nucleo/diagnosticos/p1319-ab-reproduction-01-runs.json \
  --output 00_nucleo/diagnosticos/p1319-ab-reproduction-01-comparison.json
```

## Limites

A CLI baseline eval não expõe package-path; Package não foi testado neste
A/B. O import eval bloqueado também impede reivindicar base capturada entre
arquivos. Essas obrigações, decode_csv puro, Args Rust sintético sem
occurrences, offsets impossíveis/fallback/overflow, contagens de World e
parser único requerem validação local/auditoria por outra autoridade.
Este testador não leu nem julga essas provas. Unknown continua bloqueante
para identidade ambígua, entrada ausente/duplicada, construção não explicada,
timeout/crash ou deriva; não vira PASS por default.
