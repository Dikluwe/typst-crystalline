# P782 — Splice de `#expr`/field-access bare em modo matemático

> **Passo:** 782
> **Data:** 2026-07-17
> **Commit-base:** `6636c5ea62aeffe07e3ef027358fe95e4e1e451b` — working tree com
> P780 e P781 ainda não commitados no início deste passo (ver respectivos
> relatórios).
> **Dependências:** P772y (achado original, §3.6.2), P780 (confirmação de
> débito distinto, `value_to_display_content`/`scopes.get_local` disponíveis
> para reutilizar).

---

## Sonda — mecanismo exacto do vanilla

### Vanilla tem uma distinção arquitectural que o cristalino não tem

`typst-syntax/src/ast.rs:294-296` — `Expr::MathFieldAccess(MathFieldAccess<'a>)`
é um variant **dedicado** do `Expr` genérico, produzido pelo lexer math para
field access bare (`sym.suit.heart` sem `#`): `MathFieldAccess::target()`
devolve `MathAccess<'a> = MathIdent | MathFieldAccess` (recursivo,
`typst-syntax/src/ast.rs:972-975`). Isto é **distinto** de `#sym.suit.heart`
(via Hash), que produz um `Expr::FieldAccess` genérico via
`embedded_code_expr` — o mesmo caminho de código normal usado em markup.

`ast::Math::exprs()` (`typst-syntax/src/ast.rs:889-891`) devolve o `Expr`
genérico — **qualquer** variant (específico de math ou não) passa por
`expr.eval_display(vm)` = `self.eval(vm)?.display()`
(`typst-eval/src/math.rs:176-184`, `ExprExt::eval_display`) — conversão
genérica `Value → Content`, unificada para todos os casos.

### Estado do cristalino — confirmado por leitura de fonte, não suposição

`engine/parse/math.rs:63-64`:
```rust
SyntaxKind::MathIdent | SyntaxKind::FieldAccess => { ... }
// comentário: "The lexer manages creating full FieldAccess nodes if needed"
```

O lexer math do cristalino monta directamente um nó `SyntaxKind::FieldAccess`
**genérico** (não um `MathFieldAccess` dedicado) para `sym.suit.heart` bare.
**Consequência estrutural verificada**: nesta arquitectura específica,
`#sym.suit.heart` (via Hash) e `sym.suit.heart` bare produzem **literalmente
o mesmo** `Expr::FieldAccess` — ao contrário do vanilla, onde são dois `Expr`
variants distintos. Confirmado empiricamente: alterar o tratamento de
`Expr::FieldAccess` corrigiu os dois casos simultaneamente, sem nenhum
código adicional para distinguir "veio de `#`" vs "veio bare".

### Estado do gap (medido, antes deste passo)

`grep -n "Expr::FieldAccess\|_ =>" 01_core/src/engine/eval/math.rs` confirmou
que `Expr::FieldAccess` só era tratado dentro de `eval_math_callee` (P772y,
usado exclusivamente para o **callee** de uma `FuncCall`) — `eval_math_expr`
em si não tinha nenhum braço para `Expr::FieldAccess`/`Expr::Ident`
standalone, caindo no catch-all final `_ => Ok(Content::Empty)`.

Três casos medidos, todos exit 0 sem erro, página sem o conteúdo esperado:
```
$x #sym.suit.heart y$   → só "x y"
$sym.suit.heart$        → página vazia
#let hc = sym.suit.heart; $x #hc y$   → só "x y"
```

---

## Implementação

### 1. Catch-all de `eval_math_expr` substituído por dois braços

```rust
other @ Expr::FieldAccess(_) => {
    let value = eval_math_callee(scopes, ctx, engine, other)?;
    Ok(value_to_display_content(value).unwrap_or(Content::Empty))
}
other => {
    let value = eval_expr(other, scopes, ctx, engine)?;
    Ok(value_to_display_content(value).unwrap_or(Content::Empty))
}
```

`Expr::FieldAccess` não pode passar por `eval_expr` genérico directo: o alvo
do access (`sym` em `sym.suit.heart`) é lexado como `Expr::MathIdent` mesmo
dentro do nó `FieldAccess`; `eval_expr` trata `Expr::MathIdent` como
fronteira deliberada (`Value::None`, `eval/mod.rs`), o que faz o field
access genérico falhar com "field access não suportado em none" (medido
directamente ao tentar essa abordagem primeiro). `eval_math_callee` (P772y)
já contornava exactamente este problema para o caminho de callee —
reutilizado aqui sem duplicação, só generalizado (docstring actualizada).

