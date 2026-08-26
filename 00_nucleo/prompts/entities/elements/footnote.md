# Prompt L0 — `entities/elements/footnote` — `FootnoteElem`
Hash do Código: 793262d3

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/footnote.rs`
**Origem**: modelo D (ADR-0105), **Lote 11 P326** (por largura). Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Locatável desde P1016** — o scope-out de P326
(P295 Fase 1 marker-only, ADR-0054 graded) foi revogado: `to_payload` emite
`ElementPayload::Footnote`, o counter flat `"footnote"` avança uma vez por nota
e `counter(footnote)` resolve, em paridade com o vanilla. Ver
`compiler/introspect.md` §P1016. Contentor — `map_*` recursam no `body`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct FootnoteElem {
    pub body:      Content,           // era Box<Content>
    pub numbering: Option<EcoString>,
}
```

`Content::Footnote { body, numbering }` → `Content::Footnote(Arc<FootnoteElem>)`.
Construtores ergonómicos:
- `Content::footnote(body)` → `numbering: None`.
- `Content::footnote_with_numbering(body, numbering)` (P502).
**Deriva `Hash`** (`Content` tem `impl Hash` manual).
(P502 — `native_footnote` aceita `numbering:` named; uso no marcador de rodapé scope-out per ADR-0054.)

## `impl Element for FootnoteElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1814`) |
| `is_empty` | **default `false`** — `Footnote` tem arm explícito `=> false` no hub (marker `[N]` sempre observable; `content.rs:1660`); **não delega** ao body. Preservar (não override). |
| `map_content` | **recursivo** no `body` (`content.rs:2299`) |
| `map_text` | **recursivo** no `body` (`content.rs:2543`) |
| `get_field`/`element_kind` | default |
| `to_payload` | **P1016 — override**: `Some(ElementPayload::Footnote { counter_update: CounterUpdate::Step })`. Toda a nota conta (sem gate de `caption`/`numbering`, ao contrário de `TableElem`) — **com uma excepção na linguagem, ver bloco abaixo**. Torna `Content::Footnote` locatable — ver `compiler/introspect.md` §P1016. |

> **Fonte de paridade (P1031)** — vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/model/footnote.rs`:
>
> - **`counter(footnote)` resolve** — literal, doc comment de `FootnoteEntry.note`
>   (`footnote.rs:224-231`), publicado em
>   `typst.app/docs/reference/model/footnote/#definitions-entry`:
>   *"The footnote for this entry. Its location can be used to determine the footnote
>   counter state."*, com o exemplo `counter(footnote).display(at: loc, "1: ")`. ✅ sustenta
>   a afirmação de P1016.
> - **Chave do contador** — no vanilla é o próprio elemento (`Counter::of(FootnoteElem::ELEM)`,
>   `footnote.rs:148` e `:313`), não uma string. O cristalino usa uma chave flat `"footnote"`.
>   É **mecânica** (representação da chave), diverge de propósito desde que a superfície
>   `counter(footnote)` continue a resolver — ADR-0107.
> - **"Toda a nota conta" tem uma excepção documentada.** `footnote.rs:174-178`:
>   ```rust
>   impl Count for Packed<FootnoteElem> {
>       fn update(&self) -> Option<CounterUpdate> {
>           (!self.is_ref()).then(|| CounterUpdate::Step(NonZeroUsize::ONE))
>       }
>   }
>   ```
>   Uma nota que é **referência** a outra (`FootnoteBody::Reference(Label)`,
>   `footnote.rs:114-124`) **não** avança o contador — reutiliza o número da nota original.
>
> **ACHADO ESCALADO — a forma de referência não existe no cristalino.** Medição de
> 2026-08-13 (vanilla `/usr/local/bin/typst` = `typst 0.15.1 (e0e8ca4d)`; cristalino
> `target/release/typst` da fonte em HEAD `4f64e4e69`, árvore só com edições em
> `00_nucleo/prompts/**`). Documento:
> `A#footnote[Um] <fn> B#footnote(<fn>) C#footnote[Dois]`.
>
> | Binário | Resultado |
> |---|---|
> | Vanilla | `A1 B1 C2` + entradas `1 Um` / `2 Dois` — a referência reutiliza o número 1 |
> | Cristalino | **erro**: `footnote() espera content ou string, recebeu label` |
>
> É superfície de linguagem (morfologia do argumento, ADR-0107) → paridade. Faz parte do
> mesmo padrão sistémico de `cite(<key>)` (`entities/elements/cite.md` §P1031) e
> `link(<label>)` (`compiler/layout/link.md` §P1031): **argumentos de tipo label são
> rejeitados onde a linguagem os exige**. Gate ADR-0127 (contrato público + comportamento),
> passo próprio. **Não implementado aqui.**

## `eq`

`#[derive(PartialEq)]` compara `body` (paridade `content.rs:1958`).
