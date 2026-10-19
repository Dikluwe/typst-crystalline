# L0 — entities/content/tests/projections
Hash do Código: 1e8966d8

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/entities/content/tests/projections.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own regression coverage for semantic emptiness, plain-text projection, and named-field observations across content variants. Tests observe public or crate-visible
behavior and must not prescribe private implementation structure, dynamic
dispatch, or reverse layer dependencies.

## Acceptance

Cases retain independent assertions and fixtures, execute as
part of the `Content` test suite, and introduce no productive dependency.


## TS1 — tamanho completo de texto (ratificado pelo humano, 2026-10-17)

### Medição anterior à decisão

`lab/results/construction/diagnostics/text-argumentos-2026-10-17/before.json.gz`, `before-measurement.json.gz`,
`morph-before-measurement.json.gz` e `audit.json` conservam HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status/diffstat,
fontes/L0/binários/argv e o pin9f2b6e8715237cb086899a42873660fe744622e8.
Em named-em, A mede0pt contra24pt; em affine,3pt contra27pt; nested sobre18pt
mede0pt contra36pt. As seis permutações Size/Color/body são aceitas pelo pin
mas rejeitadas pelo Tekt. Set posicional18pt é ignorado (12 contra18pt).
Fonte pinada text/mod.rs:290,303,924–930,1136–1142 e args.rs:178–247
estabelece busca tipada, preferência nomeada, body restante e composição.
style_chain.rs:513–517 e constructor.rs:123 descartam em; collapse em
style_chain.rs:394–395,419–422 aplica top-wins onde tamanho precisa fold.
É tamanho/escopo observável da linguagem; a representação Rust é escolha
interna. Não inferir intenção histórica. Refutar por restauração, Raw,
heading, math, capture/show, dupes/spans, zero/negativos ou métricas divergentes.
A sonda de igualdade refutou a hipótese de comparar os valores declarados:
no pin, text(size:2em)[A] == text(size:24pt)[A] é true. StyledElem::eq no pin
content/mod.rs:779–782 compara child. Esse defeito de igualdade requer sua
própria correção; o tipo completo não é prova de fechamento desse eixo.

### Ratificação

O humano aprovou TS1 e os dois contratos públicos da proposta de manifesto
771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba.
Implementação e testes S1–S16 sob 12 GiB/swap zero; sem release/merge/push.
Resselo somente após implementação e grafo válido. RG1 continua independente.

### Responsabilidade exclusiva adicional

Somente fixtures/adapters de testes: substituir Style::Size(Pt) por Size(Length absoluto), e expectativas de StyleDelta.size por Option<Length>. Conservar os observáveis linguísticos e os contratos funcionais existentes; não trocar geometria correta por snapshots da representação nova.
