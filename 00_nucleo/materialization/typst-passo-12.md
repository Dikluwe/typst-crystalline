# Passo 12 — Diagnóstico de Engine types e eval() esqueleto

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0016-adiamento-eval-typst-library.md`
- `00_nucleo/adr/typst-adr-0001-estrategia-migracao.md` (Opção C para comemo)
- `lab/typst-original/crates/typst-eval/src/lib.rs`
- `lab/typst-original/crates/typst-library/src/engine.rs` (ou equivalente)

Pré-condição: `cargo test` — 202 testes (180 L1 + 22 L3), zero violations.

Este passo é predominantemente diagnóstico. O código só começa na
Tarefa 3, e apenas depois do mapa de classificação da Tarefa 2 estar
completo e registado. Não é um passo de "implementar tudo" — é um
passo de "preparar a fundação correcta para eval()".

**Invariante crítica**: `01_core/src/rules/eval.rs` não deve importar
nada de `03_infra`. O acesso ao world é sempre via `TrackedWorld` (L1).
Se uma dependência de L3 for necessária para satisfazer eval(), é sinal
de classificação errada — parar e reclassificar.

---

## Tarefa 1 — Diagnósticos

**Parar aqui. Reportar output completo antes de qualquer decisão.**

### 1a — Assinatura real de eval()

```bash
grep -n -A 20 "^pub fn eval" \
  lab/typst-original/crates/typst-eval/src/lib.rs | head -40

grep "^use\|^pub use" \
  lab/typst-original/crates/typst-eval/src/lib.rs | head -20
```

### 1b — Routines

```bash
# Localizar
grep -rn "^pub struct Routines" lab/typst-original/crates/ | head -5

# Estrutura completa
grep -rA 50 "^pub struct Routines" \
  lab/typst-original/crates/typst-library/src/ | head -55

# Dependências do ficheiro
f=$(grep -rln "^pub struct Routines" lab/typst-original/crates/); \
grep "^use\|^extern" $f | grep -v "crate::\|super::\|std::" | head -20
```

### 1c — Route, Sink, Traced, Engine

```bash
# Localizar todos de uma vez
grep -rn "^pub struct Route\b\|^pub struct Sink\b\|^pub struct Traced\b\|^pub struct Engine\b" \
  lab/typst-original/crates/ | head -10

# Estrutura de cada um (ajustar path conforme output acima)
grep -A 20 "^pub struct Route\b" \
  lab/typst-original/crates/typst-library/src/engine.rs 2>/dev/null \
  || grep -rA 20 "^pub struct Route\b" lab/typst-original/crates/ | head -25

grep -A 20 "^pub struct Sink\b" \
  lab/typst-original/crates/typst-library/src/engine.rs 2>/dev/null \
  || grep -rA 20 "^pub struct Sink\b" lab/typst-original/crates/ | head -25

grep -A 20 "^pub struct Traced\b" \
  lab/typst-original/crates/typst-library/src/engine.rs 2>/dev/null \
  || grep -rA 20 "^pub struct Traced\b" lab/typst-original/crates/ | head -25

grep -A 20 "^pub struct Engine\b" \
  lab/typst-original/crates/typst-library/src/engine.rs 2>/dev/null \
  || grep -rA 20 "^pub struct Engine\b" lab/typst-original/crates/ | head -25

# Dependências externas do ficheiro de engine
f=$(grep -rln "^pub struct Engine\b" lab/typst-original/crates/); \
grep "^use\|^extern" $f | grep -v "crate::\|super::\|std::" | head -20
```

### 1d — Styles (aviso — não migrar, apenas mapear)

```bash
# Onde Styles é definido e o que contém
grep -rn "^pub struct Styles\b" lab/typst-original/crates/ | head -5
grep -rA 15 "^pub struct Styles\b" lab/typst-original/crates/ | head -20

# Dependências de styles.rs
f=$(grep -rln "^pub struct Styles\b" lab/typst-original/crates/); \
grep "^use\|^extern" $f 2>/dev/null \
  | grep -v "crate::\|super::\|std::" | head -15
```

Este diagnóstico é apenas para confirmar que Styles é complexo e não
deve ser migrado neste passo. Não iniciar migração de Styles.

### 1e — Scopes<'a>

```bash
grep -rn "^pub struct Scopes\b" lab/typst-original/crates/ | head -5
grep -rA 20 "^pub struct Scopes\b" lab/typst-original/crates/ | head -25
```

### 1f — Tamanho de typst-eval

```bash
find lab/typst-original/crates/typst-eval/src -name "*.rs" \
  | sort | xargs wc -l | tail -5
