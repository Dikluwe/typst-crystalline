# P417 — `Selector::Where`: filtragem de show rules por campos de elemento (M)

**Título**: Selector::Where — filtragem de show rules e query por campos de elemento  
**Tipo**: Materialização (M) — consumer ShowRule + infraestrutura de matching  
**Bloqueadores**: Nenhum externo; pré-condições internas verificáveis  
**Referências**: ADR-0107 (paridade linguagem), ADR-0108 (medir antes de decidir), ADR-0109 (atomização forma B), P412 (field access), P248 (show rules), P304 (footnote real)

---

## FASE A.0 — Sonda do substrato (obrigatória; 5 min)

Execute os 8 grep abaixo **antes de qualquer redação ou código**.  
**Critério de passagem**: todos os 8 produzem output conforme esperado. Se qualquer um falhar → **parar imediatamente** e reclassificar o passo.

```bash
# 1. Selector enum existe no projeto?
grep -rn "enum Selector" 01_core/src/

# 2. Selector tem variant Elem (seleção por tipo de elemento)?
grep -n "Elem" 01_core/src/entities/selector.rs 2>/dev/null || grep -n "Selector" 01_core/src/entities/content.rs | head -20

# 3. ShowRule struct existe e tem selector + replacement?
grep -rn "struct ShowRule" 01_core/src/

# 4. Show rule matching já existe no pipeline (eval ou layout)?
grep -rn "show_rule\|show_rules\|match_show" 01_core/src/rules/ | head -20

# 5. Field access (P412) existe — necessário para .where(level: 1)?
grep -rn "FieldAccess\|field_access" 01_core/src/entities/ 01_core/src/rules/eval/ 2>/dev/null | head -10

# 6. Element fields são acessíveis por nome (para matching)?
grep -rn "fields\|field_names\|get_field" 01_core/src/entities/elements/ | head -20

# 7. set/show commands existem no parser/AST?
grep -rn "Set\|Show" 01_core/src/entities/ast.rs 2>/dev/null | head -20

# 8. Query infrastructure existe (para Selector usado em #query)?
grep -rn "query\|Query" 01_core/src/rules/ | head -20
```

**Output esperado**:
1. ≥1 hit com `enum Selector` ou `Selector` definido como tipo
2. ≥1 hit com `Elem` ou similar como variant de seleção por tipo
3. ≥1 hit com `struct ShowRule` contendo `selector` e `replacement`
4. ≥1 hit com matching de show rules em eval ou layout
5. ≥1 hit com field access infrastructure (P412)
6. ≥1 hit com acesso a fields de elementos (por nome ou índice)
7. ≥1 hit com `Set` ou `Show` no AST
8. ≥0 hits (query é nice-to-have, não bloqueador; se não existir, scope-out para P417.X)

**Se (1) falhar** → Selector não existe; reclassificar para L (criar enum Selector do zero).  
**Se (3) falhar** → ShowRule não existe; reclassificar para L (infraestrutura de show rules ausente).  
**Se (4) falhar** → matching de show rules não existe; reclassificar para L.  
**Se (5) falhar** → field access (P412) não foi materializado; reclassificar para M+ (reabrir P412).  
**Se (6) falhar** → elementos não expõem fields para introspection; reclassificar para L (reabrir infraestrutura de fields).

---

## FASE A.1 — L0 (hash obrigatório)

**Documentar no L0** (`00_nucleo/prompts/entities/selector.md` + `rules/show.md`):

### A.1.1 — Decisão arquitetural: paridade linguagem (ADR-0107)

No Typst vanilla, `heading.where(level: 1)` é uma **construção linguística** de filtragem de selector. A paridade é:

- **Semântica**: `heading.where(level: 1)` seleciona apenas elementos `heading` cujo campo `level` vale `1`. O matching é por **valor do campo**, não por tipo de dado interno.
- **Sintaxe**: `<elem>.where(<field>: <value>)` — method call syntax com argumentos nomeados.
- **Morfologia**: o selector resultante é um objeto da linguagem que pode ser usado em `#show` e `#query`.

