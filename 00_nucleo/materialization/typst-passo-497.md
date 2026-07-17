---

# P497 — Materialização de Variáveis de Cor Predefinidas e `text()` em Show-Regex (D4/D5)

> **Passo:** 497
> **Data:** 2026-06-29
> **Foco:** Fechar empiricamente os 2 gaps de variáveis de cor predefinidas e `text()` em show-regex identificados no diagnóstico P490 (Grupos D4 e D5), ainda persistentes após P496. Não declarar conclusão — medir antes e depois de cada sub-tarefa.
> **Tipo:** Implementação S-size com validação via bateria P490.
> **Tamanho:** S (~20 min de implementação + 10 min de validação).
> **ADR-0075 ACEITE** — mecanismo de comparação via `typst query --format json` vs `Introspector::query_*`.
> **ADR-0054 ACEITE** — graded parity: MATCH / DIFF / ERRO_DESCRITIVO / PANIC / AUSENTE.
> **ADR-0109 ACEITE** — atomização de código.
> **Dependências:** P496 (D3 fechado), P495 (D2 fechado), P494 (D1 fechado), P247 (stroke), P393/P473 (show-regex), P492 (D4/D5 documentado).

---

## Metodologia

Para cada sub-tarefa D4 e D5, implementar o scope de variáveis de cor, correr a bateria P490 (20 ficheiros), e comparar o output estrutural contra o baseline do diagnóstico P496.

```bash
# Baseline P496 (antes de tocar código):
cargo test --test structural_parity p496_field_access_colecoes

# Após cada sub-tarefa:
cargo test --test structural_parity p497_<subtarefa>
```

Classificar cada resultado: `MATCH` | `DIFF` | `ERRO_DESCRITIVO` | `PANIC` | `AUSENTE`.

---

## Categoria 1 — Variáveis de cor em `stroke: dict` (D4)

### 1.1 — Estado baseline (P496)

```typst
// test-stroke-sides.typ
#rect(stroke: (left: 3pt + red, right: 1pt + blue, top: none, bottom: 2pt + green))
```

- **Vanilla:** `ok(0)` — rect renderiza com stroke colorido; query `heading` retorna count=0 (sem headings).
- **Cristalino (pré-497):** `ERRO_DESCRITIVO` — `red`, `blue`, `green` não reconhecidas como variáveis no scope de avaliação do dict `stroke`.
- **Classificação P496:** **DIFF**

### 1.2 — Diagnóstico de causa raiz

O ripgrep do P491 mostrou que `red`, `blue`, `green` não aparecem em nenhum arquivo de definição de variáveis predefinidas no cristalino. No vanilla, estas são variáveis globais do módulo `color` (ex: `color.red`, `color.blue`, `color.green`) expostas diretamente no scope global como atalhos.

**Hipóteses:**
1. **H1:** As cores existem como `Value::Color` em `color.rs` mas não estão injetadas no `Scope` global.
2. **H2:** As cores existem em `color.rs` mas o nome do binding difere (ex: `Color::red()` vs `red`).
3. **H3:** As cores não existem como `Value` constantes — só como construtores dinâmicos.

**Verificação rápida:**
```bash
rg -n "red|blue|green" src/stdlib/color.rs --type rs
rg -n "Scope::new_global|global_scope" src/ --type rs
```

### 1.3 — Implementação

**Arquivo alvo:** `src/eval/scope.rs` (ou `src/stdlib/color.rs`)

**Mudança:** Injetar bindings `red`, `blue`, `green`, `black`, `white`, `yellow`, `cyan`, `magenta` (e outras cores predefinidas do vanilla) no scope global durante a inicialização do eval.

**Pseudo-código:**
```rust
// Em src/eval/scope.rs ou src/stdlib/mod.rs (onde o scope global é construído)
let global = Scope::new_global();

// Cores predefinidas (subset vanilla — expandir conforme necessário)
global.define("red", Value::Color(Color::from_rgb(0xEF, 0x23, 0x11)));
global.define("blue", Value::Color(Color::from_rgb(0x00, 0x5E, 0xE5)));
global.define("green", Value::Color(Color::from_rgb(0x00, 0xB3, 0x00)));
global.define("black", Value::Color(Color::from_rgb(0x00, 0x00, 0x00)));
global.define("white", Value::Color(Color::from_rgb(0xFF, 0xFF, 0xFF)));
global.define("none", Value::None);
// ... outras cores conforme cobertura desejada
```

