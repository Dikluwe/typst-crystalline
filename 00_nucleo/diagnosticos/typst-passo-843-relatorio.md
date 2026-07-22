# Relatório — typst-passo-843: `foundations` (define) — 7 achados (#40–#46) + `array.join` (#60)

**Data**: 2026-07-22
**Proveniência das medições**:
- Commit HEAD: `6740c2729122bac880e9a642496af7d8ed4e0ba2`; working tree **não commitado** durante todo o passo (a lista exacta de ficheiros alterados está no resumo final; `git diff --stat` no fim: 19 ficheiros, +1172/−128).
- Binário vanilla: `lab/typst-original/target/release/typst` (pré-existente, não rebuildado neste passo).
- Binário cristalino: `./target/release/typst`, rebuildado após as alterações (`cargo build --release`, 17.6–17.8 s).
- Fixtures: `temp/p843/*.typ`. Vanilla: `typst compile <f> <out.pdf>`; cristalino: `typst <f> -o <out.pdf>`; texto extraído com `pdftotext` (erros lidos do stderr).

**Baseline confirmado no arranque** (HEAD, árvore limpa):
- `cargo test -p typst-core` → **4583 passed, 0 failed**
- `cargo test -p typst-infra` → **698 passed, 0 failed**

**Contagem final**:
- `typst-core` → **4621 passed, 0 failed** (+38 = 37 testes `p843_*` + 1 teste `p843_f1_duration_constructor_weeks`; testes antigos actualizados, nenhum removido)
- `typst-infra` → **698 passed, 0 failed** (inalterado)
- `crystalline-lint .` → **exit 0** (após `--fix-hashes`; restam apenas warnings V7 pré-existentes de prompts órfãos)

---

## #40 (F1) — `repr(duration)` diverge

**Antes (medido)** — `temp/p843/f1_duration.typ`:
- vanilla: `duration(seconds: 3)` · `duration(hours: 1, minutes: 1, seconds: 1)` · `duration(days: 1, hours: 2, minutes: 3, seconds: 4)` · `duration(hours: 1)` · `duration(hours: 1, minutes: 30)` (de `minutes: 90`) · `duration(weeks: 1, days: 1)`
- cristalino: `duration(3s)` · `duration(1h1m1s)` · `duration(1d2h3m4s)` · …
- Extra medido (`f1_frac.typ`): `duration(seconds: 3)/2` → vanilla `duration(seconds: 1)` (sub-segundo truncado); `duration(seconds: 1)/3` → vanilla `duration()` (zero componentes).

**Código**: `repr_duration` novo em `01_core/src/engine/eval/repr.rs` — decomposição `weeks…seconds` a partir dos nanos (segundos truncados), só componentes não-zero, formato `duration` + `pretty_array_like` (porte literal de `foundations/repr.rs:170-223`).

**Achado incidental (decisão minha, documentada)**: o constructor `duration(...)` cristalino **ignorava silenciosamente `weeks:`** (produzia valor errado sem erro). Adicionado `weeks` em `01_core/src/engine/stdlib/primitives_constructors.rs` — o vanilla aceita-o (medido) e o repr nomeado inclui a componente.

**Depois (medido, release rebuildado)**: `f1_duration.typ` cristalino ≡ vanilla, incluindo `duration(weeks: 1, days: 1)`.

**Testes**: `p843_f1_repr_duration_*` (6, em `repr.rs`), `p843_f1_duration_constructor_weeks` (em `eval/tests.rs`). Testes antigos actualizados: `repr_value_duration`, `repr_value_complex_types`.

**Nuance/limitação**: durações negativas não são representáveis na entidade cristalina (`u64` nanos); o vanilla aceita (`duration(minutes: -1, seconds: -59)` medido). Fora de escopo (exigiria mudar a entidade para signed). O constructor cristalino continua a aceitar `milliseconds/microseconds/nanoseconds` (extensão P405) que o vanilla rejeita (`unexpected argument: milliseconds`, medido) — mantido por compatibilidade, registado no L0.

---

## #41 (F2) — `repr` de content diverge

