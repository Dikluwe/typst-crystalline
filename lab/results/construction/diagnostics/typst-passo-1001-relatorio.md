# Passo 1001 — Auditoria dos 10 prompts órfãos: conteúdo real e localização no vanilla

**Tipo**: Auditoria — ler, catalogar, nada presumir, nada corrigir.  
**Estado da árvore no início**: `git status --short` mostrou ficheiros não rastreados (untracked), mas **zero modificações em ficheiros rastreados**. Lista de untracked ao início: `00_nucleo/diagnosticos/typst-dsm.html`, `typst-passo-999-relatorio.md`, `typst-passo-1000-relatorio.md`, `00_nucleo/materialization/typst-passo-1000.md`, `00_nucleo/materialization/typst-passo-1001.md`, `00_nucleo/materialization/typst-passo-999.md`, `00_nucleo/prompts/auditar-spec.md`, `test_crystalline.pdf`, `test_oracle.pdf`, `test_vanilla.pdf`.  
**Commit de referência**: `00dc949665317dedabc5ec01f0af5f4f4dc6cc80`.

---

## Parte 1 — Leitura literal de cada prompt

### 1. `engine/eval/table.md`
- **Título**: Prompt L0 — `rules/eval/table` — `#set table(numbering:)` e namespace `table.*`
- **Secções**: 1. `#set table(numbering: ...)`, 2. `native_table`, 3. Namespace anexado `table.*`, 4. Limitação conhecida (P661), 5. Verificação.
- **Instrução**: implementar numeração automática de tabelas via `StyleChain` (`table.numbering`), `native_table` com `caption`, e namespace anexado `table.header`/`table.footer`/`table.cell`/`table.hline`/`table.vline`.
- **Alvo citado**: `01_core/src/engine/eval/rules.rs` (arm `"table"`) e `01_core/src/engine/stdlib/structural.rs`.
- **Hash**: `PENDENTE_HUMAN_CALC`.

### 2. `engine/eval/decimal-arithmetic.md`
- **Título**: Prompt L0 — `rules/eval/decimal-arithmetic` — operadores `Decimal`.
- **Secções**: 1. Contexto, 2. Escopo, 3. Implementação, 4. Paridade vanilla, 5. Testes, 6. Scope-out.
- **Instrução**: adicionar operadores `+`, `-`, `*`, `/` e comparações homogéneas entre `Value::Decimal` em `eval_binary_op` em `01_core/src/engine/eval/operators.rs`.
- **Alvo citado**: `01_core/src/engine/eval/operators.rs`.
- **Hash**: `1e867e48`.

### 3. `engine/show-regex.md`
- **Título**: Prompt L0 — `show-regex` — wiring de `#show regex(...)`.
- **Secções**: 1. Contexto, 2. Arquitetura, 3. Construtor `regex(pattern)`, 4. Show rule selector, 5. Aplicação de show-rules regex, 6. Paridade vanilla, 7. Testes, 8. Scope-out.
- **Instrução**: tornar `regex(...)` construtível, permitir `#show regex(...): ...`, e aplicar transformação a nós de texto que casem.
- **Alvos citados**: `01_core/src/entities/value.rs`, `01_core/src/entities/show.rs`, `01_core/src/engine/stdlib/text.rs` (ou módulo regex dedicado), `01_core/src/engine/eval/mod.rs`, `01_core/src/engine/eval/rules.rs`.
- **Hash**: `a3f2c53b`.

### 4. `engine/stdlib/grid_hline.md`
- **Título**: Prompt L0 — `stdlib/grid_hline` — função `grid.hline()`.
- **Secções**: Assinatura, Semântica, Implementação, Casos de Aceitação, Nota de implementação.
- **Instrução**: implementar `native_grid_hline` em `01_core/src/engine/stdlib/structural.rs` (ou módulo grid/table), criando `Content::GridHLine`.
- **Alvo citado**: `01_core/src/engine/stdlib/structural.rs`.
- **Hash**: não consta.

