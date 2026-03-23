# Passo 3 — PackageSpec e contratos de domínio

## Contexto

Ler antes de começar:
- `00_nucleo/adr/0001-estrategia-migracao.md`
- `00_nucleo/adr/0004-passo1-descobertas.md` (secção PackageSpec e DTO)
- `lab/typst-original/crates/typst-syntax/src/package.rs`
- `lab/typst-original/crates/typst/src/lib.rs` (World trait)

Estado do Passo 2:
- ✓ `FileId`, `SyntaxKind`, `Span`, `SyntaxText`, `SyntaxNode`, `SyntaxSet`
- ✗ `PackageSpec` — adiado (serde, unscanny)
- ✗ `Source` — bloqueado (parse(), Passo 4)
- ✗ `World` trait — não avaliada ainda

Este passo tem duas partes independentes:
1. `PackageSpec` com DTO pattern (sem serde em L1)
2. `World` trait e contratos de I/O em `01_core/contracts/`

---

## Parte 1 — PackageSpec

### Diagnóstico obrigatório antes de escrever código

```bash
# Ver todos os tipos em package.rs
grep -n "^pub struct\|^pub enum\|^pub type" \
  lab/typst-original/crates/typst-syntax/src/package.rs

# Ver quais campos usam tipos de serde/unscanny directamente
grep -n "serde\|unscanny\|Scanner" \
  lab/typst-original/crates/typst-syntax/src/package.rs

# Ver o FromStr de PackageSpec — onde unscanny é usado
grep -n "FromStr\|impl.*PackageSpec\|fn from_str" \
  lab/typst-original/crates/typst-syntax/src/package.rs
```

Reportar o output antes de continuar.

### Tarefa 1a — Prompt L0 para PackageSpec

**Criar**: `00_nucleo/prompts/entities/package-spec.md`

O prompt deve documentar:

1. Tipos puros que vão para L1 (sem serde, sem unscanny):
   - `PackageSpec` — namespace + name + version
   - `PackageVersion` — major.minor.patch
   - `VersionBound` — se existir como tipo separado

2. O que fica em L3:
   - `PackageSpecDto` — com `#[derive(Serialize, Deserialize)]`
   - `impl FromStr for PackageSpec` — usa `unscanny`; parsing
     de strings como `@preview/algo:0.1.0` fica em L3
   - `impl From<PackageSpecDto> for PackageSpec`

3. Critérios de verificação:
   ```
   Dado PackageSpec { namespace: "preview", name: "algo", version: (0,1,0) }
   Quando comparado com outro PackageSpec idêntico
   Então são iguais (PartialEq)

   Dado PackageVersion(0, 1, 0)
   Quando comparado com PackageVersion(0, 2, 0)
   Então 0.1.0 < 0.2.0 (Ord)
   ```

### Tarefa 1b — Migrar PackageSpec para L1

**Destino**: `01_core/entities/package_spec.rs`

Regras:
- `PackageSpec`, `PackageVersion`, `VersionBound` migram sem
  qualquer atributo `#[derive(Serialize, Deserialize)]`
- Se algum campo usa um tipo de `unscanny` directamente
  (ex: `Scanner`), substituir por `String` ou criar newtype
- `Display` e `PartialOrd`/`Ord` para versões são puros — migram

**Não migrar**:
- `impl FromStr for PackageSpec` — usa `unscanny`, fica em L3
- Qualquer `impl Serialize/Deserialize`

Header obrigatório:
```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/package-spec.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-03-22
```

### Tarefa 1c — DTO em L3

**Criar**: `03_infra/dto/package_spec_dto.rs`

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/package-spec-dto.md
//! @prompt-hash <hash>
//! @layer L3
//! @updated 2026-03-22

use serde::{Deserialize, Serialize};
use typst_core::entities::package_spec::{PackageSpec, PackageVersion};

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageSpecDto {
    pub namespace: String,
    pub name: String,
    pub version: String,  // "0.1.0" — parse em From
}

