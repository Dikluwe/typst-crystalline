# Prompt L0 — `entities/elements/ref` — `RefElem`
Hash do Código: 8a6135c7

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/ref.rs`
**Origem**: modelo D (ADR-0105), **P462**. Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável** (leaf).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct RefElem {
    pub name: EcoString,
    pub supplement: Option<Content>,
}
```

`Content::Ref { name, supplement }` → `Content::Ref(Arc<RefElem>)`.
Construtores ergonómicos em `Content`:
- `Content::reference(name: impl Into<EcoString>)` — supplement `None`.
- `Content::reference_with_supplement(name, supplement: Option<Content>)`.

A keyword `ref` exigiria raw identifier em cada call-site — fricção
 desnecessária; o módulo e os construtores usam `reference`.

## `impl Element for RefElem`

| método | comportamento |
|---|---|
| `plain_text` | `format!("@{}", self.name)` |
| `is_empty` | default `false` |
| `map_content`/`map_text` | **terminais** (leaf) |
| `get_field`/`element_kind`/`to_payload` | default |

## Notas P462

- `RefElem` não guarda o número resolvido — guarda apenas o nome do label.
- A resolução do número acontece no layout via `Introspector` (oráculo de
  counters).
- `supplement` opcional: se `None`, o layout usa o supplement default do tipo
  referenciado. Se `Some`, o conteúdo é prefixado ao número formatado.
  **Os defaults do cristalino divergem da linguagem — ver bloco P1031 abaixo.**

> **Fonte de paridade (P1031)** — vanilla ratificado (`e0e8ca4d`):
>
> - **Resolução do número no layout via `Introspector`** — a documentação sustenta que a
>   referência resolve contra o estado do documento, não contra o nó:
>   `crates/typst-library/src/model/reference.rs:148-159`, campo `supplement`: *"A supplement
>   for the reference. If the `form` is set to `{"normal"}`: - For references to headings or
>   figures, this is added before the referenced number. […]"* — e o próprio `RefElem` guarda
>   só o `target: Label`. Citação **contextual**: a documentação fala do *número
>   referenciado* sem nomear o mecanismo; que esse número venha do `Introspector` no layout
>   é decisão do cristalino, provada pelas guardas, não pela documentação.
> - **Defaults de supplement** — no vanilla o default é `Smart::Auto`, resolvido para o
>   `LocalName` do elemento (ex.: `impl Refable for Packed<HeadingElem>`,
>   `crates/typst-library/src/model/heading.rs:321-327`), cujos valores são dados literais em
>   `crates/typst-library/translations/en.txt`: `figure = Figure` (l. 1), `table = Table`
>   (l. 2), `equation = Equation` (l. 3), `heading = Section` (l. 5).
>
> **ACHADO CORRIGIDO (P1073) — os supplements por defeito agora têm paridade estrita com a linguagem.**
>
> Medição de 2026-08-13 (vanilla `/usr/local/bin/typst` = `typst 0.15.1 (e0e8ca4d)`;
> cristalino `target/release/typst` da fonte em HEAD `4f64e4e69`, árvore só com edições em
> `00_nucleo/prompts/**`). Documento com `#set text(lang: "en")`,
> `#set heading(numbering: "1.")`, `#set figure(numbering: "1")`,
> `#set math.equation(numbering: "(1)")`, um heading `<h>`, uma equação `<eq>`, uma figura
> `<f>`, e a linha `Ver @h, @eq, @f.`:
>
> | Referência | Vanilla | Cristalino |
> |---|---|---|
> | `@h` (heading) | `Section 1` | `Section 1` ✅ |
> | `@eq` (equação) | `Equation 1` | `(1)` ❌ — sem supplement, e imprime o padrão de numeração em vez do número |
> | `@f` (figura) | `Figure 1` | `Fig. 1` ❌ — abreviatura em vez do `LocalName` |
>
> A redacção anterior deste L0 (*"nenhum para heading/equation"*) estava **errada nos dois
> lados** para o caso heading: a linguagem usa `Section` e o cristalino também já o faz.
> Ficam por corrigir `figure` (`Fig.` → `Figure`) e `equation` (nenhum → `Equation`), que se
> resolvem lendo o supplement da mesma tabela de traduções em vez de constantes fixas.
> Mudança de comportamento por defeito → gate ADR-0127 e passo próprio. **Não implementado
> aqui.**
>
> Nota: com `#set figure(numbering:)` ausente, `@f` no cristalino renderiza **vazio**
> (medido no mesmo passo) — consequência do ACHADO 1 de `compiler/layout_figure.md` (default
> de `figure.numbering` não aplicado), não um defeito separado do `ref`.
## P1140.25 — forma page

`RefElem` inclui `RefForm::{Normal, Page}` com default Normal e preserva a
precedência do supplement explícito. Ver `entities/page_supplement.md`.
