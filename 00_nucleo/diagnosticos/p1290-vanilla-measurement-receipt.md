# P1290 — recibo de medição bilateral

**Manifesto:** `00_nucleo/diagnosticos/p1290-manifest.json`
**SHA-256 do manifesto:** `8b18feaf2b0f75b3918d04b84e9dcf368d79df96fe7b9e2e92d2b727f4b10cda`
**Estado:** medição contratual, sem selo e sem veredito

## Proveniência

- início registrado: `2026-08-31T10:40:23-03:00`;
- reinício por invalidação: `2026-08-31T11:41:51-03:00`;
- nova congelação: `2026-08-31T11:44:52-03:00`;
- invalidação: `p1290-seal-invalidation.json`, SHA-256
  `c2e81e5db2810d720d80e2340cc79a600b12c3eea3725d20f342edd3e2d03d0e`;
- selo invalidado: SHA-256
  `18f2329888c79d64a8459bfa507f622a710745dd94c65c7586cc5b52967c7f6e`;
- `HEAD`: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`;
- árvore: não commitada; lista exata em
  `p1290-baseline-tree-state.md`, SHA-256
  `123a37709d24c3097eac85335fa574891c35dfbf1a21a68531c0338b231ac76b`;
- passo: SHA-256
  `7aaf6183c4e5d7c51b1194b4f07046733d9f26b9cc72c441f6621a4d52590838`;
- consumer: somente hash calculado, sem leitura do conteúdo,
  `7c56b8c79f489abe568ec3a104cae79f08dab81d9be3bccae6689a15a6589bf4`;
- vanilla: `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- cristalino: baseline original congelado em target próprio, sem rebuild nem
  execução do candidato atual, SHA-256
  `acc52526e1c1cfde21c4583857330a6f64540f46caf1b7fa89c666da6deee64a`,
  `typst 0.15.1 (53d21c5a)`;
- probe: `p1290-contract-probes.typ`, SHA-256
  `6ac1fed77e50b0ed7b7962048427be1c87ca9917d55c181a35d6ea4e01a6229b`.

## Comandos reproduzíveis

```sh
sha256sum /usr/local/bin/typst /tmp/p1290-contract-target/release/typst
/usr/local/bin/typst query 00_nucleo/diagnosticos/p1290-contract-probes.typ '<p1290-contract>' --field value --one --pretty
/tmp/p1290-contract-target/release/typst query 00_nucleo/diagnosticos/p1290-contract-probes.typ '<p1290-contract>' --field value --one --pretty
```

Os hashes dos dois binários coincidiram com a congelação original e as duas
queries terminaram com exit `0`. O aviso de depreciação de `query` não altera
o valor observado.

## Resultados que produzem a decisão

Em ASCII, a variável relevante é a largura do corpo interno já serializado e
unido por `", "`, não o número de itens. Corpos 49 e 50 ficam lineares; corpo
51 quebra. A confirmação independente por cardinalidade mede 17 zeros = corpo
49, linear; 18 zeros = corpo 52, multilinha. Vazio, singleton e curto são
`()` / `(0,)` / `(0, 1, 2)`. No multiline, o vanilla usa dois espaços por
nível, reindenta recursivamente e põe vírgula final em cada item.

Para `MathOp`, o vanilla sempre materializa os campos nomeados na ordem
`text`, `limits`. `csc`, `dim` e o composto `arcsin` medem `false`; `lim` mede
`true`; `math.op` do usuário conserva `false` e `true`. Markup e escaping
conservam a `repr` morfológica do conteúdo e podem induzir multiline. O
controle `math.sum` mede uma variante `symbol`, não `MathOp`.

O caso invalidado `math.op([a \# "quote"], ...)` foi removido porque misturava
escaping com `SmartQuote` cuja proveniência já está destruída no baseline
cristalino antes de `repr`. O substituto
`math.op([a \# \*], limits: false)` mede no vanilla exatamente
`op(\n  text: sequence([a], [ ], [#], [ ], [*]),\n  limits: false,\n)`.
O cristalino original mede `op(sequence([a], [ ], [#], [ ], [*]))`: a mesma
morfologia interna está disponível e a divergência restante é apenas o
contrato de campos deste owner. A remedição bilateral do probe completo
confirmou sem alteração todos os demais casos e strings.

O baseline cristalino coincide nos arrays vazios/singleton/curtos, mas mantém
lineares os casos acima da fronteira e os nestings. Para todos os `MathOp`
medidos, emite `op(<conteúdo>)`, omitindo `text:`, `limits:` e o booleano. O
controle `math.sum` coincide.

As strings completas são canônicas em `p1290-contract.json`, SHA-256
`f0936c3de1ed8715bec043b01b9d283a9be2ac1fee422fc94b835192dd3f7a50`.

## Classificação e `Unknown`

O observável é morfologia pública da linguagem (ADR-0107), não bytes de
render nem algoritmo. A correção é paridade em fluxo contínuo (ADR-0127): a
medição não exige assinatura Rust pública, modo padrão, fase nova ou quebra de
compatibilidade.

Permanece explicitamente `Unknown`: a unidade de largura para representações
não-ASCII; a proveniência de `SmartQuote` já resolvida em `Content::Text` por
fase anterior; membros de `math` e formas de conteúdo não enumerados;
construções opacas ou não aceites pelo parser; e qualquer equivalência fora
deste fragmento de `repr`. `Unknown` não equivale a sucesso e não autoriza
reconstrução por glifo, mudança de fase ou edição de outro owner.