Qualquer outro `Expr` (`Ident`, `LetBinding`, literais, ...) passa por
`eval_expr` genérico directo — paridade conceptual com `eval_display`
(vanilla). Seguro por construção: os `Expr` math-específicos nunca chegam a
este catch-all (apanhados pelos braços anteriores do match); os que
`eval_expr` não reconhece (variantes markup-only, incl. `Expr::MathIdent`
teoricamente, embora nunca alcance esta arm) já devolvem `Value::None` na
"Fronteira deliberada" — mesmo resultado (`Content::Empty`) do
comportamento antigo, sem regressão.

### 2. `eval_math_callee` ganha braço `Value::Symbol`

`sym.suit.heart` resolve em cascata: `sym` (módulo) → `.suit`
(`Value::Symbol`, grupo de variantes) → `.heart` (aplica modifier). Braço
novo, paridade `eval_field_access` (P765a, `eval/bindings.rs:1552-1563`,
`s.modified(field)`) — duplicado (não reutilizado directamente) porque
`eval_field_access` recursa via `eval_expr(access.target())`, que falha no
mesmo alvo `MathIdent` que motivou `eval_math_callee` existir.

### 3. Achado bónus, não planeado — `#let` em modo math

O mesmo catch-all antigo também descartava `Expr::LetBinding` — `#let x = 5`
dentro de `$...$` nunca mutava o scope (silenciosamente ignorado). Com a
correcção, `eval_expr`'s braço `Expr::LetBinding` executa de facto. Não é
scope creep — é consequência directa e inevitável de generalizar o
catch-all para `eval_expr`, não um objectivo adicional perseguido.

---

## Validação

```bash
$x #sym.suit.heart y$                 → x ♥ y
$sym.suit.heart$                      → ♥
#let hc = sym.suit.heart; $x #hc y$   → x ♥ y
```

Todos confirmados via `mutool trace` (`unicode="♥" glyph="heart"` presente
nos três casos), correspondendo exactamente aos três casos de teste
registados por P772y §3.6.2.

### Não-regressão (P780)

```bash
#let myvar123 = 5; $myvar123$   → "5" (inalterado)
$foobarbaz$                      → Err "unknown variable: foobarbaz" (inalterado)
```

### Suites de teste e lint

```
cargo build --workspace --release   → 0 erros
cargo test --workspace --release    → 4259+647+33+2+29+2 = 4972 passed, 0 failed
                                       (+6 testes novos: p782_field_access_bare_
                                       resolve_simbolo, p782_field_access_via_
                                       hash_resolve_simbolo, p782_hash_ident_
                                       vinculado_a_symbol_resolve, p782_let_
                                       binding_em_math_executa_de_facto, p782_
                                       nao_regride_p780_bare_mathident_
                                       vinculado, p782_nao_regride_p780_undef_
                                       continua_a_errar)
crystalline-lint . --fix-hashes     → 10 ficheiros re-hashed
crystalline-lint .                  → 0 drift (só V7 pré-existente, não
                                       relacionado)
```

Sem regressão nos testes de math existentes (P299-301, P765b, P772w, P772y,
P780) nem em nenhum outro dos 4253 testes pré-existentes.

---

## Critério de fecho do passo

- [x] Mecanismo exacto do vanilla confirmado para `#expr` e field-access
      bare — **confirmado como mecanismos distintos no vanilla**
      (`Expr::MathFieldAccess` dedicado vs `Expr::FieldAccess` genérico via
      Hash), mas **estruturalmente idênticos no cristalino** (mesmo
      `Expr::FieldAccess` para os dois, por decisão de arquitectura do
      lexer math já existente) — não uma suposição, uma consequência
      verificada por leitura de fonte.
- [x] `#expr` splice implementado, reutilizando `value_to_display_content`
      de P780.
- [x] Field-access bare implementado — mesmo braço que `#expr` (consequência
      estrutural confirmada, não assumida às cegas).
- [x] Os três casos de teste de P772y renderizam correctamente (`♥`
      confirmado via `mutool trace` nos três).
- [x] Sem regressão nos casos já corrigidos por P780.
- [x] `cargo test --workspace` verde (4972 passed, 0 failed).
- [x] `crystalline-lint .` zero violações (excepto V7 pré-existente).
- [x] L0 de eval matemático atualizado antes do fecho (`eval.md` §P782).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p782.md` (este
      ficheiro).

---

## Próximo passo

Com este fechado, resta apenas o fallback de fontes matemáticas (débito
antigo de P772w) como item de débito conhecido em aberto. Candidato a
próximo passo dedicado, ou parar aqui para um resumo da série completa
P765a-P782.
