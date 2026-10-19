# Mapa de Migração vanilla ↔ cristalino (v1)

**Gerado**: Passo 312. **Fonte mecânica**: lente `--comparar` (laudo 0078), `lab/typst-original` (vanilla) vs raiz (cristalino).
**Convive com** `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` (a tabela de cobertura por feature); não a substitui.

Este documento tem **duas colunas independentes** por módulo:

- **Mecânica** (regenerável por `lab/mapa-migracao/gerar.py` em segundos, NUNCA editada à mão): o que a lente mede por pareamento de path. Pareados, sem-par, %, destino, sugestão.
- **Declarada** (curada por humano/sessão, SEMPRE com fonte): o estado afirmado pelos documentos do projeto. Sem fonte → `pendente-de-confirmação`.

A **discordância** entre as duas é o detector automático de deriva documental (falha F4 do `diagnostico-bloqueio-processo-2026-06-09.md`): a contradição aparece sozinha na regeneração; nenhuma sessão precisa de *lembrar* de propagar nada.

> **Aviso de leitura da coluna mecânica.** A lente pareia por path normalizado na raiz do crate. O cristalino **reorganizou** o vanilla em `typst_core::entities::*` e `typst_core::engine::*` com nomes novos. Logo um item migrado-mas-renomeado aparece **sem-par dos dois lados** (não há detecção de movido por similaridade). Por isso a coluna **destino dominante** é o sinal real de para-onde-foi, e um módulo `não-iniciado`/`parcial` mecânico com destino claro e declarado `fechado-consolidado` é coerente, não contraditório.

## Regras de derivação (mecânica)

**Módulo de um item** (a partir do path):
1. Remover segmentos de resolução `<...>` (ex.: `Abs::<Add>::Output`). Verificado no JSON: todo segmento de resolução começa por `<` ao separar por `::`.
2. Remover o último segmento (o nome do item).
3. Enquanto o último segmento restante começar por maiúscula (tipos pai — módulos são snake_case, tipos CamelCase em Rust), removê-lo. O que sobra é o módulo (mínimo: o crate).

**Boilerplate vs real**: item é boilerplate se o campo `trait` está preenchido (folha de impl-de-trait — medição 0077 da lente). Verificado: **todas** as listas do JSON (`pareados`, `sem_par_antes`, `sem_par_depois`, `ambiguos`) carregam `trait`; logo a **definição primária** foi usada e o fallback (fn-de-tipo com nome canónico) **não foi necessário**.

**Sugestão mecânica** (constante `MIGRADO_PCT = 0.90` no gerador; é sugestão, não veredito): `migrado` (≥90% reais pareados) · `parcial` (entre) · `não-iniciado` (0 pareado real) · `só-boilerplate` (0 itens reais).

**Vocabulário da coluna declarada** (fechado): `fechado` · `fechado-consolidado` (implementado com forma diferente de propósito — ex.: Content enum/ADR-0026) · `parcial` · `pendente` · `fora-de-escopo` · `pendente-de-confirmação`.

**Fontes permitidas para a coluna declarada** (cada marcação cita o ficheiro): `CLAUDE.md`, `00_nucleo/adr/`, a tabela de cobertura (`cobertura`, com a ressalva de deriva ≤P283), `00_nucleo/DEBT.md`, `00_nucleo/diagnosticos/`. Sem fonte suficiente → `pendente-de-confirmação`, sem excepção.

**Portão da lente (verificado)**: pareados = 1474 (esperado 1474 ✓); sem-par antes = 10910 (~10910 ✓); sem-par depois = 1203 (~1203 ✓).

## 1. Rollup por crate

| crate | módulos | migrados | parciais | não-inic. | só-bp | reais pareados | reais sem-par |
|---|--:|--:|--:|--:|--:|--:|--:|
| test_wrapper | 2 | 0 | 0 | 1 | 1 | 0 | 11 |
| typst | 1 | 0 | 0 | 1 | 0 | 0 | 8 |
| typst_bundle | 4 | 0 | 1 | 3 | 0 | 1 | 30 |
| typst_docs | 5 | 0 | 0 | 5 | 0 | 0 | 87 |
| typst_eval | 14 | 1 | 0 | 12 | 1 | 1 | 48 |
| typst_fuzz | 1 | 0 | 0 | 1 | 0 | 0 | 2 |
| typst_html | 24 | 0 | 0 | 14 | 10 | 0 | 211 |
| typst_ide | 14 | 0 | 0 | 11 | 3 | 0 | 92 |
| typst_kit | 10 | 0 | 1 | 6 | 3 | 2 | 111 |
| typst_layout | 52 | 0 | 2 | 45 | 5 | 4 | 583 |
| typst_library | 282 | 1 | 46 | 142 | 93 | 164 | 3307 |
| typst_macros | 1 | 0 | 0 | 1 | 0 | 0 | 6 |
| typst_pdf | 38 | 0 | 1 | 27 | 10 | 1 | 353 |
| typst_realize | 2 | 0 | 0 | 2 | 0 | 0 | 44 |
| typst_render | 6 | 0 | 0 | 5 | 1 | 0 | 37 |
| typst_svg | 11 | 0 | 0 | 8 | 3 | 0 | 94 |
| typst_syntax | 14 | 8 | 3 | 3 | 0 | 491 | 112 |
| typst_timing | 1 | 0 | 0 | 1 | 0 | 0 | 17 |
| typst_utils | 17 | 0 | 1 | 14 | 2 | 2 | 88 |
| **TOTAL** | 499 | | | | | 666 | 5241 |

## 2. Tabela por módulo (mecânica + declarada)

Colunas mecânicas regeneram via `gerar.py`; as duas últimas (declarado, fonte) são curadas à mão.

### test_wrapper

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `test_wrapper` | 11+1 | 0 | 11 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `test_wrapper::process` | 0+2 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |

### typst

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `typst` | 8+2 | 0 | 8 | 0% | — | não-iniciado | pendente-de-confirmação | — |

### typst_bundle

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `typst_bundle` | 10+13 | 0 | 10 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_bundle::export_` | 8+1 | 1 | 7 | 12% | typst_infra::export (100%) | parcial | pendente-de-confirmação | — |
| `typst_bundle::introspect` | 11+22 | 0 | 11 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_bundle::link` | 2+0 | 0 | 2 | 0% | — | não-iniciado | pendente-de-confirmação | — |

### typst_docs

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `typst_docs` | 38+3 | 0 | 38 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_docs::contribs` | 4+7 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_docs::html` | 24+15 | 0 | 24 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_docs::link` | 4+0 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_docs::model` | 17+30 | 0 | 17 | 0% | — | não-iniciado | pendente-de-confirmação | — |

### typst_eval

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `typst_eval` | 2+0 | 0 | 2 | 0% | — | não-iniciado | fechado-consolidado | ADR-0017/0067 (eval→rules; attribute grammar scoping) |
| `typst_eval::access` | 2+0 | 0 | 2 | 0% | — | não-iniciado | fechado-consolidado | ADR-0017/0067 (eval→rules; attribute grammar scoping) |
| `typst_eval::ast` | 0+14 | 0 | 0 | 0% | — | só-boilerplate | fechado-consolidado | ADR-0017/0067 (eval→rules; attribute grammar scoping) |
| `typst_eval::binding` | 5+0 | 0 | 5 | 0% | — | não-iniciado | fechado-consolidado | ADR-0017/0067 (eval→rules; attribute grammar scoping) |
| `typst_eval::call` | 12+0 | 0 | 12 | 0% | — | não-iniciado | fechado-consolidado | ADR-0017/0067 (eval→rules; attribute grammar scoping) |
| `typst_eval::code` | 2+0 | 0 | 2 | 0% | — | não-iniciado | fechado-consolidado | ADR-0017/0067 (eval→rules; attribute grammar scoping) |
| `typst_eval::flow` | 4+3 | 0 | 4 | 0% | — | não-iniciado | fechado-consolidado | ADR-0017/0067 (eval→rules; attribute grammar scoping) |
| `typst_eval::import` | 4+0 | 0 | 4 | 0% | — | não-iniciado | fechado-consolidado | ADR-0017/0067 (eval→rules; attribute grammar scoping) |
| `typst_eval::markup` | 1+0 | 1 | 0 | 100% | typst_core::engine::eval (100%) | migrado | fechado-consolidado | ADR-0017/0067 (eval→rules; attribute grammar scoping) |
| `typst_eval::math` | 1+0 | 0 | 1 | 0% | — | não-iniciado | fechado-consolidado | ADR-0017/0067 (eval→rules; attribute grammar scoping) |
| `typst_eval::methods` | 5+0 | 0 | 5 | 0% | — | não-iniciado | fechado-consolidado | ADR-0017/0067 (eval→rules; attribute grammar scoping) |
| `typst_eval::ops` | 2+0 | 0 | 2 | 0% | — | não-iniciado | fechado-consolidado | ADR-0017/0067 (eval→rules; attribute grammar scoping) |
| `typst_eval::rules` | 2+0 | 0 | 2 | 0% | — | não-iniciado | fechado-consolidado | ADR-0017/0067 (eval→rules; attribute grammar scoping) |
| `typst_eval::vm` | 7+0 | 0 | 7 | 0% | — | não-iniciado | fechado-consolidado | ADR-0017/0067 (eval→rules; attribute grammar scoping) |

