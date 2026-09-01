# P1286 — plano adversarial de mutações semânticas

**Natureza:** plano ex-ante do adversário. Não é Prompt L0, implementação,
teste produtivo, oráculo, execução de mutantes, selo ou veredito.

## 1. Papel, regime e isolamento

- Regime: protocolo completo da skill `tekt-materializacao-segregada`, limitado
  ao papel **Adversário**.
- Executor: agente `/root/ataques_p1286`, checkout local compartilhado, em
  2026-08-30.
- Saída autorizada: somente este diagnóstico.
- Leitura autorizada: `AGENTS.md`; skill e referências; ADR-0107, ADR-0108,
  ADR-0127 e ADR-0129; exclusivamente
  `00_nucleo/materialization/typst-passo-1286.md`; receipts finais P1286 de
  medição, ownership, contrato v2 e pós-gate; Prompts L0 finais afetados.
- Capacidades negadas e respeitadas: ler ou escrever implementação candidata,
  ler ou escrever testes candidatos/produtivos, escrever L0, construir
  oráculos, executar mutantes, corrigir a solução, ressellar hashes ou emitir
  veredito.
- Contexto herdado: pedido de papel segregado e instruções superiores; nenhuma
  saída privada do implementador ou do testador foi recebida.
- Atestação: **executado sem atestação de isolamento físico do host**. A
  allowlist e a ordem causal foram respeitadas, mas o checkout é compartilhado.

Nenhum source ou teste candidato futuro foi inspecionado. As “alterações
mínimas futuras” abaixo são operações semânticas abstratas a injetar somente
depois de existir candidato e sob outra capacidade.

## 2. Entradas congeladas e hashes

Snapshot documental reaudited: `HEAD
53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, observado em
`2026-08-30T20:34:07-03:00` numa working tree compartilhada e não commitada.
O hash identifica bytes; não prova isolamento.

| entrada | SHA-256 |
|---|---|
| `AGENTS.md` | `bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0` |
| skill | `33a32f7bc439de3fe3aa530bd65518e512a93f40152c91b2ace0789de34a3a56` |
| `references/papeis-e-capacidades.md` | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` |
| `references/artefatos-e-gates.md` | `bf218259b4454974bf8889ce319e04c0c7ec668b9a542d0eb3b4d0492a623963` |
| ADR-0107 | `e680d22bbf4486cf93f5bfb4ec85f4ae965e6db788c2a18c48f3be000029d49d` |
| ADR-0108 | `31daec5ae9e84cb5bbdcb806e9a2b6cb9160b7e90519df53e6e0bd8809076405` |
| ADR-0127 | `5e8581b5f9ebb0798d4213e59e39ee8dfcd4f41b34d4ca639b00b1f287699ad9` |
| ADR-0129 | `64756b81ce58ca62e1a166b3776303759bc7af507a1c97a4e3ad91a8dc5b906e` |
| Passo P1286 | `02f9393468696b6196338d0c9832f39e3228176cf1c475e6ccdd754e873d11cf` |
| receipt de medição | `e16ea033bb6216f04813ace238395649c7d32b7251af14ac98d337525cf53ef8` |
| receipt de ownership | `76e33f4e6abec687b3d4670a468cc030c8569bc21264bd54513ababb474be6af` |
| receipt de contrato v2 | `16597543965ca13d37fdabf9be184f3477df92da4f6702ba14f0a4ca4918f9bb` |
| receipt pós-gate L0 | `27e2f5b931d898396319d7a62875ca377511e93ebe0e8b97cdd4f6817d6663c0` |

L0s P1286 congelados:

