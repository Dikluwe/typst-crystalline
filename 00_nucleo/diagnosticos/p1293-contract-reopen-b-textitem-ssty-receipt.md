# P1293 — recibo de reabertura B por residual `TextItem`/`ssty`

## Estado e segregação

```text
status: L0_AUTHORED_NEW_ADR_0127_HUMAN_GATE_REQUIRED
lot: B-only
blocking_measurement: +0.1001pt horizontal
unknown: 1 bloqueante
prior_seal: invalidated
product_write: none
lineage_reseal: forbidden before confirmation
lot-B-approval: false
lot-C-D: forbidden
```

Atuação como autor L0/contratual sob o protocolo completo da skill
`tekt-materializacao-segregada`, por capacidades e artefactos, sem atestação
de isolamento técnico de leitura no filesystem compartilhado. Foram lidos os
L0s proprietários, ADRs 0107/0108/0127/0129, o núcleo de fallback pinado, os
dois recibos públicos autorizados e a fonte vanilla ratificada. Não foram
lidos o patch candidato, o oracle protegido, recibos RED/discrimination ou
outputs privados. Nenhum produto, teste, oracle, gate, selo ou veredito foi
editado.

## Proveniência

- autoria: `2026-09-01T22:08:30-03:00`–`22:11:29-03:00`;
- branch/HEAD: `Tekt` / `7dd25ff0e222b6c7c640d6bc7957b98f94227507`;
- working tree compartilhada e não commitada;
- estado medido original: recibo causal abaixo, com checkpoint
  `2026-09-01T22:01:14.871396010-03:00`, 46 ficheiros tracked, 3521 inserções
  e 381 remoções e lista exacta dos ficheiros;
- estado após autoria L0 e antes deste recibo: `git diff HEAD --stat` = 47
  ficheiros, 3707 inserções, 381 remoções. A diferença sob esta autoridade é
  exactamente os três L0s abaixo; manifesto e artefactos P1293 são untracked
  na árvore compartilhada;
- manifesto de entrada: SHA-256
  `9582de20437fb1b3a77e938061cf9be0901284508c49c42fa6bd303b02df9ce7`;
- manifesto já atualizado, antes de anexar o hash deste recibo: SHA-256
  `c81bfb556ca7f180dda4967f0733728f678c336cb7c6211e14c0ed86bd261d33`;
- measurement receipt: `p1293-textitem-ic-residual-measurement-receipt.md`,
  SHA-256 `f727ddfbc4130e3f1941a67f0493d6ab69c3e52b130133e0b29a0197b772120d`;
- implementation receipt B: SHA-256
  `785c9683a5f5725c3a0d33c27525dcdcb9638399bbdadbb12f14caad563ee4c3`;
- selo predecessor agora histórico: SHA-256
  `863602b26cb9871d7ba2d84b040fe5de2dfa3d1b73ba701524c706e9cc21e9c8`,
  bloco canônico
  `4c1b3c1799c6e56efb8780538b49df0de191b33750b9d8dbaff76ba90615f7a7`.

## Medição antes da decisão

O carrier `TextStyle.math_text_item=true` chega aos sete
`FrameItem::Text` do attach e é preservado no `TextShaped`, mas o vetor inline
qualificado mede `19,8682 × 9,6041pt` contra
`19,7681 × 9,6041pt` no vanilla: residual horizontal `+0,1001pt`.

O `tr: [R]` tem base e glifo na mesma posição nos dois binários. A diferença
é a extensão posterior ao layout:

```text
R base = 736du
R.st por ssty=1 sob script math = 829du
script = 7.7pt
SpaceAfterScript = 56du a 11pt
overflow = (829 - 736) * 7.7 / 1000 - 56 * 11 / 1000
         = 0.1001pt
```

O owner causal observado é `03_infra/src/shaper.rs`: o nível da feature em
`:525-535` e o buffer/shape em `:567-585` não consultam
`math_text_item`. Os contraprobes `K` e `W` reproduzem a fórmula
(`+0,1232pt` e `+0,3311pt`); `[RR]`, inelegível, dá delta zero. `hb-shape`
confirma que `ssty=1` só escolhe `.st` sob script OpenType `math`.

`03_infra/src/font_metrics.rs:455-463` apresenta a mesma lacuna em
`ssty_level_of`. Não causou o vetor observado porque a rota shaped respondeu,
mas pode reaplicar `.st/.sts` no fallback de `advance`/ink; é uma inferência
causal refutável e deve permanecer alinhada ao shaper.

No vanilla ratificado:

- `math/ir/resolve.rs:271-305` produz `TextItem` não numérico e
  `:327-355` produz `GlyphItem`;
- `math/text.rs:15-40` envia `TextItem` ao hbox/shaper inline;
- `math/text.rs:44-63,67-124` envia números por carácter e símbolos à rota
  `GlyphFragment`;
- `math/shaping.rs:168-212` força script OpenType `math` somente nessa rota;
- `text/mod.rs:1457-1462` pede `ssty` pelo math-size, mas no shaper inline a
  feature math-only fica inerte.