### 5. `engine/stdlib/grid_vline.md`
- **Título**: Prompt L0 — `stdlib/grid_vline` — função `grid.vline()`.
- **Secções**: idênticas a `grid_hline.md`.
- **Instrução**: implementar `native_grid_vline` em `structural.rs`, criando `Content::GridVLine`.
- **Alvo citado**: `01_core/src/engine/stdlib/structural.rs`.
- **Hash**: não consta.

### 6. `engine/stdlib/table_hline.md`
- **Título**: Prompt L0 — `stdlib/table_hline` — função `table.hline()`.
- **Secções**: idênticas às de grid_hline.
- **Instrução**: implementar `native_table_hline` em `structural.rs`, criando `Content::TableHLine`.
- **Alvo citado**: `01_core/src/engine/stdlib/structural.rs`.
- **Hash**: não consta.

### 7. `engine/stdlib/table_vline.md`
- **Título**: Prompt L0 — `stdlib/table_vline` — função `table.vline()`.
- **Secções**: idênticas às de grid_hline.
- **Instrução**: implementar `native_table_vline` em `structural.rs`, criando `Content::TableVLine`.
- **Alvo citado**: `01_core/src/engine/stdlib/structural.rs`.
- **Hash**: `dc48f596`.

### 8. `engine/style/font-dict.md`
- **Título**: Prompt L0 — `rules/style/font-dict`.
- **Secções**: Contexto, Interface pública, Representação na chain custom, Erros, Scope-out, Critérios de Verificação.
- **Instrução**: no arm `"font"` de `eval_set_rule` em `01_core/src/engine/eval/rules.rs`, aceitar dict legado (chave Str/Regex → variants) e dict named fields (`family`, `variant`, `weight`, `style`, `fallback`).
- **Alvo citado**: `01_core/src/engine/eval/rules.rs`.
- **Hash**: `b307d85a`.

### 9. `infra/package_version_resolution.md`
- **Título**: Prompt L0 — `infra/package_version_resolution` — Resolução de versão implícita de pacotes.
- **Secções**: Contexto, Objetivo, Decisões arquiteturais (1–5), Restrições estruturais, Critérios de verificação.
- **Instrução**: definir como resolver versão omitida em `@preview/nome` e namespaces locais, com cache em memória no `SystemWorld`; ponto de entrada em `03_infra/src/world.rs::resolve_package` e lógica remota partilhada com `package_downloader.md`.
- **Alvos citados**: `03_infra/src/package_downloader.rs` (futuro), alterações em `03_infra/src/world.rs`.
- **Hash**: `caeadbbd`.

### 10. `package-spec-dto.md`
- **Título**: PackageSpecDto — DTO de serialização para PackageSpec.
- **Secções**: Contexto, Restrições Estruturais, Tipos, Critérios de Verificação, Histórico de Revisões.
- **Instrução**: criar `03_infra/src/dto/package_spec_dto.rs` com `PackageSpecDto` (serde), `TryFrom` para `PackageSpec`, e `FromStr` para `PackageSpec` / `VersionlessPackageSpec` usando `unscanny`, isolando `serde` em L3.
- **Alvo citado**: `03_infra/src/dto/package_spec_dto.rs`.
- **Hash**: não consta.

---

## Parte 2 — Causa da orfandade, caso a caso

### 1. `engine/eval/table.md`
- **Ficheiros alvo existem?** Sim (`rules.rs`, `structural.rs`).
- **O que citam em vez deste prompt?**
  - `01_core/src/engine/eval/rules.rs`: `@prompt 00_nucleo/prompts/engine/eval.md` (`96691e96`).
  - `01_core/src/engine/stdlib/structural.rs`: `@prompt 00_nucleo/prompts/engine/stdlib/structural.md` (`929f2e94`).
