# Passo 95 — Limpeza residual + extracção de `show_rules` do `EvalContext`

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0036-*.md` — princípio de atomização
  progressiva. Este passo é a terceira aplicação concreta.
- `00_nucleo/adr/typst-adr-0033-*.md` — paridade funcional com
  vanilla.
- `00_nucleo/DEBT.md` — entrada DEBT-1 (pendências históricas
  do `StyleChain`) e DEBT-39 (`active_guards`).
- `01_core/src/rules/eval.rs` — `EvalContext` com campos
  `show_rules: Arc<[ShowRule]>` e `active_guards: Vec<RuleId>`.
- `lab/typst-original/` — como o vanilla propaga show rules e
  guards.

Pré-condição: `cargo test` — 746 L1 + 174 L3 + 6 ignorados,
zero violations. Passo 94 concluído (`styles` extraído do
contexto).

---

## Natureza deste passo

Passo único com três tarefas:

1. **Tarefa A** — Limpeza residual do Passo 94: executar Tarefa 7
   omitida (revisar DEBT-1 à luz da atomização) e limpar
   comentários residuais que mencionam `ctx.styles`.

2. **Tarefa B** — Extracção de `show_rules` do `EvalContext`:
   terceiro pagamento da ADR-0036, análogo a `route` (Passo 92)
   e `styles` (Passo 94).

3. **Tarefa C** — Avaliação de `active_guards` no mesmo contexto:
   decidir se entra nesta atomização ou fica separado. O DEBT-39
   contém a análise técnica — este passo pode fechá-lo, deixá-lo
   aberto, ou registar nova decisão.

As três tarefas são complementares: a limpeza (A) fecha trabalho
do passo anterior; a extracção (B) é o trabalho principal; a
avaliação (C) decide se o trabalho relacionado vem junto ou em
passo separado.

---

## Decisões formalizadas neste passo

- ADR-0036 (atomização progressiva) — terceira aplicação concreta.
- ADR-0033 (paridade funcional) — comportamento de show rules
  preservado.
- Eventual decisão sobre DEBT-39 (se Tarefa C resultar em acção
  ou em nova nota).

---

## Tarefa A — Limpeza residual do Passo 94

### A.1 — Comentários residuais

```bash
# Procurar menções a ctx.styles/self.styles em comentários:
grep -n "ctx\.styles\|self\.styles\|ctx\.styles" \
    01_core/src/rules/eval.rs
```

Esperado: 2 ocorrências em comentários (conforme reporte do
Passo 94).

Para cada ocorrência:
- Se o comentário descreve comportamento histórico mas continua
  útil, adaptar para a forma actual (ex: `ctx.styles` → `styles`
  parâmetro, ou reescrever a frase).
- Se o comentário é puro histórico sem valor documental, remover.

### A.2 — Revisar DEBT-1

Ler a entrada DEBT-1 no `00_nucleo/DEBT.md`. A entrada tem
pendências listadas — algumas delas podem estar implicitamente
resolvidas pela atomização do Passo 94.

Classificar cada pendência em:

- **Resolvida implicitamente** — a pendência desapareceu como
  consequência natural da atomização. Exemplo: "`#set` é global
  ao eval (não tem scoping por bloco)" — era preocupação quando
  `styles` vivia no contexto; agora o `styles` é parâmetro
  e o scoping é natural.

- **Continua em aberto** — a pendência é ortogonal à atomização.
  Exemplo: "propriedades adicionais (fill, font-family, weight
  numérico)" — é trabalho de materialização de novos campos no
  `StyleChain`, não questão arquitectural.

- **Mudou de natureza** — a pendência ainda existe mas a forma
  mudou. Exemplo: "remover wrappers `Content::Strong/Emph` do
  layout" — pode agora ser mais fácil ou mais difícil consoante
  como a atomização afectou o layout.

