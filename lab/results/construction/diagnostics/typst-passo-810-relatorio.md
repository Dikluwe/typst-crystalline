# Relatório — typst-passo-810: triagem sistemática, lote 4 (15 módulos)

**Data:** 2026-07-21
**Executor:** Kimi Code (a pedido do utilizador, nesta conversa — prompt lido de `00_nucleo/materialization/typst-passo-810.md`). Triagem executada por 15 subagentes (1 por módulo), cada um com teste específico obrigatório (lição de P798 no prompt); este relatório consolida os resultados verbatim e foi verificado pelo autor do passo.
**Proveniência das medições:** commit HEAD `c98ffc8acceb99acc23ddebbffc27672e5dcaf47` (commit externo a esta conversa que empacotou P798–P807 às 17:36); working tree não commitado com as alterações de P808+P809 (`git diff HEAD --stat`: 18 ficheiros, +419/-876). Binário cristalino rebuildado após P809 (~19:10). Nota: os relatórios de materialização P799–P806 foram eliminados fisicamente da working tree pela sessão paralela após o commit — estão preservados no commit `c98ffc8ac`.
**Binários:** `./target/release/typst` (cristalino), `lab/typst-original/target/release/typst` (vanilla 0.15.0).

---

## Passo 1 — Selecção do lote (lista exacta registada antes da triagem)

Cruzamento da lista congelada `00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt` (secção `lacuna-inventario`: 82 módulos no snapshot actual) com os 45 módulos triados em P785/P786/P798 (listas extraídas dos relatórios `paridade-producao-p785.md` §Tabela Completa, `paridade-producao-p786.md` §Sonda e §Tabela, `paridade-producao-p798.md` §Metodologia). Restantes por match exacto: **37 módulos** (a estimativa "~21" do handoff era aproximada e por agregação mais grossa). Seleccionados os 15 de maior superfície de língua:

1. `typst_eval` (eval_string)
2. `typst_eval::methods` (missing_method)
3. `typst_library::diag` (24 itens)
4. `typst_library::foundations::calc` (45 itens)
5. `typst_library::foundations::ops` (20 itens)
6. `typst_library::foundations::plugin_` (5 itens)
7. `typst_library::foundations::scope` (7 itens)
8. `typst_library::foundations::target_` (6 itens)
9. `typst_library::layout::grid::resolve` (24 itens)
10. `typst_library::loading::cbor_` (format_cbor_error)
11. `typst_library::loading::read_` (Encoding)
12. `typst_library::math` (4 itens)
13. `typst_library::math::style` (32 itens)
14. `typst_library::pdf::accessibility` (12 itens)
15. `typst_syntax::package` (8 itens)

## Passo 2 — Triagem módulo a módulo (resumo com prova literal)

### 1. `typst_eval` — **ACHADO**
`t1`: `#eval("1 + 2")` → `3` nos dois (paridade do caso base). Divergências: `#eval("= Heading", mode: "markup")` → cristalino `error: argumento nomeado inesperado: 'mode'` / vanilla renderiza `Heading` ✓; `#eval("x + 1", scope: (x: 2))` → cristalino erro / vanilla `3`; `#eval("1 +")` → cristalino `erro de sintaxe em eval()` (genérico, `<detached>`) / vanilla `error: expected expression` com span dentro da string; `#eval(42)` → `eval() espera string, recebeu int` vs `expected string, found integer`. Pontos: vanilla `crates/typst-library/src/foundations/mod.rs:267-322`; cristalino `01_core/src/engine/stdlib/eval.rs:45,53-58,83-90`. Nota do agente (verificada): o comentário doc do cristalino afirma que o default vanilla é `"markup"` — **falso**, é `SyntaxMode::Code`.

### 2. `typst_eval::methods` — **ACHADO**
Caminho mutante/acessor em **paridade verbatim**: `"ab".push("c")` → `type string has no method `push`` nos dois; idem dict/`.at=`/temporário. Divergências: `#(1).foo()` → cristalino `cannot access fields on type int` / vanilla `type integer has no method `foo``; dict key chamada como função → cristalino `não é possível chamar int` sem hints / vanilla `cannot directly call dictionary keys as functions` + 2 hints. Pontos: vanilla `crates/typst-eval/src/call.rs:285-344`; cristalino `01_core/src/engine/eval/bindings.rs:1733,1666`, `closures.rs:837-840`.

