# Prompt L0 — entities/module
Hash do Código: dcdb6481

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/module.rs`
**ADRs relevantes**: ADR-0017 (adiamento eval/typst-library), ADR-0023 (indexmap/Scope)

## Contexto

`Module` é o resultado de avaliar um ficheiro Typst. Contém um `Scope`
de bindings e um nome. O original usa `Arc<ModuleInner>` — replicado
aqui porque módulos são clonados entre ramos de `eval()` e `Arc` torna
o clone O(1) em vez de O(n).

Campo `content: Content` do original é omitido neste passo — `Content`
não está migrado (ADR-0017). Documentado com comentário explícito.

`EcoString` substituído por `String` (ADR-0004 Opção C / padrão do projecto).
`typst_syntax::FileId` substituído por `crate::entities::file_id::FileId`.

## Interface pública

```rust
pub struct Module(Arc<ModuleInner>);

impl Module {
    pub fn new(name: impl Into<String>, scope: Scope) -> Self
    pub fn name(&self) -> &str
    pub fn scope(&self) -> &Scope
}

impl Clone for Module  // O(1) — Arc::clone
```

## Campo content omitido

```rust
// content: Content,  // ADR-0017: adiado — Content não migrado
```

Documentado em DEBT.md quando necessário. `eval()` parcial sem `Content`
ainda permite testar a estrutura de scoping.

## Critérios de Verificação

```
Dado Module::new("my-file", scope_com_x)
Quando name() e scope().get("x") forem chamados
Então "my-file" e Some(&Value(()))

Dado Module criado e clonado
Quando ambos são usados
Então clone é consistente (mesmo nome)

Dado Module::new("empty", Scope::new())
Então scope().is_empty() = true
```
