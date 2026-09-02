# P1293 — recibo de reabertura B após refutador causal de `attach`

## Estado e autoridade

```text
status: READY_FOR_COORDINATOR_FIX_HASHES_AND_REPLACEMENT_GATE_SEAL
lot: B-only
contract/oracle/RED: unchanged and protected
active-seal: none; predecessor 223fbdaf... invalidated historically
product/tests/oracle: not edited
lot-C/D: forbidden
```

- papel: `autor_contrato_p1293`, autor L0/contratual segregado;
- regime: protocolo completo da skill `tekt-materializacao-segregada`, por
  capacidades e artefatos, sem isolamento técnico de leitura no filesystem
  compartilhado;
- instante de autoria/invalidação: `2026-09-01T19:58:46-03:00`;
- HEAD/branch: `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt`;
- working tree: compartilhada e não commitada. A proveniência numérica
  decisiva abaixo pertence ao recibo independente, não a uma reconstrução
  posterior deste autor.

Entradas públicas congeladas:

- manifesto predecessor: SHA-256
  `5c10d8ad9cffa0ea2f56335646236e7eb15285d5cc333606633a21049ef563a5`;
- recibo causal independente: SHA-256
  `123a7c5b58e04a8701cb50f0c9224dd4b63ddc61c6120759391db30456277386`;
- recibo parcial B: SHA-256
  `8e9ff5cfa877191cdcf9ac61da0070ea9b6e13f945040014ce99cf7374b8dd90`;
- selo ativo predecessor: SHA-256
  `223fbdaf4a38a566c01e4839f8752ade0a9ad45e6cb2d44ddb1efc4fd819ac0b`;
- contrato canônico: SHA-256
  `2bb10fa308558a82b75acabb1a188c0832ad6708e5ff0b8246af3ced52707072`;
- recibo contratual anterior, preservado: SHA-256
  `2da9111c107cc8f1f2395c2d37674454e71c69dd4500d6b3859b7cdf44d43634`;
- oráculo/RED protegidos, preservados e não abertos nesta reabertura:
  `6f695c54582dbfbf335da7ab6d69324a712494a21b4aa725c73369eaa04a85b5e` /
  `91f3e53f1f4b01b10522821018a704aec658f7c8c37b9dae3eb92c2ca7114b5e`.

Não foram editados produto, testes, oráculo, RED/discrimination receipts,
contrato canônico, ataques ou veredito. Código corrente e vanilla ratificado
foram lidos somente nos pontos `file:line` necessários para auditar a autoria;
nenhum patch candidato foi lido como proposta nem escrito.

## Medição antes da decisão

### Cauda grande: `Semantic` mistura dois referenciais

O recibo causal independente foi medido em
`2026-09-01T19:50:26-03:00`, sobre o mesmo HEAD, com working tree
compartilhada e não commitada (`git status --short` SHA-256
`41695492598973db685969ca8e3ee44b92ed47718b970cf40bfa54fcad913dee`;
`git diff HEAD --stat` SHA-256
`207429f7fe474cb7d337921065d9a5d56117856760efae23f04ed6fbedeaa74f`).
Ele reproduz exatamente os dois fingerprints congelados:

| Vetor | Vanilla | Candidato |
|---|---:|---:|
| display, seis slots | `23.2705 × 19.4843` | `31.6481 × 19.4843` |
| inline qualificado, seis slots | `19.7681 × 9.6041` | `27.0523 × 9.6041` |

A decomposição isolou base, attach vazio e post-scripts como GREEN. Apenas
pre-scripts abrem a cauda: `tl` acrescenta `6,0368pt`, `bl` acrescenta
`8,9936pt`, e top+bottom conserva o máximo do lado esquerdo. As posições
relativas e os paths permanecem.

O caminho causal corrente é:

1. `compiler/layout/equation.rs:454-472` envolve os itens, preservando base
   primeiro e pre-script depois;