```

---

## Tarefa 2 — Mapa de classificação

Preencher após os diagnósticos. As classificações abaixo são
orientações baseadas na natureza conhecida de cada tipo — confirmar
ou corrigir com base no output real.

### Routines — provável: stub opaco em L1 (por agora)

`Routines` é a "vtable de execução" do Typst: uma struct que agrega
ponteiros de função para as operações fundamentais do compilador
(avaliar elementos, aplicar show rules, etc.). É lógica de domínio,
mas depende de `Content`, `Func`, e outros tipos ainda não migrados.

**Classificação esperada**: stub `Routines(())` em L1 para este passo.
Migração real quando `Content` e `Func` estiverem em L1.

Se o diagnóstico mostrar que `Routines` é apenas uma struct com
campos de função pointer sem deps externas → migrar directamente.

### Route — domínio puro, L1

`Route` é uma pilha de `FileId` que controla ciclos de importação
durante eval(). Evita recursão infinita quando A importa B que importa A.
É lógica pura — sem I/O, sem deps externas esperadas.

**Classificação esperada**: tipo real em L1.

```rust
// Estrutura provável — confirmar com diagnóstico
pub struct Route {
    // pilha de FileId para detecção de ciclos
    // ou Vec<FileId>, ou similar
}
```

Se o diagnóstico confirmar → implementar como tipo real, não stub.

### Sink — domínio puro, L1

`Sink` acumula avisos e erros durante eval(). É essencialmente um
`Vec<SourceDiagnostic>` com uma interface de escrita. `SourceDiagnostic`
já existe em L1.

**Classificação esperada**: tipo real simples em L1.

```rust
pub struct Sink {
    diagnostics: Vec<SourceDiagnostic>,
}

impl Sink {
    pub fn new() -> Self;
    pub fn warn(&mut self, diag: SourceDiagnostic);
    pub fn into_diagnostics(self) -> Vec<SourceDiagnostic>;
    pub fn is_empty(&self) -> bool;
}
```

Confirmar que o original não tem deps externas em Sink além de
`SourceDiagnostic`.

### Traced — stub no-op em L1

`Traced` é para profiling e debugging do compilador. Análogo a
`timing_scope!` (ADR-0006) — instrumentação que não afecta o
output semântico.

**Classificação esperada**: stub no-op em L1. Nunca vai para L3.

```rust
/// Stub no-op para rastreio de execução.
/// ADR-0006: instrumentação removida de L1 — religação futura.
pub struct Traced(());
```

Se o diagnóstico mostrar que `Traced` tem estado real necessário
para eval() funcionar correctamente → reclassificar e reportar.

### Engine — avaliar após diagnóstico

`Engine` provavelmente é uma composição de `Routines` + `TrackedWorld`
+ `Sink` + outros. Se for apenas composição de tipos já classificados,
pode ser uma struct real em L1. Se tiver deps externas → stub.

**Classificação esperada**: depende do diagnóstico.

### Styles — NÃO migrar neste passo

`Styles` é o sistema de propriedades encadeadas do Typst. É tão
complexo quanto `eval()` em si — dependências de vtable dinâmica,
`Content`, show rules. Qualquer referência a `Styles` em eval() →
substituir por stub `Styles(())` sem tentar migrar.

**Regra**: se o esqueleto de eval() precisar de `Styles` para compilar
→ criar stub opaco. Não iniciar migração de `Styles`.

---

## Tarefa 3 — Implementar tipos classificados

### 3a — Stubs para tipos que ficam como stub

Adicionar a `world_types.rs` ou criar ficheiros dedicados conforme
necessário:

```rust
// Stubs para engine types — world_types.rs ou ficheiros próprios

/// Vtable de execução do compilador Typst.
/// Stub — migração após Content e Func estarem em L1. ADR-0016.
pub struct Routines(());

/// Estado de rastreio de execução. Stub no-op. ADR-0006.
pub struct Traced(());

/// Sistema de propriedades encadeadas. Stub — NÃO migrar neste passo.
pub struct Styles(());
```

### 3b — Route real (se confirmado como domínio puro)

**Criar**: `01_core/src/entities/route.rs`

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/route.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-03-27

use crate::entities::file_id::FileId;

/// Controla ciclos de importação durante eval().
///
/// Mantém uma pilha de FileId em avaliação. Se eval() tentar
/// avaliar um ficheiro já na pilha, há um ciclo de importação.
pub struct Route {
    // Ajustar conforme estrutura real do diagnóstico
    ids: Vec<FileId>,
}

impl Route {
    pub fn new() -> Self { Self { ids: Vec::new() } }

    /// Verifica se um FileId já está na rota actual (ciclo).
    pub fn contains(&self, id: FileId) -> bool {
        self.ids.contains(&id)
    }

    /// Empurra um FileId para a rota. Retornar guard de pop automático.
    pub fn push(&mut self, id: FileId) {
        self.ids.push(id);
    }

    pub fn pop(&mut self) {
        self.ids.pop();
    }
}

impl Default for Route {
    fn default() -> Self { Self::new() }
}
```

