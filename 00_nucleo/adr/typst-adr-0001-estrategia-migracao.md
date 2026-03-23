# ⚖️ ADR-0001: Estratégia de Migração do Typst para a Arquitetura Cristalina

**Status**: `PROPOSTO`
**Data**: 2026-03-22
**Repositório**: fork de https://github.com/typst/typst

---

## Contexto

O repositório Typst é um compilador Rust estruturado como workspace
com múltiplas crates (`typst-syntax`, `typst-eval`, `typst-layout`,
`typst-pdf`, `typst-cli`, etc.). A arquitectura interna segue um
pipeline limpo:

```
Parsing → Evaluation → Layout → Export → CLI
```

O objectivo é migrar este projecto para a Arquitetura Cristalina
(Tekt), onde cada camada tem contratos explícitos, lógica pura em L1
e I/O isolado em L3.

### O problema central: `comemo`

O Typst usa `comemo` — um crate externo de memoização incremental —
de forma pervasiva em todo o pipeline de compilação. `comemo::Tracked<T>`
aparece em assinaturas de funções de domínio, e `#[comemo::memoize]`
decora funções que deveriam ser L1 puras.

Exemplos no código original:

```rust
// Em typst-eval — lógica de domínio com comemo na assinatura
pub fn eval(
    world: Tracked<dyn World>,
    route: Tracked<Route>,
    source: &Source,
) -> SourceResult<Module> { ... }

// Em typst-layout
pub fn layout_document(
    engine: &mut Engine,   // Engine contém Tracked<dyn World>
    content: &Content,
) -> SourceResult<PagedDocument> { ... }
```

`comemo::Tracked<T>` é um wrapper de infraestrutura que rastreia
acessos para invalidação incremental. A sua presença em assinaturas
de domínio cria uma dependência de L1 em infraestrutura externa —
exactamente o que V14 (ExternalTypeInContract) detecta.

### Por que `comemo` é diferente de outros externos

A maioria dos externos em L1 são atalhos preguiçosos (ex: usar
`tokio` em domínio em vez de injectar via trait). `comemo` é uma
decisão de design intencional que resolve dois problemas reais e
distintos:

**1. Compilação incremental de alta performance**
`comemo` rastreia acessos via `Tracked<T>` e invalida selectivamente
o que mudou. Remover `comemo` sem substituição equivale a remover
a compilação incremental — uma regressão funcional inaceitável.

**2. Compatibilidade com o ecossistema Typst actual**
O Typst tem um ecossistema de utilizadores, pacotes e integrações
que dependem do comportamento actual do compilador. A `World` trait
pública usa `comemo::Tracked<dyn World>` — qualquer código que
implementa `World` (ex: `typst-cli`, integrações externas) depende
desta assinatura.

Remover `comemo` da interface pública quebraria toda a cadeia de
compatibilidade. A migração cristalina não pode impor esta ruptura.

A combinação dos dois motivos torna `comemo` não-negociável durante
a migração — é infraestrutura com contrato público estabelecido,
não uma dependência opcional.

---

## Decisão

### Fase 1 — Código original para `lab/`

Todo o código Typst actual vai para `lab/typst-original/` intacto.
Isso serve como:
- Referência de comportamento correcta durante a migração
- Base para testes de paridade (output idêntico)
- Ponto de retorno se uma decisão de migração se revelar inviável

`lab/` nunca é importado por L1–L4. A migração é unidireccional:
código sai de `lab/` para as camadas cristalinas, nunca o inverso.

### Fase 2 — Topologia de camadas para o Typst cristalino

```
01_core/
  entities/     ← tipos de domínio: Source, Content, Module,
                  PagedDocument, Frame, Span, SyntaxTree, ...
  contracts/    ← World trait (desacoplada de comemo),
                  FileReader, FontLoader, PackageResolver
  rules/        ← pipeline puro: parse, eval, layout, export
                  (sem comemo — ver decisão sobre comemo abaixo)

02_shell/
  cli/          ← typst-cli refactorado (comandos compile, watch, fonts)

03_infra/
  world/        ← implementação de World: filesystem, HTTP, cache
  fonts/        ← carregamento de fontes do disco
  packages/     ← resolução de pacotes Typst
  export/       ← PDF, SVG, PNG, HTML (I/O de saída)

04_wiring/
  main.rs       ← composição: instanciar World, injectar em pipeline
```

### Fase 3 — Decisão sobre `comemo`

Três opções para tratar `comemo` na migração:

**Opção A — `comemo` autorizado em `[l1_allowed_external]`**

```toml
[l1_allowed_external]
rust = ["thiserror", "comemo"]
```

`comemo` fica visível em L1 como dependência declarada e intencional.
V14 não dispara. A whitelist documenta a decisão explicitamente.

Prós: zero refactorização do pipeline; comportamento incremental preservado.  
Contras: L1 conhece infraestrutura de memoização; viola o princípio
de pureza de L1 em sentido estrito.

**Opção B — `comemo` isolado atrás de trait em L1**

