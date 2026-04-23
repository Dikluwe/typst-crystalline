# Passo 94 — Extrair `styles` do `EvalContext`, segundo pagamento da ADR-0036

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0036-*.md` — princípio de atomização
  progressiva. Este passo é a segunda aplicação concreta
  (primeira foi o Passo 92 com `Route<'a>`).
- `00_nucleo/adr/typst-adr-0033-*.md` — paridade funcional com
  vanilla.
- `00_nucleo/diagnosticos/diagnostico-constraint-tracked-recursivo-passo-93.md`
  — padrão técnico descoberto no Passo 92/93, pode ser relevante
  se `StyleChain` for `Tracked`.
- `01_core/src/entities/style_chain.rs` — estrutura actual do
  `StyleChain`. Imutável, clone O(1) via `Arc`.
- `01_core/src/rules/eval.rs` — `EvalContext` com campo
  `styles: StyleChain`. Save/restore por bloco implementado no
  Passo 33.
- `lab/typst-original/` — como o vanilla propaga `StyleChain<'a>`.

Pré-condição: `cargo test` — 746 L1 + 174 L3 + 6 ignorados, zero
violations. Passo 93 concluído (DEBT-45 parcialmente pago).

---

## Natureza deste passo

Passo único de construção. Remove o campo `styles` do
`EvalContext` e propaga-o como parâmetro explícito nas funções
`eval_*` que o usam.

É o segundo pagamento concreto da ADR-0036. Segue o mesmo padrão
do Passo 92 (extracção do `route`), mas com diferenças
importantes:

1. **`StyleChain` não é `Tracked<T>`** (diferente de `Route<'a>`).
   É `Clone` com `Arc` interno — propagação por referência
   `&StyleChain` é suficiente, não requer `Tracked`.

2. **Save/restore por bloco é padrão existente** (Passo 33).
   O mecanismo actual é `let saved = ctx.styles.clone();
   ... ctx.styles = saved;`. Na forma atomizada, este padrão
   desaparece — o chamador cria um novo `StyleChain` via `push`
   para o sub-escopo; o original mantém-se intacto.

3. **Mutação intencional via `#set`** deve ser preservada.
   Quando `#set text(bold: true)` aparece dentro de um bloco,
   o `StyleChain` local muda. Mas essa mudança é **local ao
   bloco** — não se propaga para fora. É exactamente o que a
   forma atomizada facilita.

---

## Decisões formalizadas neste passo

- ADR-0036 (atomização progressiva) — segunda aplicação concreta.
- ADR-0033 (paridade funcional) — comportamento de `#set` e
  scoping preservado sem alteração.
- Divergência residual permitida e registada: se o vanilla
  propagar `StyleChain` de forma diferente (ex: `Tracked` ou
  field de `Engine`), avaliar e documentar.

---

## Verificação inicial (bloqueante)

Antes de iniciar, executar:

```bash
# Ver o campo styles no EvalContext:
grep -n "pub styles:\|styles: StyleChain\|self\.styles\b\|ctx\.styles\b" \
    01_core/src/rules/eval.rs

# Padrões actuais de save/restore:
grep -n "saved_styles\|styles = saved\|styles.clone()" \
    01_core/src/rules/eval.rs

# Todas as funções que actualmente lêem ou mutam ctx.styles:
grep -n "self\.styles\b\|ctx\.styles\b" 01_core/src/rules/ \
    --include="*.rs" -r | head -40
```

Reportar:
- **Contagem de usos de `ctx.styles`** ou `self.styles` — isto
  dimensiona o refactor.
- **Funções que mutam vs. só lêem** — só leitura passa por `&`,
  mutação requer decisão (ver Estratégia abaixo).
- **Se há save/restore em função Tier 3** — funções que não
  mutam mas que criam escopo mesmo assim (podem ser
  simplificadas).

Se a contagem de usos for **> 25**, parar e reportar antes de
prosseguir — pode exigir divisão em sub-passos por submódulo,
seguindo o padrão de contingência aberto no enunciado do
Passo 92.

---

## Estratégia de propagação

`StyleChain` tem dois padrões de uso observados:

### Padrão A — Leitura (maioria)

Funções que consultam o estilo activo para construir
`Content::Text` ou tomar decisões:

```rust
// Antes:
let bold = ctx.styles.bold();

// Depois:
fn eval_algo(&mut self, styles: &StyleChain, ...) -> ... {
    let bold = styles.bold();
}
```

