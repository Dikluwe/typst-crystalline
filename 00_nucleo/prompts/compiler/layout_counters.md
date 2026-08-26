# L0 — Layout: Contadores e Numeração
Hash do Código: bf43f367

## Módulo
`01_core/src/compiler/layout/counters.rs`

## Propósito
Encapsula os braços do Layouter que alteram ou exibem o estado de
contadores: `SetHeadingNumbering`, `CounterUpdate`, `CounterDisplay`.
Funções chamadas por `layout.rs` (orquestrador).

## Regras de negócio
- `SetHeadingNumbering { active }` → muta `self.counter.numbering_active`.
- `CounterUpdate { key, action }` → delega em `step_flat`/`update_flat`/
  `step_hierarchical`.
- `CounterDisplay { kind }` → lê o estado actual e gera `Content::text`.
- Nenhuma destas funções gera geometria de página directamente.

> **Fonte de paridade (P1031)** — doc comments `#[func]` do vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/introspection/counter.rs`, publicados em
> `typst.app/docs/reference/introspection/counter/`:
>
> - **Contadores têm níveis** (sustenta `step_hierarchical` vs `step_flat`) —
>   `counter.rs:38-41`: *"This function returns an array: Counters can have multiple levels
>   (in the case of headings for sections, subsections, and so on), and each item in the
>   array corresponds to one level."*
> - **`step`** — `counter.rs:481-497`: *"Increases the value of the counter by one. The
>   update will be in effect at the position where the returned content is inserted into
>   the document. If you don't put the output into the document, nothing happens!"*, com o
>   parâmetro `level` documentado como *"The depth at which to step the counter. Defaults to
>   `{1}`."* — sustenta o par `CounterUpdate{action}` → `step_*`, e o facto de a actualização
>   ser um `Content` posicional em vez de efeito imediato.
> - **`update`** — `counter.rs:501-512`: *"Updates the value of the counter. Just like with
>   `step`, the update only occurs if you put the resulting content into the document."*
> - **`display`** — `counter.rs:374-381`: *"Displays the value of the counter. You can
>   provide both a custom numbering and a custom location. Both default to `{auto}`,
>   selecting sensible defaults (the numbering of the counted element and the current
>   location, respectively). Returns the formatted output."* — sustenta `CounterDisplay`
>   produzir texto formatado a partir do estado corrente.
>
> **Natureza**: literal para a existência e semântica de `step`/`update`/`display` e para a
> natureza multi-nível dos contadores. A repartição interna em
> `step_flat`/`update_flat`/`step_hierarchical` e o campo `numbering_active` são **mecânica
> do cristalino** — não têm correspondente citável e divergem de propósito (ADR-0107).
>
> **Achado relacionado, escalado no relatório do P1031**: a numeração hierárquica de
> headings no corpo do documento não usa todos os níveis. Medido em 2026-08-13 com
> `#set heading(numbering: "1.")` + `= Um` + `== Dois` + `=== Tres`: o vanilla
> (`typst 0.15.1 (e0e8ca4d)`) rende `1.` / `1.1.` / `1.1.1.`; o cristalino
> (`target/release/typst`, fonte em HEAD `4f64e4e69`) rende `1.` / `1.` / `1.`. Note-se que
> o **outline** do mesmo documento mostra `1.` / `1.1.` correctamente nos dois binários —
> logo os valores de contador existem e o defeito está no caminho de render do heading, não
> na contagem. Gate ADR-0127, passo próprio. **Não implementado aqui.**

## Critérios de verificação
- `CounterUpdate(Update(5))` → `counter.get_flat("equation") == 5`.
- `CounterDisplay("heading")` → texto contém o número formatado.
