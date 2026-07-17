---

# P473 — `op. cit.` + wiring de `#show regex(...)`

> **Passo:** 473
> **Data:** 2026-06-26
> **Foco:** (1) `op. cit.` — forma abreviada para citação não-consecutiva de entrada já citada; (2) Wiring de `#show regex(...): it => ...` — `Value::Regex`, `Selector::Regex` em show-rules, aplicação em nós de texto.
> **Trilhas:** 6 (sub-item A) + 3 (sub-item B).
> **Tipo:** Materialização.
> **Tamanho:** S + S (~35 min total; dois sub-itens independentes).
> **ADR-0117 Cláusula 4:** Sub-item A reutiliza `last_cited_key` (P472) e `back_refs_for_key` (P472). Sub-item B reutiliza `entities::regex::Regex` existente (ADR-0077) e `apply_show_rules` em `rules/eval/rules.rs` (P70). Verificar antes de propor campo novo.

---

## Contexto

**Sub-item A — `op. cit.`** (Trilha 6):
P472 materializou `ibid.` para citações *consecutivas* da mesma entrada. `op. cit.` cobre o caso complementar: a mesma entrada é citada de novo, mas não consecutivamente (outras entradas foram citadas entre a primeira e a actual ocorrência). A infra necessária — `back_refs_for_key` e `last_cited_key` — já existe desde P472.

**Sub-item B — `#show regex(...)`** (Trilha 3):
O roteiro lista `#show regex(...)` como item pendente de Trilha 3. O L0 `show-regex.md` já documenta a arquitectura completa (passo de origem P393). O tipo `Regex` existe em L1 (`entities/regex.rs`, ADR-0077). O que falta é: `Value::Regex`, `Selector::Regex` em `entities/show.rs`, wiring em `eval_show_rule`, e aplicação em `apply_show_rules`.

Os dois sub-itens são inteiramente independentes entre si.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `last_cited_key: Option<String>` existe no Layouter? | Sim — P472 | ✅ |
| `back_refs_for_key` existe no Introspector? | Sim — P472 | ✅ |
| `ibid.` detectado em `cite.rs`? | Sim — Numeric + Normal + consecutiva | ✅ |
| `op. cit.` requer tracking de keys anteriores? | Sim — qualquer key já citada e não consecutiva | 🟡 sonda |
| `Value::Regex` existe? | Não — comentado como variante futura em `value.rs` | ❌ |
| `Selector::Regex` existe em `entities/show.rs`? | Verificar — `Regex` existe em L1; variant no `show.rs` não confirmado | 🟡 sonda |
| `apply_show_rules` em `rules/eval/rules.rs` existe? | Sim — P70 | ✅ |
| `native_regex` em stdlib registado? | Verificar `rules/stdlib/text.rs` ou similar | 🟡 sonda |
| `Regex::new(pattern)` retorna `Result`? | Sim — crate `regex` | ✅ |
| Bloqueadores técnicos? | Nenhum conhecido | ✅ |

**Sondas 🟡 com `grep`/`file:line` antes de escrever código.**

---

## Sub-item A — `op. cit.`

### A.1 — Contexto e distinção face a `ibid.`

| Forma | Condição | Exemplo |
|-------|----------|---------|
| `ibid.` | Mesma key, citação imediatamente anterior | `[1] ibid.` |
| `op. cit.` | Mesma key, citada antes mas não imediatamente | `[1] Knuth, op. cit.` |

No cristalino, `ibid.` detecta `last_cited_key == current_key`. `op. cit.` detecta `current_key ∈ previously_cited_keys AND last_cited_key != current_key`.

### A.2 — Infra necessária

**Campo `previously_cited_keys: HashSet<String>`** no Layouter — conjunto de todas as keys que já foram citadas antes da posição actual, excluindo a última (`last_cited_key` cobre esse caso).

```rust
// Em Layouter:
pub(super) last_cited_key:        Option<String>,          // P472 — para ibid.
pub(super) previously_cited_keys: HashSet<String>,         // P473 NOVO — para op. cit.
```

**Actualização** no arm de `Content::Cite` após renderizar:

```rust
// Antes de processar a citação actual:
let is_ibid   = style == Numeric && form == Normal
    && layouter.last_cited_key.as_deref() == Some(key.as_str());
let is_op_cit = style == Numeric && form == Normal
    && !is_ibid
    && layouter.previously_cited_keys.contains(key.as_str());

// Após processar (independentemente da forma escolhida):
if let Some(prev) = &layouter.last_cited_key {
    layouter.previously_cited_keys.insert(prev.clone());
}
layouter.last_cited_key = Some(key.clone());
```

**Ordem das operações:** (1) computar `is_ibid` e `is_op_cit` com os campos actuais; (2) atualizar `previously_cited_keys` com `last_cited_key` anterior; (3) atualizar `last_cited_key` com a key atual.

### A.3 — Render de `op. cit.`

