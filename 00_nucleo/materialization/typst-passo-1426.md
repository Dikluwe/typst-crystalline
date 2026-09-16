# Passo 1426 — convergência vertical de `math.root`

## Estado medido antes da decisão

P1425 fechou `math.limits` e deixou 20 rotas `math.*` residuais da coorte
P1422. A leitura do carrier cristalino mostrou que `math.root` é a próxima
fatia vertical com semântica e layout já existentes, mas sem binding público
qualificado e sem memória causal suficiente para a sua `repr`.

Medição executada em 2026-09-16T16:39:26-03:00 no HEAD limpo
`9d2ef266bdccb8e4c113d332f60eb7e0029d2098`, tree
`ae0751296e09ec4bd0f02dcbe88458b256b3d22c`. Os oito pares L0/consumer
produtivos congelados são:

| L0 | SHA-256 L0 | consumer | SHA-256 consumer |
|---|---|---|---|
| `compiler/eval.md` | `e1a433c352763e38008bf6415ee0088249d21a760780fb551be59d333b21c8e0` | `compiler/eval/mod.rs` | `f9b7d80a512df3d02b0fe0a35e7d6e46059072108024862cf06026b9d5bb13f2` |
| `compiler/math/layout/tests.md` | `75bdf64454523a50610c9e3b1dedd8500d89f1559f79848b4e647a09c84ad6ec` | `compiler/math/layout/tests.rs` | `421cb9ecd4871512c677257820e86d1b4fe90f21959a1fbced5489e47419596d` |
| `compiler/math/layout/_comum.md` | `5e53876e77c1d2edfa1c2902c6d312e229e1300a65718f011b9e3d70277f2988` | `compiler/math/layout/mod.rs` | `b2f922452608a2553b928fad10aca309edaccabc6da32a1a34ef9c27d99a8cbb` |
| `entities/content.md` | `5f81a38d6e79600849ba58e946f1b049259fffb19e8de957fa981f59c90f5e5a` | `entities/content.rs` | `70aa5c82a3029b16bafae9328cff695a7a7d3c23fba16fe593e8f0079915ee2e` |
| `compiler/stdlib/structural/math.md` | `e65e061178d78faecb8b3043320df45130d2385e9f3e511a1de0d96f4bd4f658` | `compiler/stdlib/structural/math.rs` | `d4443ffb7b4ac42e1883bb4bbc5647818899abd4dd46b3021e54b02ac05402bc` |
| `compiler/eval/math.md` | `a7a200ae2af2656ced85b058fefdcf8a04b8b5e5925f3423548005cb3a649637` | `compiler/eval/math.rs` | `a4cf19b130627d3ac3c1aa21ebd7da7f36921e961ac493a6acff1913ea5c663d` |
| `compiler/eval/repr.md` | `d5861675adaff2456cf4bf2141cf2079f8969a1800d099f82026eb89911d939f` | `compiler/eval/repr.rs` | `5c5de5353d6f5eae065e04b4a5200621f57ba8c75f672e3afeb0acc9cb7630ff` |
| `entities/elements/math_root.md` | `dfecb091446dbee2d4320d9c78723109998857e699735834d0f77c90289fabc8` | `entities/elements/math_root.rs` | `dd544158b38ea0011fe877951262d4e445c68d1881fd4ecd67d218628bf3a5f9` |

No vanilla ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
`math/root.rs` (SHA-256
`c62f8135a6b8ef26a1bd0a3a28b7cbd12750dd1d0f0ed48370e839ac34638763`)
declara `index: Option<Content>` posicional antes de `radicand: Content`; o
registry `math/mod.rs` (SHA-256
`a8a64e505979b8a6fce3bb5fef978677a40200199e9dc36738a3343ae1104957`)
registra `RootElem`. Sondas no binário ratificado mediram:

