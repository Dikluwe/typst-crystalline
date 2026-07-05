# P571 — Reconciliação da contradição sobre os snapshots P307b

**Status:** `CONCLUÍDO` (verificação directa; nenhum código de produção alterado)  
**Data:** 2026-07-05  
**Scope:** `03_infra/fixtures/p307b/reference/`, `03_infra/src/layout_bidi.rs`, `01_core/src/rules/layout/cursor.rs`, `01_core/src/rules/layout/mod.rs`, `01_core/src/rules/layout/text.rs`.

---

## 1. Resumo executivo

A contradição a resolver era:

- Relatório P568 afirmava que os 9 snapshots P307b (01–09) passavam depois da regeneração.
- Relatório P569 afirmava que 5 desses 9 (01, 02, 03, 07, 09) falhavam e classificava-as como "pré-existentes, anterior a P569".

**Conclusão:** a afirmação de P569 está **parcialmente incorrecta**. As 5 falhas observadas no working tree actual **não são causadas pelo commit P569** em `03_infra/src/layout_bidi.rs`. A causa são alterações pendentes no working tree em L1 (`01_core/src/rules/layout/cursor.rs`, `mod.rs`, `text.rs`) que introduzem `layout_space()` e anexam espaços a `FrameItem::Text`. O commit P569 isolado provoca apenas a falha preexistente em `07-multi-feature`; P570 corrige essa falha. Por conseguinte, o relatório P569 deve ser corrigido e os snapshots **não** devem ser regenerados neste passo.

| Estado testado | Snapshots a passar | Snapshots a falhar | Observação |
|---|---|---|---|
| Working tree actual (L1 + P568 + snapshots P568) | 4 (04, 05, 06, 08) | 5 (01, 02, 03, 07, 09) | Estado inicial relatado por P569. |
| HEAD limpo (`cf0f5d4da`, P570) | 9 | 0 | P570 passa sem working tree. |
| P569 isolado (`3ece290f0`) | 8 | 1 (07) | A única falha já existia em P565. |
| P565 isolado (`24eccdc1e`) | 8 | 1 (07) | Confirmado preexistente a P569. |
| HEAD + apenas P568 (L3) | 9 | 0 | `collect_text_codepoints` não quebra snapshots. |
| HEAD + apenas L1 (`layout_space`) | 0 | 9 (todos) | L1 altera bytes PDF de todos os fixtures. |

---

## 2. Ordem real dos trabalhos

### 2.1 Commits no histórico

```bash
git log --oneline -- 03_infra/fixtures/p307b/reference/
```

Resultado (relevante):

```text
cf0f5d4da P570: sonda — espaço perdido é específico do mecanismo RTL
3ece290f0 P569: separa sufixos LTR e coalescedores de espaço em layout_bidi
fac1cf029 P567: reflow de parágrafos RTL em layout_bidi.rs
24eccdc1e P565: reflow RTL posterior para quebra de linha com direcção
```

P568 **não existe como commit** no ramo `Tekt`. O trabalho de P568 (`collect_text_codepoints` e regeneração dos snapshots) encontra-se apenas no **working tree** (ficheiros modificados não commitados). A ordem real no histórico é:

```text
P565 (24eccdc1e) → P567 (fac1cf029) → P569 (3ece290f0) → P570 (cf0f5d4da)
```

### 2.2 Diff de P569

```bash
git diff 24eccdc1e..3ece290f0 -- 03_infra/src/layout_bidi.rs
```

P569 altera exclusivamente `03_infra/src/layout_bidi.rs` (L3): ordenação do vector por x crescente, coalescência de espaços, separação de sufixos LTR. **Não toca em L1**.

### 2.3 Mudanças pendentes no working tree

```bash
git diff HEAD --stat
```

O working tree contém, entre outras, alterações em L1:

- `01_core/src/rules/layout/cursor.rs` — adiciona `layout_space()` que anexa o caractere de espaço ao último `FrameItem::Text`.
- `01_core/src/rules/layout/mod.rs` — adiciona campo `pending_space_width: Pt` e altera o braço `Content::Space`.
- `01_core/src/rules/layout/text.rs` — usa `layout_space()` em vez de avançar `cursor_x`.

