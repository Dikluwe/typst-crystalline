# Prompt L0 — `compiler/eval/show_rule_termination` — loop α e terminação de show rules
Hash do Código: 833f2abb

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/show_rule_termination.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval.md` (dono de `compiler/eval/rules.rs`)
**ADRs relevantes**: ADR-0107 (paridade língua), ADR-0108 (medir antes de decidir), ADR-0033 (mensagem de profundidade byte-idêntica ao vanilla)

---

## Contexto

Este nó contém o **loop α** de terminação de show rules para um único nó: dado
um `Content` e uma função que aplica uma única regra, aplica repetidamente até
que o nó estabilize num ponto-fixo morfológico, detete um ciclo, ou atinja o
limite de profundidade. Extraído de `compiler/eval/rules.rs` no Passo 1009
conforme ADR-0109 (atomização — forma B, free function no arquivo da unidade).

A função pública deste nó é chamada pelo hub `rules.rs` dentro do `apply_node`
de cada nó durante a travessia `map_content`; ela não contém lógica de parsing
de `#show`/`#set`, nem matching de selectores, nem realização de parágrafos —
apenas a estratégia de terminação.

---

## Instrução

### 1. Contrato público

```rust
pub(crate) type ShowRuleLoopResult = (Content, bool);

pub(crate) fn run_show_rule_loop<F>(
    initial: Content,
    mut apply_one_step: F,
    max_depth: usize,
    full_error: bool,
) -> SourceResult<ShowRuleLoopResult>
where
    F: FnMut(&Content) -> SourceResult<Option<(Content, RuleId)>>;
```

- Recebe o conteúdo inicial (um nó), uma closure que tenta aplicar **uma**
  regra ao nó actual e devolve `Some((novo_nó, rule_id))` ou `None` se
  nenhuma regra casa, o limite máximo de profundidade, e a flag `full_error`.
- Retorna `Ok((content, applied))` onde `applied` indica se pelo menos uma
  regra foi aplicada.
- Retorna `Err(SourceDiagnostic::...)` se atingir o limite de profundidade ou
  detetar um ciclo.
- Não sabe nada de `Engine`, `EvalContext`, `ShowRule` ou `Selector`: é estratégia
  de terminação pura. Isso evita import reverso do nó para o hub `rules.rs`.

### 2. Algoritmo de terminação

Usar sistema de reescrita de termos com histórico de formas canónicas +
transições, em vez de tecto cego de profundidade.

```rust
let mut seen: Vec<(Content, usize)> = Vec::new();
let mut transitions: Vec<Vec<RuleId>> = Vec::new();
let mut cycle = false;
let mut history: Vec<Content> = if full_error { Vec::new() } else { Vec::new() };
let mut any_applied = false;

let mut work = initial.clone();
let mut canon = work.morph_canon();
seen.push((canon.clone(), 0));

for step in 1..=max_depth {
    let applied = match apply_one_step(&work)? {
        Some((new_work, id)) => {
            any_applied = true;
            work = new_work;
            vec![id]
        }
        None => {
            return Ok((work, any_applied));              // Desfecho 1 — ponto-fixo local
        }
    };
    transitions.push(applied);

    let new_canon = work.morph_canon();

    // Desfecho 1b — ponto-fixo morfológico.
    if new_canon == canon {
        return Ok((work, any_applied));
    }

    // P350c (sob flag): registo para classificação do hint extra no Desfecho 3.
    if full_error {
        if history.iter().any(|h| *h == new_canon) {
            cycle = true;
        }
        history.push(new_canon.clone());
    }

    // Desfecho 2 — ciclo detectado por forma canónica repetida.
    if let Some(&start) = seen.iter().find_map(|(c, s)| if *c == new_canon { Some(s) } else { None }) {
        let cycle_rules: Vec<RuleId> =
            transitions[start..step].iter().flatten().copied().collect();
        return Err(vec![show_rule_cycle_error(start, step, cycle_rules)]);
    }

    seen.push((new_canon.clone(), step));
    canon = new_canon;
}

// Desfecho 3 — limite de profundidade (backstop).
Err(vec![show_rule_depth_exceeded_error(cycle, full_error)])
```

A closure `apply_one_step` é fornecida pelo hub `rules.rs`. Ela:

- Recebe o nó actual;
- Itera as regras aplicáveis em ordem innermost-first;
- Ignora regras cujo `RuleId` esteja em `engine.active_guards`;
- Aplica no máximo uma transformação func/content por chamada;
- Devolve `Some((produced, rule.id))` ou `None`.

### 3. Gatilho de reabertura

Se `apply_show_rules` alguma vez correr com `in_context == true`, esta decisão
tem de ser revista. Actualmente (Passo 1007, confirmado por teste empírico),
`counter.get()`/`state.get()` estão bloqueados fora de `context` durante este
loop, e mutações (`counter.step()`/`state.update()`) produzem
`Content::CounterUpdate`/`StateUpdate` preservados por `morph_canon`. Logo qualquer
efeito de estado que varie entre iterações aparece na própria forma comparada,
o que justifica terminação antecipada.

### 4. Formato das mensagens de erro

- **Desfecho 3 — limite de profundidade**: manter a mensagem primária
  byte-idêntica ao vanilla (`typst-realize/src/lib.rs`):
  ```text
  maximum show rule depth exceeded
  ```
  com os dois hints:
  - `maybe a show rule matches its own output`
  - `maybe there are too deeply nested elements`
  A flag `full_error` acrescenta um terceiro hint classificatório
  (cíclico vs não-convergente), mantendo a mensagem base inalterada.

- **Desfecho 2 — ciclo**: nova capacidade sem equivalente vanilla. Usar mensagem
  distinta própria:
  ```text
  show rule cycle detected
  ```
  seguida de hint que nomeia as regras envolvidas no ciclo (a partir de
  `cycle_rules`). Não tentar imitar a mensagem de profundidade porque o
  observável é diferente: o loop estabeleceu um ciclo antes de atingir o limite.

### 5. Flag `full_error`

A flag `full_error` é passada como parâmetro ao loop. Controla apenas o hint
classificatório extra no Desfecho 3 e o registo da `history`. A detecção de
ponto-fixo (Desfecho 1) e ciclo (Desfecho 2) são comportamento base,
independentes da flag. Com a flag desligada, o caminho quente continua a alocar
o mínimo possível: `Vec` só sob condições de não-convergência.

---

## Critérios de verificação

```
Dado cadeia que estabiliza em N<64 passos → Ok no passo N (guarda: 18 testes P340)
Dado cadeia que nunca repete nem estabiliza em 64 → Err, mensagem byte-idêntica (Desfecho 3)
Dado par de regras A/B alternando → Err(ShowRuleCycle) nomeando A e B, antes de 64 (Desfecho 2, caso novo)
Dado Passo 1007 Teste 1 (counter.step() que re-casa) → Ok, guarda de não-regressão
Dado Passo 1007 Teste 2 (counter.step() fora do heading) → Ok, guarda de não-regressão
```

Aplicação final: `cargo build && crystalline-lint .` — zero violations.
