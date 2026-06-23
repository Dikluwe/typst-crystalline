# Relatório P421 — `repr()` Completo (S → M)

**Data**: 2026-06-23  
**Passo**: P421 — `repr()` completo para `Value`, `Content`, `Selector`  
**Executor**: Kimi Code CLI

---

## 1. Sonda do substrato (Fase A.0)

Executados os 6 grep. O resultado crítico:

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `native_repr`/`fn repr` existe | ❌ 0 hits |
| 2 | `Value::` em `foundations.rs` | 2 hits (doc/comments) |
| 3 | `Content::` em `foundations.rs` | vários hits (state/counter) |
| 4 | `enum Value` | ✅ 22 variants |
| 5 | `enum Content` | ✅ ~70 variants |
| 6 | `Selector` em `foundations.rs` | 0 hits de repr |

**Reclassificação**: o critério do plano determinava que, se (1) falhasse, o passo deveria ser reclassificado de S para M. A infraestrutura `repr()` teve de ser criada do zero (`native_repr` + free functions + registo na stdlib).

**Variants faltantes**: todos os variants de `Value`, `Content` e `Selector` careciam de representação explícita.

---

## 2. Decisões arquiteturais (Fase A.1)

- **Paridade linguagem (ADR-0107)**: `repr(v)` produz string reconhecível; não garante round-trip. Aspas, escaping e ordem de fields são mecânica livre.
- **Atomização forma B (ADR-0109)**: lógica em `rules/eval/repr.rs` como free functions (`repr_value`, `repr_content`, `repr_selector`). Nenhum método adicionado a `Value`, `Content` ou `Selector`.
- **Infraestrutura nova**: `native_repr` em `rules/stdlib/foundations.rs` registada na stdlib como `repr`.
- **Scope-out honesto**: `Func`, `Module`, `Dyn`, `Location`, tipos internos de layout e campos default usam representação mínima.

### L0 atualizado

- `00_nucleo/prompts/rules/stdlib/foundations.md` — seção P421 com classificação M, decisões arquiteturais e scope-out.
- Hashes `@prompt-hash` sincronizados via `crystalline-lint`.

---

## 3. Implementação (Fase B)

### Eval
- `01_core/src/rules/eval/mod.rs`: `pub(crate) mod repr;`.
- `01_core/src/rules/eval/repr.rs` — **novo módulo**:
  - `repr_value(v: &Value) -> String` — match exaustivo sobre 22 variants de `Value`.
  - `repr_content(c: &Content) -> String` — match exaustivo sobre todos os variants de `Content` (incluindo struct variants `SetPage`, `Document`, `Asset`, `SmallCaps`).
  - `repr_selector(sel: &Selector) -> String` — match sobre `Kind`, `Label`, `Location`, `And`, `Or`, `Regex`, `Where`.
  - Helpers: `repr_float`, `repr_styles`.

### Stdlib
- `01_core/src/rules/stdlib/foundations.rs`: `native_repr` expõe `repr(v)`.
- `01_core/src/rules/stdlib/mod.rs`: re-export de `native_repr`.
- `01_core/src/rules/eval/mod.rs` (`make_stdlib`): registo `scope.define("repr", Value::Func(Func::native("repr", native_repr)))`.

### Tests (16)

| Módulo | Quantidade | Cobertura |
|--------|-----------|-----------|
| `rules/eval/repr.rs` | 6 unit | primitives, array, dict, selector, text/sequence, heading, cite |
| `rules/eval/tests.rs` | 6 E2E | `repr(1)`, `repr(1.0)`, `repr("hello")`, `repr([hello world])`, `repr(cite("key"))`, `repr(bibliography("refs.bib"))` |
| preexistentes | 4 | `value_duration_repr_*`, `value_version_repr_*` (continuam verdes) |

---

## 4. Validação (Fase C)

```bash
cargo check --workspace
# → ok

cargo test -p typst-core --lib -- repr
# → 16 passed; 0 failed; 0 ignored

cargo test -p typst-core --lib -- --skip p350c_flag_on_nao_convergente_classifica
# → 3130 passed; 0 failed; 0 ignored; 1 filtered out (stack overflow preexistente)

crystalline-lint .
# → 0 errors
# → 0 drift nos prompts tocados
# → warnings preexistentes de prompts órfãos (fora do escopo P421)
```

### Critérios de fecho
- [x] 16 tests verdes (≥10 exigidos)
- [x] Lint zero errors; drift sincronizado
- [x] `repr(1)` → `"1"`
- [x] `repr(1.0)` → `"1.0"`
- [x] `repr("hello")` → `""hello""`
- [x] `repr([hello world])` → reconhecível
- [x] `repr(cite("key"))` → `"cite(<key>)"`
- [x] `repr(bibliography("refs.bib"))` → `"bibliography(\"refs.bib\")"`
- [x] `repr_selector` cobre todos os variants
- [x] Nenhum vtable/`dyn` introduzido
- [x] `match` exaustivo sobre `Value`, `Content` e `Selector`
- [x] Lógica atomizada em free functions (forma B)
- [x] L0 hashado e propagado

---

## 5. Scope-out explícito

- Round-trip perfeito (`eval(repr(x)) == x`).
- Representação completa de closures (`Func`), exports de módulo (`Module`) e valores dinâmicos opacos (`Dyn`).
- Tipos internos de layout (`FrameItem`, `Region`, etc.).
- Campos com valores default podem ser omitidos.
- `repr()` de elementos cujo construtor stdlib não é ainda invocável diretamente via eval (ex.: `heading()` como função direta).

---

## 6. Notas epistêmicas

- **Medir antes de decidir (ADR-0108)**: A sonda A.0 revelou que `native_repr` não existia, forçando a reclassificação S→M. A decisão foi tomada antes de escrever código.
- **Paridade linguagem (ADR-0107)**: `repr()` satisfeito como função de debugging. A saída é reconhecível, não byte-exata.
- **Atomização (ADR-0109)**: `repr_value`/`repr_content`/`repr_selector` são free functions; structs de dados permanecem sem métodos de representação.
- **Honestidade epistêmica**: Variants complexos usam representação scope-out (`"function"`, `"module"`, `"location"`, etc.) em vez de inventar saídas não suportadas.
- **Próximo passo P422**: `link` render visual (S) ou `text.lang` rustybuzz (XL, scope-out) ou outro gap do Inventário.
