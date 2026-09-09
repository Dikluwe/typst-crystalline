# P1331 — freeze A/B pré-C

Regime: executado sem atestação de isolamento; sem selo de refinamento.
Autor dos testes: /root/p1331_tests. Contexto inicial limitado à tarefa e regras
do repositório. Aplicadas skill tekt-materializacao-segregada e referências
papeis-e-capacidades.md e artefatos-e-gates.md completas, além de
01_core/CLAUDE.md e L0 calc.md completo. Busca limitada a ADRs por segregação
não localizou ADR específica. Não li runtime/patch calc.rs, baseline privada,
RED privado, materialization/context nem domain-probes contendo diff runtime.
O filesystem é compartilhado; disciplina de acesso não atesta isolamento.

Entradas adicionais efetivamente consultadas: oráculos históricos P1328/P1329/
P1330, APIs Args/Value/Rel/Path/Location/Locator e fonte vanilla ToAbs/CastInfo.
Somente novos p1331-ab-* foram escritos via apply_patch; rustfmt edição 2021
formatou os dois snippets antes deste freeze. Não executei cargo.
Autor B não implementa nem aprova a solução; root fará integração cega RED→GREEN.

## Proveniência e identidade

Freeze em 2026-09-09T13:46:40Z, antes de C.
HEAD d31047d7b8af7837c84adae4ded3d2ff50c62093, working tree não commitado.
Captura CLI começou em 2026-09-09T13:43:16.206842+00:00; o JSON registra
git diff HEAD --stat integral, argv, hashes de executáveis, versão normativa
e pins de entradas. Baseline pública identifica inventário produtivo; root
guarda o snapshot privado. Não uso --version como identidade da medição.

| Entrada/artefato | SHA-256 |
|---|---|
| L0 calc.md, sem linha Hash do Código | ef406128181e4cfa5b802b86ec0a9c58370cc4f7dab09365a088eef74523956a |
| p1331-manifest.json | 837bb832ce13f0686a438f672c6fe001224738a15b1a369efcc6f7533b7db141 |
| p1331-baseline-public.json | 621362a26b8d4d2553bbde444dc9bdfea6a0da98496b9e2a0dc9d60faafefe6c |
| BASE /tmp/p1330-target.f0lmDu/release/typst | 6f1db621bc0b2a7fe4fc9d05925fb96636f33232b8527b83c040b793970fbda0 |
| VANILLA /usr/local/bin/typst, a51e02804 | 7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8 |
| p1329-ab-tests-r1.rs | 567f8d4eda2fa5313af8449323248d814621cb47e5122a4ab19842dbf93fa838 |
| p1330-ab-tests.rs | 6ed8f3af58855a88c9ea15113b38c2dad6029639448b052671b4effed4f5ca59 |
| p1330-ab-p1328-successor.rs | 56edc0c1b8c92d4e25bcfb73d1c3972526ea968f0647ea9462b659c2f517b47f |
| p1330-ab-cli.py | d405464d3d0961f5c23d0d34648e8361281cfdf5399499eb11e30e35951e241c |
| p1330-ab-cli-expected.json | 7eefa5cf55e10b77631cf5ef8d68be2629892fa64309c1274f20d1305112695d |
| p1331-ab-tests.rs | 10e167749cd11bb450ec3d13cdc83330fe5c3dab5932588b8f9bba5fd448d6f4 |
| p1331-ab-p1328-successor.rs | 994f3dc7493e1230b8ceb41e53ef4c182e58120fe6f4e487637ede014edeea3a |
| p1331-ab-cli.py | 712b693f46b4bd52ce45159e2d4f8bfca8f8e277bea6a542cf0a2ff79aed3f0f |
| p1331-ab-cli-baseline.json | 6f73013c9e951ee146089eebd2267d412d742eeee051528a7427cea016d980bb |
| p1331-ab-cli-expected.json | 7c088f26f58b010409fcd2396dc0a77de1eaaeca6d0027eb91dca029c566f301 |

## Escopo congelado e medidas

