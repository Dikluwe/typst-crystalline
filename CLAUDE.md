# CLAUDE.md — typst-crystalline

Este ficheiro guia o Claude Code neste repositório.
Para decisões específicas: **ler os ADRs em `00_nucleo/adr/`**.

---

## ⚠️ Restrição de leitura — pasta de materialização

```
00_nucleo/materialization/
```

**Não ler esta pasta por iniciativa própria.**

- Só aceder quando explicitamente indicado com o path completo
- Nunca varrer ou listar o conteúdo desta pasta
- Nunca inferir o que fazer a partir do nome dos ficheiros
- Se uma tarefa parece relacionada com materialização mas não
  referencia um ficheiro explícito: **perguntar antes de agir**

Motivo: os ficheiros de materialização são instruções sequenciais
com ordem e momento específicos. Lê-los fora de contexto quebra
o estado do projecto.

---

## O que é este projecto

Migração do compilador Typst para a Arquitetura Cristalina (Tekt).

O código original está em `lab/typst-original/` (quarentena).
A migração acontece gradualmente para as camadas cristalinas.
O linter `crystalline-lint` verifica que a arquitectura é respeitada
— o critério de correcção primário é `crystalline-lint .` com zero
violations.

---

## Arquitetura Cristalina — definições

### Camadas

| Camada | Directório | Propósito |
|--------|-----------|-----------|
| L0 | `00_nucleo/` | Prompts, ADRs — fonte de verdade. Não é código. |
| L1 | `01_core/` | Domínio puro. Zero I/O. Funções determinísticas. |
| L2 | `02_shell/` | CLI, formatadores. Conhece apenas L1. |
| L3 | `03_infra/` | I/O, filesystem, fontes, pacotes. Conhece apenas L1. |
| L4 | `04_wiring/` | Composição. Conhece L1, L2, L3. Zero lógica de negócio. |
| lab | `lab/` | Quarentena. Nunca importado por L1–L4. |

### Topologia de imports permitidos

```
L4 → L1, L2, L3
L3 → L1
L2 → L1
L1 → (apenas stdlib pura e [l1_allowed_external])
lab → (qualquer, mas ninguém importa lab)
```

Qualquer violação desta topologia é detectada por V3 (ForbiddenImport).

### O que é L1 (domínio puro)

L1 contém tipos de domínio e lógica de negócio pura:
- Tipos de valor: `FileId`, `Span`, `SyntaxKind`, `SyntaxNode`
- Contratos (traits): `World`, contratos de I/O
- Regras: pipeline parse → eval → layout (sem I/O)

**L1 nunca**:
- Lê ou escreve ficheiros (`std::fs`)
- Faz chamadas de rede
- Tem estado global mutável
- Importa crates externos não declarados em `[l1_allowed_external]`

### O que é L3 (infra)

L3 implementa os contratos declarados em L1:
- `FileIdInterner` — geração de `FileId` únicos (removida de L1 no Passo 1)
- Implementação de `World` — filesystem, fontes, pacotes
- Exporters — PDF, SVG, PNG

### Diferença entre L2 e L4

**L2 (shell)**: traduz input do utilizador (CLI args) em chamadas a L1.
Conhece a interface pública de L1 mas não L3.

**L4 (wiring)**: instancia L2, L3, e injeta em L1. É o único lugar
onde todos os componentes se conhecem. Zero lógica — apenas
composição.

---

## Travas do linter — o que cada violation significa

### Fatais (bloqueiam sempre)

**V0 — UnreadableSource**
Ficheiro não pode ser lido. Corrigir permissões.

**V8 — AlienFile**
Ficheiro num directório não mapeado em `[layers]` nem `[excluded]`.
Todos os directórios do projecto devem estar mapeados.

**V10 — QuarantineLeak**
Código de produção (L1–L4) importa `lab/`. Proibido absolutamente.

### Errors (bloqueiam com `--fail-on error`)

**V1 — MissingPromptHeader**
Ficheiro em L1–L4 sem header de linhagem `@prompt`/`@layer`.
Adicionar o header antes de qualquer outra coisa.

**V2 — MissingTestFile**
Módulo L1 sem cobertura de testes. Ficheiros declaration-only
(apenas tipos, sem implementação) são isentos automaticamente.

**V3 — ForbiddenImport**
Import que viola a topologia. Ex: L1 importando L3, L2 importando L3.
Reorganizar a dependência ou mover o módulo para a camada correcta.

**V4 — ImpureCore**
Símbolo de I/O em L1: `std::fs`, `std::net`, `std::process`.
Mover para L3 e injectar via trait em L1.

**V9 — PubLeak**
Import de L1 fora das portas declaradas em `[l1_ports]`.

**V11 — DanglingContract**
Trait em `01_core/contracts/` sem implementação em L2 ou L3.

**V13 — MutableStateInCore**
Estado global mutável em L1: `static mut`, `static Mutex<T>`,
`static OnceLock<T>`, `static LazyLock<T>`, `AtomicXxx` estático.
Mover o estado para L3 e injectar.

