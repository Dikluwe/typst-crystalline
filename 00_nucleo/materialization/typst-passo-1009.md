# Passo 1009 — Fatiamento hub/nó `eval::rules` + mecanismo de paragem revisto como nó

**Tipo**: Aplicação do método (P1002) ao candidato recomendado pelo P1008, combinada com a
materialização do mecanismo de paragem discutido separadamente (revisão cruzada
Claude/Qwen + Passo 1007). **Correcção de processo**: um L0 solto para um mecanismo dentro
de `rules.rs` não pode existir por si — `V15` exige um `@prompt` por ficheiro, e `rules.rs`
já tem um dono hoje. Este passo funde as duas frentes para não implementar código dentro
do ficheiro antigo e ter de o mover outra vez no fatiamento.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1008.

---

## Fase A — Confirmar a situação actual, antes de tudo

1. **Quem é hoje o dono de `rules.rs`?**
   ```
   head -20 01_core/src/compiler/eval/rules.rs   # header de linhagem
   ```
   Confirmar o `@prompt` actual (provavelmente `compiler/eval.md`, hub grande — confirmar,
   não presumir).
2. Aplicar os 4 critérios do P1002 a `rules.rs` na íntegra (o P1008 só fez o critério-zero
   e confirmou "agregado", fan-in real — falta o trabalho completo):
   - Inventário completo (`pub`, `pub(crate)`, `fn` privadas de topo — lição do P1000).
   - Critério 2 (pureza/estado) — `rules.rs` **não** é declarativo como `operators.rs`
     (toca `Engine<'a>`/`EvalContext` extensivamente, confirmado no P1007). Este critério
     deve discriminar aqui, ao contrário dos dois casos anteriores — verificar.
   - Critério 3 (co-mudança histórica) — mesma técnica dos passos anteriores, descontando
     ruído de resselo de hash (lição do P1006).
   - Critério 4 (vanilla) — já sabemos do P1003: `typst_eval::rules` (pequeno) +
     `typst_realize` (separado). Usar como candidato de fronteira, confirmar com critério 3.
3. Decidir a lista de nós. **O mecanismo de paragem (secção seguinte) é candidato a um
   nó próprio** (`show_rule_termination.rs`) — mas só se o critério 3 o confirmar como
   unidade coesa; não impor isto à força se a evidência de co-mudança apontar para outra
   fronteira.

## Fase B — Materializar, incorporando o mecanismo de paragem

Para o nó que cobrir a aplicação/terminação do loop α de show-rules, incorporar o
conteúdo já preparado (revisão cruzada Claude/Qwen + Passo 1007) como ponto de partida:

### Mecanismo revisto (conteúdo já preparado, a integrar no L0 do nó correspondente)

**Técnica**: sistema de reescrita de termos — detecção de terminação por histórico de
formas canónicas + transições, em vez de tecto cego de profundidade.

**Justificação de segurança** (não repetir a investigação, já feita): Passo 1007 confirmou
por leitura **e teste empírico** (3 casos) que `counter.get()`/`state.get()` são bloqueados
fora de `context` durante este loop, e que mutações (`counter.step()`/`state.update()`)
produzem `Content::CounterUpdate`/`StateUpdate` preservados por `morph_canon` — logo
qualquer efeito de estado que varie entre iterações aparece na própria forma comparada.
Isto é garantia estrutural, não ausência de contra-exemplo, e justifica terminação
antecipada (não só diagnóstico pós-limite).

**Gatilho de reabertura**: se `apply_show_rules` alguma vez correr com `in_context ==
true`, esta decisão tem de ser revista.

**Algoritmo** (estrutura corrigida — `seen`/`transitions` separados, não `Vec<RuleId>` por
estado):

```rust
let mut seen: HashMap<MorphCanon, usize> = HashMap::new();
let mut transitions: Vec<Vec<RuleId>> = Vec::new();

let mut content = initial_content;
let mut canon = content.morph_canon();
seen.insert(canon.clone(), 0);

for step in 1..=MAX_SHOW_RULE_DEPTH {
    let mut applied_rules: Vec<RuleId> = Vec::new();
    content = apply_one_iteration(content, &mut applied_rules, /* ctx/engine */)?;
    transitions.push(applied_rules);

    let new_canon = content.morph_canon();

    if new_canon == canon {
        return Ok(content);                              // Desfecho 1 — ponto-fixo
    }
    if let Some(&start) = seen.get(&new_canon) {
        let cycle_rules: Vec<RuleId> =
            transitions[start..step].iter().flatten().copied().collect();
        return Err(SourceDiagnostic::show_rule_cycle(start, step, cycle_rules)); // Desfecho 2
    }
    seen.insert(new_canon.clone(), step);
    canon = new_canon;
}
Err(SourceDiagnostic::show_rule_depth_exceeded(MAX_SHOW_RULE_DEPTH))  // Desfecho 3
```

**Restrição de mensagem**: Desfecho 3 mantém a mensagem primária byte-idêntica ao vanilla
(ADR-0033). Desfecho 2 é capacidade nova, sem equivalente vanilla — **decisão a tomar
neste passo, não adiar**: mensagem própria distinta, ou primeira linha igual + hint anexado
(padrão `§P846`)? Registar a escolha e a razão no L0 final do nó.

**Critérios de verificação a incorporar** (per o rascunho anterior, ajustar aos nós reais
decididos na Fase A):
```
Dado cadeia que estabiliza em N<64 passos → Ok no passo N (guarda: 18 testes P340)
Dado cadeia que nunca repete nem estabiliza em 64 → Err, mensagem byte-idêntica (Desfecho 3)
Dado par de regras A/B alternando → Err(ShowRuleCycle) nomeando A e B, antes de 64 (Desfecho 2, caso novo)
Dado Passo 1007 Teste 1 (counter.step() que re-casa) → Ok, guarda de não-regressão
Dado Passo 1007 Teste 2 (counter.step() fora do heading) → Ok, guarda de não-regressão
```

### Resto do fatiamento

Seguir o método normal do P1002 para os outros nós que a Fase A revelar (matching de
selectors, `font-dict` — já sabemos do Passo 1001 que isto está aqui e é responsabilidade
não relacionada, candidato a nó próprio separado).

## Fase C — Validar

```
crystalline-lint .
cargo test --workspace
```
Zero regressão nos 18 testes P340 + os 3 testes do P1007 (promovidos de temporários a
permanentes, se ainda não estiverem).

## Fase D — Avaliação do método (exigência já estabelecida em P1002/P1006)

Registar se o critério 2 (pureza/estado) finalmente discriminou — é o primeiro candidato
desde `operators.rs`/`metrics.rs` onde se espera que discrimine, dado que `rules.rs` toca
`Engine<'a>` extensivamente.

---

## Resultado esperado

`eval::rules` fatiado em hub + nós, um dos quais é `show_rule_termination.rs` com o
mecanismo revisto, mensagem primária preservada, decisão explícita sobre a forma da
mensagem de ciclo, e os 21 testes de guarda (18+3) verdes.
