# P780 — `MathIdent` bare não resolve variável do utilizador em modo matemático

> **Passo:** 780
> **Data:** 2026-07-17
> **Commit-base:** `6636c5ea62aeffe07e3ef027358fe95e4e1e451b` (`refactor: renomeia
> rules para engine em L1, L0 e consumidores`) — working tree limpo no início deste
> passo (o commit anterior, `04eda8179`, já incluía P772s-y consolidados).
> **Dependências:** P772y (achado original, §3.6.1), P772l §2.5, diagnóstico
> `diagnostico-auto-lookup-math-passo-301.md`.

---

## Passo 0 — confirmar que não é achado novo

`00_nucleo/diagnosticos/paridade-producao-p772l.md` §2.5 já catalogava exactamente
este caso (`#let myvar123 = 5; $myvar123$` mostra `myvar123` literal, não `5`;
`$foobarbaz$` compila silenciosamente em vez de errar), ligado a
`diagnostico-auto-lookup-math-passo-301.md` §A.5 (P301, decisão consciente de
scope-out na altura). **Confirmado: é o mesmo debt.** Este passo é a
implementação que o fecha — não um achado novo. P772y §3.6.1 apenas
re-descobriu-o como efeito colateral, sem saber da catalogação anterior.

---

## Sonda — mecanismo exacto do vanilla

### Fronteira letra-única vs multi-carácter: decidida no lexer

`lab/typst-original/crates/typst-syntax/src/lexer.rs:742-753` — ao lexar um
identificador em modo math, se o resultado for **um único grapheme**, o token é
`SyntaxKind::MathText` (nunca `MathIdent`); só sequências de mais de um grapheme
tokenizam `MathIdent`/`MathFieldAccess`. Verificado: `01_core/src/engine/lexer/
math.rs:97-103` já replica isto **exactamente** — nenhum trabalho adicional
necessário aqui; a fronteira já chega resolvida a `eval_math_expr`.

### `get_in_math` — prioridade local absoluta (medido, não assumido)

`foundations/scope.rs:75-92`: `top → scopes (reverso) → base.math.scope()`,
erro se nada encontrado. Confirmado por compilação real que **o scope local tem
prioridade sobre símbolos/operadores conhecidos**:

```bash
$ echo '#let sin = 42; $sin$' | typst compile -    # vanilla
→ mostra "42" (não o operador sin)

$ echo '#let alpha = [x]; $alpha$' | typst compile -    # vanilla
→ mostra "x" via mutool trace (glifos t,e,x,t) (não α)
```

### Erro `unknown_variable_math` — hints diferentes de `unknown_variable`

`foundations/scope.rs:439-472` é uma função **distinta** de `unknown_variable`
(usada por P772r em código normal). Medido três casos, exit 1 em todos:

```bash
$ echo '$ foobarbaz $' | typst compile -
error: unknown variable: foobarbaz
  hint: if you meant to display multiple letters as is, try adding spaces
        between each letter: `f o o b a r b a z`
  hint: or if you meant to display this as text, try placing it in quotes:
        `"foobarbaz"`

$ echo '$ str $' | typst compile -
error: unknown variable: str
  hint: `str` is not available directly in math, but is in the standard library
  hint: to access `str` in code mode you can add a hash: `#str`
  hint: or access `str` in math mode by using the `std` module: `std.str`
```

(terceiro caso, `none`/`auto`/`false`/`true`, confirmado por leitura de fonte —
hint "adicionar `#` antes".)

---

## Implementação

1. **`Scopes::get_local`/`Scopes::has_global`** (novos, `01_core/src/engine/
   scopes.rs`) — `get_local`: top → scopes → captured, **sem** cair em `base`
   (paridade do fallback restrito de `get_in_math`, que cai em `base.math`, não
   `base.global`). `has_global`: só para a escolha do hint, nunca para
   resolução. L0: `scopes.md` §P780 (nova secção + 2 entradas na interface + 2
   linhas nos critérios). 5 testes novos.

2. **`value_to_display_content`** (novo, `engine/eval/mod.rs`) — extraído
   *verbatim* do bloco P545 de interpolação `#{expr}` em markup (comportamento
   byte-idêntico preservado, confirmado por `cargo test` sem regressão em
   nenhum teste de markup/interpolação). Conversão genérica `Value →
   Option<Content>`, paridade conceptual com `Value::display()` (vanilla,
   `ExprExt::eval_display`) que o cristalino não tem como método unificado.

3. **`eval_math_expr` (`Expr::MathIdent`)** (`engine/eval/math.rs`) — novo passo
   0, antes dos passos 1 (símbolo Unicode) e 2 (operador `math`, P301):
   `scopes.get_local(name)` — se encontrado, `value_to_display_content` e
   devolve. Passo 3 (antes fallback silencioso `MathIdent(name)`) agora
   `Err(unknown_variable_math(...))` se nenhum dos três resolver.

4. **`unknown_variable_math`** (novo, `engine/eval/math.rs`) — 3 ramos,
   hints byte-idênticos aos medidos acima.

5. **Débito descoberto e corrigido no mesmo passo — símbolo `product`**: a
   suite de testes expôs (via `layout_prod_com_limites_nao_panica`, fixture
   `$product_(k=1)^n$`) que `product` não estava em `ident_to_unicode`
   (`engine/math/symbols.rs`), só `prod` — que **não é nome de símbolo vanilla
   real** (`codex` `sym.txt:525` só define `product ∏`; confirmado:
   `$product$` resolve ∏ no vanilla, `$prod$` erra "unknown variable: prod").
   Adicionado `"product" => Some("∏")` a par de `"prod"` (mantido por
   compatibilidade retroactiva — remoção não avaliada, fora de âmbito). L0:
   `symbols.md` (interface + critério + histórico).

