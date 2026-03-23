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

**`ecow` não entra em L1** — ver Opção C abaixo.
**`serde`, `unscanny`** — não entram em L1. DTO pattern em L3.

Qualquer outro crate que V14 sinalize em L1 → criar ADR antes
de adicionar à whitelist. Não adicionar por conveniência.

---

## Padrão B3 — separação de contrato puro e contrato de performance

Quando um mecanismo de infraestrutura (ex: `comemo`) está acoplado
à declaração de um contrato de domínio, **não** usar Opção C
(newtype — funciona para dados passivos, não para mecanismos activos).
Usar B3: dois traits com blanket impl de delegação.

```rust
// Contrato puro — testável sem comemo
pub trait World: Send + Sync {
    fn library(&self) -> &Library;
    fn file(&self, id: FileId) -> FileResult<Bytes>;
}

// Contrato de performance — comemo autorizado via ADR-0001
// #[comemo::track] redeclara os métodos para gerar wrappers correctos
#[comemo::track]
pub trait TrackedWorld: World {
    fn library(&self) -> &Library;
    fn file(&self, id: FileId) -> FileResult<Bytes>;
}

// Blanket impl — qualquer World é automaticamente TrackedWorld
// Compatibilidade do ecossistema automática; dessincronização detectada
// em tempo de compilação quando World muda
impl<T: World> TrackedWorld for T {
    fn library(&self) -> &Library { World::library(self) }
    fn file(&self, id: FileId) -> FileResult<Bytes> { World::file(self, id) }
}
```

**Por que não Opção C aqui**: `comemo::Tracked` é um mecanismo
activo de rastreio. Embrulhá-lo sem invocar o rastreio seria
silenciosamente incorrecto.

**Dívida planeada**: o boilerplate de delegação desaparece no
Passo 10 quando `comemo` for isolado em L3 (ADR-0001 Opção B).

## Stubs opacos para tipos bloqueantes

Quando um tipo de domínio depende de outros tipos ainda não
migrados, criar newtypes opacos que compilam agora:

```rust
// Interior muda sem alterar a interface
pub struct Bytes(Vec<u8>);
pub struct Font(Vec<u8>);
pub struct Library(());       // substituído no Passo 4
pub struct FontBook(());      // substituído no Passo 5

pub type FileResult<T> = Result<T, FileError>;

#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error("not found")]    NotFound,
    #[error("access denied")] AccessDenied,
    #[error("{0}")]          Other(String),
}
```

**Custo zero**: quando o tipo real chegar, muda só o interior.
**Testabilidade**: L1 testa com mocks sem filesystem nem fontes.
**Propriedade**: erros de domínio em `String`, não em tipos externos.

Quando um tipo externo aparece na interface pública de um tipo
de domínio (ex: `EcoString` em `SyntaxNode`), o padrão correcto
**não** é autorizar o externo em L1. É definir um newtype próprio:

```rust
// 01_core/entities/syntax_text.rs — L1 define o contrato
pub struct SyntaxText(Arc<str>);

impl SyntaxText {
    pub fn as_str(&self) -> &str { &self.0 }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

// 03_infra — L3 faz a conversão na fronteira
impl From<EcoString> for SyntaxText {
    fn from(s: EcoString) -> Self {
        SyntaxText(Arc::from(s.as_str()))
    }
}
```

**Por que não Opção A** (autorizar o externo):
L1 ficaria dependente dos contratos da biblioteca externa.
Se `ecow` mudar a API, L1 muda.

**Por que não Opção B** (substituir por `Arc<str>` directamente):
L1 fica casado com `Arc<str>`. Se amanhã a performance exigir
outra representação, L1 muda novamente.

**Opção C**: L1 define o que é uma string de domínio. A
representação interna é um detalhe privado — pode mudar de
`Arc<str>` para `ecow` ou outra coisa sem alterar a interface.

### DTO pattern para serde em L3

Quando um tipo de domínio precisa de serialização:

```rust
// 01_core/entities/package_spec.rs — puro, sem serde
pub struct PackageSpec { pub name: String, pub version: Version }

// 03_infra/dto/package_spec_dto.rs — apenas em L3
#[derive(Serialize, Deserialize)]
struct PackageSpecDto { name: String, version: String }

impl From<PackageSpecDto> for PackageSpec { ... }
```