```rust
if is_op_cit {
    // Formato: "[N] Author, op. cit."
    // onde N é o número de citação da entry, Author é o apelido do primeiro autor
    let n = citation_number;
    let author_abbrev = entry
        .author.as_deref()
        .and_then(|a| a.split(',').next())
        .unwrap_or(&entry.key);
    let text = format!("[{}] {}, op. cit.", n, author_abbrev);
    layouter.layout_content(&Content::text(text));
    return;
}
```

**Alternativa minimalista** (sem apelido): renderizar apenas `[N] op. cit.` — mais simples e menos dependente da estrutura de `BibEntry`. A sonda deve verificar se `BibEntry` tem campo `author` acessível.

**Decisão:** usar o apelido se acessível; fallback para `[N] op. cit.` sem autor. Divergência declarada face ao vanilla (que usa renderização CSL completa).

### A.4 — Interacção com `ibid.`

A detecção é mutuamente exclusiva por construção:

```
ibid.   = last_cited_key == current_key
op.cit. = previously_cited_keys.contains(current_key) AND NOT ibid.
normal  = NOT ibid. AND NOT op. cit.
```

### A.5 — Scope-out

- **`op. cit.` em estilos não-numéricos** — apenas `Numeric + Normal`.
- **`ibid., p. N`** / **`op. cit., p. N`** com page override — scope-out.
- **Reset de `previously_cited_keys` em nova secção** — não implementado; op. cit. vale para todo o documento.
- **`loco citato` / `loc. cit.`** — scope-out.

---

## Sub-item B — Wiring de `#show regex(...)`

O L0 `show-regex.md` documenta a arquitectura completa. Este sub-item executa-a.

### B.1 — `Value::Regex`

**Ficheiro:** `entities/value.rs`

Adicionar variante ao enum `Value`:

```rust
/// **P473** — Valor regex. Tipo `Regex` existe em L1 (ADR-0077).
Regex(Regex),
```

- `type_name()` → `"regex"`.
- `repr()` → `"regex(\"pattern\")"` (ler `re.as_str()`).
- `From<Regex> for Value`.

**Nota:** verificar se `Regex` implementa `PartialEq`, `Clone`, e `Hash`. Se não, o variant `Regex(Regex)` pode precisar de `Arc<Regex>`. Sonda antes.

### B.2 — `Selector::Regex` em `entities/show.rs`

O `entities/show.rs` define o `Selector` local de show-rules (distinto do `entities/selector.rs` de query). Verificar se já tem `Regex` variant.

```rust
// Em entities/show.rs — enum Selector (show-rules):
pub enum Selector {
    Text(EcoString),
    NodeKind(NodeKind),
    // ... existentes ...
    Regex(Regex),   // P473 NOVO
}
```

### B.3 — Construtor `native_regex`

**Ficheiro:** `rules/stdlib/text.rs` (ou módulo regex)

```rust
pub fn native_regex(_ctx, args, ..) -> Result<Value, EvalError> {
    let pattern: EcoString = args.expect_positional("pattern")?;
    args.expect_no_more()?;
    let re = Regex::new(&pattern)
        .map_err(|e| EvalError::custom(format!("regex: {e}")))?;
    Ok(Value::Regex(re))
}
```

Registado no scope como `"regex"`.

### B.4 — Wiring em `eval_show_rule`

**Ficheiro:** `rules/eval/rules.rs` (ou `eval/mod.rs`)

No arm de eval da show-rule, quando o selector avalia para `Value::Regex(re)`:

```rust
Value::Regex(re) => Selector::Regex(re),
```

`Transformation::Style` permanece inválido para `Selector::Regex` (paridade `Selector::Text`).

### B.5 — Aplicação em `apply_show_rules`

**Ficheiro:** `rules/eval/rules.rs`

Após o loop existente de NodeKind/DynKind, antes de retornar:

```rust
// Para cada show-rule com Selector::Regex:
for rule in show_rules.iter().rev() {
    if let Selector::Regex(re) = &rule.selector {
        content = content.map_text(|text| {
            if re.is_match(text) {
                apply_transformation(&rule.transformation, Content::text(text.to_string()))
            } else {
                Content::text(text.to_string())
            }
        });
    }
}
```

**Nota:** `map_text` percorre recursivamente todos os nós de texto. O comportamento é: nó de texto que casa é transformado inteiro (sem split interno). A paridade é semântica (ADR-0107) — todo o nó que casa é substituído.

### B.6 — Spec L0 actualizada

O L0 `show-regex.md` já existe com hash `a3f2c53b`. Após implementação, actualizar hash e marcar como `materializado P473`.

---

## Tests

### Sub-item A

- **L1:** `previously_cited_keys` vazio por defeito no Layouter.
- **L2:** Primeira citação de "knuth73" → normal `[1]`. Segunda citação consecutiva → `ibid.`. Terceira citação de "other80" → `[2]`. Quarta citação de "knuth73" → `op. cit.` (não consecutiva).
- **L2:** `ibid.` e `op. cit.` são mutuamente exclusivos no mesmo arm.
- **L2:** Estilo `AuthorDate` ignora `op. cit.` (apenas `Numeric + Normal`).
- **L2:** Após citação nova ("other80"), a key anterior ("knuth73") entra em `previously_cited_keys`.