### 3c — Sink real (se confirmado como domínio puro)

**Criar**: `01_core/src/entities/sink.rs`

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/sink.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-03-27

use crate::entities::source_result::SourceDiagnostic;

/// Colector de diagnósticos durante eval().
pub struct Sink {
    diagnostics: Vec<SourceDiagnostic>,
}

impl Sink {
    pub fn new() -> Self { Self { diagnostics: Vec::new() } }

    pub fn warn(&mut self, diag: SourceDiagnostic) {
        self.diagnostics.push(diag);
    }

    pub fn is_empty(&self) -> bool { self.diagnostics.is_empty() }

    pub fn into_diagnostics(self) -> Vec<SourceDiagnostic> {
        self.diagnostics
    }
}

impl Default for Sink {
    fn default() -> Self { Self::new() }
}
```

### 3d — Scopes<'a> em rules (se necessário para eval())

Se `Scopes<'a>` for necessário para o esqueleto de eval():

**Criar**: `01_core/src/rules/scopes.rs`

```rust
use crate::entities::{scope::Scope, value::Value};

/// Pilha de âmbitos durante avaliação.
pub struct Scopes<'a> {
    top:    &'a Scope,
    scopes: Vec<Scope>,
}

impl<'a> Scopes<'a> {
    pub fn new(top: &'a Scope) -> Self {
        Self { top, scopes: Vec::new() }
    }

    pub fn push(&mut self) { self.scopes.push(Scope::new()); }

    pub fn pop(&mut self) -> Option<Scope> { self.scopes.pop() }

    pub fn define(&mut self, name: impl Into<String>, value: Value) {
        if let Some(s) = self.scopes.last_mut() { s.define(name, value); }
    }

    /// Pesquisa do âmbito mais local para o global.
    pub fn get(&self, name: &str) -> Option<&Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) { return Some(v); }
        }
        self.top.get(name)
    }
}
```

---

## Tarefa 4 — eval() esqueleto

**Apenas executar após Tarefa 3 completa e compilando.**

### Invariante obrigatória

`eval.rs` em L1 não importa nada de `03_infra`. Se uma dep de
L3 parecer necessária, é classificação errada — parar e resolver.

### O que o esqueleto entrega

O esqueleto não avalia Typst. Entrega:
1. Assinatura correcta — compila com os tipos reais
2. Retorna `Module` real (com `Scope` vazio)
3. É o ponto de partida para implementação incremental de eval()

O que **não** faz ainda: resolver variáveis, avaliar expressões,
aplicar show rules, produzir Content. Isso requer `Value` real.

### Sobre Value(()) e o subset necessário

Com `Value(())` stub, eval() compila mas não consegue resolver
variáveis durante a travessia da AST. Para execução real mínima
(ex: `#let x = 1` → `x` resolve para `1`), serão necessárias
pelo menos as variantes `Value::Int`, `Value::Float`, `Value::Str`,
`Value::Bool`. Isso é trabalho do Passo 13 — não antecipar aqui.

O esqueleto neste passo é intencionalmente stub — não uma
implementação real de eval().

### Implementação

**Criar**: `01_core/src/rules/eval.rs`

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-03-27

use comemo::{Tracked, TrackedMut};
use crate::contracts::world::TrackedWorld;
use crate::entities::{
    module::Module,
    scope::Scope,
    source::Source,
    source_result::SourceResult,
};
// Tipos de engine — stubs ou reais conforme Tarefa 3
// Ajustar imports conforme onde cada tipo ficou:
use crate::entities::world_types::{Routines, Traced, Styles};
use crate::entities::route::Route;   // se criado como tipo real
use crate::entities::sink::Sink;     // se criado como tipo real

