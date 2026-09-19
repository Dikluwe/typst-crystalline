# Prompt L0 — `wiring/tests/p1462-format-svg`
Hash do Código: 52f77a04

**Estado:** CONTRATO DE TESTE P1462 CONFIRMADO — sem consumer e com hash
sentinela não selado até a materialização posterior ao novo selo R2.

**Camada:** L4 (testes de integração)
**Alvo planejado:** `04_wiring/tests/p1462_format_svg.rs`
**Consumer futuro exclusivo:** `04_wiring/tests/p1462_format_svg.rs`
**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Responsabilidade

Executar `CARGO_BIN_EXE_typst` como processo externo e provar o observável
integrado de `format.svg`. O harness não importa módulos privados, não
implementa parsing/precedência e não legitima código produtivo.

## Corpus e matriz

Cada caso usa diretório temporário isolado e exporta SVG válido de uma fonte
mínima. Cobrir:

- ausência de regra e de `--pretty` produz serialização compacta;
- `#set format.svg(pretty: true)` sem flag produz serialização indentada;
- regra `true` com `--pretty=false` produz compacta;
- regra `false` com `--pretty=true` produz indentada;
- repetição root `true → false` e inversa provam a precedência lexical;
- `true → #set format.svg()` permanece indentada: named omitido não escreve
  `false` nem apaga a decisão anterior;
- regra dentro de container falha com a mensagem pública medida e não deixa
  artefato de sucesso;
- named desconhecido e tipo inválido falham sem warning de target ignorado;
- chamada direta `format.svg()` falha com `cannot be constructed manually`.

Os asserts verificam exit, stdout/stderr, existência do artefato, XML válido,
presença/ausência de indentação e igualdade morfológica normalizada entre as
duas serializações. Não fixar tamanho ou SHA-256 do SVG como contrato.

## Controles

Um caso sem `format.svg` preserva o default anterior. `format.png`,
`format.pdf` e `format.html` continuam ausentes. A suíte não afirma paridade
global do SVG, não testa geometria, fonts, destinations ou HTML frame.

Mutation obrigatória troca CLI/documento, colapsa `None` em `false`, ignora a
regra, faz a regra vazia escrever `false`, aceita nested e altera o default.
Repetição/inversão e ao menos um caso de falha permanecem no conjunto mantido.
Unknown, timeout, binário sem proveniência ou XML inválido bloqueiam; nunca
contam como sucesso.

## Gate

O teste RED só pode ser criado depois da confirmação humana explícita do L0
causal P1462. A confirmação foi recebida em 2026-09-19; a correção R2 de
paridade invalida o ensaio R1 inconclusivo e exige novo selo. Depois dele, o
header aponta exclusivamente para este prompt e a execução RED precede a
implementação candidata.