| grupo | `path = SHA-256` |
|---|---|
| smartquote | `compiler/stdlib/text/smartquote.md = 8200a170e2f5a8efd734d89743e7051116e35f5f28a4606e493e6493ef8a48c0`; `compiler/layout/smartquote.md = 9b260fa42ecc1dd3a5229773704f0638d99a005312a98de5544851e9b1afa9da`; `compiler/lang/quotes.md = d303c271dec7b92ed1feadeb24c4e63db804e2b959f13d0a3eaa40c51c46c48e`; `compiler/eval/rules.md = d5bdd9b0bd4e95a06082216b1b5f261b66064dc7653391e294f584d2de714f21`; `compiler/eval.md = de0d8018d85d75e990746e49e35faa1025c0a244b4fff5f1639eb2edb3b802f8`; `entities/elements/smartquote.md = d217b2f408e58c0b896c64f91acd0cfb3d4e11f193f828babdcb9cd9aba2062e`; `entities/content.md = be1188f3073cd470a0aa51b63bcc8a2f162b954ddc2a3e7de30ed19e25a2db3d` |
| line | `compiler/stdlib/shapes.md = 8746d996872e58b432cbe29994da5fec4b9edaae8d27c658cac1b31f8cceccbb`; `infra/export/svg.md = 38820e8471fb05d7826ec359e153c02ef13fb641df5791101c87cf05daf596bb` |
| color | `compiler/stdlib/color.md = 3b5c3f4d161acef90bd13079ca2656425c12bdcbd19787a5904cb03cc96f1ff6`; `compiler/stdlib/foundations/color.md = c450d3608ffbc7e924aa67b28c1e82b2b613316ac45bdcfc2cc2155a998bcef6`; `entities/color.md = f31ea87db04e18cb3fff4fe85ffebd881b18920f659ba3ccae21cd551a76d468` |
| PDF transversal | `compiler/stdlib/pdf.md = cc9f72f371dc767570a021c4512b73dd7deba785469535051e88fb40ffaac0da`; `entities/layout_types.md = 1df6881d8d9d692213d5cbd005f05782c846493f03c00b347324ec000cda1fd6`; `compiler/layout.md = 94e07bca9e9fb15ee538e5559b228c8b7bf755050b5daf438a4d51ed5f37a4ff`; `infra/export/builder.md = e990b0be9f1c948eb7133a39f0a116f26d51625b3787f188ea85f2358a504a64`; `infra/export/stream.md = 98c007188cac86a8093689e0680e3fc916855e698c01d4903d602c787c2a0245`; `infra/pipeline.md = 316b4d88560561a8f114234c0ef2c8a4397a66604c584df4a3faecd0d6f8996e`; `shell/cli.md = f9f9b4fe286baa16c058a2973084378be427264aa5cb2214cce925f45d9accbc` |
| quatro owners pós-gate | `entities/elements/pdf_attach.md = 151e5aa215abdeac306f77edd50ff1eb8ba0a49495dc4aab90461f09d02f845a`; `entities/elements/pdf_artifact.md = d1c643277f0bb2060c8aec560bffd7bb67d00b2edd033af65d9b9cc560a25a79`; `compiler/layout/pdf_attach.md = d57ee15295168649b3f6d0f9a3a7bb0862f776013c52122cae69c819d36eb841`; `compiler/layout/pdf_artifact.md = 0de4583b7abfc421bfbecb7e330e9fbe20386bd2f62a2ecdbafe28221e45583b` |

