# P1286 — receipt pós-gate dos quatro owners L0

**Natureza:** recibo de autoria L0 pós-confirmação. Não é código, teste,
oráculo, ataque, resselo, selo nem veredito de materialização.

## 1. Predecessor causal e autoridade

- Confirmação humana recebida em 2026-08-30: **“Siga a engenharia da
  refatoração e pode implementar”**.
- Contrato autorizado: `00_nucleo/diagnosticos/p1286-contract-receipt.md` v2,
  SHA-256
  `16597543965ca13d37fdabf9be184f3477df92da4f6702ba14f0a4ca4918f9bb`.
- Snapshot: `HEAD 53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, working
  tree compartilhada e não commitada.
- Papel: autor independente dos Prompts L0 `/root/contrato_l0_p1286`.
  Escritas limitadas aos quatro prompts deste manifesto e a este receipt.
  Código, testes, oráculos, ataques, fix-hashes e veredito permaneceram
  negados.
- Regime completo da skill `tekt-materializacao-segregada`, executado sem
  atestação de isolamento físico do host.

Entradas normativas adicionais:

| entrada | SHA-256 |
|---|---|
| ADR-0109 | `0cbd3049418073e9b8efd0be1a19030c9ecde905a25ebf3d20922e3cef02676e` |
| ADR-0129 | `64756b81ce58ca62e1a166b3776303759bc7af507a1c97a4e3ad91a8dc5b906e` |
| skill | `33a32f7bc439de3fe3aa530bd65518e512a93f40152c91b2ace0789de34a3a56` |
| referência de papéis | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` |
| referência de artefatos/gates | `bf218259b4454974bf8889ce319e04c0c7ec668b9a542d0eb3b4d0492a623963` |

## 2. Manifesto 1:1 criado

| Prompt L0 proprietário | consumer produtivo exclusivo | SHA-256 do prompt |
|---|---|---|
| `entities/elements/pdf_attach.md` | `01_core/src/entities/elements/pdf_attach.rs` | `151e5aa215abdeac306f77edd50ff1eb8ba0a49495dc4aab90461f09d02f845a` |
| `entities/elements/pdf_artifact.md` | `01_core/src/entities/elements/pdf_artifact.rs` | `d1c643277f0bb2060c8aec560bffd7bb67d00b2edd033af65d9b9cc560a25a79` |
| `compiler/layout/pdf_attach.md` | `01_core/src/compiler/layout/pdf_attach.rs` | `d57ee15295168649b3f6d0f9a3a7bb0862f776013c52122cae69c819d36eb841` |
| `compiler/layout/pdf_artifact.md` | `01_core/src/compiler/layout/pdf_artifact.rs` | `0de4583b7abfc421bfbecb7e330e9fbe20386bd2f62a2ecdbafe28221e45583b` |

Cada prompt contém `Hash do Código: 00000000` como sentinela explícita e
não selada. O valor não afirma identidade com código inexistente e deve ser
substituído somente pelo fluxo normal depois que o consumer proprietário
existir. Nenhum prompt pode ser reutilizado por outro source.

## 3. Forma B e Núcleo

Os dois layouts consomem o Núcleo existente
`00_nucleo/prompts/_nuclei/layout/element-form-b.toml` com pin efetivo
`6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5`,
o mesmo aceito pelos owners vigentes. O SHA-256 dos bytes TOML observados é
`7d6d0ee3c89677434debb41f1deb8975e8c5addfb2dfad2845eb93c0b8240bc7`;
não confundir hash bruto do ficheiro com o pin efetivo validado por V26.

Os contratos preservam match exaustivo, despacho estático, braço magro e free
function descendente na camada de render. Não há import reverso, vtable,
`dyn`, PropMap ou `pub(crate)`. Nenhum Núcleo novo foi criado.

## 4. Escopo congelado e `Unknown`

- `PdfAttachElem` é marker invisível, não podável e sem I/O; layout registra o
  `Arc` em ordem no side-channel, sem frame/cursor. Duplicado é decisão global
  posterior da pipeline.
- `PdfArtifactElem` preserva kind/body; layout conserva visual/texto e cria
  envelope `SemanticKind::Artifact`, nunca Formula.
- Standards PDF não medidos, compressão, casos opacos de PathOrStr, nested
  artifacts, multi-page/running matter e efeito real em AT permanecem
  `Unknown`. `Unknown` nunca é sucesso.

## 5. Estado temporário de ownership

Pela cardinalidade normativa ADR-0129, os quatro prompts estão temporariamente
sem consumer físico e portanto o estado conceitual esperado é **V15 órfão até
a materialização dos quatro sources**. Isso é a janela pós-gate autorizada,
não um estado apto a resselo.

O comando read-only
`crystalline-lint --checks v15,v26 --fail-on warning .` terminou, neste estágio
de sentinela e ficheiros ainda ausentes, com exit 0 e `No violations found`.
A ausência de diagnóstico mecânico não converte os owners sem consumer em
relação 1:1 satisfeita: o preflight V15/V26 deve ser repetido após criar os
sources e antes de qualquer `--fix-hashes`. Não foi executado fix-hashes.

## 6. Estado da cadeia

A autorização humana foi materializada apenas como quatro raízes L0. A próxima
autoridade pode implementar exclusivamente esses consumers e os owners
existentes já enumerados no contrato v2. Este papel para aqui; não aprova a
própria implementação e não emite equivalência ou veredito.
