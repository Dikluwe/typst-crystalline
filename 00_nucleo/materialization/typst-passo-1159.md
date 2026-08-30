# P1159 — realizar callbacks de page numbering e fechar o relayout

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN`
**Dependência:** P1158 contrato/transporte GREEN; gate P1157 aprovado

## Objetivo

Completar a metade funcional explicitamente pendente: layout inicial sela
Numbering e números lógicos; execução com Engine produz Content visível
`[current, total]` e de referência `[current]`; PageStore é injetado e o
relayout consome as vistas sem executar closures no layouter.

## Ordem

1. estender PageStore com números lógicos, spans e duas vistas Content;
2. fazer o layout inicial exportar os inputs de realização;
3. adicionar em L1 um realizador que reutiliza `stdlib/numbering`;
4. orquestrar em L3 layout → realização → injeção → relayout;
5. consumir vista visível na margem e unária em `ref(form: "page")`;
6. devolver Numbering cru em `location.page-numbering()` após o ciclo;
7. reproduzir as sondas P1157 no cristalino e rodar workspace integral.

## Critérios

- `V1/2`, `V2/2`; referência `R1`; lógico `N7/9..N9/9` e ref `N7`;
- footer explícito suprime margem e preserva `AUTO1` na referência;
- erros de aridade preservam diagnóstico;
- nenhum callback é chamado no layouter;
- typst-core, infra, workspace, fmt, diff e lint GREEN.

## 1. Proveniência de entrada

Antes de alterar código, registrar:

```text
date --iso-8601=seconds
git rev-parse HEAD
git branch --show-current
git status --short
git diff HEAD --stat
```

O HEAD esperado é `34e3ffb2e`; a working tree contém P1157/P1158 e os seus
resselos. Não separar, descartar ou sobrescrever esses hunks. A baseline P1158
é `typst-core 5.224/5.224` e `cargo check --workspace` GREEN.

## 2. Estado RED que deve ser demonstrado

Adicionar primeiro testes que comprovem a lacuna atual:

1. `page(numbering: (current, total) => ...)` é aceito, mas não emite margem;
2. `ref(form: "page")` com Func cai no placeholder `?` atual;
3. `location.page-numbering()` ainda devolve `none`;
4. footer explícito não pode apagar o Numbering usado pela referência;
5. counter lógico iniciado em 7 não pode ser substituído pelo índice físico.

Os testes devem reproduzir as sondas P1157 no nível de língua. O RED esperado
é ausência/placeholder ou valor incorreto; não escrever assertions contra
frames, bytes PDF ou identidade Rust.

## 3. Extensão de `PageStore`

Atualizar primeiro o L0 `entities/page_store.md`, se o código exigir nomes ou
tipos diferentes dos já aprovados. Materializar slots alinhados por página:

```text
numberings: Vec<Option<Numbering>>
logical_numbers: Vec<usize>
numbering_spans: Vec<Span>
visible_numberings: Vec<Option<Content>>
reference_numberings: Vec<Option<Content>>
```

Invariantes:

- todos os vetores completos têm comprimento `total_pages`;
- slot ausente é diferente de `Content::Empty` realizado;
- `numbering_for_page` continua devolvendo o objeto cru;
- accessors de vistas nunca executam callback;
- store permanece L1 puro, cloneável e imutável depois da injeção.

Adicionar testes unitários para store vazio, pattern, Func, conteúdo vazio,
fora de range e alinhamento 1:1.

## 4. Dados exportados pelo primeiro layout

O primeiro layout deve preservar em `Page`/`PagedDocument`, por página:

- Numbering ativo;
- número lógico corrente do counter(page);
- total lógico final;
- span da configuração;
- supplement e política marginal já existentes.

Não inferir corrente lógico de `pages.len()`. Usar o CounterRegistry/
Introspector vigente ou criar um snapshot explícito legitimado pelo L0. Se não
for possível obter o número lógico sem novo campo/método público além do gate
P1157, atualizar L0 e parar num gate ADR-0127 estreito antes dessa ampliação.

## 5. Realizador com Engine

No owner `compiler/stdlib/numbering.rs`, extrair um helper compartilhado que:

- recebe `&Numbering`, slice de inteiros, span, Scopes, EvalContext e Engine;
- Pattern usa `format_pattern`;
- Func usa `apply_func` com argumentos posicionais;
- converte o Value devolvido pela semântica normal de markup para Content;
- preserva diagnósticos e warnings do callback.

Não duplicar o dispatcher em L3. L3 apenas constrói Engine/ciclo e chama o
helper L1.

## 6. Ciclo no pipeline

Em `03_infra/src/pipeline.rs`, depois do primeiro layout:

```text
layout inicial
  → coleta Page inputs
  → realiza visible(current, total) e reference(current) com Engine
  → constrói/injeta PageStore completo no TagIntrospector
  → relayout com o mesmo Content e store conhecido
  → repete somente se páginas/números lógicos mudarem
