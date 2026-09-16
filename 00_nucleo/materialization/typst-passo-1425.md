# Passo 1425 — convergência vertical de `math.limits`

## Estado medido antes da decisão

O P1422 deixou 22 rotas `math.*` como `PARTIAL`: a forma qualificada está
ausente e a forma sintática alternativa diverge. Depois de P1423–P1424 fechar
a coorte HTML, uma leitura do registry vanilla refutou tratar as 22 rotas math
como lote homogéneo:

- `lab/typst-original/crates/typst-library/src/math/mod.rs:43-78` registra
  apenas `abs`, `norm` e `round` como funções; as outras 19 são constructors
  de elementos distintos;
- `01_core/src/compiler/stdlib/structural/math.rs:1138-1329` é dono do scope
  fechado do módulo math, mas só `accent` possui hoje uma nativa diretamente
  registrável entre as 22;
- `mid`, `stretch` e `primes` não possuem carrier cristalino que preserve sua
  identidade; outros elementos agregados também não conservam automaticamente
  a identidade pública do constructor vanilla;
- `MathLimitsOverrideElem` já preserva `body`, `limits` e `inline`, e o layout
  P992 já consome esses dados. É a única rota residual cuja semântica e
  geometria estão materializadas de ponta a ponta sem novo elemento.

Medição executada em 2026-09-16T15:38:36-03:00 no HEAD limpo
`ffc9ef4ed3d50806f857c8655836b76623adb85f`, tree
`d4033168bc83958a442a6c58abb33804ba55891b`. O L0 estrutural tinha SHA-256
`e58334e2be9db99a59ad35b251316e29d0064383ace335a1c9419eddd8b9f42e` e
consumer SHA-256
`14ba94bf6de39bf9a8d905499181d5c9e7ac7d3063f994816391449ed2de6578`.
O L0 de repr tinha SHA-256
`c55ad24ea7f620d82da7eb961380b85f20e00699cfbde04764cf5357d0300ee1`
e consumer SHA-256
`be1d3be57c9ac167254aed0b25fa216305dae7e71eee807c7c678d06e5fbc0fa`.

No vanilla ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`, a
fonte `math/attach.rs` SHA-256
`7efdda527260ad0ac91436b35abe5f98b85019ff8491192c1ed5077c17953f5f`
declara `limits(body, inline: true)`. Sondas no binário ratificado
`lab/typst-original/target/release/typst` mediram:

| Expressão | Resultado público |
|---|---|
| `repr(math.limits([x]))` | `limits(body: [x])` |
| `repr(math.limits([x], inline: false))` | `limits(body: [x], inline: false)` |
| `math.limits()` | `missing argument: body` |
| `math.limits([x], inline: 1)` | `expected boolean, found integer` |
| `math.limits([x], nope: true)` | `unexpected argument: nope` |
| `math.limits([x], [y])` | `unexpected argument` |
| `math.limits(body: [x])` | ``the argument `body` is positional`` |

O carrier atual não distingue omissão de `inline` de presença explícita com o
valor default `true`. Essa distinção é morfologia da linguagem porque a
`repr` conserva o named explicitamente escrito. A inferência é que um bit de
presença no carrier agregado basta; seria refutada por fonte ou sonda do mesmo
pin que exigisse outro estado, outro default ou geometria diferente.

## Decisão proposta

Materializar `math.limits` como primeira fatia vertical das 22 rotas math:

1. registrar `math.limits` como função pública do módulo fechado;
2. aceitar exatamente um `Content` posicional obrigatório e somente
   `inline: bool`, default `true`;
3. acrescentar `inline_explicit: bool` a `MathLimitsOverrideElem` e à função
   pública `Content::math_limits_override`, preservando omissão versus
   presença explícita inclusive quando o valor é `true`;
4. fazer tanto a sintaxe bare existente quanto a chamada qualificada preencher
   o mesmo carrier; `scripts` mantém `limits:false` e presença falsa;
5. preservar `inline_explicit` em walkers e na recursão de
   `apply_math_default`;
6. representar `limits(body: ...)` quando omitido e acrescentar
   `inline: <bool>` somente quando explicitamente fornecido.

A aceitação é semântica, sintática e morfológica. Igualdade de estrutura Rust,
bytes de render e escolha de helper são mecânica e não dão crédito. Mensagens
de erro são observáveis e devem seguir o fragmento medido.

## Limites explícitos

P1425 não materializa as outras 21 rotas P1422, não cria carrier para
`mid`/`stretch`/`primes`, não corrige outros constructors agregados, não muda
geometria de attachments e não declara paridade math geral. Também não
altera `scripts`, salvo transportar o novo bit como falso.

Casos sem identidade verificável, falha de build/execução, timeout, OOM, swap,
stream truncado ou ilegível são `Unknown` e bloqueiam. `Unknown` nunca recebe
crédito.

## Gate ADR-0127

O passo adiciona uma função pública de linguagem, acrescenta campo público à
entidade e altera a assinatura pública do constructor L1. Incide diretamente
nos itens 1 e 4 do ADR-0127.

Fluxo obrigatório:

1. atualizar antes do código os L0 proprietários de stdlib, entidade,
   `Content`, eval math, layout comum e repr;
2. commitar esta proposta e os L0s;
3. **PARAR sem alterar L1 nem headers de linhagem**;
4. continuar somente após confirmação explícita do dono;
5. depois da confirmação, produzir contrato, oráculos, ataques e testes
   segregados, selar, obter RED e só então implementar.

## Regime posterior ao gate

Usar protocolo completo `tekt-materializacao-segregada`, declarando
“executado sem atestação de isolamento” no workspace partilhado. O contrato
deve proteger binding, aridade, casts, named, presença explícita, carrier,
walkers, repr, sintaxe bare e layout já vigente. Mutation score exigido: 1.0.

## Gates de fecho após confirmação

- RED do binding qualificado e do bit de presença;
- RED morfológico para omitido, `inline:true` e `inline:false`;
- RED dos quatro grupos de diagnóstico medidos;
- GREEN bilateral da chamada qualificada e da sintaxe bare;
- controle `scripts` preservado;
- três ordens determinísticas e hashes selados intactos;
- `cargo fmt --all -- --check`, testes focais, `cargo build`,
  `crystalline-lint .` e `git diff --check`.

