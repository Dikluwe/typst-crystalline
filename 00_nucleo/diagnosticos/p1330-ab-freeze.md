# P1330 — freeze A/B pré-C

Regime: executado sem atestação de isolamento. Autor A/B `/root/p1330_tests`
recebeu a tarefa, regras do repositório, skill Tekt e referências completas,
01_core/CLAUDE.md, L0 vigente, manifesto R2, baseline pública e oráculos
históricos. Não leu calc.rs, patch, baseline privada nem RED privado. As
capacidades são disciplina de acesso no filesystem compartilhado, não um
sandbox atestado. Autor não implementa nem aprova a própria solução.

Entradas permitidas adicionais efetivamente lidas: APIs públicas Args e Decimal.
Outputs permitidos: somente novos `00_nucleo/diagnosticos/p1330-ab-*`, escritos
com apply_patch; rustfmt edição 2021 sobre os dois snippets novos. O script
assemble é scaffold histórico de autoria, não gerador normativo final; o snippet
final inclui controles adicionais Float/Decimal escritos diretamente antes deste
freeze. Nunca reconstruir o congelado por esse scaffold.

## Identidade causal

Freeze em 2026-09-09T13:13:31Z, anterior à autorização de C pelo autor A/B.
HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
A captura CLI iniciou em 2026-09-09T13:12:10.941866+00:00; seu JSON contém
git diff HEAD --stat integral, argv e hashes dos executáveis. A baseline pública
contém inventário produtivo e diff/stat da entrada; não se usa --version como
proveniência. O root conserva o snapshot privado integral.

| Entrada/artefato | SHA-256 |
|---|---|
| L0 calc.md, excluindo linha Hash do Código | `9a5d734e086297d49d80d567bdb5527919049de19b7aea11f43b4467a972d2db` |
| p1330-manifest-r2.json | `fa90e9d05855842467eb7baac85d81a157af03bb285b9c1d40df0faf1f660acf` |
| p1330-baseline-public.json | `c2f6500ed2f98fe2e37875b476fdb8570483ad516662cb804c4960c18a0781c2` |
| BASE /tmp/p1329-target.bg3p5A/release/typst | `9f347f742a5cdb5c4c36a4af985b4ff122ac1bb118a1e018b960e7f5d105c2ec` |
| VANILLA /usr/local/bin/typst, ratificado a51e02804 | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| p1329-ab-tests-r1.rs | `567f8d4eda2fa5313af8449323248d814621cb47e5122a4ab19842dbf93fa838` |
| p1329-ab-p1328-successor.rs | `1fb3d1c0b6ddfd486aa1cfd4cbdaffbdf2ecd774021b5512d217b769e92c3a7c` |
| p1329-ab-cli-r2.py | `ee0d6a5da2283d50b0591a24db58f93a8300091729b6d6f2ad48e3457b393ab2` |
| p1329-ab-cli-expected-r2.json | `9501ae018ada17565bfc54227c244bea9af99e3988c9a212b262e92a0ee7c239` |
| p1330-ab-tests.rs | `6ed8f3af58855a88c9ea15113b38c2dad6029639448b052671b4effed4f5ca59` |
| p1330-ab-p1328-successor.rs | `56edc0c1b8c92d4e25bcfb73d1c3972526ea968f0647ea9462b659c2f517b47f` |
| p1330-ab-cli.py | `d405464d3d0961f5c23d0d34648e8361281cfdf5399499eb11e30e35951e241c` |
| p1330-ab-cli-baseline.json | `8ac2d68d4be81b5e3b9ab6a46e1f8c7aea6469cfdcb421d69c328d5b52229b86` |
| p1330-ab-cli-expected.json | `7eefa5cf55e10b77631cf5ef8d68be2629892fa64309c1274f20d1305112695d` |

## Obrigações e medidas antes de C

O novo módulo tem oito testes densos: inteiros extremos e vizinhos, precisão
acima de 2^53, Float/Decimal acima do limite inteiro, origem nativa conflitante,
primeiro posicional após metadados named, origem detached/ausente/vazia, guards,
rotas reais alias/With/nested/spread/arguments, origens distintas e UTF-8/linhas,
warnings e math mantendo conteúdo. Diagnóstico exige quantidade, severidade,
mensagem, hints, trace e span; valores exigem espécie e magnitude. Nenhum teste
depende de igualdade estrutural Rust para provar o novo recorte.

O sucessor P1328 substitui somente duas antigas asserções de saturação: nativa
MIN passa a erro detached completo; expressão do mínimo passa a erro completo
na origem do argumento. Diff após rustfmt confirmou ausência de outras mudanças.
O módulo P1329 deve ser preservado integralmente pelo integrador; seus controles
Float/NaN/Inf/dimensões/mistos/conteúdo continuam obrigações vigentes, observando
a fronteira local-versus-paridade já congelada em P1329.

A CLI tem 83 casos × quatro perfis = 332 células. Dos 63 casos históricos,
248 células foram herdadas byte a byte e conferidas contra BASE; somente quatro
células overflow receberam sucessoras. Novos casos somam 20, sem multiplicação
artificial de uma mesma rota. BASE coincide integralmente com vanilla em 220
células e diverge do oráculo sucessor em 44, todas de overflow. Essas contagens
vêm do JSON baseline e seus pins acima; não são GREEN compilado nem prova geral
de paridade. O RED compilado permanece responsabilidade privada do root.

As novas expectativas de overflow vêm do vanilla pré-C. Três rotas de origem
pré-vinculada (`overflow-with-bound`, `overflow-with-nested`,
`overflow-arguments`) congelam literalmente `calc.abs` no trace externo em vez
de `abs`, dívida do dispatcher já estabelecida por P1329 e L0. A construção dessa
expectativa ocorre uma única vez antes C; o comparador não normaliza nenhum
output. O literal mínimo direto mantém Float cristalino versus erro parser
vanilla. Named/aridade mantêm os guards cristalinos anteriores ao overflow,
incluindo named antes da aridade. Essas diferenças são dívidas, não Unknown
silenciosamente aceito e não claims de paridade.

Autoria teve uma passagem sem falha na captura/freeze; nenhum ajuste foi guiado
por output candidato. Budget proporcional: baseline integral uma vez; gate
final normal/repetido/invertido uma vez; eventual falha exige primeiro recorte
focal e hipótese pública. Duas revisões sem ganho na mesma causa exigem rever
contrato/observabilidade antes de outra execução integral. Unknown obrigatório
bloqueia fechamento. Alteração em entrada protegida invalida este freeze.

## Execução pelo integrador após C

Integrar cegamente os dois snippets, mantendo o módulo P1329 e o restante do
owner; executar RED antes de C e os mesmos testes GREEN após C. Rodar o runner
congelado três vezes (saídas novas, nunca sobrescrever):

```sh
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1330-ab-cli.py --candidate /tmp/p1330-target.f0lmDu/release/typst --output 00_nucleo/diagnosticos/p1330-ab-cli-normal.json
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1330-ab-cli.py --candidate /tmp/p1330-target.f0lmDu/release/typst --output 00_nucleo/diagnosticos/p1330-ab-cli-repeat.json
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1330-ab-cli.py --candidate /tmp/p1330-target.f0lmDu/release/typst --order reverse --output 00_nucleo/diagnosticos/p1330-ab-cli-reverse.json
```

O autor A/B aguarda somente os recibos CLI públicos para auditar literalmente
os outputs e a estabilidade; não lerá runtime, patch nem RED privado. Build,
workspace, fmt, lint e linhagem finais pertencem ao root/verificador.
