# P1330 — revisão preliminar independente

Revisor: agente `/root/p1330_review`, em ambiente compartilhado com o coordenador.
Regime A/B executado sem atestação de isolamento e sem selo de refinamento.
Entradas: instruções AGENTS fornecidas, CLAUDE raiz/01_core, skill
`/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md` e suas duas
referências, ADR-0107/0108/0127/0129, L0 calc completo, fontes baseline calc,
Args/call_dispatch/math e fontes vanilla calc/Args. Busca textual das ADRs não
encontrou ADR específica de materialização segregada. Contexto herdado inclui
o pedido do coordenador e seu diagnóstico preliminar checked_abs; esta revisão
reverificou a fonte. Escrita autorizada somente em novos `p1330-review-*`.
Nenhum produto, L0, teste ou oráculo foi alterado por este revisor.

## Proveniência das observações

HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
Consulta dos hashes de fontes em `2026-09-09T13:05:49Z`; ainda coincidem com
o inventário do baseline recebido. Medições CLI abaixo são as recebidas do
coordenador, não uma repetição independente de execução.

| Entrada | SHA-256 |
|---|---|
| p1330-baseline.json | a3f732bfb2fda2f177dbf3b33caddcd84b219af6fac22f96eae9644e784b6987 |
| p1330-baseline-public.json | c2f6500ed2f98fe2e37875b476fdb8570483ad516662cb804c4960c18a0781c2 |
| prompts/compiler/stdlib/calc.md | 69a23e0769e9bf7e6b7acbd4a62201f642f1065a991211e5481aa6eace647d7f |
| 01_core/src/compiler/stdlib/calc.rs | 028017afa2f034409b437389469bea8782106a602affbea2531dd139d49d2d54 |
| 01_core/src/entities/args.rs | f3e77a9bf2d038cbede40c8b5304dd00cb18f7c2f5b5cd9d53822a44ba7e0c4b |
| 01_core/src/compiler/eval/call_dispatch.rs | 9b11c388cf50666beecad9a6c92fbebfb323cdf5edbb550f4be04a96e0c3e9d4 |
| lab/typst-original/crates/typst-library/src/foundations/calc.rs | 0eb0836dc8bf17ba214d04be2c14f137350766ec44b6f5afe238cf467f78f484 |
| lab/typst-original/crates/typst-library/src/foundations/args.rs | b681b149809326f2479b99966232680771f8d95a170c82180d30cbd22274849b |

O baseline contém snapshot/diff/stat/inventário e fontes originais; a projeção
pública contém argv, horários e saídas integrais das 50 execuções recebidas.
UTC inicial do estado: `2026-09-09T13:04:16.518942+00:00`.
O executável cristalino é `/tmp/p1329-target.bg3p5A/release/typst`, SHA-256
`9f347f742a5cdb5c4c36a4af985b4ff122ac1bb118a1e018b960e7f5d105c2ec`.
Vanilla ratificado `a51e02804`: `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Nenhuma string de versão foi usada para identificar o alvo.

Diff/stat anterior exato recebido (a primeira linha abrevia o prefixo;
o caminho completo é `00_nucleo/prompts/compiler/eval/bindings/field_access.md`):

```text
 .../prompts/compiler/eval/bindings/field_access.md | 187 ++++-
 00_nucleo/prompts/compiler/eval/call_dispatch.md   |  53 +-
 00_nucleo/prompts/compiler/eval/modules.md         |  47 +-
 00_nucleo/prompts/compiler/eval/tests.md           |  78 +-
 00_nucleo/prompts/compiler/stdlib/calc.md          | 173 ++++-
 00_nucleo/prompts/compiler/stdlib/loading.md       | 198 ++++-
 00_nucleo/prompts/wiring.md                        | 104 ++-
 01_core/src/compiler/eval/bindings/field_access.rs | 731 +++++++++++++++++-
 01_core/src/compiler/eval/call_dispatch.rs         |  92 ++-
 01_core/src/compiler/eval/modules.rs               |   5 +-
 01_core/src/compiler/eval/tests.rs                 | 236 +++++-
 01_core/src/compiler/stdlib/calc.rs                | 853 ++++++++++++++++++++-
 01_core/src/compiler/stdlib/loading.rs             | 466 +++++++++--
 04_wiring/src/main.rs                              |  50 +-
 14 files changed, 3166 insertions(+), 107 deletions(-)
