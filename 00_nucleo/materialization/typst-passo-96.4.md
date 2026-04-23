# Passo 96.4 — Reestruturação de `parse.rs` em submódulos por tipo de nó

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md` — ADR
  `EM VIGOR` com as 7 regras e 4 ajustes. Este passo aplica a
  ADR pela segunda vez após promoção.
- `00_nucleo/DEBT.md` — entrada DEBT-46, checkbox do 96.4
  pendente.
- `01_core/src/rules/parse.rs` — ficheiro actual, 2255 linhas.
- Passo 96.1 e 96.2 para referência do padrão aplicado a
  `eval.rs`.

Pré-condição: `cargo test` — 746 L1 + 174 L3 + 6 ignorados,
zero violations. Passo 96.3 concluído (ADR-0037 `EM VIGOR`).

---

## Natureza deste passo

Passo único de reestruturação. Aplica ADR-0037 a `parse.rs`:
divisão por tipo de nó em submódulos `parse/`.

Em comparação com o `eval.rs`:
- **Menos complexo**: as funções de parse têm menos dependências
  cruzadas; não há `EvalContext` equivalente a propagar.
- **Clusters naturais mais óbvios**: `markup`, `code`, `math`,
  `rules`, `patterns` são divisões claras.
- **Menos armos em dispatcher**: parse tem pontos de entrada
  (`parse`, `parse_code`, `parse_math`), não dispatcher enorme.

Expectativa: execução mais rápida que o 96.1 (~60% do tempo).

**Não altera semântica observável** (ADR-0033). Preserva testes
de parsing existentes.

---

## Clusters propostos

Com base na análise prévia (Passo 96 recolha de dados), a
decomposição proposta:

```
01_core/src/rules/parse.rs (antes: 2255 linhas)
    ↓ transforma-se em:
01_core/src/rules/parse/
    mod.rs          — pub fn parse, parse_code, parse_math, estado
                      do parser (Parser struct, lookahead)
    markup.rs       — markup, markup_expr, strong, emph, heading,
                      list, enum, raw, link, ref, label
    code.rs         — code, block, expression, statement, literal
    math.rs         — math, math_expr, math_attach, math_frac,
                      math_delimited
    rules.rs        — let_binding, set_rule, show_rule, if/while/for
    patterns.rs     — padrões de destructuring (Tuple/Array/Binding),
                      se forem suficientes para ficheiro próprio
```

Ajustes durante execução permitidos (política Passo 96.1):
cluster muito pequeno absorve outro; cluster > 800 linhas
subdivide.

---

## Fase 0 — Preparação

### 0.1 — Verificação do estado actual

```bash
# Tamanho actual:
wc -l 01_core/src/rules/parse.rs

# Funções top-level:
grep -n "^pub fn\|^fn\|^pub struct\|^struct\|^impl" \
    01_core/src/rules/parse.rs

# Testes no ficheiro:
grep -c "^\s*#\[test\]" 01_core/src/rules/parse.rs

