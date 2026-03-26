# Passo 6 — eval() e Module (v2)

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/0001-estrategia-migracao.md` (Opção C para comemo)
- `00_nucleo/adr/0005-packagespec-world.md` (World trait, stubs)
- `00_nucleo/adr/0015-ecow.md` (EcoString → SyntaxText/String)

Pré-condição: `cargo test -p typst-core` — 150 testes, zero violations.
Passos 4 e 5 concluídos — `parse()`, `Source` real, AST tipada em L1.

### Numeração de ADRs

As ADRs 0001–0015 estão ocupadas. Qualquer novo externo encontrado
neste passo recebe ADR a partir de **0016**. Seguir o mesmo processo
dos passos anteriores: uma ADR por crate, decidir antes de implementar.

### SourceResult — tipo em falta

A interface de `eval()` usa `SourceResult<T>` que ainda não existe
em L1. É provável que seja:

```rust
pub type SourceResult<T> = Result<T, EcoVec<SourceDiagnostic>>;
```

`EcoVec` é de `ecow` — aplicar ADR-0015 (substituir por `Vec`).
`SourceDiagnostic` precisa de ser avaliado: se for tipo de domínio
puro (span + mensagem + hints) → L1. Verificar durante o diagnóstico.

---

## Diagnóstico obrigatório — typst-eval

**Parar aqui. Este diagnóstico determina o passo inteiro.**

```bash
# Dependências directas de typst-eval
cat lab/typst-original/crates/typst-eval/Cargo.toml

# Quantos ficheiros e quais
find lab/typst-original/crates/typst-eval/src -name "*.rs" | wc -l
find lab/typst-original/crates/typst-eval/src -name "*.rs" | sort

# Assinatura de eval()
grep -n "^pub fn eval\|^pub fn compile" \
  lab/typst-original/crates/typst-eval/src/lib.rs | head -10

# comemo em eval() — espera-se Tracked<dyn World>
grep -n "Tracked\|comemo\|#\[comemo" \
  lab/typst-original/crates/typst-eval/src/lib.rs | head -20

# typst-library em eval — este é o bloqueante crítico
grep -n "typst_library\|typst_layout\|typst_realize" \
  lab/typst-original/crates/typst-eval/src/lib.rs | head -10

# SourceResult e SourceDiagnostic — onde são definidos
grep -rn "pub type SourceResult\|pub struct SourceDiagnostic" \
  lab/typst-original/crates/ | head -5

# Module — onde é definido e o que contém
grep -rn "^pub struct Module" \
  lab/typst-original/crates/ | head -5

# Value — onde é definido
grep -rn "^pub enum Value\|^pub struct Value" \
  lab/typst-original/crates/ | head -5

# Content — onde é definido
grep -rn "^pub struct Content\|^pub enum Content" \
  lab/typst-original/crates/ | head -5
```

**Reportar o output completo antes de continuar.**

---

## Decisão de camada (após diagnósticos)

### Se typst-eval depende de typst-library (esperado)

`typst-library` implementa as funções builtin do Typst (text, page,
heading, etc.) e tem 40+ dependências incluindo I/O de fontes e
rendering. `eval()` não pode ser L1 sem trazer essa cadeia inteira.

**Decisão: eval() vai para L3.**

```
03_infra/src/eval/mod.rs   ← motor de avaliação (eval() pública)
03_infra/src/eval/vm.rs    ← Vm, stack de scopes (se existir separado)
```

`Module`, `Value`, `Content` precisam de avaliação individual:
- Tipo de domínio puro (apenas campos primitivos e tipos L1) → L1
  como stub ou migração completa
- Dependência de typst-library ou I/O → L3

### Se typst-eval é surpreendentemente limpo

Avaliar cada externo com o processo estabelecido nas ADRs 0006–0015:
- Tabelas de standard internacional → `[l1_allowed_external]`
- Algoritmo de utilidade puro → inline
- I/O ou infraestrutura → L3
- Uma ADR por crate, começando em 0016

**Não assumir. Os diagnósticos decidem.**

---

## Tarefa 1 — SourceResult e SourceDiagnostic em L1

Independentemente de onde `eval()` ficar, `SourceResult<T>` é
o tipo de retorno do pipeline completo — pertence a L1.

Após o diagnóstico, criar `01_core/src/entities/source_result.rs`:

```rust
/// Erro de diagnóstico do compilador Typst.
/// Span + mensagem + hints — tipo de domínio puro.
pub struct SourceDiagnostic {
    pub span:     Span,
    pub severity: DiagnosticSeverity,
    pub message:  SyntaxText,
    pub hints:    Vec<SyntaxText>,
    // traces e outros campos a confirmar no diagnóstico
}

pub enum DiagnosticSeverity { Error, Warning }

