# P423 — `Selector::And`/`Or` no Consumer de Show Rules: combinadores de selectors (S-M)

**Título**: Selector combinators — `And`/`Or` funcionando no matcher de show rules e query  
**Tipo**: Materialização (S-M) — consumer show rules + eval  
**Bloqueadores**: Nenhum externo; `And`/`Or` já existem no enum `Selector` (P417)  
**Referências**: ADR-0107 (paridade linguagem), ADR-0108 (medir antes de decidir), ADR-0109 (atomização forma B), P417 (Selector::Where), P248 (show rules), P422 (link render)

---

## FASE A.0 — Sonda do substrato (obrigatória; 3 min)

Execute os 6 grep abaixo **antes de qualquer redação ou código**.  
**Critério de passagem**: 4/6 mínimo; itens 5 e 6 nice-to-have. Se qualquer um dos 4 obrigatórios falhar → **parar imediatamente** e reclassificar.

```bash
# 1. Selector::And existe no enum?
grep -rn "And" 01_core/src/entities/selector.rs | head -10

# 2. Selector::Or existe no enum?
grep -rn "Or" 01_core/src/entities/selector.rs | head -10

# 3. Show rule matcher consome And/Or?
grep -rn "And\|Or" 01_core/src/engine/eval/rules.rs | head -20

# 4. Query infrastructure consome And/Or?
grep -rn "And\|Or" 01_core/src/entities/introspector.rs | head -20

# 5. [NICE-TO-HAVE] Parser aceita sintaxe `heading | figure`?
grep -rn "\|" 01_core/src/engine/parse/ | head -10

# 6. [NICE-TO-HAVE] Parser aceita sintaxe `heading & figure`?
grep -rn "&" 01_core/src/engine/parse/ | head -10
```

**Output esperado**:
1. ≥1 hit com `And` como variant de `Selector`
2. ≥1 hit com `Or` como variant de `Selector`
3. ≥0 hits (pode ser que And/Or ainda não sejam consumidos no matcher — este é o gap)
4. ≥0 hits (pode ser que And/Or ainda não sejam consumidos em query)
5. ≥0 hits (parser de `|` pode não existir)
6. ≥0 hits (parser de `&` pode não existir)

**Se (1) falhar** → `And` não existe no enum; reclassificar para S (adicionar variant).  
**Se (2) falhar** → `Or` não existe no enum; reclassificar para S.  
**Se (3) mostrar que And/Or JÁ são consumidos** → reclassificar para "já implementado" ou S (documentação/verificação).  
**Se (3) e (4) mostrarem 0 hits** → And/Or existem no enum mas não no consumer; este é o gap do P423.

---

## FASE A.1 — L0 (hash obrigatório)

**Documentar no L0** (`00_nucleo/prompts/entities/selector.md` + `rules/show.md`):

### A.1.1 — Decisão arquitetural: paridade linguagem (ADR-0107)

No Typst vanilla, `heading | figure` e `heading & figure` são **construções linguísticas** de combinação de selectors:
- **Semântica**: `A | B` seleciona elementos que matcham A **ou** B (disjunção). `A & B` seleciona elementos que matcham A **e** B (conjunção). A avaliação é curto-circuito quando possível.
- **Sintaxe**: `selector1 | selector2` (pipe) para OR; `selector1 & selector2` (ampersand) para AND. Também suportado via methods: `selector1.or(selector2)`, `selector1.and(selector2)`.
- **Morfologia**: `Selector::Or(Box<Selector>, Box<Selector>)` e `Selector::And(Box<Selector>, Box<Selector>)` são objetos da linguagem.

**O que NÃO é paridade (mecânica; diverge de propósito):**
- A ordem de avaliação (curto-circuito esquerda-para-direita vs paralelo).
- A estrutura interna do `match` (vanilla pode usar set operations; crystalline usa recursão).
- A mensagem de erro quando `And` é contraditório (ex.: `heading & paragraph` nunca matcha).

### A.1.2 — Decisão arquitetural: atomização forma B (ADR-0109)

A lógica de matching de `And`/`Or` vive na **camada de eval/show**, não no struct:

- `entities/selector.rs` — `Selector::And`/`Or` variants (já existem, P417).
- `rules/show/where_match.rs` (ou `selector_match.rs`) — free function `matches()` estendida com arms `And` e `Or` (forma B).
- `rules/eval/closures.rs` — eval de method calls `.or()` e `.and()` (se parser não suportar `|`/`&` inline).

**Não usar Opção A** (`impl Selector { fn matches(...) }` em `entities/`) — lógica de matching é consumer, não dado (ADR-0109, rejeitado).

### A.1.3 — Estratégia de implementação

| Opção | Descrição | Magnitude | Risco | Nota |
|-------|-----------|-----------|-------|------|
| **α** — Parser `\|`/`&` + eval direto | Adicionar tokens `|` e `&` no parser; eval constrói `Selector::Or`/`And` | M | Médio | Paridade sintática completa; mas pode conflitar com outros usos de `\|` e `&` |
| **β** — Methods `.or()`/`.and()` apenas | `selector1.or(selector2)` como method call; eval constrói `Selector::Or`/`And` | **S** | **Baixo** | Paridade semântica sem mudar parser; sintaxe ligeiramente diferente |
| **γ** — Parser `\|`/`&` + methods | Ambos: operadores infixos e methods | M | Médio | Completo; mas duplica infraestrutura |

**Decisão recomendada**: **Opção β** — methods `.or()`/`.and()` apenas.  
**Razão ADR-0108**: medição do substrato confirma que o caminho infixo é M, não S:
- `SyntaxKind` não tem tokens `|`/`&` (`rg 'Pipe|Ampersand' 01_core/src/entities/syntax_kind.rs` → 0 hits).
- O lexer de code (`01_core/src/engine/lexer/code.rs:83-90`) trata `&` e `|` como caracteres inválidos, dando erro com hint para usar `and`/`or`.
- Adicionar `|`/`&` exigiria: (1) novos `SyntaxKind`; (2) braços no lexer; (3) inclusão no `BINARY_OP` set (`syntax_set.rs:139`); (4) novos variantes/ramos em `BinOp`/`from_kind` (`operators.rs:61`); (5) eval de `Binary` para construir `Selector::Or`/`And` quando operandos são `Value::Selector`. Potencial conflito com math mode (`|` e `&` já usados em math como `MathText`/`MathAlignPoint`), mas math usa lexer separado (`lexer/math.rs`).
- Os methods `.or()`/`.and()` reutilizam a infraestrutura de method calls existente (P417 já fez `.where()`). É S. A paridade semântica é idêntica; apenas a sintaxe diverge levemente (mecânica livre per ADR-0107).

**Se o parser já suportar `|`/`&`** (sonda A.0 itens 5-6): usar opção γ (ambos) sem custo adicional.

### A.1.4 — Algoritmo de matching

1. **Eval de `.or()` / `.and()`**:
   - `selector1.or(selector2)` → `Selector::Or(Box::new(selector1), Box::new(selector2))`.
   - `selector1.and(selector2)` → `Selector::And(Box::new(selector1), Box::new(selector2))`.
   - Validação: ambos os operandos devem ser `Value::Selector` (ou convertíveis).

2. **Matching em show rules / query**:
   ```rust
   fn matches(content: &Content, selector: &Selector) -> bool {
       match selector {
           Selector::Elem(e) => elem_matches(content, e),
           Selector::Where { base, field, value } => {
               where_match::matches_where(content, base, field, value)
           }
           Selector::And(a, b) => matches(content, a) && matches(content, b),
           Selector::Or(a, b) => matches(content, a) || matches(content, b),
           // ... outros arms
       }
   }
   ```
   - `And`: curto-circuito esquerda-para-direita; se `a` falha, não avalia `b`.
   - `Or`: curto-circuito esquerda-para-direita; se `a` matcha, não avalia `b`.

3. **Casos edge**:
   - `heading & paragraph` → sempre `false` (contraditório). Isso é correto; não é erro.
   - `heading | heading` → redundante mas válido.
   - `(heading.where(level: 1)) & (heading.where(level: 2))` → sempre `false`.

### A.1.5 — Paridade vanilla

- Vanilla: `heading | figure` em `#show`/`#query`; `heading & figure` em `#show`/`#query`; curto-circuito; contraditórios retornam `false`.
- Cristalino P423: paridade semântica (Or/And funcionam em show rules e query); sintaxe via methods (`.or()`, `.and()`) em vez de operadores infixos — mecânica livre per ADR-0107.

