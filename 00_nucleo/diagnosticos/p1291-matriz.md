# P1291 — matriz medida e ownership

Medição independente: `2026-08-31T10:37:16-03:00`; integração da matriz:
`2026-08-31T10:47:38-03:00`; `HEAD 53d21c5a602f4045a769a0ab0c935baa5ecd3b88`;
árvore não commitada, lista exata em `p1291-baseline-status.txt` (SHA-256
`46007bc74479c66497b0c1ccd94f1585ca3d0f0997b9ba58207f80c6eb4a794a`). Vanilla
`/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`, revisão
ratificada `a51e02804`.

## Medição antes da decisão

| membro | chamada mínima / completa | morfologia e efeito | erros medidos | fonte vanilla | owner cristalino | decisão |
|---|---|---|---|---|---|---|
| `bb` | `bb([x])` | `styled(child: [x], ..)`; double-struck; no mesmo eixo o setter mais interno vence | body obrigatório; excesso/tipo/named rejeitados | `math/mod.rs:88`; `style.rs:46-55` | `stdlib/math_style.rs` + binding em `structural/math.rs` | contínuo |
| `frak` | `frak([x])` | `styled(child: [x], ..)`; Fraktur | idem `bb` | `math/mod.rs:86`; `style.rs:121-132` | idem | contínuo |
| `inline` | `inline([x])`; `inline([x], cramped: true)` | `styled(child: [x], ..)`; tamanho inline e cramped explícito | `cramped` exige boolean; segundo posicional e named desconhecido rejeitados | `math/mod.rs:90`; `style.rs:186-206` | idem | contínuo |
| `scripts` | `scripts([x])` | `scripts(body: [x])`; força attachments laterais | body obrigatório e único; tipo/named rejeitados | `math/mod.rs:68`; `attach.rs:68-78` | `MathLimitsOverrideElem`; adapter privado em `structural/math.rs` | contínuo |
| `serif` | `serif([x])` | `styled(child: [x], ..)`; força Plain, mas preserva setter de glyph mais interno | idem `bb` | `math/mod.rs:82`; `style.rs:147-163` | `stdlib/math_style.rs` + binding em `structural/math.rs` | contínuo |
| `cancel` | mínimo `cancel([x])`; completo com `length`, `inverted`, `cross`, `angle`, `stroke`, `background` | `CancelElem` transporta seis opções para layout | casts tipados por opção | `math/mod.rs:52`; `cancel.rs:17-109` | `MathCancelElem` atual só tem `body` | gate ADR-0127 |
| `underline` | `underline([x])` | `math::UnderlineElem`, identidade distinta da decoração textual | body obrigatório e único; tipo/named rejeitados | `math/mod.rs:55`; `underover.rs:4-14` | novo owner math necessário; `text/deco.rs` refutado | gate ADR-0127 |
| `vec` | `vec()`; `vec([a])`; múltiplos; completo com `delim`, `align`, `gap` | `VecElem`; coluna vetorial, alinhamento e gap observáveis | casts de delimiter/alignment/gap/children | `math/mod.rs:65`; `matrix.rs:18-68,327-354` | novo owner vetorial; `MathMatrix` não transporta align/identidade | gate ADR-0127 |

Todos os oito paths medidos têm `type == function` e `repr` com nome curto. A
igualdade `math.underline == underline` é `false` no vanilla: isto refuta o alias
textual mesmo quando o `repr` do conteúdo mínimo parece semelhante.

## Classificação

Os cinco membros contínuos usam contratos e variantes existentes; a mudança desta
fatia é a montagem explícita do namespace, mais um adapter privado de `scripts` que
delega ao constructor já dono da morfologia. Os três membros no gate exigem contrato
Rust público, identidade de elemento ou novo transporte para layout. Por ADR-0127,
não entram no lote seguro sem confirmação humana e não recebem crédito parcial.

## Resultado integrado

Os cinco membros contínuos fecharam RED→GREEN: `14/14` testes P1291 e `31/31`
testes P311b. A repetição bilateral confirmou nomes curtos e mensagens públicas;
também reproduziu, em ambos os binários, `serif(bb(A B C)) == bb(A B C)` e
`bb(serif(A B C)) == serif(A B C)` no SVG, provando inner-wins. A representação
textual do conteúdo produzido continua dependência explícita de P1290 e não foi
mascarada neste passo.

`cancel`, `underline` e `vec` permanecem abertos no gate ADR-0127. Portanto o
ganho certificado deste lote é de cinco bindings funcionais; o crédito final
`MATCH` fica sujeito ao resíduo de `repr` de P1290, e nunca é contado como `+8`.