### 3. `typst_library::diag` — **ACHADO**
Paridade em 4/7: `unknown variable` com e sem hint de subtracção (verbatim), `is not a valid package name`, `package not found`. Achados:
- **(a)** `#set text(nonexistent-prop: 12pt)` → cristalino **warning** (exit 0!) `propriedade '...' ainda não suportada` + hint que referencia "ADR-0040" / vanilla **erro** (exit 1) `unexpected argument`. Um documento que falha no vanilla compila no cristalino. (`eval/rules.rs:125-136` vs vanilla `foundations/args.rs:262`.)
- **(b)** `#set text(font: "FamiliaQueNaoExiste")` → cristalino silêncio / vanilla `warning: unknown font family`. (`03_infra/src/shaper.rs:178-195` vs vanilla `text/mod.rs:1583`.)
- **(c)** `#set text(size: 12)` → cristalino aceite em silêncio / vanilla `error: expected length, found integer` + hint `did you mean 12pt?`. (`eval/rules.rs:1337-1342` vs vanilla `foundations/cast.rs:341`.)

### 4. `typst_library::foundations::calc` — **ACHADO** (6 sub-achados; ~40 chamadas em paridade)
51 chamadas comparadas linha a linha + 9 ficheiros dedicados:
- **(a)** `calc.asin/acos/atan/atan2` devolvem `float`; vanilla devolve tipo **`angle`** (`30deg/60deg/45deg/63.43deg`). (`stdlib/calc.rs:422-495` vs vanilla `calc.rs:302-359`.)
- **(b)** `calc.quo(-7, 2)` → cristalino `-3` (trunc) / vanilla `-4` (floored); o comentário no código cristalino que cita "-3" como paridade está errado. (`calc.rs:892-921` vs vanilla `calc.rs:1140-1177`.)
- **(c)** `calc.pow(2, -1)` → cristalino erro / vanilla `0.5`.
- **(d)** `decimal` não suportado em nenhuma função calc (`calc.abs(decimal("-342.440"))` erro vs vanilla `342.440`); ausente o erro dedicado `cannot apply this operation to a decimal and a float` + hint.
- **(e)** Funções extra que o vanilla não tem: `calc.log10`, `calc.deg`, `calc.rad` (vanilla: `module `calc` does not contain `log10``).
- **(f)** Precisão: `calc.erf` por aproximação A&S (documentada como consciente, diverge na 7.ª casa); `calc.log(1000)` → `2.9999999999999996` vs `3` (vanilla despacha base 10 para `libm::log10`); `exp(1)`/`cosh(1)` último dígito. Mensagens de erro em português + `<detached>` em 100% dos casos testados. `calc.abs` de Length confirmado por leitura de código (não testável — `Sub length-length` falha antes; registado como não confirmado por teste).

### 5. `typst_library::foundations::ops` — **ACHADO** (9 sub-achados)
`vals.typ` (21 expressões) **byte-idêntico** nos dois. Divergências:
- **(a)** ordenação de `str` ausente (`"a" < "b"` → erro `cannot apply Lt to str and str` vs vanilla `true`).
- **(b)** ordenação lexicográfica de `array` ausente. **(c)** ordenação de `bool` ausente.
- **(d)** divisões `Relative/Relative` e `Ratio/Ratio` ausentes (`50% / 25%` → erro vs vanilla `2.0`; **não** cobertas pelo scope-out do L0 `ops.md`).
- **(e)** repetição `Str * Int` (`"ab" * 2` → erro vs vanilla `"abab"`).
- **(f)** igualdade `Length == Relative` com rel zero (`10pt == (10pt + 0%)` → `false` vs `true`).
- **(g)** ordenação `Length < Relative` ausente (L0 cobre parcialmente).
- **(h)** coerção `Int`↔`Float` não propagada a igualdade/contenção aninhada (`(1,2) == (1.0,2.0)` → `false` vs `true`).
- **(i)** textos das mensagens de erro de tipo (`cannot apply Add to int and str` vs `cannot add integer and string`) — **declarado aceite** no L0 `ops.md:107-113,446-450`; registado para auditoria de substância. Erros com paridade exacta: `cannot divide by zero`, `cannot divide these two lengths`. Não confirmado (bloqueado por outras lacunas): comparações `datetime` (construtor ausente), NaN (`float.nan` ausente).