Propagação simples: `&StyleChain` como parâmetro.

### Padrão B — Mutação local (bloco ou `#set`)

Funções que entram em novo escopo e podem adicionar delta:

```rust
// Antes (com save/restore):
let saved = ctx.styles.clone();
if let Some(delta) = extract_set_delta(...) {
    ctx.styles = ctx.styles.push(delta);
}
eval_block_body(ctx, ...);
ctx.styles = saved;

// Depois (forma atomizada):
fn eval_block(&mut self, styles: &StyleChain, block: &Block) -> ... {
    let local_styles = match extract_set_delta(block) {
        Some(delta) => styles.push(delta),
        None => styles.clone(),  // ou &styles propagado
    };
    eval_block_body(self, &local_styles, block)
}
```

O save/restore desaparece porque `styles` no chamador **nunca
foi mutado** — o novo `local_styles` é variável local ao bloco.

### Padrão C — Raiz

Ponto de entrada público (`pub fn eval(...)`) cria o
`StyleChain` inicial:

```rust
pub fn eval(world, source) -> SourceResult<Module> {
    let styles = StyleChain::default_chain();
    let route = Route::root().with_id(source.id());
    eval_markup(&mut ctx, route.track(), &styles, source.root())
}
```

---

## Sequência de tarefas

### Tarefa 1 — Mapear funções por tier

Classificação semelhante ao Passo 92:

- **Tier 1** — funções que mutam `ctx.styles` (`Expr::Code`,
  `Expr::Content`, `Expr::Closure` body, `#set` rule
  processing). Precisam de aplicar Padrão B.
- **Tier 2** — funções que só lêem `ctx.styles`. Aplicam
  Padrão A.
- **Tier 3** — funções intermédias que chamam Tier 1 ou 2.
  Propagam o parâmetro mecanicamente.
- **Tier 4** — funções folha que nunca tocam em styles.
  Não mudam.

Relatar contagem por tier antes de prosseguir para Tarefa 2.

### Tarefa 2 — Refactor de assinaturas

Adicionar `styles: &StyleChain` como parâmetro aos tiers 1, 2, 3.

**Posição do parâmetro**: imediatamente após `route`, se presente:

```rust
fn eval_expr(
    &mut self,
    route: Tracked<'r, Route<'r>>,
    styles: &StyleChain,
    expr: &Expr,
) -> SourceResult<Value>
```

**Razão da ordem**: convenção de "parâmetros de contexto
propagado" antes do parâmetro semântico (`expr`, `markup`, etc.).
Mantém consistência com o padrão do Passo 92.

### Tarefa 3 — Aplicar Padrão B em sítios de mutação

Para cada função Tier 1 (blocos, closures, `#set`):

1. Remover `let saved = ctx.styles.clone()` e `ctx.styles = saved`.
2. Construir `local_styles` com `styles.push(delta)` ou equivalente.
3. Propagar `&local_styles` às chamadas dentro do bloco.

Verificar cuidadosamente que o comportamento observável é
idêntico — o teste L1 `set_dentro_bloco_nao_vaza_para_fora`
(Passo 33) deve continuar a passar sem alteração.

### Tarefa 4 — Remover campo `styles` de `EvalContext`

Depois do refactor compilar e todos os testes passarem:

```rust
// Antes:
pub struct EvalContext<'w> {
    // ...
    pub styles: StyleChain,
    // ...
}

// Depois:
pub struct EvalContext<'w> {
    // ... (sem styles)
}
```

Construtor `EvalContext::new` deixa de receber ou inicializar
o campo. O `StyleChain` inicial é criado no ponto de entrada
`pub fn eval(...)` e passado como parâmetro.

### Tarefa 5 — Actualizar ponto de entrada

```rust
pub fn eval(world, source) -> SourceResult<Module> {
    let initial_styles = StyleChain::default_chain();
    let route = Route::root().with_id(source.id());
    let mut ctx = EvalContext::new(world, source);
    eval_markup(&mut ctx, route.track(), &initial_styles, source.root())
}
```

### Tarefa 6 — Testes

Verificação:

```bash
# Todos os testes do StyleChain/#set continuam a passar:
cargo test --package typst-core styles 2>&1 | tail -20
cargo test --package typst-core set_ 2>&1 | tail -20

# Teste de atomização: nenhum ctx.styles remanescente:
grep -n "ctx\.styles\|self\.styles\b" 01_core/src/rules/eval.rs

# Verificação final:
cargo test --workspace 2>&1 | tail -10
cargo run --package crystalline-lint 2>&1 | tail -5
```

