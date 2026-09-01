# P1286 — receipt do contrato L0 independente

**Natureza:** contrato candidato ex-ante; não é implementação, teste, oráculo,
ataque, selo nem veredito de materialização.

**Revisão:** v2 — saneamento perene das cláusulas normativas antigas. Substitui
o receipt v1 SHA-256
`5990fadb5c8ba60e452c1cb862cf49dd8b8c9da103bfcb6ab64b334081f0bd27`;
baseline, receipts de entrada e política de `Unknown` permanecem pinados.

## 1. Regime, papel e capacidades

- Regime: protocolo completo da skill `tekt-materializacao-segregada`.
- Papel: autor independente do contrato/L0 `/root/contrato_l0_p1286`.
- Entradas autorizadas: `AGENTS.md`, skill e referências diretas, ADR-0107,
  ADR-0108, ADR-0127 e ADR-0129; somente o passo explicitamente autorizado
  `00_nucleo/materialization/typst-passo-1286.md`; receipts P1286 de medição e
  ownership; fontes vanilla pinadas; sources cristalinos e L0s proprietários
  afetados.
- Escritas autorizadas: somente os L0s listados no manifesto e este receipt.
- Entradas proibidas respeitadas: nenhum outro ficheiro de
  `00_nucleo/materialization/` ou `00_nucleo/context/` foi listado ou lido.
- Capacidades negadas: escrever código, testes, oráculos, mutações, ataques,
  resselo de hashes de código ou veredito.
- Ambiente: checkout compartilhado; portanto **executado sem atestação de
  isolamento físico do host**. A separação de papel/capacidade foi respeitada.

## 2. Manifesto congelado