```

Máximo de cinco iterações, coerente com os fixpoints existentes. Convergência
usa número de páginas, números lógicos, Numbering e Content realizado; não usa
bytes ou frames. Erro de callback encerra imediatamente com SourceDiagnostic.

Preservar reinjeção de BibStore, headings, positions, warnings e demais stores.
Não criar estado global ou I/O em L1.

## 7. Consumers

### Margem visível

`cursor.rs` e o caminho simétrico em `layout/mod.rs`:

- Pattern existente continua funcional;
- Func consulta `visible_numbering_for_page` do store conhecido;
- marginal auto emite o Content realizado;
- header/footer explícito ou none não emite a vista;
- não há chamada de `apply_func` no layouter.

### Referência

`layout/references.rs` elimina o placeholder `?` para Func e consome
`reference_numbering_for_page`. Supplement explícito conserva precedência;
conteúdo vazio permanece linkável sem texto numérico.

### Location

`eval/call_dispatch.rs` implementa `page-numbering` convertendo Numbering cru
para `Value::Str | Value::Func`; ausência vira `Value::None`. Como ContextBlock
é expandido antes do primeiro layout atualmente, o pipeline deve reexpandir ou
fornecer o store da iteração anterior antes de avaliar essa consulta. Não usar
default falso para mascarar a fase.

## 8. Testes GREEN

Reproduzir no cristalino, nos dois níveis adequados:

```text
V1/2, V2/2
R1
repr de page-numbering funcional: (..) => ..
N7/9, N8/9, N9/9; ref N7
page-run lexical: X1/2 e II
footer explícito + ref: AUTO1
callback unário visível: unexpected argument
callback ternário: missing argument: c
retorno 42 e none com morfologia vanilla
```

Adicionar controles de pattern simples/composto e `numbering: none` para evitar
regressão da superfície já GREEN.

## 9. Validação final

Executar e registrar números/proveniência:

```text
cargo test -p typst-core --lib
cargo test -p typst-infra --lib
cargo test --workspace
cargo check --workspace
cargo build --workspace
cargo fmt --all -- --check
git diff --check
crystalline-lint .
```

Zero failures, zero violations e zero V5. Warnings históricos não fazem parte
do fechamento. Atualizar P1158/P1159 e os L0s para retirar a marca de
incompletude somente depois de todas as sondas e do workspace integral GREEN.

## Execução

Materializados o PageStore completo, o realizador único L1, o ciclo L3 com
reexpansão contextual, os consumers de margem/referência e o retorno cru por
Location. O counter lógico iniciado em 7 produziu `N7/9`, `N8/9`, `N9/9`; a
referência com footer explícito produziu `R7` sem margem automática.

Proveniência: HEAD `34e3ffb2e06b105835939d0b4fd62e31c250211f`, working tree
não commitado com P1157–P1159; medição iniciada em
`2026-08-25T09:09:56-03:00`.

Validação: `typst-core` 5.225/5.225; `typst-infra` 841/841 após as quatro
sondas P1159; workspace integral GREEN; zero failures.
