# Diagnóstico P1140.24 — running matter de página

**Estado:** fechado  
**Medição final:** 2026-08-24T17:09:06-03:00  
**HEAD:** `45b547073d7686cdd5d3e3030c82de3e22ec395f`  
**Árvore:** working tree não commitado

## Resultado

O cristalino passou a transportar e compor `numbering`, `number-align`,
`header`, `header-ascent`, `footer` e `footer-descent` no contrato interno de
página. O novo domínio fica em `entities/page_running.rs` e a composição forma
B em `compiler/layout/page_running.rs`.

| Argumento | Representação | Consumer/prova |
|---|---|---|
| `numbering` | padrão opcional já vigente | numeração simples e composta na camada marginal |
| `number-align` | horizontal + top/bottom | posição física; horizon rejeitado em eval |
| `header` | omitido/Auto/None/Content | Content visual; Content/None suprimem número top |
| `header-ascent` | `Rel<Length>` | ratio contra margem top + parte absoluta |
| `footer` | omitido/Auto/None/Content | Content visual; Content/None suprimem número bottom |
| `footer-descent` | `Rel<Length>` | ratio contra margem bottom + parte absoluta |

Os defaults são center+bottom, header/footer Auto e offsets de 30%. Esse 30%
vem do contrato medido do vanilla ratificado, não de calibração empírica. A
numeração composta preserva um snapshot de alinhamento, margens e offsets por
página enquanto aguarda o total de páginas.

Header, footer e número são guardados em `Page.foreground`, separado de
`Page.items`. Assim permanecem visuais nos exporters, mas não contaminam
`Page::plain_text`; as suítes integrais de query, introspecção e tagging
permaneceram verdes.

## RED → GREEN

A ampliação inicial do contrato produziu RED estrutural: constructors de
`Content::SetPage` sem os cinco novos deltas e patterns não exaustivos falharam
na compilação. Após migração controlada, o teste focado P1140.24 contém cinco
casos verdes: defaults, posição, aceitação de argumentos, rejeição de horizon e
isolamento/supressão por header explícito.

Os testes históricos P538d e P541 inicialmente falharam porque ainda
inspecionavam o número em `Page.items`. Foram atualizados para observar também
a camada marginal, mantendo suas provas de fonte definida e total de páginas.

## Gates

- `cargo test -p typst-core p1140_24`: 5 passados, zero falhas.
- `cargo test -p typst-core -- --test-threads=1`: 5187 passados, zero falhas.
- `cargo test -p typst-infra -- --test-threads=1`: 835 passados, zero falhas.
- `cargo test --workspace -- --test-threads=1`: verde; lotes reportados de
  5187, 835, 53, 2, 55 e 2 testes passados; 3 ignorados.
- `cargo build --workspace`: verde.
- `crystalline-lint .`: exit 0, zero violations e zero prompt drift.
- `git diff --check`: verde.

## Proveniência

No instante final, `git diff HEAD --stat` reportou 117 ficheiros rastreados,
2784 inserções e 599 remoções. O total inclui toda a árvore acumulada e não é
atribuível isoladamente a P1140.24. Novos ficheiros ainda não rastreados também
fazem parte da working tree registrada pela sessão.

## Fronteira seguinte

`page` e `std.page` continuam deliberadamente ausentes. `supplement` e as
referências de página pertencem ao P1140.25; a exposição pública e o rebaseline
final permanecem para a frente posterior.