- **Conteúdo implementado?** Sim. `native_table`, `native_table_header/footer/cell/hline/vline`, `table.numbering` e registo do namespace `table.*` existem em `structural.rs`, `eval/mod.rs` e `eval/rules.rs`.
- **Causa**: o código foi materializado, mas os headers dos ficheiros foram selados com os prompts agregadores (`eval.md`, `structural.md`), não com este prompt específico.

### 2. `engine/eval/decimal-arithmetic.md`
- **Ficheiro alvo existe?** Sim (`operators.rs`).
- **O que cita?** `@prompt 00_nucleo/prompts/engine/eval/ops.md` (`140349c4`).
- **Conteúdo implementado?** Sim. Todos os braços `Value::Decimal` para `Add`, `Sub`, `Mul`, `Div`, comparações e `Neg` existem em `operators.rs`.
- **Causa**: a aritmética decimal foi fundida no prompt agregador `ops.md`; o ficheiro não cita `decimal-arithmetic.md`.

### 3. `engine/show-regex.md`
- **Ficheiros alvo existem?** Sim.
- **O que citam?**
  - `entities/value.rs`: `entities/value.md` (`5bf2ca7d`).
  - `entities/show.rs`: `entities/show.md` (`d3381e55`).
  - `engine/stdlib/text.rs`: `engine/stdlib/text.md` (`d55f9c11`).
  - `engine/eval/mod.rs`: `engine/eval.md` (`96691e96`).
  - `engine/eval/rules.rs`: `engine/eval.md` (`96691e96`).
- **Conteúdo implementado?** Sim. `Value::Regex`, `Selector::Regex`, `native_regex`, wiring em `eval_show_rule` e aplicação em `apply_show_rules` estão todos presentes.
- **Causa**: a funcionalidade regex/show foi distribuída pelos prompts agregadores de cada módulo (`value.md`, `show.md`, `text.md`, `eval.md`); nenhum ficheiro cita `show-regex.md`.

### 4. `engine/stdlib/grid_hline.md`
- **Ficheiro alvo existe?** Sim (`structural.rs`).
- **O que cita?** `engine/stdlib/structural.md` (`929f2e94`).
- **Conteúdo implementado?** Sim. `native_grid_hline`, `Content::GridHLine`, e entidade `GridHLineElem` existem.
- **Causa**: a função foi agrupada no prompt/ficheiro `structural.md`; o prompt específico `grid_hline.md` não é citado.

### 5. `engine/stdlib/grid_vline.md`
- **Ficheiro alvo existe?** Sim (`structural.rs`).
- **O que cita?** `engine/stdlib/structural.md` (`929f2e94`).
- **Conteúdo implementado?** Sim. `native_grid_vline`, `Content::GridVLine`, `GridVLineElem` existem.
- **Causa**: idêntico a `grid_hline.md`.

### 6. `engine/stdlib/table_hline.md`
- **Ficheiro alvo existe?** Sim (`structural.rs`).
- **O que cita?** `engine/stdlib/structural.md` (`929f2e94`).
- **Conteúdo implementado?** Sim. `native_table_hline`, `Content::TableHLine`, `TableHLineElem` existem.
- **Causa**: idêntico aos anteriores.

### 7. `engine/stdlib/table_vline.md`
- **Ficheiro alvo existe?** Sim (`structural.rs`).
- **O que cita?** `engine/stdlib/structural.md` (`929f2e94`).
- **Conteúdo implementado?** Sim. `native_table_vline`, `Content::TableVLine`, `TableVLineElem` existem.
- **Causa**: idêntico aos anteriores.

### 8. `engine/style/font-dict.md`
- **Ficheiro alvo existe?** Sim (`rules.rs`).
- **O que cita?** `engine/eval.md` (`96691e96`).
- **Conteúdo implementado?** Sim. `parse_font_dict_named_fields`, `parse_font_dict_legacy`, mensagens de erro e representação na chain `"text.font"` existem em `rules.rs`.
- **Causa**: a funcionalidade foi integrada no prompt agregador `eval.md`; o ficheiro não cita `font-dict.md`.

