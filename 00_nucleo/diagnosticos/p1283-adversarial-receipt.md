# A-P1283-v4 — recibo adversarial independente

## Natureza e alcance

Este documento é um diagnóstico adversarial. Não é Prompt L0, não legitima código e não
é um veredito sobre uma implementação candidata.

- Contrato atacado: `C-P1283-v4`, confirmado pelo autor do contrato como semanticamente
  idêntico a `C-P1283-v3`; a revisão v4 alterou somente hashes mecânicos `Hash do Código`.
- Papel: adversário do contrato.
- Regime: protocolo completo de materialização segregada.
- Saída: conjunto determinístico de 16 mutações negativas e 3 controles.
- Acesso proibido e não realizado: código candidato, testes candidatos e diffs produtivos.
- Escrita autorizada e realizada: somente este diagnóstico.

O contrato foi inicialmente recebido como resumo normativo textual selado. Após a
materialização do recibo canônico, o adversário leu somente esse artefato, confirmou o
estado `SEALED_SPEC_FOR_IMPLEMENTATION`, a mesma matriz 16+3 e o pin SHA-256 abaixo. O
recibo canônico substitui a identidade textual anterior como predecessor causal deste
artefato.

## Entradas congeladas disponíveis

| Entrada | Identidade normativa | SHA-256 do ficheiro |
|---|---|---|
| Contrato C-P1283-v4 | `00_nucleo/diagnosticos/p1283-contract-receipt.md`; estado `SEALED_SPEC_FOR_IMPLEMENTATION` | `80eb980b8f22e23f3c9f56e2526f28a8591bde5eae3849b44348c60af239dc3f` |
| Passo 1283 | `00_nucleo/materialization/typst-passo-1283.md` | `67de6827bf12e281e45cda4fa4d1ed67dd1a9956f3d55b3f36f9a1ae9e6d892f` |
| L0 `sym` | `Hash do Código: 9bfa8e7f` | `42a2127215e9a37d69c90695e1be66a61320fa086ffd36b51a1ac3441c816f3d` |
| L0 `emoji` | `Hash do Código: c4ecf7d3` | `d5fda4cff41bab53b5e877061a63aeb1fda57896d328552a637e6e9cc8e3291f` |
| L0 structural math | `Hash do Código: b761923a` | `4441fe00c99c3475179c4a90060f5698e4472a368756ead6e4312781229617bf` |
| Codex 0.3.0 `sym.txt` | catálogo pinado | `6ee467d9939acb5c7d0a3eba30c9f640d157529cbf57df752367deb343e0fc16` |
| Codex 0.3.0 `emoji.txt` | catálogo pinado | `8691ca68e09b6fedca61e00e824648e79f7502772eefe7af367e404e26161489` |
| Vanilla `foundations/symbol.rs` | semântica de `Symbol` | `4213a43cc71d0653992ce4af5af40848ef491e81a1a5fbcdb560736b1adad18a` |
| Vanilla `symbols.rs` | adaptação recursiva Codex | `991815cb75729d5e7e9a0988dcf91f7de9cdc2030ea44398575a2d4d82463a79` |
| Vanilla `math/mod.rs` | composição do módulo math | `a8a64e505979b8a6fce3bb5fef978677a40200199e9dc36738a3343ae1104957` |

Os hashes dos L0 acima substituem os pins de A-P1283-v3. A matriz adversarial não muda
porque a revisão v4 foi declarada puramente mecânica.

## Classificador

- `Preserved`: todos os observáveis exigidos para o caso coincidem, sem divergência
  intencional aplicável.
- `Violated`: ao menos um observável determinável — existência, kind, valor Unicode,
  `repr`, variant, modifier, estrutura, mirror, provenance ou diagnóstico — diverge.
- `Unknown`: a observação é opaca, ambígua, não suportada pelo observador ou esgota o
  budget. `Unknown` nunca é convertido em `Preserved`, nunca rejeita uma mutação negativa
  e nunca conta no numerador do mutation score.
