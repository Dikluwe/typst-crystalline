---

# P498 — Separação de Elemento Locatable vs. Output de Show-Rule (D3c Residual)

> **Passo:** 498
> **Data:** 2026-06-29
> **Foco:** Fechar o último gap da bateria P490: `query(heading)` retorna count=0 após `#show heading.where(...)` porque show-rules consomem o elemento locatable durante o eval. Não declarar conclusão — medir antes e depois.
> **Tipo:** Implementação arquitetural M-size com validação via bateria P490.
> **Tamanho:** M (~60 min de análise + implementação + validação).
> **ADR-0075 ACEITE** — mecanismo de comparação via `typst query --format json` vs `Introspector::query_*`.
> **ADR-0054 ACEITE** — graded parity: MATCH / DIFF / ERRO_DESCRITIVO / PANIC / AUSENTE.
> **ADR-0109 ACEITE** — atomização de código.
> **Dependências:** P497 (D4/D5 fechado), P496 (D3a/b fechado), P474 (show-where-multi scope-out original), P393/P473 (show-regex).

---

## Metodologia

Analisar a divergência arquitetural entre vanilla e cristalino no tratamento de show-rules, implementar a separação entre elemento locatable e output renderizado, correr a bateria P490 (20 ficheiros), e comparar o output estrutural.

```bash
# Baseline P497 (antes de tocar código):
cargo test --test structural_parity p497_variaveis_cor_predefinidas

# Após implementação:
cargo test --test structural_parity p498_d3c_residual
```

Classificar cada resultado: `MATCH` | `DIFF` | `ERRO_DESCRITIVO` | `PANIC` | `AUSENTE`.

---

## Categoria 1 — Diagnóstico da Divergência Arquitetural (D3c)

### 1.1 — Estado baseline (P497)

```typst
// test-show-where-multi.typ
#show heading.where(level: 1, outlined: true): it => upper(it.body)
= Heading nível 1
```

- **Vanilla:** `typst query test-show-where-multi.typ "heading"` → `ok(1)` — o heading é locatable mesmo após a transformação de show-rule.
- **Cristalino (pré-498):** `query(heading)` → `count=0` — o `Content::Heading` foi substituído pelo output da show-rule durante o eval, deixando de existir no documento final.
- **Classificação P497:** **DIFF** (arquitetural)

### 1.2 — Causa raiz

No **vanilla**, o modelo de conteúdo separa **elementos** (o que o usuário escreveu) de **output** (o que aparece na página). O `Introspector` consulta o **modelo de elementos** (pré-show-rules), não o documento renderizado. Uma show-rule transforma o *output* visual, mas o *elemento* original persiste para introspection.

No **cristalino**, o eval aplica show-rules **diretamente no Content**, substituindo `Content::Heading` pelo output da transformação. O `Introspector` é construído sobre o conteúdo **pós-show-rules**, pelo que o heading deixa de existir.

**Hipóteses de resolução:**

| Hipótese | Descrição | Impacto | Risco |
|----------|-----------|---------|-------|
| **H1** | Guardar uma cópia do AST original (pré-show-rules) e fazer o `Introspector::query` sobre essa cópia. | Médio — duplica memória para introspection. | Baixo — não altera semântica de renderização. |
| **H2** | Marcar o elemento como "locatable" mesmo após transformação (tag persistente no Content). | Alto — altera o modelo de dados de Content. | Médio — pode afetar outras partes do eval. |
| **H3** | Separar o modelo em "source" (AST original) e "rendered" (output pós-show-rules), como o vanilla faz. | Alto — refactor arquitetural significativo. | Alto — touch em múltiplas camadas. |
| **H4** | Aplicar show-rules apenas no layout/render, não no eval de conteúdo. | Alto — move show-rules de eval para layout. | Alto — pode quebrar show-rules que dependem de eval. |

**Recomendação:** **H1** — guardar cópia do AST original para introspection. É o menor impacto arquitetural que resolve o gap sem alterar a semântica de renderização.

### 1.3 — Verificação rápida

```bash
# Entender como o Introspector é construído:
rg -n "Introspector::new\|struct Introspector" src/ --type rs

# Entender onde show-rules são aplicadas:
rg -n "apply_show_rule\|show_rule" src/eval/ --type rs

# Verificar se existe separação source/rendered:
rg -n "source.*content\|original.*content" src/ --type rs
```

---

## Categoria 2 — Implementação (H1: Cópia do AST Original)

### 2.1 — Arquitetura

```
[Parsing] → [AST Original] → [Eval com Show-Rules] → [Content Pós-Show-Rules]
                ↓                                          ↓
         [Introspector::query] ← [Guarda referência]   [Layout/Render]
```

O `Introspector` deve receber uma referência ao **AST original** (pré-show-rules), não ao conteúdo pós-show-rules.

### 2.2 — Arquivo alvo

**`src/introspect/mod.rs`** (ou onde `Introspector` é definido)

**Mudança 1:** Adicionar campo `original_content` ao `Introspector`:

```rust
pub struct Introspector {
    /// Conteúdo pós-show-rules (para layout/render).
    rendered: Content,
    /// Conteúdo pré-show-rules (para query/introsppection).
    original: Content,
}

impl Introspector {
    pub fn new(rendered: Content, original: Content) -> Self {
        Self { rendered, original }
    }

    pub fn query(&self, selector: &Selector) -> Vec<Content> {
        // Query sobre o AST original, não o renderizado
        self.query_recursive(&self.original, selector)
    }
}
```

