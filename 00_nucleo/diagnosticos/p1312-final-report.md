# P1312 — read: tipo inválido e origem do erro

**Implementado e aprovado no recorte read, com veredito independente PASS_SCOPED.**

## Correção e retificação da seleção

`read(42)` passa de `read() requer string com o caminho, recebeu int`, sem
origem primária, para **`expected path or string, found integer`**, marcando
somente `42`. O diagnóstico usa o nome público longo do tipo e a origem do
primeiro valor posicional, mesmo com named anterior ou argumento pré-ligado
por With. Origem legitimamente detached continua detached; não foi inventado
range com base na chamada.

Há uma retificação importante em relação à seleção P1309: **read e CSV não
têm o mesmo contrato**. O vanilla ratificado declara `Spanned<PathOrStr>` em
`loading/read.rs:28`, mas `Spanned<DataSource>` em `loading/csv.rs:30`.
`csv(bytes("a,b"))` retorna linhas no vanilla e ainda falha no cristalino;
`read(bytes("a,b"))` é inválido em ambos. Compartilhar o helper cristalino
não legitimava tratar os dois como uma mesma família path-only.

Por isso P1312 fecha apenas o diagnóstico de read. A seleção histórica não
foi reescrita nem a coorte inteira declarada concluída. CSV/DataSource/Bytes
fica para revisão própria. Fonte da decisão: `p1312-measurement.json` e
`p1312-selection.json`, produzidos antes do código, com binários pinados,
fontes literais, argv e estado completos.

## Implementação e fronteiras

Em `01_core/src/compiler/stdlib/loading.rs:1025`, `arg_read_path` é privado
e exclusivo de read. O helper CSV antigo permanece intacto. A única troca
na composição de read é a chamada desse helper; não mudaram Path/Str,
resolução/leitura, encoding, named/excesso, argumento ausente ou ordem de
validação. Nenhuma assinatura pública, entidade, dependência ou fase foi criada.
L0 proprietário atualizado antes do Rust: `loading.md`, amendment P1312;
hash de prompt `174f831b`, Hash do Código `33952fd4`.

Symbol continua sendo dívida: vanilla converte o símbolo a string e tenta
ler o arquivo; o cristalino continua rejeitando, agora com
`expected path or string, found symbol` e origem do valor. Esse efeito do
formatter é explicitamente testado, **não é paridade Symbol**. Named desconhecido
continua precedendo o cast no cristalino, mesmo quando vanilla escolhe outro
erro. As mensagens de missing e encoding também não foram corrigidas neste lote.

## Resultados medidos

RED local: três falhas reais de assertion (texto/tipo/span) e um controle
passando; GREEN: **quatro testes passaram**, sem falhas ou ignorados. O controle
cobre Path/Str válidos, missing e a rejeição CSV existente.

Teste independente: **840/840 comparações passaram**, zero falhas/Unknown,
nas ordens normal, repetida e inversa. São 70 casos nos quatro perfis:
29 alvos de paridade, três expectativas Symbol e 38 controles. As 280
expectativas distintas foram congeladas antes do patch; 116 observações
alvo e 12 Symbol eram RED no baseline. Fonte: `p1312-ab-freeze.json`, SHA-256
`440d7e744067dffab23371700bf421bf6dd2830c7d8a60cf26ccc3b6ea54f725`,
em `2026-09-08T01:39:22.046026Z`; comparação final em `01:49:13.283353Z`:
`p1312-ab-comparison.json`, SHA-256
`9849b0b27574deb7dcfb7546c3b91f2699e4c7f0ad907dcef7b014933374e44e`.
Saídas literais, fontes e horários de cada execução em
`p1312-ab-candidate-runs.json`; método e ajustes préfreeze em `p1312-ab-receipt.md`.

Os replays históricos conservam os oráculos antigos. No corpus P1310 há
**1.032 Preserved / 4 Violated / 0 Unknown**; no P1311, **216 / 4 / 0**.
Essas violações são exclusivamente os controles antigos `read.wrong` e
`read-loader-error` nos quatro perfis. Todas passaram a coincidir com saídas
vanilla literais medidas antes deste patch; não houve outro delta.

No P1308, o resultado bruto é **1.946 Preserved / 36 Violated / 0 Unknown**.
São as 32 correções anteriores mais quatro novas de `read.wrong-type`, todas
iguais ao vanilla pinado. Contra o candidato P1311, **1.978 envelopes ficaram
idênticos**. Os recibos `p1312-p1310-replay.json`, `p1312-p1311-replay.json` e
`p1312-p1308-delta.json` conservam os observáveis completos e isolam os deltas;
não apresentam esses replays históricos como integralmente verdes.