**O que NÃO é paridade (mecânica; diverge de propósito — ADR-0107 / P329):**
- A estrutura interna do `Selector` no vanilla (vtable, reflection, etc.). O crystalline usa enum (ADR-0026/0105-D); a paridade é comportamental, não estrutural.
- A ordem de resolução de show rules no vanilla (multi-passe, fixpoint). O crystalline resolve em ordem de definição; a saída observável é o contrato.
- O `==` mecânico do Rust entre selectors. A igualdade de selectors é semântica (dois `Where` com mesmo elem/field/value são iguais), não estrutural por `PartialEq` derivado.

### A.1.2 — Estratégia de implementação (atomização forma B, ADR-0109)

| Opção | Descrição | Magnitude | Risco |
|-------|-----------|-----------|-------|
| **α** — Where como variant com closure dinâmico | `Selector::Where(Box<dyn Fn(&Content) -> bool>)` | M | **Alto** — vtable, perda de exaustividade, viola ADR-0026/0109 |
| **β** — Where como variant com (elem, field, value) estático | `Selector::Where { elem: ElemName, field: FieldName, value: Value }` | **M** | **Baixo** — enum estático, match exaustivo, jump table, paridade ADR-0109 |

**Decisão recomendada**: **Opção β** — estrutura de dados estática, sem vtable, sem `dyn`. O matching é feito por `match` exaustivo no consumer (show rule matcher), com delegação a free function na camada de render (forma B, ADR-0109). A lógica de "does this content match this Where selector?" vive em `rules/show/where_match.rs` (ou similar), não no arquivo do struct `Selector`.

### A.1.3 — Estrutura de dados

```rust
// Em entities/selector.rs (ou equivalente) — ADR-0109: enum fechado, sem vtable
#[derive(Clone, Debug, PartialEq)]  // PartialEq semântico, não mecânico (ADR-0107)
pub enum Selector {
    Elem(ElemName),           // já existe
    Label(Label),             // já existe (scope-out se não)
    Regex(Regex),             // já existe (scope-out se não)
    // --- NOVO ---
    Where {
        base: Box<Selector>,  // o selector base (tipicamente Elem)
        field: FieldName,     // nome do campo (String ou interned string)
        value: Value,         // valor esperado (Value::Int, Value::Str, etc.)
    },
    // --- existentes ---
    Or(Box<Selector>, Box<Selector>),
    And(Box<Selector>, Box<Selector>),
    // ... outros
}
```

**Nota ADR-0107**: `PartialEq` aqui é **semântico** — dois `Where` são iguais se `base`, `field` e `value` são iguais. O `Value::PartialEq` já deve ser semântico (ADR-0025 estendido; se não for, conserto é pré-requisito ou scope-out).

### A.1.4 — Algoritmo de matching

1. **Parsing**: `heading.where(level: 1)` parseia como method call `where` em `Selector` com named args. O AST já deve suportar method calls (verificar em A.0).
2. **Eval**: O eval do method call `where` constrói `Selector::Where { base: Box::new(Selector::Elem(Heading)), field: "level", value: Value::Int(1) }`.
3. **Matching em show rules** (consumer):
   - O matcher de show rules itera sobre as regras definidas.
   - Para cada `ShowRule { selector, replacement }`:
     - Se `selector` é `Selector::Elem(e)` → match se `content` é daquele elem (já existe).
     - Se `selector` é `Selector::Where { base, field, value }` → match se:
       1. `content` matcha `base` (recursivo)
       2. `content` tem o campo `field`
       3. o valor do campo `field` em `content` é **semanticamente igual** a `value` (ADR-0107: não usar `PartialEq` mecânico do Rust; usar igualdade de `Value` do Typst)
4. **Atomização (ADR-0109)**: a lógica de matching de `Where` vive em `rules/show/where_match.rs` como free function:
   ```rust
   // rules/show/where_match.rs — forma B, ADR-0109
   pub(super) fn matches_where(
       content: &Content,
       base: &Selector,
       field: &FieldName,
       expected: &Value,
   ) -> bool {
       // 1. match base recursivamente
       // 2. extrair field do content
       // 3. comparar com expected via Value::eq_semantic (ou Value::eq se já for semântico)
   }
   ```
   O `match` no consumer fica magro:
   ```rust
   // rules/show/matcher.rs (ou mod.rs)
   match selector {
       Selector::Elem(e) => elem_matches(content, e),
       Selector::Where { base, field, value } => {
           where_match::matches_where(content, base, field, value)
       }
       // ... outros arms delegam
   }
   ```