# Dependências externas (crates e outros módulos do projecto):
grep -n "^use " 01_core/src/rules/parse.rs | head -30
```

Reportar:
- Linhas totais confirmadas (esperado 2255).
- Número de funções top-level (estimado 40–60).
- Número de testes e sua organização (único `mod tests` ou
  espalhados).
- Estado (struct `Parser` existe? É usado por todas as funções?).

### 0.2 — Criar directório e `mod.rs`

```bash
mkdir -p 01_core/src/rules/parse
git mv 01_core/src/rules/parse.rs 01_core/src/rules/parse/mod.rs
```

Preserva história. Compila sem alterações de código.

Verificar:
```bash
cargo check --package typst-core 2>&1 | tail -10
```

---

## Fase 1 — Extracções por cluster

### Ordem recomendada (mesmo critério do 96.1)

1. **math** — isolado, funções com nomes consistentes `math_*`.
2. **patterns** — isolado, pode revelar-se pequeno e ser
   absorvido por `code.rs`.
3. **rules** — bem contido, `set_rule`, `show_rule`,
   `let_binding`, controlo de fluxo.
4. **markup** — médio (muitas funções).
5. **code** — mais trabalhoso, interage com markup via `block`.

### Procedimento por cluster

Idêntico ao Passo 96.1:

1. Identificar funções do cluster.
2. Criar `parse/<cluster>.rs` com cabeçalho ADR-0037.
3. Mover funções (decidir visibilidade: `pub(super)` vs
   `pub(crate)`).
4. Declarar `mod <cluster>;` em `parse/mod.rs`.
5. Actualizar chamadas (provavelmente `super::X::func()` do
   submódulo; ou paths directos dentro do mesmo submódulo).
6. Mover testes específicos do cluster.
7. Verificar: `cargo check` + `cargo test`. Se falhar,
   rollback do cluster.

### Atenção a estado partilhado

Se `parse.rs` tem struct `Parser` com métodos chamados por
todas as funções:

- **Opção A**: deixar `Parser` em `mod.rs`, submódulos recebem
  `&mut Parser` como parâmetro.
- **Opção B**: mover `Parser` para `parse/parser.rs` (novo
  submódulo dedicado).

A decisão depende do tamanho do `impl Parser`. Se for pequeno
(< 200 linhas), fica em `mod.rs`. Se for grande (>> 300 linhas),
ganha ficheiro próprio.

### Armos inline no dispatcher (se existirem)

Se `parse` ou `markup_expr` têm `match` sobre `SyntaxKind` ou
token types, aplicar Regra 4 revista:
- Armos triviais (1–3 linhas) inline.
- Armos longos delegam para funções de submódulos.

---

## Fase 2 — Verificação final

### 2.1 — Tamanhos

```bash
wc -l 01_core/src/rules/parse/*.rs | sort -rn
```

Alvo: nenhum ficheiro > 800 linhas. Se algum ficar perto (ex:
700+), não é problema — a Regra 2 é orientativa.

### 2.2 — Testes e linter

```bash
cargo test --workspace 2>&1 | tail -10
cargo run --package crystalline-lint 2>&1 | tail -5
```

Esperado: 746 L1 + 174 L3 + 6 ignorados. Zero violations.

### 2.3 — Testes específicos de parsing

```bash
cargo test --package typst-core parse 2>&1 | tail -10
cargo test --package typst-core lex 2>&1 | tail -10
```

Todos devem passar (ADR-0033).

---

## Fase 3 — Actualizar DEBT-46

Marcar o quarto checkbox:

```markdown
- [x] `parse.rs` reestruturado por tipo de nó (markup, code,
      math, rules). Passo 96.4. **Concluído** — N submódulos
      criados, todos abaixo de 800 linhas. Tamanho final do
      `mod.rs`: X linhas.
```

O DEBT-46 não fecha (5 checkboxes restantes).

---

## Critérios de conclusão

- [ ] Directório `01_core/src/rules/parse/` criado com
      `mod.rs` e submódulos.
- [ ] `01_core/src/rules/parse.rs` já não existe como ficheiro
      top-level.
- [ ] 4–5 submódulos criados (ajustes documentados se
      diferiu do proposto).
- [ ] Nenhum submódulo > 800 linhas sem excepção Regra 6.
- [ ] Testes de parsing passam sem alteração.
- [ ] `cargo test --workspace` inalterado (746 L1 + 174 L3 + 6
      ignorados).
- [ ] `crystalline-lint` → zero violations.
- [ ] Nenhum `unsafe` novo.
- [ ] DEBT-46 com quarto checkbox marcado.
- [ ] Nenhum ADR alterado.

---

## Ao terminar, reportar

Fase 0:
- Funções top-level do `parse.rs`.
- Estado do `Parser` struct (pequeno, grande, já tinha ficheiro
  próprio).
- Número de testes.

Fase 1 (por cluster):
- Cluster extraído, tamanho do submódulo, tamanho remanescente
  do `mod.rs`.
- Ajustes de cluster aplicados.

Fase 2:
- Tamanhos finais de todos os ficheiros em `parse/`.
- Testes verdes, zero violations.

Fase 3:
- Confirmação DEBT-46 actualizado.

Observações para a aplicação contínua da ADR-0037:
- Alguma das 7 regras foi difícil de aplicar neste passo?
- Os 4 ajustes da Passo 96.3 foram suficientes ou revelou-se
  necessidade de mais?

Go/No-Go para Passo 96.5:
- **Go incondicional** se a reestruturação foi limpa. Passo
  96.5 = `stdlib.rs` (1711 linhas), divisão por módulo da
  stdlib.
- **Go com nota** se houve fricções menores. Registar para
  eventual revisão futura da ADR-0037 (não bloqueia 96.5).
- **No-Go** se a reestruturação comprometeu testes ou revelou
  acoplamento intratável. Reverter e reavaliar.
