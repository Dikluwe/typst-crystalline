# P1293 — recibo de reabertura B por perda de proveniência `TextItem`

## Estado e autoridade

```text
status: READY_FOR_COORDINATOR_FIX_HASHES_AND_REPLACEMENT_GATE_SEAL
lot: B-only
cause: BLOCKED_TEXTITEM_PROVENANCE_LOSS
contract/oracle/RED: unchanged and protected
active-seal: none; predecessor 932eb2b0... invalidated historically
product/tests: not edited
lot-C/D: forbidden
```

- papel: `autor_contrato_p1293`, autor L0/contratual segregado;
- regime: protocolo completo da skill `tekt-materializacao-segregada`, por
  capacidades e artefatos, sem isolamento técnico de leitura no filesystem
  compartilhado;
- instante da decisão/invalidação: `2026-09-01T20:33:33-03:00`;
- HEAD/branch: `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt`;
- working tree: compartilhada e não commitada.

Entradas congeladas:

- manifesto predecessor: SHA-256
  `f38dc66dfffe2ead5c1f640edbbf7484783fb838678c34b0619a90a5d6b5bcad`;
- recibo B bloqueado: SHA-256
  `8141334dfbb81315818e2926c2749dc23124b51fd359f7130e2a8d680d9dd6f4`;
- recibo causal independente: SHA-256
  `123a7c5b58e04a8701cb50f0c9224dd4b63ddc61c6120759391db30456277386`;
- selo predecessor: SHA-256
  `932eb2b0b617c31c2c3870424113047543f69908957709b0786876f44ec87f15`;
- bloco canônico predecessor: SHA-256
  `a78b88d65de062d02b2012db42b22d9ebc372fb0fac2e7da626fb7197232d635`;
- contrato canônico: SHA-256
  `2bb10fa308558a82b75acabb1a188c0832ad6708e5ff0b8246af3ced52707072`;
- recibo contratual, preservado: SHA-256
  `2da9111c107cc8f1f2395c2d37674454e71c69dd4500d6b3859b7cdf44d43634`;
- oráculo/RED protegidos, somente hash e inalterados:
  `6f695c54582dbfbf335da7ab6d69324a712494a21b4aa725c73369eaa04a85b5e` /
  `91f3e53f1f4b01b10522821018a704aec658f7c8c37b9dae3eb92c2ca7114b5e`.

Não foram lidos nem editados patch candidato, produto, testes, oráculo,
RED/discrimination receipts, contrato canônico, ataques ou veredito. A fonte
vanilla ratificada foi lida read-only; os consumers atuais foram verificados
somente por hash nesta autoria.

## Medição antes da decisão

### Refutador do primeiro reparo

O recibo B, medido em `2026-09-01T20:28:31-03:00` sobre o mesmo HEAD e
working tree não commitada (`git status --short` SHA-256
`27397d0a889680a07c6d9006aa05954af0c97e823bb6f49bfb7e66121129a697`),
registra:

- reparo do referencial `Semantic`: GREEN próprio e bilateral display;
- sete dos oito B-P07: GREEN;
- attach inline qualificado: vanilla `19,7681 × 9,6041`, candidato
  `19,8154 × 9,6041`, residual bloqueante `+0,0473pt`;
- `Unknown = 1`, sem aprovação de B.

A neutralização de IC na largura do `MathBox` funcionou localmente. A extensão
final, porém, remede o `FrameItem::Text` emitido:

```text
line_content_right
  -> item_width(FrameItem::Text, style.math=true)
  -> metrics.advance(text, size, style)
```

Como o mesmo estilo `math=true` sobreviveu no item, a via de glifo matemático
reintroduziu IC depois que o discriminante `Content::Text` foi apagado. Logo a
estratégia anterior de alterar só `MathBox.width` está refutada; transportar
um número corrigido sem a morfologia não fecha a remedição posterior.

### Auditoria vanilla ratificada

