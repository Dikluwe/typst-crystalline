# P1292 — amendment-13: resselo após normalização de EOF no pre-commit

**Papel:** autor contratual segregado, sem escrita em produto, testes, oracle,
ataques ou veredito

**Predecessor:** contrato canônico v13
`61387d1be09b46740094112dbc2adea4f1df1491b4ca18fab1d4b90f9fc60010`,
seal v13
`bff65c3e83f5dc84bcc8eaad03eb4463e7f0dd2bef3a1f194470f51a56496db4`

## Proveniência

- `HEAD`: `0eb39f8ecb48930515f2cadb6a378450855b5a72`;
- working tree não commitada;
- auditoria: `2026-09-01T13:04:46-03:00`;
- manifest corrente SHA-256
  `c3dbc43554474f81a8ce6b9b92557470eeb34cc76b33f6c1509f9e4195abe31f`;
- alvo de paridade e todos os observáveis públicos permanecem os do v13.

## Auditoria byte a byte

A validação dos 27 L0s protegidos pelo v13 encontrou exatamente uma
divergência:

```text
00_nucleo/prompts/entities/elements/flush.md
v13:   7e1290bd948e4270bc380bf13fbf209819a679cfa87ad7d58280814e83cb9319
atual: 75967e0915aa211ef297c7f88059c3ce675c892de450930137760605df223c37
```

O arquivo atual tem 1.867 bytes e termina em uma única `LF`, imediatamente
depois de `realização de floats no fluxo.`. Acrescentar somente uma `LF` aos
bytes atuais produz 1.868 bytes e exatamente o SHA v13:

```text
sha256(current_bytes + b"\n")
= 7e1290bd948e4270bc380bf13fbf209819a679cfa87ad7d58280814e83cb9319
```

Logo o v13 diferia apenas por uma linha vazia adicional no EOF. Todos os 1.867
bytes atuais, inclusive o texto normativo integral, coincidem. Os outros 26
L0s permaneceram byte-idênticos ao v13.

## Classificação e decisão

A remoção da linha vazia final não muda semântica, morfologia, aceitação,
ownership, default, API, fase ou compatibilidade. Ressellar como v14 com os
mesmos 27 paths, substituindo apenas o hash de
`entities/elements/flush.md` pelo SHA atual.

Os objetos canônicos permanecem exatamente:

```text
lots
edeb8a195e15263dbfbc71de9d242657e79f9dd65e94c97e990b066e03f119bf

comparison_policy
71eb4eafff8d88233aaa3df6b2666a651db75ebcdfd0fe2d470dc6620d190a46
```

P1030, campanha adversarial, oracle protegido e expectativas públicas não são
reabertos. O v14 altera somente a identidade dos bytes L0 protegidos e a ponte
causal do selo.

## Gate V5 e reparo mecânico do consumer

A primeira validação do candidato v14 interrompeu o resselo corretamente: o
consumer `01_core/src/entities/elements/flush.rs` ainda declarava
`@prompt-hash 7e1290bd`, identidade do L0 antes da normalização. V5 reportou
o L0 atual `75967e09` contra o header antigo.

O coordenador, fora da autoridade deste autor contratual, corrigiu somente o
header para `@prompt-hash 75967e09`. O consumer resultante tem SHA-256 bruto
`0208c6b3d86f3f64b7c84051dead1e5487b8d02ccf92dbf902257cafccc3cad1`.
Nenhum L0 ou corpo produtivo foi alterado nesse reparo. O resselo só prossegue
após V5/V15/V26 retornarem zero.

## ADR-0127

Não há novo gate humano. O delta provado é exclusivamente whitespace de EOF,
sem efeito normativo ou produtivo. Qualquer mismatch adicional, diferença que
não fosse a única `LF`, mudança de lots/comparison ou falha V5/V15/V26 teria
interrompido o resselo.