Estas alterações **não têm Prompt L0 correspondente** em `00_nucleo/prompts/infra/layout_bidi.md`; o L0 vigente (hash `603ffda0`, P569) rejeita explicitamente mudar o Layouter (L1).

---

## 3. Medições

### 3.1 Estado actual do working tree

```bash
cargo test -p typst-infra p307b_snapshot_tests
```

Resultado:

```text
test result: FAILED. 4 passed; 5 failed; 0 ignored
Falhas: 01, 02, 03, 07, 09
```

### 3.2 HEAD limpo (P570)

```bash
git stash push -u
cargo test -p typst-infra p307b_snapshot_tests
```

Resultado:

```text
test result: ok. 9 passed; 0 failed; 0 ignored
```

### 3.3 P569 isolado

```bash
git checkout 3ece290f0
cargo test -p typst-infra p307b_snapshot_tests
```

Resultado:

```text
test result: FAILED. 8 passed; 1 failed; 0 ignored
Falha: 07-multi-feature (actual=2130B expected=2130B, bytes diferentes)
```

### 3.4 P565 isolado

```bash
git checkout 24eccdc1e
cargo test -p typst-infra p307b_snapshot_tests
```

Resultado:

```text
test result: FAILED. 8 passed; 1 failed; 0 ignored
Falha: 07-multi-feature (actual=2130B expected=2130B)
```

### 3.5 Isolamento da causa no working tree

Aplicando apenas as alterações de P568 (L3) sobre HEAD:

```bash
git checkout cf0f5d4da
git apply <patch P568 L3>
cargo test -p typst-infra p307b_snapshot_tests
```

Resultado:

```text
test result: ok. 9 passed; 0 failed; 0 ignored
```

Aplicando apenas as alterações de L1 (`layout_space`) sobre HEAD:

```bash
git checkout cf0f5d4da
git apply <patch L1>
cargo test -p typst-infra p307b_snapshot_tests
```

Resultado:

```text
test result: FAILED. 0 passed; 9 failed; 0 ignored
```

### 3.6 Snapshots no working tree

Os snapshots de referência no working tree têm os tamanhos reportados por P568:

| Fixture | Tamanho (B) |
|---|---|
| 01-markup-plain.pdf | 1022 |
| 02-markup-heading.pdf | 1591 |
| 03-text-styling.pdf | 2114 |
| 04-shapes.pdf | 1228 |
| 05-gradient-linear.pdf | 947 |
| 06-gradient-conic.pdf | 1132 |
| 07-multi-feature.pdf | 2727 |
| 08-image-jpeg.pdf | 1706 |
| 09-cidfont.pdf | 30978 |

No entanto, aplicando os snapshots do working tree sobre HEAD + P568 (L3) isolado, **todos os 9 falham**. Isto indica que os snapshots foram regenerados num estado que incluía também alterações de L1 (ou outras alterações do working tree), e não apenas P568.

---

## 4. Análise

### 4.1 P569 não é a causa das 5 falhas

- P569 altera apenas L3 (`layout_bidi.rs`).
- P569 isolado apresenta apenas a falha preexistente em `07-multi-feature`, a mesma de P565.
- P570 (commit seguinte) corrige essa falha; HEAD limpo passa 9.
- Logo, as falhas 01, 02, 03, 09 **não existem** no histórico commitado até P570.

### 4.2 A causa são as alterações de L1 no working tree

- `layout_space()` em L1 anexa o caractere de espaço a `FrameItem::Text` e altera a forma como os espaços são emitidos.
- Aplicando apenas L1 sobre HEAD, **todos os 9 snapshots falham**.
- Aplicando L1 + P568 (L3) + snapshots do working tree, obtém-se exactamente as 5 falhas observadas inicialmente.

### 4.3 Sobre a frase "pré-existente, anterior a P569"

A frase no relatório P569 é incorrecta se interpretada como "as 5 falhas existiam antes do commit P569". A medição mostra que:

- A única falha preexistente a P569 é `07-multi-feature`.
- As falhas 01, 02, 03, 09 surgem apenas no working tree actual, causadas por alterações de L1 não commitadas.