### Correcção ao diagnóstico de P772y §3.6.3

P772y tinha registado "glifo `♥` (U+2665) não renderiza em nenhum contexto —
gap de cobertura de fonte" como débito nº3, independente dos débitos nº1/nº2
(resolução de variável). **Medição pós-P780 mostra que essa classificação
estava errada**: `#let loves = math.class("relation", sym.suit.heart); $x
loves y$` agora renderiza `♥` correctamente (`unicode="♥" glyph="heart"`,
`mutool trace`) — a causa nunca foi cobertura de fonte, foi exactamente o
débito nº1 (`MathIdent` bare não resolvia `loves`, logo o body de
`math.class(...)` nunca chegava ao layout). O gap que **persiste** é mais
estreito e genuinamente distinto: `$sym.suit.heart$` **bare** — field access
directo numa sequência math, fora de qualquer `FuncCall`/`MathIdent` — continua
a produzir página vazia (cai no `_ => Ok(Content::Empty)` genérico de
`eval_math_expr`; nem P772y nem P780 cobrem este caminho).

---

## Validação

```bash
#let myvar123 = 5
$ myvar123 $
```
→ mostra `5` (não `myvar123` literal). Confirmado (`mutool trace`, glifo `5`).

```bash
$ foobarbaz $
```
→ `error: unknown variable: foobarbaz` + 2 hints, byte-idênticos ao vanilla.

```bash
#let r = [nunca deve aparecer]
$ r $
```
→ símbolo itálico `𝑟`/`r` (letra única — lexer produz `MathText`, ignora o
binding; comportamento confirmado do vanilla preservado).

```bash
#let str = 5
$ str $
```
→ mostra `5` (sombra local vence stdlib — `get_local` pára antes de `base`).

```bash
$ str $   # sem binding local
```
→ `error: unknown variable: str` + 3 hints (`in_global`), byte-idênticos ao
vanilla.

```bash
#let loves = math.class("relation", "z")
$x loves y$
$x = y$
```
→ mesmo delta de posição x nos dois casos (`8.445pt`, THICK) — `math.class()`
(P772y) + resolução de variável (P780) combinadas correctamente.

```
cargo build --workspace --release   → 0 erros
cargo test --workspace --release    → 4252+647+33+2+29+2 = 4965 passed, 0 failed
crystalline-lint . --fix-hashes     → 12 ficheiros re-hashed
crystalline-lint .                  → 0 drift (só V7 pré-existente, não relacionado)
```

2 regressões de teste, ambas corrigidas (não são bugs — são testes que
verificavam o comportamento pré-P780, agora incorrecto face ao vanilla):

- `p303_regressao_undef_sem_parens_preservado` → renomeado
  `p780_undef_sem_parens_erra_unknown_variable`, asserção actualizada para
  `Err` com os hints correctos.
- `layout_prod_com_limites_nao_panica` (`$product_(k=1)^n$`) → corrigido pela
  raiz (adição de `"product"` a `ident_to_unicode`, §5 acima), teste original
  inalterado, volta a passar.

Sem regressão nos restantes 4250 testes de `typst-core` (math P299-301, P765b,
P772w, P772y todos verdes).

---

## Critério de fecho do passo

- [x] Confirmado que é o mesmo debt de P301 — referenciado como tal (Passo 0).
- [x] Fronteira letra-única vs multi-carácter confirmada contra o vanilla real
      — decidida no lexer, já replicada correctamente no cristalino, sem
      trabalho adicional.
- [x] `get_local`/equivalente implementado, consultando o scope antes do
      fallback simbólico.
- [x] Erro `unknown variable` com hints para identificador realmente
      desconhecido (3 variantes de hint, medidas e replicadas).
- [x] Variável multi-carácter resolve correctamente.
- [x] Letra única continua simbólica mesmo se definida no scope.
- [x] `cargo test --workspace` verde (4965 passed, 0 failed).
- [x] `crystalline-lint .` zero violações (excepto V7 pré-existente).
- [x] L0 de eval matemático atualizado antes do fecho (`eval.md` §P780,
      `scopes.md` §P780, `symbols.md` — todos com hash real via
      `--fix-hashes`).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p780.md` (este
      ficheiro).

---

## Próximo passo

Dois débitos remanescentes, ambos já registados por P772y/P780 e ainda em
aberto:

1. **Splice de `#expr`/field-access bare em modo math** — `$x #sym.suit.heart
   y$`, `$x #hc y$` (com `hc` `Content`-valued), e `$sym.suit.heart$` bare
   (sem `#`, fora de `FuncCall`) todos produzem página vazia para o valor
   interpolado, sem erro. Distinto do débito fechado aqui (que era
   especificamente sobre `Expr::MathIdent`, resolvido no scope pelo *nome*).
   Precisa de sonda própria no caminho de `Expr::FieldAccess`/interpolação
   `#` dentro de `eval_math_expr`.
2. `image::pdf` e fallback de fontes matemáticas — débitos antigos de P772w,
   ainda não endereçados.

Não há mais nada ligado à "cobertura de glifo `♥`" — essa linha de
investigação fecha-se aqui, reclassificada como o mesmo debt do item 1 acima
(não um problema de fonte).
