# Diagnóstico: mecanismo `Route` do vanilla

**Tipo vanilla**: `Route<'a>`
**Localização vanilla**: `lab/typst-original/crates/typst-library/src/engine.rs`
**Data do diagnóstico**: 2026-04-22
**Contexto**: Passo 85 (preparação para futura resolução do DEBT-40)

**Natureza**: registo factual do estado do vanilla na data acima.
Decisões arquitecturais derivadas deste diagnóstico ficam em
ADR/passo separados. Este ficheiro não contém decisões.

---

## 1. Localização

- **Definição**: `lab/typst-original/crates/typst-library/src/engine.rs:251`
  (`pub struct Route<'a>`).
- **Uso central**: campo `route: Route<'a>` no struct `Engine<'a>`
  (mesmo ficheiro, linha 33).
- **Ponto de entrada raiz**: `lab/typst-original/crates/typst/src/lib.rs:128`
  (`Route::default().track()` antes de iniciar a compilação).
- **Verificação de ciclos**: `lab/typst-original/crates/typst-eval/src/import.rs:232`
  (`engine.route.contains(source.id())` em `import_file`); idêntico em
  `lab/typst-original/crates/typst-eval/src/lib.rs:50` (`fn eval`).
- **Pontos de extensão (`Route::extend`)**: 12 ocorrências em
  `typst-bundle`, `typst-eval/{call,lib}`, `typst-html/{document,fragment}`,
  `typst-layout/{flow/{collect,mod}, inline/mod, pages/{mod,run}}`,
  `typst-library/introspection/{counter,state}`.

---

## 2. Definição estrutural

Snippet (engine.rs:249–274):

```rust
/// The route the engine took during compilation. This is used to detect
/// cyclic imports and excessive nesting.
pub struct Route<'a> {
    /// The parent route segment, if present.
    outer: Option<Tracked<'a, Self, <Route<'static> as Track>::Call>>,
    /// This is set if this route segment was inserted through the start of a
    /// module evaluation.
    id: Option<FileId>,
    /// This is set whenever we enter a function, nested layout, or are applying
    /// a show rule. ...
    len: usize,
    /// The upper bound we've established for the parent chain length.
    upper: AtomicUsize,
}
```

Derives explícitos: nenhum `#[derive]`. `Default` e `Clone` implementados
manualmente (linhas 424 e 430). `Clone::clone` recria `AtomicUsize::new`
a partir de leitura `Relaxed` — não é simples copy bitwise.

Atributo relevante: `#[comemo::track]` no segundo bloco `impl` (linha
389), o que torna o trait `Track` activo para os métodos lá dentro
(`contains`, `within`).

---

## 3. Operações

Construção:
- `pub fn root() -> Self` (linha 278) — segmento raiz, `id: None`,
  `outer: None`, `len: 0`.
- `pub fn extend(outer: Tracked<'a, Self>) -> Self` (288) — novo
  segmento ligado ao pai, `len: 1`.
- `pub fn with_id(self, id: FileId) -> Self` (298) — anota o FileId
  do módulo a entrar.
- `pub fn unnested(self) -> Self` (303) — força `len: 0` (segmento
  que não conta para profundidade).
- `pub fn track(&self) -> Tracked<'_, Self>` (311) — atalha o trait
  `Track::track` quando o segmento corrente nada acrescenta.

Mutação local:
- `pub fn increase(&mut self)` (319), `pub fn decrease(&mut self)` (324)
  — incremento/decremento de `len` para `Engine` interno.

Verificação (bloco `#[comemo::track]`):
- `pub fn contains(&self, id: FileId) -> bool` (393) — true se o
  `id` está em qualquer segmento da cadeia.
- `pub fn within(&self, depth: usize) -> bool` (398) — true se a
  profundidade total não excede `depth`; usa `upper: AtomicUsize`
  como cache reduzível por `compare_exchange` (Relaxed).

Limites:
- `MAX_SHOW_RULE_DEPTH = 64`, `MAX_LAYOUT_DEPTH = 72`,
  `MAX_HTML_DEPTH = 72`, `MAX_CALL_DEPTH = 80` (linhas 335–344).
- Métodos `check_*_depth` retornam `bail!` com hint específico em
  cada categoria.

---

## 4. Mecanismo de recursão

Cada chamada que entra num novo escopo (import, function call, show
rule, layout, etc.) cria um **novo `Engine` no stack** com um novo
`Route::extend(parent)`. Snippet típico (typst-eval/src/import.rs:227–245):

```rust
fn import_file(engine: &mut Engine, id: FileId, span: Span) -> SourceResult<Module> {
    let source = engine.world.source(id).at(span)?;

    // Prevent cyclic importing.
    if engine.route.contains(source.id()) {
        bail!(span, "cyclic import");
    }

    eval(
        engine.routines,
        engine.world,
        engine.traced,
        TrackedMut::reborrow_mut(&mut engine.sink),
        engine.route.track(),     // ←─ pai passado aqui
        &source,
    )
}
```

E em `typst-eval/src/lib.rs:62`:

```rust
let engine = Engine {
    routines, world,
    introspector: Protected::new(introspector.track()),
    traced, sink,
    route: Route::extend(route).with_id(id),  // ←─ novo segmento filho
};
```

Resposta às perguntas explícitas do enunciado:

- **O frame é passado por valor ou referência?** Por referência —
  `engine.route.track()` devolve `Tracked<'_, Route>` (covariante na
  lifetime, ver comentário em engine.rs:255–257).