2. `compiler/layout/helpers.rs:28-30` usa o primeiro filho como posição do
   `Semantic`;
3. `helpers.rs:60-69` mede a largura como
   `right(children) - min_x(children)`;
4. `helpers.rs:77-87` soma novamente posição e largura.

O resultado é `first_child_x + rightmost_child_right - min_child_x`, e não o
limite direito dos filhos. A hipótese H1 é refutada se a diferença não for
exatamente `first_child_x - min_child_x` ou se a correção exigir mover filhos;
os probes independentes preservaram H1 com `Unknown = 0`.

### Residual: proveniência `TextItem` ainda existe em `_comum/mod.rs`

Após separar a cauda semântica, o marcador seguinte mede residual
`+0,1760pt` no pre-script qualificado `[L]` e `+0,2233pt` no post-script
qualificado `[R]`. Os deltas são exatamente IC de fonte: `16du` para `x` a
11pt e `29du` para `R` a 7,7pt.

`03_infra/src/font_metrics.rs:1511-1537` acrescenta IC ao avanço de um
glifo-base singular em modo math. No vanilla, conteúdo textual não numérico é
`TextItem` (`typst-library/src/math/ir/resolve.rs:271-305`) e segue o hbox
inline (`typst-layout/src/math/text.rs:15-40`), sem `update_glyph`; folhas
matemáticas são `GlyphFragment` e conservam IC
(`math/fragment/glyph.rs:207-215`).

A auditoria corrente fecha a autoria que o medidor deixou `Unknown`:

- `compiler/math/layout/mod.rs:630-658` trata `MathIdent`/`MathText`
  separadamente;
- em `:887-903`, o `layout_node` ainda recebe o discriminante
  `Content::Text` antes de o projetar em `FrameItem::Text`;
- o trait vigente já fornece `char_italics_correction`
  (`compiler/layout/metrics.rs:166-171`), implementado sem alteração em
  `infra/font_metrics.rs:638-650,1761-1780`.

Logo `_comum/mod.rs` pode retirar somente o termo IC já incluído na largura de
um `Content::Text` de um glifo-base, antes da perda de proveniência. Isso
preserva `MathIdent`, `MathText`, `GlyphFragment`, números e formas extensíveis.
Não é necessário novo payload, trait, entidade, campo de `FrameItem`, default
ou mudança de fase. H2 é refutada se a subtração do termo já disponível não
fechar os dois resíduos, alterar uma folha math ou exigir transportar nova
proveniência; nesse caso o implementador deve parar.

## L0 atualizado primeiro

| Owner 1:1 | SHA antes | SHA após autoria | Consumer corrente, somente hash |
|---|---|---|---|
| `00_nucleo/prompts/compiler/layout/helpers.md` | `bf0f562c4141200001b0c63b246ecc2a93fa4a41fdc9966633d2062f09862c7d` | `21dab06a57c40427edc05bc9c7cc3df4d7783ebe30ac023d6293751f25ca1a70` | `01_core/src/compiler/layout/helpers.rs` `933e4d51a6d97a811512ce03861b3de789c22898d6a5de3af52af37e68633e92` |
| `00_nucleo/prompts/compiler/math/layout/_comum.md` | `09e32fbcfa6c12e88b05744041e9430f266204e71a10816f0d16442007fa5b19` | `5fced591ec80ae9e0b06bfb2ad342ff5cfc262dfddec1e49454edc67d5c2df24` | `01_core/src/compiler/math/layout/mod.rs` `312be2fd7aed72f2c6e4c90a442e21f70f2e755ef4ee093cd270f7a3dffe853b` |
| auditado, inalterado: `00_nucleo/prompts/infra/font_metrics.md` | `0ac116614bbdafadb31179892140603e327fd7827199b7aab999c8ba8a58b299` | byte-idêntico | `03_infra/src/font_metrics.rs` `2716bf725b8bb6802fcfea13ea0a0a9caa1b49278c472760e7874b087833dfd6` |

