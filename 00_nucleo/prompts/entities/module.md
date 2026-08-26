# Prompt L0 — entities/module
Hash do Código: 3afc4142

**Passo**: P536
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
    pub fn document_info(&self) -> &DocumentInfo
    pub fn set_document_info(&mut self, info: DocumentInfo)
}

impl Clone for Module  // O(1) — Arc::clone
```

## Campos do `ModuleInner`

```rust
struct ModuleInner {
    name: String,
    scope: Scope,
    content: Option<Content>,
    introspection_content: Option<Content>,
    bib_styles: HashMap<u64, Arc<IndependentStyle>>,
    document_info: DocumentInfo,
}
```

- `content` — output renderizado pós-show-rules (usado por layout/PDF).
- `introspection_content` — conteúdo original pré-show-rules (P498), usado
  para construir o `TagIntrospector`. Garante que `query(heading)` encontra
  o elemento mesmo quando uma show-rule o transforma no output renderizado.
- `bib_styles` — styles CSL resolvidos em eval time (P429 / DEBT-63).
- `document_info` — metadados do documento definidos por `#set document(...)`
  (P536). Transporte eval → pipeline → exportador PDF (`/Info`).

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
