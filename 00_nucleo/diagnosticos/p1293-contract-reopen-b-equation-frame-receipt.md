# P1293 — recibo de reabertura contratual B: frame lógico de Formula e presença vazia de attach

**Papel segregado:** autor contratual/L0 (`autor_contrato_p1293`)  
**Estado:** `L0_AUTHORED_AWAITING_LINEAGE_RESEAL_AND_REPLACEMENT_GATE`  
**Instante:** `2026-09-01T22:51:58-03:00`  
**HEAD:** `7dd25ff0e222b6c7c640d6bc7957b98f94227507`  
**Working tree:** não commitada; contém trabalho P1293 compartilhado. Este recibo não atribui a este papel alterações fora da allowlist abaixo.

## 1. Entradas congeladas e segregação

| Entrada | SHA-256 verificado |
|---|---|
| `00_nucleo/diagnosticos/p1293-manifest.json` antes desta reabertura | `e1def7d8db9c63273bbe45ee753b20fdd4a297a1a2240953e4aa1b658609ef86` |
| `00_nucleo/diagnosticos/p1293-textitem-spacing-residual-measurement-receipt.md` | `afc6fc675765303f37f3a03228cc76d8c745347afcbacdc2edb5201e7f560a69` |
| `00_nucleo/diagnosticos/p1293-implementation-receipt-b.md` | `382df2f2d7623acd7ba1eb10ee66f483f1af546c752c36bb66ba0769747c107d` |
| selo serial consumido/refutado `p1293-contract-seal.json` | `8ed4c7287cc399ca8278238a691b1edbe557e5003ec191482a0566ad4062db90` |
| bloco canônico do selo consumido | `9eeba6e35a3897f10fdeee2844ccafcdac44fbf548caffdb4b2ac866d64cbec9` |
| contrato canônico protegido, somente pin conhecido | `2bb10fa308558a82b75acabb1a188c0832ad6708e5ff0b8246af3ced52707072` |
| oracle protegido, somente pin conhecido | `6f695c54582dbfbf335da7ab6d69324a712494a21b4aa725c73369eaa04a85b5` |

Foram lidos os L0 vigentes, ADR-0107/0108/0127/0129 e os dois recibos públicos acima. Não foram lidos nem editados oracle protegido, contrato protegido, testes, RED/discriminação, ataques ou veredito. Nenhum código produtivo foi lido ou escrito por este papel nesta reabertura; os `file:line` causais abaixo provêm do recibo independente pinado.

## 2. Medição antes da decisão

O recibo causal, medido em `2026-09-01T22:42:59.184987970-03:00` no mesmo HEAD e em working tree não commitada identificada por aquele recibo, estabelece:

- attach inline terminal em página `width:auto`: candidato `19,1521 × 9,6041pt`, vanilla ratificado `19,7681 × 9,6041pt`; residual `−0,6160pt`;
- R, K, W e RR terminais perdem exatamente `0,6160pt`, apesar de advances, GSUB, IC e tamanhos distintos;
- com marcador posterior `Z`, `x(Z)=19,7681pt` e a largura final é `26,4121pt` nos dois renderers: o cursor vivo já contém a extensão integral;
- `R=736du` a `7,7pt` fecha a tinta em `19,1521pt`; `SpaceAfterScript=56du` a `11pt` fornece os `0,6160pt` restantes do `EquationExtent` lógico;
- vanilla `tr: []` mede `6,424pt`, contra base `5,808pt`; o candidato mede `5,808pt` em ambos. Logo markup vazio sintaticamente fornecido reserva spacing, enquanto slot omitido e `none` semântico não reservam.

Cadeia causal pública medida:

1. `math/layout/mod.rs:598-624` conserva a largura em `EquationExtent`;
2. `layout/equation.rs:454-472` materializa Formula com os filhos visuais, e `:495-499` avança o cursor pelo extent integral;
3. `layout/mod.rs:2118-2138` faz terminal flush e `cursor.rs:664-666` reinicia o cursor;
4. `layout/mod.rs:1222-1252` usa `helpers::line_content_right`; `helpers.rs:46-77` vê somente a união material porque o envelope Formula não conserva a cauda lógica;
5. `math/layout/attach.rs:298-301,397-426` já calcula a constante/total para scripts não vazios; o refutador `tr: []` prova que sua classificação por materialidade é estreita demais.

Inferência: o residual terminal tem duas obrigações separadas e fechadas — o owner Formula deve conservar o mesmo `EquationExtent` que já move o cursor, e o owner attach deve distinguir `none` semântico de markup sintaticamente presente ainda que sem tinta. Refutam a inferência: Group não sobreviver ao flush, marcador posterior deslocar, posições dos filhos mudarem, `[]` não possuir carrier distinto de `Content::Empty`, ou `none` passar a reservar spacing. Qualquer refutador bloqueia; não legitima novo payload/helper/tipo público.

## 3. Decisão L0 e ownership 1:1

### `compiler/layout/equation.md` → `compiler/layout/equation.rs`

