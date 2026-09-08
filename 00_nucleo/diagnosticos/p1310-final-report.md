# P1310 — erros de tipo na fonte dos decoders

Implementado e aprovado no fragmento especificado. Veredito independente:
`PASS_SCOPED_WITH_DISCLOSED_AB_TEMPORAL_LIMITATION`. A limitação temporal da
revisão da suíte está descrita abaixo; não houve bloqueio funcional pendente.

## O que mudou

O ramo comum de tipo inválido em `cbor`, `json`, `toml`, `xml` e `yaml` agora
emite a lista completa dos tipos esperados e o nome público do tipo recebido,
apontando para a origem do valor. Exemplo concreto:

```text
json(42)
antes: json() requer caminho (str) ou bytes, recebeu int
       span primário detached
agora: expected path, string, or bytes, found integer
       span primário em 42
```

A implementação usa a primeira ocorrência posicional de Args para obter
`value_span`; não usa o span agregado, não escolhe um named anterior e não
inventa localização quando a origem é detached. O nome longo vem do formatter
canônico já existente. Somente o ramo inválido de `resolve_data` mudou:
assinaturas, tipos admitidos Path/Str/Bytes, parsing e I/O ficaram intactos.

Passo escrito em `00_nucleo/materialization/typst-passo-1310.md`; L0 proprietário
atualizado antes do código em `00_nucleo/prompts/compiler/stdlib/loading.md`.
Owner: `01_core/src/compiler/stdlib/loading.rs`. ADR-0127: correção interna de
paridade em fluxo contínuo; não houve novo contrato público.

## Evidência e limites

Quatro testes unitários novos distinguem nome longo, span do valor, ausência
legítima de origem e preservação das entradas válidas/argumento ausente.
Antes do patch: três falhas reais e um controle aprovado. Depois: quatro
aprovados. Os recibos `p1310-unit-red.json` e `p1310-unit-green.json` conservam
comandos, diff/stat, UTC e saídas integrais; RED foi falha de assert, não de build.

O replay fresco do contrato P1308 manteve **1962 observações** e mudou somente
**20**: os cinco casos `decoder.<formato>.wrong-type` nos quatro perfis.
Essas vinte saídas passaram a coincidir com o envelope vanilla pinado.
O recibo antigo retorna `Violated` para elas porque esperava o defeito anterior;
`p1310-p1308-delta.json` discrimina nominalmente esse fechamento das regressões.
Zero Unknown. Preservação de envelopes P1308 não significa paridade geral.

### O que continua aberto

- Argumento ausente, named/excesso e sua ordem de validação continuam como
  antes. Não houve alteração em `call_dispatch`, `read`, `csv` ou encoders.
- **Symbol não foi corrigido como fonte.** A medição independente revelou que
  o vanilla converte Symbol em string e tenta ler um arquivo. O cristalino
  continua rejeitando-o; seu diagnóstico recebe o novo texto e `value_span`
  pelo ramo comum. Esse delta foi explicitado no L0 antes do candidato e tem
  expectativa normativa separada, nunca contado como igualdade vanilla.
- As 37 famílias de mutação produtiva pendentes de P1307 continuam pendentes.
  Nenhuma mutação de produto foi executada, e não se publica score de mutação.
- Não se reexecutou o inventário global P1309 nem se recalculou percentual de
  paridade global. Este fechamento é do fragmento diagnóstico medido.

## Testes independentes e correção do próprio teste

A skill de materialização segregada orientou o regime A/B proporcional:
`/root` escreveu intenção, testes locais e código;
`/root/p1310_tests` escreveu testes CLI sem ler o patch;
`/root/p1310_review` revisou sem editar os artefatos julgados.
Regime: **executado sem atestação de isolamento técnico**. Não é um selo do
protocolo completo de refinamento.

O catálogo independente cobre tipos diversos, With, alias, spread de Array e
Args, sink, filter, map, join, multilinha e controles válidos Path/Str/Bytes.
Args interno sintético sem ocorrências é coberto pelo teste unitário; não é
apresentado como caso construído pela CLI.

