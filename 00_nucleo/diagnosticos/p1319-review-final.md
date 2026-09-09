# P1319 — parecer final independente

**Veredito: PASS no recorte P1319, com limitações documentadas.**

CSV nativo Path/Str com buffer inteiro inválido em UTF-8 agora conserva a
causa vencedora do parser, acrescenta caminho virtual e posição e usa a origem
do argumento. O diff respeita o owner loading, a pureza e os contratos atuais.
Não foi encontrado achado bloqueante no candidato final.

Regime A/B executado sem atestação técnica de isolamento. Root é autor da
obrigação, testes locais e código; `/root/p1319_tests` produziu oráculos sem
ler candidato; `/root/p1319_review` julgou sem editar produto ou oráculos.
Este parecer não é selo de refinamento, mutation score ou paridade geral CSV.

## Evidência e reprodução

Auditoria reproduzível `p1319-review-audit.cjs`, SHA-256
`427715bc906bf03e79c86f6d0a18d96f23fd8dbb4f0cbcbf2167758818e63ed5`.
Saída final `p1319-review-final-audit.json`, SHA-256
`43972448740d8373dbf50c45d75baea66ac258e45bc334e008ebbd243c563d45`,
status `PASS_WITH_DOCUMENTED_LIMITATIONS`, sem pendências. O JSON preserva
horário, hashes de gates, argv e proveniência. Comando somente leitura:

```bash
git show d31047d7b8af7837c84adae4ded3d2ff50c62093:01_core/src/compiler/stdlib/loading.rs | node 00_nucleo/diagnosticos/p1319-review-audit.cjs 00_nucleo/diagnosticos/p1319-ab-candidate-runs.json 00_nucleo/diagnosticos/p1319-ab-comparison.json
```

O comando reconstitui a fonte RED a partir do commit e diff capturado,
confere seu SHA e exige módulo inteiro de testes idêntico ao candidato.
Recalcula norma, hash A efetivo com núcleo/pin e hash reverso B; revalida
todos os inputs do freeze; vincula os gates ao source/L0 final; compara cada
observação candidata integralmente à expectativa nas três ordens.

## Estado final examinado

HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
As únicas alterações rastreadas contra HEAD são:

```text
00_nucleo/prompts/compiler/stdlib/loading.md |  92 +++++++++-
01_core/src/compiler/stdlib/loading.rs       | 247 +++++++++++++++++++++++----
2 files changed, 306 insertions(+), 33 deletions(-)
```

Nenhum diagnóstico/oráculo histórico rastreado foi alterado. Todos os 57
inputs do freeze foram recalculados sem divergência, incluindo fixtures e
artefatos históricos. Os artefatos novos de calibração r0/r1 permanecem
identificados e intactos. O revisor não abriu materialization/context.

| Artefato final | SHA-256 |
|---|---|
| Source loading.rs | 15078b5441b514a200c54039589521cb4755b99a99a68b5a37983c247f759800 |
| L0 raw | 1bd9633f0cdfb5a1ef7ca0265b55ac7ea8ad5b28c41035a2aaaa3572edff0dd1 |
| Norma L0 congelada | da59f9964566dc99d3341145fe8200df15ff8df172158c79d91542d47015e823 |
| Source sem header de hash | ed0ff72c73b5e4a207250cbfcd203f8e5df54d1ff0609e65dffea1d238e34c00 |
| L0 efetivo com núcleo | 94cfda0e06d972d88be83cbdcecc3d42619babad509d8bc417e220994c0adbd6 |
| Binário candidato | 37a8a23d6e2b5daf355d90d510bca7efb399547730e1d807e8ae871be75748bd |
| Freeze A/B | 61d7c5005e889ae0857230f27a4ca8db5ecc38734abfe8699418459c37a8908f |

## Gates julgados

Todos os números desta seção derivam dos recibos finais cujos hashes e estados
before/after são registrados no JSON da auditoria, sobre o source/L0 acima.

- RED compilou: 65 testes passaram e quatro falharam pela ausência do sufixo
  contratado. Não houve alteração de testes após o candidato.
- GREEN local r2: 69 passaram, nenhuma falha ou ignorado.
- Workspace r2: 6661 passaram, nenhuma falha, três ignorados, exit 0.
- Build workspace release, fmt e diff-check r2: exit 0.
- Lint r2: zero erros, 240 warnings, 1137 infos, exit 0. Não se afirma
  ausência literal de todos os findings. O warning V16 acrescentado pelo
  candidato inicial foi eliminado explicitando os casos do enum; a fonte
  r2 difere da r1 somente nessa enumeração.
- V5/V15/V26 e cálculo independente bidirecional: corretos no estado final.
- A/B: 7740 comparações exatas, zero falhas/Unknown, normal/repeat/reverse.
  O auditor recalculou todas as comparações, sem confiar apenas no PASS do
  testador. Recibo de execução SHA
  `903ad38bc4e1306e8637d2e7f801ae044d6f65fe5c3a25ce02d2c46c92ace31d`;
  comparação SHA
  `69219e4762054c05b3f04a861b4edca450e5f2e69aad67e044579b64ce44d5dc`.

As 2580 expectativas congeladas são 400 correções de diagnóstico vanilla
integral, 32 correções normativas explícitas e 2148 preservações, exercitadas
nas três ordens. As correções normativas conservam dívidas de detached/excesso;
não recebem alegação indevida de igualdade vanilla.

## Limitação do linter e fronteiras restantes

O linter instalado apresentou falso negativo reverso: depois do primeiro
candidato, declarou `Nothing to fix` e V5/V15/V26 sem violações embora
`Hash do Código` ainda identificasse o RED. O fenômeno foi preservado no
recibo anterior ao reparo e explicado na revisão de linhagem. O autor corrigiu
apenas a metadata B; o revisor calculou A e B independentemente, inclusive
o núcleo, e confirmou o par final A=`94cfda0e`, B=`ed0ff72c`.

Isso compensa a limitação para este par final, não corrige a ferramenta
externa nem permite confiar novamente em `Nothing to fix` como prova reversa.
Nenhuma mudança no linter externo foi realizada neste trabalho.

UTF-8 válido permanece no comportamento legado: o range externo com FileId
exige investigação/contrato próprios. O candidato não usa include_path/source
como atalho. I/O, Symbol, validação unknown/missing/excesso, resolução causal
de Str e csv.encode continuam fora do recorte.

Package e chamadas World foram verificados localmente; parser único e
tratamento seguro de posição inexistente/impossível foram examinados no
source. O corpus CLI não prova base capturada cross-file: suas quatro sondas
de import falham antes de CSV e permanecem controles de dívida. As sondas
de subdiretório não foram apresentadas como substituto dessa prova.

O recorte P1319 pode ser encerrado com este resultado e esses limites. Não
resta trabalho obrigatório conhecido dentro da correção delimitada.