**Mudança 2:** No ponto onde o `Introspector` é construído (provavelmente após o eval), passar o AST original:

```rust
// Antes:
let introspector = Introspector::new(content);

// Depois:
let original_content = content.clone(); // ou guardar antes de aplicar show-rules
let content = apply_show_rules(content, &scope)?;
let introspector = Introspector::new(content, original_content);
```

**Nota:** Se `Content` não implementa `Clone` de forma eficiente, considerar `Rc<Content>` ou `Arc<Content>` para evitar duplicação profunda.

### 2.3 — Alternativa: H1b (Lazy Clone)

Se o clone do AST original for custoso, implementar **lazy clone** — só clonar os sub-ramos que são locatable (headings, figures, etc.), não o documento inteiro:

```rust
pub struct Introspector {
    rendered: Content,
    locatable_elements: Vec<Content>, // só elementos que podem ser queried
}

// Durante o eval, quando um elemento locatable é criado, guardar referência:
fn eval_heading(...) -> Content {
    let heading = Content::Heading(...);
    introspector.register_locatable(heading.clone());
    heading
}
```

Isso evita duplicar o documento inteiro, mas requer tocar no eval de cada elemento locatable.

---

## Categoria 3 — Validação

### 3.1 — Teste principal

Rodar `test-show-where-multi.typ` contra vanilla e cristalino:

```bash
# Vanilla:
typst query --format json test-show-where-multi.typ "heading"
# Esperado: [{"body": "Heading nível 1", "level": 1, ...}]

# Cristalino:
cargo run -p typst-wiring -- query test-show-where-multi.typ "heading"
# Esperado pós-P498: count=1, conteúdo do heading original
```

### 3.2 — Testes de não-regressão

Rodar a bateria completa P490 (20 ficheiros). Esperado:

| Ficheiro | Selector | Vanilla | Cristalino pré-498 | Esperado pós-498 | Δ |
|---|---|---|---|---|---|
| test-show-where-multi.typ | `heading` | ok(1) | ok(0) | ok(1) | **DIFF → MATCH** |
| (outros 19) | — | — | MATCH | MATCH | preservados |

**Resultado final esperado:**
- **MATCH:** 20/20
- **DIFF:** 0
- **AUSENTE:** 0
- **PANIC:** 0

---

## Formato do relatório de resultados

```
| 498 D3c residual | Vanilla: ok(1) | Cristalino: ok(1) | MATCH | AST original guardado para introspection |
```

Colunas: `sub-tarefa | vanilla | cristalino | classificação | notas`

---

## Critério de fecho

- [ ] Diagnóstico arquitetural completo: vanilla vs cristalino no tratamento de show-rules documentado.
- [ ] Implementação H1 (ou alternativa escolhida) funciona: `query(heading)` retorna count=1 após show-rule.
- [ ] Teste unitário `p498_d3c_residual` passa.
- [ ] Bateria P490 completa: **20/20 MATCH**.
- [ ] DIFFs restantes: **0**.
- [ ] PANICs: 0 (preservado).
- [ ] AUSENTEs: 0 (preservados).
- [ ] Documentação arquitetural atualizada (ADR sobre separação source/rendered, se necessário).
- [ ] Sentinela `p498_d3c_residual` adicionada em `lab/parity/tests/structural_parity.rs`.
- [ ] `00_nucleo/diagnosticos/paridade-funcional-p498.md` produzido com tabela de resultados.
- [ ] **Bateria P490: 20/20 MATCH** — paridade funcional completa alcançada.

---

## Próximo passo (P499)

Com P498 fechado, a bateria P490 atinge **20/20 MATCH**. A paridade funcional do diagnóstico inicial está completa.

**Recomendações para P499+:**

1. **P499 = Audit de cobertura stdlib expandida** — criar novos ficheiros `.typ` para funcionalidades não cobertas pela bateria P490 (ex: `image` com `fit`, `page` com `header`/`footer`, `place` avançado, `calc` args nomeados restantes, `str` métodos, `dict` métodos).

2. **P500 = Performance benchmark** — comparar tempo de compilação cristalino vs vanilla em corpus maior (DEBT-42).

3. **P501 = Relatório final de paridade P490–P498** — consolidar a matriz de cobertura stdlib, documentar decisões arquiteturais (D3c), e produzir artefacto para publicação.

---

## A. Apêndice — Referência rápida do gap D3c

```typst
// 498: show where multi-field com query
#show heading.where(level: 1, outlined: true): it => upper(it.body)
= Heading nível 1

// Vanilla: typst query ... "heading" → count=1
// Cristalino pré-498: query(heading) → count=0 (elemento consumido pela show-rule)
// Cristalino pós-498: query(heading) → count=1 (elemento original preservado para introspection)
```

---

## B. Apêndice — Decisão arquitetural: Vanilla vs Cristalino

| Aspecto | Vanilla | Cristalino (pré-P498) |
|---------|---------|----------------------|
| Show-rules | Aplicadas no **layout**, não no eval | Aplicadas no **eval**, substituindo Content |
| Introspection | Consulta **AST original** (pré-show-rules) | Consulta **Content pós-show-rules** |
| Memória | Mantém duas representações (source + rendered) | Mantém uma representação (rendered) |
| Complexidade | Maior — separação de concerns | Menor — eval único |

A decisão do P498 (H1) adota o modelo vanilla para introspection sem alterar o modelo de eval do cristalino.
