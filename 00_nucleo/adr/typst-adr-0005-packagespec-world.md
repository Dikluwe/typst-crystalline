# ⚖️ ADR-0005: PackageSpec (DTO pattern) e World trait

**Status**: `PROPOSTO`
**Data**: 2026-03-22

---

## Contexto

O Passo 3 migra dois módulos com dependências externas problemáticas:

**`PackageSpec`** depende de `serde` (derives em todos os tipos) e
`unscanny` (parsing `FromStr`). Nenhum dos dois pertence a L1 —
`serde` é serialização de infraestrutura, `unscanny` é parsing de
strings de configuração.

**`World` trait** usa `comemo::Tracked<dyn World>` nas assinaturas
do pipeline de compilação. `comemo` está autorizado em L1 (ADR-0001),
mas a questão é se `Tracked` aparece *dentro* da declaração da trait
ou apenas nas funções que a *consomem*.

---

## Decisão 1 — PackageSpec: DTO pattern

`serde` e `unscanny` não entram em L1. Padrão:

```
L1: PackageSpec puro — apenas tipos de domínio, sem derives externos
L3: PackageSpecDto — com Serialize/Deserialize
L3: impl TryFrom<PackageSpecDto> for PackageSpec
L3: impl FromStr for PackageSpec — usa unscanny, fica em L3
```

Em L1, `PackageSpec` usa apenas `String` e tipos primitivos:

```rust
// 01_core/entities/package_spec.rs
pub struct PackageSpec {
    pub namespace: String,
    pub name:      String,
    pub version:   PackageVersion,
}

pub struct PackageVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
```

`Display`, `PartialOrd`, `Ord` para versões são puros — migram para L1.
`FromStr` para parsing de `@preview/algo:0.1.0` fica em L3.

Em L3:
```rust
// 03_infra/dto/package_spec_dto.rs
#[derive(Serialize, Deserialize)]
pub struct PackageSpecDto {
    pub namespace: String,
    pub name:      String,
    pub version:   String,
}

impl TryFrom<PackageSpecDto> for PackageSpec {
    type Error = PackageParseError;
    fn try_from(dto: PackageSpecDto) -> Result<Self, Self::Error> {
        // parse dto.version com unscanny — fica confinado a L3
        ...
    }
}
```

---

## Decisão 2 — World trait: B3 + stubs opacos (decisão final)

**Diagnóstico confirmado**: `#[comemo::track]` está na declaração
da trait (Possibilidade B). `comemo` está autorizado em L1
(ADR-0001), mas não deve aparecer no contrato puro de `World`.

**Decisão: B3 com stubs imediatos.**

### Implementação final: B3 + blanket impl com delegação explícita

O padrão completo — verificado como compatível com `comemo`:

```rust
// 01_core/contracts/world.rs

// Contrato puro — sem comemo, testável com mocks simples
pub trait World: Send + Sync {
    fn library(&self)            -> &Library;
    fn book(&self)               -> &FontBook;
    fn main(&self)               -> FileId;
    fn source(&self, id: FileId) -> FileResult<Source>;
    fn file(&self, id: FileId)   -> FileResult<Bytes>;
    fn font(&self, index: usize) -> Option<Font>;
    fn today(&self, offset: Option<i64>) -> Option<Datetime>;
}

// Contrato de performance — comemo autorizado via ADR-0001
// #[comemo::track] aqui, não em World
#[comemo::track]
pub trait TrackedWorld: World {
    // Redeclara os métodos que precisam de rastreio
    // (necessário para que #[comemo::track] gere wrappers correctos)
    fn library(&self)            -> &Library;
    fn book(&self)               -> &FontBook;
    fn source(&self, id: FileId) -> FileResult<Source>;
    fn file(&self, id: FileId)   -> FileResult<Bytes>;
    fn font(&self, index: usize) -> Option<Font>;
    fn today(&self, offset: Option<i64>) -> Option<Datetime>;
}

// Blanket impl — qualquer World é automaticamente TrackedWorld
// Compatibilidade do ecossistema automática
impl<T: World> TrackedWorld for T {
    fn library(&self)            -> &Library      { World::library(self) }
    fn book(&self)               -> &FontBook     { World::book(self) }
    fn source(&self, id: FileId) -> FileResult<Source> { World::source(self, id) }
    fn file(&self, id: FileId)   -> FileResult<Bytes>  { World::file(self, id) }
    fn font(&self, index: usize) -> Option<Font>  { World::font(self, index) }
    fn today(&self, offset: Option<i64>) -> Option<Datetime> { World::today(self, offset) }
}
```