**Antes (medido)** — `f2_content.typ`, `f2_nest.typ`, `f2_misc.typ`, `f2_iso.typ`:
- vanilla: `repr([hi *bold*])` → `sequence([hi], [ ], strong(body: [bold]))`; `repr([a [b [c]] *x*])` → modo **vertical** (uma peça por linha, indentação 2 espaços, vírgula final); `repr([])` → `[]`; `repr([c])` → `[c]`.
- Descoberta de sonda: em markup, `[c]` aninhado é **texto literal** (não content block) — por isso o vanilla mostra `[[], [c], []]` (= `Text("[")`, `Text("c")`, `Text("]")`).
- cristalino: `["hi"space*"bold"*]`, etc.

**Código**: `repr_content` em `repr.rs` — Text → `[texto]` cru (sem aspas — `TextElem::repr` vanilla), Space → `[ ]`, Empty/sequência vazia → `[]`, Sequence → `sequence` + `pretty_array_like` (MAX_WIDTH=50, horizontal/vertical), Strong → `strong(body: …)`, Emph → `emph(body: …)`. Os restantes variantes (~60) mantêm a forma cristalina prévia.

**Depois (medido)**: `sequence([hi], [ ], strong(body: [bold]))` ≡ vanilla; caso aninhado bate na **string** (unit test exacto) — ver limitação abaixo.

**Testes**: `p843_f2_repr_content_*` (5, incl. recursão 2 níveis e quebra vertical de 59 colunas). Actualizados: `repr_content_text_and_sequence`, `repr_content_heading`, `repr_value_content_heading`, `repr_value_content_label`, `p421_repr_sequence`.

**Limitações (a rever)**:
1. **Render de `\n` em texto no PDF cristalino está quebrado** (pré-existente, fora de F2): `#"a,\nb,"` renderiza como `a,,` — a segunda linha desaparece e a vírgula duplica. A string do repr vertical está correcta (unit test), mas a extracção via PDF dos casos verticais mostra este artefacto. Recomendo achado novo para a camada de layout/texto.
2. **Parser cristalino funde texto corrido**: `[hello world]` é um único `Text` no cristalino; vanilla produz `Text+Space+Text` (`sequence([hello], [ ], [world])`). Divergência de parsing, fora de F2 (o teste `p421_repr_sequence` foi mudado para o caso medido `[hi *bold*]`, onde a estrutura coincide).
3. Os restantes variantes de `Content` (heading, emph-like, raw, etc.) mantêm formato cristalino — scope-out explícito; cobrir todos exigiria o sistema de campos dos elementos (trabalho de outro passo).

---

## #42 (F3) — `repr(type(none))`/`repr(type(auto))` divergem

**Antes (medido)** — `f3_type_none_auto.typ`: vanilla `type(none) type(auto) int`; cristalino `none auto int`.

**Código**: braço `Value::Type` de `repr_value` em `repr.rs` — `Type::None` → `"type(none)"`, `Type::Auto` → `"type(auto)"`, restantes → nome curto (paridade `ty.rs:159-163`).

**Depois (medido)**: ≡ vanilla.

**Testes**: `p843_f3_repr_type_none_e_auto`, `p843_f3_repr_type_restantes_nome_curto`.

---

## #43 (F4) — tipo `bytes` sem constructor

**Antes (medido)** — `f4_*.typ`:
- vanilla: `bytes((1,2,3))` → `bytes(3)`; `bytes("abc")` → `bytes(3)`; `bytes(())` → `bytes(0)`; `bytes("α")` → `bytes(2)`; `bytes(3)` → **erro** `expected string, array, or bytes, found integer`; `bytes((256,))`/`bytes((-1,))` → `number must be between 0 and 255`; `bytes(("a",))` → `expected integer, found string`; `bytes()` → `missing argument: value`.
- cristalino: `error: type bytes does not have a constructor`.

**Código**: `native_bytes` em `stdlib/foundations.rs` + braço `Type::Bytes` no despacho de `closures.rs` + export em `stdlib/mod.rs`.

**Depois (medido)**: todos os casos ≡ vanilla (valores e mensagens verbatim).

**Testes**: `p843_f4_bytes_*` (5). Fecha a lacuna transversal registada em P810/P819.

---

## #44 (F5) — tipo `datetime` sem constructor

