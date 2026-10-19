# Relatório — Passo 864: Agrupamento por Parbreak em listas/enums/termos

**Data:** 2026-07-23  
**Commit base:** `06a336b0c3ae42949c2bced6c4c6511b9d89bebd`  
**Working tree:** alterações não commitadas (ver diff abaixo).  
**Passo:** P864  
**ADR de referência:** ADR-0109 (atomização, forma B), ADR-0107 (paridade com a língua), ADR-0108 (medir antes de decidir).

---

## 1. Objetivo

Fazer com que `ListItem`, `EnumItem` e `TermItem` separados por `Content::Parbreak` numa mesma `Sequence` sejam tratados como **grupos distintos** em layout, introduzindo o espaçamento de parágrafo entre grupos e reiniciando a numeração de enums — paridade com o comportamento vanilla para listas/enums/termos separados por linha em branco no markup.

---

## 2. Decisões tomadas

- **Manter a lógica no `sequence.rs`** conforme ADR-0109 (atomização, forma B). Não criar `Content::List`/`Content::Enum`/`Content::Terms` sintéticos; o agrupamento é decidido em tempo de layout pelo contexto de vizinhança numa `Sequence`.
- **Estado no `Layouter`:** adicionar `ItemGroup` enum (`List`, `Enum`, `Terms`) e os campos `last_seen_item_group` / `parbreak_since_last_item`.
- **Aplicar avanço de parágrafo extra** (`paragraph_advance`) quando um item do mesmo tipo é precedido por `Content::Parbreak`.
- **Resetar estado de agrupamento:** `last_was_loose_item = false` e `enum_counter = None` no momento do avanço, para não acumular espaçamento de itens soltos e para reiniciar a numeração do enum.
- **Avanço:** reutilizar o cálculo P762 (`top_edge + |bottom_edge| + leading default`).

---

## 3. Ficheiros alterados

### Alterações próprias do P864

| Ficheiro | Alteração |
|----------|-----------|
| `01_core/src/engine/layout/mod.rs` | Adicionado `ItemGroup`, campos `last_seen_item_group` / `parbreak_since_last_item`, inicialização no `Layouter::new`. |
| `01_core/src/engine/layout/sequence.rs` | Lógica de agrupamento por Parbreak; helper `item_group` e `paragraph_advance`. |
| `01_core/src/engine/layout/tests.rs` | 7 novos testes de layout para lista/enum/terms com e sem Parbreak, e caso misto lista→enum. |
| `00_nucleo/prompts/engine/layout.md` | Secção P864 com estado, lógica, avanço, reset de contador e validação. |
| `00_nucleo/prompts/engine/layout/list_item.md` | Passo P864 na semântica e critérios de validação. |
| `00_nucleo/prompts/engine/layout/enum_item.md` | Passo P864 na semântica, reset de `enum_counter`, critérios de validação. |
| `00_nucleo/prompts/engine/stdlib/structural.md` | Nota P864 em `native_terms` sobre agrupamento por Parbreak no layout. |
| `01_core/src/engine/layout/list_item.rs` | Atualização de `@prompt-hash` e `@updated`. |
| `01_core/src/engine/layout/enum_item.rs` | Atualização de `@prompt-hash` e `@updated`. |
| `01_core/src/engine/stdlib/structural.rs` | Atualização de `@prompt-hash` e `@updated`. |

### Alterações de desbloqueio — débito do P863

Estas correções foram necessárias para que a árvore compilasse antes do P864. São mencionadas aqui como trabalho de desbloqueio, **não como feature do P864**:

| Ficheiro | Alteração |
|----------|-----------|
| `01_core/src/engine/eval/rules.rs` | Correções de compilação do P863: `realize_body!` usa `Arc::clone($e)`; `Outline` tratado como folha; containers multi-conteúdo usam `Arc::clone(e)`. |
| `01_core/src/engine/introspect/locatable.rs` | Adicionado `Content::Par { .. }` ao bucket não-locatable (erro pré-existente de P863). |

---

## 4. Medição e validação

### 4.1 Build