**Nota:** `none` já deve existir como `Value::None` no scope global. Se não existir, adicionar junto com as cores.

### 1.4 — Validação

Rodar `test-stroke-sides.typ` contra vanilla e cristalino. Esperado:
- `stroke: (left: 3pt + red, ...)` → **MATCH** (compila sem erro, rect com stroke colorido)
- `stroke: 1pt + red` (forma simples) → **MATCH** (não-regressão de P247)

---

## Categoria 2 — `text()` em show-rule com regex (D5)

### 2.1 — Estado baseline (P496)

```typst
// test-show-regex.typ
#show regex("\d+"): it => text(red, it)
O número 42 e o número 100 aparecem a vermelho.
```

- **Vanilla:** `ok(0)` — texto "42" e "100" ficam vermelhos; query `heading` retorna count=0 (sem headings).
- **Cristalino (pré-497):** `ERRO_DESCRITIVO` — `text` não reconhecido em contexto de show-regex, ou `red` não reconhecido (gap D4 propagado).
- **Classificação P496:** **DIFF**

### 2.2 — Diagnóstico de causa raiz

**Hipóteses:**
1. **H1:** O gap D5 é **puramente D4** — `text()` existe mas `red` não está no scope, causando erro no argumento `red`.
2. **H2:** O gap D5 é **independente** — `text()` não está disponível no contexto de show-rule (problema de scope do show-rule, não de variáveis globais).
3. **H3:** Ambos — `text` não está no scope do show-rule E `red` não está no scope global.

**Verificação rápida:**
```bash
# Testar isoladamente:
#show regex("\d+"): it => text(blue, it)   // se blue também falha → H1 ou H3
#show regex("\d+"): it => underline(it)     // se underline funciona → H2 (text ausente no scope)
```

### 2.3 — Implementação

**Se H1 (D5 = D4):** Resolver D4 automaticamente resolve D5. Nenhuma mudança adicional necessária.

**Se H2/H3 (D5 independente):** Verificar se `text` está no scope do show-rule. O show-rule cria um novo scope local; se `text` não é herdado do scope global, adicionar herança ou injetar `text` explicitamente.

**Arquivo alvo:** `src/eval/show.rs` ou `src/engine/show.rs`

**Pseudo-código:**
```rust
// No eval de show-rule, garantir que o scope do callback herda o scope global
let mut local_scope = Scope::child(parent_scope); // ou equivalente
// text já deve estar disponível via herança se foi injetado no global
```

### 2.4 — Validação

Rodar `test-show-regex.typ` contra vanilla e cristalino. Esperado:
- `text(red, it)` → **MATCH** (texto "42" e "100" ficam vermelhos)
- `text(blue, it)` → **MATCH** (não-regressão)
- `underline(it)` → **MATCH** (show-rule base funciona, não-regressão P393/P473)

---

## Formato do relatório de resultados

Para cada sub-tarefa, produzir uma linha:

```
| 497a stroke cores | Vanilla: ok(0) | Cristalino: ok(0) | MATCH | red/blue/green/none no scope global |
| 497b show-regex text | Vanilla: ok(0) | Cristalino: ok(0) | MATCH | D5 = D4 (resolvido por herança) |
```

Colunas: `sub-tarefa | vanilla | cristalino | classificação | notas`

**Classificações:**
- `MATCH` — output estruturalmente equivalente.
- `DIFF` — ambos produzem resultado mas diferente (investigar).
- `ERRO_DESCRITIVO` — cristalino retorna erro com mensagem clara (aceitável se scope-out).
- `PANIC` — cristalino crasha (bug prioritário, deve ser zero).
- `AUSENTE` — funcionalidade não reconhecida.

---

## Testes de não-regressão

Após as 2 sub-tarefas, rodar a bateria completa P490 (20 ficheiros):

| Ficheiro | Selector | Vanilla | Cristalino pré-497 | Esperado pós-497 | Δ |
|---|---|---|---|---|---|
| test-stroke-sides.typ | `heading` | ok(0) | ERRO_DESCRITIVO | ok(0) | **DIFF → MATCH** |
| test-show-regex.typ | `heading` | ok(0) | ERRO_DESCRITIVO | ok(0) | **DIFF → MATCH** |
| test-show-where-multi.typ | `heading` | ok(1) | ok(0) | ok(0) | DIFF (D3c residual) |
| test-table.typ | `table` | ok(1) | ok(1) | ok(1) | MATCH (P496) |
| test-array.typ | `metadata` | ok(0) | ok(0) | ok(0) | MATCH (P496) |
| (outros 15) | — | — | MATCH | MATCH | preservados |