### A.1.5 — Paridade vanilla

- Vanilla: `heading.where(level: 1)` funciona em `#show` e `#query`; campos aninhados não suportados diretamente (`.where(level: 1)` apenas, não `.where(nested.field: 1)`); múltiplos campos via `.where(a: 1, b: 2)`.
- Cristalino P417: paridade estrutural (single field, single level); múltiplos campos scope-out (podem ser encadeados: `heading.where(level: 1).where(numbering: "1.")` se `And` já existir, ou adiado).

### A.1.6 — Scope-out explícito

- Múltiplos campos em um único `.where(...)` (ex.: `heading.where(level: 1, numbering: "1.")`) — scope-out; se `And` já existe, encadeamento é workaround aceitável
- Campos aninhados (`.where(parent.child: 1)`) — scope-out
- `Selector::Where` em `#query` — scope-out se query não existir (ver A.0 item 8)
- Regex matching em fields de string — scope-out (se `Selector::Regex` já existe, pode ser combinado)
- Type coercion no matching de `value` (ex.: `level: 1.0` matchando `level: 1`) — scope-out; usa igualdade estrita de `Value` (ADR-0025 para Int/Float já cobre `1 == 1.0`)

---

## CHECKPOINT A

**Só prosseguir para Fase B quando confirmar que:**
1. Guardou e computou hash do L0 (A.1) em `selector.md` + `show.md`
2. Sonda A.0 produziu 8/8 OK (ou 7/8 com item 8 = 0 hits aceitável se query não existir)
3. Decisão β validada (enum estático, sem vtable, forma B)
4. `FieldName` / acesso a fields de elementos é viável (item 6 da sonda)

**Hash L0 esperado**: `<computar após redação>`

---

## FASE B — Código

### B.1 — Entities (`entities/selector.rs` ou equivalente)

1. **Adicionar variant `Where`** ao enum `Selector`:
   ```rust
   Where {
       base: Box<Selector>,
       field: EcoString,  // ou FieldName interno
       value: Value,
   },
   ```
2. **Nenhuma alteração em `Content`** — ADR-0109: enum fechado, sem vtable.
3. **Implementar `Display` / `Repr` para `Selector::Where`** se `repr()` existe — scope-out se `repr()` ainda não cobre Selector.

### B.2 — Parser / AST

1. **Method call `where` em `Selector`**:
   - Se o parser já suporta method calls genéricos (`obj.method(args)`): adicionar `where` como método reservado de `Selector`.
   - Se não: adicionar sintaxe específica ou scope-out para parser simplificado (ex.: `where(heading, level: 1)` como função global temporária — **não recomendado**; melhor reabrir parser de method calls).
   - **Verificar em A.0**: se method calls não existem, reclassificar para L (parser).

### B.3 — Eval (`rules/eval/` ou equivalente)

1. **Eval de `Selector::Where`**:
   - O eval do method call `where` recebe `base: Selector`, `field: String`, `value: Value`.
   - Constrói `Selector::Where { base: Box::new(base), field, value }`.
   - Validação: `base` deve ser `Selector::Elem` (ou outro selector que produza elementos com fields; scope-out se base for `Or`/`And` — pode ser adiado).

### B.4 — Consumer: Show Rule Matcher (`rules/show/` — forma B, ADR-0109)

1. **Criar `rules/show/where_match.rs`** (ou `selector_match.rs`):
   ```rust
   pub(super) fn matches(
       content: &Content,
       selector: &Selector,
   ) -> bool {
       match selector {
           Selector::Elem(e) => elem_matches(content, e),
           Selector::Where { base, field, value } => {
               matches_where(content, base, field, value)
           }
           // delegar outros para arquivos existentes
       }
   }

   fn matches_where(
       content: &Content,
       base: &Selector,
       field: &EcoString,
       expected: &Value,
   ) -> bool {
       // 1. match base primeiro
       if !matches(content, base) {
           return false;
       }
       // 2. extrair field do content
       let actual = extract_field(content, field);
       // 3. comparar semanticamente
       actual.map_or(false, |v| v == *expected)  // Value::eq deve ser semântico (ADR-0025)
   }
   ```