### 6. `typst_library::foundations::plugin_` — **ACHADO**
Caminho feliz em **paridade total** (WASM real gerado pelo agente): `plugin("hello.wasm").hello()`, `#import plugin(...)`, memória ausente, aridade, `plugin errored with:`, `plugin panicked:` — todos verbatim. Achados: (a) `plugin.transition` ausente (não registado no L0 como scope-out); (b) mensagens L1 divergentes (ficheiro inexistente, tipo do argumento, argumento não-bytes); (c) spans `<detached>` em todo o caminho plugin (o L0 diz "span do callsite" — não confirmado em código); (d) texto do erro de parse WASM difere (versão/feature wasmi); (e) colateral: sem construtor `bytes()` nem `read(encoding:)` — variante `plugin(bytes)` não testável por documento.

### 7. `typst_library::foundations::scope` — **ACHADO**
Núcleo em **paridade verbatim em 12 testes**: `cannot mutate a constant: calc`, `unknown variable` (markup, math, hints de letras/citação/`#none`/`std.`/subtracção plural), mutação de capturadas (`variables from outside the {function|context expression} are read-only...`), sombra de stdlib legal e mutável, precedência captured > constant. Achados: (a) mecanismo `Deprecation` **ausente** — `$join$` → cristalino `unknown variable: join` (erro) / vanilla `warning: `join` is deprecated, use `bowtie.big` instead` (compila); `bowtie` ausente da tabela de símbolos; (b) `$bowtie.big$` → mensagem em português `variável desconhecida: bowtie` (`eval/math.rs:151-154`) vs vanilla sempre inglês com hints. Nota: vanilla emite `while calling `f` at ...` em erros dentro de closures; o cristalino não — registado como apresentação.

### 8. `typst_library::foundations::target_` — **ACHADO**
Núcleo em paridade: `#context target()` → `paged` nos dois (o ângulo do prompt dizia `"pdf"` — **incorrecto**, vanilla devolve `"paged"`, confirmado); `if target() == "paged"` e `repr(type(target()))` → `str` ✓. Achados: (a) **`#target()` fora de `#context` não erra no cristalino** (devolve `paged`) — vanilla: `error: can only be used when context is known` + 2 hints (`native_target` em `stdlib/foundations.rs:1393-1407` ignora `ctx.in_context`); (b) `target(1)` → `target() não aceita argumentos, recebeu 1` vs `unexpected argument` com span. Colateral registado: `#context type("abc")` rende vazio (defeito de `value_to_content` para `Value::Type` em `stdlib/state.rs:137-157`).

### 9. `typst_library::layout::grid::resolve` — **ACHADO** (3)
- **(a)** Header/footer não repetem em quebras de página (débito conhecido reconfirmado): `table.header` aparece 1× no cristalino vs 8× no vanilla (uma por página); footer idem. Scope-out já documentado em `layout/grid.rs:188-192` — continua divergência de língua.
- **(b)** Mensagens de erro divergentes: conflito célula↔célula (`grid placement: conflito — célula explicit...` vs `attempted to place a second cell at column 0, row 0` + hint), colspan overflow, coluna inválida (o cristalino confunde "invalid column" com "exceed num_cols"). A de conflito célula↔header está em paridade de texto e hint (P789), mas perde o span (`<detached>`).
- **(c)** `table.footer` fora do fim aceite no cristalino (vanilla: `error: footer must end at the last row`). Não confirmado: `repeat: false`, níveis de header, múltiplos footers, conflito célula↔footer (scope-out registado em `grid.rs:231-234`). Nota lateral: larguras de coluna auto divergem (11 vs 8 páginas no mesmo doc) — registada como pista, não deste módulo.

