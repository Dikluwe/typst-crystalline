# Relatório — typst-passo-856: validação de `ref(<label>)` — casos de erro

**Data de execução:** 2026-07-23  
**Commit base:** `dfe3c2282ec0d6bd97d5834f00214e7c7f5d2d49`  
**Estado do working tree:** alterações introduzidas por este passo nos ficheiros listados em §Alterações.

---

## Resumo executivo

Foram encontradas **divergências em relação ao vanilla Typst 0.15.0** nos casos de erro de `#ref(<label>)`. O cristalino tratava corretamente label inexistente e heading sem `numbering`, mas:

- Dava `label <x> does not exist in the document` para labels existentes em texto/lista/raw.
- Compilava silenciosamente (renderizando "?") para equations sem `numbering`.

Foram corrigidas introduzindo um registo de **labels não referenciáveis** no `Introspector` e estendendo as validações no `layout_ref`. Todas as mensagens agora batem com o vanilla; o caminho feliz (heading/equation numerados, figuras) continua sem regressão.

---

## Sonda dos casos de erro

Ficheiros de teste gerados em `temp/p856/`.

### Resultados antes da correção

| Caso | Vanilla 0.15.0 | Cristalino (antes) |
|---|---|---|
| Label inexistente | `label <naoexiste> does not exist in the document` | ✓ igual |
| Heading sem `numbering` | `cannot reference heading without numbering` + hint | ✓ igual |
| Equation sem `numbering` | `cannot reference equation without numbering` + hint | **compilava com sucesso** (renderizava "?") |
| Label num parágrafo | `cannot reference text` | `label <lbl> does not exist in the document` |
| Label num item de lista | `cannot reference text` | `label <item> does not exist in the document` |
| Label num bloco raw | `cannot reference raw directly, try putting it into a figure` | `label <code> does not exist in the document` |

### Resultados após a correção

| Caso | Vanilla 0.15.0 | Cristalino (depois) |
|---|---|---|
| Label inexistente | `label <naoexiste> does not exist in the document` | ✓ igual |
| Heading sem `numbering` | `cannot reference heading without numbering` + hint | ✓ igual |
| Equation sem `numbering` | `cannot reference equation without numbering` + hint | ✓ igual |
| Label num parágrafo | `cannot reference text` | ✓ igual |
| Label num item de lista | `cannot reference text` | ✓ igual |
| Label num bloco raw | `cannot reference raw directly, try putting it into a figure` | ✓ igual |

---

## Onde as validações aconteciam (e onde estavam em falta)

- **Camada L1** (`01_core/src/engine/layout/references.rs`):
  - `layout_ref` já validava label inexistente (P788) e heading sem `numbering`.
  - Não validava equation sem `numbering`, nem labels existentes mas associadas a conteúdo não numerável (texto, raw, lista).
- **Camada L1** (`01_core/src/engine/introspect.rs`):
  - O walk de introspecção propagava labels auto (`<label>`) para o body, mas só registava no `Introspector` quando o body emitia `Tag` locatable.
  - `Content::ListItem` / `Content::EnumItem` eram terminais no walk — labels dentro de itens de lista não eram processadas.
  - Equations emitiam `Tag`, mas a flag `numbering_active` não era exposta ao `layout_ref`.
- **L3** (`03_infra/src/measurements.rs`):
  - `CountingIntrospector` implementa o trait `Introspector` e precisou de forwardar os novos métodos.

---

## Alterações implementadas

### 1. Novo tipo `UnreferencableKind` (L1)

Ficheiro novo:

```text
01_core/src/entities/label_kind.rs
```

Enum com variants:

- `Text` — texto, parágrafos, listas, enums, terms, etc.
- `Raw` — blocos raw/código.
- `EquationWithoutNumbering` — equation com numbering desactivado.
- `Other` — fallback.

Exportado via `01_core/src/entities/mod.rs`.

### 2. Extensão do `Introspector` (L1)

Ficheiro:

```text
01_core/src/entities/introspector.rs
```

Adicionados ao trait:

```rust
fn equation_has_numbering(&self, location: Location) -> Option<bool>;
fn unreferencable_label_kind(&self, label: &Label) -> Option<UnreferencableKind>;
```

Adicionados ao `TagIntrospector`:

```rust
pub equation_numbering: HashMap<Location, bool>,
pub unreferencable_labels: HashMap<Label, UnreferencableKind>,
```

### 3. População durante o walk (L1)

Ficheiro:

```text
01_core/src/engine/introspect.rs
```

- Populate arm `ElementPayload::Equation` regista `equation_numbering` e, quando `block && !numbering_active`, insere a label em `unreferencable_labels` com `EquationWithoutNumbering`.
- Walk arm `Content::Label` (auto e manual) regista labels cujo body não é locatable em `unreferencable_labels`.
- Walk arms `Content::ListItem` e `Content::EnumItem` deixaram de ser terminais — descem no `body` para que labels anexadas a itens sejam processadas.
- Helper `classify_unreferencable_body` classifica corpo como `Text`, `Raw` ou `EquationWithoutNumbering`.

### 4. Validação no `layout_ref` (L1)

Ficheiro:

```text
01_core/src/engine/layout/references.rs
```

- Verificação de `unreferencable_label_kind` passou a ser **a primeira** (antes de `known`), porque alguns labels não referenciáveis — como equations sem numbering — também são conhecidos por `query_by_label`.
- Adicionada validação de equation sem `numbering`, simétrica à de heading.

### 5. Forward em `CountingIntrospector` (L3)

Ficheiro:

```text
03_infra/src/measurements.rs
```

Implementação dos novos métodos do trait delegando para `inner`.

### 6. L0 atualizado

Ficheiro:

```text
00_nucleo/prompts/engine/layout_references.md
```

Adicionada secção §P856 descrevendo o registo de labels não referenciáveis e as mensagens de erro paridade vanilla.

### 7. Testes

Ficheiro:

```text
01_core/src/engine/layout/tests.rs
```

Novo módulo `p856_ref_unreferencable_labels` com testes para:

- texto → `cannot reference text`
- raw → `cannot reference raw directly, try putting it into a figure`
- equation sem numbering → `cannot reference equation without numbering`
- equation com numbering continua a funcionar
- label inexistente continua a dar `does not exist`

---

## Validação final

### Testes por crate

```bash
cargo test -p typst-core
```

```text
test result: ok. 4655 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out
doc-tests typst_core: ok. 0 passed; 0 failed; 3 ignored
```

```bash
cargo test -p typst-shell
```

```text
test result: ok. 36 passed; 0 failed; 0 ignored
```

```bash
cargo test -p typst-infra
```

```text
test result: ok. 714 passed; 0 failed; 5 ignored
```

```bash
cargo test -p typst-wiring
```

```text
test result: ok. 31 passed; 0 failed; 0 ignored
Running tests/crystalline_lint.rs: ok. 2 passed; 0 failed
```

### Teste completo do workspace

```bash
cargo test --workspace
```

Resultado global: **todos os testes passaram**.

### Linter arquitetural

```bash
crystalline-lint .
```

```text
warning: Prompt órfão: '00_nucleo/prompts/infra/package_version_resolution.md' não é referenciado por nenhum arquivo em L1–L4. [V7]
```

Zero violações relacionadas com este passo. O warning V7 é preexistente.

---

## Conclusão

A validação dos casos de erro de `ref(<label>)` está **fechada**. O cristalino agora reproduz as mensagens do vanilla Typst 0.15.0 para labels inexistentes, headings sem numbering, equations sem numbering, e labels existentes mas associadas a texto/raw/lista. A suíte de testes do workspace passa na totalidade.