- `IntentionalVariantDeprecationDivergence`: aplica-se somente quando uma variante marcada
  como deprecated no Codex está presente, resolve com kind e valor corretos, mas não emite
  warning próprio porque o transporte de depreciação por variante está explicitamente
  fora do escopo. A classificação não autoriza omitir ou alterar a variante.

## Ataques negativos determinísticos

Cada linha define uma transformação mutante, uma testemunha independente e o oráculo que
o contrato deve produzir. Todos os 16 casos têm resultado esperado `Violated`.

| ID | Transformação mutante | Testemunha e observáveis exatos | Oráculo esperado |
|---|---|---|---|
| N01 | Remover um símbolo direto do catálogo. | Remover `sym.alpha`. A fonte Codex declara `alpha α`; o path deve existir, ter kind `symbol` e valor `U+03B1` (`α`). | `Violated` |
| N02 | Remover ou achatar os submódulos `gender`/`control`. | `type(sym.gender) = module`, `sym.gender.male.stroke.t = ⚨`; `type(sym.control) = module`, `sym.control.dc.one = ␑`. O mesmo nesting deve existir no mirror math. | `Violated` |
| N03 | Trocar `Symbol` por módulo ou vice-versa. | Substituir `sym.alpha` por módulo vazio. O kind observado deve continuar `symbol`, nunca `module`. | `Violated` |
| N04 | Alterar um valor base. | Trocar `sym.alpha` de `α` para `β`. Existência e kind não compensam o valor base incorreto. | `Violated` |
| N05 | Remover ou trocar VS15. | Transformar `sym.trademark` de `U+2122 U+FE0E` (`™︎`) em `U+2122` ou em sequência com outro selector. A base e o `repr` integral deixam de coincidir. | `Violated` |
| N06 | Remover ou trocar VS16. | Transformar `emoji.heart` de `U+2764 U+FE0F` (`❤️`) em `U+2764` ou noutro selector. `emoji.heart.excl` também deve preservar `U+2763 U+FE0F`. | `Violated` |
| N07 | Truncar uma sequência ZWJ. | Remover `U+200D` de `emoji.dancing.ballet = U+1F9D1 U+200D U+1FA70` (`🧑‍🩰`), produzindo dois clusters ou outro valor. | `Violated` |
| N08 | Remover ou renomear uma variante. | Remover `emoji.heart.lightblue` ou renomear o modifier para `skyblue`. O path original deve existir, aparecer no `repr` e resolver para `🩵`. | `Violated` |
| N09 | Alterar o valor de uma variante. | Trocar `emoji.heart.arrow` de `💘` para `💝`. O modifier continua presente, mas o valor observado diverge. | `Violated` |
| N10 | Tornar significativa a ordem de aplicação dos modifiers. | `sym.bowtie.stroked.big.l.r` e `sym.bowtie.r.l.big.stroked` aplicam o mesmo conjunto e devem resolver ambos para `⟗`. Falha de uma permutação é divergência. | `Violated` |
| N11 | Em parent sem bare, escolher simplesmente a primeira variante ou fabricar bare. | `emoji.arrow` deve resolver por best-match vazio para `U+2199 U+FE0F` (`↙️`), não para a primeira variante listada, `U+27A1 U+FE0F` (`➡️`). O `repr` deve continuar sem variante `""` inventada. | `Violated` |
| N12 | Quebrar um lado de alias ou apenas as suas variantes. | Comparar identidade integral de base e lista ordenada de variants em `dollar/pataca`, `yen/yuan`, `emptyset/nothing` e `gradient/nabla`. Em particular, `emptyset` e `nothing` preservam base `∅` e as mesmas seis variantes, incluindo `.zero = U+2205 U+FE0E`. | `Violated` |
| N13 | Omitir um trecho da cópia recursiva `sym → math`. | Remover `math.control.dc` do mirror. A travessia recursiva deve encontrar `math.control.dc.one = ␑`, `math.gender.male.stroke.t = ⚨` e, salvo bindings math próprios, o mesmo subgrafo público de `sym`. | `Violated` |
| N14 | Sobrescrever binding próprio de math durante o mirror. | Substituir `math.sqrt` por símbolo. `math.sqrt`, `math.class`, `math.equation` e `math.op` devem continuar `function`; `repr(math.sqrt([x]))` deve ser `root(radicand: [x])`. | `Violated` |
| N15 | Suprimir ou alterar a depreciação de topo de `join`. | `#sym.join` deve resolver para `⨝`, concluir com sucesso e emitir exatamente um warning, sem hint: `` `join` is deprecated, use `bowtie.big` instead ``. Ausência, texto diferente, erro ou warning duplicado divergem. | `Violated` |
| N16 | Remover a extensão `registered` ou creditá-la como vanilla. | `sym.registered` e `math.registered` devem existir como `symbol`, valor `®`, e manter provenance/classificação de extensão cristalina. Remoção ou marcação como paridade vanilla falha; esta família possui dois ramos mutantes, mas um único item no denominador congelado. | `Violated` |