```text
$ cargo build -p typst-core
   Compiling typst-core v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.73s
```

Compilação bem-sucedida (warnings pré-existentes, nenhum erro).

### 4.2 Testes novos do P864

```text
$ cargo test -p typst-core layout_lista -- --nocapture
running 3 tests
test engine::layout::tests::layout_lista_sem_parbreak_continua_grupo ... ok
test engine::layout::tests::layout_lista_para_enum_nao_adiciona_espaco_extra ... ok
test engine::layout::tests::layout_lista_parbreak_separa_grupos ... ok
test result: ok. 3 passed; 0 failed

$ cargo test -p typst-core layout_enum -- --nocapture
running 5 tests
test engine::layout::tests::layout_enum_tight_default_preserva_gap_natural ... ok
test engine::layout::tests::layout_enum_tight_false_adiciona_espaco ... ok
test engine::layout::tests::layout_enum_item_respeita_indentacao ... ok
test engine::layout::tests::layout_enum_parbreak_separa_grupos_e_reinicia_numero ... ok
test engine::layout::tests::layout_enum_sem_parbreak_continua_numero ... ok
test result: ok. 5 passed; 0 failed

$ cargo test -p typst-core layout_terms -- --nocapture
running 2 tests
test engine::layout::tests::layout_terms_sem_parbreak_continua_grupo ... ok
test engine::layout::tests::layout_terms_parbreak_separa_grupos ... ok
test result: ok. 2 passed; 0 failed
```

Os 7 testes novos específicos do P864 passam.

### 4.3 Workspace tests

```text
$ cargo test --workspace
...
test result: FAILED. 4681 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out
```

Falha única e pré-existente do P863:

```text
---- engine::eval::tests::tests::p863_show_par_func_transforma_paragrafo stdout ----
thread '...' panicked at 01_core/src/engine/eval/tests.rs:6730:9:
representação deve refletir a transformação strong: "sequence([space, text(\"Hello.\")])"
```

Esta falha não foi introduzida pelo P864. O teste `p863_show_par_func_transforma_paragrafo` falha porque a realização de parágrafos (`Content::Par`) não está a ser despoletada para `#show par: it => strong(it)` — débito documentado do P863.

### 4.4 Linter

```text
$ crystalline-lint .
warning: Prompt órfão: '00_nucleo/prompts/infra/package_version_resolution.md' ... [V7]
```

Zero violations novas; apenas V7 pré-existente.

---

## 5. Diff resumido

```text
 00_nucleo/prompts/engine/layout.md            | 112 +++++++++-
 00_nucleo/prompts/engine/layout/enum_item.md  |  33 +++-
 00_nucleo/prompts/engine/layout/list_item.md  |  28 +++-
 00_nucleo/prompts/engine/stdlib/structural.md |   7 +-
 01_core/src/engine/eval/rules.rs              | 261 +++++++++++++++++++++++---
 01_core/src/engine/introspect/locatable.rs    |   2 +
 01_core/src/engine/layout/enum_item.rs        |   3 +-
 01_core/src/engine/layout/list_item.rs        |   4 +-
 01_core/src/engine/layout/mod.rs              |  98 +++++++++-
 01_core/src/engine/layout/sequence.rs         |  57 +++++-
 01_core/src/engine/layout/tests.rs            | 256 ++++++++++++++++++++++++-
 01_core/src/engine/stdlib/structural.rs       |   3 +-
 12 files changed, 804 insertions(+), 60 deletions(-)
```

---

## 6. Contagem de testes

- Testes novos do P864: **7**
- Testes do workspace que passam: **4681**
- Testes do workspace que falham: **1** (P863, pré-existente)

---

## 7. Notas e próximos passos

- O P864 está funcionalmente fechado: o agrupamento por Parbreak, o avanço de parágrafo e o reset de enum estão implementados e validados pelos 7 testes novos.
- A falha `p863_show_par_func_transforma_paragrafo` deve ser tratada num passo dedicado ao P863; não é regressão do P864.
- Os hashes de lineage (`@prompt-hash`) foram sincronizados nos ficheiros de layout e structural para refletir os L0s actualizados.
