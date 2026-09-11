# P1339 — implementação de float/version, entrega de integração r1

Executor `/root/p1339_remaining_l0`, agora implementador subordinado ao root.
Regime: executado sem atestação de isolamento. O contexto anterior continha
desenho L0 público; nenhum candidato ou teste independente privado existia
durante aquela autoria. Não sou autor do contrato, oráculos ou veredito.

## Autoridade e entradas

Hashes verificados antes de código:

- `p1339-implementation-authorities.json`:
  `cab15a1dce423a12fe198a640b541ee00b3f8743ec0f6a9e97519416d940f3c0`;
- `p1339-seal.json`:
  `35f00c4b9e15a010692017f5083ea4f451f4104a3730022136972e151ac0a8ee`;
- `p1339-contract-r3.json`:
  `c0cd1826679ddaf77e937a677839c5cbe64fdae6ec71bfa34e635d584d9c3e17`;
- `p1339-red.json`:
  `e4e564e606283c15ef792d59de2d36f1a402d6d5d5077f7d6581b23219076f7d`;
- `p1339-verifier-red-acceptance-r1.json`:
  `91f3953682384cfe4f195e3462611ca4c1ab328c5ac859eefc0c0398731852c7`.

L0 float, version, primitives-constructors e _comum conferiram com os raw
hashes do selo antes e após as alterações. Oráculos públicos batch2 e suas
referências foram lidos como entradas imutáveis. Não foram alterados L0,
contrato, oráculos, baseline, ataques ou saídas do verificador.

## RED local anterior à implementação

Testes acrescentados aos módulos produtivos sob cfg(test), sem semântica
nova: dois testes float e um version, com operações públicas, valores, bytes,
diagnósticos/hints/spans. Os recibos locais mantêm HEAD, status e diff/stat
integrais, hashes dos quatro sources permitidos, comandos/canais e custo.
Esses testes locais não são apresentados como A/B independente.

- `p1339-implementation-numerics-local-red-float.json`, SHA-256
  `729aff14f5c3573b57497fc1b57d89477d0fc473801a9826f18ecd4ec5f66267`:
  cargo test release p1339_float_, dois testes executados e duas falhas por
  ausência de inf/signum; exit 101. Repetição feita para capturar recibo integral
  após execução inicial direta, também RED, cuja compilação durou 2m49s.
- `p1339-implementation-numerics-local-red-version.json`, SHA-256
  `743d001f5a142ece85198d1ca81f2fc092343d877fff7c64687dec648ef6848e`:
  cargo test release p1339_version_, um teste executado e falha pela ausência
  de version.at; exit 101.
- Por pedido do root, executei seu teste já compilado de locatable, sem
  escrever esse teste: `p1339-implementation-numerics-local-red-locatable.json`,
  SHA-256 `b15bbc8055bdf0e7d39bc7ad6ffe39b10b985f2f49f043b154b135c53295f7ad`;
  um teste executado e falha na afirmação is_locatable, exit 101.

Target compartilhado `/tmp/p1339-target.UD8gh7`, compilação coordenada com
root. As adições simultâneas de testes estão nos recibos e não são tratadas
como árvore imutável para verificação final. Código semântico começou somente
após esses REDs e a liberação coletiva do root.

O primeiro recorder em Node falhou com `spawnSync git EPERM` antes de executar
cargo e não produziu recibo; foi conservado como artefato, sem crédito de
teste. O recorder Python sucessor usa subprocess e preserva canais integrais.

## Implementação entregue

`float.rs` descobre inf/nan/signum/from-bytes/to-bytes. Novas nativas possuem
parser por ocorrências, casts estritos, required positional com hint,
endian e size com validação causal, sobras fechadas e âncoras de origem.
Binary32/64 e sinal usam operações puras; nenhuma fórmula foi movida ao
field access. Predicados anteriores permanecem com suas implementações.

`version.rs` acrescenta adapter at que valida os dois posicionais e delega
a Version::at. A forma ligada reconstrói Args com receiver; o constructor
e PARITY_VERSION não foram alterados.

Assinaturas combinadas com root:

```text
dispatch_float_method_spanned(
  receiver:f64, receiver_span:Span, method:&str, args:Args,
  ctx:&mut EvalContext, world:&dyn World, current_file:FileId
) -> SourceResult<Value>

version_type_field(field:&str) -> Option<Value>

dispatch_version_method(
  receiver:&Version, method:&str, args:Args,
  ctx:&mut EvalContext, world:&dyn World, current_file:FileId
) -> SourceResult<Value>
```

Todos internos pub(crate). `dispatch_float_method` legado conserva assinatura
e delega ao novo helper com receiver detached. O caller novo fornece span
lexical do receiver e Args.span da chamada inteira. Isso permite cast de
from-bytes ligado falhar sobre o próprio receiver Float. A versão tem receiver
tipado, sem cast ligado que necessite inventar origem.

Hubs têm somente exports: version helpers via primitives_constructors;
float spanned diretamente de foundations::float pelo stdlib/mod.rs, sem tocar
foundations/mod.rs fora da allowlist. Root possui os callers restantes.

## Estado de entrega e limites

Marco UTC `2026-09-10T05:53:37Z`, HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitada:

| Source | SHA-256 nesta entrega |
|---|---|
| `01_core/src/compiler/stdlib/foundations/float.rs` | `14f21c8c42a928d8c6d445034e111e604a1dea1fd55272ae81e9481a0b98c15d` |
| `01_core/src/compiler/stdlib/primitives_constructors/version.rs` | `393fb73129907ce65865cedefba439f2d1badd5cbc003e3fc5d0944f293afdb6` |
| `01_core/src/compiler/stdlib/primitives_constructors.rs` | `e88ce8ac3d22bd94d119863480e692d04f4423ecc0776ee0c9db2baccda55c7b` |
| `01_core/src/compiler/stdlib/mod.rs` | `9d81b8102609bbad6363ebd1d449bb3a750a25da15aafaa1198e2d839fa05f05` |

Rustfmt dos quatro arquivos (skip_children) e git diff --check local passaram.
Um erro sintático de parentização no cast f32→f64 foi detectado pelo primeiro
rustfmt e corrigido antes de compilação. Ainda não há GREEN desta implementação
nesta entrega r1: faltam callers root e fechamento dos matches Selector antes
de build/test integrados. Não há claim funcional final, score de mutação,
resselo, commit ou veredito. Próximo trabalho: executar testes locais e
sondas públicas congeladas sobre o binário integrado, preservando o selo.