Hashes das fontes vanilla: `resolve.rs`
`115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112`,
`item.rs` `273dbf55cb714ad7e40aa13c4648293260eee8d2818001952fbf21f9a4180b3a`,
`math/text.rs` `c913d5620f91e1c747cecd3e05283245c1e558f9db280a193cc69972941b15e8`,
`math/shaping.rs` `ac656e97fd662e2b4705b8b222adae3d7c00df11928bde4c9cfcfbd896afa713`,
`inline/shaping.rs`
`673ae33f6b1e4100e9522e252fa1dcd3f3b28726b26d7c1b3e1674d7b84fc68b`
e `text/mod.rs`
`8eec4f723d9eca5602f3af0460cdc3c8507d6c0d2d3bfe109fd012ba95b7f444`.

## Classificação e novo gate

ADR-0107: `TextItem` versus `GlyphItem` é morfologia da linguagem; o carrier,
o pedido GSUB e a cache são mecânica interna. ADR-0108: a decisão sucede a
medição `file:line`, distingue medido de inferido e registra refutadores.

A confirmação humana de `2026-09-01T21:28:22-03:00` autorizou
explicitamente “consume it only for IC/cache”. O L0 público vigente dizia que
o bit “suprime exclusivamente a IC” e listava mudança de shaping como
refutador. Logo usar `math_text_item` para excluir `ssty` amplia materialmente
a semântica do campo público. Classificação: **ADR-0127 categoria 1, novo gate
humano obrigatório**. A confirmação anterior não transfere.

Escopo exacto a confirmar:

> O campo público já existente e default-false `TextStyle.math_text_item`
> também exclui `ssty/.st/.sts` no shaper e no fallback métrico, mantendo
> `math=true`, família/fallback, todos os demais eixos de shaping e as rotas
> MathIdent/MathText/NumberItem/GlyphFragment inalterados.

Não há segundo campo, trait, assinatura, `FrameItem`, default de linguagem,
mudança de fase, constante de fixture ou heurística nominal. Mesmo assim, a
ampliação contratual impede fluxo contínuo até confirmação.

## L0s autorados e ownership 1:1

| Owner | SHA antes | SHA após | Consumer congelado | SHA consumer |
|---|---|---|---|---|
| `00_nucleo/prompts/entities/layout_types.md` | `c7243b67dc03e51cfe733d6f1bc455dc946c16bba5a27854cd42c802b37838d4` | `e05bcef4537fafa5ae237922b4237ba35def074187a796db024d0033b2361e1c` | `01_core/src/entities/layout_types.rs` | `38761a95d2b39d34868af3bb5bbfd58dd9404b902c2d242da93ebe6a94b7a1ed` |
| `00_nucleo/prompts/infra/shaper.md` | `3b5c41daea269720e0aac8ea546aae4b31d8bb43d8831d5094e030d5ddbe24f2` | `fbeaa9aa47bfc6ba97f2f163ba84e8728395ad3c8121cc5cf776aad5869b5456` | `03_infra/src/shaper.rs` | `e5231d6bbeb2235f6582c04bd385faabbf0a214405903ec88bbdec05ce54c185` |
| `00_nucleo/prompts/infra/font_metrics.md` | `4fac5e047b87f83a7f06b3f59f062308d7d1f9d67fd51347e8e26aeb2c093190` | `93dfd7de5951cdb95bdad2367a6696cc17156dbb949da8f4821465bc11aa86a1` | `03_infra/src/font_metrics.rs` | `67fd998c7ff584f19aecf65daed4c1b74a245221a3d2ce3905f4c3bc200de6ef` |

Cada owner permanece 1:1 com o consumer indicado; não há owner 1:N nem novo
Núcleo. `attach.md`/`attach.rs` não foram alterados: a diferença `br` medida é
ortogonal e não dominante, sem refutador novo.

## Gates sem escrita de produto

| Comando | Resultado |
|---|---|
| `python3 -m json.tool p1293-manifest.json` | PASS |
| `crystalline-lint --checks V5 --fail-on warning .` | PASS, zero violações |
| `crystalline-lint --checks V15,V26 --fail-on warning .` | PASS, zero violações |
| `crystalline-lint --fix-hashes --dry-run .` | PASS read-only; exatamente três drifts esperados |

Dry-run exacto, sem write:

```text
layout_types.rs old=039a9459 hash-a=82f0d3d7 hash-b=29f025ff
font_metrics.rs old=505b2660 hash-a=57461647 hash-b=41138876
shaper.rs old=46a80551 hash-a=cda75693 hash-b=de355de7
```

O `--fix-hashes` real está proibido antes do novo gate. Após confirmação, o
coordenador pode ressellar somente os três headers acima, executar novo gate
discriminatório e pedir selo substituto.

Allowlist produtiva proposta para o selo posterior ao resselo/gate:

```text
03_infra/src/shaper.rs
03_infra/src/font_metrics.rs
00_nucleo/diagnosticos/p1293-implementation-receipt-b.md
```

`layout_types.rs` só integra o escopo mecânico de resselo do header; não
requer mudança funcional adicional. `helpers.rs`, `attach.rs`, restantes
owners, contrato/oracle/RED/gate privados, testes, C/D, ataques e veredito
permanecem congelados. `Unknown` nunca é sucesso e bloqueia B.

O hash deste recibo é calculado externamente após a escrita e registrado no
manifesto; não cria selo nem veredito.