Actualizar a entrada DEBT-1:
- Marcar resolvidas implicitamente com `[x]` e nota "resolvida
  pelo Passo 94 (atomização de `styles`)".
- Manter as em aberto com `[ ]`.
- Reescrever as que mudaram de natureza.

Se todas as pendências ficarem resolvidas, mover DEBT-1 para
Secção 2. Se restarem pendências legítimas, manter na Secção 1.

### A.3 — Verificação da Tarefa A

```bash
# Zero menções residuais a ctx.styles / self.styles:
grep -n "ctx\.styles\|self\.styles" 01_core/src/rules/ -r \
    --include="*.rs"

# DEBT-1 observável:
grep -n "^## DEBT-1\b" 00_nucleo/DEBT.md
```

Esperado: zero resultados do primeiro comando; DEBT-1 continua a
existir (na Secção 1 ou 2 conforme decidido).

---

## Tarefa B — Extracção de `show_rules` do `EvalContext`

### B.1 — Verificação inicial

```bash
# Estrutura actual do show_rules:
grep -B 2 -A 3 "pub show_rules:\|show_rules: Arc" \
    01_core/src/rules/eval.rs

# Usos de ctx.show_rules:
grep -n "ctx\.show_rules\|self\.show_rules\|&self\.show_rules" \
    01_core/src/rules/eval.rs

# Métodos helper existentes:
grep -n "push_show_rule\|truncate_show_rules" \
    01_core/src/rules/eval.rs
```

Reportar:
- Número de usos de `show_rules`.
- Funções que actualmente mutam (`push_show_rule`,
  `truncate_show_rules`) vs. funções que só lêem.
- Se há save/restore do `show_rules` equivalente ao que havia
  para `styles` antes do Passo 94.

Se número de usos for **> 15**, parar e reportar antes de
prosseguir.

### B.2 — Estratégia de propagação

`show_rules` tem característica importante: é `Arc<[ShowRule]>` —
**já é compartilhável eficientemente** (clone O(1) via `Arc`).
Isto simplifica a atomização.

Padrão similar ao Passo 94 (Padrão B para mutação):

```rust
// Antes (campo no contexto):
ctx.push_show_rule(rule);  // muta ctx.show_rules
eval_block_body(ctx, ...);
ctx.truncate_show_rules(saved_len);

// Depois (parâmetro):
fn eval_block(&mut self, show_rules: &mut Arc<[ShowRule]>, ...) {
    let local_rules = {
        let mut rules = Arc::from_iter(show_rules.iter().cloned());
        rules_mut.push(rule);
        Arc::from(rules)
    };
    eval_block_body(self, &local_rules, ...)
}
```

**Observação**: diferente de `StyleChain` (que tem método
`.push()` que retorna nova chain), `Arc<[T]>` não tem push
directo — tem de reconstruir. Mas isto é trabalho que já
acontecia via `push_show_rule`; agora acontece localmente.

### B.3 — Decisão sobre tipo do parâmetro

O Passo 94 escolheu `&mut StyleChain` com justificação sobre
`#set` semantics. `show_rules` tem caso análogo?

**Análise**:

- `#show` aparece num bloco e afecta os irmãos subsequentes
  (semântica similar a `#set`). Isto sugere `&mut Arc<[ShowRule]>`
  análogo a `&mut StyleChain`.

- Alternativa: `&Arc<[ShowRule]>` (imutável) e cada `#show`
  cria novo `Arc` local ao bloco. Mais puro, mas exige que o
  caller propague.

**Decisão recomendada**: seguir o padrão do Passo 94 por
consistência — `&mut Arc<[ShowRule]>`. Se o diagnóstico da B.1
revelar que `show_rules` é puramente push-once (sem semântica de
"afecta irmãos"), reavaliar.

### B.4 — Classificar funções por tier

- **Tier 1** — funções que mutam `show_rules` (provavelmente
  `Expr::ShowRule`, blocos com save/restore análogo ao styles).