### 10. `typst_library::loading::cbor_` — **ACHADO**
Decode válido em paridade (`Nome: Alice, idade: 30, tags: ("a", "b")` idêntico). Mensagem de erro diverge: cristalino `cbor inválido: Semantic(None, "invalid type: break, expected non-break")` (Debug do ciborium, português, `<detached>`) / vanilla `failed to parse parse CBOR (invalid type: break, expected non-break in invalid.cbor)` (texto amigável + nome do ficheiro + span). Pontos: `stdlib/loading.rs:168-172` vs vanilla `loading/cbor.rs:88-98`. Colateral registado: construtor `bytes()` ausente; `array.join` ausente.

### 11. `typst_library::loading::read_` — **ACHADO**
`read("dados.txt", encoding: "utf8")` e `encoding: none` → cristalino `error: argumento nomeado inesperado em read(): 'encoding'` (**língua válida no vanilla**: devolve str / `bytes(44)`); `encoding: "latin1"` → vanilla `error: expected "utf8" or none` (mensagem dedicada) vs a mesma rejeição genérica. Pontos: `stdlib/loading.rs:438` (`reject_named` como 1ª instrução) vs vanilla `loading/read.rs:24-47`. Colateral registado e refutado o comentário do código: ficheiro não-UTF8 sem `encoding:` → cristalino devolve `bytes` em silêncio (o comentário diz "heurística vanilla") / vanilla **erra** `file is not valid UTF-8` — a "heurística vanilla" não corresponde ao vanilla medido.

### 12. `typst_library::math` — **ACHADO** (5)
Paridade medida com controlo: `math.class("relation")` insere THICK (~3.06pt) dos dois lados nos dois binários; `unary` remove espaços nos dois; `mat` sem `&` centra colunas nos dois; Mathy em markup faz auto-wrap nos dois; erros de sintaxe em paridade. Achados:
- **(a)** Domínio de classes: cristalino aceita 15 nomes (`alphabetic`, `diacritic`, `glyph-part`, `space`, `special` a compilarem) vs vanilla rejeita-os no cast (`expected "normal", ..., or "vary"`). (`entities/math_class.rs:115-134` vs vanilla `foundations/cast.rs:502-520`.)
- **(b)** Mensagens de erro divergentes em todos os casos (pt + `<detached>`; `3` reportado como `content` vs `found integer`).
- **(c)** `math`/`sym` bare em math mode compila no cristalino (field access genérico); vanilla: `unknown variable: math` + 3 hints. Causa transversal ao avaliador math (`eval/math.rs:139-146` vs avaliação dedicada do vanilla).
- **(d)** `fence` "spaced" e `class("normal")` sem efeito — **scope-out já registado no L0 de `spacing.rs`**; registado na mesma como divergência de língua (o vanilla mantém o espaço inter-átomo à volta de fences).
- **(e)** `LeftRightAlternator` ausente dentro de células de `mat` — `$ mat(a &= b; x x x x &= y y) $`: vanilla right-aligna `a` no `&` e alinha os `=`; cristalino consome o `&` sem alinhar (`math/layout/matrix.rs:27` vs vanilla `math/ir/resolve.rs:1068` + `typst-layout/math/run.rs:320-331`). Nota: vanilla emite U+FE0E após ♥ (símbolos) — registado para o módulo de símbolos.