### A.1.6 — Scope-out explícito

- Operadores infixos `|` e `&` no parser — scope-out se parser não suportar (opção β é o fallback).
- `Selector::Not` (negação) — scope-out; não existe no enum ainda.
- Otimização de contraditórios (detectar `heading & paragraph` em compile time) — scope-out; runtime `false` é aceitável.
- `Selector::And`/`Or` com mais de 2 operandos (associatividade) — scope-out; encadeamento de methods resolve: `a.or(b).or(c)`.

---

## CHECKPOINT A

**Só prosseguir para Fase B quando confirmar que:**
1. Guardou e computou hash do L0 (A.1) em `selector.md` + `show.md`
2. Sonda A.0 produziu 4/6 OK (mínimo)
3. `And`/`Or` existem no enum mas não no consumer (gap confirmado)
4. Decisão β (methods `.or()`/`.and()`) validada — infraestrutura de method calls existe (P417 `.where()`)

**Hash L0 esperado**: `<computar após redação>`

---

## FASE B — Código

### B.1 — Entities (inalterado)

`Selector::And` e `Selector::Or` já existem no enum (P417). Nenhuma alteração.

### B.2 — Eval / Closures (`rules/eval/closures.rs` ou `rules/eval/bindings.rs`)

Adicionar method handlers para `.or()` e `.and()` em `Value::Selector`:

```rust
// Em eval de method calls para Selector
"or" => {
    let other = args.expect::<Selector>("other")?;
    let base = selector.clone();
    Ok(Value::Selector(Selector::Or(
        Box::new(base),
        Box::new(other),
    )))
}
"and" => {
    let other = args.expect::<Selector>("other")?;
    let base = selector.clone();
    Ok(Value::Selector(Selector::And(
        Box::new(base),
        Box::new(other),
    )))
}
```

**Nota**: se a infraestrutura de method calls para `Selector` já existe (P417 `.where()`), reutilizar o mesmo padrão.

### B.3 — Consumer: Show Rule Matcher (`rules/show/where_match.rs` ou `selector_match.rs`)

Adicionar arms ao `match` de `matches()`:

```rust
Selector::And(a, b) => matches(content, a) && matches(content, b),
Selector::Or(a, b) => matches(content, a) || matches(content, b),
```

**Atomização (ADR-0109)**: se `matches()` já é uma free function magra com `match` delegando, adicionar os dois arms de 1 linha cada. Se `matches()` for monolítico inline, atomizar para `rules/show/selector_match.rs` com `match` magro.

### B.4 — Query (`entities/introspector.rs`)

Adicionar arms ao `query()` de `Introspector`:

```rust
Selector::And(a, b) => {
    let result_a = self.query(a);
    let result_b = self.query(b);
    // Interseção: elementos que estão em ambos
    result_a.into_iter()
        .filter(|item| result_b.contains(item))
        .collect()
}
Selector::Or(a, b) => {
    let mut result = self.query(a);
    result.extend(self.query(b));
    // Deduplicar (se necessário)
    result.sort_by_key(|item| item.id());
    result.dedup_by_key(|item| item.id());
    result
}
```

**Nota**: se `query` retorna `Vec<Content>` ou similar, a interseção/deduplicação depende de `PartialEq` ou `id`. Se não houver `id` único, usar `PartialEq` (semântico, ADR-0107).

### B.5 — Tests

**Mínimo 10 tests**:
- 2 unit `Selector::And`/`Or`: construção, clone, debug
- 2 unit eval: `.or()` constrói `Or`, `.and()` constrói `And`
- 3 unit matching: `And` positivo, `And` negativo (contraditório), `Or` positivo
- 3 E2E: `#show (heading | figure): set text(red)` aplicado a heading, aplicado a figure, não aplicado a paragraph

---

## FASE C — Validação

```bash
cargo test -p typst-core --lib -- selector_and_or
# → 10 passed; 0 failed; 0 ignored

crystalline-lint .
# → 0 errors
# → 0 drift nos prompts tocados
```

