# Passo 2 — ecow e SyntaxNode

## Contexto

Ler antes de começar:
- `00_nucleo/adr/0001-estrategia-migracao.md`
- `00_nucleo/adr/0004-passo1-descobertas.md`
- `lab/typst-original/crates/typst-syntax/src/node.rs`
- `lab/typst-original/crates/typst-syntax/src/set.rs`

Estado do Passo 1:
- ✓ `FileId`, `SyntaxKind`, `Span` em `01_core/entities/`
- ✗ `SyntaxNode` bloqueado por `ecow::EcoString`
- ✗ `PackageSpec` bloqueado por `ecow`, `serde`, `unscanny`
- ✗ `Source` bloqueado por `parse()` → Passo 4

---

## Decisão obrigatória antes de qualquer código — ecow

**Parar aqui. Não escrever código antes de resolver esta decisão.**

`ecow` é uma crate do próprio ecossistema Typst que fornece
`EcoString` — uma string clone-on-write com clone O(1) via
contagem de referências. É usada internamente no CST para
eficiência de memória em tokens de texto.

Verificar o que `SyntaxNode` usa de `ecow`:
```bash
grep -n "ecow\|EcoString\|EcoVec" lab/typst-original/crates/typst-syntax/src/node.rs | head -20
```

### As duas opções

**Opção A — autorizar `ecow` em `[l1_allowed_external]`**

```toml
[l1_allowed_external]
rust = ["thiserror", "comemo", "ecow"]
```

Justificativa para autorizar: `ecow` é um utilitário de
representação de dados — sem I/O, sem estado global, sem
efeitos colaterais. `EcoString` é conceptualmente equivalente
a `Arc<str>` com ergonomia adicional. A sua presença em L1
não viola a pureza funcional, apenas optimiza a memória.

Risco: cada crate autorizada em L1 é uma decisão que precisa
de ser mantida. `ecow` é pequena e estável, mas é uma
dependência do ecossistema Typst — se o Typst mudar, L1 muda.

**Opção B — substituir EcoString por Arc<str> em L1**

`SyntaxNode` em L1 usa `Arc<str>` para texto de tokens.
Em L3, ao construir o CST a partir do parser, converter
`EcoString` → `Arc<str>`. `ecow` fica confinada a L3.

Custo: adapter de conversão em L3; possível impacto de
performance nas operações de clone do CST.

### Como decidir

Correr este diagnóstico no código original:
```bash
# Quantas vezes EcoString aparece em node.rs
grep -c "EcoString" lab/typst-original/crates/typst-syntax/src/node.rs

# EcoString é parte da interface pública de SyntaxNode?
grep "pub.*EcoString\|-> EcoString\|EcoString.*->" lab/typst-original/crates/typst-syntax/src/node.rs
```

Se `EcoString` é parte da interface pública de `SyntaxNode`
(aparece em assinaturas `pub fn`), a Opção B exige adapters
em cada ponto de contacto com L3 — custo alto.

Se `EcoString` é apenas interna (campos privados), a Opção B
é mais simples — substituir o campo, converter na construção.

**Reportar o resultado ao developer antes de avançar.**
O developer decide A ou B e actualiza o `crystalline.toml`.
Só depois continuar para a Tarefa 1.

---

## Tarefa 1 — Prompt L0 para SyntaxNode

**Criar**: `00_nucleo/prompts/entities/syntax-node.md`

O prompt deve documentar:

1. O que é `SyntaxNode` no domínio do Typst:
   - Nó da árvore sintática concreta (CST)
   - Três representações internas: Inner (nós com filhos),
     Leaf (tokens de texto), Error (erros de parse)
   - Imutável após construção — o parser constrói, L1 apenas lê

2. Os campos que vêm para L1 (independentemente de A ou B):
   - `kind: SyntaxKind` — tipo do nó (já em L1)
   - `span: Span` — localização no texto (já em L1)
   - Children e texto dependem da decisão de ecow

3. O que fica em L3:
   - A construção de `SyntaxNode` a partir do output do parser
   - Conversões de representação se Opção B

4. Critérios de verificação:
   ```
   Dado SyntaxNode com kind == SyntaxKind::Ident
   Quando kind() for chamado
   Então retorna SyntaxKind::Ident

   Dado SyntaxNode folha com texto "foo"
   Quando text() for chamado  
   Então retorna "foo"

   Dado SyntaxNode com erros
   Quando errors() for chamado
   Então retorna lista não vazia
   ```

Actualizar `@updated` e registar no histórico de revisões.

