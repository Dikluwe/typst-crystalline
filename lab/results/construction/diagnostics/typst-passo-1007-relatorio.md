# Passo 1007 — Verificação: `apply_show_rules` é puro ou fecha sobre contexto externo?

**Tipo**: Verificação — ler, testar e responder, sem desenhar nem implementar mecanismo de ciclo.  
**Estado da árvore**: commit `00dc949665317dedabc5ec01f0af5f4f4dc6cc80`, `git status` limpo após remoção dos testes temporários.  
**Data**: 2026-08-12.

---

## Fase A — Localização do mecanismo

O passo referia-se a `01_core/src/engine/eval/rules.rs`, mas o Passo 1005 moveu o eval para `01_core/src/compiler/eval/`. O mecanismo atual vive em:

- `01_core/src/compiler/eval/rules.rs:588` — `pub(crate) fn apply_show_rules(...)`.
- `01_core/src/compiler/eval/rules.rs:962` — `pub(crate) fn intercept_content(...)` (ponto de entrada).
- `01_core/src/compiler/eval/rules.rs:627` — closure `apply_all` que contém o loop de revisitação α (P348).
- `01_core/src/entities/content.rs:3278` — `pub fn morph_canon(&self) -> Content`.

---

## Fase B — Assinatura e captura

### 1. Assinatura de `apply_show_rules`

```rust
pub(crate) fn apply_show_rules(
    mut content: Content,
    rules: &[ShowRule],
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content>
```

Além do `Content`, recebe:
- `rules: &[ShowRule]` — snapshot das regras ativas (Arc clone em `intercept_content`, linha 982).
- `ctx: &mut EvalContext` — flags (`full_error`, `apply_show_rules`, `in_context`, etc.).
- `engine: &mut Engine<'_>` — agregador com `world`, `library`, `introspector`, `sink`, `route`, `styles`, `show_rules`, `active_guards`, `current_file`, `font_metrics`.

### 2. O que é lido ou mutado durante o loop de revisitação

Dentro do loop α (`rules.rs:651-781`):

| Recurso | Uso durante o loop | Efeito no resultado |
|---|---|---|
| `engine.active_guards` | `push(rule.id)` / `pop()` antes/depois de `apply_func` | Impede recursão durante a chamada do recipe. Não altera a forma do `Content` retornado. |
| `engine.route` | Lido por `route_check_show_depth(engine.route)?` | Verifica teto de profundidade; não modifica `route`. |
| `engine.styles` | Lido por `counter_display` (pattern de numbering) e show-set | Não muda durante o loop. |
| `engine.sink` | Pode ser mutado por closures via `apply_func` (warnings) | Não afeta o `Content` retornado. |
| `ctx.introspector` | Lido por `state.get()` / `counter.get()` / `counter.display()` | **Só disponível quando `ctx.in_context` é true** (ver abaixo). |
| `ctx.full_error` | Lido uma vez (Copy) | Liga/desliga histórico de morfologias; não muda durante o loop. |

### 3. `Transformation::Func` e captura de contexto

A aplicação de `Transformation::Func` chama `closures::apply_func(func.clone(), args, &mut scopes, ctx, engine)` (`rules.rs:681`). A closure capturou o scope no momento da definição da show rule. Ela pode, em princípio:

- Ler `engine.world` (I/O, query).
- Ler `engine.introspector` (query, counter, state).
- Escrever em `engine.sink` (warnings).
- Chamar nativas como `counter.step()` / `state.update()`.

**Restrição crítica**: `counter.get()`, `state.get()` e `counter.display()` só funcionam **dentro de context** (`in_context == true`). O loop α de `apply_show_rules` corre durante o eval principal, fora de context. Logo, durante a revisitação de um nó, a closure **não pode consultar valores de counter/state** — só pode emitir os Content placeholders `CounterUpdate` / `StateUpdate`.

### 4. `Transformation::Style`

O show-set é tratado **depois** do loop α (`rules.rs:798-817`). Ele dobra os styles numa `StyleChain` local e embrulha o resultado em `Content::Styled(..., Styles::from_delta(delta))`. O styles fica **dentro** do `Content` resultante, logo faz parte da "forma" comparada por `morph_canon`.

### 5. O que `morph_canon` normaliza/ignora

`01_core/src/entities/content.rs:3278`:

```rust
pub fn morph_canon(&self) -> Content {
    let mut transform = |node: &Content| -> SourceResult<Option<Content>> {
        Ok(match node {
            Content::Text(s) => Some(Content::Text(s.clone())),
            Content::Styled(body, styles) if styles.is_semantically_empty() => {
                Some((**body).clone())
            }
            _ => None,
        })
    };
    self.map_content(&mut transform).expect("...")
}
```

**Apenas** remove `Content::Styled` que são semanticamente vazios. Não normaliza textos, não ignora styles significativos, não descarta `CounterUpdate`, `StateUpdate`, `Dynamic`, etc. A forma canónica é, portanto, uma representação fiel do conteúdo estrutural e dos side-effects declarativos (placeholders de counter/state).

