# Análise de degradação metodológica — relatórios e ADRs

Trabalho retrospectivo solicitado em `analise-degradacao-instrucao-claude-code.md`. Mapa quantitativo de cada relatório de passo (`typst-passo-*-relatorio.md`) e cada ADR (`typst-adr-*.md`), nos termos definidos pelo enunciado: tamanho dos relatórios, frequência de termos metodológicos, contagem de identificadores `N=X` e de menções a "cumulativo", "consolidado", "cross-domínio"/"cross-feature", "aplicação concreta/cumulativa/isolada" e "reservas" de identificadores futuros. Para os ADRs: data, status, dimensão, classificação técnica/meta/limiar.

Universo:

- 91 relatórios `typst-passo-*-relatorio.md` em `00_nucleo/materialization/` (P98 → P160a).
- 65 ficheiros `typst-adr-*.md` em `00_nucleo/adr/` (ADR-0001 → ADR-0066, com saltos em 0056 e 0063 e variante 0026-R1).
- Período coberto: 2026-04-23 13:07 → 2026-04-27 22:18, ou 4 dias e 9 horas.

Critério de classificação dos ADRs (§"Como classificar ADRs" do enunciado):

- **técnica**: decisão sobre código, tipos, pipeline, paridade vanilla, dependências, performance, estrutura do repositório.
- **meta**: decisão sobre o processo de escrever passos, formalização de padrões metodológicos.
- **limiar**: ambígua entre as duas — listada para escrutínio separado.

---

### Secção 1 — Tabela de passos

Ordem cronológica por data git de criação. Universo: 91 relatórios `typst-passo-*-relatorio.md` em `00_nucleo/materialization/`.

Colunas: data | passo | chars | linhas | ADRs únicas | ADRs total | padrão/subpadrão/patamar/limiar | `N=X` | cumulativ* | consolidad*/consolida* | cross-domínio/cross-feature | aplicação concreta/cumulativa/isolada | reservas de identificadores.