2. **Integrar no matcher existente**:
   - O `match` no consumer fica magro (1 linha por variant), delegando para free functions.
   - Se o matcher é monolítico (inline em `mod.rs`), **atomizar** (ADR-0109): extrair para `rules/show/matcher.rs` com `match` magro, mover lógica de cada variant para seu arquivo.

### B.5 — Field extraction (`rules/show/field_extract.rs` — forma B)

```rust
pub(super) fn extract_field(content: &Content, field: &EcoString) -> Option<Value> {
    match content {
        Content::Heading(h) => match field.as_str() {
            "level" => Some(Value::Int(h.level as i64)),
            "numbering" => h.numbering.clone().map(Value::Str), // ou Value::Content
            // ... outros fields de Heading
            _ => None,
        },
        Content::Figure(f) => match field.as_str() {
            "kind" => Some(Value::Str(f.kind.clone())),
            // ...
            _ => None,
        },
        // ... outros arms, 1 linha cada, delegando para <elem>::extract_field
        _ => None,
    }
}
```

**Nota ADR-0109**: se `extract_field` ficar grande, atomizar por elemento: `rules/show/fields/heading.rs`, `fields/figure.rs`, etc. — cada um com `pub(super) fn extract_field_heading(...)`. O `match` em `field_extract.rs` fica magro (1 linha por elemento).

### B.6 — Tests

**Mínimo 16 tests**:
- 4 unit `Selector::Where` struct: construção, clone, debug, partial_eq semântico
- 4 unit `matches_where`: match positivo, match negativo (field errado), match negativo (value errado), match negativo (base não matcha)
- 4 unit `extract_field`: heading.level, heading.numbering, figure.kind, field inexistente
- 4 E2E show rules: `#show heading.where(level: 1): set text(red)` aplicado, não aplicado a heading level 2, não aplicado a paragraph, aplicado a query (se query existir)

---

## FASE C — Validação

```bash
crystalline-lint .
# → 0 drift
# → 0 violations
# → tests: +16 verdes
# → workspace total: <baseline> + 16 verdes
```

**Critério de fecho**:
- [ ] 16 tests verdes
- [ ] Lint zero
- [ ] `heading.where(level: 1)` funciona em `#show`
- [ ] Matching semântico (ADR-0107): `1 == 1.0` matcha se ADR-0025 já estiver ativo
- [ ] Nenhum vtable / `dyn` / `Box<dyn>` introduzido (ADR-0109 / ADR-0026)
- [ ] `match` exaustivo preservado (ADR-0105 cl.3)
- [ ] Lógica atomizada em free functions na camada de render (ADR-0109 forma B)
- [ ] L0 hashado e propagado

---

## Notas epistêmicas

- **Medir antes de decidir (ADR-0108)**: A.0 verifica 8 pré-condições antes de qualquer decisão de código. Se parser de method calls não existe, o passo para antes de gastar LOC.
- **Paridade linguagem (ADR-0107)**: O `Where` é medido pelo que a linguagem faz (selecionar por campo), não pela mecânica do vanilla (reflection, vtable, ordem de passes).
- **Atomização (ADR-0109)**: A lógica de matching vive na camada de render (`rules/show/`), não no arquivo do struct `Selector` (`entities/`). A forma é B (free function), não A (método no struct). O `match` no consumer é magro (1 linha por variant); a lógica gorda muda para arquivos da feature.
- **Sem vtable (ADR-0026/0105-D)**: `Selector::Where` contém dados estáticos (`elem`, `field`, `value`), não closure dinâmica. O despacho é por `match` estático com jump table O(1).
- **Próximo passo P418**: `bibliography/cite` CSL (XL) ou `repr()` completo (S) ou `Selector::And`/`Or` (M) se ainda não existirem.