No entanto, a frase pode ser reinterpretada como "não causadas pelo commit P569 em L3", o que é verdadeiro. Recomenda-se corrigir o relatório P569 para remover a ambiguidade.

### 4.4 Por que P568 relatou 9 snapshots a passar

**Inferência:** P568 regenerou os snapshots num estado do working tree que já incluía as alterações de L1 (`layout_space`). Os tamanhos dos PDFs aumentaram consistentemente com a inclusão de espaços como conteúdo textual. No entanto, o código L1 não foi commitado, e o conjunto snapshots+L1 actual não é internamente consistente (5 falham).

**O que refutaria esta inferência:** encontrar um commit P568 no histórico que inclua L1. Não foi encontrado.

---

## 5. Decisão / Classificação

| Questão | Decisão | Base de medição |
|---|---|---|
| P569 causou as 5 falhas? | **Não** | P569 isolado tem apenas 1 falha (07), preexistente em P565. |
| As falhas são pré-existentes a P569? | **Parcialmente** | Apenas `07-multi-feature` é preexistente. As outras 4 surgem do working tree. |
| A causa é o código L1 no working tree? | **Sim** | Aplicação isolada de L1 faz 9/9 falhar. |
| Devem-se regenerar os snapshots P307b? | **Não neste passo** | Regenerar validaria código L1 sem Prompt L0 correspondente (violação da Trava Arquitetural, AGENTS.md). |
| Deve-se corrigir o relatório P569? | **Sim** | A frase "pré-existente, anterior a P569" para 5 falhas está incorrecta. |

---

## 6. Próximos passos

1. **Corrigir `00_nucleo/diagnosticos/paridade-producao-p569.md`**: substituir a classificação das 5 falhas como "pré-existentes" por uma nota que indique que as falhas 01, 02, 03, 09 foram observadas num working tree com alterações de L1 pendentes, não no commit P569.
2. **Resolver o código L1 pendente**: se as alterações de `layout_space()` forem desejadas, é necessário actualizar o Prompt L0 `00_nucleo/prompts/infra/layout_bidi.md` (ou criar um novo L0) antes de commitar o código L1.
3. **Só depois regenerar snapshots P307b**: quando o código L1 estiver legitimado por L0 e o conjunto código+snapshots for internamente consistente.

---

## 7. Comandos de reprodução

```bash
# Estado actual do working tree
cargo test -p typst-infra p307b_snapshot_tests

# HEAD limpo (P570) — 9 passam
git stash push -u
cargo test -p typst-infra p307b_snapshot_tests
git stash pop

# P569 isolado — 8 passam, 1 falha (07)
git checkout 3ece290f0
cargo test -p typst-infra p307b_snapshot_tests

# P565 isolado — 8 passam, 1 falha (07)
git checkout 24eccdc1e
cargo test -p typst-infra p307b_snapshot_tests
```

---

## 8. Ficheiros referenciados

| Ficheiro | Relevância |
|---|---|
| `03_infra/fixtures/p307b/reference/*.pdf` | Snapshots P307b. |
| `03_infra/src/layout_bidi.rs` | Alterado por P569 (L3). |
| `01_core/src/rules/layout/cursor.rs` | Alterado no working tree por L1 (`layout_space`). |
| `01_core/src/rules/layout/mod.rs` | Alterado no working tree por L1 (`pending_space_width`, braço `Content::Space`). |
| `01_core/src/rules/layout/text.rs` | Alterado no working tree por L1 (usa `layout_space()`). |
| `00_nucleo/prompts/infra/layout_bidi.md` | L0 vigente rejeita alterar o Layouter (L1). |
| `00_nucleo/diagnosticos/paridade-producao-p569.md` | Relatório a corrigir. |

---

## 9. Conclusão

A contradição entre P568 e P569 é resolvida da seguinte forma:

- P569 (commit) **não introduziu** as 5 falhas.
- As 5 falhas observadas no working tree são causadas por alterações de L1 pendentes (`layout_space`), não legitimadas pelo L0 vigente.
- O relatório P569 contém uma afirmação incorrecta que deve ser corrigida.
- Não se regeneram snapshots neste passo, porque isso validaria código L1 sem L0.