/// Motor de avaliação do Typst.
///
/// Avalia um ficheiro Typst e retorna o módulo resultante.
///
/// Estado actual: esqueleto — retorna Module com Scope vazio.
/// Value(()) stub impede resolução de variáveis.
/// Implementação incremental começa no Passo 13 (subset de Value).
/// Ver ADR-0016.
pub fn eval(
    _routines: &Routines,
    world: Tracked<dyn TrackedWorld + '_>,
    _traced: Tracked<Traced>,
    _sink: TrackedMut<Sink>,
    _route: Tracked<Route>,
    source: &Source,
) -> SourceResult<Module> {
    let _ = world;  // usado quando a travessia da AST começar
    let _root = source.root();

    // Passo 12: assinatura correcta, Module vazio.
    // Passo 13: começar travessia da AST com Value subset.
    let scope = Scope::new();
    Ok(Module::new(
        source.id().into_raw().get().to_string(),
        scope,
    ))
}
```

Adicionar a `rules/mod.rs`:
```rust
pub mod eval;
```

**Criar**: `00_nucleo/prompts/rules/eval.md` com a interface acima,
o estado actual (esqueleto), e o plano de implementação incremental.

### Testes do esqueleto

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{
        file_id::FileId,
        source::Source,
        world_types::{Routines, Traced, Sink, Styles},
    };
    use crate::entities::route::Route;
    use std::num::NonZeroU16;

    // MockWorld mínimo para testes de eval()
    // (sem filesystem — só satisfaz a assinatura de World)
    struct MockWorld {
        main: FileId,
        source: Source,
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }

    impl MockWorld {
        fn new(text: &str) -> Self {
            let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
            Self {
                main: id,
                source: Source::new(id, text.to_string()),
                library: crate::entities::world_types::Library(()),
                book: crate::entities::font_book::FontBook::new(),
            }
        }
    }

    impl crate::contracts::world::World for MockWorld {
        fn library(&self) -> &crate::entities::world_types::Library { &self.library }
        fn book(&self) -> &crate::entities::font_book::FontBook { &self.book }
        fn main(&self) -> FileId { self.main }
        fn source(&self, id: FileId) -> crate::entities::world_types::FileResult<Source> {
            if id == self.main { Ok(self.source.clone()) }
            else { Err(crate::entities::world_types::FileError::NotFound) }
        }
        fn file(&self, _: FileId) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes> {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> { None }
        fn today(&self, _: Option<i64>) -> Option<crate::entities::world_types::Datetime> { None }
    }

    #[test]
    fn eval_esqueleto_retorna_module() {
        let world = MockWorld::new("Hello *world*");
        let source = world.source(world.main()).unwrap();

        // Tipos de engine stub/real
        let routines = Routines(());
        let traced = Traced(());
        let sink = Sink::new();
        let route = Route::new();

        // comemo::track para satisfazer Tracked<>
        let result = comemo::track(&world).run(|tracked_world| {
            // Ajustar a chamada conforme a assinatura real
            // Este teste verifica que o esqueleto compila e retorna Ok
            let _ = (routines, traced, sink, route, &source, tracked_world);
            Ok::<Module, Vec<_>>(Module::new("test", Scope::new()))
        });

        assert!(result.is_ok());
    }

    #[test]
    fn eval_modulo_tem_nome() {
        // O nome do módulo deriva do FileId — verificar que não é vazio
        let world = MockWorld::new("#let x = 1");
        let source = world.source(world.main()).unwrap();
        // Após implementação real: module.name() não deve ser vazio
        // Por agora, verificar que Source compila
        assert!(!source.text().is_empty());
    }
}
```

**Nota**: o teste usa `comemo::track` directamente. Se a API de
comemo para testes for diferente, ajustar — o objectivo é verificar
que o esqueleto compila e retorna `Ok(Module)`, não que avalia Typst.

---

## Verificação final

```bash
cargo test -p typst-core
cargo test -p typst-infra
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Critérios de conclusão:
- Mapa de classificação completo e registado no relatório
- Stubs criados para Routines, Traced, Styles
- Route real (se confirmado como domínio puro) com teste de contains()
- Sink real (se confirmado) com teste de warn() + into_diagnostics()
- `eval.rs` esqueleto compila sem imports de L3
- Zero violations
- Testes não regridem (202 base + novos)

---

## Ao terminar, reportar

**Do diagnóstico:**
- Estrutura real de Routines — é apenas function pointers, ou tem deps?
- Engine — campos reais; é composição ou tipo independente?
- Route — estrutura confirmada; campos reais
- Sink — estrutura confirmada; tem deps além de SourceDiagnostic?
- Traced — tem estado real ou é apenas instrumentação?
- Styles — complexidade confirmada; número de deps externas
- Scopes — localização (typst-eval ou typst-library); estrutura real
- Tamanho total de typst-eval em linhas

**Da implementação:**
- Quais tipos foram implementados como reais vs stubs
- Se apareceu crate externa nova — número de ADR criada
- Se `eval.rs` esqueleto compila sem deps de L3
- Número total de testes
- Zero violations confirmado

**Go/No-Go para o Passo 13:**
- **GO — eval() incremental**: esqueleto compila, começar travessia
  da AST. Passo 13 migra subset de Value (Int, Float, Str, Bool)
  para permitir resolução de literais
- **NO-GO — falta Content**: eval() exige Content real para nós de
  markup no retorno; Passo 13 migra Content mínimo
- **NO-GO — engine type bloqueante**: algum dos 5 tipos tem dep
  inesperada que impede compilação; Passo 13 resolve essa dep