- **Tier 2** — funções que lêem (chamam `intercept_content` ou
  `apply_show_rules`).
- **Tier 3** — propagação.
- **Tier 4** — intocadas.

### B.5 — Refactor de assinaturas

Adicionar `show_rules: &mut Arc<[ShowRule]>` (ou `&Arc<[ShowRule]>`
consoante B.3) como parâmetro nas funções dos tiers 1, 2, 3.

**Ordem do parâmetro**: após `styles`. Mantém consistência:

```rust
fn eval_expr(
    &mut self,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    expr: &Expr,
) -> SourceResult<Value>
```

### B.6 — Remover campo de `EvalContext`

```rust
// Antes:
pub struct EvalContext<'w> {
    // ...
    pub show_rules: Arc<[ShowRule]>,
    // ...
}

// Depois:
pub struct EvalContext<'w> {
    // ... (sem show_rules)
}
```

Remover também os helpers `push_show_rule` e
`truncate_show_rules` — deixaram de ser métodos do `EvalContext`;
as operações são locais a cada função.

### B.7 — Ponto de entrada

```rust
pub fn eval(world, source) -> SourceResult<Module> {
    let initial_styles = StyleChain::default_chain();
    let mut initial_show_rules: Arc<[ShowRule]> = Arc::from([]);
    let route = Route::root().with_id(source.id());
    let mut ctx = EvalContext::new(world, source);
    eval_markup(
        &mut ctx,
        route.track(),
        &mut initial_styles,  // ajustar conforme assinatura actual
        &mut initial_show_rules,
        source.root(),
    )
}
```

### B.8 — Testes

Verificar:

```bash
# Testes de show rules continuam a passar:
cargo test --package typst-core show_ 2>&1 | tail -15

# Zero ctx.show_rules remanescente:
grep -n "ctx\.show_rules\|self\.show_rules" 01_core/src/rules/ \
    -r --include="*.rs"

# Verificação final:
cargo test --workspace 2>&1 | tail -10
cargo run --package crystalline-lint 2>&1 | tail -5
```

Esperado:
- 746 L1 + 174 L3 + 6 ignorados (inalterado).
- Zero `ctx.show_rules` remanescente.
- Zero violations.

---

## Tarefa C — Avaliar `active_guards`

### C.1 — Ler DEBT-39

Rever a entrada DEBT-39 no `00_nucleo/DEBT.md`. O DEBT foi aberto
no Passo 84.4 com justificação técnica: `active_guards` tem
padrão push/pop frequente (sem clone), e migrar para `Arc<[T]>`
regrediria performance.

### C.2 — Decisão à luz da ADR-0036

A ADR-0036 aplica-se ao `active_guards`? Duas leituras:

**Leitura 1 — Aplica-se**. `active_guards` é campo do
`EvalContext` → dívida arquitectural. Extrair como parâmetro
`&mut Vec<RuleId>` segue o princípio.

