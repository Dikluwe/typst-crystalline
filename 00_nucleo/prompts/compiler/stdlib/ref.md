# Prompt L0 — `rules/stdlib/ref` — `ref(name, supplement: ?)`
Hash do Código: —

**Camada**: L1..L3 · **Alvo**: `01_core/src/compiler/stdlib/ref.rs`

---

## `native_ref`

```rust
pub fn native_ref(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value>
```

Emite `Content::Ref(RefElem { name, supplement })`.

- 1º arg posicional: nome do label (`Str`). Não pode ser vazio.
- Named arg `supplement`: conteúdo prefixado ao número (`Content`). Se omitido,
  `None` → layout usa supplement default do tipo referenciado.
- Erros semânticos contextualizados (nome vazio, arg não-string,
  supplement não-content).

## Tests L1

- `native_ref("sec1")` emite `Content::Ref` com `name == "sec1"` e
  `supplement == None`.
- `native_ref("sec1", supplement: Some("Section "))` preserva o supplement.
- Nome vazio e arg não-string retornam `Err`.

## Resolução numérica P462 (L2/L3)

No layout, `layout_ref` resolve `@name` usando `Introspector`:

1. Consulta `introspector.counter_key_for_label(label)`.
2. Se houver `counter_key`, busca `query_by_label(label)` e formata o número com
   o numbering aplicado ao counter (heading: alphabetical; equation: `(n)`;
   table/figure: plain number do counter).
3. Concatena `supplement` explícito, se houver; senão usa default do tipo
   (`"Fig. "` para figure, `"Table "` para table, nenhum para heading/equation).
4. Se label não existe ou não tem `counter_key`, fallback legacy para figuras
   resolvidas; último fallback renderiza `"?"`.

## Tests L2/L3

- `@intro` num documento com `heading("Intro", label: "intro")` produz
  `"1"` (heading numbering default alfabético).
- `@f1` num documento com figura etiquetada e caption numerado produz
  `"Fig. 1"`.
- `@eq1` com equação numerada produz `"(1)"`.
- `@tbl1` com tabela numerada produz `"Table 1"`.
- Supplement explícito `ref("f1", supplement: "Figura ")` produz `"Figura 1"`.
- Label desconhecido produz `"?"`.