### Análise de custo — dívida técnica planeada

**Risco 1 — Dessincronização**: se `World::file` mudar de assinatura,
o blanket impl deixa de compilar. O erro não é silencioso — é
imediato em tempo de compilação. O custo é a correcção manual,
não a detecção.

**Risco 2 — Verbosidade**: ~14 linhas de delegação pura. Custo fixo,
não proporcional à complexidade do domínio.

**Ganho 1 — Pureza de L1**: testes de `01_core` não importam
`comemo`. `World` pura é testável com mocks simples sem o motor
de memoização.

**Ganho 2 — Inversão de dependência**: o pipeline (L4) usa
`TrackedWorld`. Não sabe que por trás existe `comemo` — apenas
que o contrato exige rastreabilidade.

**Ganho 3 — Compatibilidade automática**: qualquer `impl World for T`
ganha `TrackedWorld` via blanket impl. Ecossistema externo não
precisa de mudanças.

### Data de vencimento da dívida

O boilerplate de delegação é temporário por design. No Passo 10,
`comemo` é isolado em L3 (ADR-0001 Opção B). Quando isso acontecer,
`TrackedWorld` desaparece de L1 — o blanket impl e a delegação
desaparecem com ela. Dívida com plano de eliminação conhecido.

### Verificação empírica — resultado

**`TrackedWorld: World` (supertrait) não compila com `#[comemo::track]`.**

A proc-macro gera tipos internos (`__ComemoSurface`) que precisam de
implementar todos os supertraits de `TrackedWorld`. Como `World` seria
supertrait, os tipos gerados teriam de implementar `World` — o que
não fazem automaticamente.

**Fix aplicado**: supertrait removido. `TrackedWorld` é uma trait
independente que redeclara todos os métodos (incluindo `main()`).
O blanket impl delega para `World`. A separação semântica é mantida
por convenção e pelo blanket impl, não por herança formal.

```rust
// SEM supertrait — comemo não suporta
#[comemo::track]
pub trait TrackedWorld {           // não TrackedWorld: World
    fn library(&self) -> &Library;
    fn main(&self) -> FileId;
    // ... todos os métodos redeclarados
}

impl<T: World> TrackedWorld for T {
    fn library(&self) -> &Library { World::library(self) }
    fn main(&self) -> FileId { World::main(self) }
    // ...
}
```

**Implicação**: a invariante "TrackedWorld declara os mesmos métodos
que World" é enforçada pelo blanket impl (dessincronização detectada
em tempo de compilação quando `World` muda) mas não pelo sistema de
tipos via supertrait. Documentar nos comentários de `world.rs`.

### Stubs opacos para os 7 tipos bloqueantes

Os tipos que `World` retorna ainda não existem em L1. Criar
newtypes opacos que compilam agora e são substituídos nos passos
seguintes. O interior do newtype pode mudar sem alterar `World`.

```rust
// 01_core/entities/world_types.rs
/// Conteúdo binário de um ficheiro carregado.
/// Interior opaco — pode mudar de Vec<u8> para Bytes real no Passo 5.
pub struct Bytes(Vec<u8>);

/// Fonte carregada. Opaco até Font ser migrado no Passo 5.
pub struct Font(Vec<u8>);

/// Biblioteca de funções e valores do Typst.
/// Opaco até Library ser migrada no Passo 4.
pub struct Library(());

/// Livro de fontes com metadados.
/// Opaco até FontBook ser migrado no Passo 5.
pub struct FontBook(());

/// Data e hora.
pub struct Datetime { year: i32, month: u8, day: u8 }

/// Resultado de operação de ficheiro.
pub type FileResult<T> = Result<T, FileError>;

/// Erro de acesso a ficheiro.
#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error("not found")]
    NotFound,
    #[error("access denied")]
    AccessDenied,
    #[error("{0}")]
    Other(String),
}
```

