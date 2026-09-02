# Prompt L0 — `compiler/eval` — dispatcher e contexto
Hash do Código: c0181201

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/mod.rs`
**Vanilla ratificado:** `a51e02804`
**ADRs:** ADR-0024, ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Medição anterior à decisão

O consumer possui `EvalContext`, os entrypoints públicos, a criação do scope
base, a passagem dupla, `eval_markup` e o dispatcher exaustivo de `Expr`; os
corpos especializados já delegam aos seus módulos donos.

## Contrato

### P1225/P1285 — formatter público de representação para serialização

L1 expõe `repr_stroke_value(&Stroke) -> String` exclusivamente para consumidores
que precisam da representação morfológica pública de `Stroke`. A função delega
ao owner `compiler/eval/repr.rs`; não expõe `repr_value` genérico, não faz I/O e
não autoriza fallback de tipos desconhecidos. Consumer imediato: serializer
JSON nominal da CLI em `02_shell/src/cli.rs`.

P1285 mede no vanilla ratificado
`lab/typst-original/crates/typst-library/src/foundations/value.rs:343-362` que
todo `Value` fora do conjunto estruturado é serializado como string de sua
`repr` pública. Para que L2 cumpra esse contrato sem duplicar formatação nem
aceder ao módulo privado, o gate ADR-0127 propõe acrescentar:

```rust
pub fn repr_value_for_serialization(value: &Value) -> String;
```

A função é uma fachada pura e total sobre `repr::repr_value`; não serializa
JSON/YAML, não faz I/O, não usa `Debug` e não decide quais variantes são
estruturadas. Essa classificação permanece no owner L2
`00_nucleo/prompts/shell/cli.md`. `repr_stroke_value` permanece compatível.
Esta ampliação de assinatura pública foi **CONFIRMADA PELO DONO EM 2026-08-30**
no gate P1285 e está autorizada para materialização.

Os entrypoints constroem scope fresco e avaliam `Source` sem I/O direto.
`eval_expression` avalia código isolado. O dispatcher preserva spans, scopes,
short-circuit, joins e eventos de fluxo. `EvalContext` transporta limites,
metadados, target, features e `FlowEvent`, sem estado global mutável. Conteúdo
de introspecção pré-show e conteúdo final pós-show permanecem distintos;
evento residual no entrypoint é erro. Mudança pública, de default ou fase para
no gate ADR-0127.

## Aceitação

Entrypoints, scope base, duas passagens, dispatcher, spans e fluxo residual são
cobertos pela suíte de eval; L1 permanece puro.

## P1215 — source numerizada em `eval_expression`

Medição mostrou que `eval_expression` avaliava via `native_eval` com
`Args::positional`, portanto com span detached. O entrypoint deve criar uma
`Source` em modo code com `world.main()` e o texto integral, avaliar os filhos
da raiz numerizada no mesmo scope fresco e devolver diagnósticos cujos spans
resolvem contra uma `Source` idêntica. Isto preserva assinatura, valores,
features e fase; apenas deixa de descartar localização pública no CLI `eval`.

## P1286 — markup smartquote consome o mesmo contrato de quotes

### Medição anterior à decisão

`eval_markup` já consulta `smartquote.enabled`/`smartquote.quotes`, mas resolve
o token antes do layout e não conhece `alternative`. A chamada programática
percorre layout. O baseline usa uma configuração única para ambas as faces.

### Decisão

Markup consulta `enabled`, `alternative` e o valor canônico de `quotes` da
StyleChain e usa o mesmo resolver puro de `compiler/lang/quotes`. Continua a
emitir `Content::Text` e não muda de fase; chamada programática continua a
emitir o leaf vigente. O fragmento P1286 exige igualdade morfológica dos pares
medidos entre `#set`/markup e chamada direta, sem alegar unificação do stack
contextual do vanilla.

## P1292 — namespace de `place.flush` preservado por `.with`

### Medição anterior à decisão

No baseline, o scope base registra `place` com `Func::native` e field access
falha. No vanilla ratificado, `place.flush` é função nomeada `flush`, e
`place.with(dx: 1pt).flush` resolve o mesmo membro e continua chamável.
`Func::With` cristalino já delega `scope()` à função interior.

