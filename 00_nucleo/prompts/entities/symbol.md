# Prompt L0 — `Symbol` — símbolo Unicode nomeado com modifiers e constructor
Hash do Código: 7923dec9

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/symbol.rs`, `01_core/src/entities/value.rs`, `01_core/src/compiler/eval/bindings.rs`, `01_core/src/compiler/eval/closures.rs`, `01_core/src/compiler/eval/repr.rs`, `01_core/src/compiler/stdlib/sym.rs`, `01_core/src/compiler/stdlib/foundations.rs`
**Origem**: Passo 471 — `Value::Symbol` subset minimal (S); Passo 765a — modifiers encadeados e constructor.
**ADRs**: ADR-0017 (portão aberto P395), ADR-0107 (paridade linguagem), ADR-0108 (medir antes de decidir), ADR-0029 (pureza L1).

---

## 1. Contexto

O Typst vanilla expõe `Symbol` como tipo de runtime para caracteres simbólicos acessíveis via notação de ponto (`sym.arrow.r`, `sym.eq.not`) e via constructor `symbol(...)`.

> **Fonte de paridade (P1031)** — **citação literal** do doc comment `#[ty]` do vanilla
> ratificado (`e0e8ca4d`), `crates/typst-library/src/foundations/symbol.rs:19-37`, publicado
> em `typst.app/docs/reference/foundations/symbol/`:
>
> *"A Unicode symbol. Typst defines common symbols so that they can easily be written with
> standard keyboards. The symbols are defined in modules, from which they can be accessed
> using **field access notation**: - General symbols are defined in the `sym` module and are
> accessible without the `sym.` prefix in math mode. - Emoji are defined in the `emoji`
> module. Moreover, **you can define custom symbols with this type's constructor
> function**."*
>
> ```example
> #sym.arrow.r \
> #sym.gt.eq.not \
> $gt.eq.not$ \
> #emoji.face.halo
> ```
>
> Confirma as três afirmações do contexto: tipo de runtime, acesso por notação de ponto
> (com os exemplos `sym.arrow.r` e a forma composta), e o construtor. Acrescenta duas
> precisões que este L0 não regista: (a) em **modo math** os símbolos de `sym` são
> acessíveis **sem o prefixo `sym.`**; (b) existe um segundo módulo, `emoji`. Ambas são
> **lacunas documentadas** do subset minimal de P471.

Passo 471 implementou o subset minimal: `Symbol` como `{ ch, name }`, módulo `sym` com nomes simples/compostos planos, e `Value::Symbol`.

Passo 765a acrescenta:
1. Modifiers encadeados via field access (`sym.arrow.r.filled`).
2. Constructor `symbol(...)` para symbols runtime.
3. `repr()` de symbols com variants.

## 2. Tipo L1

```rust
// 01_core/src/entities/symbol.rs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub ch: char,
    pub name: EcoString,
    pub variants: Vec<(EcoString, char)>,
    pub applied: Vec<EcoString>,
}
```

- `ch`: caractere efectivo após modifiers aplicados.
- `name`: nome canónico base (vazio para symbols runtime).
- `variants`: lista de variantes `(modifiers_dotted, char)`. A variante base usa modifiers vazios.
- `applied`: modifiers já aplicados via field access.

## 3. Variant `Value::Symbol`

```rust
// 01_core/src/entities/value.rs
Symbol(crate::entities::symbol::Symbol),
```

Actualizações:
- `type_name()` → `"symbol"`.
- `From<Symbol> for Value`.
- `Hash` via `format!("{:?}", self)`.

## 4. Construtores

- `Symbol::new(ch, name)` — symbol simples, uma só variante vazia.
- `Symbol::with_variants(ch, name, variants)` — symbol nativo do módulo `sym`.
- `Symbol::runtime(variants)` — symbol criado pelo utilizador via `symbol(...)`.

## 5. Modifiers encadeados

`Symbol::modified(modifier: &str) -> Option<Symbol>`:

1. Rejeita symbols simples.
2. Adiciona o modifier a `applied`.
3. Filtra variants que contenham todos os modifiers aplicados.
4. Selecciona a variante com o menor número de modifiers extra.
5. Devolve novo `Symbol` com `ch` actualizado.

## 6. Field access em `Value::Symbol`

Em `bindings.rs::eval_field_access`, o arm `Value::Symbol(s)` chama `s.modified(field)` e devolve `Value::Symbol`. Erro `unknown symbol modifier` quando `None`.

## 7. Constructor `symbol(...)`

- `eval/mod.rs`: `scope.define("symbol", Value::Type(Type::Symbol))`.
- `closures.rs`: `Value::Type(Type::Symbol)` despacha para `native_symbol(...)`.
- `foundations.rs::native_symbol`: aceita variantes posicionais (string base ou array `(modifiers, char)`), valida grapheme único, devolve `Symbol::runtime(variants)`.

## 8. Conversão em markup

`Value::Symbol(s)` em markup converte-se em `Content::Text(EcoString::from(s.ch))`.

## 9. Representação `repr()`

`eval/repr.rs` formata `Value::Symbol` como `symbol({inner})`, onde `inner` lista as variants compatíveis com os modifiers aplicados, removendo os modifiers já aplicados.

## 10. Módulo `sym`

`stdlib/sym.rs` mantém `SYM_SIMPLE` e adiciona `arrow` como symbol com variants. `sym_lookup("arrow.r.filled")` aplica modifiers sequencialmente.

## 11. Scope-out

- Tabela completa de variants do vanilla: apenas `arrow` materializado com subset medido.
- Variantes com deprecation/warnings.
- `Symbol::func()` para math accents callable.

## 12. Critérios de verificação

```
sym.arrow.r → Value::Symbol com ch='→'
sym.arrow.r.filled → Value::Symbol com ch='➡'
symbol("🖂", ("stamped", "🖃")).stamped → "🖃"
repr(symbol(("bold", "α"), ("italic", "α"))) == "symbol((\"bold\", \"α\"), (\"italic\", \"α\"))"
```