- **A ligação ao pai é qual tipo?** `Option<Tracked<'a, Self, <Route<'static> as Track>::Call>>`
  — opcional `Tracked` do pai, com `Call` atravessada via `Route<'static>`
  para preservar covariância. Não é `&Route`, não é `Arc<Route>`. É um
  `Tracked` do `comemo`, que é (semanticamente) uma referência mediada
  por proxy de tracking.

---

## 5. Mecanismo de detecção de ciclo

`Route::contains` (engine.rs:393–395):

```rust
pub fn contains(&self, id: FileId) -> bool {
    self.id == Some(id) || self.outer.is_some_and(|outer| outer.contains(id))
}
```

Recursão linear sobre a cadeia `outer` — equivalente a percorrer uma
linked list. Complexidade amortizada por verificação: O(profundidade da
cadeia de imports activa). Em prática essa profundidade é tipicamente
muito pequena (uma dúzia, no máximo) e está limitada superiormente pelos
limites `MAX_*_DEPTH` (64–80). Sem `HashSet` — a comparação faz-se segmento
a segmento.

Cada chamada `outer.contains(id)` atravessa o proxy `Tracked` do `comemo`,
o que tem um custo adicional face a uma chamada directa de método (ver
secção 6).

---

## 6. Integração com `comemo`

Sim — `Route` é parte do mecanismo de tracking do `comemo`. Sinais:

1. O `impl<'a> Route<'a>` que contém `contains` e `within` está anotado
   com `#[comemo::track]` (engine.rs:389–390). Isto gera o trait `Track`
   para `Route` e expõe os métodos via proxy.
2. O campo `outer` é `Option<Tracked<'a, Self, ...>>` — o pai é guardado
   como handle `Tracked`, não como referência directa.
3. `Route::track(&self) -> Tracked<'_, Self>` (311) tem fast-path que
   devolve directamente o `outer` quando o segmento corrente nada
   acrescenta — optimização de cache do `comemo` (evita criar uma nova
   chave de tracking quando o estado é idêntico ao pai).
4. O `upper: AtomicUsize` existe **especificamente** para que diferentes
   profundidades não-excedentes possam reusar a mesma entrada de cache
   do `comemo` (comentário no campo, linhas 268–272: "We don't know the
   exact length (that would defeat the whole purpose because it would
   prevent cache reuse...").

Em suma: `Route` foi desenhado em torno do `comemo` desde o início —
não é uma estrutura "neutra" depois decorada com tracking.

---

## 7. Divergências actuais do cristalino

Comparação directa com `01_core/src/rules/eval.rs` (linhas 50–238):

| Aspecto | Vanilla `Route` | Cristalino |
|---------|----------------|-----------|
| Ligação pai-filho | Linked list via `Option<Tracked<...>>` | `Vec<FileId>` plano em `EvalContext.import_stack` |
| Propagação | Cada novo escopo cria `Engine` no stack com `Route::extend(parent)` | `EvalContext` único reutilizado; push/pop no `Vec` |
| Detecção de ciclo | `Route::contains(id)` — recursão O(profundidade) | `Vec::contains(&id)` — varredura linear O(N) |
| RAII | Não há guard — frame morre quando `Engine` sai de scope | `ImportGuard` com `unsafe { (*stack_ptr).retain(...) }` no `Drop` |
| Tracking | `#[comemo::track]` em métodos `contains`/`within` | Sem tracking — `Vec` simples |
| Cache de profundidade | `AtomicUsize upper` para reuso de cache `comemo` | Não aplicável (sem cache) |
| Limites de profundidade | 4 categorias (`SHOW_RULE`/`LAYOUT`/`HTML`/`CALL`) com hints distintos | 1 limite genérico (`max_call_depth = 250`, `EvalContext::check_call_depth`) |
| Identidade do escopo | `id: Option<FileId>` por segmento | Mesma chave (FileId) é o elemento do `Vec` |
| `unsafe` necessário | Zero (linked list segue lifetimes do borrow checker via `Tracked`) | Um bloco em `ImportGuard::drop` (DEBT-40) |
| Dependência | `comemo`, `rustc_hash` (importado mas não usado por `Route`) | Apenas `std` |

Observações factuais adicionais:

- O cristalino não tem ainda separação entre tipos de profundidade
  (call vs. show rule vs. layout). `EvalContext.max_call_depth` é único.
- O cristalino usa a mesma estrutura (`Vec<FileId>`) para detectar ciclos
  e (implicitamente) para registar profundidade — vanilla separa: ciclos
  são `id`+chain; profundidade é `len`+`upper`.
- O `ImportGuard` do cristalino existe porque `EvalContext` é mutado em
  vez de clonado por escopo. Vanilla evita o problema porque cada escopo
  recursivo aloca um novo `Engine` no stack e o `Route::extend` é
  imutável.

---

## Referências

- `lab/typst-original/crates/typst-library/src/engine.rs:251`
  — definição de `Route<'a>`.
- `lab/typst-original/crates/typst-eval/src/import.rs:227`
  — `import_file` com verificação `route.contains`.
- `lab/typst-original/crates/typst-eval/src/lib.rs:50`
  — `eval` (entrada de módulo) com mesma verificação.
- `lab/typst-original/crates/typst/src/lib.rs:128`
  — `Route::default().track()` na raiz da compilação.
- `01_core/src/rules/eval.rs:62,169,215,228`
  — `import_stack`, `enter_import`, `ImportGuard`, `Drop`.
- `00_nucleo/DEBT.md` — DEBT-40 (resolução fica para passo dedicado).
- `00_nucleo/adr/typst-adr-0032-*.md` — política de `unsafe` em L1.
- `00_nucleo/adr/typst-adr-0034-*.md` — esta convenção de diagnóstico.
