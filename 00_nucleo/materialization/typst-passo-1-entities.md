# Passo 1 — Tipos de domínio: typst-syntax → 01_core/entities/

## Contexto

Ler antes de começar:
- `00_nucleo/adr/0001-estrategia-migracao.md`
- `lab/typst-original/crates/typst-syntax/src/` (estrutura actual)

O Passo 1 migra apenas os tipos de domínio puros de `typst-syntax`
— sem dependências externas, sem I/O, sem `comemo`. O parser em si
(`parse()`) fica para o Passo 4 junto com o resto do pipeline.

**Critério de selecção para este passo:**
Um tipo vai para `01_core/entities/` se e só se:
1. Não tem I/O (sem `std::fs`, sem `std::net`)
2. Não tem dependências externas além de `thiserror`
3. Representa um conceito de domínio do compilador Typst

**Sequência obrigatória por módulo:**
1. Criar o prompt L0 em `00_nucleo/prompts/`
2. Escrever testes que **falham**
3. Copiar/adaptar o código de `lab/typst-original/`
4. Verificar que os testes passam
5. `cargo build && crystalline-lint .` — zero violations

---

## Módulos a migrar neste passo

### 1. `FileId` e `PackageSpec`

**Origem**: `lab/typst-original/crates/typst-syntax/src/file.rs`

**Porquê L1**: identidade de ficheiros e pacotes é conceito de
domínio puro. `FileId` é um inteiro com metadata; `PackageSpec`
é um valor que identifica um pacote.

**Destino**: `01_core/entities/file_id.rs`

**Atenção**: `FileId` usa internamente um inteiro atómico para
geração de IDs únicos (`AtomicU16`). Isso é `AtomicXxx` em L1 —
V13 vai disparar se estiver em posição `static`.

Verificar no código original como `FileId` gera IDs:
```bash
grep -n "Atomic\|static\|OnceLock" lab/typst-original/crates/typst-syntax/src/file.rs
```

Se houver `static AtomicU16`, há duas opções:
- **A**: mover a geração de IDs para L3 (injectar via contrato)
- **B**: se o `AtomicU16` não estiver em posição `static` mas
  num campo de struct, V13 não dispara — verificar

Decidir e registar no prompt L0 antes de implementar.

**Prompt a criar**: `00_nucleo/prompts/entities/file-id.md`

---

### 2. `Span` e `SpanInterner`

**Origem**: `lab/typst-original/crates/typst-syntax/src/span.rs`

**Porquê L1**: spans são coordenadas de localização no texto
— conceito de domínio puro, sem I/O.

**Destino**: `01_core/entities/span.rs`

**Atenção**: `SpanInterner` provavelmente usa estado interno
para mapear spans para localizações. Verificar:
```bash
grep -n "Mutex\|RwLock\|static\|LazyLock" lab/typst-original/crates/typst-syntax/src/span.rs
```

Se `SpanInterner` tem estado mutável global, o mesmo raciocínio
de `FileId` aplica-se — V13 vai detectar.

**Prompt a criar**: `00_nucleo/prompts/entities/span.md`

---

### 3. `SyntaxNode` e `SyntaxKind`

**Origem**: `lab/typst-original/crates/typst-syntax/src/node.rs`
           `lab/typst-original/crates/typst-syntax/src/kind.rs`

**Porquê L1**: a árvore sintática e os seus tipos são o resultado
do parsing — conceito de domínio central. O parsing em si não
migra neste passo, mas o tipo que representa o resultado sim.

**Destino**: `01_core/entities/syntax_node.rs`
             `01_core/entities/syntax_kind.rs`

**Atenção**: `SyntaxNode` é uma árvore com referências contadas
(`Arc` internamente para partilha em parsing incremental).
`Arc` em L1 é permitido quando é estrutural ao tipo de domínio —
não é estado global mutável. V13 não dispara para `Arc` em campos
de struct; apenas para `static Mutex<T>` e similares.

Verificar que `SyntaxNode` não tem statics:
```bash
grep -n "^static\|^pub static" lab/typst-original/crates/typst-syntax/src/node.rs
```

**Prompt a criar**: `00_nucleo/prompts/entities/syntax-node.md`

---

### 4. `Source`

**Origem**: `lab/typst-original/crates/typst-syntax/src/source.rs`

**Porquê L1**: `Source` representa um ficheiro de texto já carregado
em memória — é um valor de domínio, não uma operação de I/O.
O I/O de carregar o ficheiro fica em L3; o tipo que representa
o conteúdo carregado fica em L1.

**Destino**: `01_core/entities/source.rs`

**Prompt a criar**: `00_nucleo/prompts/entities/source.md`

---

## Actualizar `01_core/Cargo.toml`

Após verificar as dependências reais de cada módulo migrado,
actualizar as dependências da crate:

```toml
[dependencies]
thiserror = { workspace = true }
comemo    = { workspace = true }  # apenas se algum tipo migrado precisar
# adicionar outras apenas se necessário — cada linha é uma decisão
```

Principio: adicionar dependências ao `Cargo.toml` de `01_core`
apenas quando um tipo migrado as exige. Não antecipar.

---

## Actualizar `00_nucleo/prompts/core.md`

O prompt `core.md` actual é um esqueleto ("Em migração.").
Após migrar os módulos deste passo, actualizar para documentar
os módulos existentes e os seus contratos.

---

## Verificação final do Passo 1

```bash
cargo test -p typst-core
cargo build
crystalline-lint .
```

Esperado:
- Testes dos módulos migrados passam
- Zero violations (ou apenas V1 de hashes a corrigir)
- Se V13 disparar: registar o caso, decidir A ou B, corrigir

```bash
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Ao terminar o Passo 1, reportar:
- Quais módulos foram migrados com sucesso
- Se V13 disparou e como foi resolvido
- Qualquer dependência externa inesperada encontrada

Essa informação vai para o ADR-0002 antes do Passo 2.