O scope-out v5 que atribuía o resíduo a attach/fraction e excluía equation/cursor/auto-page foi explicitamente refutado. `Semantic::Formula` deve preservar a extensão lógica integral usando somente `FrameItem::Group` existente, transparente e identity, como único frame geométrico interno: filhos localizados sem relayout; `inner_width` e `inner_height` derivados do mesmo `EquationExtent`; morfologia Formula, posições/render, cursor posterior, block/numbering, RTL/bidi e páginas não-auto preservados. Não há novo campo/variant/helper, segunda medição nem mudança de fase.

### `compiler/math/layout/attach.md` → `compiler/math/layout/attach.rs`

`None` e `Some(Content::Empty)` permanecem equivalentes à omissão no layout e não recebem spacing. Markup vazio sintaticamente presente (`[]`) não é `none`: o carrier estrutural já existente reserva uma contribuição `SpaceAfterScript` mesmo quando a MathBox não tem tinta/largura. As fórmulas passam a depender de presença do slot, jamais de materialidade/width; máximos, kerns, shifts, limits, cramped, IC e alinhamentos permanecem inalterados.

As duas relações são owners produtivos distintos e 1:1. Não há owner 1:N nem Núcleo Tekt novo. `layout/mod.rs`, `layout/helpers.rs`, `entities/layout_types.rs` e demais consumers não são legitimados.

## 4. Hashes L0 antes/depois e consumers intocados

| Prompt L0 | SHA-256 antes | SHA-256 depois | Consumer | SHA-256 atual, sem alteração autoral |
|---|---|---|---|---|
| `00_nucleo/prompts/compiler/layout/equation.md` | `8a557ebeb62cef433b30d7f1db66acb7ed27526e72770ceb2bd0a12514117387` | `68e0b1cbf55efa8118dade912c0e7be017b8fc2d1d85399ab0b354e39883cf9d` | `01_core/src/compiler/layout/equation.rs` | `5a33b6dc9e7981fbeaf3fd1645460cc58f2f85ed89644d67e589fae301c41af0` |
| `00_nucleo/prompts/compiler/math/layout/attach.md` | `db1abcc014473353baab0ed8d678b9449cd30a3668f98cb456f18b20cd2078af` | `cb10eb2d94d5936cde09b06b98d9c14afcc443a9b3b15b58c595f642e3aefca9` | `01_core/src/compiler/math/layout/attach.rs` | `4a3b6195cacbba955a70f7011b711bc2b39c2d025841344810ae26c96653e8f5` |

Controles produtivos fora do escopo no instante do hash: `layout/helpers.rs` = `91a0bfddc52592427d9403d8cc5a6956b396c8e30c947c0a93bae035d19c8116`; `entities/layout_types.rs` = `782b5866d5c97beb41f1df4338fb62bf5f133fabfbf02181d1c6760c631ea4db`. O seu conteúdo não foi alterado por este papel. `layout/mod.rs` também ficou fora da escrita autoral.

## 5. Classificação

- **ADR-0107:** extensão Formula, largura auto, posição posterior e presença/omissão de slot são linguagem; Group, carrier e predicados são mecânica interna.
- **ADR-0108:** a decisão sucede as medições e a cadeia `file:line`, explicita inferências e refutadores e não generaliza valores de fixture.
- **ADR-0127:** fluxo contínuo. A solução usa tipos/variantes/payload existentes e não muda contrato público, default, fase ou compatibilidade. Não há novo gate humano.
- **ADR-0129:** exatamente dois owners 1:1; nenhum compartilhamento 1:N foi inventado.

O contrato canônico/oracle já exigem os observáveis e permanecem byte-identical. A reabertura não muda comparação, lotes nem política `Unknown`; `Unknown` continua bloqueante.

## 6. Invalidação e gates

O selo serial SHA-256 `8ed4c7287cc399ca8278238a691b1edbe557e5003ec191482a0566ad4062db90` foi consumido pelo candidato refutado e fica histórico, sem autorização de novas escritas. O seu arquivo não foi editado. O candidato B não está aprovado.

Validações pré-resselo, sem escrita de headers:

```text
crystalline-lint --checks V15,V26 --fail-on warning .
✓ No violations found

crystalline-lint --fix-hashes --dry-run .
Would fix ./01_core/src/compiler/layout/equation.rs ... old=04a90b1b hash-a=f1c7240d hash-b=c0c8cc69
Would fix ./01_core/src/compiler/math/layout/attach.rs ... old=050c25a2 hash-a=7be814c8 hash-b=bcea7b36
```

Resultado: V15/V26 verdes; dry-run aponta **exatamente** os dois consumers proprietários. `--fix-hashes` não foi executado e nenhum header foi alterado.

## 7. Próxima transição permitida

O coordenador pode, fora deste papel, executar o resselo mecânico exatamente dos dois headers, revalidar V5/V15/V26 e solicitar novo gate discriminatório/seal substituto. A allowlist produtiva proposta para esse futuro selo é estrita:

- `01_core/src/compiler/layout/equation.rs`;
- `01_core/src/compiler/math/layout/attach.rs`;
- `00_nucleo/diagnosticos/p1293-implementation-receipt-b.md`.

Até o novo selo, todo produto permanece congelado. `layout/mod.rs`, `layout/helpers.rs`, `entities/layout_types.rs`, shaper/font_metrics e todos os demais owners, além dos lotes C/D, ficam fora da allowlist.