Baseline ratificado declarado: `upstream/main a51e02804`; a associação
independente dos bytes locais a esse commit permanece `Unknown`. Binário
vanilla medido: `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

### Reauditoria após o refinamento dos L0

Os três owners suplementares não tornam nenhum dos 25 mutantes equivalente e
não criam uma nova classe negativa independente neste manifesto:

- `compiler/stdlib/foundations/color.md` torna observável o resultado de mix
  por `rgb(color)`. Falha dessa conversão faz o candidato base falhar e deixa os
  IDs `CM-*` em `Unknown`; não pode salvar um mix mutado.
- `infra/export/svg.md` congela para paths abertos de dois pontos a lente
  `translate(start)` + `M 0 0 l delta`. Isso fortalece, sem trocar, os cinco
  witnesses `LN-*` de início, direção, bbox e precedência.
- `shell/cli.md` aceita ocultamente `--pdf-standard 2.0` apenas para tornar
  executável a sonda Background; não promete standard PDF nem
  `/AFRelationship`. O refinamento é incorporado em `AR-FORMULA-02` sem
  ampliar o fragmento `Unknown`.

O refinamento de `infra/export/builder.md` preserva attachments em documentos
sem frame visual através de uma página branca de compatibilidade. Essa
obrigação é agora explicitamente exercida por `PA-DROP-01`. Portanto os mesmos
25 IDs continuam semanticamente não equivalentes e o denominador permanece
fechado.

## 3. Política comum de witnesses e `Unknown`

Cada mutante abaixo é semanticamente não equivalente: existe uma testemunha
que exige resultado diferente entre contrato e mutação. “Black-box” mede
semântica/morfologia pública; SVG, QDF, `mutool` e `pdfdetach` são lentes para
geometria/tagging/payload, nunca exigência de igualdade byte-a-byte do ficheiro
inteiro. Um teste L1/L3 pode observar o carrier apenas onde esse carrier é o
contrato explícito entre camadas.

Resultado do gate por execução:

- `Violated`/morto: witness passa no candidato não mutado e falha no mutante
  isolado pela diferença indicada.
- `Preserved`: reservado aos controlos positivos no candidato não mutado;
  nunca é atribuído a mutante que sobrevive.
- `Unknown`: identidade/compilação/injeção não provada, ferramenta não distingue
  o observável, parser é opaco, orçamento termina ou o witness não chega ao
  ponto mutado. `Unknown` não mata mutante e bloqueia o gate.
- “equivalente”, “não aplicável” ou “indeterminado” não dá crédito. Como todos
  os IDs abaixo têm witness discriminatório, uma injeção sintaticamente
  equivalente deve ser recolocada no ponto decisório efetivo; se isso não for
  possível, o ID fica pendente, não removido do denominador.

Se o candidato base já falhar um witness, não há campanha válida para esse ID:
o resultado é `Unknown` e o contrato não pode ser selado até reparar o base.

## 4. Fatia 1 — `smartquote`

| ID | alteração mínima futura | witness obrigatório | resultado esperado (morte) | regra específica de `Unknown` |
|---|---|---|---|---|
| `SQ-PRECEDENCE-01` | Fazer `alternative` localizado prevalecer sobre override explícito `quotes`. | Black-box HTML/texto: língua `de`, `alternative:true`, `quotes:"()"`, duas folhas ao redor de `X`. | Morfologia correta é `(X)`; saída `»X«` ou outra localizada mata o mutante. | Se o harness não separar glifos do warning HTML ou não provar a configuração efetiva, `Unknown`. |
| `SQ-AUTO-02` | Tratar `quotes:auto` explícito como campo omitido e herdar as quotes da StyleChain. | Black-box: `#set text(lang:"de")`, `#set smartquote(quotes:"()")`; chamada direta com `quotes:auto` abre/fecha `X`. | Deve apagar a herança e produzir `„X“`; `(X)` mata o mutante. | Se chamada direta e set-rule não puderem ser exercidas no mesmo documento, `Unknown`. |
| `SQ-GRAPHEME-03` | Contar Unicode scalar values/bytes em vez de grapheme clusters. | Black-box: `quotes:"a\u{301}b"` (dois graphemes, três scalars) ao redor de `X`. | Deve aceitar e produzir `áXb`; erro de cardinalidade mata o mutante. Controle negativo `quotes:"x"` conserva `expected 2 characters, found 1 character`. | Normalização automática que torne impossível provar os clusters ou perda de codepoints na lente dá `Unknown`. |
| `SQ-LANG-04` | Ignorar a língua corrente e usar sempre o fallback inglês. | Black-box: língua `de`, defaults, duas folhas ao redor de `X`. | Deve produzir `„X“`; `“X”` mata o mutante. | Se a língua não puder ser confirmada no input avaliado, `Unknown`. |
| `SQ-ALTERNATIVE-05` | Ignorar `alternative:true` ou mapear o par alternativo ao primário. | Black-box: língua `de`, sem quotes explícitas, `alternative:true`, duas folhas ao redor de `X`. | Deve produzir `»X«`; `„X“` mata o mutante. Controle em língua sem alternativa não substitui este witness. | Língua/região fora da matriz congelada ou somente um glifo sem estado open/close dá `Unknown`. |