| Expressão | Resultado público |
|---|---|
| `repr(math.root())` | erro `missing argument: radicand` |
| `repr(math.root([x]))` | erro `missing argument: radicand` |
| `repr(math.root([3], [x]))` | `root(index: [3], radicand: [x])` |
| `repr(math.root(none, [x]))` | `root(index: none, radicand: [x])` |
| `math.root([3], [x], [y])` | erro `unexpected argument` |
| `math.root(3, [x])` | erro `expected content, found integer` |
| `math.root([3], 1)` | erro `expected content, found integer` |
| `math.root([3], none)` | `root(index: [3], radicand: [])` |
| `math.root([3], radicand: [x])` | erro ``the argument `radicand` is positional`` |
| `math.root([3], [x], foo: 1)` | erro `unexpected argument: foo` |

As sondas de sintaxe bare mediram que `root(x)` falha por radicando ausente,
`root(3, x)` produz o mesmo elemento indexado e `root(, x)` conserva índice
como conteúdo vazio. Portanto ausência sintática (`sqrt`) é distinta de
índice explicitamente `none` e de índice vazio. A inferência é que um bit de
presença no carrier existente é suficiente; seria refutada por uma sonda do
mesmo pin que exigisse outro estado observável.

## Decisão proposta

1. acrescentar `index_explicit: bool` a `MathRootElem` e à assinatura pública
   `Content::math_root(index, radicand, index_explicit)`;
2. registrar `math.root` como função pública que aceita exatamente dois
   posicionais: índice `Content | none` e radicando `Content`;
3. fazer chamada qualificada e sintaxe bare convergirem no mesmo carrier:
   `math.root(none, [x])` usa `None/true`, `root(, x)` usa
   `Some(Content::Empty)/true`, e `sqrt` usa `None/false`;
4. preservar o bit nos walkers, na igualdade observacional e nas reconstruções
   de estilo/default; o algoritmo de layout ignora-o;
5. representar `None/false` como `root(radicand: ...)`, `Some/true` como
   `root(index: ..., radicand: ...)` e `None/true` como
   `root(index: none, radicand: ...)`.

A aceitação é semântica, sintática e morfológica. Estrutura Rust, bytes de
render e escolha de helper são mecânica. As mensagens medidas são observáveis.

## Limites explícitos

P1426 não altera a geometria de radicais, a semântica de `math.sqrt`, parser,
AST ou as outras 20 rotas residuais. Os 14 callsites da suíte de layout só
recebem o terceiro argumento mecanicamente coerente com a fixture existente.
Casos sem identidade verificável, falha de build/execução, timeout, OOM, swap,
stream truncado ou ilegível são `Unknown` e bloqueiam; nunca recebem crédito.

## Gate ADR-0127

O passo acrescenta campo público, altera assinatura pública do constructor L1
e cria binding público de linguagem. Incide nos itens 1 e 4 do ADR-0127.

Fluxo obrigatório:

1. atualizar antes do código os oito L0 proprietários congelados;
2. commitar esta proposta e os L0s;
3. **PARAR sem alterar L1 nem headers de linhagem**;
4. continuar somente após confirmação explícita do dono;
5. depois da confirmação, produzir contrato, oráculos, ataques e testes
   segregados, selar, obter RED e só então implementar.

## Regime posterior ao gate

Usar protocolo completo `tekt-materializacao-segregada`, declarando
“executado sem atestação de isolamento” no workspace partilhado. O contrato
deve cobrir binding, aridade, casts, named, três estados do índice, carrier,
walkers, repr, sintaxe bare e não-regressão geométrica. Mutation score exigido:
1.0; `Unknown` é fail-closed.

## Gates de fecho após confirmação

- RED do binding e dos três estados causais do índice;
- RED morfológico de raiz quadrada, índice presente e `none` explícito;
- RED dos diagnósticos medidos;
- GREEN bilateral da chamada qualificada e da sintaxe bare;
- controle `math.sqrt` e layout preservados;
- três ordens determinísticas e hashes selados intactos;
- `cargo fmt --all -- --check`, testes focais, `cargo build`,
  `crystalline-lint .` e `git diff --check`.

## Confirmação do gate

**PENDENTE.** Nenhuma implementação L1 está autorizada por esta proposta até
confirmação explícita do dono.