**Antes (medido)** — `f5_*.typ`:
- vanilla: `datetime(year: 2024, month: 1, day: 1)` ok; com hora ok; **só-hora** `datetime(hour: 14, minute: 30, second: 5)` ok (repr `datetime(hour: 14, minute: 30, second: 5)`); erros verbatim: `month is invalid` (month: 13), `date is invalid` (30 fev), `time is invalid` (hour: 25), `date is incomplete` + hint, `time is incomplete` + hint, `at least one of date or time must be fully specified` + 2 hints, posicional → `unexpected argument`, `year: 2024.5` → `expected integer, found float`.
- vanilla repr: formato **nomeado** (`datetime(year: 2024, month: 1, day: 1)`), com quebra vertical quando > 50 colunas — **não** o ISO que o cristalino usava.
- cristalino: `error: type datetime does not have a constructor`.

**Código**:
- Entidade `Datetime` (`entities/world_types.rs`) reestruturada: `date: Option<time::Date>` (antes obrigatória), `new_time(h,m,s)` novo, `from_parts` novo; `year()/month()/day()/weekday()` passam a `Option` (paridade com a entidade vanilla `Datetime::{Date, Time, Datetime}`). Únicos call sites: `repr.rs` (reescrito) e um teste em `03_infra/src/world.rs` (actualizado).
- `native_datetime` em `stdlib/foundations.rs` + braço `Type::Datetime` no despacho.
- `repr_datetime` reescrito para o formato nomeado (só componentes presentes).

**Decisões minhas (documentadas)**:
1. Corrigi também o **repr de datetime** (achado cobria só o constructor, mas o constructor torna o repr observável e a divergência era medida directa). Testes antigos `repr_value_datetime_*` actualizados.
2. Restruturei a entidade para suportar só-hora (medido no vanilla), em vez de rejeitar com erro divergente.

**Depois (medido)**: todos os casos ≡ vanilla (valores, mensagens e hints).

**Testes**: `p843_f5_datetime_*` (6). Actualizados: `repr_value_datetime_date_only`, `repr_value_datetime_with_time` (agora vertical, 62 colunas), testes da entidade em `world_types.rs`, teste em `03_infra/src/world.rs`.

---

## #45 (F6) — `panic`: assinatura e mensagem divergentes

**Antes (medido)** — `f6_*.typ`:
- vanilla: `panic("this is wrong")` → `panicked with: this is wrong`; `panic("a", 1, (x: 2))` → `panicked with: a, 1, (x: 2)`; `panic(42)` → `panicked with: 42`; `panic()` → `panicked`.
- cristalino: mensagem nua; não-strings → erro de tipo; múltiplos args → erro de aridade.

**Código**: `native_panic` reescrito (`stdlib/panic.rs`) — variádico; strings cruas, outros valores via `repr_value`; separador `", "`; prefixo ` with: ` só com argumentos.

**Depois (medido)**: ≡ vanilla nos 4 casos.

**Testes**: `p843_f6_panic_*` (4). Actualizados: `native_panic_aborta_com_mensagem`, `native_panic_aceita_mensagem_vazia`.

**Nuance**: a keyword alternativa `error(...)` do vanilla (`#[func(keywords = ["error"])]`) não foi implementada — scope-out registado no L0.

---

## #46 (F7) — `assert`: mensagens divergentes

**Antes (medido)** — `f7_*.typ`:
- vanilla: `assert(false)` → `assertion failed`; `assert(false, message: "custom msg")` → `assertion failed: custom msg`; `assert.eq(1, 2)` → `equality assertion failed: value 1 was not equal to 2` (já em paridade no cristalino).
- cristalino: `Asserção falhou` / mensagem nua.

**Código**: `native_assert` (`stdlib/assert.rs`) — mensagens verbatim; `assert.eq`/`assert.ne` **não tocados** (paridade mantida, testes de não-regressão adicionados).

**Depois (medido)**: ≡ vanilla nos 3 casos.

**Testes**: `p843_f7_assert_*` (4, incl. não-regressão de `assert.eq`/`assert.ne`). Actualizados: `native_assert_false_gera_erro_com_mensagem_padrao`, `eval_assert_false_gera_erro_com_mensagem_padrao` (ambos esperavam português).

---

## #60 (incidental) — `array.join` inexistente

**Antes (medido)** — `join*.typ`:
- vanilla: `("a","b").join("-")` → `"a-b"`; `.join()` → `"ab"`; `("a",).join("-")` → `"a"`; `().join("-")` → `none`; `().join("-", default: "x")` → `"x"`; `last:` funciona (3, 2 e 1 elementos); `(1,2).join("-")` → **erro** `cannot join integer with string`.
- cristalino: `error: type array has no method 'join'`.

