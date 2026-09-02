# P1293 — recibo de reabertura B por colisão de eixos de estilo `TextItem`

## Estado e autoridade

```text
status: WAITING_FOR_NEW_HUMAN_GATE_ADR_0127_CATEGORY_1
lot: B-only
cause: BLOCKED_TEXTITEM_STYLE_AXIS_COLLISION
contract/oracle/RED: unchanged and protected
active-seal: none; predecessor 728fa489... invalidated historically
product/tests: not edited
fix-hashes: not run
lot-C/D: forbidden
```

- papel: `autor_contrato_p1293`, autor L0/contratual segregado;
- regime: protocolo completo da skill `tekt-materializacao-segregada`, por
  capacidades e artefatos, sem isolamento técnico de leitura no filesystem
  compartilhado;
- instante da medição/autoria/invalidação: início
  `2026-09-01T21:10:57-03:00`, fechamento
  `2026-09-01T21:14:21-03:00`;
- HEAD/branch: `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt`;
- working tree: compartilhada e não commitada; no início, `git status
  --short` SHA-256
  `5a7730faa5f9611738862a1ad0ea82f8044fd073f034501b35ef4711092e45b0`,
  62 entradas; `git diff HEAD --stat` SHA-256
  `f8e065afa63c274112e2bfed76f3d9e457222ebb86cb1e36019d1709cd979bff`,
  `38 files changed, 3126 insertions(+), 373 deletions(-)`.

Entradas congeladas:

| Entrada | SHA-256 |
|---|---|
| manifesto predecessor | `adf445ceef4ed042dd0b65c3e3fe24e6b8d589a27a9ff7155736a81649facb13` |
| recibo B bloqueado | `bab06da4eb0db5dd0a37d2c679a6b51bef0d321c64128e56c536e5ba46bba6ce` |
| recibo causal independente | `123a7c5b58e04a8701cb50f0c9224dd4b63ddc61c6120759391db30456277386` |
| selo serial predecessor | `728fa489f75840b6e9dc65810a3f4a45d801a31e5ec1fc2ecf61ee31eb96a088` |
| bloco canônico predecessor | `d90b48a01933bf0a1352b7e2deeba47c3a5f65d4a6b60a9eb16ada0b7083f2ad` |
| contrato canônico | `2bb10fa308558a82b75acabb1a188c0832ad6708e5ff0b8246af3ced52707072` |
| recibo contratual preservado | `2da9111c107cc8f1f2395c2d37674454e71c69dd4500d6b3859b7cdf44d43634` |
| oracle protegido, hash-only | `6f695c54582dbfbf335da7ab6d69324a712494a21b4aa725c73369eaa04a85b5` |
| RED protegido, hash-only | `91f3e53f1f4b01b10522821018a704aec658f7c8c37b9dae3eb92c2ca7114b5e` |

Não foram editados produto, testes, contrato canônico, oracle, RED/gate
discriminatório, ataques ou veredito. O produto corrente foi consultado
read-only somente para mapear carriers e literais exaustivos; a decisão
observável deriva dos recibos públicos e da fonte vanilla, não foi adaptada a
um patch. Não há atestação de isolamento técnico de leitura.

## Medição antes da decisão

### Refutador da projeção `math=false`

O recibo B, medido em `2026-09-01T21:02:48-03:00`, registra:

- testes próprios: `29/29` GREEN;
- sete dos oito vetores B-P07: GREEN;
- attach display: `23,2705 × 19,4843`, GREEN exato;
- attach inline qualificado: vanilla `19,7681 × 9,6041`, candidato
  `19,1521 × 9,6041`, RED `-0,6160pt`;
- estado anterior do mesmo vetor: `19,8154pt`; `math=false` retirou
  `0,6633pt`, embora o residual anterior fosse apenas `+0,0473pt`;
- `Unknown = 1`, B não aprovado, C/D proibidos.

A mudança foi causalmente efetiva, mas excessiva: `style.math` controla tanto
a cadeia/fonte/shaping matemático como a aplicação da IC. Logo ele não pode
representar sozinho a proveniência `TextItem`.

### Fonte vanilla ratificada e carriers cristalinos

Fontes vanilla auditadas:

| Fonte | SHA-256 | Medição `file:line` |
|---|---|---|
| `typst-library/src/math/ir/resolve.rs` | `115d775641509a755b19e0e23e303d38d4a7cd76e0112` | `TextElem` separado de `SymbolElem` em `149-161`; texto não numérico vira `TextItem` em `271-305`; símbolo vira `GlyphItem` em `327-355` |
| `typst-library/src/math/ir/item.rs` | `273dbf55cb714ad7e40aa13c4648293260eee8d2818001952fbf21f9a4180b3a` | `TextItem` é Alphabetic/spaced em `881-904` |
| `typst-layout/src/math/text.rs` | `c913d5620f91e1c747cecd3e05283245c1e558f9db280a193cc69972941b15e8` | `TextItem` usa inline hbox preservando styles em `15-40`; número/glifo usam `GlyphFragment` em `44-64,67-123` |
| `typst-layout/src/math/fragment/glyph.rs` | `1b57225c90db398ecf9a9af7c4bef47285eb5e434a554948bd086cbe7ad411fb` | IC entra somente no x-advance do glifo não extensível em `207-215` |
| `typst-layout/src/math/shaping.rs` | `ac656e97fd662e2b4705b8b222adae3d7c00df11928bde4c9cfcfbd896afa713` | shaper de glifo força script `math` em `168-212` |
| `typst-layout/src/inline/shaping.rs` | `673ae33f6b1e4100e9522e252fa1dcd3f3b28726b26d7c1b3e1674d7b84fc68b` | inline resolve styles em `784-815` e infere script em `954-991` |

