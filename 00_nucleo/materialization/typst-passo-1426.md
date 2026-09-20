# Passo 1426 — convergência vertical de `math.root`

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Estado medido antes da decisão

P1425 fechou `math.limits` e deixou 20 rotas `math.*` residuais da coorte
P1422. A leitura do carrier cristalino mostrou que `math.root` é a próxima
fatia vertical com semântica e layout já existentes, mas sem binding público
qualificado e sem memória causal suficiente para a sua `repr`.

Medição executada em 2026-09-16T16:39:26-03:00 no HEAD limpo
`9d2ef266bdccb8e4c113d332f60eb7e0029d2098`, tree
`ae0751296e09ec4bd0f02dcbe88458b256b3d22c`. Os oito pares L0/consumer
produtivos congelados são:

| L0 | identificador administrativo L0 | consumer | identificador administrativo consumer |
|---|---|---|---|
| `compiler/eval.md` | [dado administrativo removido] | `compiler/eval/mod.rs` | [dado administrativo removido] |
| `compiler/math/layout/tests.md` | [dado administrativo removido] | `compiler/math/layout/tests.rs` | [dado administrativo removido] |
| `compiler/math/layout/_comum.md` | [dado administrativo removido] | `compiler/math/layout/mod.rs` | [dado administrativo removido] |
| `entities/content.md` | [dado administrativo removido] | `entities/content.rs` | [dado administrativo removido] |
| `compiler/stdlib/structural/math.md` | [dado administrativo removido] | `compiler/stdlib/structural/math.rs` | [dado administrativo removido] |
| `compiler/eval/math.md` | [dado administrativo removido] | `compiler/eval/math.rs` | [dado administrativo removido] |
| `compiler/eval/repr.md` | [dado administrativo removido] | `compiler/eval/repr.rs` | [dado administrativo removido] |
| `entities/elements/math_root.md` | [dado administrativo removido] | `entities/elements/math_root.rs` | [dado administrativo removido] |

No vanilla ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
`math/root.rs` (identificador administrativo
[dado administrativo removido])
declara `index: Option<Content>` posicional antes de `radicand: Content`; o
registry `math/mod.rs` (identificador administrativo
[dado administrativo removido])
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

**Confirmado explicitamente pelo dono em 2026-09-16.** A materialização fica
autorizada somente dentro do recorte `math.root` e dos oito L0 proprietários
acima; as outras 20 rotas math permanecem fora de escopo.
