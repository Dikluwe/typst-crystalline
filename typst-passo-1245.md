# P1245 — especificar o gap público de tiling sem implementá-lo

**Estado:** FECHADO — MATERIALIZADO E CERTIFICADO VIA P1254  
**Predecessor:** P1244  
**Saída:** proposta L0/ADR pronta para decisão humana, sem código incompatível.

Para conteúdo arbitrário, offset e angle ausentes, medir assinaturas e semântica
vanilla, inventariar consumers e demonstrar qual alteração pública mínima seria
necessária. Comparar opções de engenharia pelo observável da linguagem, não pela
estrutura Rust. Verificar ADR-0127/0129 e ownership 1:1.

Este passo deve parar após diagnóstico e rascunho L0 quando houver campo, método,
default ou fase nova. Não ressellar como aprovado nem implementar durante a
ausência do dono.

## Resultado

O contrato cristalino atual representa corpo fechado Image/Gradient/Color,
`size`, `spacing` e `relative`, mas o construtor rejeita Gradient e não carrega
Content arbitrário, `offset` ou `angle`. O vanilla usa frame de conteúdo,
size+spacing e transform derivado de offset+angle.

A opção proposta mantém `Tiling` como valor público declarativo L1 com
`Content`, size, spacing, offset, angle e relative. A fase de layout existente
materializa uma representação estritamente privada; não existe novo contrato
público `ResolvedTiling`, e exporters não repetem layout. Foram inventariados
17 consumers produtivos e três correspondências textuais excluídas. Quatro
opções foram comparadas; contrato resolvido público, resolução por exporter e
rasterização precoce permanecem rejeitadas na proposta.

Os L0 proprietários `entities/tiling` e `stdlib/tiling` foram propostos antes
do código. O dono confirmou explicitamente os dois hashes registrados no recibo
`p1245-owner-confirmation.tsv`, satisfazendo o gate humano ADR-0127. O texto
histórico dentro dos L0 que ainda diz “não confirmado” não prevalece sobre o
recibo posterior e byte-exato; ele foi preservado para não fabricar novo hash
sem nova aprovação.

O pré-selo segregado da decisão foi produzido no P1254 com cobertura C01–C15 e
18/18 ataques discriminados. A implementação produtiva não pertence ao P1245:
o próprio P1254 exige criar primeiro o Prompt L0 proprietário do consumer de
layout. Escrever esse código dentro deste passo violaria a trava L0 e
ADR-0129. Assim, P1245 fecha a decisão pública; P1254 continua como autoridade
para materialização RED→GREEN e certificado final.

## Fechamento produtivo (2026-08-28)

P1254 concluiu a materialização aprovada: `Tiling` aceita Content arbitrário,
size/spacing/offset/angle/relative; o layout resolve o corpo uma vez e produz
grupos repetidos e recortados consumidos pelo pipeline normal. `relative`
parent/self preserva fases distintas e offset é aplicado antes da rotação.

O pré-selo revisado obteve score `1.0`; o certificado final independente é
PASS. A prova executável percorre Content tiling → layout real → página → SVG,
com landmarks vermelho/azul ordenados, repetição, clip e duas saídas
byte-idênticas. PDF, HTML e stroke tiling permanecem `Unknown` exatamente no
seu target/papel, sem promoção cruzada.

Certificado: `00_nucleo/diagnosticos/p1254-final-certificate.tsv`.

## Fechamento saneado anterior (2026-08-28)

- hashes L0 aprovados e novamente verificados: `fef0967145b237b4...` e
  `080487e1bc80b7a...`;
- pré-selo revalidado independentemente: C01–C15, A01–A18, score `1.0`;
- `Unknown` não promove paridade e permanece adjudicado por target;
- nenhuma implementação candidata foi lida pelo verificador;
- `git diff --check`: PASS;
- hashes do recibo e pré-selo revalidados: `b06203c327a3590d...` e
  `5a43a6756ef03ba9...`.

Receipt: `00_nucleo/diagnosticos/p1245-closure-receipt.tsv`.

Artefatos: `00_nucleo/diagnosticos/typst-p1245-tiling-contract-gap.md`,
`p1245-tiling-consumers.tsv`, `p1245-tiling-options.tsv` e
`p1245-tiling-l0-draft.md`. Nenhum código, whitelist ou header produtivo foi
alterado.

Receipt: `00_nucleo/diagnosticos/p1245-contract-gap-receipt.tsv`.
