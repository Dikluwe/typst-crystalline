# Prompt L0 — `Symbol` — grapheme Unicode nomeado com modifiers e constructor
Hash do Código: 5a1c3363

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

**P1161 — medição anterior à decisão.** No vanilla ratificado `a51e02804`,
`emoji.heart` serializa/renderiza `U+2764 U+FE0F`; `symbol("👩‍💻")`,
`symbol("👍🏽")` e `symbol("🇧🇷")` são aceites, enquanto `symbol("")` e
`symbol("ab")` falham com `variant value must be exactly one grapheme
cluster`. `emoji.heart == symbol("❤️")` é `false` mesmo com codepoints
idênticos, e duas construções runtime
iguais são `true`, e concatenação preserva o cluster inteiro. Fonte ratificada:
`foundations/symbol.rs:51-129,176-196,219-310,339-400`.

Classificação: o cluster, constructor, `repr`, concatenação, modifiers,
igualdade e erros são semântica/morfologia observável. `SymbolInner`,
`Arc<Modified>` e strings estáticas/runtime do vanilla são mecânica e não são
copiados. A igualdade observada não é reduzida ao texto visível: a identidade
e a lista de variants permanecem parte do valor da linguagem.

## 2. Tipo L1

```rust
// 01_core/src/entities/symbol.rs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub value: EcoString,
    pub name: EcoString,
    pub variants: Vec<(EcoString, EcoString)>,
    pub applied: Vec<EcoString>,
}
```

- `value`: valor efectivo após modifiers aplicados; contém **exactamente um
  extended grapheme cluster**, que pode ter múltiplos Unicode scalar values.
- `name`: nome canónico base (vazio para symbols runtime).
- `variants`: lista de variantes `(modifiers_dotted, value)`. Cada valor tem
  exactamente um grapheme cluster; a variante base usa modifiers vazios.
- `applied`: modifiers já aplicados via field access.

`EcoString` é a representação escolhida: preserva o cluster sem I/O, já é
permitida em L1 e clona O(1). Um enum `Scalar | Cluster` é rejeitado porque a
linguagem só distingue “um grapheme válido”, não duas classes semânticas.
Copiar o enum estático/runtime e o `Arc` do vanilla é rejeitado como mecânica
sem ganho observável. Manter `char` mais um sufixo é rejeitado porque permite
estados partidos e obriga cada consumer a reconstruir a morfologia.

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

- `Symbol::new(value, name)` — symbol simples, uma só variante vazia; `value`
  aceita `impl Into<EcoString>` para conservar call sites de um codepoint.
- `Symbol::with_variants(value, name, variants)` — symbol nativo dos módulos
  `sym`/`emoji`.
- `Symbol::runtime(variants)` — symbol criado pelo utilizador via `symbol(...)`.

Os constructors internos recebem valores já validados ou tabelas estáticas e
mantêm a invariante de um grapheme por variante. O constructor de linguagem é
o dono dos diagnósticos para input runtime.

## 5. Modifiers encadeados

`Symbol::modified(modifier: &str) -> Option<Symbol>`:

1. Rejeita symbols simples.
2. Adiciona o modifier a `applied`.
3. Filtra variants que contenham todos os modifiers aplicados.
4. Selecciona a variante com o menor número de modifiers extra.
5. Devolve novo `Symbol` com `value` actualizado sem normalizar nem remover
   variation selectors/ZWJ.

## 6. Field access em `Value::Symbol`

Em `bindings.rs::eval_field_access`, o arm `Value::Symbol(s)` chama `s.modified(field)` e devolve `Value::Symbol`. Erro `unknown symbol modifier` quando `None`.

## 7. Constructor `symbol(...)`

- `eval/mod.rs`: `scope.define("symbol", Value::Type(Type::Symbol))`.
- `closures.rs`: `Value::Type(Type::Symbol)` despacha para `native_symbol(...)`.
- `foundations/cast.rs::native_symbol`: aceita variantes posicionais (string
  base ou array `(modifiers, value)`), valida **um grapheme cluster não vazio**
  preservando a string completa e devolve `Symbol::runtime(variants)`.

## 8. Conversão em markup

`Value::Symbol(s)` em markup converte-se em `Content::Text(s.value.clone())`.
Conversões para string/content e math usam sempre o cluster integral, nunca
`chars().next()`.

## 9. Representação `repr()`

`eval/repr.rs` formata `Value::Symbol` como `symbol({inner})`, onde `inner`
lista as variants compatíveis com os modifiers aplicados, removendo os
modifiers já aplicados. O escape percorre a string completa; VS16 aparece
como `\u{fe0f}` e ZWJ como `\u{200d}` conforme a medição.

**P1163 — forma compacta/multilinha.** Medição em 2026-08-25 contra o vanilla
ratificado `a51e02804` (`foundations/symbol.rs:339-390` e
`foundations/repr.rs:170-223`): a lista resultante passa por
`pretty_array_like(..., false)`. A soma em bytes das peças e separadores cabe
horizontalmente até 50; acima disso, cada variant ocupa uma linha indentada
por dois espaços, com vírgula final. Assim `repr(emoji.heart)` é multilinha,
enquanto `repr(emoji.heart.arrow) == "symbol(\"💘\")"` e symbols runtime
curtos permanecem compactos. Esta forma textual é sintaxe observável; o helper
e a sua localização são mecânica interna (ADR-0107).

## 10. Módulo `sym`

`stdlib/sym.rs` mantém `SYM_SIMPLE` e adiciona `arrow` como symbol com variants. `sym_lookup("arrow.r.filled")` aplica modifiers sequencialmente.

## 11. Scope-out

- Tabela completa de variants do vanilla: apenas `arrow` materializado com subset medido.
- Variantes com deprecation/warnings.
- `Symbol::func()` para math accents callable.
- Normalização Unicode: os codepoints fornecidos são preservados; NFC/NFD não
  é introduzido sem nova medição e decisão.

## 12. Critérios de verificação

```
sym.arrow.r → Value::Symbol com value="→"
sym.arrow.r.filled → Value::Symbol com value="➡"
symbol("🖂", ("stamped", "🖃")).stamped → "🖃"
symbol("♥️") → value U+2665 U+FE0F, sem truncamento
symbol("❤️") → value U+2764 U+FE0F, sem truncamento
symbol("👩‍💻"), symbol("👍🏽"), symbol("🇧🇷") → aceites
symbol(""), symbol("ab") → erro de exactamente um grapheme cluster
repr(symbol(("bold", "α"), ("italic", "α"))) == "symbol((\"bold\", \"α\"), (\"italic\", \"α\"))"
```