Esperado:
- 746 L1 + 174 L3 + 6 ignorados (contagem inalterada; não há
  novos testes obrigatórios, nem testes removidos — é refactor
  puro).
- `ctx.styles` e `self.styles` com zero ocorrências.
- Zero violations.

### Tarefa 7 — Actualizar DEBT-1 (se aplicável)

O DEBT-1 (`StyleChain`) tem várias pendências listadas. Uma
delas — implicitamente resolvida pela atomização — é "`#set` é
global ao eval (não tem scoping por bloco)". Mas essa pendência
já tinha sido marcada como resolvida no Passo 33 (via
save/restore).

Agora, com a atomização, o mecanismo é **mais natural**: o
`StyleChain` local a cada bloco é intrinsecamente isolado. A
pendência "wrappers Content::Strong/Emph do layout" provavelmente
continua aberta (separada do scoping).

**Decisão no reporte**: verificar o DEBT-1 após o Passo 94 e
reportar:
- Se alguma pendência foi implicitamente resolvida pela
  atomização (candidato a marcar como resolvido).
- Se alguma pendência deve ser repensada à luz da nova
  arquitectura.

Este passo **não altera** o DEBT-1 — a decisão sobre actualização
fica para sub-passo de governança separado se for necessário.

---

## Critérios de conclusão

- [ ] Campo `styles: StyleChain` removido de `EvalContext`.
- [ ] Parâmetro `styles: &StyleChain` adicionado a funções
      `eval_*` que precisam (tiers 1, 2, 3 da Tarefa 1).
- [ ] Save/restore manual de `ctx.styles` eliminado (Padrão B
      aplicado em todos os Tier 1).
- [ ] Ponto de entrada `pub fn eval(...)` cria
      `StyleChain::default_chain()` e propaga.
- [ ] Testes `set_dentro_bloco_*` e `set_false_reverte_*` passam
      sem alteração.
- [ ] `grep "ctx\.styles\|self\.styles"` em `01_core/src/rules/`
      retorna zero resultados.
- [ ] Contagem total: 746 L1 + 174 L3 + 6 ignorados (inalterada).
- [ ] `crystalline-lint` → zero violations.
- [ ] Nenhum `unsafe` novo introduzido.
- [ ] Nenhum ADR alterado.
- [ ] DEBT-1 observado e reportado, mas não alterado neste passo.

---

## Ao terminar, reportar

Tarefa 1 (mapeamento):
- Contagem de funções por tier (1, 2, 3, 4).
- Se total de funções alteradas excede 25, razão para ter
  continuado apesar da recomendação.

Tarefa 2 (refactor):
- Número total de funções com parâmetro `styles: &StyleChain`
  adicionado.
- Linhas alteradas em `eval.rs` (diff size).

Tarefa 3 (Padrão B):
- Funções onde save/restore foi eliminado.
- Confirmação de que o teste `set_dentro_bloco_nao_vaza_para_fora`
  passa sem modificação.

Tarefa 4 (remover campo):
- Confirmação de campo eliminado.

Tarefa 5 (entrada):
- Forma final do `pub fn eval`.

Tarefa 6 (verificação):
- Contagem final de testes.
- Zero violations.
- Zero `ctx.styles` remanescente.

Tarefa 7 (DEBT-1):
- Observações sobre pendências que podem estar implicitamente
  resolvidas pela atomização.

Go/No-Go para Passo 95:
- **Go** se a atomização foi aplicada limpamente. Passo 95 a
  decidir em conversa. Candidatos:
  - Continuar extracção de campos do `EvalContext` (terceiro
    pagamento da ADR-0036): `show_rules`,
    `figure_numbering`, outros.
  - Materializar `Style` + `LazyHash` (folhas de `Styles`, para
    avançar a cadeia de stubs comemo).
  - Materializar `Introspection` (desbloqueia `Sink`).
  - Actualização do DEBT-1 à luz das descobertas do Passo 94.
- **No-Go parcial** se o mapeamento da Tarefa 1 revelou escopo
  muito maior que o esperado. Opções:
  - Dividir em sub-passos por submódulo (cada um paga parte
    do DEBT agregador).
  - Reverter se a atomização introduzir problemas não
    antecipados que afectam a correcção.