No cristalino:

1. `TextStyle` é público e viaja em `FrameItem::Text`/`TextShaped`
   (`entities/layout_types.rs:155-170,215-248,347-385`);
2. `line_content_right` recebe somente o item e remede `Text` por
   `text_width(text, style.size, style)`
   (`compiler/layout/metrics.rs:265-295`);
3. `FontBookMetrics::advance` soma IC sob `style.math` para glifo-base singular
   (`font_metrics.rs:495-547`);
4. `FallbackFontMetrics::advance` usa `style.math` para a cadeia math
   (`:1423-1440`) e novamente para IC (`:1511-1537`);
5. `AdvanceWidthKey` distingue `math`/`math_size`, mas não proveniência
   `TextItem` (`:977-997,1151-1163`).

Depois que `Content::Text` vira `FrameItem::Text`, não existe carrier privado
que alcance a remedição. A distinção tem de atravessar a entidade pública ou
uma superfície pública maior.

### Alternativas e refutadores

| Alternativa | Resultado |
|---|---|
| `math=false` | refutada por `-0,6160pt`; altera fallback/shaping além da IC |
| reutilizar `math_script`, `cramped`, sub/sup ou outro campo | rejeitada: eixos com semântica independente e combinações válidas próprias |
| novo parâmetro em `FontMetrics::advance` | contrato público de trait e não resolve sozinho o transporte por `FrameItem` |
| novo campo/variant em `FrameItem` | contrato público maior e duplicação do carrier de estilo já presente |
| tabela lateral por identidade | não sobrevive clones/transforms; exigiria estado/acoplamento proibido |
| `TextStyle.math_text_item: bool=false` | menor forma causal: preserva `math=true` e separa somente IC |

Inferência escolhida: o estado `math=true && math_text_item=true` representa
`TextItem` dentro de math; `math=true && math_text_item=false` representa
glifo/número math. Refutadores: mudança de fonte/shaping, cache colidente,
qualquer MathIdent/MathText/número afetado ou residual não zerado. Refutador
presente bloqueia; não autoriza heurística nominal nem novo owner implícito.

## Decisão L0 primeiro — bloqueada no gate humano

Campo público proposto:

```text
TextStyle.math_text_item: bool
default: false
```

Regras:

- somente `Content::Text` direto em math define `true`, mantendo `math=true`;
- o mesmo estilo chega a largura, ink bounds, MathBox e FrameItem;
- `FontMetrics::advance` soma IC somente quando
  `math && !math_text_item && singular-base`;
- `AdvanceWidthKey` inclui o novo bit;
- MathIdent, MathText, números, GlyphFragment, extensíveis e
  MathClassOverride promovido mantêm `false`;
- nenhum `StyleDelta`, namespace de utilizador, default de linguagem, trait,
  FrameItem, fase ou heurística nominal é criado.

ADRs:

- ADR-0107: `TextItem`/`GlyphItem` é morfologia de linguagem; o bit Rust é
  mecânica interna de preservação;
- ADR-0108: decisão posterior ao refutador numérico e às vias `file:line`;
- ADR-0129: cada edição futura permanece sob owner 1:1 próprio;
- **ADR-0127 categoria 1**: novo campo público em entidade, paragem
  obrigatória antes de código, independentemente de default `false`.

## Owners 1:1 e hashes

Cinco L0s são necessários. Os três primeiros são semânticos; os dois últimos
são causalmente obrigatórios porque possuem literals exaustivos de
`TextStyle` que precisam de default/propagação explícitos.