### Sub-item B

- **L1:** `Value::Regex(re).type_name() == "regex"`.
- **L1:** `native_regex("\\d+")` → `Value::Regex(re)` com `re.is_match("abc123") == true`.
- **L1:** `native_regex("[")` → erro de eval (regex inválida).
- **L2:** `#show regex("\\d+"): it => strong(it)` sobre texto `"abc123"` → resultado contém styled strong.
- **L2:** Mesmo show-rule sobre texto `"abc"` (sem dígitos) → sem alteração.
- **L2:** Múltiplas show-rules (regex + NodeKind) — ambas aplicadas independentemente.
- **L2:** `native_regex` rejeita argumentos nomeados → erro descritivo.

---

## Spec L0

### Sub-item A

- `engine/layout/cite.md` — secção `op. cit.`, campo `previously_cited_keys`, interacção com `ibid.`.

### Sub-item B

- `rules/show-regex.md` — marcar `materializado P473`; actualizar hash.
- `entities/value.md` — `Value::Regex` variant; remover de lista de variantes futuras.

---

## Critério de fecho

- [ ] Sondas: `previously_cited_keys` (ausente no Layouter), `Value::Regex` (ausente), `Selector::Regex` em `show.rs`, `native_regex` (ausente) — todos com `file:line`.
- [ ] `previously_cited_keys: HashSet<String>` adicionado ao Layouter.
- [ ] Arm de `Content::Cite` actualiza `previously_cited_keys` antes de actualizar `last_cited_key`.
- [ ] `is_op_cit` detectado; render `[N] op. cit.` (com ou sem autor conforme disponibilidade).
- [ ] `ibid.` e `op. cit.` mutuamente exclusivos.
- [ ] `Value::Regex(Regex)` adicionado ao enum `Value`.
- [ ] `type_name()`, `repr()`, `From<Regex>` para `Value::Regex`.
- [ ] `native_regex` implementado e registado no scope como `"regex"`.
- [ ] `Selector::Regex(Regex)` adicionado a `entities/show.rs`.
- [ ] `eval_show_rule` mapeia `Value::Regex(re)` → `Selector::Regex(re)`.
- [ ] `apply_show_rules` aplica regras `Selector::Regex` sobre `Content::Text`.
- [ ] 12+ testes verdes (5 sub-A + 7 sub-B).
- [ ] Spec L0 actualizada (3 ficheiros).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **Trilha 6: 4/5 completo** (estilos numéricos P468 + back-refs/ibid. P472 + LoF/LoT P472 + op. cit. P473).
- [ ] **Trilha 3: 2/3 completo** (sonda `Selector::Where` P467 + `#show regex(...)` P473).

---

## Próximo passo

Com P473, o panorama fica:

- **Trilha 6:** 4/5 completo. Restante: `LoF/LoT com page numbers` (requer 2-pass) — provável scope-out longo prazo.
- **Trilha 3:** 2/3 completo. Restante: `Selector::Where` + `#show elem.where(field: value)` — depende do resultado da sonda P467.
- **Trilha 8:** 5/8 completo. Restante: `pad`/`corners`/`sides` (sonda antes de propor — pode já estar fechado); parâmetros restantes decorações.

Opções para P474:

- **P474** — `Selector::Where` + `#show heading.where(level: N)` (Trilha 3, M) — fecha Trilha 3.
- **P474** — Sonda real de `pad`/`corners`/`sides` para verificar se Trilha 8 tem itens pendentes reais.
- **P474** — Extensão de espaços de cor (`cmyk`, `oklab` user-facing) — Trilha 4 refino.

---

## Estado pós-P472 (para referência)

| Passo | Descrição | Estado | Trilha |
|-------|-----------|--------|--------|
| P465 | `repr()` completo | ✅ FECHADO | 8 |
| P466 | Métodos array/dict/str | ✅ FECHADO | 8 |
| P467 | Sonda `Selector::Where` | ✅ FECHADO | 3 |
| P468 | Estilos numéricos `[1]`, `[2]` | ✅ FECHADO | 6 |
| P469 | `Value::Relative` (`Rel<Length>`) | ✅ FECHADO | 8 |
| P470 | Marcadores list/enum + i18n caption | ✅ FECHADO | 8 |
| P471 | `Symbol` + `highlight` params + `sub`/`super` size | ✅ FECHADO | 8 |
| P472 | Back-refs + ibid. + LoF/LoT | ✅ FECHADO | 6 |
| **P473** | `op. cit.` + `#show regex(...)` | 🔄 EM PREPARAÇÃO | 6 + 3 |

**Trilha 1: COMPLETA.**
**Trilha 2: COMPLETA.**
**Trilha 3: 1/3 completo** (pré-P473).
**Trilha 4: COMPLETA** (Linear/Radial/Conic/Focal — P262–P269).
**Trilha 6: 3/5 completo** (pré-P473).
**Trilha 7: COMPLETA** (columns/colbreak — P216–P221).
**Trilha 8: 5/8 completo.**
**Inventário de débitos: LIMPO.**