**Leitura 2 — Não aplica-se**. A ADR-0036 Regra 4 ("excepções
permitidas") reconhece que "estado genuinamente global à
avaliação pode permanecer no contexto". O `active_guards`
pode encaixar aqui — é colector de estado activo durante
recursão de show rules.

### C.3 — Opções

**Opção 1 — Extrair `active_guards` também neste passo**:

Propagar `&mut Vec<RuleId>` como parâmetro junto com `show_rules`.
Fecha DEBT-39 e aplica ADR-0036 consistentemente.

Custo: aumenta escopo do Passo 95; adiciona parâmetro à mesma
lista de funções que já é propagada.

**Opção 2 — Deixar `active_guards` no contexto, fechar DEBT-39**:

Decidir que `active_guards` é excepção legítima (Regra 4 da
ADR-0036) e documentar. Fecha DEBT-39 com marca de resolução
"não aplicável — excepção legítima da ADR-0036".

Custo: requer argumentação clara no DEBT sobre porque é
excepção.

**Opção 3 — Deixar DEBT-39 em aberto**:

Adiar decisão. DEBT-39 continua a registar "não migrado; aguarda
contexto adicional".

Recomendação: **Opção 1**. Razão: se o `EvalContext` está a ser
desmantelado progressivamente, `active_guards` é coerente com o
esforço. A Regra 4 da ADR-0036 é escape para casos onde a
extracção traz custo real (performance mensurável, complexidade
de propagação); `active_guards` não tem esse perfil — o custo
de propagação é pequeno e a performance de `Vec` vs. `&mut Vec`
é equivalente.

A decisão final fica para ti após teres a informação da Tarefa B.

### C.4 — Execução consoante opção

**Se Opção 1**: adicionar `active_guards: &mut Vec<RuleId>` como
parâmetro junto com `show_rules`. Fechar DEBT-39 (mover para
Secção 2).

**Se Opção 2**: escrever nota em DEBT-39 identificando Regra 4 da
ADR-0036, mover DEBT-39 para Secção 2 como "resolvido por
decisão arquitectural".

**Se Opção 3**: não tocar em DEBT-39.

---

## Critérios de conclusão

**Tarefa A (limpeza)**:
- [ ] Comentários residuais sobre `ctx.styles` ajustados ou
      removidos (2 ocorrências do Passo 94).
- [ ] DEBT-1 revisitado: pendências classificadas (resolvidas/
      abertas/mudaram) e entrada actualizada.

**Tarefa B (show_rules)**:
- [ ] Campo `show_rules: Arc<[ShowRule]>` removido de
      `EvalContext`.
- [ ] Parâmetro adicionado a funções tiers 1, 2, 3.
- [ ] Helpers `push_show_rule` e `truncate_show_rules` removidos.
- [ ] Ponto de entrada `pub fn eval` cria `Arc::from([])` e
      propaga.
- [ ] Testes de show rules continuam a passar.

**Tarefa C (active_guards)**:
- [ ] Decisão registada (Opção 1, 2 ou 3).
- [ ] Se Opção 1 ou 2: DEBT-39 movido para Secção 2 com nota.
- [ ] Se Opção 3: DEBT-39 permanece em Secção 1.

**Geral**:
- [ ] 746 L1 + 174 L3 + 6 ignorados (contagem inalterada).
- [ ] `crystalline-lint` → zero violations.
- [ ] Nenhum `unsafe` novo introduzido.
- [ ] Nenhum ADR alterado.

---

## Ao terminar, reportar

Tarefa A:
- Comentários ajustados ou removidos.
- Classificação das pendências do DEBT-1 (quantas resolvidas,
  quantas em aberto).
- Se DEBT-1 foi movido para Secção 2 ou não.

Tarefa B:
- Contagem total de usos de `show_rules` antes do refactor.
- Número de funções com parâmetro adicionado.
- Decisão sobre `&mut Arc<[ShowRule]>` vs. `&Arc<[ShowRule]>`.
- Linhas alteradas.

Tarefa C:
- Opção escolhida.
- Razão da decisão (especialmente relevante se diferente da
  recomendação).

Geral:
- Contagem final de testes.
- Zero violations.

Go/No-Go para Passo 96:
- **Go** se todas as tarefas concluídas. Passo 96 a decidir.
  Candidatos:
  - Continuar extracção: próximos campos candidatos do
    `EvalContext` (pode sobrar pouca coisa legítima à luz
    da Regra 4).
  - Materializar `Style` + `LazyHash` (folhas de `Styles`).
  - Materializar `Introspection` (desbloqueia `Sink`).
  - Abertura do `Engine<'a>` — momento oportuno dado que o
    `EvalContext` foi desmantelado parcialmente.
- **No-Go parcial** se Tarefa B revelar escopo muito maior do
  que previsto, ou se Tarefa C forçar trabalho adicional
  não antecipado.