impl TryFrom<PackageSpecDto> for PackageSpec {
    type Error = PackageParseError;
    fn try_from(dto: PackageSpecDto) -> Result<Self, Self::Error> {
        // parse dto.version → PackageVersion usando unscanny em L3
        todo!()
    }
}
```

`serde` e `unscanny` são dependências de `typst-infra`, não de
`typst-core`.

---

## Parte 2 — World trait e contratos

### Diagnóstico obrigatório

```bash
# Ver a World trait actual
grep -n "trait World\|fn main_source\|fn library\|fn file\|fn today\|fn packages" \
  lab/typst-original/crates/typst/src/lib.rs | head -30

# Ver quais tipos a World trait expõe na sua interface
grep -n "comemo::Tracked\|Source\|FileId\|Bytes\|PackageSpec" \
  lab/typst-original/crates/typst/src/lib.rs | head -20

# Ver se há outras traits relacionadas (FontLoader, etc.)
grep -n "^pub trait" lab/typst-original/crates/typst/src/lib.rs
```

Reportar o output antes de continuar.

### Decisão obrigatória sobre World e comemo

A `World` trait usa `comemo::Tracked<dyn World>` nas assinaturas
do pipeline. `comemo` está autorizado em L1 (ADR-0001 Opção C —
isolamento adiado para Passo 10).

Verificar: a `World` trait em si usa `comemo::Tracked` na sua
própria declaração, ou apenas as funções do pipeline que a
consomem?

```bash
grep -n "comemo" lab/typst-original/crates/typst/src/lib.rs
```

- Se `comemo` aparece apenas nas funções que consomem `World`
  (fora da trait): `World` migra para L1 limpa, sem `comemo`
- Se `comemo` aparece dentro da declaração da trait: registar
  e decidir antes de migrar

### Tarefa 2a — Prompt L0 para World

**Criar**: `00_nucleo/prompts/contracts/world.md`

`World` é o contrato central do compilador Typst — a fronteira
entre o domínio puro e o ambiente de execução. É o melhor
exemplo de design cristalino que o Typst já tem.

O prompt deve documentar:
- O que `World` declara (métodos: `main_source`, `library`,
  `file`, `today`, `packages` ou equivalentes)
- O que **não** entra na trait: implementação de filesystem,
  HTTP, cache — tudo isso fica em L3
- Critérios de verificação com mocks mínimos

### Tarefa 2b — Migrar World para 01_core/contracts/

**Destino**: `01_core/contracts/world.rs`

Apenas a declaração da trait — sem implementação.
A implementação concreta (`SystemWorld` ou equivalente) fica
em L3 para o Passo 5.

Se a trait declara métodos que retornam tipos ainda não migrados
(ex: `Bytes` para conteúdo de ficheiros), criar placeholders
ou newtypes mínimos em L1 conforme necessário — seguindo o
mesmo padrão de `SyntaxText`.

---

## Actualizar mod.rs

```rust
// 01_core/src/entities/mod.rs — adicionar:
pub mod package_spec;

// 01_core/src/lib.rs — adicionar:
pub mod contracts;

// 01_core/src/contracts/mod.rs — criar:
pub mod world;
```

---

## Verificação final

```bash
cargo test -p typst-core
cargo test -p typst-infra   # se typst-infra já tem testes
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Se V14 disparar para `serde` em `typst-core`:
- `serde` entrou em L1 indevidamente — reverter
- Verificar que derives de serde estão apenas em `typst-infra`

Se V11 disparar para `World` (DanglingContract):
- `World` foi declarada em `contracts/` mas ainda não tem
  implementação em L3 — esperado neste passo
- Adicionar a `[orphan_exceptions]` temporariamente:
  ```toml
  "01_core/contracts/world.rs" = "implementação em L3, Passo 5"
  ```

---

## Ao terminar o Passo 3, reportar

- Tipos migrados e tipos que ficaram bloqueados
- Se `comemo` aparece dentro da declaração de `World`
- Qualquer novo externo que V14 sinalizou
- Número de testes

Esta informação vai para ADR-0005 antes do Passo 4.