### 9. `infra/package_version_resolution.md`
- **Ficheiros alvo existem?** Sim (`package_downloader.rs`, `world.rs`).
- **O que citam?**
  - `package_downloader.rs`: `infra/package_downloader.md` (`b6450b35`).
  - `world.rs`: `infra/system-world.md` (`89eec3cb`).
- **Conteúdo implementado?** Parcialmente/sim. `HttpPackageDownloader::latest_version_remote`, `SystemWorld::resolve_package` e cache do downloader existem. A lógica de resolução de versão implícita está concentrada no `package_downloader.rs`/`world.rs`.
- **Causa**: a resolução de versão foi materializada sob os prompts agregadores `package_downloader.md` e `system-world.md`; `package_version_resolution.md` não é citado.

### 10. `package-spec-dto.md`
- **Ficheiro alvo existe?** **Não**. `03_infra/src/dto/package_spec_dto.rs` não existe; não existe sequer o diretório `03_infra/src/dto/`.
- **Conteúdo implementado de outra forma?** Sim, mas num local diferente e com violação da intenção do DTO:
  - `PackageSpec::from_str` e `VersionlessPackageSpec::from_str` estão implementados diretamente em `01_core/src/entities/package_spec.rs` (L1), selado com `entities/package-spec.md` (`f09d78d5`).
  - Em L3, `package_downloader.rs` usa `PackageSpec` via `use typst_core::entities::package_spec::{PackageSpec, PackageVersion}`.
- **Causa**: o DTO nunca foi criado. A serialização/parse de `PackageSpec` foi implementada diretamente em L1, contrariando a separação pretendida (L1 puro, L3 com `serde`/`unscanny`).

---

## Parte 3 — Onde vive no vanilla

### 1. `engine/eval/table.md`
- **Crate/ficheiros**: `typst-library/src/model/table.rs` (elemento `Table`, `TableHeader`, `TableFooter`, `TableCell`, `TableHLine`, `TableVLine`); `typst-library/src/layout/grid/mod.rs` (Grid partilha infraestrutura com Table); `typst-eval/src/rules.rs` (set/show rules).
- **Nível**: a definição dos elementos table e seus sub-elementos vive em `typst-library` (model + layout), não isoladamente em `typst-eval`. O eval do vanilla (`typst-eval`) trata das regras de set/show, mas os elementos são da library.
- **Nota**: no cristalino, a separação é diferente — `native_table` e subfunções estão em `engine/stdlib/structural.rs` (L1) e o wiring de namespace/numbering em `engine/eval` (L1).

### 2. `engine/eval/decimal-arithmetic.md`
- **Crate/ficheiros**: `typst-library/src/foundations/ops.rs` (operações binárias/unárias); `typst-library/src/foundations/decimal.rs` (tipo Decimal).
- **Nível**: operações de Decimal no eval vanilla ficam em `foundations/ops.rs`, dentro da library, não num crate `eval` separado.
- **Nota**: o vanilla suporta coerção `Decimal ↔ Int` (contrariando o scope-out do prompt cristalino). O cristalino mantém aritmética homogénea sem coerção.

### 3. `engine/show-regex.md`
- **Crate/ficheiros**: `typst-library/src/foundations/selector.rs` (`Selector::Regex`, `Value::Regex`); `typst-realize/src/lib.rs` (aplicação de show-rules regex a nós de texto, com split interno por match).
- **Nível**: o selector regex é em `foundations` (library); a aplicação da show-rule regex é em `typst-realize` (render/realize), não em `typst-eval`.
- **Nota**: no cristalino, a aplicação está em `engine/eval/rules.rs::apply_show_rules` (ainda dentro do pipeline de eval, antes do layout).