Snapshot contratual em 2026-08-30, `HEAD
53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, working tree não commitada.

| entrada | identidade |
|---|---|
| passo P1286 | `02f9393468696b6196338d0c9832f39e3228176cf1c475e6ccdd754e873d11cf` |
| baseline ratificado | `upstream/main a51e02804` (associação independente dos bytes locais permanece `Unknown`) |
| receipt de medição vanilla | `e16ea033bb6216f04813ace238395649c7d32b7251af14ac98d337525cf53ef8` |
| receipt de ownership | `76e33f4e6abec687b3d4670a468cc030c8569bc21264bd54513ababb474be6af` |
| binário vanilla medido | `/usr/local/bin/typst`, SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| receipt contratual predecessor v1 | `5990fadb5c8ba60e452c1cb862cf49dd8b8c9da103bfcb6ab64b334081f0bd27` |

Pins das fontes vanilla efetivamente decisivas:

| fonte | SHA-256 |
|---|---|
| `text/smartquote.rs` | `5ecdcbff06730194a01fc8d8da1acc5564ea914efdefb6b550d5cd0dfe286a1d` |
| `visualize/line.rs` | `5816aaffa4a339799fec9e69f6a87b31953a7401b3bcb711b6bc83beac9d84ba` |
| `typst-layout/src/shapes.rs` | `875df8a8b9775d305bd05be60cb3e2889a671053990bb543ec7f70912ff7d35f` |
| `visualize/color.rs` | `80473eba7460cb0f398e7937946e6412c1a8cb1cadffe00580aea9b48f6c0713` |
| `pdf/attach.rs` | `525d2505b229baa9ae1a6078de290bddaeba23a4378304c695aa36c55f63aaed` |
| `pdf/accessibility.rs` | `dba1d5f5e4fe3e8c24376cd96616be0f183926fa15b6303ee75c3806e8567d51` |
| `typst-pdf/src/attach.rs` | `7d88f3efd8579ccd8b91070c408a6ba139715acd9a01911e5d43e7e513005a13` |
| `typst-layout/src/rules.rs` | `72b34f1640483893fd2f5da814f6260921b3786cfa60dd1046073330af12e634` |
| `typst-pdf/src/tags/tree/build.rs` | `81fb72ab34cdf9bb24629e32133c00859a386680544473f125cf845a586f1045` |
| `typst-pdf/src/tags/util/mod.rs` | `9eb9bb6154de578d872117cbca7590752598de09ee1810cd80eaeed1b41caef2` |

## 3. L0s alterados pelo contrato

Os seguintes owners existentes foram atualizados; `Hash do Código` foi
deliberadamente preservado e nenhum header produtivo foi ressellado:

| fatia | L0s proprietários alterados |
|---|---|
| smartquote | `compiler/stdlib/text/smartquote.md`, `compiler/layout/smartquote.md`, `compiler/lang/quotes.md`, `compiler/eval/rules.md`, `compiler/eval.md`, `entities/elements/smartquote.md`, `entities/content.md` |
| line | `compiler/stdlib/shapes.md` |
| color.mix | `compiler/stdlib/color.md`, `entities/color.md` |
| PDF | `compiler/stdlib/pdf.md`, `entities/content.md`, `entities/layout_types.md`, `compiler/layout.md`, `infra/export/builder.md`, `infra/export/stream.md`, `infra/pipeline.md` |

Nenhum Prompt L0 novo foi criado: os consumers propostos
`entities/elements/pdf_{attach,artifact}.rs` e layouts correspondentes ainda
não existem e um prompt materializável agora seria órfão V15. Pós-confirmação,
cada consumer novo exige owner 1:1 próprio antes de código. Nenhum Núcleo foi
criado; os pins vigentes permaneceram byte-idênticos.

Hashes SHA-256 dos L0s no snapshot contratual, sem resselo de headers de
código:

| L0 | SHA-256 |
|---|---|
| `compiler/stdlib/text/smartquote.md` | `8200a170e2f5a8efd734d89743e7051116e35f5f28a4606e493e6493ef8a48c0` |
| `compiler/layout/smartquote.md` | `9b260fa42ecc1dd3a5229773704f0638d99a005312a98de5544851e9b1afa9da` |
| `compiler/lang/quotes.md` | `d303c271dec7b92ed1feadeb24c4e63db804e2b959f13d0a3eaa40c51c46c48e` |
| `compiler/eval/rules.md` | `d5bdd9b0bd4e95a06082216b1b5f261b66064dc7653391e294f584d2de714f21` |
| `compiler/eval.md` | `de0d8018d85d75e990746e49e35faa1025c0a244b4fff5f1639eb2edb3b802f8` |
| `entities/elements/smartquote.md` | `d217b2f408e58c0b896c64f91acd0cfb3d4e11f193f828babdcb9cd9aba2062e` |
| `compiler/stdlib/shapes.md` | `8746d996872e58b432cbe29994da5fec4b9edaae8d27c658cac1b31f8cceccbb` |
| `compiler/stdlib/color.md` | `3b5c3f4d161acef90bd13079ca2656425c12bdcbd19787a5904cb03cc96f1ff6` |
| `entities/color.md` | `f31ea87db04e18cb3fff4fe85ffebd881b18920f659ba3ccae21cd551a76d468` |
| `compiler/stdlib/pdf.md` | `cc9f72f371dc767570a021c4512b73dd7deba785469535051e88fb40ffaac0da` |
| `entities/content.md` | `be1188f3073cd470a0aa51b63bcc8a2f162b954ddc2a3e7de30ed19e25a2db3d` |
| `entities/layout_types.md` | `1df6881d8d9d692213d5cbd005f05782c846493f03c00b347324ec000cda1fd6` |
| `compiler/layout.md` | `94e07bca9e9fb15ee538e5559b228c8b7bf755050b5daf438a4d51ed5f37a4ff` |
| `infra/export/builder.md` | `696cc89913d6af023b328cd29fcf3cfd7d2d3cf8044b998b900dd4fbb491bddd` |
| `infra/export/stream.md` | `98c007188cac86a8093689e0680e3fc916855e698c01d4903d602c787c2a0245` |
| `infra/pipeline.md` | `316b4d88560561a8f114234c0ef2c8a4397a66604c584df4a3faecd0d6f8996e` |

## 4. Fragmentos observáveis congelados

### Smartquote

- Defaults preservados: `double=true`, `enabled=true`, `alternative=false`,
  `quotes=auto`.
- Alemão: primário `„…“`, alternativo `»…«`; quotes explícitas prevalecem.
- Quotes aceitam auto/string/array/dict single+double; string conta graphemes;
  erros de cardinalidade/chave são os medidos.
- `#set` usa `Styles`; a chamada direta permanece `Content::SmartQuote` e usa
  campos públicos opcionais do leaf. A alternativa de embrulhar a chamada em
  `Content::Styled` foi refutada: `repr_content` elimina o wrapper, mas
  `content.func()` observa o `elem_name` externo e devolveria `styled`.

### Line

- Origem positiva/negativa, vetor invertido e comprimento negativo preservam
  os pontos; `end` prevalece sobre `length`/`angle`.
- Lowering interno: path aberto com pontos absolutos e caixa
  `max(start,end,zero)`, nunca `max-min` nem `abs(delta)`.
- `dx`/`dy` legados permanecem extensão compatível fora da alegada paridade.

### Color.mix

- Uma ou mais cores; cor nua pesa 1; par `(cor,peso)` aceita float/ratio;
  pesos são relativos e soma deve ser positiva.
- N>2 é rejeitado em HSL/HSV/Oklch; process spaces usam média ponderada
  direta no espaço alvo.
- API Rust binária existente permanece; nenhum `mix_iter` público.

### Pdf.attach