### 13. `typst_library::math::style` — **ACHADO** (6 + 1 grave)
Paridade: alfabetos `bb`/`frak`/`mono`/`sans` (incl. dígitos), excepções letterlike de `cal`, `serif`, `upright`. Achados:
- **(a)** `display`/`inline`/`script`/`sscript` **sem efeito geométrico** — todas as alturas idênticas no cristalino (x h=12.54 em todas) vs vanilla (11.00/11.00/7.70/5.50); `display(∑)` não selecciona a variante grande (w=6.51 sempre vs 15.88 vanilla). O código existe (`math_style.rs:45-50`, `math/layout/mod.rs:397-412`) mas não produz efeito mensurável.
- **(b)** Itálico por defeito (P809) **perdido dentro de wrappers de tamanho**: `$display(x)$`/`$script(x)$` emitem `x` plain vs vanilla `𝑥` — `letter_base` retorna `None` para `Script|SScript|Display|Inline` antes do mapeamento itálico (`entities/math_style.rs:148-149`); no vanilla os eixos são ortogonais.
- **(c)** `scr` mapeado para o bloco errado (bold-script U+1D4D0 vs script U+1D49C/excepções letterlike do vanilla) e **variation selectors U+FE00/U+FE01 nunca emitidos** (codex: `to_chancery = to_script + VS1`, `to_roundhand = to_script + VS2` — roundhand usa os MESMOS codepoints de chancery diferenciados pelo selector; `map_glyph` retorna 1 char, limitação estrutural face ao `[char; 2]` do codex).
- **(d)** `$frak()$` sem argumento **não erra** (vanilla: `missing argument: body`) e gera **PDF corrompido** (`Kid object is wrong type (null)`) — validação ausente em `stdlib/math_style.rs:79` + possível bug separado de export com equação vazia (pista de follow-up).
- **(e)** Mensagens de erro divergentes (`bb()` 2 args, `cramped` type reportado como `content`).
- **(f)** Símbolos `NN RR ZZ QQ CC` inexistentes (vanilla resolve para ℕℝℤℚℂ; âmbito do módulo de símbolos).
- `#math.display(...)` via módulo: `module 'math' does not contain field "display"` vs vanilla funciona.

### 14. `typst_library::pdf::accessibility` — **ACHADO**
Gate `A11yExtras` **confirmado empiricamente**: `pdf.table-summary`/`header-cell`/`data-cell` rejeitadas nos dois binários (mensagem genérica de módulo — paridade de comportamento). `pdf.artifact[...]` passthrough idêntico. Achado: `pdf.artifact(kind: "header")[...]` → cristalino `argumento nomeado inesperado: 'kind'` / vanilla aceita (named arg do `ArtifactElem`, `pdf/accessibility.rs:48-53` vs `stdlib/pdf.rs:62-75`). Nota do agente: o `kind` só afecta o tag tree (scope-out global do exportador); se "aceite-e-ignorado" for considerado coberto, o achado é reclassificável — decisão do dono.

### 15. `typst_syntax::package` — **ACHADO** (1)
Parsing de spec em **paridade verbatim em 5 casos** (nome em falta, versão curta, versão não numérica, 4.º componente, namespace inválido — mensagem e coluna 1:8 idênticas). Achado: pacote não encontrado em namespace **não-`@preview`** → cristalino `pacote '@local/inexistente:1.0.0' não encontrado na cache local; download ainda não implementado (ver P-γ de P678)` (português + texto desactualizado — P763 implementou o downloader) / vanilla `package not found (searched for @local/inexistente:1.0.0)`. (`03_infra/src/world.rs:521-524`; o caminho `@preview` já emite a mensagem vanilla via `contracts/package_downloader.rs:63`.) Nota: sem downloads reais nesta triagem; manifest/`typst.toml` não exercitado (exige pacote em cache).

## Passo 3 — Taxa de sinal real (cálculo mostrado)

Classificação por módulo: **15 achados / 15 módulos triados = taxa de sinal 100%**. A taxa é alta face aos lotes anteriores (27%/80%/60%) porque os 15 módulos seleccionados foram os de **maior superfície de língua** dos 37 restantes (a selecção não foi aleatória — foi pelos módulos mais ricos; os 22 restantes são maioritariamente internos/mecânicos: `text::font::*`, `layout::*` internos, `visualize::image::*` já decididos, `utils::pico::*`, `syntax::span`). Nuance obrigatória: vários módulos têm núcleos funcionais em paridade (scope 12/12 testes centrais, plugin caminho feliz, package 5/6, target_ valor/semântica, math relation/unary, ops 21 expressões byte-idênticas, calc ~40 chamadas) — os achados concentram-se em guard rails (validações ausentes), mensagens de erro e features específicas.

## Tabela de achados pendentes (formato handoff — fila para passos dedicados)

