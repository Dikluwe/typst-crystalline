# CLAUDE.md — typst-crystalline

Este ficheiro guia o Claude Code neste repositório.
Para decisões específicas: **ler os ADRs em `00_nucleo/adr/`**.

---

## ⚠️ Restrição de leitura — pastas de materialização e context

```
00_nucleo/materialization/
00_nucleo/context/
```

**Não ler estas pastas por iniciativa própria.**

- Só aceder quando explicitamente indicado com o path completo
- Nunca varrer ou listar o conteúdo destas pastas
- Nunca inferir o que fazer a partir do nome dos ficheiros
- Se uma tarefa parece relacionada com materialização ou contexto mas não
  referencia um ficheiro explícito: **perguntar antes de agir**

Motivo: os ficheiros de materialização são instruções sequenciais
com ordem e momento específicos. Os ficheiros de contexto contêm análises
e estados históricos. Lê-los fora de contexto quebra o estado do projecto
e pode gerar alucinações baseadas em discussões passadas.

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
| L1 | `01_core/` | Domínio puro. Zero I/O de sistema. Funções determinísticas. |
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
- Regras: pipeline parse → eval → layout (sem I/O de sistema)

**L1 nunca**:
- Lê ou escreve ficheiros (`std::fs`) — I/O de sistema
- Faz chamadas de rede — I/O de sistema
- Acede ao relógio do SO (`SystemTime::now()`, `OffsetDateTime::now_utc()`) — I/O de sistema
- Lê variáveis de ambiente (`std::env`) — I/O de sistema
- Tem estado global mutável (`static mut`, `static Mutex<T>`, `static OnceLock<T>`)
- Importa crates externos não declarados em `[l1_allowed_external]`
- Usa `Arc<Mutex<T>>` como estado partilhado entre threads (concorrência → L3)

**L1 pode e deve usar** (ADR-0029, ADR-0030):
- `Arc<T>` em campos de struct para clone O(1) no hot path de eval()
- `Vec`, `Box`, `String`, `HashMap` — gestão de memória RAM
- `EcoString`, `FxHashMap` e outras crates utilitárias declaradas em `[l1_allowed_external]`
- Estruturas de dados de alta performance quando o domínio as justifica

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
Símbolo de I/O de sistema em L1: `std::fs`, `std::net`, `std::process`.
Mover para L3 e injectar via trait em L1.

**V9 — PubLeak**
Import de L1 fora das portas declaradas em `[l1_ports]`.

**V11 — DanglingContract**
Trait em `01_core/contracts/` sem implementação em L2 ou L3.

**V13 — MutableStateInCore**
Estado global mutável em L1: `static mut`, `static Mutex<T>`,
`static OnceLock<T>`, `static LazyLock<T>`, `AtomicXxx` estático.
Mover o estado para L3 e injectar.

Nota: V13 detecta estado *global* mutável — não `Arc` em campos
de struct. `Arc<T>` como campo de struct é gestão de RAM e é
correcta em L1 (ADR-0029).

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

## Princípios de design Rust

### Enums sobre booleanos e Options parciais (obrigatório)

Estados inválidos devem ser irrepresentáveis. Substituir
`is_valid: bool` + `error: Option<String>` por enum que torna
a contradição impossível.

### Parse, não validar (obrigatório)

Funções recebem dados brutos e retornam tipos validados.
Downstream não revalida — o tipo é a prova.

### Newtype para primitivos de domínio (obrigatório)

Envolver primitivos que representam conceitos de domínio em
structs de campo único.

---

## Restrições por camada

### L1 — Core (lógica pura)

| Regra | Detalhe |
|-------|---------|
| Zero I/O de sistema | Sem `std::fs`, `std::net`, `std::process`, `std::env`, relógio do SO |
| Erros | `thiserror` com enums tipados — sem `anyhow` |
| Estado global | Sem `static mut`, `static Mutex<T>`, `static OnceLock<T>`, `static AtomicXxx` |
| Gestão de RAM | `Arc`, `Rc`, `Vec`, `Box` permitidos — são memória, não I/O (ADR-0029) |
| Concorrência | Nenhuma — sem `Arc<Mutex<T>>` como estado partilhado entre threads |
| Traits seladas | Para contratos não destinados a implementação externa |
| Typestate | Para operações com ordenação obrigatória |

