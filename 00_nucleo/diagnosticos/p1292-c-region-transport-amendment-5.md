# P1292 — amendment-5: transporte C pela região efetiva

**Papel:** autor segregado do contrato + auditor de ownership

**Manifest autorizado:**
`23e12e76e401399e9d65977166d4f87006df7da05b83f8fd8f0abe6d48ab508f`

**HEAD:** `0eb39f8ecb48930515f2cadb6a378450855b5a72`

**Proveniência:** working tree não commitada; medição em
`2026-09-01T01:57:27-03:00`; `git diff HEAD --stat` registrou 35 arquivos,
1.556 inserções e 330 remoções. Binário vanilla ratificado
`/usr/local/bin/typst` SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
binário candidato `target/debug/typst` SHA-256
`f5f10503bf30b1a612d1013063e1baebb4b91ce6a16426cca3e392ef3406a9aa`.

## Refutação do transporte anterior

O lote C já tem surface e parsing de `Rel<Length>` verdes. O transporte v5,
porém, chamava `measure(..., width: ..., height: ...)`; no candidato isso
termina antes de `MathVec`, porque os named `width`/`height` pertencem ao
owner preexistente de `measure` (ADR-0054), fora de P1292. Adaptar essa API,
introduzir named novos ou criar outro owner para servir ao teste confundiria
uma limitação do transporte com a obrigação C.

A refutação atinge somente o mecanismo de observação. Não altera os resultados
públicos A-D, a fórmula de `gap`, a morfologia, os erros nem os 22 ownerships.

## Prova black-box bilateral

Cada fixture compila um documento real para SVG pela CLI. A própria página
fornece a região ao layout da equação:

```typst
#set page(width: auto, height: 100pt, margin: 0pt)
#math.equation(math.vec([A], [B], gap: 10%))
```

O segundo conjunto troca somente `height: 100pt` por `height: auto`. Os quatro
gaps executados em cada região são `0%`, `10%`, `1em` e `10% + 1em`. O
observável é a distância vertical entre as baselines dos dois filhos visíveis
e únicos `A`/`B`; IDs de glifos, bytes e ordem interna de itens não participam
da comparação.

| Região / gap | Vanilla delta A→B | Candidato delta A→B | Controle da raiz |
|---|---:|---:|---:|
| finite 100pt / `0%` | 7.6692pt | 7.6692pt | 100pt |
| finite 100pt / `10%` | 17.6692pt | 17.6692pt | 100pt |
| finite 100pt / `1em` | 18.6692pt | 18.6692pt | 100pt |
| finite 100pt / `10% + 1em` | 28.6692pt | 28.6692pt | 100pt |
| auto / `0%` | 7.6692pt | 7.6692pt | 7.7pt |
| auto / `10%` | 7.6692pt | 7.6692pt | 7.7pt |
| auto / `1em` | 18.6692pt | 18.6692pt | 22.88pt |
| auto / `10% + 1em` | 18.6692pt | 18.6692pt | 22.88pt |

Controles executados: cada SVG finite declarou root height exatamente 100pt;
cada SVG auto declarou altura finita; o extrator encontrou exatamente duas
baselines-filho em todos os 16 SVGs; `0%` estabeleceu o delta-base e `1em`
estabeleceu que a parcela absoluta sobrevive nos dois regimes. Em finite,
`10% - 0% = 10pt` e `mixed - 1em = 10pt`; em auto, ambas as diferenças são
zero. Assim o transporte alcança `MathVec`, discrimina a parcela relativa e
não precisa de `metadata`, `query` ou `measure` com named.

Hashes das oito fixtures, na ordem auto `0`, `10%`, `1em`, mixed e finite
`0`, `10%`, `1em`, mixed:

```text
ef32161b4e1a79be925185047714f0562c5b3716dbb79bff1609a0038e9d6d16
311f03da7cfa8ac18608fdbe5202a1dd2fe80ed2d8d972fced750e9e7772cab8
0de5148412ef254af2ac34cc2372cfda1b31aa2907bd3cb232c1884cdf365903
b18f315cf385b849af05a25c4273e9b31fe0d541ee0ac63784e00e9b2bbf1bf3
c93dad97cdabcbb5157bddbc3008abae50f252083b4d8a8aac37752bfb585a50
89255a1ef0ef4a6bc814cad95ea0497e97f4bb6c4328e4b8c4a8508e8d334fcc
da37916d5fe1630b5253f70c0a8cdc06b3d77be00c4c66a50014795523c97199
5cb0290dddfe8a3641aee9067812bdee4aeb7964c147e90f57a6025a7478bd62
```

Comandos essenciais:

```text
target/debug/typst compile --format svg <fixture> <candidate.svg>
/usr/local/bin/typst compile --format svg <fixture> <vanilla.svg>
python3 <extrator read-only de viewBox e transforms públicos do SVG>
```

## Decisão e limite de inferência

No nível da linguagem, uma região finita resolve a parcela percentual contra
sua altura efetiva; `height:auto` a colapsa a zero sem apagar a parcela
absoluta. A igualdade bilateral dos deltas confirma o transporte C atual.

Inferência: as baselines dos filhos são um observável adequado da separação
vertical porque o único valor variado entre cada par é `gap` e os dois
marcadores permanecem únicos. Refutadores: marcador ausente/duplicado,
alteração do conteúdo, root finite diferente de 100pt, delta de `10%` não
igual ao base em auto, ou parcela `1em` perdida. Qualquer refutador invalida o
sub-selo; não autoriza mudanças em `measure`, página, equation ou outro owner.

Os L0s vigentes já fixam integralmente esta regra:

- `compiler/math/layout/vec.md` SHA-256
  `772331ee68ec1069b57e80dd6674ef60d62fcd8db83d7d3fae15a18b7b82b764`;
- `entities/elements/math_vec.md` SHA-256
  `72f2ba668f201d0fb0edf7d2d6bd8f5f85aab156b883a8e644b427f7cd2d1afa`.

Nenhum L0 foi alterado. O amendment muda somente o transporte comparativo C
do contrato, preserva o set de 22 owners e entrega ao autor do oráculo uma
substituição causal do transporte inválido. **PARAGEM.**