/// Resultado de operação do compilador.
/// EcoVec<SourceDiagnostic> substituído por Vec<SourceDiagnostic> (ADR-0015).
pub type SourceResult<T> = Result<T, Vec<SourceDiagnostic>>;
```

Se `SourceDiagnostic` tiver campos que dependem de `typst-library`
— criar stub opaco e resolver no passo seguinte.

---

## Tarefa 2 — Module em L1 ou L3?

`Module` é o resultado de avaliar um ficheiro Typst. Após o
diagnóstico, avaliar:

```bash
grep -n "struct Module\|impl Module\|Content\|Scope\|Value\|EcoString" \
  lab/typst-original/crates/typst-eval/src/module.rs 2>/dev/null | head -30

# Dependências externas de module.rs
grep "^use\|^extern" \
  lab/typst-original/crates/typst-eval/src/module.rs 2>/dev/null \
  | grep -v "crate::\|super::\|std::" | head -20
```

**Se Module é domínio puro** (contém apenas Content stub + Scope
sem dependências externas):
→ `01_core/src/entities/module.rs` com stub opaco para Content

**Se Module depende de typst-library**:
→ `03_infra/src/eval/module.rs` — decide no Passo 7 quando
  Content for migrado

Em qualquer caso, criar um stub em L1 para que `World::source()`
e o pipeline possam compilar:

```rust
// 01_core/src/entities/module.rs (stub)
/// Resultado da avaliação de um ficheiro Typst.
/// Stub opaco — interior definido quando Content migrar.
pub struct Module(());
```

---

## Tarefa 3 — Prompt L0 (após decisão de camada)

**Se eval() em L3:**
Criar `00_nucleo/prompts/infra/eval.md`

Interface pública de L3:
```rust
pub fn eval(
    world: comemo::Tracked<dyn TrackedWorld>,
    source: &Source,
) -> SourceResult<Module>
```

Documentar:
- Dependências de `03_infra/Cargo.toml` adicionadas (uma por uma)
- Fronteira L1→L3: `Source` e `World` entram, `Module` sai
- `comemo::Tracked` autorizado via ADR-0001

**Se eval() em L1:**
Criar `00_nucleo/prompts/rules/eval.md`

Documentar cada externo com ADR correspondente (começar em 0016).

---

## Tarefa 4 — Migrar eval()

### Se L3

**Destino**: `03_infra/src/eval/`

Actualizar `03_infra/Cargo.toml` — uma dependência de cada vez:

```toml
# Cada linha é uma decisão documentada em ADR
# Não copiar o Cargo.toml original de typst-eval cegamente
[dependencies]
typst-core = { path = "../01_core" }
comemo     = { workspace = true }  # ADR-0001
# ... adicionar conforme necessário, com comentário de ADR
```

A fronteira L1→L3:
- `Source` (L1) entra — L3 lê `source.root()` e `source.id()`
- `World` (L1 trait) é injectado via `comemo::Tracked<dyn TrackedWorld>`
- `Module` (L1 stub ou L3) sai como resultado

Substituições a aplicar durante a migração:
| Original | Substituição | ADR |
|----------|-------------|-----|
| `EcoVec<SourceDiagnostic>` | `Vec<SourceDiagnostic>` | 0015 |
| `EcoString` em mensagens | `String`/`SyntaxText` | 0015 |
| Qualquer novo externo | ADR 0016+ antes de adicionar | — |

### Se L1

Seguir o protocolo estabelecido nos Passos 4–5:
1. V14 sinaliza externo → criar ADR (0016+)
2. I/O → mover para L3
3. Standard internacional sem I/O → `[l1_allowed_external]`
4. Algoritmo de utilidade → inline
5. Nunca adicionar por conveniência

---

## Verificação final

```bash
cargo test -p typst-core
cargo test -p typst-infra    # se eval() foi para L3
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Teste de integração mínimo (se eval() funcionar end-to-end):
```rust
#[test]
fn parse_eval_texto_simples() {
    // MockWorld mínimo — implementa World trait de L1
    // parse("Hello") → Source → eval(world, &source) → Module
    // Não precisa de fontes reais nem filesystem
    struct MockWorld;
    impl World for MockWorld {
        fn library(&self) -> &Library { /* stub */ }
        fn book(&self) -> &FontBook { /* stub */ }
        fn main(&self) -> FileId { /* stub */ }
        fn source(&self, _: FileId) -> FileResult<Source> { /* stub */ }
        fn file(&self, _: FileId) -> FileResult<Bytes> { /* stub */ }
        fn font(&self, _: usize) -> Option<Font> { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }
    // ...
}
```

---

## Ao terminar, reportar

- Decisão tomada (L1 ou L3) e justificação com base nos diagnósticos
- Se `Module`, `Value`, `Content` foram para L1 ou L3
- Se `SourceResult`/`SourceDiagnostic` foram migrados para L1
- Externos novos que V14 sinalizou — números de ADR criados (0016+)
- Se o teste de integração parse→eval passou
- Número total de testes

Esta informação vai para o Passo 7
(SystemWorld em L3 — implementação real de World).