Build workspace release, fmt/check, diff/check e linhagem concluídos.
`crystalline-lint`: **zero erros, 240 warnings e 1.138 infos**. As duas infos
novas são V19/V20 para o padrão Path/Str do helper; warnings não aumentaram.
Não se declara ausência de dívida.

**Incidente de validação:** a primeira suíte completa terminou com exit 101:
`pdf_pre_scripts_contem_bt_et` e `pdf_pre_scripts_dos_lados_nao_vazio`
falharam com `unclosed delimiter`, em `03_infra/src/integration_tests.rs:160`.
O recibo `p1312-workspace-tests.json` foi preservado. Os três testes focais
`pdf_pre_scripts_` passaram sem alteração de código, conforme
`p1312-workspace-focal.json`. As fontes desses testes não chamam read; seu
helper temporário usa apenas subsec_nanos (`integration_tests.rs:116-127`).
Colisão de fixture é hipótese plausível, **não causa comprovada**. A falha
intermitente não foi corrigida nem atribuída definitivamente neste passo.
A repetição completa do mesmo comando, sem alteração de candidato/testes,
terminou com **6.633 aprovados, zero falhas e três doctests ignorados**:
`p1312-workspace-tests-r1.json`, SHA-256
`c78e8441de7810eb2b20c5ca84a368a1eeea4fdd77642cdd1cbfd9a66eebe299`.
São ignorados antigos de layout_with_introspector e
TagIntrospector::{inject_pages,inject_positions}, não testes novos omitidos.
O gate final `p1312-gates.json` mantém a primeira falha e verifica os hashes
e diffs idênticos entre execuções, além de preservar 159 artefatos históricos.
Veredito independente **PASS_SCOPED**, sem achado acionável no recorte:
`p1312-verification.json`, SHA-256
`f2c9b6e18e6a4bd036ec3e740e096616a6b1a033829b99a0425891254dadd638`.
O revisor reconstruiu a fonte anterior removendo somente as adições P1312 e
obteve o SHA exato do baseline, confirmando CSV e lógica P1310 preservados.
Fundamentação e ressalva do incidente em `p1312-review-final.md`.

## Proveniência e limites

Todas as medições ocorreram sobre **working tree não commitado**, HEAD
`eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, já com P1310/P1311. Baseline
`p1312-baseline.json`, SHA-256
`0597c75b13da999990886b32577b563bab330d8d1be4e5bf85e97ff0f03588c5`.
Os recibos registram UTC, comando, fonte e diff/stat completo antes/depois.
Diff contra HEAD no candidato medido:

```text
00_nucleo/prompts/compiler/eval/bindings/field_access.md | 62 linhas de diff (P1311)
00_nucleo/prompts/compiler/stdlib/loading.md            | 125 linhas de diff (P1310+P1312)
01_core/src/compiler/eval/bindings/field_access.rs       | 175 linhas de diff (P1311)
01_core/src/compiler/stdlib/loading.rs                  | 306 linhas de diff (P1310+P1312)
4 files changed, 658 insertions(+), 10 deletions(-)
```

Medição inicial: `2026-09-08T01:27:03.008979–01:27:04.728488Z`;
RED: `01:30:47.060003–01:32:36.656417Z`; GREEN:
`01:43:25.264056–01:45:05.077168Z`; build:
`01:43:30.215678–01:45:38.427493Z`; workspace inicial:
`01:44:05.469284–01:48:21.190857Z`; focal:
`01:50:26.577016–01:50:26.838790Z`; workspace R1:
`01:51:07.806080–01:51:46.352731Z`, todos em 2026-09-08 UTC.

Vanilla ratificado upstream `a51e02804`, `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Baseline `/dev/shm/p1311-target.JE8Cyy/release/typst`, SHA-256
`4bbced9ec793fb84daa560e0d965958b33e1eb2f5ecd4bfd3bc30b5900bb09fb`.
Candidato `/dev/shm/p1312-target.B8uLa9/release/typst`, SHA-256
`446c3ccfa5ef7f5eaf6031543f4d28566dac6eb53dca842ec7eb16acb084fbf0`.
Target dedicado em RAM com cópia do cache e rebuild do workspace; não se
alega rebuild limpo de todas as dependências. Binário anterior preservado.

A skill `tekt-materializacao-segregada` determinou autoria independente dos
testes A/B e revisão distinta antes/depois do patch. Regime A/B proporcional,
**executado sem atestação de isolamento técnico**, pois o filesystem é
compartilhado. Não há selo completo, score de mutação de produto ou medição
nova de paridade geral. Nenhum stage, commit, push ou limpeza histórica.