```

## Medição da fonte e do fragmento observável

1. `calc.rs:139-141` cristalino executa named antes do match de aridade e
   satura MIN em MAX. Vanilla `foundations/calc.rs:86,1365-1366` usa
   checked_abs e erro exato `the result is too large`. Vanilla
   `foundations/args.rs:118-120` ancora o cast em `value.span`, enquanto
   `finish:259-264` só trata argumentos restantes após o consumo.
   O baseline `calc.abs(-9223372036854775807 - 1)` devolve MAX no cristalino
   e erro no vanilla. Isto é semântica da linguagem e diagnóstico, não
   necessidade de copiar o mecanismo de cast Rust.
2. O L0 baseline manda preservar saturação em P1328, P1329 e na tabela
   de funções base. P1330 precisa substituí-la explicitamente em todos esses
   pontos antes de código. Não basta acrescentar uma seção contraditória.
   `calc.rs:1713` contém o controle A/B legado MIN → MAX; necessita sucessão
   explícita pelo autor A/B, preservando os demais casos e o snippet histórico.
3. `Args:19-20,66-79` distingue occurrence.span/value_span e mantém detached
   em Args sintético. `call_dispatch:406-439` conserva origem real: Array
   spread usa o span do spread, spread de Args conserva as ocorrências.
   `merge_with_args:1061-1064` concatena pré-argumentos antes dos novos e
   mantém suas origens, apesar do span agregado ser da chamada final.
4. `trace_call:1031-1049` só omite trace quando a chamada contém o erro.
   O baseline vanilla With e spread de arguments ancora a criação anterior
   do valor e acrescenta trace `abs`; a função cristalina está registrada
   como `calc.abs` em calc:58. É inferência da fonte que o novo erro acionará
   a dívida nominal existente nessas rotas. Deve ser congelada e conferida
   integralmente antes/depois de C, não removida ou normalizada no comparador.
5. MIN com `bad: 1` retorna named no cristalino e overflow no vanilla;
   MIN com segundo posicional retorna aridade no cristalino e overflow no
   vanilla. São diferenças medidas de precedência que ficam preservadas
   no escopo informado. Não apresentar esses casos como paridade.
6. Literal `-9223372036854775808` é rejeitado pelo parser vanilla com
   `cannot write minimum integer manually`; o cristalino o avalia como
   Float. Não mede o mesmo domínio Int de abs. Decimal dessa magnitude
   é aceito pelos dois e deve permanecer Decimal. Usar a expressão aritmética
   acima ou Int nativo para testar overflow; corrigir parser/int.min
   exigiria outro escopo. A saída JSON `null` de infinito não prova o valor
   e precisa de teste semântico/nativo para distinguir NaN de Inf.

## Decisão preliminar e limites

O recorte é correção interna de paridade ADR-0127 contínua, desde que se
limite ao único Int não representável depois dos guards atuais e preserve
assinaturas/entidades/pipeline. A evidência é suficiente para atualizar
primeiro o L0; ainda não aprova L0 novo, RED, candidato ou fechamento.
É inferência que apenas este owner basta. Necessidade de alterar Args,
dispatcher, operadores, parser ou fase refuta essa suficiência e exige
revisão de escopo antes de expansão.

Aceitação a revisar: Error único com texto exato, sem hints/trace nativos;
primeira ocorrência posicional value_span ou detached, sem fallback ao
agregado/occurrence.span. Incluir âncoras distintas, primeira ocorrência
detached mesmo quando houver outra origem resolvível, UTF-8/múltiplas linhas,
alias, With, Array/Args spread, positivos vizinhos/zero/MAX, Float/Decimal,
e named/aridade com MIN. Preservar integrais P1328 e P1329, incluindo math
content, comprimento misto, não finitos locais dimensionais e sua exclusão
do domínio de paridade. Perfis e execução CLI completa/repetida/invertida
complementam a semântica; Unknown obrigatório não satisfaz aceitação.

Não há selo, mutation score, alegação de isolamento técnico ou paridade
geral de calc.abs/calc. Os dois agentes possuem acesso físico ao workspace;
a segregação aqui é de autoria/escrita declarada. Limitação operacional:
um `git status --short` inicial exibiu incidentalmente nomes históricos
em materialization; não foram abertos nem usados como contexto. As consultas
seguintes ficaram nos caminhos autorizados. Nenhuma pasta restrita foi
varrida com ferramenta de leitura de diretório.