Fontes e hashes:

| Fonte | SHA-256 |
|---|---|
| `typst-library/src/math/ir/resolve.rs` | `115d775641509a755b19e0e23e303d38d4a7cd76e0112` |
| `typst-library/src/math/ir/item.rs` | `273dbf55cb714ad7e40aa13c4648293260eee8d2818001952fbf21f9a4180b3a` |
| `typst-layout/src/math/text.rs` | `c913d5620f91e1c747cecd3e05283245c1e558f9db280a193cc69972941b15e8` |
| `typst-layout/src/inline/shaping.rs` | `673ae33f6b1e4100e9522e252fa1dcd3f3b28726b26d7c1b3e1674d7b84fc68b` |
| `typst-layout/src/math/shaping.rs` | `ac656e97fd662e2b4705b8b222adae3d7c00df11928bde4c9cfcfbd896afa713` |

Medição `file:line`:

1. `resolve_realized` separa `TextElem` de `SymbolElem`
   (`resolve.rs:149-161`);
2. texto não numérico vira `TextItem` com a cadeia de estilos e bounds locais;
   números viram `NumberItem` (`resolve.rs:28-36,271-305`); símbolos viram
   `GlyphItem` (`:327-355`);
3. `TextItem` é Alphabetic/spaced (`item.rs:881-904`) e `layout_text` preserva
   os styles ao chamar `inline::layout_inline` (`math/text.rs:15-40`);
4. o shaper inline resolve tamanho e demais eixos da cadeia
   (`inline/shaping.rs:784-815`) e não força script `math`: só honra script
   textual explícito e, de resto, deixa o buffer inferi-lo (`:954-991`);
5. `GlyphItem` segue `layout_glyph`/`GlyphFragment`
   (`math/text.rs:67-123`), cujo shaper força script `math`
   (`math/shaping.rs:168-212`). `NumberItem` também usa `GlyphFragment` por
   caractere (`math/text.rs:44-64`).

A fonte sustenta que `TextItem` conserva tamanho/estilos matemáticos resolvidos,
mas não usa a mecânica de glifo matemático. Portanto o discriminador já
existente em `TextStyle` pode transportar essa morfologia no próprio
`FrameItem`; não é necessário campo, trait, entidade ou variante nova.

## Decisão L0 primeiro

Somente `00_nucleo/prompts/compiler/math/layout/_comum.md` foi atualizado:

| Owner 1:1 | SHA antes | SHA após autoria | Consumer atual, somente hash |
|---|---|---|---|
| `_comum.md` | `7f4ff5c22de793ce1e8641843aa213d758b5edffe51a04044f20dee817f88414` | `dfb46c0a49e5f1e8aa5df2d1920a82f6c89d566c6c0839846992f3888b200333` | `01_core/src/compiler/math/layout/mod.rs` `c48f0cf5f00b17a801683519cd4648ef909dbd0ad9b53cb76133cd4006a1617e` |

O L0 agora substitui somente a estratégia anterior de largura: para
`Content::Text` direto, copiar o estilo efetivo integral e desativar apenas a
via de glifo matemático (`math=false`, ou equivalente estrito) antes de medir
e emitir. O mesmo estilo textual acompanha `MathBox` e `FrameItem::Text`, de
modo que remedições posteriores continuam na via inline/textual e não
reintroduzem IC. Não há subtração manual adicional.

`MathIdent`, `MathText`, número, `GlyphFragment`, formas extensíveis e
`MathClassOverride(Content::Text)` promovido a glifo mantêm `math=true`.
Tamanho, `math_size`, cramped, fonte, peso, itálico, bold, fill, tracking,
leading, idioma e demais eixos existentes são preservados.

Owners congelados e byte-idênticos nesta autoria:

| Owner/consumer | SHA-256 |
|---|---|
| `compiler/layout/helpers.md` | `7481a304279c26a6df3ff9c5258e883d2f5a45eba53b8de2c717f543a68f2c6c` |
| `compiler/layout/helpers.rs` | `91a0bfddc52592427d9403d8cc5a6956b396c8e30c947c0a93bae035d19c8116` |
| `infra/font_metrics.md` | `0ac116614bbdafadb31179892140603e327fd7827199b7aab999c8ba8a58b299` |
| `infra/font_metrics.rs` | `2716bf725b8bb6802fcfea13ea0a0a9caa1b49278c472760e7874b087833dfd6` |
| `math/layout/attach.md` | `db1abcc014473353baab0ed8d678b9449cd30a3668f98cb456f18b20cd2078af` |
| `math/layout/attach.rs` | `4a3b6195cacbba955a70f7011b711bc2b39c2d025841344810ae26c96653e8f5` |

Ownership continua 1:1; núcleos pinados por `_comum.md` não mudaram e V26
permanece verde. Não foi criado owner compartilhado nem núcleo novo.

## Classificação ADR-0107/0108/0127

- ADR-0107: TextItem/GlyphItem é morfologia; `TextStyle.math` é mecanismo
  interno já existente para conservar a via de shaping/avanço;
- ADR-0108: a decisão segue o residual, o caminho de remedição e as duas vias
  vanilla `file:line`; refutadores são residual, perda de estilo/tamanho,
  regressão de glifos math ou necessidade de contrato novo;
- ADR-0127: correção interna de paridade no owner existente, sem trait,
  entidade, `FrameItem`, campo público, default, compatibilidade ou fase.

Classificação: **fluxo contínuo**, sem novo gate humano. Qualquer necessidade
de superfície pública ou mudança de fase torna a solução `Unknown`, invalida a
allowlist e exige parada para gate humano.

## Invalidação histórica do selo

O selo predecessor SHA-256
`932eb2b0b617c31c2c3870424113047543f69908957709b0786876f44ec87f15`
foi marcado inativo em `p1293-contract-seal.json` às
`2026-09-01T20:33:33-03:00`. O arquivo após invalidação tem SHA-256
`17898ee3553eaad492b124feded0ccff0a138ac95e88208296833920fbf7f0ca`.
O bloco canônico predecessor permanece histórico e byte-conceitualmente
preservado; nenhum selo novo foi emitido. B não está aprovado; C/D continuam
proibidos.

## Gates e drift

Executados depois da autoria L0, sem escrever consumer/header:

| Gate | Resultado |
|---|---|
| `crystalline-lint --checks V5 --fail-on warning .` | PASS, zero violações |
| `crystalline-lint --checks V15,V26 --fail-on warning .` | PASS, zero violações |
| `crystalline-lint --fix-hashes --dry-run .` | PASS, exatamente um drift |
| `git diff --check` | PASS |

Dry-run exato:

```text
mod.rs old=5cffdac9 hash-a=e97ae2df hash-b=877e5c93
```

Qualquer segundo drift, mudança em helpers/font_metrics/attach ou falha
V5/V15/V26 é blocker. O coordenador deve aplicar somente esse resselo e rodar
novo gate discriminatório antes de novo selo serial.

## Allowlist proposta para o próximo selo B

Somente:

1. `01_core/src/compiler/math/layout/mod.rs` — projetar `Content::Text` direto
   desde o início pela via textual no estilo existente;
2. `00_nucleo/diagnosticos/p1293-implementation-receipt-b.md` — evidência,
   hashes e capabilities, sem aprovação.

`helpers.rs`, `attach.rs`, `font_metrics.rs`, demais consumers/owners,
prompts, manifesto, selo, testes, contrato/oracle/RED/gate, ataques, veredito e
C/D ficam congelados. O novo selo deve exigir o oitavo B-P07 GREEN exato,
preservação dos sete já GREEN e guardas de TextItem/GlyphItem, `Unknown = 0`,
V5/V15/V26 e parada para julgamento independente.