Houve uma falha na classificação da suíte R1: três grupos com named-prefix
exigiam igualdade vanilla, contrariando a exclusão de named já escrita no L0.
O primeiro replay registrou 976 Preserved e 60 Violated, zero Unknown.
O revisor identificou que o candidato preservava corretamente o comportamento
prévio nesses casos. Nenhuma mudança no produto foi feita para satisfazê-los.

R2 alterou somente a política dos 15 casos afetados, selecionando os envelopes
baseline medidos antes do patch. R1 e seu replay permanecem preservados.
**Essa correção de classificação aconteceu após existir o candidato**: não
se alega congelamento integral de R2 antes da implementação. O texto do L0 e
os dados literais usados na correção já eram anteriores ao candidato; a
verificação independente confirmou esse delta restrito. A calibração
focal passou em 120 observações, incluindo controles de fronteira.

O corpus R2 separa 195 casos de paridade vanilla, cinco de efeito normativo
Symbol e 59 de preservação baseline: 259 casos × quatro perfis = 1036 células.
As referências bilaterais foram medidas antes do candidato; as três ordens
do replay executam o candidato contra essas expectativas congeladas.
Saídas brutas são conservadas; o adapter R4 identifica a projeção de transporte
CLI e mantém mensagem, hints, origem resolvida e traces no envelope diagnóstico.

As três ordens R2 passaram: **1036/1036 por ordem**, zero Unknown e nenhuma
instabilidade. São 780 células de paridade corrigidas, 20 do efeito normativo
Symbol e 236 preservações. O recibo `p1310-ab-r2-stability.json`, emitido em
`2026-09-08T00:32:49.410080Z`, tem SHA-256
`09aaaf3de40153dca198daf4168ff653a0efed93cf6f5d2cd750de248defe55b`.
O revisor conferiu igualdade dos envelopes, hashes dos canais brutos,
completude das chaves, estabilidade e a origem pré-candidato dos dados R2.
Não encontrou achado acionável pendente no recorte. Emissão em
`2026-09-08T00:35:24.393047Z`:

- `p1310-verification.json`: SHA-256
  `82b78b6320822a2ea72c2690ae36dbe486179fbcd49ab2dd1c02dff2ecbe73df`.
- `p1310-review-evidence.json`: SHA-256
  `9c7f4a518b4e30db174bb7b3b1a288b8626c6652aec08152ac1300f8a567b286`.

## Gates e proveniência

Estado medido: HEAD `eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, working tree
não commitado. Diff produtivo exato nos recibos: somente `loading.md` e
`loading.rs`; arquivos novos de passo/evidência são separados no status.
Baseline `p1310-baseline.json`, SHA-256
`3a19c3b842c5bdcc2d4e6df3ea8778a407a9843b2fc5ec183f3b07620c25d65f`.

Vanilla ratificado upstream `a51e02804`, binário SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Candidato construído no target RAM dedicado
`/dev/shm/p1310-target.VeBP6Q/release/typst`, SHA-256
`7f110b464745b880c357a9676fa302995873df5e67c5127adf0229cd25c5c146`.
O target usa uma cópia do cache anterior; o build do candidato foi executado
novamente e o binário baseline P1309 não foi sobrescrito.

`p1310-gates.json` registra horários, pins e recontagem dos comandos:

- Build release, fmt e diff-check: exit zero.
- Workspace: **6624 passaram, zero falhas, três ignorados**.
- Linter: **zero erros**, 240 warnings e 1136 infos preservados, classificados
  por regra no recibo; exit zero não significa zero findings.
- Ownership/Núcleos válidos; header efetivo `ec1dd115`; `Hash do Código`
  `1f56a80a`, conferido sem a própria linha `@prompt-hash`.
- Todos os artefatos P1309 pinados no baseline preservados.

Doctests ignorados: `layout_with_introspector` em `compiler/layout/mod.rs:2779`,
`TagIntrospector::inject_pages` em `entities/introspector.rs:543` e
`TagIntrospector::inject_positions` em `entities/introspector.rs:521`.
Não foram retirados testes para produzir GREEN.

Sem stage, commit ou push. Nenhum passo seguinte foi escrito.