**DIFFs restantes esperados:** 3 - 2 = **1** (D3c residual: show-where-multi, divergência arquitectural).  
**PANICs esperados:** **0** (preservado).  
**AUSENTEs esperados:** 2 (test-set-local, test-columns — count=0, não regressão).

**Resultado final da bateria P490 pós-P497:**
- **MATCH:** 17/20 ficheiros (incluindo os 2 AUSENTEs que são count=0 esperados)
- **DIFF:** 1/20 (D3c residual — show-where-multi, divergência arquitectural documentada)
- **AUSENTE:** 0 (todos os AUSENTEs do P490 resolvidos em P494)
- **PANIC:** 0

---

## Critério de fecho

- [ ] 497a implementado: `red`, `blue`, `green`, `none` (e outras cores vanilla) no scope global.
- [ ] 497b implementado: `text(red, it)` funciona em contexto de show-regex (ou confirmado que D5 = D4).
- [ ] 2 testes unitários novos passam (`p497_stroke_cores_predefinidas`, `p497_show_regex_text_color`).
- [ ] Bateria P490 completa: 2 DIFFs de D4/D5 viraram MATCH.
- [ ] DIFFs restantes: **1** (D3c residual, divergência arquitectural documentada).
- [ ] PANICs: 0 (preservado).
- [ ] AUSENTEs: 0 (todos resolvidos em P494).
- [ ] Documentação atualizada (`rules/stdlib/color.md` com lista de cores predefinidas; `rules/style/stroke.md` se aplicável).
- [ ] ADR-0107 checklist atualizado (2 itens marcados implementado).
- [ ] Sentinela `p497_variaveis_cor_predefinidas` adicionada em `lab/parity/tests/structural_parity.rs`.
- [ ] `00_nucleo/diagnosticos/paridade-funcional-p497.md` produzido com tabela de resultados.

---

## Próximo passo (P498)

Com P497 fechado, a bateria P490 atinge **17/20 MATCH** com 1 DIFF residual (D3c) e 0 PANICs.

**Recomendações para P498+:**

1. **P498 = Audit de cobertura stdlib expandida** — criar novos ficheiros `.typ` para funcionalidades não cobertas pela bateria P490 (ex: `image` com `fit`, `page` com `header`/`footer`, `place` avançado, `calc` args nomeados restantes).

2. **P498 = Performance benchmark** — comparar tempo de compilação cristalino vs vanilla em corpus maior (DEBT-42).

3. **P498 = Documentação de paridade** — produzir relatório final de paridade funcional consolidando P490-P497, com matriz de cobertura stdlib e nota sobre D3c residual.

4. **P498 = DEBT-42 Benchmark Plan** — executar o plano de benchmark documentado em `typst-cobertura-vaistalino.md`.

---

## A. Apêndice — Referência rápida dos gaps D4/D5

```typst
// 497a: stroke com cores predefinidas
#rect(stroke: (left: 3pt + red, right: 1pt + blue, top: none, bottom: 2pt + green))

// 497b: show-regex com text() e cor
#show regex("\d+"): it => text(red, it)
O número 42 aparece a vermelho.

// Cores vanilla a materializar (subset mínimo)
// red, blue, green, black, white, yellow, cyan, magenta, none
```

---

## B. Apêndice — Cores vanilla (referência para implementação)

| Nome | Hex (RGB) | Notas |
|---|---|---|
| `red` | `#EF2311` | Vermelho primário |
| `blue` | `#005EE5` | Azul primário |
| `green` | `#00B300` | Verde primário |
| `black` | `#000000` | Preto |
| `white` | `#FFFFFF` | Branco |
| `yellow` | `#F5D800` | Amarelo |
| `cyan` | `#00B3B3` | Ciano |
| `magenta` | `#E500E5` | Magenta |
| `none` | — | `Value::None` (já deve existir) |

**Nota:** O vanilla tem um palette extenso (maroon, olive, lime, purple, teal, navy, etc.). O subset acima cobre os gaps D4/D5 do P490. O palette completo pode ser materializado em passo futuro (DEBT).
