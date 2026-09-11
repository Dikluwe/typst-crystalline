# Prompt L0 — fachada `compiler/stdlib/mod.rs`
Hash do Código: c239668b

**Camada**: L1
**Origem**: fatiado de `rules/stdlib.md` em **P314** (ADR-0104, atomicidade
para agentes). O único consumer produtivo deste L0 é
`01_core/src/compiler/stdlib/mod.rs` (fachada e helpers locais). As notas
históricas de convenção não legitimam outros consumers: cada módulo tem
seu owner exclusivo; compartilhamento normativo requer Núcleo (ADR-0129).

> Nota: `stdlib/structural.rs` tem agora o seu próprio prompt em
> `00_nucleo/prompts/compiler/stdlib/structural.md` (P430).
> Nota: `stdlib/layout.rs` tem agora o seu próprio prompt em
> `00_nucleo/prompts/compiler/stdlib/layout.md` (P432).
> Nota: `stdlib/calc.rs` tem o seu prompt dedicado em
> `00_nucleo/prompts/compiler/stdlib/calc.md` (actualizado no P433 para o subset
> trig/hiperbólicas/log/exp/constantes).
> Nota: `stdlib/assert.rs` tem agora o seu próprio prompt em
> `00_nucleo/prompts/compiler/stdlib/assert.md` (P434).
> Nota: `stdlib/shapes.rs` tem agora o seu próprio prompt em
> `00_nucleo/prompts/compiler/stdlib/shapes.md` (P435).
> Nota: `stdlib/transforms.rs` tem agora o seu próprio prompt em
> `00_nucleo/prompts/compiler/stdlib/transforms.md` (P436).
> Nota: `stdlib/gradients.rs` tem agora o seu próprio prompt em
> `00_nucleo/prompts/compiler/stdlib/gradients.md` (P437).
> Nota: `stdlib/foundations.rs` tem agora o seu próprio prompt expandido em
> `00_nucleo/prompts/compiler/stdlib/foundations.md` (P438). Com este fecho,
> **DEBT-57 está encerrado** — todos os ficheiros stdlib de L1 têm spec L0
> dedicada.

---

## Contexto e Objetivo

Enquanto `eval.rs` é o motor que **caminha pela AST**, este módulo contém as
**ferramentas nativas** que Typst expõe no seu escopo global — funções
implementadas directamente em Rust e registadas como `Value::Func` durante a
inicialização do compilador.

**Separação de responsabilidades crítica:**
- `eval.rs`: sabe como avaliar `Expr::LetBinding`, loops, condicionais → produz `Value`
- `stdlib`: sabe o que `abs(-5)` retorna → implementa as funções que `eval` *chama*

## Convenção de assinatura (Passo 71 — DEBT-24)

```rust
fn native_X(ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value>
```
Funções sem I/O usam `_ctx` (prefixo underscore suprime o warning).
`native_image` usa `ctx.world.read_bytes(path)` para aceder ao ficheiro.
Aceita positional (`args.items`) e named args (`args.named`).
Funções que não aceitam named args chamam `expect_no_named(&args.named)?` no início.

## Helpers Internos

| Função | Uso |
|--------|-----|
| `coerce_to_f64(v, ctx)` | `Int`→`f64`, `Float`→`f64`, outros → Err com contexto |
| `guard_float(f)` | NaN → Err "não é um número", Inf → Err "infinito" |
| `format_float(f)` | compacto sem trailing zeros; garante ponto decimal (`"3.0"`) |
| `format_length(l)` | `Length` → `"12pt"`, `"1.5em"`, `"6pt + 1em"` |

## Sistema de Tipos — Regras de Promoção

```
Int + Int   → Int       (sem promoção)
Int + Float → Float     (coerce_to_f64)
Float pow Float → guarda NaN/Inf via guard_float
Int/Int divisão → Float (semântica eval.rs, não stdlib)
```

## P1215 — reexport de metadados de chamada

O hub reexporta `CollectionCallSpans` somente em `pub(crate)` para ligar
`eval/call_dispatch` ao owner `stdlib/collections`. O tipo não entra na API da
linguagem nem em contratos externos e não contém valores avaliados.

## P1289 — reexport interno do owner `float`

O registo raiz reexporta em `pub(crate)` somente
`float_type_field`, `is_float_instance_method` e `dispatch_float_method` do
owner `foundations/float`. O hub não contém lookup, fórmula ou validação e não
amplia a API Rust externa.

## P1292 — reexport interno mínimo de `native_flush`

### Medição anterior à decisão

No baseline SHA-256
`02d53b1588b008c295e2dcab0adc0e6ad2e8604d9972c5fa0b2503d340826f3f`,
`01_core/src/compiler/stdlib/mod.rs` declara `layout` como submódulo privado e
constitui a fachada já consumida por `compiler/eval/mod.rs`. A lista de
reexports de layout expõe as demais nativas necessárias ao registo, mas ainda
omite `native_flush`. A implementação/construção de `native_flush` já pertence
ao owner dedicado `compiler/stdlib/layout.md`, SHA-256
`6ff688ec12444582ec9ec87bb7b432019ff568882fbb1c66170eb47367e62dc4`;
movê-la para o hub violaria a atomização e o ownership 1:1.

### Decisão

O registo raiz reexporta somente dentro da crate:

```rust
pub(crate) use crate::compiler::stdlib::layout::native_flush;
```

O reexport é uma ligação de fachada, não um segundo owner. `native_flush`
continua definida e legitimada exclusivamente por `stdlib/layout.md`; este
prompt legitima apenas a linha de reexport em `stdlib/mod.rs`. O hub não cria
wrapper, lookup, cast, validação ou branch, não torna o módulo `layout`
público, não usa wildcard e não promove a função à API Rust externa.

A superfície Typst `place.flush`, seus argumentos/erros e a construção de
`Content::Flush` permanecem nos owners P1292 já selados. Esta correção só
permite que o consumer da fachada alcance a função dedicada; não altera
vetores A-D, default, fase de pipeline ou compatibilidade. Qualquer necessidade
de lógica no hub, `pub use` externo, mudança em `layout.rs` ou segundo caminho
de implementação refuta este amendment e exige novo owner/escopo.

## P1307-R3 — ligação dos encoders, pendente do gate público

### Medição anterior à decisão

No snapshot `00_nucleo/diagnosticos/p1307-r3-baseline.json`, SHA-256
`b50e726c5830c0f91a6875d2d0a758903bc93b0bd719f4b5357828522a999293`,
HEAD `b303f1f15b610e09872b567027e0d806387fde8c` mais o diff P1306 integral
ali guardado, `01_core/src/compiler/stdlib/mod.rs:134-137` reexporta as
nativas de loading; `compiler/eval/mod.rs:1723-1725` registra os três
decoders sem namespace. O único encoder existente é CBOR. Isso situa a
ligação no hub, mas a implementação no owner loading.

### Decisão e limite

Este consumer deve reexportar explicitamente `native_json_encode`,
`native_toml_encode` e `native_yaml_encode` de loading, apenas em
`pub(crate)`, para o registro de eval. Não muda os reexports preexistentes,
não contém serializers, seleção de âncoras, casts ou wrappers. A assinatura
das nativas é a vigente em `Func::native`, não o exemplo histórico reduzido
do Passo 71. A API Typst e os defaults são definidos pelo owner loading;
este L0 legitima somente a ligação.

O literal de Args no teste local de `mod.rs:2899` deve migrar pelo construtor
sintético aprovado no owner Args, sem inventar origem. O código só pode ser
materializado após aprovação ADR-0127 do conjunto P1307-R3; este rascunho
não declara a aprovação obtida. A inferência de suficiência é refutada por
necessidade de lógica no hub ou alteração dos demais namespaces.

## P1334 — ligação interna de calc_abs

### Medição anterior à decisão

`00_nucleo/diagnosticos/p1334-baseline-public.json`, SHA-256
`bb1e8dde73ba8ae771b72d5c2e344092d797b0a6cf89c9418fa3c553503631aa`,
identifica a árvore/UTC e perda de origem missing. Em
`01_core/src/compiler/stdlib/mod.rs:24,76,211-214`, calc é privado,
make_calc_module é reexportado e calc_abs só é importado em testes;
`compiler/stdlib/calc.rs:131` já declara calc_abs pub(crate). O helper
`compiler/eval/call_dispatch.rs:455-487` precisa alcançar esse ponteiro.

### Obrigação

Reexportar explicitamente calc_abs de calc em pub(crate), sem wrapper,
wildcard, registro paralelo, módulo público ou API Rust externa nova.
Este owner legitima somente a ligação; função/assinatura continuam em
`compiler/stdlib/calc.md`, transporte em `compiler/eval/call_dispatch.md`.
Nenhum helper, fórmula, registro ou reexport anterior muda. Necessidade de
lógica no hub refuta o recorte. Glue interno de paridade ADR-0127 contínuo,
sem entidade, contrato público ou fase nova. Verificar build/testes de
identidade via fachada, ownership 1:1, linhagem e preservação do módulo.

## P1339 — ligação interna de version.at

### Medição anterior à decisão

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`:
`stdlib/mod.rs:49,163-165` declara primitives_constructors privado e encaminha
apenas seus constructors. A implementação e descoberta de version.at passam
ao owner `primitives-constructors/version.md`, com ligação intermediária
declarada em `primitives-constructors.md`. Não existe necessidade de lógica
de versão nesta fachada.

### Decisão

Reexportar `version_type_field` e `dispatch_version_method` de
primitives_constructors apenas em `pub(crate)`, de forma explícita e sem
wrapper. Field access e call dispatch alcançam os helpers por esta fachada;
Version::at continua dona da regra de índice, o owner stdlib/version da
validação da chamada. Não ampliar API externa, namespaces ou constructors,
alterar PARITY_VERSION, criar fórmula/lookup ou usar wildcard no hub.

## P1339 — ligação limitada de Array a partir de Bytes

Medição anterior: `stdlib/mod.rs:49,94,167` já encaminha os constructors
atomizados. O pré-requisito ausente está documentado em
`diagnosticos/p1339-array-prerequisite-measure-r2.json` com HEAD/árvore/UTC.
A autorização `diagnosticos/p1339-array-authorization.md` permite reexportar
`primitives_constructors::native_array_bytes` somente em `pub(crate)`.
Sem wrapper, validação, conversão, registro de Func ou API Rust externa.
Esta exceção à proibição de ampliar constructors da seção version.at fica
restrita à ligação do novo owner `primitives-constructors/array.md`.