### 4–7. `grid_hline.md`, `grid_vline.md`, `table_hline.md`, `table_vline.md`
- **Crate/ficheiros**: `typst-library/src/layout/grid/mod.rs` (`GridHLine`, `GridVLine`, e enum `GridItem::HLine/VLine`); `typst-library/src/model/table.rs` (`TableHLine`, `TableVLine`, e enum `TableItem::HLine/VLine`).
- **Nível**: elementos de linha vivem lado a lado com os elementos grid/table em `typst-library`, partilhando resolução em `layout/grid/resolve.rs` e renderização em `typst-layout/src/grid/lines.rs`.
- **Nota**: no cristalino, as linhas são entidades L1 (`GridHLineElem`, `TableHLineElem`, etc.) e a separação células/linhas é feita em `engine/stdlib/structural.rs` e `engine/stdlib/layout.rs`.

### 8. `engine/style/font-dict.md`
- **Crate/ficheiros**: pesquisa por `font.*dict`, `FontNamePattern`, `parse_font` no vanilla não devolveu resultados. A fonte no vanilla é tratada principalmente em `typst-library/src/text/font/` (book, info, variant) e em `typst-library/src/text/mod.rs`.
- **Nível**: não há evidência de um dict de fonte com a forma do prompt cristalino no vanilla 0.15.0. A seleção de fonte no vanilla usa `FontBook`/`FontVariant` e listas de famílias.
- **Nota**: este prompt descreve uma funcionalidade que pode ser uma extensão/paridade cristalina específica, não um mecanismo idêntico no vanilla.

### 9. `infra/package_version_resolution.md`
- **Crate/ficheiros**: `typst-kit/src/packages.rs` (`latest_version`, resolução remota/local, `VersionlessPackageSpec`); `typst-cli/src/init.rs` (uso em `typst init`); `typst-syntax/src/package.rs` (`PackageSpec`, `VersionlessPackageSpec`, `FromStr`).
- **Nível**: a resolução de versão implícita vive em `typst-kit` (package management), não no eval. O CLI `init` é o principal consumidor.
- **Nota**: no cristalino, a resolução está em `03_infra/src/package_downloader.rs` + `03_infra/src/world.rs` (ambos L3), o que espelha a separação vanilla (kit + CLI).

### 10. `package-spec-dto.md`
- **Crate/ficheiros**: `typst-syntax/src/package.rs` (`PackageSpec`, `VersionlessPackageSpec`, `FromStr`, `Display`).
- **Nível**: no vanilla, o parse de `PackageSpec` a partir de string vive em `typst-syntax` (crate de sintaxe), não num DTO de infraestrutura.
- **Nota**: o cristalino implementou o parse diretamente em `01_core/src/entities/package_spec.rs` (L1), em vez de um DTO em L3. O vanilla usa `typst-syntax`; o cristalino não tem crate equivalente, daí a decisão de DTO, mas essa decisão nunca foi materializada.

---

## Parte 4 — Catálogo final