### L2 — Shell (CLI e formatadores)

| Regra | Detalhe |
|-------|---------|
| Erros | `anyhow` permitido para propagação CLI |
| Imports | Apenas L1 — nunca L3 |
| Concorrência | Nenhuma — execução sequencial |

### L3 — Infra (implementações de I/O)

| Regra | Detalhe |
|-------|---------|
| Erros | `thiserror` — erros I/O tipados que mapeiam para erros L1 |
| Imports | Apenas L1 — nunca L2 ou L4 |
| Concorrência | `Arc<Mutex<T>>` ou canais permitidos para walking paralelo |

### L4 — Wiring (composição)

| Regra | Detalhe |
|-------|---------|
| Lógica | Zero — qualquer `if/else` de negócio é defeito estrutural |
| Erros | `anyhow` para propagação top-level |
| Concorrência | Spawning de threads apenas aqui |
| `expect()` em threads rayon | Proibido — panic numa thread rayon produz mensagem pouco informativa; usar `?` ou tratamento explícito |

---

## Regras de teste

### Atomização

Um teste por comportamento. Sem setup partilhado. Sem teste que
depende de outro. Mocks implementam traits L1 — nunca I/O real.

### Localização

Testes co-localizados no mesmo ficheiro via `#[cfg(test)]`.
Nunca ficheiros `_test.rs` separados.

### Cobertura mínima obrigatória por função

Para cada função pública em L1:
- Caminho feliz (input válido → output correcto)
- Caminho negativo (input inválido → erro correcto)
- Caso limite (zero, vazio, máximo)

---

## Decisão sobre performance em L1 (ADR-0029, ADR-0030)

**A pergunta correcta** ao escolher uma estrutura de dados em L1:

> "Esta estrutura tem efeitos colaterais no sistema operativo?"

- **Sim** → não pertence a L1 (mover para L3)
- **Não** → pertence a L1 se o domínio a justificar

**A pergunta incorrecta**:

> "Esta estrutura é uma optimização de performance?"

Performance de alocação e gestão de RAM não é "optimização" — é
parte do comportamento correcto de um compilador. Um compilador que
copia árvores O(n) quando podia partilhá-las via `Arc` não é mais
puro — é incorrectamente lento.

**Exemplos concretos**:

```rust
// ✓ Arc em campo de struct — gestão de RAM, correcto em L1 (ADR-0029)
pub struct Module(Arc<ModuleInner>);      // clone O(1) em eval()
pub struct SyntaxNode(Arc<NodeData>);     // partilha do CST sem cópia
pub struct Func(Arc<FuncRepr>);           // clone O(1) de closures

// ✓ EcoString em Value::Str — clone O(1) no hot path (ADR-0024)
Value::Str(EcoString),

// ✓ FxHashMap em Scope — hasher sem I/O, mais rápido (ADR-0018)
IndexMap<EcoString, Binding, FxBuildHasher>,

// ❌ Estado global mutável — V13, proibido
static INTERNER: Mutex<HashMap<PathBuf, FileId>> = Mutex::new(HashMap::new());
static COUNTER: AtomicU16 = AtomicU16::new(1);
static CACHE: OnceLock<Vec<SyntaxKind>> = OnceLock::new();

// ❌ Re-exports com self:: — V14 (ADR-0004)
pub use self::entities::FileId;

// ❌ I/O de sistema — V4
use std::fs;
use std::env;
```

**Sobre tipos tipográficos** (ADR-0029 revoga ADR-0028):
Ao materializar `Length`, `Abs`, `Rel`, `Angle`, `Ratio`, `Color`
em L1, diagnosticar a estrutura real no original Typst vanilla
antes de definir o tipo. Não simplificar por conveniência — a
representação vanilla é pura se não usa I/O de sistema.