---

## Tarefa 2 — Migrar SyntaxNode

**Origem**: `lab/typst-original/crates/typst-syntax/src/node.rs`
**Destino**: `01_core/entities/syntax_node.rs`

Verificar antes de copiar:
```bash
# Dependências directas de node.rs
grep "^use\|^pub use" lab/typst-original/crates/typst-syntax/src/node.rs
```

O que migra para L1:
- `SyntaxNode` — o tipo principal
- Métodos de leitura: `kind()`, `span()`, `text()`, `children()`,
  `errors()`, `has_error()`, `is_error()`, `is_leaf()`

O que **não** migra para L1:
- `reparser.rs` — reparsing incremental (depende do parser, Passo 4)
- Qualquer método que constrói `SyntaxNode` a partir de strings brutas

Adicionar header de linhagem:
```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/syntax-node.md
//! @prompt-hash <hash após --fix-hashes>
//! @layer L1
//! @updated 2026-03-22
```

---

## Tarefa 3 — SyntaxSet

**Origem**: `lab/typst-original/crates/typst-syntax/src/set.rs`
**Destino**: `01_core/entities/syntax_set.rs`

`SyntaxSet` é um bitset de `SyntaxKind` — zero dependências
externas, zero I/O. Migração directa.

Verificar:
```bash
grep "^use\|^pub use\|extern" lab/typst-original/crates/typst-syntax/src/set.rs
```

Esperado: apenas `use crate::SyntaxKind` — sem externos.

Adicionar ao prompt `00_nucleo/prompts/entities/syntax-node.md`
ou criar `syntax-set.md` separado se a complexidade justificar.

---

## Tarefa 4 — PackageSpec (se Opção A aprovada)

**Origem**: `lab/typst-original/crates/typst-syntax/src/package.rs`
**Destino**: `01_core/entities/package_spec.rs`

Verificar dependências:
```bash
grep "^use\|^pub use\|extern" lab/typst-original/crates/typst-syntax/src/package.rs
```

`PackageSpec` depende de `ecow`, `serde`, `unscanny`.

- `ecow`: resolvido pela decisão desta tarefa
- `serde`: utilitário de serialização — **não autorizar em L1**
  sem discussão. `PackageSpec` precisa de `serde`? Verificar se
  é apenas para o CLI (L2) ou para o domínio (L1).
- `unscanny`: scanner de strings — avaliar se é necessário em L1
  ou apenas em L3 para parsing de specs

Se `serde` ou `unscanny` forem necessários em L1, **parar e
reportar ao developer** antes de adicionar à whitelist.

Se `PackageSpec` usa `serde` apenas para serialização de
configuração (CLI/L2), a solução é:
- `PackageSpec` em L1 sem `serde`
- Implementar `Serialize`/`Deserialize` em L2 ou L3 via newtype

---

## Tarefa 5 — Actualizar 01_core/Cargo.toml

Após as tarefas anteriores, adicionar apenas as dependências
que foram explicitamente autorizadas:

```toml
[dependencies]
thiserror = { workspace = true }
comemo    = { workspace = true }
# ecow apenas se Opção A foi aprovada:
# ecow = { workspace = true }
```

E adicionar `ecow` ao workspace `Cargo.toml` se necessário:
```toml
[workspace.dependencies]
ecow = "0.2"
```

---

## Tarefa 6 — Actualizar mod.rs / lib.rs

Padrão correcto (sem `pub use self::` — ADR-0004):

```rust
// 01_core/src/lib.rs
pub mod entities;

// 01_core/src/entities/mod.rs
pub mod file_id;
pub mod span;
pub mod syntax_kind;
pub mod syntax_node;   // novo
pub mod syntax_set;    // novo
// pub mod package_spec;  // apenas se Tarefa 4 concluída
```

---

## Verificação final

```bash
cargo test -p typst-core
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Se V14 disparar para `ecow`:
- Opção A aprovada → adicionar ao `crystalline.toml` e repetir
- Opção B → verificar que EcoString não aparece em L1

Se V14 disparar para `serde` ou `unscanny`:
- **Parar. Reportar ao developer.** Não adicionar à whitelist
  sem decisão explícita.

---

## Ao terminar o Passo 2, reportar

- Decisão tomada sobre ecow (A ou B) e resultado
- Se `PackageSpec` foi migrado ou bloqueado (serde/unscanny)
- Qualquer outro externo inesperado que V14 sinalizou
- Estado final de `01_core/entities/`

Esta informação vai para ADR-0005 antes do Passo 3.