### Decisão

O scope base constrói um namespace fechado de `place` com exatamente o membro
`flush`, ligado a `compiler/stdlib/layout.md::native_flush`, e registra `place`
por `Func::native_with_namespace`. A função principal continua a mesma
`native_place`; não vira Module, não perde chamabilidade e não copia argumentos
pré-ligados para `flush`. A delegação vigente de `Func::With::scope()` é
preservada, não duplicada neste owner.

Aceitação: `repr(place.flush) == "flush"`,
`repr(place.with(dx: 1pt).flush) == "flush"` e ambas as chamadas constroem a
mesma `Content::Flush`. Named alheio ao namespace não é inventado.

## P1288 — filtragem uniforme de bindings por feature (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`01_core/src/compiler/eval/mod.rs:117-120` transporta `Features` pelo path
HTML, e `:911-921` aplica um caso especial somente ao identificador `html`.
O módulo `pdf` é instalado integralmente em `:2113`; não há filtragem das três
funções `a11y-extras` no contexto efetivo.

### Decisão proposta

`EvalContext.features` usa o owner canônico `entities::compiler_features`.
A construção/resolução do scope expõe cada binding gated somente quando a
feature correspondente está presente. Sem `A11yExtras`,
`pdf.table-summary`, `pdf.header-cell` e `pdf.data-cell` não existem; com a
feature, os três existem em conjunto. `Html` continua independente e o target
não modifica o set.

O diagnóstico de binding gated ausente conserva a semântica pública medida de
feature não habilitada. O mecanismo concreto pode ser scope filtrado ou lookup
condicional; igualdade estrutural do scope não é contrato. Não criar binding
stub nem registrar só uma das três funções.

## P1293 — identidades públicas curtas nos namespaces `grid` e `table`

### Medição anterior à decisão

Em `2026-09-01T13:24:01-03:00`, no baseline
`7dd25ff0e222b6c7c640d6bc7957b98f94227507` com working tree não commitada,
`01_core/src/compiler/eval/mod.rs:1804-1827,2059-2078` instalava os dez fields
corretos, mas passava nomes `grid_*`/`table_*` a `Func::native`. Os aliases flat
separados permanecem em `:2083-2110`. A fonte vanilla pinada
`layout/grid/mod.rs:422-438,574-677,767-768` e
`model/table.rs:289-305,494-613,732-733` declara nomes curtos. O recibo P1293
mediu chamadas diretas e via `.with` para os dez siblings e confirmou que a
causa da diferença pública é somente o primeiro argumento textual da instância
namespaced.

### Decisão

Cada instância anexada ao namespace usa exatamente este nome público:

| Namespace | members e `repr` |
|---|---|
| `grid` | `cell`, `header`, `footer`, `hline`, `vline` |
| `table` | `cell`, `header`, `footer`, `hline`, `vline` |

Chaves de scope, function pointers, argumentos, payloads e chamada direta ou
via `.with(...)` permanecem os vigentes. Os seis aliases flat cristalinos
`grid_cell`, `grid_header`, `grid_footer`, `table_cell`, `table_header` e
`table_footer` conservam seus nomes históricos; P1293 não cria aliases flat
`*_hline`/`*_vline` nem renomeia símbolos Rust.

O baseline também mediu uma divergência preexistente fora das 15 diferenças
de superfície: `grid/table.header` e `footer` cristalinos exigem pelo menos uma
célula, enquanto o vanilla aceita zero ou vários children, e algumas aridades
extras alcançam serializer/diagnóstico diferente. P1293 **não** corrige nem
reivindica paridade dessa aridade/diagnóstico; os calls atuais devem ser
preservados byte a byte quanto a function pointer e comportamento. Uma mudança
nessa divergência exige medição e L0 próprios.

Aceitação exige os dez nomes curtos, os mesmos tipos de conteúdo, `.with`
preservado e aliases flat intactos. Algum sibling com underscore, pointer/call
alterado ou divergência fora de escopo relaxada bloqueia o lote. `Unknown`
nunca é sucesso. A mudança de identidade pública fica bloqueada pelo gate
humano P1293 antes do código.
