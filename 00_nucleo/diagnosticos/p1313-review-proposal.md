# P1313 — revisão da proposta concreta

**Veredito: `READY_FOR_OWNER_REVIEW`; gate ADR-0127 continua pendente.**
Não foram encontrados achados que impeçam apresentar esta versão ao dono.
Este veredito julga a proposta, não aprova sua materialização nem certifica
uma implementação.

Revisor `/root/p1313_review`, 2026-09-08T11:30:15Z; mesmo regime e capacidades
do parecer `p1313-review-gate.md`: A/B sem atestação técnica, escrita apenas
em `p1313-review*`. O coordenador autorizou explicitamente nesta revisão a
leitura de `00_nucleo/materialization/typst-passo-1313.md`; nenhum outro
arquivo daquela pasta foi aberto.

## Entradas e verificações

- L0 `00_nucleo/prompts/compiler/stdlib/loading.md`, SHA-256
  `fe8e147fca053d9ebd9f9eeb4b6b4fa1bdf51346e00ea3f8576a6fdb38e9e4ed`.
- Passo `00_nucleo/materialization/typst-passo-1313.md`, SHA-256
  `b88fce4e98f39dcfb774a526d237a253d6187aad2aeb6ee18e0bd378e1cddda1`.
- Medição `00_nucleo/diagnosticos/p1313-measurement.json`, SHA-256
  `f07efa356c554a4b80a5702c516c4e7f7bee51f4b70fc8cc419ea6cf8754cb43`.
- Baseline `00_nucleo/diagnosticos/p1313-baseline.json`, SHA-256
  `0ad67b8f92c25160f044c352241572916716e2489ad105a6314608d51f59de92`.

Os hashes da medição e baseline coincidem com as referências da proposta.
Foram lidas as saídas bilaterais registradas, sem nova execução dos binários.
HEAD e diff da medição permanecem iguais entre before/after no recibo.
O owner `01_core/src/compiler/stdlib/loading.rs` tem SHA-256
`acdb71c8775765a6f54bcbd9925ec76d7493a6aac2b1675dcd7f1fac9e1cd8be`,
idêntico ao registrado no baseline P1313: não há patch produtivo P1313 nesse
arquivo na revisão. A seção de código nativo legada foi consultada para
conferir a ordem de validação, não para projetar testes contra um candidato.

HEAD `eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, working tree não commitado;
`git diff HEAD --stat` na revisão:

```text
 .../prompts/compiler/eval/bindings/field_access.md |  62 ++++-
 00_nucleo/prompts/compiler/stdlib/loading.md       | 205 +++++++++++++-
 01_core/src/compiler/eval/bindings/field_access.rs | 175 +++++++++++-
 01_core/src/compiler/stdlib/loading.rs             | 306 ++++++++++++++++++++-
 4 files changed, 738 insertions(+), 10 deletions(-)
```

## Fundamentação

A emenda apresenta medição antes da decisão. As saídas registradas sustentam
Bytes válidos, vazio, delimiter/row-type, With e cast inteiro; os erros
registrados também sustentam as fronteiras de parsing, opções, missing e
prioridade de named. A referência declarativa DataSource vem da fonte vanilla,
e sua observação bilateral está separada da intenção.

A ampliação de entrada é explícita e condicionada à aprovação. A proposta
mantém o conflito normativo explicado no parecer de gate, não reclassifica
silenciosamente a mudança como fluxo contínuo e não trata sua própria redação
como autorização. O passo confirma a mesma condição antes de testes L1 e
patch. A atualização futura da tabela vigente e a substituição estreita de
P1141/P1312 estão nomeadas, sem conceder efeitos imediatos à cláusula proposta.

A política observável é suficientemente concreta para apresentação: Bytes
alimenta o decoder existente sem World; Path/Str conserva resolução/leitura;
cast inválido aponta para o primeiro value_span posicional; origem detached
não recebe range inventado. A ordem de named/fonte/opções/dados/decode
coincide com a ordem legada inspecionada. A obrigação de opções inválidas
não causarem leitura antecipada impede que uma reutilização apressada de
helper altere incidentalmente essa ordem.

As rotas de parsing e opções recém-alcançáveis por Bytes recebem política
explícita legada. O texto não promete paridade dos seus diagnósticos e exige
congelar seus deltas antes do candidato. Symbol permanece dívida declarada,
com mudança de mensagem/origem normativa separada de igualdade vanilla.
Read/P1312, decoders P1310, campos P1311 e encoders estão protegidos. Assim,
aceitação de Bytes não é apresentada como fechamento integral de CSV.

## Limites da revisão

As expectativas independentes e controles de ausência de I/O ainda precisam
ser elaborados e congelados após aprovação, antes do candidato, como a proposta
determina. Em particular, mensagens/âncoras legadas para Bytes malformados não
podem ser escolhidas retrospectivamente a partir do patch. Não houve execução
de RED/GREEN, build, lint, replay ou confirmação humana nesta revisão.

Nenhum artefato julgado foi editado pelo revisor. O texto está apto para a
decisão do dono; implementação permanece bloqueada pelo gate documentado.