`helpers.md` agora contrata `left=min_x`, `right=max(x+width)`, posição
horizontal `left` e largura `right-left`, sem mover ou reordenar filhos.
`_comum.md` contrata a neutralização exclusivamente do IC indevido de
`Content::Text`; o backend de métricas continua canônico e inalterado para
glifos matemáticos.

Os núcleos pinados pertinentes foram lidos integralmente e não mudaram:
`layout/coordinates`, `math/callback-realization`,
`math/layout-observables` e `fonts/fallback-selection`. Ownership permanece
1:1; nenhum owner 1:N ou núcleo novo foi criado.

## Classificação ADR-0107/0108/0127

- ADR-0107: morfologia `TextItem`/`GlyphFragment` e geometria horizontal são
  língua; discriminantes, ordem de filhos e fórmulas Rust são mecânica causal;
- ADR-0108: ambas as decisões são posteriores às medições `file:line`, com H1,
  H2 e refutadores explícitos;
- ADR-0127: duas fórmulas internas de paridade em owners existentes. Não há
  API/trait/entidade/campo público/default/compatibilidade/fase novos.

Classificação: **fluxo contínuo**, sem novo gate humano. Se a materialização
requerer qualquer uma dessas superfícies públicas ou mudança de fase, a
autorização falha e um novo gate humano torna-se obrigatório.

## Invalidação histórica do selo

O selo predecessor SHA-256
`223fbdaf4a38a566c01e4839f8752ade0a9ad45e6cb2d44ddb1efc4fd819ac0b`
foi marcado inativo em `p1293-contract-seal.json` às
`2026-09-01T19:58:46-03:00`. O arquivo após a invalidação tem SHA-256
`e91fd87571fb8ec305103d5d3eb49fe7406e0ac257636e755540f3dcd4f1dab3`.
Motivo: o serial não autorizava o owner `helpers` e não continha a nova
obrigação do owner comum para `Content::Text`. O bloco canônico histórico foi
preservado; nenhum selo substituto foi emitido. B não está aprovado; C/D
continuam proibidos.

## Gates e drift esperado

Executados após autoria L0, sem escrever produto ou headers:

| Gate | Resultado |
|---|---|
| `crystalline-lint --checks V5 --fail-on warning .` | PASS, zero violações |
| `crystalline-lint --checks V15,V26 --fail-on warning .` | PASS, zero violações |
| `crystalline-lint --fix-hashes --dry-run .` | PASS, exatamente dois drifts |

Dry-run exato:

```text
helpers.rs old=b2164f22 hash-a=ecaf33a5 hash-b=f37cdeac
mod.rs     old=4a1a2d60 hash-a=5cffdac9 hash-b=059fad40
```

Qualquer drift adicional, falha V5/V15/V26 ou mudança em `font_metrics.rs` é
blocker. O coordenador deve aplicar somente esses dois ressellos e executar
novo gate discriminatório antes de qualquer selo serial substituto.

## Allowlist proposta para o próximo selo B

Escrita mínima do implementador, somente após resselo + gate + novo selo:

1. `01_core/src/compiler/layout/helpers.rs` — referencial horizontal único de
   `FrameItem::Semantic`;
2. `01_core/src/compiler/math/layout/mod.rs` — neutralização do IC somente na
   projeção direta de `Content::Text`;
3. `00_nucleo/diagnosticos/p1293-implementation-receipt-b.md` — evidência e
   hashes, sem valor de aprovação.

`03_infra/src/font_metrics.rs`, `compiler/math/layout/attach.rs`, traits,
entidades, `FrameItem`, demais owners, prompts, testes, contrato/oracle/RED,
gate, manifesto, ataques, veredito e lotes C/D ficam congelados. O novo selo
deve exigir os fingerprints congelados bilaterais, regressões de
MathIdent/MathText/GlyphFragment, V5/V15/V26, dry-run e `Unknown = 0`, seguido
de julgamento independente.