**Critério de fecho**:
- [ ] 10 tests verdes
- [ ] Lint zero errors; drift sincronizado
- [ ] `selector1.or(selector2)` constrói `Selector::Or`
- [ ] `selector1.and(selector2)` constrói `Selector::And`
- [ ] `#show heading.or(figure): set text(red)` aplica a heading
- [ ] `#show heading.or(figure): set text(red)` aplica a figure
- [ ] `#show heading.or(figure): set text(red)` não aplica a paragraph
- [ ] `#show heading.and(figure): set text(red)` não aplica a nada (contraditório)
- [ ] Query com `And`/`Or` funciona (se query infrastructure existir)
- [ ] Nenhum vtable/`dyn` introduzido
- [ ] `match` exaustivo preservado
- [ ] Lógica atomizada em free functions (forma B)
- [x] L0 hashado e propagado

---

## Relatório de Execução — P423

**Data**: 2026-06-23
**Executor**: assistente IA (Kimi Code CLI)
**Sonda A.0**:
- `And`/`Or` existem em `01_core/src/entities/selector.rs` (P209C) ✅
- Consumer de show rules (`rules/eval/rules.rs`) não consumia `And`/`Or` ✅ (gap confirmado)
- Query (`entities/introspector.rs`) já consumia `And`/`Or` (P209C) ✅
- Parser de `|`/`&` não existe — opção β (methods `.or()`/`.and()`) mantida ✅

**L0**: `00_nucleo/prompts/entities/show.md` atualizado com variants `And(Vec<Selector>)`/`Or(Vec<Selector>)`, semântica de matching, atomização forma B e tests obrigatórios. Hash recalculado e propagado para `01_core/src/entities/show.rs` (`07f7e3b0`).

**Implementação**:
- `01_core/src/entities/show.rs`: +variants `And(Vec<Selector>)` / `Or(Vec<Selector>)`.
- `01_core/src/engine/eval/bindings.rs`: +`eval_selector_or_and()` com helper `value_to_query_selector()` que aceita `Value::Selector` ou funções nativas `heading`/`figure`.
- `01_core/src/engine/eval/closures.rs`: interceptação de `selector.or(other)` / `selector.and(other)` antes do dispatch genérico.
- `01_core/src/engine/eval/rules.rs`: conversão recursiva `And`/`Or` em `query_selector_to_show_selector`; matching recursivo com curto-circuito em `selector_matches`; `is_node_rule` extraído para função de módulo e estendido para `And`/`Or`.
- `01_core/src/engine/eval/tests.rs`: 6 tests E2E + 8 tests unitários em `rules.rs` (total 16 tests P423).

**Validação**:
- `cargo test -p typst-core --lib -- p423` → 16 passed, 0 failed.
- `crystalline-lint .` → 0 errors (apenas warnings preexistentes de prompts órfãos).
- `cargo build -p typst-core` → ok.

**Notas**:
- O teste `p350c_flag_on_nao_convergente_classifica` apresenta stack overflow, mas verificou-se que é **preexistente** (reproduzível no commit base `fac1f4940` sem as alterações deste passo).
- Sintaxe infixa `|`/`&` permanece scope-out (parser não suporta); paridade semântica atingida via methods.
- Nenhum vtable/`dyn` introduzido; `match` exaustivo preservado; lógica atomizada em free functions.

---

## Notas epistêmicas

- **Medir antes de decidir (ADR-0108)**: A.0 verifica se `And`/`Or` existem no enum e se estão no consumer. Se já estiverem no consumer, o passo é "já implementado" ou S (documentação).
- **Paridade linguagem (ADR-0107)**: O contrato é `Or` = disjunção, `And` = conjunção, curto-circuito. A sintaxe (methods vs operadores) é mecânica livre. O contraditório retornando `false` é semântica correta.
- **Atomização (ADR-0109)**: `Selector::And`/`Or` são dados no enum. O matching vive em `rules/show/selector_match.rs` (ou equivalente) como free function. O eval de methods vive em `rules/eval/closures.rs` (ou equivalente).
- **Honestidade epistêmica**: Se o parser já suportar `|`/`&`, usar (opção γ) sem custo. Se não, methods são honestamente documentados como sintaxe alternativa mecânica livre.
- **Próximo passo P424**: consumer PDF annotation URI (fecha P422) ou `text.lang` rustybuzz (XL, scope-out) ou outro gap do Inventário.