### typst_fuzz

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `typst_fuzz` | 2+7 | 0 | 2 | 0% | — | não-iniciado | pendente-de-confirmação | — |

### typst_html

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `typst_html` | 12+16 | 0 | 12 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_html::charsets` | 7+0 | 0 | 7 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_html::convert` | 21+4 | 0 | 21 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_html::css` | 18+2 | 0 | 18 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_html::document` | 13+2 | 0 | 13 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_html::dom` | 35+48 | 0 | 35 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_html::encode` | 23+0 | 0 | 23 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_html::foundations::auto` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_html::foundations::datetime` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_html::foundations::duration` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_html::foundations::float` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_html::foundations::none` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_html::foundations::str` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_html::fragment` | 4+0 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_html::introspect` | 11+19 | 0 | 11 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_html::layout::dir` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_html::link` | 9+0 | 0 | 9 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_html::model::link` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_html::num::nonzero` | 0+2 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_html::rules` | 4+0 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_html::tag` | 14+0 | 0 | 14 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_html::typed` | 37+46 | 0 | 37 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_html::typed::datetime` | 3+0 | 0 | 3 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_html::visualize::color` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |

### typst_ide

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `typst_ide` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_ide::analyze` | 4+0 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_ide::complete` | 44+9 | 0 | 44 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_ide::definition` | 2+2 | 0 | 2 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_ide::docs` | 5+2 | 0 | 5 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_ide::document` | 0+4 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_ide::dom` | 0+4 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_ide::jump` | 10+3 | 0 | 10 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_ide::jump::jump_from_document_sealed` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_ide::jump::jump_in_document_sealed` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_ide::matchers` | 7+2 | 0 | 7 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_ide::string` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_ide::tooltip` | 10+3 | 0 | 10 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_ide::utils` | 7+0 | 0 | 7 | 0% | — | não-iniciado | pendente-de-confirmação | — |

### typst_kit

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `typst_kit::boxed` | 0+2 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_kit::diagnostics` | 9+14 | 0 | 9 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_kit::downloader` | 16+7 | 0 | 16 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_kit::files` | 22+6 | 0 | 22 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_kit::fonts` | 15+3 | 2 | 13 | 13% | typst_infra::fonts (100%) | parcial | pendente-de-confirmação | — |
| `typst_kit::packages` | 25+2 | 0 | 25 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_kit::server` | 21+0 | 0 | 21 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_kit::sync` | 0+2 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_kit::text::font` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_kit::watcher` | 5+0 | 0 | 5 | 0% | — | não-iniciado | pendente-de-confirmação | — |