Os cinco witnesses devem ainda ser repetidos pela face `#set smartquote`/markup
quando aplicável. Divergência entre chamada direta e markup mata o mesmo ID;
ela não é desculpa para declarar uma face `Unknown` se a outra passou.

## 5. Fatia 2 — `line(start:, end:)`

| ID | alteração mínima futura | witness obrigatório | resultado esperado (morte) | regra específica de `Unknown` |
|---|---|---|---|---|
| `LN-MAX-01` | Calcular bbox como `max-min` em vez de `max(0,start,end)`. | Teste L1 do lowering para `start:(10pt,20pt), end:(40pt,50pt)` e lente SVG de geometria. | Pontos absolutos `(10,20)→(40,50)`, largura `40`, altura `50`; SVG normaliza para `translate(10 20)` + delta `(30,30)`. Caixa `30×30` mata o mutante. | Screenshot sem coordenadas ou SVG que não permita reconstruir posição e extensão dá `Unknown`. |
| `LN-ABS-02` | Aplicar `abs` a `end-start`, perdendo direção. | Black-box/L1: `start:(40pt,50pt), end:(10pt,20pt)`. | Vetor observável é `(-30,-30)`; SVG usa `translate(40 50)` + delta negativo. Endpoint `(70,80)` ou vetor positivo mata. | Se a lente só mede comprimento e não direção, `Unknown`. |
| `LN-NORMALIZE-03` | Subtrair o mínimo aos pontos e normalizar o path para origem zero. | Teste L1 + SVG: `start:(-10pt,-20pt), end:(30pt,40pt)`. | Preserva pontos absolutos e SVG `translate(-10 -20)` + delta `(40,60)`, com bbox `30×40`; pontos `(0,0)→(40,60)` matam. | Clipping que esconda os pontos sem carrier L1 observável dá `Unknown`. |
| `LN-START-04` | Descartar `start` e emitir apenas o delta a partir de zero. | Black-box/L1: `start:(10pt,20pt), end:(40pt,50pt)`. | Segmento começa em `(10,20)`; SVG usa `translate(10 20)` e delta `(30,30)`. `(0,0)→(30,30)` mata. | Comparar somente comprimento/ângulo é insuficiente e resulta `Unknown`. |
| `LN-END-PRECEDENCE-05` | Quando `end` existe, ainda aplicar `length`/`angle`. | Black-box: `start:(10pt,20pt), end:(40pt,50pt), length:99pt, angle:180deg`. | SVG permanece `translate(10 20)` + delta `(30,30)`; qualquer endpoint derivado de 99pt/180deg mata. | Se o parser rejeitar a combinação antes do lowering ou a lente não distinguir endpoints, `Unknown`. |

Nenhum witness exige que a representação interna seja `Path`; exige os pontos,
direção, precedência e caixa contratados. O teste L1 só pode afirmar a forma
`Path` porque o L0 P1286 a escolheu explicitamente, não por paridade mecânica.

## 6. Fatia 3 — `color.mix`

| ID | alteração mínima futura | witness obrigatório | resultado esperado (morte) | regra específica de `Unknown` |
|---|---|---|---|---|
| `CM-SEQUENTIAL-01` | Reduzir com chamadas binárias sucessivas, normalizando a cada par, em vez de acumular componentes e dividir uma vez pela soma total. | Query black-box: `rgb(color.mix((red,1),(green,2),(blue,3), space:rgb))`. | Representação pública exata `rgb("#61969c")`; qualquer outra cor mata. | Se a serialização pública de `rgb` não estiver disponível ou arredondamento alheio impedir distinguir a mutação, `Unknown`. |
| `CM-RENORM-02` | Normalizar pesos percentuais como se precisassem somar 100%, ou preencher o restante implicitamente. | Query: pesos `20%,30%,10%` em `space:rgb`. | `rgb("#999f66")`; resultado dependente do “restante” ou erro por soma 60% mata. | Se o harness converter ratios em valores diferentes antes de `mix`, `Unknown`. |
| `CM-NEGATIVE-03` | Rejeitar, zerar ou aplicar `abs` a peso negativo mesmo quando a soma total é positiva. | Query: `(red,-1),(green,1),(blue,2), space:rgb`. | Aceita e produz `rgb("#00bade")`; erro ou outra cor mata. Controle `(red,-1),(green,1)` deve falhar com `sum of weights must be positive`. | Saturação/serialização que não permita distinguir a cor pública dá `Unknown`; aceitar apenas o controle não mata o mutante. |
| `CM-HUE-NGT2-04` | Permitir redução de três cores em HSL, HSV ou Oklch. | Três compilações black-box, uma por espaço hue, com `red,green,blue`. | Cada uma falha com `cannot mix more than two colors in a hue-based space`; sucesso em qualquer espaço mata. | Espaço não reconhecido ou erro anterior de parsing/tipo dá `Unknown`, não morte. |