| Owner 1:1 | SHA antes | SHA após autoria | Consumer congelado | SHA consumer |
|---|---|---|---|---|
| `00_nucleo/prompts/entities/layout_types.md` | `3fcc63a2afc79fbed2df95993048ad1d71b7b44b44bb5b3ba453a13319008a93` | `c7243b67dc03e51cfe733d6f1bc455dc946c16bba5a27854cd42c802b37838d4` | `01_core/src/entities/layout_types.rs` | `d5e4b948de6ec75f11ec754dc6a8d987193051ac1005b1d96837ad165a3e1ffb` |
| `00_nucleo/prompts/compiler/math/layout/_comum.md` | `7bda2815c1f0f904c764e29d52486cf87b2368f2da0194c9e7e37f690e6a2ce9` | `50eac6e08e2e5fd4cb4e0bcde6f66b601d5a5c7784e287c6c498041f4a8de94b` | `01_core/src/compiler/math/layout/mod.rs` | `4146b834d72c948c219f4b418f7e60c9d48ae1ec091b474bce1342ccb05f654b` |
| `00_nucleo/prompts/infra/font_metrics.md` | `0ac116614bbdafadb31179892140603e327fd7827199b7aab999c8ba8a58b299` | `4fac5e047b87f83a7f06b3f59f062308d7d1f9d67fd51347e8e26aeb2c093190` | `03_infra/src/font_metrics.rs` | `2716bf725b8bb6802fcfea13ea0a0a9caa1b49278c472760e7874b087833dfd6` |
| `00_nucleo/prompts/entities/style_chain.md` | `5aa47f5fcf53788030583f67c23c82f3199f9df6afb70efa5aae28af4676f5a4` | `1efe881899ab5cdc290be6350e2da6f5e3ea8cbff60ab4e65e01585ec28cdf6c` | `01_core/src/entities/style_chain.rs` | `b49594292bcf9bf89cefe173f852f35f988a6350d73828d6d5cbb3ad0ef48185` |
| `00_nucleo/prompts/compiler/layout/text.md` | `96f52d3c8e9c94dbf593b356e691aad12b5a7272c7e5ada0507dd7fa2fb62fd1` | `94a1cab40ec6c458e5dc057d7a515d31eb85b941d4cf9791d5108fb1abffdc62` | `01_core/src/compiler/layout/text.rs` | `0ac50ea2d6079bd2c610ea7446af9926f151e1f1a3613925e872ae2b79b9aeeb` |

Os Núcleos Tekt pinados foram lidos e permanecem byte-idênticos; não foi
criado Núcleo novo nem owner 1:N. `compiler/layout/metrics.rs` não muda: ele já
propaga a `TextStyle` ao método de largura. `infra/shaper.rs` não muda: o bit
não seleciona shaping. Nenhum outro owner foi legitimado.

## Invalidação histórica do selo

O selo serial SHA-256
`728fa489f75840b6e9dc65810a3f4a45d801a31e5ec1fc2ecf61ee31eb96a088`
foi marcado inativo em sua metadata, sem alterar o bloco canônico serial
`d90b48a01933bf0a1352b7e2deeba47c3a5f65d4a6b60a9eb16ada0b7083f2ad`
nem o contrato canônico
`2bb10fa308558a82b75acabb1a188c0832ad6708e5ff0b8246af3ced52707072`.
O arquivo após invalidação tem SHA-256
`2a0f0cc101d8f39caf8bfb2eba2c4cd45fbd4032aec8a5194858a58c45d673cc`;
zero metadata de selo permanece ativa. Nenhum selo novo foi emitido.

## Gates e dry-run sem escrita

| Gate | Resultado |
|---|---|
| JSON do selo invalidado | PASS |
| `crystalline-lint --checks V5 --fail-on warning .` | PASS, zero violações |
| `crystalline-lint --checks V15,V26 --fail-on warning .` | PASS, zero violações |
| `crystalline-lint --fix-hashes --dry-run .` | PASS, exatamente cinco drifts esperados; nenhum write |
| `git diff --check` | PASS |

Dry-run exato:

```text
layout/text.rs old=218dd82f hash-a=f23d45b2 hash-b=6a5949ed
math/layout/mod.rs old=e97ae2df hash-a=289307df hash-b=6fd00e5c
entities/layout_types.rs old=ad66aa56 hash-a=039a9459 hash-b=76c11fc6
entities/style_chain.rs old=46ec8cf7 hash-a=c08b7ab8 hash-b=2daa3854
infra/font_metrics.rs old=5f1a8078 hash-a=505b2660 hash-b=a1d371e8
```

`--fix-hashes` não foi executado, conforme a trava. Qualquer sexto drift,
mudança de consumer antes da confirmação, alteração de contrato/oracle/RED ou
falha V15/V26 invalida esta candidatura.

## Escopo exato solicitado ao humano

Confirmar ou rejeitar **somente**:

1. adicionar `pub math_text_item: bool` a `TextStyle`, default `false`;
2. setar `true` exclusivamente em `Content::Text` direto dentro de math,
   mantendo `math=true` e todos os demais eixos;
3. usar o bit apenas para excluir IC de glifo singular e na chave de cache;
4. adaptar os dois literals exaustivos: `StyleChain` fixa `false` e o merge de
   `layout/text` preserva o valor;
5. depois da confirmação, permitir ao coordenador ressellar exatamente os
   cinco pares e exigir novo gate discriminatório antes de qualquer selo ou
   escrita de produto.

A confirmação não aprova o lote B, não autoriza C/D, não autoriza novo
payload/trait/FrameItem/default/fase e não autoriza implementação antes do
resselo e do selo serial substituto.