| data | passo | chars | linhas | ADR.u | ADR.t | padr. | N=X | cum. | cons. | cross | aplic. | resv. |
|------|-------|------:|-------:|------:|------:|------:|----:|-----:|------:|------:|-------:|------:|
| 2026-04-23 | 98 | 8073 | 267 | 1 | 7 | 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 99 | 8716 | 260 | 7 | 20 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 100 | 9896 | 304 | 4 | 10 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 101 | 9535 | 298 | 1 | 1 | 0 | 0 | 0 | 4 | 0 | 0 | 0 |
| 2026-04-23 | 102 | 8136 | 235 | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 103 | 9159 | 266 | 3 | 7 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 104 | 9270 | 302 | 5 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 105 | 9253 | 230 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 106 | 8619 | 269 | 3 | 7 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 107 | 11063 | 324 | 4 | 13 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 108 | 6875 | 189 | 2 | 3 | 1 | 0 | 0 | 1 | 0 | 0 | 0 |
| 2026-04-23 | 109 | 10597 | 298 | 4 | 9 | 4 | 0 | 0 | 1 | 0 | 0 | 0 |
| 2026-04-23 | 110 | 5793 | 180 | 2 | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 111 | 9633 | 295 | 4 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 112 | 6565 | 183 | 2 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 113 | 9665 | 304 | 5 | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 114 | 6358 | 196 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 115 | 8824 | 307 | 4 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 1 |
| 2026-04-23 | 116 | 10136 | 306 | 3 | 6 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 117 | 10218 | 296 | 6 | 10 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 118 | 7200 | 201 | 1 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 119 | 10260 | 300 | 5 | 14 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 120 | 9766 | 333 | 2 | 7 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 121 | 9223 | 286 | 1 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-23 | 122 | 10538 | 333 | 1 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 123 | 8836 | 288 | 1 | 3 | 0 | 0 | 0 | 1 | 0 | 0 | 0 |
| 2026-04-24 | 124 | 7075 | 227 | 3 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 125 | 8173 | 238 | 4 | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 126 | 9200 | 296 | 3 | 6 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 127 | 8423 | 276 | 3 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 128 | 8617 | 282 | 3 | 7 | 0 | 0 | 0 | 2 | 0 | 0 | 0 |
| 2026-04-24 | 129 | 10118 | 309 | 5 | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 130 | 8988 | 280 | 2 | 3 | 0 | 0 | 1 | 1 | 0 | 0 | 0 |
| 2026-04-24 | 131a | 7814 | 215 | 7 | 19 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 131b | 11575 | 349 | 6 | 18 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 132a | 8039 | 238 | 6 | 18 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 132b | 12834 | 375 | 6 | 13 | 2 | 0 | 1 | 10 | 0 | 0 | 0 |
| 2026-04-24 | 133 | 10330 | 319 | 2 | 4 | 1 | 0 | 1 | 1 | 0 | 0 | 0 |
| 2026-04-24 | 134 | 11482 | 370 | 6 | 13 | 0 | 0 | 0 | 1 | 0 | 0 | 0 |
| 2026-04-24 | 135 | 10246 | 311 | 4 | 18 | 3 | 0 | 0 | 1 | 0 | 0 | 0 |
| 2026-04-24 | 136 | 11258 | 354 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 137 | 10437 | 359 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 138 | 11578 | 359 | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 139 | 12193 | 397 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 140a | 9198 | 298 | 8 | 32 | 3 | 0 | 2 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 140b | 10740 | 281 | 6 | 15 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 141 | 10611 | 293 | 5 | 19 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 143 | 16387 | 388 | 26 | 51 | 0 | 0 | 0 | 0 | 0 | 1 | 0 |
| 2026-04-24 | 145 | 15416 | 465 | 20 | 58 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 144 | 13553 | 352 | 6 | 22 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 146 | 15537 | 425 | 4 | 14 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 147 | 11894 | 299 | 4 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-24 | 148 | 11496 | 290 | 6 | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-25 | 149 | 12301 | 326 | 6 | 33 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-25 | 150 | 13966 | 381 | 3 | 3 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-25 | 151 | 11048 | 315 | 2 | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-25 | 152 | 7979 | 219 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-25 | 153 | 13330 | 355 | 4 | 11 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-25 | 154a | 11212 | 321 | 5 | 23 | 1 | 0 | 1 | 0 | 0 | 0 | 0 |
| 2026-04-25 | 154b | 10033 | 231 | 2 | 9 | 3 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-25 | 155 | 13318 | 323 | 4 | 10 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-25 | 156a | 19025 | 466 | 2 | 2 | 6 | 1 | 0 | 0 | 0 | 0 | 0 |
| 2026-04-25 | 156b | 23527 | 580 | 8 | 84 | 10 | 0 | 0 | 0 | 0 | 0 | 2 |
| 2026-04-25 | 156c | 24874 | 694 | 6 | 24 | 2 | 0 | 5 | 0 | 0 | 1 | 0 |
| 2026-04-25 | 156d | 20164 | 572 | 5 | 23 | 3 | 1 | 4 | 0 | 0 | 0 | 0 |
| 2026-04-25 | 156e | 22805 | 653 | 3 | 17 | 3 | 4 | 6 | 0 | 0 | 0 | 0 |
| 2026-04-25 | 156f | 25039 | 688 | 5 | 17 | 8 | 2 | 2 | 0 | 0 | 0 | 0 |
| 2026-04-25 | 156g | 24497 | 680 | 4 | 17 | 10 | 1 | 3 | 0 | 0 | 0 | 0 |
| 2026-04-25 | 156h | 19687 | 567 | 4 | 17 | 7 | 2 | 2 | 0 | 0 | 0 | 0 |
| 2026-04-26 | 156i | 28473 | 832 | 5 | 29 | 10 | 11 | 19 | 6 | 0 | 0 | 0 |
| 2026-04-26 | 156j | 12608 | 304 | 4 | 12 | 18 | 20 | 8 | 4 | 0 | 0 | 0 |
| 2026-04-26 | 156k | 11939 | 259 | 7 | 54 | 15 | 20 | 1 | 2 | 0 | 0 | 0 |
| 2026-04-26 | 156l | 15580 | 337 | 5 | 29 | 12 | 23 | 8 | 1 | 0 | 7 | 0 |
| 2026-04-26 | 157 | 13951 | 325 | 6 | 33 | 11 | 24 | 6 | 1 | 0 | 8 | 0 |
| 2026-04-26 | 157a | 16269 | 386 | 9 | 27 | 14 | 18 | 9 | 1 | 10 | 2 | 0 |
| 2026-04-26 | 157b | 17280 | 408 | 8 | 38 | 18 | 12 | 11 | 0 | 10 | 5 | 0 |
| 2026-04-26 | 157c | 20027 | 436 | 9 | 51 | 18 | 19 | 9 | 1 | 15 | 1 | 1 |
| 2026-04-27 | 158 | 14388 | 341 | 8 | 35 | 8 | 14 | 5 | 0 | 3 | 8 | 0 |
| 2026-04-27 | 158a | 15256 | 351 | 9 | 26 | 12 | 6 | 8 | 1 | 5 | 1 | 0 |
| 2026-04-27 | 159 | 14793 | 336 | 9 | 43 | 10 | 11 | 4 | 0 | 3 | 4 | 0 |
| 2026-04-27 | 159a | 20748 | 478 | 9 | 40 | 31 | 15 | 9 | 0 | 8 | 5 | 0 |
| 2026-04-27 | 159b | 18395 | 429 | 8 | 45 | 12 | 9 | 4 | 1 | 0 | 3 | 0 |
| 2026-04-27 | 158b | 11413 | 250 | 4 | 11 | 14 | 16 | 4 | 3 | 6 | 1 | 0 |
| 2026-04-27 | 159c | 13901 | 307 | 7 | 19 | 15 | 30 | 4 | 3 | 8 | 2 | 0 |
| 2026-04-27 | 159d | 13846 | 301 | 6 | 18 | 10 | 25 | 5 | 2 | 2 | 3 | 0 |
| 2026-04-27 | 158c | 13286 | 289 | 7 | 21 | 14 | 32 | 4 | 2 | 5 | 1 | 0 |
| 2026-04-27 | 159f | 15186 | 329 | 7 | 17 | 23 | 36 | 4 | 3 | 2 | 1 | 0 |
| 2026-04-27 | 159e | 14301 | 311 | 7 | 17 | 25 | 41 | 10 | 2 | 2 | 1 | 1 |
| 2026-04-27 | 159g | 16526 | 350 | 7 | 19 | 39 | 49 | 12 | 2 | 2 | 1 | 0 |
| 2026-04-27 | 160 | 13529 | 312 | 7 | 49 | 12 | 23 | 6 | 3 | 9 | 3 | 0 |
| 2026-04-27 | 160a | 13001 | 290 | 12 | 54 | 23 | 35 | 10 | 4 | 2 | 5 | 0 |