### 6. `active_guards` vs loop de revisitação

São mecanismos **independentes** mas complementares:

- `active_guards` (stack de `RuleId`) impede recursão **durante a chamada do recipe** (criação aninhada). É restaurado por `push`/`pop` a cada aplicação.
- O loop α detecta ponto-fixo **após** o recipe devolver, comparando `morph_canon` do output com o work.

O `active_guards` não garante terminação do loop α; só evita que a mesma regra seja aplicada recursivamente dentro de uma única chamada.

---

## Fase C — Casos testados empiricamente

Foram adicionados três testes temporários a `01_core/src/compiler/eval/tests.rs`, executados com:

```bash
cargo test -p typst-core p1007 -- --nocapture
```

Resultado: **3 passed; 0 failed**.

### Teste 1 — `counter.step()` no corpo de heading que re-casa

```typst
#show heading: it => [= #counter("x").step()]
= a
```

- **Comportamento**: a regra transforma o heading num heading cujo corpo é `CounterUpdate`. Na 2ª revisitação, a morfologia é idêntica (`CounterUpdate` idêntico) → ponto-fixo.
- **Resultado**: termina sem erro.
- **Implicação**: `morph_canon` "vê" o `CounterUpdate`; o mecanismo não é enganado por um placeholder de estado.

### Teste 2 — `counter.step()` concatenado fora do heading

```typst
#show heading: it => counter("x").step() + it
= a
```

- **Comportamento**: a regra transforma o heading numa `Sequence(CounterUpdate, heading)`. A sequence não casa com o selector `heading`, logo a revisitação para na 1ª aplicação.
- **Resultado**: output contém `"a"`, sem erro.
- **Implicação**: a transformação é determinística; a morfologia captura corretamente a mudança de kind.

### Teste 3 — `counter.step()` no corpo de heading produzido, recursivamente

```typst
#show heading: it => [= #counter("x").step() #it.body]
= a
```

- **Comportamento**: cada revisitação acrescenta um `CounterUpdate` ao corpo do heading. A morfologia muda a cada passo → nunca estabiliza → teto `MAX_SHOW_RULE_DEPTH`.
- **Resultado**: erro `"maximum show rule depth exceeded"`.
- **Implicação**: quando o side-effect altera a morfologia a cada passo, o teto backstop atua como esperado.

---

## Fase D — Resposta directa

**Veredicto: 2 — Impuro, mas de forma inofensiva para este mecanismo especificamente.**

### Por que não é puro (1)

`apply_show_rules` não é uma função `Content → Content` pura:

1. Recebe `&mut Engine<'_>` e `&mut EvalContext`.
2. Pode chamar closures de usuário que fazem I/O (`read`, `image`), emitem warnings (`sink`) ou produzem placeholders de estado.
3. `engine.active_guards` é mutado durante a execução.

### Por que é inofensivo para detecção de ciclo por `morph_canon` (2, não 3)

Para que a proposta `HashMap<MorphCanon, (passo, Vec<RuleId>)>` falhasse, seria necessário que a mesma forma canónica pudesse surgir com significado diferente consoante o contexto, **durante o loop de revisitação de um único nó**. Os testes e a análise do código mostram que isso **não acontece**:

1. **Consultas a counter/state são bloqueadas fora de context**: `counter.get()`, `state.get()`, `counter.display()` exigem `ctx.in_context`. Durante `apply_show_rules`, `in_context` é falso. Logo, a closure não pode ler valores de estado que mudariam entre iterações.

2. **Mutações de counter/state produzem Content placeholders**: `counter.step()` e `state.update()` devolvem `Content::CounterUpdate` / `Content::StateUpdate`. Esses elementos fazem parte da árvore e são preservados por `morph_canon`. Se uma iteração acrescenta um placeholder, a morfologia muda (Teste 3). Se não acrescenta, a morfologia estabiliza (Teste 1).

3. **`morph_canon` não descarta informação semântica relevante**: só remove `Content::Styled` semanticamente vazios. Styles significativos, placeholders de estado, texto e estrutura permanecem na chave.

4. **O contexto externo (styles, route, show_rules) não muda durante o loop**: `engine.styles` é o mesmo; `engine.route` não é modificado; `rules` é um snapshot. As únicas mutações são `active_guards` (push/pop), que não afetam a forma do output.

### Ressalva

Se no futuro `apply_show_rules` for usado em contexto (`in_context == true`) — por exemplo, se a fase de realize for introduzida separadamente — esta conclusão teria de ser reavaliada, porque `counter.get()` / `state.get()` passariam a estar disponíveis e a ler do `introspector`, que pode mudar entre iterações. Hoje, isso não acontece.

---

## O que este passo NÃO fez

- Não desenhou nem implementou mecanismo de detecção de ciclo.
- Não alterou `apply_show_rules` nem `morph_canon`.
- Não decidiu se a proposta de Qwen avança.

## Resultado entregue

Resposta fechada (**2**) com evidência de `file:line` e testes empíricos, pronta para condicionar o L0 do mecanismo de paragem de `eval::rules` quando esse passo for escrito.