- Path é sempre o primeiro posicional obrigatório; bytes explícitos são apenas
  o segundo posicional opcional. Nome virtual, bytes, MIME, description e
  relationship são preservados; marker é visualmente vazio.
- PDF normal incorpora EmbeddedFile/Filespec/name tree; duplicado é erro
  global antes do export.
- PDF/A-3 relationship foi medido no vanilla, mas permanece `Unknown` no
  cristalino sem eixo de standard PDF.

### Pdf.artifact

- Visual e texto permanecem; com tags enabled, o body fica dentro de
  `/Artifact ... EMC`, sem MCID/StructElem; `other` usa BMC.
- Header e Background têm as property lists medidas. Tags disabled torna o
  wrapper visualmente transparente e omite marcação.
- A árvore atual **já tem tagging de Formula P1140.6**; P1286 amplia a
  discriminação semântica e não parte de “ausência global de tags”.

## 5. Política de `Unknown`

`Unknown` nunca é `Preserved`, sucesso, equivalência nem permissão para
fallback. Permanecem Unknown: proveniência independente lab↔`a51e02804`;
smartquote regional/nesting/prime/apóstrofo e morfologia de `content.fields()`
para os novos argumentos; line percentual/infinito/clipping;
spot color/alpha fora da matriz; PDF standards e compressão não medidos;
fallbacks de todos os artifact kinds por versão; efeito real de artifact em
AT/screen reader, reflow e copy-paste. Em particular, detectar `/Artifact` no
PDF não prova comportamento em AT.

## 6. Classificação ADR-0127 exata

| fatia | classe | consequência |
|---|---|---|
| smartquote | novos campos/tipos públicos em `SmartQuoteElem`; carrier `Styled` refutado pela morfologia de `content.func()` | **PARAGEM OBRIGATÓRIA** |
| line start/end | correção de paridade por lowering a `ShapeKind::Path` existente | fluxo contínuo |
| color.mix N/pesos | correção interna; API Rust pública preservada | fluxo contínuo |
| pdf.attach | novo contrato público (`Content`, tipos e campo de `PagedDocument`) | **PARAGEM OBRIGATÓRIA** |
| pdf.artifact | novo contrato público (`Content`, tipos e `SemanticKind`) | **PARAGEM OBRIGATÓRIA** |

Não há mudança proposta de default do produto, ordem/fase do pipeline,
trait `World`, variante `Value`, variante nova de `FrameItem` ou assinatura
pública L1–L4. A pipeline ganha validação interna entre layout e dispatch, sem
nova fase. Em caso de rejeição dos contratos públicos, as duas fatias contínuas
não ficam automaticamente autorizadas por este receipt: ainda exigem
RED→GREEN independente e revalidação.

## 7. Lista fechada de alterações públicas/pipeline propostas

1. `SmartQuoteElem` ganha os campos públicos opcionais `alternative` e
   `quotes`; são acrescentados os tipos públicos `SmartQuotePair`,
   `SmartQuoteOverrides` e `SmartQuoteQuotes`, sem mudar a assinatura de
   `Content::smartquote(double)`.
2. `Content::PdfAttach(Arc<PdfAttachElem>)`.
3. `Content::PdfArtifact(Arc<PdfArtifactElem>)`.
4. structs públicos `PdfAttachElem` e `PdfArtifactElem` com os campos fechados
   no L0 `entities/content.md`.
5. enums públicos `AttachedFileRelationship` e `ArtifactKind` com os domínios
   fechados no mesmo L0.
6. `PagedDocument.attachments: Vec<Arc<PdfAttachElem>>`.
7. `SemanticKind::Artifact(ArtifactKind)`.
8. Pipeline interna: coletar attachments no layout, validar nomes duplicados
   após layout e antes do dispatch, embutir no builder; envelopar artifact no
   layout e serializar no stream sem structure node.

Qualquer campo, variante, trait, assinatura, default, fase, standard PDF,
configuração CLI ou carrier adicional está fora desta lista e exige nova
medição/classificação.

## 8. Estado do gate e limitação de atestação

Validação estrutural read-only no snapshot: `crystalline-lint --checks
v15,v26 --fail-on warning .` terminou com exit 0 e `No violations found`;
`git diff --check` terminou com exit 0 e stream vazio. Não foi executado
`--fix-hashes`: V5/drift é esperado até o gate e está fora da capacidade deste
papel.

O contrato está redigido, mas **não selado**: faltam confirmação humana
ADR-0127 para smartquote e os itens PDF, criação pós-confirmação dos owners 1:1
dos novos consumers, oráculos/ataques independentes, gate discriminatório e
selo.
`Unknown` não foi usado como sucesso. A cadeia deve parar aqui; não ressellar
hashes de código e não escrever implementação/testes.