**Sobre hashing em tipos pesados** (ADR-0031):
Tipos como `Source` que precisam de `Hash`/`Eq` para uso com
`comemo` devem pré-computar o hash na construção (`content_hash: u64`)
em vez de usar `LazyHash` com mutabilidade interior. O hash é
calculado uma vez, armazenado como campo imutável, consultado em O(1).

---

## DTO pattern para serde em L3

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

---

## Header de linhagem obrigatório

Todo ficheiro criado ou editado em L1–L4 deve começar com:

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/<nome>.md
//! @prompt-hash <sha256[0..8]>
//! @layer L<n>
//! @updated YYYY-MM-DD
```

---

## Quando um prompt está errado vs quando o código está errado

Antes de corrigir qualquer problema, determinar a origem:

**O prompt está errado quando:**
- O prompt especifica explicitamente um algoritmo ou padrão incorreto
- O prompt omite um caso que deveria cobrir
- O prompt contradiz um ADR existente

**Neste caso:** corrigir o prompt L0 primeiro. Só depois
rematerializar. Materializar código a partir de um prompt errado
reproduz o bug.

**O código está errado quando:**
- O prompt especifica o resultado correcto mas a implementação diverge
- Um detalhe de implementação não coberto pelo prompt foi feito incorrectamente

**Neste caso:** corrigir o código directamente.

---

## Restrições Rust — Todas as camadas

### Borrowing (obrigatório)

Preferir referências a valores owned. Sem `clone()` sem comentário
justificativo. Sem `Rc<RefCell<T>>` — usar o borrow checker.

### Lifetimes (obrigatório)

Lifetimes explícitos quando o compilador os exige. Sem `'static`
para evitar pensar em lifetimes.

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
cargo test -p typst-infra
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

---

## ADRs — ler antes de decidir

| ADR | Estado | Decisão |
|-----|--------|---------|
| ADR-0001 | IMPLEMENTADO | Estratégia de migração; comemo em L1; lab/ em [excluded] |
| ADR-0004 | IMPLEMENTADO | FileId interner → L3; SyntaxText(Arc<str>); self:: dispara V14 |
| ADR-0005 | IMPLEMENTADO | PackageSpec DTO; World B3; stubs opacos |
| ADR-0015 | IMPLEMENTADO | Remoção de ecow do parser (não revogada) |
| ADR-0016 | IMPLEMENTADO | LazyHash removido; early hashing via content_hash (ADR-0031) |
| ADR-0018 | IMPLEMENTADO | rustc_hash autorizado em L1; revoga ADR-0007 |
| ADR-0024 | IMPLEMENTADO | EcoString em Value::Str — clone O(1) em eval() |
| ADR-0026 | IMPLEMENTADO | Content como enum; Arc em Sequence antes do Passo 30 |
| ADR-0028 | **REVOGADA** | Revogada por ADR-0029 |
| ADR-0029 | ACCEPTED | Pureza física — Arc em struct permitido; revoga ADR-0028 |
| ADR-0030 | ACCEPTED | Performance de RAM é domínio de L1; corrige ADR-0004/0015 |
| ADR-0031 | ACCEPTED | Early hashing em Source; complementa ADR-0016 |

ADRs `IMPLEMENTADO` e `ACCEPTED` são vinculativos.
ADRs `REVOGADA` não devem ser seguidas — ler a ADR que as revoga.

---

## Sequência de migração

```
✓ Passos 0–25 — estrutura base, parser, eval, layout, export PDF, tipos tipográficos
  Passo 26 — DEBT-4 continuação: funções nativas de conversão e cálculo
  Passo 27 — DEBT-4 conclusão
  Passo 28–29 — DEBT-3: safety rails
  Passo 30 — DEBT-1: StyleChain
  Passo 31+ — DEBT-2: closures lazy, DEBT-6: eval_for_test
```

Ficheiros de cada passo em `00_nucleo/materialization/` —
**só aceder quando explicitamente indicado**.