`Source` já está planeada para Passo 4 — criar stub mínimo:

```rust
/// Ficheiro de texto carregado em memória.
/// Stub — substituído quando parse() migrar no Passo 4.
pub struct Source {
    pub id:   FileId,
    pub text: String,
}
```

### Por que stubs opacos são correcto aqui

**Custo zero de mudança**: quando `Source` real chegar no Passo 4,
muda apenas o interior do newtype. `World` não percebe.

**Testabilidade imediata**: testes de L1 podem usar `World` com
mocks sem precisar do filesystem, motor de fontes ou parser.

**Propriedade do domínio**: `FileError` com `String` pertence ao
domínio de L1. Não depende de tipos de erro de bibliotecas externas.

### O que este ADR não decide

- A representação interna final de `Bytes`, `Font`, `Library`,
  `FontBook` — decidida nos passos 4 e 5
- Se `LazyHash<T>` dos métodos originais é preservado ou substituído
  — avaliar no Passo 4 quando `Library` migrar de facto
- A implementação concreta de `World` em L3 — Passo 5

---

## Adenda — resultado do diagnóstico

**`comemo` em `World`**: Caso B confirmado.
```
linha 59: #[comemo::track]
pub trait World: Send + Sync {
```

**Tipos bloqueantes identificados**:
`LazyHash<Library>`, `FontBook`, `Source`, `FileResult`, `Bytes`,
`Font`, `Datetime` — todos tratados com stubs opacos neste passo.

**`PackageVersion::compiler()`** chama `typst_utils::version()` —
dependência externa. Não migra para L1 neste passo.

**Decisão aplicada**: B3 + stubs. `World` pura em L1 sem `comemo`.
`TrackedWorld` estende com `comemo` para o pipeline incremental.

---

## Prompts afectados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `00_nucleo/prompts/entities/package-spec.md` | Criar — PackageSpec puro em L1 |
| `00_nucleo/prompts/infra/package-spec-dto.md` | Criar — DTO em L3 |
| `00_nucleo/prompts/contracts/world.md` | Criar — World trait em L1 |

---

## Consequências

### ✅ Positivas

- `serde` e `unscanny` confirmadamente fora de L1
- O padrão DTO generaliza-se: qualquer tipo de domínio que precise
  de serialização usa este padrão — não um caso especial
- `World` em `01_core/contracts/` torna explícito o contrato central
  do compilador — o que o Typst já tinha implicitamente

### ❌ Negativas

- DTO é verboso — `PackageSpecDto` duplica os campos de `PackageSpec`
- Se `PackageSpec` tiver muitas variantes, o número de DTOs cresce
- `FromStr` em L3 significa que parsing de specs de pacotes não é
  testável sem L3 — testes de paridade podem ser mais complexos

### ⚙️ Neutras

- V11 (DanglingContract) dispara para `World` em `contracts/`
  até L3 ter implementação — usar `V11 = { level = "warning" }`
  em `[rules]` do `crystalline.toml` (suportado a partir do
  ADR-0014 do crystalline-lint). Restaurar para `error` no Passo 5.

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Autorizar `serde` em L1 | Zero DTOs | L1 dependente de serde; mudanças de serde afectam domínio |
| `serde` com feature flag em L1 | Opcional | Feature flags em L1 são um cheiro arquitectural |
| PackageSpec apenas em L3 | Sem DTO | Domínio sem o conceito de pacote — semanticamente incorrecto |

---

## Referências

- ADR-0001 — `comemo` autorizado em L1; isolamento no Passo 10
- ADR-0004 — DTO pattern decidido para PackageSpec
- V11 (DanglingContract) — trait sem implementação
- `lab/typst-original/crates/typst-syntax/src/package.rs`
- `lab/typst-original/crates/typst/src/lib.rs`