**Código**: `array_join` em `stdlib/collections.rs` + braço no despacho. Reutiliza `operators::join` (a op da linguagem, P728) — mesma semântica do vanilla (`Array::join` delega em `ops::join`).
**Também**: a mensagem de erro de `operators::join` passou a usar **nomes longos** (`integer`/`string`/`boolean`) — paridade plena com o vanilla `mismatch!`; antes era divergência de texto aceite (P728). Fecha essa divergência também nos code blocks (`{ 1; 2 }`).

**Depois (medido)**: todos os casos ≡ vanilla.

**Testes**: `p843_join_*` (6, incl. `default:`, `last:`, content via op join, erro verbatim).

---

## Ficheiros tocados

Código L1 (`01_core`):
- `src/engine/eval/repr.rs` — F1, F2, F3, repr datetime (F5); helpers `pretty_comma_list`/`pretty_array_like`; testes.
- `src/engine/eval/closures.rs` — despacho `Type::Bytes`/`Type::Datetime`.
- `src/engine/eval/operators.rs` — erro do `join` com nomes longos.
- `src/engine/eval/tests.rs` — módulo `tests_p843` (25 testes); `p421_repr_sequence`; assert antigo.
- `src/engine/stdlib/foundations.rs` — `native_bytes`, `native_datetime`, helper `long_type_name`.
- `src/engine/stdlib/mod.rs` — exports; testes panic/assert antigos.
- `src/engine/stdlib/panic.rs` — F6.
- `src/engine/stdlib/assert.rs` — F7.
- `src/engine/stdlib/collections.rs` — `array_join` (#60).
- `src/engine/stdlib/primitives_constructors.rs` — `weeks` no constructor duration.
- `src/entities/world_types.rs` — `Datetime` com data opcional + `new_time`/`from_parts`.

L3 (`03_infra`): `src/world.rs` — 1 teste ajustado ao `year(): Option`.

L0 (prompts actualizados + `crystalline-lint --fix-hashes .`):
- `00_nucleo/prompts/engine/stdlib/foundations.md` (repr tabela + constructors bytes/datetime + F3)
- `00_nucleo/prompts/engine/stdlib/panic.md` (reescrito: variádico)
- `00_nucleo/prompts/engine/stdlib/assert.md` (mensagens)
- `00_nucleo/prompts/engine/stdlib/collections.md` (`join`)
- `00_nucleo/prompts/engine/stdlib/primitives-constructors.md` (`weeks`; nota ms/μs/ns)
- `00_nucleo/prompts/engine/eval/ops.md` (erro join com nomes longos)
- `00_nucleo/prompts/world-types.md` (`Datetime`)

Fixtures: `temp/p843/*.typ` (+ PDFs de medição).

## Desvios e limitações a rever antes do commit

1. **Bug pré-existente encontrado (não corrigido)**: render de `\n` embutido em texto no PDF cristalino perde linhas/duplica pontuação (`#"a,\nb,"` → `a,,`). Afecta só a extracção via PDF dos reprs verticais; as strings estão correctas (unit tests). Sugiro registar como achado novo (camada de layout/texto).
2. **Parser funde texto corrido** (`[hello world]` = 1 Text vs 3 nós no vanilla) — divergence de morfologia do content tree, fora de F2; o teste E2E usa o caso medido `[hi *bold*]`.
3. **Repr de content: só os variantes centrais** (Text/Space/Empty/Sequence/Strong/Emph) estão no formato vanilla; os restantes ~60 mantêm formato cristalino (scope-out; exige sistema de campos por elemento).
4. **Durações negativas** não representáveis (`u64` nanos) — vanilla aceita; exigiria entidade signed.
5. **`milliseconds/microseconds/nanoseconds`** no constructor duration: extensão cristalina mantida (vanilla rejeita, medido) — registado no L0.
6. **`error(...)`** como alias de `panic` (keyword vanilla) não implementado — scope-out no L0.
7. **Hints de `datetime`** incluídos (o renderer cristalino já os mostra); hint de `bytes` n/a.
8. Decisões minhas explicitadas no corpo: repr de datetime corrigido junto com F5; reestruturação da entidade `Datetime`; `weeks` no constructor duration; nomes longos no erro do `join` (fecha divergência P728).