**V14 — ExternalTypeInContract**
Import externo em L1 não declarado em `[l1_allowed_external]`.
Dois casos distintos neste projecto:

1. **Crate nova encontrada**: avaliar se pertence a L1 (utilitário
   de domínio puro) ou a L3 (infraestrutura). Discutir com o
   developer antes de adicionar à whitelist.

2. **`pub use self::X::Y` em mod.rs**: o linter não reconhece
   `self::` como referência interna — dispara V14 incorrectamente.
   **Padrão correcto**: usar apenas `pub mod X;` em `lib.rs` e
   `mod.rs`. Não usar re-exports com `self::` em L1. (ADR-0004)

### Warnings

**V5 — PromptDrift**
Hash do prompt diverge. Correr `crystalline-lint --fix-hashes .`
após editar qualquer ficheiro em `00_nucleo/prompts/`.

**V6 — PromptStale**
Interface pública diverge do snapshot. Correr
`crystalline-lint --update-snapshot .` após validar a mudança.

**V7 — OrphanPrompt**
Prompt sem materialização. Adicionar a `[orphan_exceptions]` se
for um prompt que não materializa código (ex: ADRs, documentação).

**V12 — WiringLogicLeak**
Declaração de tipo em L4 que não é adapter. Mover para L1 ou L3.

---

## Externos autorizados em L1

Declarados em `crystalline.toml` → `[l1_allowed_external]`:

| Crate | Motivo |
|-------|--------|
| `thiserror` | derive(Error) para tipos de erro de domínio |
| `comemo` | compilação incremental + compatibilidade com ecossistema Typst (ADR-0001) |

**`ecow`**: decisão pendente para Passo 2 — `SyntaxNode` e
`PackageSpec` dependem de `ecow::EcoString`. Não adicionar sem
ADR aprovado.

Qualquer outro crate que V14 sinalize em L1 → criar ADR antes
de adicionar à whitelist. Não adicionar por conveniência.

---

## Padrões proibidos em L1

```rust
// ❌ Estado global mutável (V13)
static INTERNER: Mutex<HashMap<PathBuf, FileId>> = Mutex::new(HashMap::new());
static COUNTER: AtomicU16 = AtomicU16::new(1);
static CACHE: OnceLock<Vec<SyntaxKind>> = OnceLock::new();

// ❌ Re-exports com self:: (V14 — gap do linter, ADR-0004)
pub use self::entities::FileId;
pub use self::span::Span;

// ❌ I/O (V4)
use std::fs;

// ✓ Declaração de módulo — correcto
pub mod entities;
pub mod span;
pub mod syntax_kind;

// ✓ Newtype puro — correcto
pub struct FileId(NonZeroU16);

// ✓ Mutex em campo de struct, não static — correcto em L3
pub struct TsParser {
    subdirs_buffer: Mutex<Vec<Box<str>>>,
}
```

---

## Protocolo de nucleação (obrigatório antes de qualquer código)

1. Verificar se existe prompt em `00_nucleo/prompts/` para o módulo
2. **Prompt existe** → ler completamente
3. **Prompt não existe** → PARAR. Propor prompt. Não avançar.
4. Escrever testes que **falham** (a partir dos critérios do prompt)
5. Implementar para os testes passarem
6. Adicionar header de linhagem
7. `cargo build && crystalline-lint .` — zero violations
8. `crystalline-lint --fix-hashes .` se V5 disparar

---

## Header de linhagem obrigatório

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/<nome>.md
//! @prompt-hash <sha256[0..8]>
//! @layer L<n>
//! @updated YYYY-MM-DD
```

---

## lab/typst-original/ — quarentena

- **Nunca importar** em L1–L4
- **Nunca modificar**
- **Consultar** para entender o código antes de migrar um módulo
- Para compilar separadamente: `cd lab/typst-original && cargo build`

---

## Comandos

```bash
cargo build
cargo test -p typst-core
cargo test
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint --format sarif . > results.sarif
```

---

## ADRs — ler antes de decidir

| ADR | Estado | Decisão |
|-----|--------|---------|
| ADR-0001 | PROPOSTO | Estratégia de migração; comemo em L1; lab/ em [excluded] |
| ADR-0002 | IDEIA | Hierarquia de contenção como mecanismo de layout |
| ADR-0003 | IDEIA | Coexistência de comemo e hierarquia de contenção |
| ADR-0004 | IMPLEMENTADO | FileId interner → L3; ecow pendente; self:: dispara V14 |

ADRs `IMPLEMENTADO` são vinculativos.
ADRs `IDEIA` são referência conceptual — não implementar.

---

## Sequência de migração

```
✓ Passo 0 — estrutura base, lab/, workspace cristalino
✓ Passo 1 — FileId, SyntaxKind, Span migrados
            SyntaxNode e Source bloqueados (ecow, parse())
→ Passo 2 — decidir ecow; migrar SyntaxNode
  ...
  Passo 10 — isolamento de comemo em L3 (ADR-0001 Opção B)
```

Ficheiros de cada passo em `00_nucleo/materialization/` —
**só aceder quando explicitamente indicado**.