## Controles do classificador

Os controles não entram no denominador das mutações.

| ID | Construção de controle | Resultado esperado |
|---|---|---|
| C01 | `sym.gt.tri` existe, tem kind `symbol`, resolve para `⊳` e não emite warning próprio da variante, embora o Codex marque a variante como deprecated. | `IntentionalVariantDeprecationDivergence` |
| C02 | `sym.gt.tri` está ausente e a ausência é justificada indevidamente pela exceção de depreciação por variante. | `Violated` |
| C03 | `join` existe e resolve, mas não emite exatamente o warning de topo exigido em N15. | `Violated` |

Um resultado `Unknown` em qualquer N01–N16 não satisfaz o ataque e impede considerar essa
mutação rejeitada. Um resultado `Preserved` em C01 também é incorreto: ele apagaria a
divergência intencional individualizada do diagnóstico.

## Denominador, score e gate

- Mutações negativas válidas no denominador: `16`.
- Controles no denominador: `0`.
- Numerador exigido para selar poder discriminatório: `16` mutações classificadas como
  `Violated`.
- Mutation score exigido: `16 / 16 = 1.0`.
- Qualquer mutação `Preserved`, `Unknown` ou classificada indevidamente como
  `IntentionalVariantDeprecationDivergence` reduz o score abaixo de `1.0` e impede o gate.

Este score é a exigência do conjunto A-P1283-v4. O adversário não executou o conjunto
contra uma implementação candidata e, portanto, não declara um score observado de
implementação.

## Independência e limitações

- O adversário recebeu o resumo normativo selado de C-P1283-v4, o passo explicitamente
  autorizado, os três L0 e as fontes Codex/vanilla. Depois da materialização canônica,
  leu somente `p1283-contract-receipt.md`, confirmou seu SHA-256 e pinou-o neste recibo.
- Não leu código nem testes candidatos, não inspecionou diffs produtivos e não adaptou as
  testemunhas a uma solução.
- Não escreveu contrato, L0, baseline, implementação, testes ou veredito; escreveu somente
  este recibo autorizado.
- A capacidade de filesystem do ambiente não foi mecanicamente reduzida a uma allowlist
  de leitura. A independência foi preservada por disciplina de entradas, mas não é
  atestada por isolamento técnico. A formulação proporcional é: **executado sem atestação
  de isolamento**.
- O contrato canônico está ligado a este recibo pelo SHA-256
  `80eb980b8f22e23f3c9f56e2526f28a8591bde5eae3849b44348c60af239dc3f`.
- O recibo cobre somente o fragmento observável declarado para P1283 e não prova
  equivalência funcional geral de Typst.