Sete testes novos densos cobrem famílias rejeitadas, strings numéricas sem
coerção, Bool/Str e seus nomes longos, Relative zero/não zero, Path e Location
já construídos, metadados named sem named values, primeira ocorrência
posicional conflitante, ausência/vazio/named-only/detached, guards, chamadas
públicas, alias/With/nested/spread/arguments, UTF-8/linhas, warning completo,
math e espécies numéricas já aceitas. Diagnósticos exigem quantidade,
severidade, mensagem, hints, trace e origem. Location é construído por Locator:
o teste não afirma cobrir produção por introspecção ou mudança de fase.

O sucessor P1328 muda apenas mensagem/span/trace de string/symbol. A ramificação
de sqrt preserva as mesmas asserções anteriores; diff conferido após rustfmt.
P1329 e P1330 devem ser integrados integralmente sem nenhuma mudança.

CLI: 126 casos × quatro perfis = 504 células; 83 casos históricos + 43 novos.
324 células históricas foram herdadas literalmente e conferidas contra BASE.
Somente string/symbol × quatro perfis (oito células) mudaram. BASE coincide
com vanilla em 252 células e diverge do oráculo sucessor em 164. Esses números
vêm dos JSONs pinados acima; não são resultado de testes compilados nem prova
de paridade geral. O RED compilado é privado do root.

Três novas rotas (fallback-with-bound, fallback-with-nested,
fallback-arguments-spread) derivam uma única vez antes C a expectativa literal
vanilla com nome externo calc.abs em lugar de abs, dívida normativa do
dispatcher. O comparador não normaliza nenhuma saída observada. Guards e
path() sem argumento preservam BASE como dívida anterior ao fallback.

Uma hipótese de classificação foi corrigida com medição pré-C: o caso chamado
fallback-math-content contém string explícita em math e ambos os binários
tratam a entrada como string. Seu nome/metadata inicial permanecem no JSON
bruto para preservar a captura; o freeze classifica a observação como
fallback-correction, e o teste exige string. Não se generaliza isso para
números escritos em math; controles históricos de content permanecem.
A descoberta não alterou norma ou outputs medidos.

Custo de autoria: uma captura integral (1008 processos dos dois binários),
uma inspeção focal dos casos novos e duas sondas BASE de repr para controles.
Um erro de scaffold (nome NORM ausente) foi corrigido antes de iniciar a
captura; não teve observações ou impacto discriminatório. Uma revisão de
hipótese math corrigiu quatro classificações pré-C, sem regressão histórica.
Budget final: normal/repetido/invertido uma vez; em falha, recorte focal com
hipótese pública antes de repetir corpus integral. Duas revisões sem ganho
na mesma causa exigem revisão. Mandatory Unknown bloqueia fechamento.
Alteração em entrada protegida invalida este freeze. Nenhuma mudança em
testes/expectativas é autorizada após C.

## Execução e recibos

Comandos já executados antes C:

```sh
rustfmt --edition 2021 00_nucleo/diagnosticos/p1331-ab-tests.rs 00_nucleo/diagnosticos/p1331-ab-p1328-successor.rs
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1331-ab-cli.py --output 00_nucleo/diagnosticos/p1331-ab-cli-baseline.json
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1331-ab-cli.py --freeze-from 00_nucleo/diagnosticos/p1331-ab-cli-baseline.json --output 00_nucleo/diagnosticos/p1331-ab-cli-expected.json
```

Após integrar cegamente e verificar RED→GREEN, o integrador executa:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1331-ab-cli.py --candidate /tmp/p1331-target.rtY0la/release/typst --output 00_nucleo/diagnosticos/p1331-ab-cli-normal.json
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1331-ab-cli.py --candidate /tmp/p1331-target.rtY0la/release/typst --output 00_nucleo/diagnosticos/p1331-ab-cli-repeat.json
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1331-ab-cli.py --candidate /tmp/p1331-target.rtY0la/release/typst --order reverse --output 00_nucleo/diagnosticos/p1331-ab-cli-reverse.json
```

Autor A/B aguarda somente recibos CLI públicos para auditar saídas literais e
estabilidade; não lê runtime/patch/RED privado. Build, workspace, fmt, lint,
linhagem e veredito final pertencem ao root/revisor.