### typst_layout

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `typst_layout::document` | 8+10 | 3 | 5 | 38% | typst_core::entities::layout_types (100%) | parcial | pendente-de-confirmação | — |
| `typst_layout::flow` | 19+6 | 0 | 19 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::flow::block` | 5+0 | 0 | 5 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::flow::collect` | 29+10 | 0 | 29 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::flow::compose` | 26+1 | 0 | 26 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::flow::distribute` | 26+0 | 0 | 26 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::grid` | 4+0 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::grid::layouter` | 49+5 | 0 | 49 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::grid::lines` | 5+7 | 0 | 5 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::grid::repeated` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::grid::rowspans` | 10+1 | 0 | 10 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::image` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::inline` | 11+4 | 0 | 11 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::inline::box_` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::inline::collect` | 19+3 | 0 | 19 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::inline::deco` | 7+5 | 0 | 7 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::inline::finalize` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::inline::line` | 36+17 | 0 | 36 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::inline::linebreak` | 31+3 | 0 | 31 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::inline::prepare` | 5+0 | 0 | 5 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::inline::shaping` | 59+18 | 0 | 59 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::introspect` | 8+19 | 0 | 8 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::layout::fragment` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_layout::layout::frame` | 0+2 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_layout::lists` | 2+0 | 0 | 2 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::math` | 18+0 | 0 | 18 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::math::accent` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::math::cancel` | 3+0 | 0 | 3 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::math::fenced` | 3+0 | 0 | 3 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::math::fraction` | 2+0 | 0 | 2 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::math::fragment` | 48+8 | 0 | 48 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::math::line` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::math::radical` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::math::run` | 10+2 | 0 | 10 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::math::scripts` | 9+0 | 0 | 9 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::math::shaping` | 3+5 | 0 | 3 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::math::table` | 5+0 | 0 | 5 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::math::text` | 5+0 | 0 | 5 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::modifiers` | 6+2 | 0 | 6 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::pad` | 4+0 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::pages` | 6+0 | 0 | 6 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::pages::collect` | 3+0 | 0 | 3 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::pages::finalize` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::pages::run` | 4+1 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::repeat` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::result` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_layout::rules` | 3+0 | 0 | 3 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::shapes` | 67+4 | 1 | 66 | 1% | typst_core::entities::geometry (100%) | parcial | pendente-de-confirmação | — |
| `typst_layout::stack` | 13+4 | 0 | 13 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::transforms` | 7+0 | 0 | 7 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_layout::vec` | 0+2 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_layout::visualize::curve` | 0+3 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |

### typst_library

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `typst_library` | 18+41 | 2 | 16 | 11% | typst_core::contracts::world (50%) | parcial | pendente-de-confirmação | — |
| `typst_library::borrow` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::boxed` | 0+7 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::diag` | 59+54 | 9 | 50 | 15% | typst_core::entities::source_result (78%) | parcial | fechado-consolidado | ADR-0045 + ADR-0085 (formato/imutável; entities::source_result) |
| `typst_library::duration` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::engine` | 32+29 | 16 | 16 | 50% | typst_core::entities::world_types (81%) | parcial | fechado-consolidado | ADR-0044 (engine agregador) |
| `typst_library::engine::__ComemoSurface` | 3+0 | 3 | 0 | 100% | typst_core::entities::world_types::__ComemoSurface (100%) | migrado | fechado-consolidado | ADR-0044 (engine agregador) |
| `typst_library::engine::__ComemoSurfaceMut` | 8+0 | 3 | 5 | 38% | typst_core::entities::world_types::__ComemoSurfaceMut (100%) | parcial | fechado-consolidado | ADR-0044 (engine agregador) |
| `typst_library::foundations` | 6+0 | 0 | 6 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::foundations::args` | 35+25 | 2 | 33 | 6% | typst_core::entities::args (100%) | parcial | fechado-consolidado | ADR-0059 (Args tipo separado) |
| `typst_library::foundations::array` | 85+37 | 0 | 85 | 0% | — | não-iniciado | parcial | cobertura A.8 + bloqueio C4 (methods parciais; construct Cat B pendente) |
| `typst_library::foundations::assert` | 4+3 | 0 | 4 | 0% | — | não-iniciado | fechado | cobertura A.8 (native_assert) |
| `typst_library::foundations::auto` | 16+30 | 0 | 16 | 0% | — | não-iniciado | fechado | cobertura B.1 (Value::Auto) |
| `typst_library::foundations::bytes` | 30+27 | 5 | 25 | 17% | typst_core::entities::world_types (100%) | parcial | pendente-de-confirmação | — |
| `typst_library::foundations::calc` | 98+22 | 0 | 98 | 0% | — | não-iniciado | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::abs` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::acos` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::acosh` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::asin` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::asinh` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::atan` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::atan2` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::atanh` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::binom` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::ceil` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::clamp` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::cos` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::cosh` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::div_euclid` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::erf` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::even` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::exp` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::fact` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::floor` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::fract` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::gcd` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::lcm` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::ln` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::log` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::max` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::min` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::norm` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::odd` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::perm` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::pow` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::quo` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::rem` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::rem_euclid` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::root` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::round` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::sin` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::sinh` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::sqrt` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::tan` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::tanh` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::calc::trunc` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.8 + diagnostico-calc-passo-283 (74%) |
| `typst_library::foundations::cast` | 11+27 | 0 | 11 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::foundations::content` | 62+40 | 7 | 55 | 11% | typst_core::entities::content (100%) | parcial | fechado-consolidado | cobertura B.2 + ADR-0026 (vtable→enum fechado) |
| `typst_library::foundations::content::element` | 28+13 | 0 | 28 | 0% | — | não-iniciado | fechado-consolidado | cobertura B.2 + ADR-0026 (vtable→enum fechado) |
| `typst_library::foundations::content::field` | 43+11 | 0 | 43 | 0% | — | não-iniciado | fechado-consolidado | cobertura B.2 + ADR-0026 (vtable→enum fechado) |
| `typst_library::foundations::content::packed` | 31+82 | 0 | 31 | 0% | — | não-iniciado | fechado-consolidado | cobertura B.2 + ADR-0026 (vtable→enum fechado) |
| `typst_library::foundations::content::raw` | 26+7 | 0 | 26 | 0% | — | não-iniciado | fechado-consolidado | cobertura B.2 + ADR-0026 (vtable→enum fechado) |
| `typst_library::foundations::content::vtable` | 27+2 | 0 | 27 | 0% | — | não-iniciado | fechado-consolidado | cobertura B.2 + ADR-0026 (vtable→enum fechado) |
| `typst_library::foundations::context` | 10+18 | 0 | 10 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::foundations::context::__ComemoSurface` | 3+0 | 0 | 3 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::foundations::context::__ComemoSurfaceMut` | 3+0 | 0 | 3 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::foundations::datetime` | 32+30 | 7 | 25 | 22% | typst_core::entities::world_types (100%) | parcial | parcial | ADR-0021 (datetime) |
| `typst_library::foundations::decimal` | 22+28 | 0 | 22 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::foundations::dict` | 34+28 | 0 | 34 | 0% | — | não-iniciado | parcial | cobertura A.8 (methods parciais) |
| `typst_library::foundations::duration` | 15+28 | 0 | 15 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::foundations::eval` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente | cobertura A.8 (eval(string) ausente) |
| `typst_library::foundations::fields` | 4+0 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::foundations::float` | 6+13 | 0 | 6 | 0% | — | não-iniciado | fechado | cobertura A.8 (constructor) |
| `typst_library::foundations::func` | 39+31 | 2 | 37 | 5% | typst_core::entities::func (100%) | parcial | parcial | cobertura A.2 + DEBT-2 (closures eager) |
| `typst_library::foundations::int` | 5+13 | 0 | 5 | 0% | — | não-iniciado | fechado | cobertura A.8 (constructor) |
| `typst_library::foundations::label` | 6+9 | 0 | 6 | 0% | — | não-iniciado | fechado | cobertura A.1 (Content::Labelled) |
| `typst_library::foundations::module` | 14+14 | 6 | 8 | 43% | typst_core::entities::module (100%) | parcial | parcial | cobertura A.2 (#import limitado) |
| `typst_library::foundations::none` | 1+15 | 0 | 1 | 0% | — | não-iniciado | fechado | cobertura B.1 (Value::None) |
| `typst_library::foundations::ops` | 28+0 | 0 | 28 | 0% | — | não-iniciado | fechado | cobertura A.8 (control flow) |
| `typst_library::foundations::panic` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente | cobertura A.8 (ausente) |
| `typst_library::foundations::path` | 9+19 | 0 | 9 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::foundations::plugin_` | 25+13 | 0 | 25 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::foundations::plugin_::plugin` | 2+3 | 0 | 2 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::foundations::repr` | 11+0 | 1 | 10 | 9% | typst_core::engine::stdlib::foundations (100%) | parcial | parcial | cobertura A.8 (subset) |
| `typst_library::foundations::repr::repr` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | parcial | cobertura A.8 (subset) |
| `typst_library::foundations::scope` | 45+22 | 12 | 33 | 27% | typst_core::entities::scope (58%) | parcial | fechado | cobertura A.2 (#let scoping) |
| `typst_library::foundations::selector` | 18+29 | 0 | 18 | 0% | — | não-iniciado | parcial | cobertura A.9 (And/Or/Regex parcial P209) |
| `typst_library::foundations::str` | 68+84 | 2 | 66 | 3% | typst_core::entities::regex (100%) | parcial | parcial | cobertura A.8 + ADR-0077 (methods parciais; regex L1) |
| `typst_library::foundations::styles` | 71+62 | 5 | 66 | 7% | typst_core::entities::style (80%) | parcial | fechado-consolidado | cobertura B.3/B.4 + ADR-0038/0039 (DEBT-1 fechado P142) |
| `typst_library::foundations::styles::rule` | 3+1 | 0 | 3 | 0% | — | não-iniciado | fechado-consolidado | cobertura B.3/B.4 + ADR-0038/0039 (DEBT-1 fechado P142) |
| `typst_library::foundations::symbol` | 23+41 | 0 | 23 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::foundations::sys` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::foundations::target_` | 9+16 | 0 | 9 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::foundations::target_::target` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::foundations::ty` | 14+18 | 0 | 14 | 0% | — | não-iniciado | fechado | cobertura A.8 (type()) |
| `typst_library::foundations::value` | 17+46 | 1 | 16 | 6% | typst_core::entities::value (100%) | parcial | fechado-consolidado | cobertura B.1 + ADR-0024/0058 (30→18 variants) |
| `typst_library::foundations::version` | 10+25 | 0 | 10 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::hash` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::introspection` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::introspection::convergence` | 14+4 | 0 | 14 | 0% | — | não-iniciado | fechado | ADR-0072 (m7 fixpoint runtime fechado) |
| `typst_library::introspection::counter` | 50+81 | 1 | 49 | 2% | typst_core::entities::counter_update (100%) | parcial | fechado | cobertura A.9 (implementado⁺ P176/177/210B) |
| `typst_library::introspection::here_` | 2+0 | 0 | 2 | 0% | — | não-iniciado | fechado | cobertura A.9 (here/locate P208) |
| `typst_library::introspection::here_::here` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.9 (here/locate P208) |
| `typst_library::introspection::introspector` | 40+22 | 1 | 39 | 2% | typst_core::entities::introspector (100%) | parcial | fechado-consolidado | ADR-0073/0076 (introspector completion) |
| `typst_library::introspection::locate_` | 2+0 | 0 | 2 | 0% | — | não-iniciado | fechado | cobertura A.9 (P208C) |
| `typst_library::introspection::locate_::locate` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.9 (P208C) |
| `typst_library::introspection::location` | 22+61 | 1 | 21 | 5% | typst_core::entities::location (100%) | parcial | fechado-consolidado | cobertura A.9 + ADR-0068 (location-aware) |
| `typst_library::introspection::locator` | 18+19 | 1 | 17 | 6% | typst_core::entities::locator (100%) | parcial | fechado-consolidado | ADR-0074 (locator substores trackable) |
| `typst_library::introspection::locator::__ComemoSurface` | 1+0 | 0 | 1 | 0% | — | não-iniciado | fechado-consolidado | ADR-0074 (locator substores trackable) |
| `typst_library::introspection::locator::__ComemoSurfaceMut` | 1+0 | 0 | 1 | 0% | — | não-iniciado | fechado-consolidado | ADR-0074 (locator substores trackable) |
| `typst_library::introspection::metadata` | 2+6 | 0 | 2 | 0% | — | não-iniciado | fechado | cobertura A.9 (P169) |
| `typst_library::introspection::position` | 12+20 | 0 | 12 | 0% | — | não-iniciado | parcial | cobertura A.9 (sem stdlib expose) |
| `typst_library::introspection::query_` | 8+28 | 0 | 8 | 0% | — | não-iniciado | fechado | cobertura A.9 (implementado⁺ P209) |
| `typst_library::introspection::query_::query` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.9 (implementado⁺ P209) |
| `typst_library::introspection::state` | 22+42 | 1 | 21 | 5% | typst_core::entities::state_update (100%) | parcial | fechado | cobertura A.9 (P171) |
| `typst_library::introspection::tag` | 6+10 | 0 | 6 | 0% | — | não-iniciado | fechado | ADR-0069 (post-recursion tag emission) |
| `typst_library::layout` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::layout::abs` | 27+36 | 3 | 24 | 11% | typst_core::entities::layout_types (100%) | parcial | pendente-de-confirmação | — |
| `typst_library::layout::align` | 26+118 | 0 | 26 | 0% | — | não-iniciado | fechado | cobertura A.5 (implementado) |
| `typst_library::layout::angle` | 26+45 | 5 | 21 | 19% | typst_core::entities::layout_types (100%) | parcial | pendente-de-confirmação | — |
| `typst_library::layout::axes` | 27+52 | 2 | 25 | 7% | typst_core::entities::axes (100%) | parcial | pendente-de-confirmação | — |
| `typst_library::layout::columns` | 7+14 | 0 | 7 | 0% | — | não-iniciado | parcial | cobertura A.5 (multi-region flow ausente) |
| `typst_library::layout::container` | 37+64 | 0 | 37 | 0% | — | não-iniciado | fechado | cobertura A.5 (box/block P250/252 completos) |
| `typst_library::layout::container::callbacks` | 9+12 | 0 | 9 | 0% | — | não-iniciado | fechado | cobertura A.5 (box/block P250/252 completos) |
| `typst_library::layout::corners` | 14+20 | 2 | 12 | 14% | typst_core::entities::corners (100%) | parcial | pendente | cobertura A.5 (inset modeling ausente) |
| `typst_library::layout::dir` | 16+13 | 1 | 15 | 6% | typst_core::entities::dir (100%) | parcial | pendente-de-confirmação | — |
| `typst_library::layout::em` | 10+30 | 0 | 10 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::layout::fr` | 7+33 | 0 | 7 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::layout::fragment` | 10+5 | 0 | 10 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::layout::frame` | 52+24 | 4 | 48 | 8% | typst_core::entities::layout_types (100%) | parcial | pendente-de-confirmação | — |
| `typst_library::layout::grid` | 51+121 | 0 | 51 | 0% | — | não-iniciado | fechado | cobertura A.5 (implementado⁺ P224-230) |
| `typst_library::layout::grid::resolve` | 48+30 | 0 | 48 | 0% | — | não-iniciado | fechado | cobertura A.5 (implementado⁺ P224-230) |
| `typst_library::layout::hide` | 2+7 | 0 | 2 | 0% | — | não-iniciado | fechado | cobertura A.5 (P156C) |
| `typst_library::layout::layout_` | 4+6 | 1 | 3 | 25% | typst_core::engine::layout (100%) | parcial | pendente-de-confirmação | — |
| `typst_library::layout::layout_::layout` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::layout::length` | 16+38 | 1 | 15 | 6% | typst_core::entities::layout_types (100%) | parcial | pendente-de-confirmação | — |
| `typst_library::layout::measure_` | 2+0 | 0 | 2 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::layout::measure_::measure` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::layout::pad` | 6+13 | 0 | 6 | 0% | — | não-iniciado | fechado | cobertura A.5 (implementado⁺ P156C/L) |
| `typst_library::layout::page` | 23+71 | 2 | 21 | 9% | typst_core::entities::parity (100%) | parcial | fechado | cobertura A.5 (page+pagebreak P81/156E) |
| `typst_library::layout::place` | 11+29 | 0 | 11 | 0% | — | não-iniciado | fechado | cobertura A.5 (implementado⁺ P223) |
| `typst_library::layout::point` | 15+24 | 1 | 14 | 7% | typst_core::entities::layout_types (100%) | parcial | pendente-de-confirmação | — |
| `typst_library::layout::ratio` | 11+35 | 2 | 9 | 18% | typst_core::entities::layout_types (100%) | parcial | pendente-de-confirmação | — |
| `typst_library::layout::rect` | 4+4 | 1 | 3 | 25% | typst_core::entities::layout_types (100%) | parcial | pendente-de-confirmação | — |
| `typst_library::layout::regions` | 10+5 | 2 | 8 | 20% | typst_core::entities::region (100%) | parcial | pendente-de-confirmação | — |
| `typst_library::layout::rel` | 9+41 | 0 | 9 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::layout::repeat` | 4+8 | 0 | 4 | 0% | — | não-iniciado | fechado | cobertura A.5 (P156J) |
| `typst_library::layout::sides` | 20+23 | 2 | 18 | 10% | typst_core::entities::sides (100%) | parcial | pendente | cobertura A.5 (inset modeling ausente) |
| `typst_library::layout::size` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::layout::spacing` | 11+32 | 0 | 11 | 0% | — | não-iniciado | fechado | cobertura A.5 (h/v P156D) |
| `typst_library::layout::stack` | 5+17 | 0 | 5 | 0% | — | não-iniciado | fechado | cobertura A.5 (P156I) |
| `typst_library::layout::transform` | 36+52 | 0 | 36 | 0% | — | não-iniciado | fechado | cobertura A.5 (rotate/scale/move/skew P78) |
| `typst_library::loading` | 9+26 | 0 | 9 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::loading::cbor_` | 2+0 | 0 | 2 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::loading::cbor_::cbor` | 4+3 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::loading::csv_` | 5+15 | 0 | 5 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::loading::csv_::csv` | 2+3 | 0 | 2 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::loading::json_` | 2+0 | 0 | 2 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::loading::json_::json` | 4+3 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::loading::read_` | 3+9 | 0 | 3 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::loading::read_::read` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::loading::toml_` | 3+0 | 0 | 3 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::loading::toml_::toml` | 4+3 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::loading::xml_` | 4+0 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::loading::xml_::xml` | 2+3 | 0 | 2 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::loading::yaml_` | 3+0 | 0 | 3 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::loading::yaml_::yaml` | 4+3 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::math` | 14+17 | 0 | 14 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::math::accent` | 12+19 | 0 | 12 | 0% | — | não-iniciado | fechado | cobertura A.4 (P296) |
| `typst_library::math::attach` | 24+40 | 0 | 24 | 0% | — | não-iniciado | fechado | cobertura A.4 (implementado) |
| `typst_library::math::cancel` | 8+20 | 0 | 8 | 0% | — | não-iniciado | fechado | cobertura A.4 (P296) |
| `typst_library::math::equation` | 8+19 | 0 | 8 | 0% | — | não-iniciado | fechado | cobertura A.4 (implementado) |
| `typst_library::math::frac` | 9+27 | 0 | 9 | 0% | — | não-iniciado | fechado | cobertura A.4 (implementado) |
| `typst_library::math::ir` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::math::ir::item` | 89+38 | 0 | 89 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::math::ir::multiline` | 8+4 | 0 | 8 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::math::ir::process` | 11+3 | 0 | 11 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::math::ir::resolve` | 51+0 | 0 | 51 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::math::lr` | 20+13 | 0 | 20 | 0% | — | não-iniciado | fechado | cobertura A.4 (P76) |
| `typst_library::math::lr::abs` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.4 (P76) |
| `typst_library::math::lr::ceil` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.4 (P76) |
| `typst_library::math::lr::floor` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.4 (P76) |
| `typst_library::math::lr::norm` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.4 (P76) |
| `typst_library::math::lr::round` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.4 (P76) |
| `typst_library::math::matrix` | 27+73 | 0 | 27 | 0% | — | não-iniciado | fechado | cobertura A.4 (matrix/cases impl⁺ P79) |
| `typst_library::math::op` | 4+7 | 0 | 4 | 0% | — | não-iniciado | fechado | diagnostico-math-op-passo-298 (P298/299) |
| `typst_library::math::root` | 5+7 | 0 | 5 | 0% | — | não-iniciado | fechado | cobertura A.4 (implementado⁺) |
| `typst_library::math::root::sqrt` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.4 (implementado⁺) |
| `typst_library::math::style` | 36+11 | 0 | 36 | 0% | — | não-iniciado | fechado | ADR-0102/0103 (math style P311a) |
| `typst_library::math::style::bb` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | ADR-0102/0103 (math style P311a) |
| `typst_library::math::style::bold` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | ADR-0102/0103 (math style P311a) |
| `typst_library::math::style::cal` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | ADR-0102/0103 (math style P311a) |
| `typst_library::math::style::display` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | ADR-0102/0103 (math style P311a) |
| `typst_library::math::style::frak` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | ADR-0102/0103 (math style P311a) |
| `typst_library::math::style::inline` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | ADR-0102/0103 (math style P311a) |
| `typst_library::math::style::italic` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | ADR-0102/0103 (math style P311a) |
| `typst_library::math::style::mono` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | ADR-0102/0103 (math style P311a) |
| `typst_library::math::style::sans` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | ADR-0102/0103 (math style P311a) |
| `typst_library::math::style::scr` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | ADR-0102/0103 (math style P311a) |
| `typst_library::math::style::script` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | ADR-0102/0103 (math style P311a) |
| `typst_library::math::style::serif` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | ADR-0102/0103 (math style P311a) |
| `typst_library::math::style::sscript` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | ADR-0102/0103 (math style P311a) |
| `typst_library::math::style::upright` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | ADR-0102/0103 (math style P311a) |
| `typst_library::math::underover` | 28+68 | 0 | 28 | 0% | — | não-iniciado | fechado | cobertura A.4 (P297) |
| `typst_library::model` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::model::asset` | 4+16 | 0 | 4 | 0% | — | não-iniciado | pendente | cobertura A.6 (ausente; Fase 3 ADR-0060) |
| `typst_library::model::bibliography` | 52+47 | 0 | 52 | 0% | — | não-iniciado | parcial | cobertura A.6 + DEBT-55 (P159; sem hayagriva) |
| `typst_library::model::cite` | 11+31 | 1 | 10 | 9% | typst_core::entities::citation_form (100%) | parcial | parcial | cobertura A.6 (P159; sem CSL) |
| `typst_library::model::divider` | 2+5 | 0 | 2 | 0% | — | não-iniciado | fechado | cobertura A.6 (P154B) |
| `typst_library::model::document` | 19+57 | 0 | 19 | 0% | — | não-iniciado | pendente | cobertura A.6 (ausente) |
| `typst_library::model::emph` | 2+6 | 0 | 2 | 0% | — | não-iniciado | fechado | cobertura A.1 (implementado) |
| `typst_library::model::enum_` | 13+29 | 0 | 13 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::model::figure` | 26+45 | 0 | 26 | 0% | — | não-iniciado | fechado | cobertura A.6 (implementado⁺ P158) |
| `typst_library::model::footnote` | 21+42 | 0 | 21 | 0% | — | não-iniciado | parcial | cobertura A.6 + bloqueio C3 (P295 marker only) |
| `typst_library::model::heading` | 12+15 | 0 | 12 | 0% | — | não-iniciado | fechado | cobertura A.6 (implementado) |
| `typst_library::model::link` | 38+78 | 0 | 38 | 0% | — | não-iniciado | parcial | cobertura A.6 (capturado; sem render visual) |
| `typst_library::model::link::__ComemoSurface` | 1+0 | 0 | 1 | 0% | — | não-iniciado | parcial | cobertura A.6 (capturado; sem render visual) |
| `typst_library::model::link::__ComemoSurfaceMut` | 1+0 | 0 | 1 | 0% | — | não-iniciado | parcial | cobertura A.6 (capturado; sem render visual) |
| `typst_library::model::list` | 12+33 | 0 | 12 | 0% | — | não-iniciado | parcial | cobertura A.6 (function form parcial) |
| `typst_library::model::numbering_` | 18+25 | 0 | 18 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::model::numbering_::numbering` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::model::outline` | 32+44 | 0 | 32 | 0% | — | não-iniciado | fechado | cobertura A.6 (implementado) |
| `typst_library::model::par` | 27+88 | 0 | 27 | 0% | — | não-iniciado | parcial | cobertura A.6 (P138; sem Content::Par) |
| `typst_library::model::quote` | 8+19 | 0 | 8 | 0% | — | não-iniciado | fechado | cobertura A.6 (P155) |
| `typst_library::model::reference` | 12+29 | 0 | 12 | 0% | — | não-iniciado | fechado | cobertura A.6 (implementado⁺) |
| `typst_library::model::strong` | 3+7 | 0 | 3 | 0% | — | não-iniciado | fechado | cobertura A.1 (implementado) |
| `typst_library::model::table` | 48+102 | 0 | 48 | 0% | — | não-iniciado | parcial | cobertura A.6 (table impl; cell/header/footer parcial) |
| `typst_library::model::terms` | 8+25 | 0 | 8 | 0% | — | não-iniciado | fechado | cobertura A.6 (P154B) |
| `typst_library::model::title` | 4+6 | 0 | 4 | 0% | — | não-iniciado | pendente | cobertura A.6 (ausente; Fase 3 ADR-0060) |
| `typst_library::month` | 0+4 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::num::nonzero` | 0+25 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::option` | 0+11 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::path` | 0+11 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::pdf` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::pdf::accessibility` | 25+43 | 0 | 25 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::pdf::accessibility::data_cell` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::pdf::accessibility::header_cell` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::pdf::accessibility::table_summary` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::pdf::attach` | 6+19 | 0 | 6 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::pico` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::result` | 0+19 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::routines` | 10+7 | 1 | 9 | 10% | typst_core::entities::world_types (100%) | parcial | pendente-de-confirmação | — |
| `typst_library::scalar` | 0+5 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::span` | 0+13 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::string` | 0+15 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::symbols` | 3+0 | 0 | 3 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::sync` | 0+16 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::text` | 41+210 | 5 | 36 | 12% | typst_core::entities::font_list (100%) | parcial | pendente-de-confirmação | — |
| `typst_library::text::case` | 8+14 | 0 | 8 | 0% | — | não-iniciado | fechado | cobertura A.3 (upper/lower/replace) |
| `typst_library::text::case::lower` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.3 (upper/lower/replace) |
| `typst_library::text::case::upper` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | fechado | cobertura A.3 (upper/lower/replace) |
| `typst_library::text::deco` | 30+52 | 0 | 30 | 0% | — | não-iniciado | fechado | cobertura A.3 (P284-286 underline/strike/overline) |
| `typst_library::text::font` | 26+23 | 2 | 24 | 8% | typst_core::entities::math_constants (50%) | parcial | fechado | cobertura A.3 + ADR-0053 (string/array/multi-doc; dict scope-out) |
| `typst_library::text::font::book` | 72+87 | 7 | 65 | 10% | typst_core::entities::font_book (100%) | parcial | fechado | cobertura A.3 + ADR-0053 (string/array/multi-doc; dict scope-out) |
| `typst_library::text::font::color` | 26+16 | 0 | 26 | 0% | — | não-iniciado | fechado | cobertura A.3 + ADR-0053 (string/array/multi-doc; dict scope-out) |
| `typst_library::text::font::exceptions` | 7+3 | 0 | 7 | 0% | — | não-iniciado | fechado | cobertura A.3 + ADR-0053 (string/array/multi-doc; dict scope-out) |
| `typst_library::text::font::variant` | 15+55 | 9 | 6 | 60% | typst_core::entities::font_book (100%) | parcial | fechado | cobertura A.3 + ADR-0053 (string/array/multi-doc; dict scope-out) |
| `typst_library::text::item` | 13+8 | 0 | 13 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::text::lang` | 15+45 | 2 | 13 | 13% | typst_core::entities::lang (100%) | parcial | parcial | cobertura A.3 + ADR-0057 (hyphenation impl⁺; shaping scope-out) |
| `typst_library::text::linebreak` | 4+6 | 0 | 4 | 0% | — | não-iniciado | parcial | cobertura A.1/A.3 (só math) |
| `typst_library::text::lorem_` | 3+0 | 0 | 3 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::text::lorem_::lorem` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::text::raw` | 39+41 | 0 | 39 | 0% | — | não-iniciado | fechado | cobertura A.1/A.3 (implementado) |
| `typst_library::text::shift` | 17+26 | 0 | 17 | 0% | — | não-iniciado | pendente | cobertura A.3 (script ausente) |
| `typst_library::text::smallcaps_` | 4+11 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::text::smartquote` | 23+30 | 0 | 23 | 0% | — | não-iniciado | fechado | cobertura A.1 (P287) |
| `typst_library::text::space` | 3+6 | 0 | 3 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::vec` | 0+7 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_library::visualize` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_library::visualize::color` | 97+58 | 10 | 87 | 10% | typst_core::entities::color (100%) | parcial | parcial | ADR-0083 (paridade subset; cobertura A.7 desatualizada/deriva) |
| `typst_library::visualize::curve` | 39+78 | 0 | 39 | 0% | — | não-iniciado | fechado | cobertura A.7 (P293/294) |
| `typst_library::visualize::gradient` | 50+54 | 7 | 43 | 14% | typst_core::entities::gradient (100%) | parcial | parcial | ADR-0087/0088/0089 (linear/radial/conic P262-272; cobertura A.7 desatualizada/deriva) |
| `typst_library::visualize::image` | 34+64 | 1 | 33 | 3% | typst_infra::export::images (100%) | parcial | fechado | cobertura A.7 (PNG/JPEG; DEBT-26..29 fechados) |
| `typst_library::visualize::image::pdf` | 13+6 | 0 | 13 | 0% | — | não-iniciado | fechado | cobertura A.7 (PNG/JPEG; DEBT-26..29 fechados) |
| `typst_library::visualize::image::raster` | 25+43 | 0 | 25 | 0% | — | não-iniciado | fechado | cobertura A.7 (PNG/JPEG; DEBT-26..29 fechados) |
| `typst_library::visualize::image::svg` | 22+3 | 0 | 22 | 0% | — | não-iniciado | fechado | cobertura A.7 (PNG/JPEG; DEBT-26..29 fechados) |
| `typst_library::visualize::line` | 7+10 | 0 | 7 | 0% | — | não-iniciado | fechado | cobertura A.7 (P285) |
| `typst_library::visualize::paint` | 4+10 | 1 | 3 | 25% | typst_core::entities::paint (100%) | parcial | fechado | cobertura A.7 + ADR-0086 (rgb/luma; solid) |
| `typst_library::visualize::polygon` | 7+11 | 0 | 7 | 0% | — | não-iniciado | fechado | cobertura A.7 (implementado) |
| `typst_library::visualize::shape` | 47+70 | 0 | 47 | 0% | — | não-iniciado | fechado | cobertura A.7 (rect/ellipse/circle; square ausente) |
| `typst_library::visualize::stroke` | 14+69 | 1 | 13 | 7% | typst_core::entities::geometry (100%) | parcial | parcial | cobertura A.7 (sem todas variantes) |
| `typst_library::visualize::tiling` | 10+17 | 0 | 10 | 0% | — | não-iniciado | pendente | cobertura A.7 (ausente) |

### typst_macros

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `typst_macros` | 6+0 | 0 | 6 | 0% | — | não-iniciado | pendente-de-confirmação | — |

### typst_pdf

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `typst_pdf` | 7+12 | 0 | 7 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::attach` | 2+0 | 0 | 2 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::convert` | 29+2 | 0 | 29 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::foundations::content::field` | 0+3 | 0 | 0 | 0% | — | só-boilerplate | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::image` | 7+8 | 0 | 7 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::layout::abs` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::layout::axes` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::layout::grid::resolve` | 0+2 | 0 | 0 | 0% | — | só-boilerplate | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::layout::point` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::layout::sides` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::layout::transform` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::link` | 5+0 | 0 | 5 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::metadata` | 6+7 | 0 | 6 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::outline` | 3+0 | 0 | 3 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::page` | 1+2 | 0 | 1 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::paint` | 12+0 | 0 | 12 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::pdf::accessibility` | 0+2 | 0 | 0 | 0% | — | só-boilerplate | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::shape` | 2+0 | 0 | 2 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::tags` | 14+1 | 0 | 14 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::tags::context` | 27+3 | 0 | 27 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::tags::context::figure` | 7+7 | 0 | 7 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::tags::context::grid` | 23+8 | 0 | 23 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::tags::context::list` | 6+2 | 0 | 6 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::tags::context::outline` | 3+2 | 0 | 3 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::tags::context::table` | 18+11 | 0 | 18 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::tags::groups` | 47+13 | 0 | 47 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::tags::resolve` | 14+5 | 1 | 13 | 7% | typst_core::entities::element_kind (100%) | parcial | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::tags::resolve::accumulator` | 9+0 | 0 | 9 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::tags::tree` | 29+7 | 0 | 29 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::tags::tree::build` | 30+6 | 0 | 30 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::tags::tree::text` | 19+20 | 0 | 19 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::tags::util` | 4+0 | 0 | 4 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::tags::util::idvec` | 13+10 | 0 | 13 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::tags::util::prop` | 3+0 | 0 | 3 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::text` | 4+8 | 0 | 4 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::util` | 10+0 | 0 | 10 | 0% | — | não-iniciado | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::visualize::shape` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |
| `typst_pdf::visualize::stroke` | 0+2 | 0 | 0 | 0% | — | só-boilerplate | parcial | diagnostico-export-passo-307a + ADR-0027/0095 (export PDF; subsetting/dedup) |

### typst_realize

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `typst_realize` | 40+0 | 0 | 40 | 0% | — | não-iniciado | fechado-consolidado | ADR-0071 (walk pipeline redesign; realize→rules::walk) |
| `typst_realize::spaces` | 4+3 | 0 | 4 | 0% | — | não-iniciado | fechado-consolidado | ADR-0071 (walk pipeline redesign; realize→rules::walk) |

### typst_render

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `typst_render` | 14+2 | 0 | 14 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_render::image` | 3+0 | 0 | 3 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_render::layout::abs` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_render::paint` | 9+4 | 0 | 9 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_render::shape` | 5+0 | 0 | 5 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_render::text` | 6+5 | 0 | 6 | 0% | — | não-iniciado | pendente-de-confirmação | — |

### typst_svg

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `typst_svg` | 49+11 | 0 | 49 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_svg::image` | 3+0 | 0 | 3 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_svg::paint` | 6+9 | 0 | 6 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_svg::path` | 14+5 | 0 | 14 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_svg::path::outline` | 1+0 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_svg::pico` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_svg::shape` | 2+0 | 0 | 2 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_svg::string` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_svg::text` | 1+1 | 0 | 1 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_svg::visualize::color` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_svg::write` | 18+8 | 0 | 18 | 0% | — | não-iniciado | pendente-de-confirmação | — |

### typst_syntax

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `typst_syntax` | 1+6 | 1 | 0 | 100% | typst_core::entities::syntax_mode (100%) | migrado | pendente-de-confirmação | — |
| `typst_syntax::ast` | 198+550 | 198 | 0 | 100% | typst_core::entities::ast::expr (34%) | migrado | fechado-consolidado | ADR-0016 (lazyhash; entities::ast) |
| `typst_syntax::highlight` | 7+0 | 0 | 7 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_syntax::kind` | 9+4 | 9 | 0 | 100% | typst_core::entities::syntax_kind (100%) | migrado | fechado | ADR-0001 (entities::syntax_kind) |
| `typst_syntax::lexer` | 50+1 | 50 | 0 | 100% | typst_core::engine::lexer (100%) | migrado | fechado-consolidado | ADR-0010/0012/0013/0014 (deps L1; rules::lexer) |
| `typst_syntax::lines` | 22+7 | 0 | 22 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_syntax::node` | 79+34 | 79 | 0 | 100% | typst_core::entities::syntax_node (100%) | migrado | fechado-consolidado | ADR-0016 (lazyhash; entities::syntax_node) |
| `typst_syntax::package` | 24+57 | 6 | 18 | 25% | typst_core::entities::package_spec (100%) | parcial | parcial | ADR-0005 (PackageSpec/World subset) |
| `typst_syntax::parser` | 112+15 | 112 | 0 | 100% | typst_core::engine::parse::parser (48%) | migrado | fechado-consolidado | ADR-0001 (estratégia; rules::parse) |
| `typst_syntax::path` | 56+34 | 3 | 53 | 5% | typst_core::entities::file_id (100%) | parcial | pendente-de-confirmação | — |
| `typst_syntax::reparser` | 7+0 | 0 | 7 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_syntax::set` | 8+2 | 8 | 0 | 100% | typst_core::entities::syntax_set (100%) | migrado | fechado | ADR-0001 (entities::syntax_set) |
| `typst_syntax::source` | 12+6 | 7 | 5 | 58% | typst_core::entities::source (100%) | parcial | fechado | ADR-0031 (early hashing em Source) |
| `typst_syntax::span` | 18+8 | 18 | 0 | 100% | typst_core::entities::span (100%) | migrado | fechado | ADR-0031 (early hashing; entities::span) |

### typst_timing

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `typst_timing` | 17+5 | 0 | 17 | 0% | — | não-iniciado | pendente-de-confirmação | — |

### typst_utils

| módulo | itens (R+bp) | par(R) | s/par(R) | % | destino dominante | mecânica | **declarado** | **fonte** |
|---|---|--:|--:|--:|---|---|---|---|
| `typst_utils` | 14+8 | 2 | 12 | 14% | typst_core::entities::math_class (50%) | parcial | pendente-de-confirmação | — |
| `typst_utils::bitset` | 8+10 | 0 | 8 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_utils::deferred` | 3+1 | 0 | 3 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_utils::duration` | 2+1 | 0 | 2 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_utils::fat` | 4+0 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_utils::hash` | 13+17 | 0 | 13 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_utils::listset` | 4+0 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_utils::macros` | 4+0 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_utils::option` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_utils::pico` | 11+16 | 0 | 11 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_utils::pico::bitcode` | 4+0 | 0 | 4 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_utils::pico::exceptions` | 3+0 | 0 | 3 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_utils::protected` | 5+2 | 0 | 5 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_utils::round` | 2+0 | 0 | 2 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_utils::scalar` | 5+45 | 0 | 5 | 0% | — | não-iniciado | pendente-de-confirmação | — |
| `typst_utils::sync` | 0+1 | 0 | 0 | 0% | — | só-boilerplate | pendente-de-confirmação | — |
| `typst_utils::version_` | 8+2 | 0 | 8 | 0% | — | não-iniciado | pendente-de-confirmação | — |

## 3. Censo inverso — módulos do cristalino sem-par (o que é novo)

Só censo (sem coluna declarada): itens do cristalino sem correspondente no vanilla, por módulo.

| módulo cristalino | itens novos (R+bp) |
|---|---|
| `typst_core::entities::args` | 1+0 |
| `typst_core::entities::ast` | 1+0 |
| `typst_core::entities::ast::markup` | 1+0 |
| `typst_core::entities::bib_entry` | 14+4 |
| `typst_core::entities::bib_store` | 10+3 |
| `typst_core::entities::citation_form` | 1+0 |
| `typst_core::entities::color` | 8+0 |
| `typst_core::entities::content` | 26+0 |
| `typst_core::entities::content_hash` | 2+0 |
| `typst_core::entities::corners` | 1+0 |
| `typst_core::entities::counter_registry` | 12+3 |
| `typst_core::entities::dir` | 3+1 |
| `typst_core::entities::element_info` | 3+4 |
| `typst_core::entities::element_kind` | 2+2 |
| `typst_core::entities::element_payload` | 1+4 |
| `typst_core::entities::font_book` | 4+2 |
| `typst_core::entities::font_list` | 4+0 |
| `typst_core::entities::func` | 9+0 |
| `typst_core::entities::geometry` | 5+3 |
| `typst_core::entities::glyph_variants` | 14+18 |
| `typst_core::entities::gradient` | 37+10 |
| `typst_core::entities::image_sizer` | 2+1 |
| `typst_core::entities::introspector` | 4+31 |
| `typst_core::entities::label_registry` | 9+3 |
| `typst_core::entities::layout_types` | 35+45 |
| `typst_core::entities::layouter_runtime_state` | 1+3 |
| `typst_core::entities::location` | 2+0 |
| `typst_core::entities::locator` | 2+1 |
| `typst_core::entities::math_class` | 2+3 |
| `typst_core::entities::math_constants` | 2+0 |
| `typst_core::entities::math_style` | 7+4 |
| `typst_core::entities::metadata_store` | 6+3 |
| `typst_core::entities::module` | 1+0 |
| `typst_core::entities::package_spec` | 3+8 |
| `typst_core::entities::page_store` | 8+3 |
| `typst_core::entities::paint` | 3+0 |
| `typst_core::entities::position` | 1+1 |
| `typst_core::entities::ptr_eq_arc` | 1+6 |
| `typst_core::entities::regex` | 3+2 |
| `typst_core::entities::region` | 8+0 |
| `typst_core::entities::resolved_label_store` | 6+3 |
| `typst_core::entities::scope` | 5+0 |
| `typst_core::entities::sealed_positions` | 6+12 |
| `typst_core::entities::sealed_positions::__ComemoSurface` | 1+0 |
| `typst_core::entities::sealed_positions::__ComemoSurfaceMut` | 1+0 |
| `typst_core::entities::show` | 1+1 |
| `typst_core::entities::sides` | 1+0 |
| `typst_core::entities::sink` | 4+4 |
| `typst_core::entities::source` | 3+2 |
| `typst_core::entities::state_registry` | 8+3 |
| `typst_core::entities::style` | 2+1 |
| `typst_core::entities::style_chain` | 19+5 |
| `typst_core::entities::syntax_node` | 1+0 |
| `typst_core::entities::syntax_text` | 5+10 |
| `typst_core::entities::value` | 9+18 |
| `typst_core::entities::world_types` | 12+12 |
| `typst_core::entities::world_types::__ComemoSurfaceMut` | 1+0 |
| `typst_core::engine::eval` | 7+0 |
| `typst_core::engine::eval::bindings` | 4+0 |
| `typst_core::engine::eval::closures` | 5+0 |
| `typst_core::engine::eval::control_flow` | 3+0 |
| `typst_core::engine::eval::markup` | 8+0 |
| `typst_core::engine::eval::math` | 3+0 |
| `typst_core::engine::eval::modules` | 2+0 |
| `typst_core::engine::eval::operators` | 2+0 |
| `typst_core::engine::eval::rules` | 6+0 |
| `typst_core::engine::introspect` | 8+0 |
| `typst_core::engine::introspect::convergence` | 1+0 |
| `typst_core::engine::introspect::extract_payload` | 1+0 |
| `typst_core::engine::introspect::fixpoint` | 3+1 |
| `typst_core::engine::introspect::from_tags` | 3+0 |
| `typst_core::engine::introspect::locatable` | 1+0 |
| `typst_core::engine::lang::figure_supplement` | 2+0 |
| `typst_core::engine::lang::quotes` | 1+0 |
| `typst_core::engine::layout` | 32+6 |
| `typst_core::engine::layout::figure` | 1+0 |
| `typst_core::engine::layout::grid` | 1+0 |
| `typst_core::engine::layout::grid_placement` | 5+3 |
| `typst_core::engine::layout::helpers` | 7+0 |
| `typst_core::engine::layout::hyphenation` | 1+0 |
| `typst_core::engine::layout::image` | 3+0 |
| `typst_core::engine::layout::metrics` | 2+2 |
| `typst_core::engine::layout::outline` | 1+0 |
| `typst_core::engine::layout::references` | 2+0 |
| `typst_core::engine::layout::slicing` | 3+0 |
| `typst_core::engine::lexer::scanner` | 25+6 |
| `typst_core::engine::lexer::scanner::sealed` | 1+0 |
| `typst_core::engine::math::layout` | 30+2 |
| `typst_core::engine::math::symbols` | 6+0 |
| `typst_core::engine::parse::patterns` | 1+0 |
| `typst_core::engine::scopes` | 5+0 |
| `typst_core::engine::stdlib` | 2+0 |
| `typst_core::engine::stdlib::assert` | 1+0 |
| `typst_core::engine::stdlib::calc` | 48+0 |
| `typst_core::engine::stdlib::figure_image` | 3+0 |
| `typst_core::engine::stdlib::foundations` | 30+0 |
| `typst_core::engine::stdlib::gradients` | 8+0 |
| `typst_core::engine::stdlib::layout` | 29+0 |
| `typst_core::engine::stdlib::math_style` | 13+0 |
| `typst_core::engine::stdlib::shapes` | 8+0 |
| `typst_core::engine::stdlib::structural` | 29+0 |
| `typst_core::engine::stdlib::text` | 9+1 |
| `typst_core::engine::stdlib::transforms` | 4+0 |
| `typst_core::utils` | 1+4 |
| `typst_infra::export` | 2+0 |
| `typst_infra::export::builder` | 11+0 |
| `typst_infra::export::fonts` | 7+0 |
| `typst_infra::export::gradients` | 10+8 |
| `typst_infra::export::gradients::adaptive` | 2+0 |
| `typst_infra::export::gradients::cmyk` | 3+0 |
| `typst_infra::export::gradients::conic` | 6+0 |
| `typst_infra::export::gradients::function_dict` | 2+0 |
| `typst_infra::export::gradients::linear` | 2+0 |
| `typst_infra::export::gradients::radial` | 2+0 |
| `typst_infra::export::gradients::relative` | 2+0 |
| `typst_infra::export::images` | 13+0 |
| `typst_infra::export::stream` | 13+0 |
| `typst_infra::font_metrics` | 5+7 |
| `typst_infra::fonts` | 8+0 |
| `typst_infra::image_sizer` | 1+1 |
| `typst_infra::layout` | 1+0 |
| `typst_infra::measurements` | 12+36 |
| `typst_infra::pipeline` | 8+0 |
| `typst_infra::query_helpers` | 8+11 |
| `typst_infra::world` | 10+11 |
| `typst_shell::cli` | 6+15 |
| `typst_shell::diagnostic` | 1+0 |

Total cristalino sem-par: 850 reais + 353 bp.

## 4. Discordâncias — o detector de deriva

Módulos onde mecânica e declarada conflitam, nos dois sentidos. Módulos `fechado-consolidado` com mecânica `não-iniciado` **não** entram (a citação estrutural — ex.: ADR-0026 — É a nota de consolidação que explica o desvio). Módulos `pendente-de-confirmação` também não entram (são incógnitas honestas, não contradições).

**Leitura desta secção**: §4.1 tem **60** entradas, §4.2 tem **0**. A massa de §4.1 é o **fingerprint esperado da reescrita** Content-enum/`native_*`: features que a cobertura declara implementadas mas cujos tipos foram **renomeados** (Elem struct → variant de enum), logo a lente — que pareia por nome — vê 0 pareados. Não são regressões presumidas; cada uma é candidata a **uma nota de consolidação de uma linha** (promover a `fechado-consolidado` citando o passo/ADR que a materializou) — ou, se a nota não puder ser escrita, à verificação de que a feature existe mesmo em L1.

§4.2 está vazia, e isso é **uma limitação reportada, não um resultado**: a lente pareia por nome de símbolo; uma feature reescrita-e-renomeada nunca pareia, logo o sinal "mecânica `migrado` mas declaração atrasada" (a classe exacta da deriva P284–P311b) é **invisível** a este detector dentro de `typst_library`. O único conjunto que pareia mecanicamente é `typst_syntax` (nomes preservados), que está declarado. Conclusão honesta: para reescrita, o cruzamento útil é §4.1 + a coluna *destino dominante*, não §4.2.

### 4.1 — Declarado `fechado` mas mecânica `não-iniciado` (ou a declaração está errada, ou falta a nota de consolidação)

| módulo | declarado | mecânica | fonte declarada | a confirmar |
|---|---|---|---|---|
| `typst_library::foundations::assert` | fechado | não-iniciado | cobertura A.8 (native_assert) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::foundations::auto` | fechado | não-iniciado | cobertura B.1 (Value::Auto) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::foundations::calc` | fechado | não-iniciado | cobertura A.8 + diagnostico-calc-passo-283 (74%) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::foundations::float` | fechado | não-iniciado | cobertura A.8 (constructor) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::foundations::int` | fechado | não-iniciado | cobertura A.8 (constructor) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::foundations::label` | fechado | não-iniciado | cobertura A.1 (Content::Labelled) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::foundations::none` | fechado | não-iniciado | cobertura B.1 (Value::None) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::foundations::ops` | fechado | não-iniciado | cobertura A.8 (control flow) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::foundations::ty` | fechado | não-iniciado | cobertura A.8 (type()) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::introspection::convergence` | fechado | não-iniciado | ADR-0072 (m7 fixpoint runtime fechado) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::introspection::here_` | fechado | não-iniciado | cobertura A.9 (here/locate P208) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::introspection::locate_` | fechado | não-iniciado | cobertura A.9 (P208C) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::introspection::metadata` | fechado | não-iniciado | cobertura A.9 (P169) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::introspection::query_` | fechado | não-iniciado | cobertura A.9 (implementado⁺ P209) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::introspection::tag` | fechado | não-iniciado | ADR-0069 (post-recursion tag emission) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::layout::align` | fechado | não-iniciado | cobertura A.5 (implementado) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::layout::container` | fechado | não-iniciado | cobertura A.5 (box/block P250/252 completos) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::layout::container::callbacks` | fechado | não-iniciado | cobertura A.5 (box/block P250/252 completos) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::layout::grid` | fechado | não-iniciado | cobertura A.5 (implementado⁺ P224-230) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::layout::grid::resolve` | fechado | não-iniciado | cobertura A.5 (implementado⁺ P224-230) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::layout::hide` | fechado | não-iniciado | cobertura A.5 (P156C) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::layout::pad` | fechado | não-iniciado | cobertura A.5 (implementado⁺ P156C/L) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::layout::place` | fechado | não-iniciado | cobertura A.5 (implementado⁺ P223) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::layout::repeat` | fechado | não-iniciado | cobertura A.5 (P156J) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::layout::spacing` | fechado | não-iniciado | cobertura A.5 (h/v P156D) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::layout::stack` | fechado | não-iniciado | cobertura A.5 (P156I) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::layout::transform` | fechado | não-iniciado | cobertura A.5 (rotate/scale/move/skew P78) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::math::accent` | fechado | não-iniciado | cobertura A.4 (P296) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::math::attach` | fechado | não-iniciado | cobertura A.4 (implementado) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::math::cancel` | fechado | não-iniciado | cobertura A.4 (P296) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::math::equation` | fechado | não-iniciado | cobertura A.4 (implementado) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::math::frac` | fechado | não-iniciado | cobertura A.4 (implementado) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::math::lr` | fechado | não-iniciado | cobertura A.4 (P76) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::math::matrix` | fechado | não-iniciado | cobertura A.4 (matrix/cases impl⁺ P79) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::math::op` | fechado | não-iniciado | diagnostico-math-op-passo-298 (P298/299) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::math::root` | fechado | não-iniciado | cobertura A.4 (implementado⁺) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::math::style` | fechado | não-iniciado | ADR-0102/0103 (math style P311a) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::math::underover` | fechado | não-iniciado | cobertura A.4 (P297) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::model::divider` | fechado | não-iniciado | cobertura A.6 (P154B) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::model::emph` | fechado | não-iniciado | cobertura A.1 (implementado) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::model::figure` | fechado | não-iniciado | cobertura A.6 (implementado⁺ P158) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::model::heading` | fechado | não-iniciado | cobertura A.6 (implementado) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::model::outline` | fechado | não-iniciado | cobertura A.6 (implementado) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::model::quote` | fechado | não-iniciado | cobertura A.6 (P155) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::model::reference` | fechado | não-iniciado | cobertura A.6 (implementado⁺) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::model::strong` | fechado | não-iniciado | cobertura A.1 (implementado) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::model::terms` | fechado | não-iniciado | cobertura A.6 (P154B) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::text::case` | fechado | não-iniciado | cobertura A.3 (upper/lower/replace) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::text::deco` | fechado | não-iniciado | cobertura A.3 (P284-286 underline/strike/overline) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::text::font::color` | fechado | não-iniciado | cobertura A.3 + ADR-0053 (string/array/multi-doc; dict scope-out) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::text::font::exceptions` | fechado | não-iniciado | cobertura A.3 + ADR-0053 (string/array/multi-doc; dict scope-out) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::text::raw` | fechado | não-iniciado | cobertura A.1/A.3 (implementado) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::text::smartquote` | fechado | não-iniciado | cobertura A.1 (P287) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::visualize::curve` | fechado | não-iniciado | cobertura A.7 (P293/294) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::visualize::image::pdf` | fechado | não-iniciado | cobertura A.7 (PNG/JPEG; DEBT-26..29 fechados) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::visualize::image::raster` | fechado | não-iniciado | cobertura A.7 (PNG/JPEG; DEBT-26..29 fechados) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::visualize::image::svg` | fechado | não-iniciado | cobertura A.7 (PNG/JPEG; DEBT-26..29 fechados) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::visualize::line` | fechado | não-iniciado | cobertura A.7 (P285) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::visualize::polygon` | fechado | não-iniciado | cobertura A.7 (implementado) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |
| `typst_library::visualize::shape` | fechado | não-iniciado | cobertura A.7 (rect/ellipse/circle; square ausente) | a migração ocorreu sob nome novo (→ marcar `fechado-consolidado` com destino) ou a feature não está em L1? |

### 4.2 — Declarado `pendente`/`parcial` mas mecânica `migrado` (o trabalho aconteceu e ninguém declarou — a classe P284–P311b)

_Nenhuma._

## 5. Evidências para o veredito (sem veredito)

Números que a decisão "está valendo / o que fazer" precisa — **sem responder por ela**.

### Estrutural

- **Maior SCC (ciclo)**: vanilla **203** → cristalino **15** (JSON da lente, `ciclos_antes`/`ciclos_depois`). Nº de ciclos: 11 → 5.
- **Crates**: 21 (vanilla) → 4 (cristalino).
- **`crystalline-lint .`** (rodado P312, branch `Tekt`): **3 violations V9** (`ForbiddenImport`/vazamento de encapsulamento) em L3 — `03_infra/src/font_metrics.rs:10` e `03_infra/src/layout.rs:7` importam subdiretórios internos de L1 em vez das portas `[l1_ports]`. **NÃO é zero** nesta branch. (O critério de sucesso do projeto é zero; a branch `Tekt` ainda não o cumpre — facto reportado, não corrigido aqui.)

### Funcional

- **Cobertura declarada** (tabela de cobertura): user-facing (impl+impl⁺) ≈ **61%** de 141 entradas; medição global declarada **70,9%** vs real estimada **~73,1%** (`diagnostico-bloqueio-processo-2026-06-09.md` §F4, números de 2026-05-19). **Ressalva**: a tabela está sincronizada só até P283; deriva conhecida P284–P311b.
- **Testes**: última contagem documentada **2 899+** (`diagnostico-bloqueio-processo-2026-06-09.md` §C3, 2026-06-09). `cargo test --workspace` não foi corrido neste passo (compilação pesada; fora do orçamento de tempo do mapa).
- **ADRs vigentes**: 86+ (idem §C3).

### O "falta honesto"

Módulos com mecânica `não-iniciado` **e** declarado `pendente`/`pendente-de-confirmação` (não migrados e não declarados como fechados): **179** módulos. Por crate:

| crate | módulos falta-honesto |
|---|--:|
| test_wrapper | 1 |
| typst | 1 |
| typst_bundle | 3 |
| typst_docs | 5 |
| typst_fuzz | 1 |
| typst_html | 14 |
| typst_ide | 11 |
| typst_kit | 6 |
| typst_layout | 45 |
| typst_library | 60 |
| typst_macros | 1 |
| typst_render | 5 |
| typst_svg | 8 |
| typst_syntax | 3 |
| typst_timing | 1 |
| typst_utils | 14 |

> A maioria são tipos/infra internos do vanilla sem declaração de escopo no projeto (não confundir com regressão): a sua ausência de fonte é precisamente o que `pendente-de-confirmação` regista.

### O passivo (DEBT.md — uma linha por item em aberto)

- **DEBT-2** — Closures eager vs lazy capture — PARCIALMENTE RESOLVIDO.
- **DEBT-9** — Cobertura de paridade — tracking contínuo (meta-débito).
- **DEBT-42** — `get_unchecked` no scanner — EM ABERTO (bloqueado).
- **DEBT-43** — Linter: whitelist crate-level vs type-level — EM ABERTO.
- **DEBT-50** — Show selector Strong/Emph não distingue origem — EM ABERTO.
- **DEBT-55** — Bibliography + Cite (pré-condição hayagriva) — PARCIALMENTE RESOLVIDO (P258; paridade manual P159A-G).
  (Restantes DEBT no ficheiro estão ENCERRADOS/RESOLVIDOS.)

### Ressalvas permanentes (não são deriva — são limites da medição)

- **`typst_macros`**: a lente extrai só 6 itens reais (`typst_macros` no censo) — falha de extração conhecida de proc-macros; o crate de macros do vanilla está sub-representado.
- **Itens transformados de kind** (ex.: a hierarquia vtable de `Content` → enum fechado): não pareiam **por definição da chave** (`path_completo`), porque mudaram de forma de propósito (ADR-0026). Aparecem como sem-par dos dois lados; é consolidação, não ausência.

---

## Manutenção

- **Coluna mecânica + censo + rollup + portão**: regenerar com `python3 lab/mapa-migracao/gerar.py <json>` e substituir. Nunca editar à mão.
- **Coluna declarada + fonte**: só muda citando o passo/ADR que mudou o estado. Uma marcação sem fonte é um bug do documento — usar `pendente-de-confirmação`.
- **Discordâncias (§4)**: recalculam-se ao cruzar as duas colunas; cada entrada é uma tarefa de reconciliação (corrigir a declaração ou escrever a nota de consolidação).