| # | Módulo | Achado |
|---|---|---|
| 1 | `typst_eval` | `#eval` sem `mode:`/`scope:`; erros de sintaxe/tipo genéricos + span detached |
| 2 | `typst_eval::methods` | Método inexistente: mensagens divergentes no caminho normal; dict-key-call sem hints dedicados |
| 3 | `typst_library::diag` | `#set` prop inválida → warning (exit 0) em vez de erro; sem warning "unknown font family"; `set text(size: <int>)` aceite em silêncio |
| 4 | `foundations::calc` | asin/acos/atan/atan2 float vs `angle`; quo trunc vs floored; pow int-neg erro; decimal não suportado; log10/deg/rad extra; erf/log/exp precisão |
| 5 | `foundations::ops` | ordenação str/array/bool ausente; div Relative/Relative e Ratio/Ratio; Str*Int; eq/ord Length↔Relative; coerção Int↔Float não aninhada; (mensagens de erro: aceite declarado no L0) |
| 6 | `foundations::plugin_` | `plugin.transition` ausente (não registado no L0); mensagens L1 divergentes; spans detached; texto erro parse WASM |
| 7 | `foundations::scope` | `Deprecation` ausente (`join` erro vs warning; `bowtie` ausente); mensagem math em português |
| 8 | `foundations::target_` | `#target()` fora de `#context` não erra; texto de erro com args diverge; colateral: `#context type()` vazio |
| 9 | `layout::grid::resolve` | header/footer não repetem (débito reconfirmado); mensagens de erro divergentes; footer fora do fim aceite |
| 10 | `loading::cbor_` | mensagem de erro CBOR diverge (Debug vs texto amigável + ficheiro + span) |
| 11 | `loading::read_` | `encoding:` rejeitado (língua válida); não-UTF8 devolve bytes em silêncio (vanilla erra — comentário "heurística vanilla" refutado) |
| 12 | `typst_library::math` | domínio de classes 15 vs 10; field access bare em math compila; fence spaced/class("normal") sem efeito (scope-out L0); LeftRightAlternator em mat ausente; mensagens divergentes |
| 13 | `math::style` | display/inline/script/sscript sem efeito geométrico; itálico P809 perdido em wrappers de tamanho; scr bloco errado + sem variation selectors; frak() sem arg + PDF corrompido; NN/RR/ZZ/QQ/CC ausentes; `#math.display` ausente |
| 14 | `pdf::accessibility` | `pdf.artifact(kind:)` rejeitado (vanilla aceita — só tag tree; decisão de reclassificação do dono) |
| 15 | `typst_syntax::package` | erro "pacote não encontrado" para namespace não-preview diverge (mensagem PT desactualizada) |

**Achados transversais registados** (não são de um módulo): PDF corrompido com equação vazia (`$frak()$` — pista de bug de export); construtor `bytes()` ausente; `float.nan` ausente; construtor `datetime(...)` ausente; `array.join` ausente; spans `<detached>` generalizados nas mensagens de erro L1; mensagens em português vs inglês (parcialmente aceite em L0s — auditoria de substância pendente, ADR-0108); vanilla emite U+FE0E/variation selectors em símbolos.

## Contagem actualizada

Módulos restantes não triados após este lote: **22** (37 − 15): `typst_library::foundations` (define), `typst_library::introspection`, `typst_library::layout`, `typst_library::layout::abs`, `typst_library::layout::axes`, `typst_library::layout::corners`, `typst_library::layout::em`, `typst_library::layout::fragment`, `typst_library::layout::frame`, `typst_library::loading`, `typst_library::text::font::book`, `typst_library::text::font::exceptions`, `typst_library::text::font::info`, `typst_library::text::font::metrics`, `typst_library::text::font::variations`, `typst_library::visualize::image::pdf`, `typst_library::visualize::image::raster`, `typst_library::visualize::image::svg`, `typst_syntax::span`, `typst_utils::pico::bitcode`, `typst_utils::pico::exceptions`, `typst_library::foundations::styles::rule`. A varredura original **não** está completa — restam estes 22 para o lote 5.