## 7. Fatia 4 — `pdf.attach`

| ID | alteração mínima futura | witness obrigatório | resultado esperado (morte) | regra específica de `Unknown` |
|---|---|---|---|---|
| `PA-DROP-01` | Ao criar a página branca de compatibilidade para um documento sem frames, reconstruir o documento sem copiar o side-channel de attachments. | Black-box attachment-only: um attachment mínimo e nenhum conteúdo visual; `pdfdetach -list` e extração. | Exporta com a página branca de compatibilidade e exatamente um EmbeddedFile/Filespec/name-tree entry com payload extraível; zero attachments, erro por documento vazio ou reconstrução que perca metadata mata. | Ferramenta sem suporte a attachments ou PDF não aberto estruturalmente dá `Unknown`. |
| `PA-BYTES-02` | Copiar, recodificar, truncar ou substituir bytes explícitos durante eval/layout/export. | Black-box com bytes `[0,65,255]`; extrair e comparar o payload. | Extraído é exatamente `[0,65,255]`; qualquer alteração mata. Aqui bytes são o payload observável, não igualdade mecânica do PDF inteiro. | Falha de extração sem prova de que o PDF contém payload comparável dá `Unknown`. |
| `PA-PATH-03` | Trocar, limpar ou recalcular silenciosamente o path/nome virtual depois de eval. | Teste L1 do carrier e L3 do Filespec/name tree com path virtual congelado `dir/virtual.bin`. | O carrier e a identidade de deduplicação preservam o path; nome/identidade diferente mata. | Regras de `PathOrStr` fora da matriz ou ferramenta que mostra somente basename tornam a face L3 `Unknown`; o carrier L1 continua obrigatório. |
| `PA-METADATA-04` | Omitir ou alterar `mime_type`, `description` ou `relationship` no carrier/builder. | Teste L1 preserva `application/octet-stream`, `three bytes`, `Supplement`; L3 PDF normal observa Subtype/Desc. | Todos os campos permanecem no carrier e MIME/Desc aparecem quando aplicáveis; perda ou mutação mata. Emissão de relationship em PDF normal/A-3 não é exigida aqui. | Standard PDF e emissão de `/AFRelationship` permanecem `Unknown`; não podem matar nem salvar este mutante. |
| `PA-DUPLICATE-05` | Deduplicar, sobrescrever ou aceitar silenciosamente duas ocorrências do mesmo path. | Black-box pipeline com duas chamadas para `same.bin`, payloads iguais ou diferentes. | Aborta antes do export com `attempted to attach file same.bin twice`; sucesso, last-wins ou um único attachment mata. | Se uma chamada for podada/não coletada antes da validação, o resultado é `Unknown` para este ID e falha adicional de `PA-DROP-01`. |

## 8. Fatia 5 — `pdf.artifact`

