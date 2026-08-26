# Prompt L0 — `compiler/eval/show_rule_termination` — loop α e terminação de show rules
Hash do Código: b5897386

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
tem de ser revista. Mutações (`counter.step()`/`state.update()`) produzem
`Content::CounterUpdate`/`StateUpdate` preservados por `morph_canon`. Logo qualquer
efeito de estado que varie entre iterações aparece na própria forma comparada,
o que justifica terminação antecipada.

> **Correcção P1031 — a premissa anterior estava errada, e o gatilho já disparou.**
>
> A redacção anterior desta secção dizia: *"Actualmente (Passo 1007, confirmado por teste
> empírico), `counter.get()`/`state.get()` estão bloqueados fora de `context` durante este
> loop"*. Isso é **falso enquanto afirmação sobre a linguagem Typst** — e o "teste empírico"
> do P1007 não deixou registo de proveniência (comando, resultado, commit), pelo que não é
> reproduzível. Frase removida.
>
> **Documentação oficial (citação literal)** — `typst.app/docs/reference/language/context/`,
> fonte em `lab/typst-original/docs/content/reference/language/context.typ:13`:
> *"Aside from explicit context expressions, context is also established implicitly in some
> places that are also aware of their location in the document: Show rules provide context
> [nota: currently, all show rules provide style context, but only show rules on locatable
> elements provide a location context] and numberings in the outline, for instance, also
> provide the proper context to resolve counters."*
>
> Ou seja: na linguagem, o corpo de uma show rule **é** contexto. `counter.get()` lá dentro
> é legítimo e deve resolver.
>
> **Medição directa (P1031, 2026-08-13)** — ficheiro
> `#let c = counter("x")` + `#show heading: it => [got:#c.get()|#it.body]` + `#c.step()` +
> `= Um` + `#c.step()` + `= Dois`:
>
> | Binário | Resultado |
> |---|---|
> | Vanilla ratificado `/usr/local/bin/typst` (`typst 0.15.1 (e0e8ca4d)`) | `got:(1,)\|Um got:(2,)\|Dois` |
> | Cristalino `target/release/typst` (fonte em HEAD `4f64e4e69`; árvore de trabalho só com edições em `00_nucleo/prompts/**`) | **erro**: `counter.get() can only be used inside context` |
>
> Com `#context c.get()` explícito dentro da show rule, o cristalino compila mas produz
> `got:\|Um got:\|Dois` — valor **silenciosamente vazio**, em vez de `(1,)`/`(2,)`.
>
> **Consequência para este L0**: o argumento de terminação antecipada **não** pode
> apoiar-se em "as leituras de estado estão bloqueadas". Apoia-se apenas na parte que
> sobrevive: as *mutações* aparecem como `Content::CounterUpdate`/`StateUpdate` na forma
> canónica comparada. Quando o cristalino passar a dar contexto às show rules, esta secção
> tem de ser reavaliada — o gatilho de reabertura já está activo, não é hipotético.
>
> **Achado escalado (não corrigido aqui)**: show rules não estabelecem contexto no
> cristalino. É divergência de linguagem com resultado silenciosamente errado no caso
> `#context`; exige passo próprio e gate ADR-0127 (mudança de comportamento por defeito).
> Registado no relatório do P1031 como achado a escalar, prioridade alta.
>
> **P1037 — a causa do caso `#context` está localizada; a classificação do gate afina-se.**
> O resultado vazio **não** vem daqui: vem de a introspecção correr sobre a árvore
> **pré-show-rules** (`entities/module.rs:96`, P498), pelo que um `ContextBlock` criado por
> uma show rule tem `id` que nunca entra em `intr.context_block_locations` e acaba trocado
> por `Content::Empty` em `substitute_context_blocks`. Medição, cadeia causal por
> `file:line` e o descarte da hipótese concorrente (walk parcial de containers, esse sim
> corrigido em P1037): `00_nucleo/prompts/infra/pipeline.md` §P1037. O gate continua, mas o
> ponto aplicável é o **3** (mudança de fase do pipeline: introspectar o produto das show
> rules), não só o 2.
>
> **Nota de proveniência sobre a versão do binário**: `--version` do cristalino imprimiu
> `typst 0.15.0 (0f8487b9)` nesta medição, mas o HEAD era `4f64e4e69`. O hash do `--version`
> **não** é prova do commit compilado: `02_shell/build.rs` só declara
> `rerun-if-env-changed=TYPST_COMMIT_SHA` (cópia fiel do vanilla,
> `lab/typst-original/crates/typst-utils/build.rs`), logo o cargo não reexecuta o script
> quando o HEAD muda e o carimbo fica preso ao primeiro build daquele `target/`.

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