| # | Prompt | Conteúdo real (resumo da Instrução) | Causa da orfandade | Onde vive no cristalino hoje | Onde vive no vanilla | Nota |
|---|---|---|---|---|---|---|
| 1 | `engine/eval/table.md` | `#set table(numbering:)`, `native_table` com `caption`, namespace `table.*` (header/footer/cell/hline/vline). | Código implementado, mas ficheiros selados com prompts agregadores (`eval.md`, `structural.md`). | `engine/eval/rules.rs` (numbering), `engine/eval/mod.rs` (registo namespace), `engine/stdlib/structural.rs` (native_table e subfunções), `entities/elements/table*.rs`. | `typst-library/src/model/table.rs` (elementos); `typst-library/src/layout/grid/` (resolução partilhada); `typst-eval/src/rules.rs` (set/show). | A separação eval vs library é diferente; no cristalino tudo está em L1. |
| 2 | `engine/eval/decimal-arithmetic.md` | Operadores e comparações homogéneas `Decimal op Decimal`. | Funcionalidade fundida no prompt agregador `ops.md`. | `engine/eval/operators.rs` (`eval_binary_op`, `eval_unary_op`). | `typst-library/src/foundations/ops.rs`; `typst-library/src/foundations/decimal.rs`. | Vanilla faz coerção Decimal↔Int; cristalino não. |
| 3 | `engine/show-regex.md` | Construtor `regex()`, `Selector::Regex`, aplicação de `#show regex(...)` a texto. | Funcionalidade distribuída pelos prompts agregadores `value.md`, `show.md`, `text.md`, `eval.md`. | `entities/value.rs` (`Value::Regex`), `entities/show.rs` (`Selector::Regex`), `engine/stdlib/collections.rs`/`text.rs` (`native_regex`), `engine/eval/rules.rs` (`eval_show_rule`, `apply_show_rules`). | `typst-library/src/foundations/selector.rs`; `typst-realize/src/lib.rs` (aplicação regex). | Aplicação no vanilla é em `realize`, não em `eval`. |
| 4 | `engine/stdlib/grid_hline.md` | Função `grid.hline()` e entidade `GridHLine`. | Agrupado no prompt/ficheiro agregador `structural.md`. | `engine/stdlib/structural.rs` (`native_grid_hline`), `entities/elements/grid_hline.rs`. | `typst-library/src/layout/grid/mod.rs` (`GridHLine`, `GridItem::HLine`). | — |
| 5 | `engine/stdlib/grid_vline.md` | Função `grid.vline()` e entidade `GridVLine`. | Agrupado em `structural.md`. | `engine/stdlib/structural.rs` (`native_grid_vline`), `entities/elements/grid_vline.rs`. | `typst-library/src/layout/grid/mod.rs` (`GridVLine`, `GridItem::VLine`). | — |
| 6 | `engine/stdlib/table_hline.md` | Função `table.hline()` e entidade `TableHLine`. | Agrupado em `structural.md`. | `engine/stdlib/structural.rs` (`native_table_hline`), `entities/elements/table_hline.rs`. | `typst-library/src/model/table.rs` (`TableHLine`, `TableItem::HLine`). | — |
| 7 | `engine/stdlib/table_vline.md` | Função `table.vline()` e entidade `TableVLine`. | Agrupado em `structural.md`. | `engine/stdlib/structural.rs` (`native_table_vline`), `entities/elements/table_vline.rs`. | `typst-library/src/model/table.rs` (`TableVLine`, `TableItem::VLine`). | — |
| 8 | `engine/style/font-dict.md` | Dict legado e named fields para `#set text(font: ...)`. | Integrado no prompt agregador `eval.md`. | `engine/eval/rules.rs` (`parse_font_dict_named_fields`, `parse_font_dict_legacy`, arm `"font"`). | Não encontrado equivalente direto; fontes em `typst-library/src/text/font/`. | Pode ser extensão cristalina específica. |
| 9 | `infra/package_version_resolution.md` | Resolução de versão implícita de pacotes, cache no `SystemWorld`. | Materializado sob prompts agregadores `package_downloader.md` e `system-world.md`. | `package_downloader.rs` (`latest_version_remote`), `world.rs` (`resolve_package`). | `typst-kit/src/packages.rs` (`latest_version`); `typst-cli/src/init.rs`; `typst-syntax/src/package.rs`. | Alinhado com vanilla: resolução fora do eval. |
| 10 | `package-spec-dto.md` | DTO `PackageSpecDto` em L3 com serde/unscanny, isolando parse de L1. | **Nunca implementado**. O parse foi feito diretamente em L1. | `01_core/src/entities/package_spec.rs` implementa `FromStr` (L1). Não existe `03_infra/src/dto/`. | `typst-syntax/src/package.rs` (`PackageSpec`, `FromStr`). | O cristalino não tem crate de sintaxe equivalente; a intenção do DTO nunca foi concretizada. |

---

## O que este passo NÃO fez

- Não corrigiu nenhum prompt.
- Não moveu nenhum código.
- Não presumiu que os 10 casos partilham a mesma causa.

## Resultado entregue

Catálogo completo dos 10 prompts órfãos, com conteúdo real, causa de orfandade, localização atual no cristalino e localização correspondente no vanilla, pronto para alimentar o passo seguinte de decisão/arbitragem.