| ID | alteração mínima futura | witness obrigatório | resultado esperado (morte) | regra específica de `Unknown` |
|---|---|---|---|---|
| `AR-PASSTHROUGH-01` | Colapsar `PdfArtifactElem` ao body antes do stream. | Black-box tags enabled: `pdf.artifact[artifact-secret]`, QDF/stream e controle plain. | Visual/texto preservados e body cercado por `/Artifact BMC ... EMC`; ausência do wrapper mata. | `pdftotext` sozinho não distingue o contrato e dá `Unknown`; é obrigatória inspeção estrutural balanceada. |
| `AR-FORMULA-02` | Reutilizar `SemanticKind::Formula` ou emitir tag `/Formula` para artifact. | Black-box/L3 com `kind:"header"`, tags enabled, e sonda irmã `kind:"background"` invocada com o argumento oculto `--pdf-standard 2.0`; inspecionar stream e structure tree. | Header usa `/Artifact<</Attached[/Top]/Subtype/Header/Type/Pagination>>BDC`; Background usa `/Artifact<</Type/Background>>BDC`; nenhum cria `/Formula` ou StructElem. Formula/StructElem ou property list trocada mata. | Parser que não correlacione stream e árvore, ou qualquer alegação geral de standards a partir da flag oculta, dá `Unknown`. |
| `AR-MCID-03` | Atribuir MCID próprio ao envelope Artifact. | Black-box tags enabled com artifact `other` entre dois conteúdos estruturais. | O Artifact não contém `/MCID` nem entrada ParentTree/StructElem; qualquer MCID atribuído a ele mata. | Contar texto `MCID` globalmente sem localizar o envelope é indeterminado e dá `Unknown`. |
| `AR-DESCENDANT-MCID-04` | Continuar a alocar/emitir MCID ou StructElem de Formula descendente enquanto o envelope Artifact está ativo. | L3/black-box: Formula dentro de `pdf.artifact`, mais Formula irmã fora como controle. | Descendente não ganha MCID/StructElem; irmã externa continua `/Formula` estruturada. Leakage interno ou supressão global mata. | Sem controle externo ou sem correlação de ParentTree, `Unknown`. |
| `AR-TAGS-OFF-VISUAL-05` | Com tags disabled, omitir o body ou alterar layout/pintura/texto do artifact. | Compilar artifact e body plain com tags disabled; comparar texto, geometria/pintura e contagem de filhos, não bytes PDF. | Conteúdo visual/textual é igual no fragmento observado; perda, duplicação ou deslocamento mata. | Igualdade de hash do PDF não prova nada; renderer/text lens inconclusivo dá `Unknown`. |
| `AR-TAGS-OFF-MARK-06` | Emitir BMC/BDC/EMC ou objetos estruturais de artifact mesmo com tags disabled. | O mesmo par tags-disabled, inspecionado por QDF/catálogo/ParentTree. | Zero marcação Artifact, MCID, StructElem e structure tree atribuível ao wrapper; qualquer uma mata. | Ocorrências em metadata/comentários sem parse de operadores não contam; resultado `Unknown`. |

Header deve ainda reproduzir a property list medida
`/Attached[/Top]/Subtype/Header/Type/Pagination`; Background, quando o eixo de
versão aplicável estiver disponível, usa `/Type/Background`. Fallbacks dos
outros kinds e efeito real em AT continuam `Unknown` e estão fora deste gate.

## 9. Gate discriminatório e score obrigatório

A campanha futura deve:

1. verificar todos os witnesses no candidato não mutado;
2. aplicar exatamente um ID por vez no ponto decisório efetivo;
3. repetir a execução pelo menos duas vezes e numa segunda ordem dos IDs;
4. registrar comando, ambiente, hashes do candidato/testes/harness, stdout,
   stderr e observável; e
5. restaurar o candidato antes do ID seguinte, sem editar contrato/oráculos.

Há **25 mutantes válidos e semanticamente não equivalentes** neste manifesto.
O único score selável é:

```text
mutation_score = mutantes válidos mortos / mutantes válidos = 25 / 25 = 1.0
```

Todos devem resultar `Violated`. Sobrevivente, `Unknown`, equivalente alegado,
injeção não aplicada, witness que falha no base ou resultado indeterminado
impede score 1.0 e bloqueia o selo. Casos opacos deliberados devem continuar
`Unknown`, mas não substituem nenhum dos 25 mutantes negativos.

Mudança em qualquer hash congelado invalida este plano como entrada selável e
exige revisão adversarial a partir do primeiro artefato afetado. Este documento
não declara que a campanha foi executada e não emite veredito de P1286.