```rust
// 01_core/contracts/incremental.rs
pub trait IncrementalContext {
    fn track<T: ?Sized>(&self) -> TrackedRef<T>;
}

// L1 usa IncrementalContext — não comemo directamente
// L3 implementa IncrementalContext usando comemo internamente
```

Prós: L1 pura; comemo fica em L3 onde pertence.  
Contras: requer redesenho significativo das assinaturas do pipeline;
risco de perda de performance se o wrapper não for zero-cost.

**Opção C — Migração em duas fases com A → B**

Fase inicial: Opção A (pragmática, permite arrancar).  
Fase futura: Opção B quando a migração estiver estável e os
contratos de L1 forem bem compreendidos.

**Decisão: Opção C.**

A migração arranca com `comemo` em `[l1_allowed_external]`. Quando
o pipeline cristalino estiver funcional e os testes de paridade
passarem, um ADR dedicado ao isolamento de `comemo` em L3 será
escrito com base em experiência concreta.

### Fase 4 — `World` trait

O Typst já tem uma `World` trait que define o contrato entre o
compilador e o ambiente (filesystem, fontes, pacotes). Esta trait
vai para `01_core/contracts/world.rs` — é o exemplo mais puro de
design cristalino que o Typst já tem.

A única mudança: remover `comemo::Tracked` da assinatura pública
de `World` (na Opção B) ou preservar temporariamente (Opção A).

---

## Sequência de migração

A migração é incremental — o projecto deve compilar e os testes
de paridade devem passar a cada passo.

```
Passo 0: lab/typst-original/ criado, Cargo.toml workspace inicial
Passo 1: 00_nucleo/adr/ e 00_nucleo/prompts/ criados
Passo 2: 01_core/entities/ — tipos de domínio migrados de typst-syntax
Passo 3: 01_core/contracts/ — World trait e contratos de I/O
Passo 4: 01_core/rules/ — pipeline parse→eval→layout (sem export)
Passo 5: 03_infra/ — implementações de World, fontes, pacotes
Passo 6: 03_infra/export/ — PDF, SVG, PNG, HTML
Passo 7: 02_shell/ — CLI refactorado
Passo 8: 04_wiring/ — composição final
Passo 9: testes de paridade — output idêntico ao lab/typst-original/
Passo 10: ADR sobre isolamento de comemo (Opção B)
```

Cada passo tem um critério de conclusão:
- `cargo build` sem erros
- `crystalline-lint .` com zero violations
- testes de paridade do passo passam

---

## O que este ADR não decide

- Como migrar os testes do Typst (unitários e de integração)
- Se `comemo` será eventualmente isolado em L3 (ADR separado)
- Performance: a migração não deve degradar performance, mas
  benchmarks detalhados são para após a paridade funcional
- Plugins WASM: `typst-eval` tem suporte a plugins WASM;
  a sua camada será decidida durante o Passo 4

---

## Consequências

### ✅ Positivas

- `lab/typst-original/` serve como referência de comportamento
  imutável durante toda a migração
- A topologia cristalina torna explícitas as dependências que o
  Typst já tem implicitamente (World como contrato central)
- V14 vai revelar todos os externos que entram em L1 — alguns
  serão autorizados (comemo), outros serão surpresas a corrigir

### ❌ Negativas

- A migração é longa — o Typst tem ~50k linhas de Rust
- `comemo` em `[l1_allowed_external]` é tecnicamente uma concessão
  arquitectural; será revista no Passo 10
- Testes de paridade requerem manter `lab/typst-original/`
  compilável durante toda a migração

### ⚙️ Neutras

- O workspace Cargo vai ter membros nas camadas cristalinas E
  em `lab/typst-original/` em paralelo durante a migração
- `crystalline-lint` deve ser configurado para ignorar
  `lab/typst-original/` via `[excluded]`

---

## crystalline.toml inicial

```toml
[project]
root = "."

[languages]
rust = { grammar = "tree-sitter-rust", enabled = true }

[layers]
L0  = "00_nucleo"
L1  = "01_core"
L2  = "02_shell"
L3  = "03_infra"
L4  = "04_wiring"

[excluded]
build = "target"
vcs   = ".git"
cargo = ".cargo"
# lab/ vai para [excluded], não [layers].
# Se fosse [layers], o linter analisaria o código de quarentena
# e dispararia V1 em todo o lab/typst-original/ — inaceitável.
# lab/ é quarentena total: excluída do linter, nunca importada
# por L1–L4. Ver Passo 0 do processo de migração.
lab   = "lab"

[l1_allowed_external]
# comemo: memoização incremental — decisão intencional, ver ADR-0001
# será isolado em L3 no Passo 10
rust = ["thiserror", "comemo"]

[orphan_exceptions]
# ADRs não materializam código directamente
"00_nucleo/adr/0001-estrategia-migracao.md" = "ADR — não materializa código"
```

---

## Referências

- Typst architecture: https://docs.rs/typst (pipeline Parse→Eval→Layout→Export)
- comemo: https://github.com/typst/comemo
- Arquitetura Cristalina: 00_nucleo/prompts/linter-core.md
- V14 ExternalTypeInContract: crystalline-lint ADR-0012