`serde` nunca entra em L1.

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
4. **Testes primeiro** — ver secção abaixo
5. Implementar para os testes passarem
6. Adicionar header de linhagem
7. `cargo build && crystalline-lint .` — zero violations
8. `crystalline-lint --fix-hashes .` se V5 disparar

---

## Testes primeiro — regra absoluta

**Nunca escrever código de produção antes dos testes.**

A IA tem tendência natural a escrever o código e depois os testes
que descrevem o que o código faz. Isso produz testes que são uma
sombra da implementação — não detectam bugs introduzidos durante
a materialização.

### Fase 1 — Testes (antes de qualquer implementação)

```
1. Ler a secção "Critérios de Verificação" do prompt L0
2. Escrever os testes no #[cfg(test)] do módulo
3. cargo test -p typst-core <módulo>
4. VERIFICAR QUE OS NOVOS TESTES FALHAM
```

**Se um teste passar sem código de produção existir, o teste
está errado.** Um teste que passa imediatamente não fornece
nenhuma garantia — é documentação executável do comportamento
actual, que pode ser o comportamento errado.

### Fase 2 — Implementação

```
5. Escrever o código de produção para os testes passarem
6. cargo test -p typst-core <módulo>
7. VERIFICAR QUE TODOS OS TESTES PASSAM
8. cargo build && crystalline-lint .
9. VERIFICAR ZERO VIOLATIONS
```

### Cobertura obrigatória

Cada cenário `Dado/Quando/Então` do prompt deve ter um teste.
Os **caminhos negativos** são obrigatórios:

```rust
// Para cada função pública em L1:
#[test] fn caso_normal() { ... }       // input válido → output correcto
#[test] fn caso_invalido() { ... }     // input inválido → erro correcto
#[test] fn caso_limite() { ... }       // zero, vazio, máximo

// Para tipos migrados de lab/typst-original/:
#[test] fn paridade_com_original() { ... }  // comportamento idêntico ao original
```

### O que fazer quando um teste passa imediatamente

**Caso 1 — O comportamento já está correcto**
Manter o teste com comentário:
```rust
// Contrato correcto — teste adicionado para prevenir regressão
```

**Caso 2 — O teste está mal escrito**
Reescrever até falhar, ou documentar por que é impossível
testar este comportamento com `cargo test`.

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

**Critério de correcção primário:**
```bash
cargo build && crystalline-lint .
# ✓ No violations found
```

**V11 configurável** (a partir do ADR-0014 do crystalline-lint):
`V11 = { level = "warning" }` em `[rules]` funciona. Usar durante
migração enquanto contratos em L1 não tiverem implementação em L3.

---

## ADRs — ler antes de decidir

| ADR | Estado | Decisão |
|-----|--------|---------|
| ADR-0001 | PROPOSTO | Estratégia de migração; comemo em L1; lab/ em [excluded] |
| ADR-0002 | IDEIA | Hierarquia de contenção como mecanismo de layout |
| ADR-0003 | IDEIA | Coexistência de comemo e hierarquia de contenção |
| ADR-0004 | IMPLEMENTADO | FileId interner → L3; ecow→SyntaxText (Opção C); self:: dispara V14 |
| ADR-0005 | PROPOSTO | PackageSpec DTO; World B3 (puro + TrackedWorld); stubs opacos |

ADRs `IMPLEMENTADO` são vinculativos.
ADRs `IDEIA` são referência conceptual — não implementar.

---

## Sequência de migração

```
✓ Passo 0 — estrutura base, lab/, workspace cristalino
✓ Passo 1 — FileId, SyntaxKind, Span migrados
✓ Passo 2 — SyntaxText (Opção C), SyntaxNode, SyntaxSet migrados
✓ Passo 3 — PackageSpec (DTO pattern), world_types (stubs),
            World + TrackedWorld (B3, sem supertrait — limitação comemo)
            69 testes
→ Passo 4 — parse(), Source real, pipeline eval/layout
  Passo 5 — SystemWorld em L3; V11 volta a ser verificado
  ...
  Passo 10 — isolamento de comemo em L3 (ADR-0001 Opção B)
```

Ficheiros de cada passo em `00_nucleo/materialization/` —
**só aceder quando explicitamente indicado**.
