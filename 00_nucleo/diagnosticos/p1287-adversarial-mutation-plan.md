# P1287 — plano adversarial de mutação

**Papel:** Adversário segregado `/root/ataques_p1287`  
**Regime:** protocolo completo da skill `tekt-materializacao-segregada`  
**Escopo:** poder discriminatório do harness/oráculos congelados; nenhum candidato  
**Meta:** pelo menos 12 mutantes semanticamente não equivalentes, todos rejeitados

## Capacidade e limites

Leitura autorizada: skill e duas referências, `AGENTS.md`, somente
`00_nucleo/materialization/typst-passo-1287.md`, manifesto/receipts de contrato,
baseline e oráculo, e os quatro artefatos congelados enumerados abaixo. Escrita
autorizada: somente este plano e o receipt adversarial. Mutantes existem apenas
em mirrors sob `/tmp`; nenhum artefato protegido é alterado.

Não ler `01_core`–`04_wiring`; não executar `target/debug/typst` nem
`target/release/typst`; não corrigir contrato, runner, testes, fixtures,
baseline, L0 ou produção; não emitir veredito global.

## Entradas congeladas

| Entrada | SHA-256 |
|---|---|
| manifesto P1287 | `5d4cc9a6180300c8402be4a91b30104db08874f8540c9bc8af5b895a9fdf725b` |
| receipt de contrato | `4683d23a5a489e258931ea3389be809c7b4e5b143c4f7a66ec7210c4dbd4f7a5` |
| receipt baseline vanilla | `3bbba47235e4865bb3c9e788bb709215dd0a53297ee587d4334a01ca42126cd6` |
| runner | `745a4c470704de54f9a550d220a3092bfa8452e3b02cb7010846758eab36bd37` |
| testes | `3ac4daa6ff8cb2c9d8396ec4288b737c8496d0473c5d5d901165574af241c535` |
| baseline de oráculo | `ff09c8146b73684b2e746c631a6103817b6df7070aa8248e2d88dbca34749488` |
| receipt de oráculo | `b8ba1fa8d3d7bf1a305deee400ed811b5e034888ec4aeb3797392a95ea97e109` |

## Desenho da campanha

Cada mutante usa mirror independente com runner, testes, baseline, manifesto e
12 fixtures copiados. A injeção é uma única substituição textual exata no runner;
M03, M04 e M06 adicionam também drift apenas na cópia protegida correspondente.
O mutante precisa importar, falhar um probe que materializa a obrigação e, após
restauração a partir do original, o mesmo probe precisa passar. Falha de import
não conta como kill válido.

| ID | Decisão semanticamente corrompida | Probe discriminatório |
|---|---|---|
| M01 | `Unknown` no baseline vira `Preserved` | lattice exige `Unknown` |
| M02 | `Unknown` no candidato vira `Preserved` | lattice exige `Unknown` |
| M03 | drift do manifesto é ignorado | bytes alterados devem gerar `RuntimeError` |
| M04 | drift do baseline é ignorado | bytes alterados devem gerar `RuntimeError` |
| M05 | pin da suíte é ignorado | SHA de suíte falso deve gerar `RuntimeError` |
| M06 | drift de fixture é ignorado | C01 alterada deve gerar `RuntimeError` |
| M07 | E10 é omitido do inventário | lista deve ser exatamente C01–C12/E01–E11 |
| M08 | `Unknown` bilateral/malformado vira `Preserved` | bilateral opaco continua `Unknown` |
| M09 | PNG descarta alfa, criando tolerância geral | diferença só no alfa deve ser `Violated` |
| M10 | PDF colapsa fonte e acessibilidade | `fonts`, `marked`, `structure_tree` separados |
| M11 | ordem reverse é ignorada | desacordo forward/reverse deve ser `Unknown` |
| M12 | carrier parcial opaco recebe crédito | E04 parcial válido continua `Unknown` |

## Critério e linguagem permitida

`mutation_score = mutantes válidos mortos / mutantes válidos`. O gate local
exige `1.0`. Qualquer sobrevivente válido impede o selo adversarial; mutante
inválido não entra no denominador. O resultado atesta somente estas 12 decisões
do fragmento observável congelado, sem equivalência funcional geral e sem
veredito de paridade.