### Secção 2 — Tabela de ADRs

Universo: 65 ficheiros `typst-adr-*.md` em `00_nucleo/adr/`. Categoria por critério em §0 do enunciado.

| data | ADR | linhas | status | categoria | título |
|------|-----|-------:|--------|-----------|--------|
| 2026-03-22 | 0001-estrategia-migracao | 290 | `IMPLEMENTADO` | técnica | ADR-0001: Estratégia de Migração do Typst para a Arquitetura Cristalina |
| 2026-03-22 | 0002-hierarquia-contencao | 177 | `IDEIA` | técnica | ADR-0002: Hierarquia de Contenção como Mecanismo de Layout |
| 2026-03-22 | 0003-comemo-contencao | 154 | `IDEIA` | técnica | ADR-0003: comemo e Hierarquia de Contenção — Coexistência |
| 2026-03-22 | 0004-passo1-descobertas | 198 | `IMPLEMENTADO` | técnica | ADR-0004: Descobertas do Passo 1 — FileId, ecow, V14 com self:: |
| 2026-03-23 | 0005-packagespec-world | 337 | `PROPOSTO` | técnica | ADR-0005: PackageSpec (DTO pattern) e World trait |
| 2026-03-23 | 0006-typst-timing | 113 | `PROPOSTO` | técnica | ADR-0006: Remoção de `typst_timing` de L1 |
| 2026-03-23 | 0007-rustc-hash | 85 | `REVOGADO` | técnica | ADR-0007: Substituição de `rustc_hash` por `std::collections` |
| 2026-03-23 | 0008-defer-inline | 113 | `PROPOSTO` | técnica | ADR-0008: Inlining de `typst_utils::defer!` em L1 |
| 2026-03-23 | 0009-math-class | 148 | `PROPOSTO` | técnica | ADR-0009: `default_math_class` → `01_core/entities/math_class.rs` |
| 2026-03-23 | 0010-unicode-ident | 121 | `PROPOSTO` | técnica | ADR-0010: `unicode_ident` → `[l1_allowed_external]` |
| 2026-03-23 | 0011-unicode-math-class | 113 | `PROPOSTO` | técnica | ADR-0011: `unicode_math_class` → `[l1_allowed_external]` |
| 2026-03-23 | 0012-unicode-script | 90 | `PROPOSTO` | técnica | ADR-0012: `unicode_script` → `[l1_allowed_external]` |
| 2026-03-23 | 0013-unicode-segmentation | 114 | `PROPOSTO` | técnica | ADR-0013: `unicode_segmentation` → `[l1_allowed_external]` |
| 2026-03-23 | 0014-unscanny | 151 | `PROPOSTO` | técnica | ADR-0014: `unscanny` → `01_core/rules/lexer/scanner.rs` (inline) |
| 2026-03-23 | 0015-ecow | 144 | `PROPOSTO` | técnica | ADR-0015: `ecow` removido do parser — `String`/`SyntaxText` interno |
| 2026-03-26 | 0016-lazyhash | 81 | `IMPLEMENTADO` | técnica | ADR-0016: `typst_utils::LazyHash` → removido de L1 |
| 2026-03-26 | 0017-adiamento-eval-typst-library | 73 | `IMPLEMENTADO` | técnica | ADR-0017: Adiamento de eval() e estratégia typst-library |
| 2026-03-26 | 0018-rustc-hash | 134 | `EM VIGOR` | técnica | ADR-0018: `rustc_hash` → `[l1_allowed_external]` — revoga ADR-0007 |
| 2026-03-27 | 0019-ttf-rustybuzz | 154 | `IMPLEMENTADO` | técnica | ADR-0019: `ttf-parser` e `rustybuzz` → L3 |
| 2026-03-27 | 0020-fontdb | 87 | `ADIADO` | técnica | ADR-0020: `fontdb` → L3 (adiada) |
| 2026-03-28 | 0021-datetime | 137 | `IMPLEMENTADO` | técnica | ADR-0021: `time` crate → `[l1_allowed_external]` e `Datetime` real |
| 2026-03-28 | 0022-fontbook | 103 | `IMPLEMENTADO` | técnica | ADR-0022: `FontBook` real em L3 |
| 2026-03-28 | 0023-indexmap | 93 | `IMPLEMENTADO` | técnica | ADR-0023: `indexmap` → `[l1_allowed_external]` |
| 2026-03-28 | 0024-ecow-value | 121 | `IMPLEMENTADO` | técnica | ADR-0024: `ecow` → `[l1_allowed_external]` para `Value::Str` |
| 2026-03-28 | 0025-int-eq-float | 108 | `IMPLEMENTADO` | técnica | ADR-0025: `Int == Float` — desvio de `PartialEq` vs semântica do original |
| 2026-03-28 | 0026-content-divergencia | 108 | `IMPLEMENTADO` | técnica | ADR-0026: Content cristalino — divergência intencional do original |
| 2026-03-29 | 0026-R1-content-arc | 118 | `IMPLEMENTADO` | técnica | ADR-0026 (Revisão): Content cristalino — actualização de ADR-0026 |
| 2026-03-29 | 0027-cidfont-subsetting | 61 | `IMPLEMENTADO` | técnica | ADR-0027: CIDFont com subsetting via ttf-parser |
| 2026-03-29 | 0028-typographic-types | 52 | `REVOGADO` | técnica | ADR-0028: Representação simplificada dos tipos tipográficos em Value |
| 2026-03-29 | 0029-pureza-fisica-revoga-adr-0028 | 243 | `EM VIGOR` | técnica | ADR-0029: Definição física de pureza em L1 — revoga ADR-0028 |
| 2026-03-29 | 0030-performance-dominio-l1 | 175 | `EM VIGOR` | técnica | ADR-0030: Gestão eficiente de RAM é domínio de L1 — alinhamento filosófico |
| 2026-03-29 | 0031-early-hashing-source | 172 | `IMPLEMENTADO` | técnica | ADR-0031: Early hashing em Source — complementa ADR-0016 |
| 2026-04-22 | 0032-unsafe-em-l1 | 206 | `EM VIGOR` | técnica | ADR-0032: Política de `unsafe` em L1 |
| 2026-04-22 | 0033-paridade-funcional-vanilla | 241 | `EM VIGOR` | técnica | ADR-0033: Paridade funcional com vanilla como invariante arquitectural |
| 2026-04-22 | 0034-diagnostico-tipos-vanilla | 171 | `EM VIGOR` | técnica | ADR-0034: Diagnóstico obrigatório antes de materializar tipo do vanilla |
| 2026-04-22 | 0035-ecovec-autorizacao | 147 | `EM VIGOR` | técnica | ADR-0035: `ecow::EcoVec` autorizado em L1 |
| 2026-04-22 | 0036-atomizacao-progressiva | 210 | `EM VIGOR` | técnica | ADR-0036: Atomização progressiva — estado partilhado como dívida |
| 2026-04-22 | 0037-coesao-por-dominio | 409 | `EM VIGOR` | técnica | ADR-0037: Coesão por domínio — ficheiros limitados a uma responsabilidade clara |
| 2026-04-23 | 0038-sistema-estilos-l1 | 345 | `EM VIGOR` | técnica | ADR-0038: Sistema de estilos em L1 (`Style`, `Styles`, `StyleChain`) |
| 2026-04-23 | 0039-frameitem-style | 176 | `EM VIGOR` | técnica | ADR-0039: Forma de estilo no `FrameItem::Text` |
| 2026-04-23 | 0040-activacao-set-rule | 163 | `EM VIGOR` | técnica | ADR-0040: Activação de `#set` em eval |
| 2026-04-23 | 0041-activacao-show-rule | 168 | `EM VIGOR` | técnica | ADR-0041: Activação de `#show` — heading, strong, emph |
| 2026-04-23 | 0042-sink-materializado | 194 | `EM VIGOR` | técnica | ADR-0042: `Sink` materializado em L1 |
| 2026-04-23 | 0043-canal-sink-saida | 156 | `EM VIGOR` | técnica | ADR-0043: Canal de saída do `Sink` — `TrackedMut` no caller, formatação em L3 |
| 2026-04-23 | 0044-engine-agregador | 239 | `EM VIGOR` | técnica | ADR-0044: `Engine<'a>` como agregador de estado de eval em L1 |
| 2026-04-23 | 0045-formato-diagnosticos | 201 | `EM VIGOR` | técnica | ADR-0045: Formato de diagnósticos: resolução em L1, formatação em L3 |
| 2026-04-23 | 0046-cli-minima | 206 | `EM VIGOR` | técnica | ADR-0046: CLI mínima em L4 (compile com diagnostics) |
| 2026-04-23 | 0047-cli-clap | 192 | `EM VIGOR` | técnica | ADR-0047: Argparsing com `clap` na CLI |
| 2026-04-23 | 0048-diagnosticos-com-cor | 236 | `EM VIGOR` | técnica | ADR-0048: Cores ANSI nos diagnósticos (L3 formata com `bool`, L4 decide) |
| 2026-04-23 | 0049-cli-em-l2 | 190 | `EM VIGOR` | técnica | ADR-0049: CLI vive em L2 (correcção de ADRs 0046/0047/0048) |
| 2026-04-23 | 0050-formatter-em-l2 | 207 | `EM VIGOR` | técnica | ADR-0050: Formatter de diagnósticos em L2 (completa ADR-0049) |
| 2026-04-23 | 0051-flags-funcionais | 255 | `EM VIGOR` | técnica | ADR-0051: Flags funcionais em L2 — pattern e primeira flag (`-o`) |
| 2026-04-24 | 0052-lang-tipo-semantico | 186 | `IMPLEMENTADO` | técnica | ADR-0052: Lang como tipo semântico em L1 |
| 2026-04-24 | 0053-font-tipo-composto | 193 | `IMPLEMENTADO` | técnica | ADR-0053: Font como tipo composto em L1 |
| 2026-04-24 | 0054-criterio-fecho-debt-1 | 140 | `EM VIGOR` | limiar | ADR-0054: Critério de fecho de DEBT-1 inclui consumo integral |
| 2026-04-24 | 0055-font-consumer-cidfont | 217 | `IMPLEMENTADO` | técnica | ADR-0055: Font consumer via pipeline CIDFont existente |
| 2026-04-24 | 0057-lang-hyphenation | 193 | `IMPLEMENTADO` | técnica | ADR-0057: Lang hyphenation em L1 via crate `hypher` |
| 2026-04-25 | 0058-value-type-simplificado | 144 | `EM VIGOR` | técnica | ADR-0058: Tipo simplificado — `type()` devolve `Value::Str` em vez de `Value::Type(Type)` |
| 2026-04-25 | 0059-args-tipo-separado | 159 | `EM VIGOR` | técnica | ADR-0059: `Args` como tipo separado, não-variant de `Value` |
| 2026-04-25 | 0060-model-structural-roadmap | 709 | `IMPLEMENTADO` (Fase 1 fechada; Fas | técnica | ADR-0060: Model (structural) roadmap — Fase 1 + Fase 2 + Fase 3 |
| 2026-04-25 | 0061-layout-fase-x-roadmap | 1074 | `PROPOSTO` | técnica | ADR-0061: Layout Fase X — page model + multi-column + footnote area roadmap |
| 2026-04-26 | 0064-smart-para-option-default | 263 | `EM VIGOR` | meta | ADR-0064: Tradução `Smart<T>` vanilla → `Option<T>`/default cristalino |
| 2026-04-26 | 0065-inventariar-primeiro | 271 | `EM VIGOR` | meta | ADR-0065: Inventariar primeiro — sub-passo `.1` para qualquer decisão arquitectural não-tr |
| 2026-04-27 | 0062-hayagriva-bibliography-parsing | 223 | `PROPOSTO` | técnica | ADR-0062: Autorização de crate `hayagriva` para bibliography + cite (CSL parsing) |
| 2026-04-27 | 0066-introspection-runtime-adiada | 293 | `PROPOSTO` | limiar | ADR-0066: Introspection runtime — promoção da reserva conceptual a ficheiro PROPOSTO |

### Secção 3 — Curvas de evolução

Métrica calculada como média aritmética sobre os primeiros 20 e os últimos 20 relatórios da sequência cronológica de 91 (universo é P98–P160a). Ponto de viragem definido conforme o enunciado: primeiro índice `i` em que três valores consecutivos `v[i-2], v[i-1], v[i]` são todos superiores ao dobro da mediana histórica até esse índice. Se a mediana histórica é zero, o limiar usado é 1 (qualquer valor não-zero conta como "superior").

| métrica | média 20 primeiros | média 20 últimos | mediana global | máximo (passo) | viragem |
|---------|-------------------:|-----------------:|---------------:|---------------:|---------|
| chars (tamanho do relatório) | 8 819,2 | 15 180,8 | 11 258,0 | 28 473 (P156i) | P156e |
| linhas | 265,4 | 341,2 | 309,0 | 832 (P156i) | P156e |
| ADRs únicas mencionadas | 3,2 | 7,5 | 4,0 | 26 (P156b) | P158a |
| ADRs total mencionadas | 6,9 | 32,3 | 12,0 | 84 (P156b) | P131a |
| `padrão`/`subpadrão`/`patamar`/`limiar` | 1,0 | 16,8 | 1,0 | 39 (P159g) | P156d |
| `N=X` | 0,0 | 22,9 | 0,0 | 49 (P159g) | P156h |
| `cumulativ*` | 0,0 | 6,7 | 0,0 | 19 (P156i) | P156c |
| `consolidad*`/`consolida*` | 0,3 | 1,6 | 0,0 | 10 (P132b) | P156i |
| `cross-domínio`/`cross-feature` | 0,0 | 4,6 | 0,0 | 15 (sub-passo 159) | P157a |
| `aplicação concreta`/`cumulativa`/`isolada` | 0,0 | 3,1 | 0,0 | 8 (P159a) | P156l |

Notas literais:

- `chars` e `linhas` triplicam de tamanho médio entre P98 e P156i. P156i é o relatório isolado mais longo (832 linhas, 28 473 chars).
- `ADRs total` viragem em P131a é estatística e não substantiva: o valor absoluto (19 menções, 7 únicas) é compatível com um relatório que toca em vários ADRs uma vez cada. O salto de mediana ocorre porque a mediana histórica até P131a é baixíssima.
- `padrões`, `N=X`, `cumulativ*`, `consolidad*`, `aplicações`, `cross-*` são todas zero ou perto de zero antes de P156a, e crescem em série a partir do bloco P156c–P157a.
- `consolidad*` tem outlier isolado em P132b (10 ocorrências), produto de uma discussão pontual sobre consolidação de uma decisão; não é início de tendência (P133–P155 voltam a valores baixos).
- O máximo histórico de `padrões` (39) e `N=X` (49) ocorre em P159g, o último relatório do sub-bloco 159. Picos individuais máximos das métricas de processo aparecem dentro dos sub-passos P156j–P159g.

### Secção 4 — Cronologia de ADRs meta

Universo identificado por classificação em §2: 2 ADRs **meta** estritas e 2 ADRs **limiar**. Total: 4 ficheiros (`0054`, `0064`, `0065`, `0066`) em 65 ADRs (6 %).

| ordem | ID | criada em | proposta em (passo) | criada em (passo) | menções em relatórios posteriores |
|------:|----|-----------|---------------------|-------------------|-----------------------------------:|
| 1 | ADR-0054 | 2026-04-24 17:09 | P135 (via diagnóstico `diagnostico-shaping-passo-135.md` referenciado no header) | P140a (mesmo timestamp git) | 36 relatórios distintos |
| 2 | ADR-0065 | 2026-04-26 15:01 | série P156C–P156J (corpo da ADR enumera estes passos como precedente directo) | P156k (mesmo timestamp git) | 21 relatórios distintos |
| 3 | ADR-0064 | 2026-04-26 15:01 | série P156D–P156J (corpo da ADR enumera estes passos como precedente directo) | P156k (mesmo timestamp git) | 19 relatórios distintos |
| 4 | ADR-0066 | 2026-04-27 22:18 | P160 §1 (corpo da ADR refere "P160 §1 confirmou empiricamente") | P160a (mesmo timestamp git) | 1 relatório (P160a, único posterior) |

Observações literais sobre datas e ordenação:

- ADR-0054 e ADR-0066 estão classificadas como **limiar** porque misturam decisão técnica (DEBT, número-de-slot) com formalização do processo de fecho/promoção. ADR-0064 e ADR-0065 são meta puras — formalizam padrões de tradução e sub-passo de inventário.
- A primeira ADR de natureza meta-processual (limiar) é **ADR-0054**, criada em 2026-04-24 (P140a-equivalente). As duas ADRs meta estritas surgem **dois dias depois**, em 2026-04-26 (P156k-equivalente).
- Mensurada por número de menções subsequentes: ADR-0054 é a mais citada (36 menções); ADR-0065 (21) e ADR-0064 (19) seguem-se; ADR-0066 ainda só foi mencionada uma vez (foi criada no penúltimo relatório do universo).

### Secção 5 — Interpretação

Análise restrita aos números das tabelas anteriores. Sem narrativa causal, sem recomendação de acção.

**1. Em qual passo aparece a primeira ADR meta?**

A primeira ADR de natureza meta-processual em ficheiro é **ADR-0054** (categoria limiar), criada em 2026-04-24 17:09 com timestamp git coincidente com P140a. O seu enunciado deriva do diagnóstico `diagnostico-shaping-passo-135.md`, pelo que a sua proposta é atribuível a **P135**. As duas ADRs meta puras (`0064`, `0065`) só aparecem em **P156k** (2026-04-26), 49 horas depois.

**2. A partir de qual passo as métricas inflacionadas crescem consistentemente?**

As viragens ocorrem todas dentro do bloco P156a–P157a (Secção 3):

- `cumulativ*`: P156c
- `padrão/subpadrão/patamar/limiar`: P156d
- `chars`/`linhas`: P156e
- `N=X`: P156h
- `consolidad*`: P156i
- `aplicação concreta/cumulativa/isolada`: P156l
- `cross-domínio/cross-feature`: P157a

Antes de P156a as métricas em causa são essencialmente zero. A partir de P156a o tamanho médio do relatório duplica e os termos metodológicos passam a aparecer todos os passos. **P156a** é o primeiro relatório com um salto perceptível (chars de 11 581 médios em P150–P155 para 19 025 em P156a; `padrões` de 0–2 para 6).

**3. Há ponto de viragem único, ou degradação gradual?**

Os números mostram **uma única janela curta** — sete sub-passos do bloco 156 (P156a a P156l, espalhados por menos de 24 horas, 2026-04-25 19:15 a 2026-04-26 15:26) — durante a qual todas as métricas de processo passam de zero a regime sustentado. Antes desta janela, 50 relatórios (P98 a P155, 2 dias) mantêm valores próximos de zero em todas as métricas excepto `chars` e `ADRs`. Após a janela, 14 relatórios (P157 a P160a, 1 dia) mantêm os novos valores ou ampliam-nos (pico de `padrões` e `N=X` em P159g).

Não é, portanto, degradação gradual. É uma transição abrupta confinada à série de sub-passos do P156.

**4. Qual a relação entre criação de ADRs meta e crescimento das métricas?**

Correlação temporal observada (sem inferência causal):

- A inflação das métricas começa em **P156a (2026-04-25 19:15)**.
- ADR-0064 e ADR-0065 (meta puras) são criadas em **P156k (2026-04-26 15:01)**, 19 horas após P156a, **depois** de seis sub-passos já no novo regime (P156a–P156j).
- ADR-0054 (limiar), embora anterior (2026-04-24), só atinge alta frequência de menção a partir de P135-P140 (timestamp da sua criação) e o pico de menções coincide com o bloco P156–P159 (consultar tabela §1).

Os números **não suportam** que a criação de ADRs meta tenha precedido a inflação das métricas: as métricas saltam primeiro (P156a–P156j), as ADRs meta puras são escritas a meio do bloco (P156k), e o regime persiste depois. Fica registada a coincidência temporal entre (a) o aparecimento dos sub-passos sufixados (`156a`–`156l`, `158a`–`158c`, `159a`–`159g`) e (b) o regime metódico inflacionado, sem que a análise resolva qual factor terá precedido o outro.

---

## Lacunas documentais

- O universo de relatórios começa em **P98**. Os passos P0 (`0-arranque`), P1 (`1-entities`) até P97 não têm ficheiro `*-relatorio.md` em `00_nucleo/materialization/` (apenas o ficheiro de instrução `typst-passo-N.md`). A análise quantitativa cobre 91 relatórios de uma sequência total de ~160 passos; os primeiros 60 % do projecto não foram inspeccionados quantitativamente.
- Existem ficheiros de instrução para passos sem relatório correspondente após P98 (ex.: `typst-passo-142.md` sem `typst-passo-142-relatorio.md`; `typst-passo-156k-meta.md` separado do relatório principal). Lacunas pontuais não foram reconstruídas.
- ADR-0056 e ADR-0063 não existem em ficheiro: a numeração tem dois saltos. ADR-0026 tem variante `0026-R1`. Ambos os factos aparecem na tabela §2 sem comentário adicional.
- Os relatórios P131a/P131b, P132a/P132b, P140a/P140b, P156k-meta indicam adopção do sufixo alfabético antes do bloco P156, mas é em P156a–P156l (e depois P158a–P158c, P159a–P159g) que o sufixo se torna o padrão dominante.
