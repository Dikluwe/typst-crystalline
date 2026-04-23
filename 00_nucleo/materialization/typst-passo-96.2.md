# Passo 96.2 — Completar delegação dos armos do dispatcher `eval_expr`

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md` — ADR
  `PROPOSTO`. Regra 4 (dispatchers pequenos) não foi aplicada
  em pleno no Passo 96.1; este passo completa a aplicação.
- `00_nucleo/DEBT.md` — entrada DEBT-46. Este passo não marca
  novo checkbox — completa o trabalho do Passo 96.1 que
  deixou pendência.
- `01_core/src/rules/eval/mod.rs` — dispatcher actual com ~40
  armos, ~500 linhas. Total do ficheiro: 879 linhas.
- Submódulos existentes: `markup.rs`, `math.rs`, `operators.rs`,
  `control_flow.rs`, `closures.rs`, `bindings.rs`, `rules.rs`,
  `tests.rs`.

Pré-condição: `cargo test` — 746 L1 + 174 L3 + 6 ignorados,
zero violations. Passo 96.1 concluído.

---

## Natureza deste passo

Passo único de reestruturação. Completa trabalho do Passo 96.1
em aspecto específico: delegação de armos do dispatcher
`eval_expr` para funções dos submódulos correspondentes.

Objectivo: cada armo longo (> 5 linhas ou > 3 referências ao
mesmo submódulo) passa a uma linha de delegação. Armos triviais
(construtores simples, constantes) permanecem inline.

Após este passo, `mod.rs` fica significativamente menor (alvo:
450–550 linhas), validando empiricamente a Regra 4 revista que
o Passo 96.3 vai formalizar no ADR-0037.

**Não altera semântica observável** (ADR-0033). Preserva a
atomização do Passo 92–95 (ADR-0036). Não toca em submódulos
que já cumprem a Regra 4.

---

## Decisões formalizadas neste passo

Nenhuma. Este passo **aplica** a Regra 4 da ADR-0037 com
interpretação revista que o Passo 96.3 vai registar
formalmente. A interpretação é:

- **Armos triviais ficam inline**: construtores simples, valores
  literais, delegação mecânica a função única existente.
- **Armos longos delegam**: > 5 linhas de lógica ou lógica
  coesa que pertence semanticamente a um submódulo.

Exemplo de armo trivial (fica inline):

```rust
Expr::Int(n) => Ok(Value::Int(n.get())),
Expr::Bool(b) => Ok(Value::Bool(b.get())),
Expr::None(_) => Ok(Value::None),
```

Exemplo de armo longo (delega):

```rust
// Antes (inline):
Expr::Strong(strong) => {
    let body = strong.body();
    let mut local_styles = styles.push(StyleDelta { bold: Some(true), .. });
    let content = eval_markup(ctx, route, &mut local_styles, /* ... */, body)?;
    Ok(Value::Content(Content::Strong(Arc::new(content))))
}

// Depois (delega):
Expr::Strong(s) => markup::eval_strong(ctx, route, styles, /* ... */, s),
```

---

## Tarefa 1 — Identificar candidatos a extracção

### 1.1 — Localizar `eval_expr`

```bash
grep -n "fn eval_expr" 01_core/src/rules/eval/mod.rs
```

### 1.2 — Listar armos por tamanho

Examinar o `match` principal e classificar cada armo:

- **Trivial** (1–3 linhas): permanece inline.
- **Longo** (> 5 linhas): extrai para submódulo.
- **Médio** (4–5 linhas): decisão caso a caso — se a lógica
  pertence semanticamente a um submódulo, extrai; se é
  glue code específico do dispatcher, permanece inline.

Reportar antes de prosseguir:
- Número total de armos no `eval_expr`.
- Número classificado como trivial (permanece).
- Número classificado como longo (extrai).
- Número classificado como médio (decisão por armo).

### 1.3 — Mapear armo → submódulo destino

Para cada armo que vai extrair, identificar o submódulo de
destino com base no domínio:

| Armo `Expr::*` | Submódulo |
|----------------|-----------|
| `Text`, `Strong`, `Emph`, `Heading`, `Link`, `Raw`, `List`, `Enum`, `Ref`, `Label` | `markup.rs` |
| `Math*` | `math.rs` |
| `Binary`, `Unary` | `operators.rs` |
| `Conditional`, `While`, `For`, `Break`, `Continue`, `Return` | `control_flow.rs` |
| `Closure`, `FuncCall` | `closures.rs` |
| `Let`, `Ident` | `bindings.rs` |
| `SetRule`, `ShowRule` | `rules.rs` |
| `ModuleInclude`, `ModuleImport` | `modules.rs` (criar se não existe — ver nota abaixo) |
| `CodeBlock`, `ContentBlock` | permanece em `mod.rs` (scoping cross-cutting) |

**Nota sobre `modules.rs`**: o Passo 96.1 pode não ter criado
este submódulo (o reporte lista 6 submódulos sem `modules.rs`).
Verificar se `ModuleInclude` / `ModuleImport` estão em
`mod.rs`, `markup.rs`, ou outro. Se necessário, criar
`modules.rs` como parte deste passo — é extracção adicional
coerente com o Passo 96.1.

---

## Tarefa 2 — Extrair armos longos

### 2.1 — Extracção por submódulo

Para cada submódulo destino, **extrair em bloco** (todos os
armos daquele submódulo de uma vez):

1. Identificar os armos a mover.
2. Criar função `pub(super) fn eval_<nome>(ctx, route, styles,
   show_rules, active_guards, node) -> SourceResult<Value>`
   no submódulo, com a lógica do armo.
3. Substituir o corpo do armo em `mod.rs` por uma linha de
   delegação: `Expr::<Nome>(n) => submódulo::eval_<nome>(ctx,
   route, styles, show_rules, active_guards, n),`

### 2.2 — Ordem recomendada

Mesmo critério do Passo 96.1 — começar pelos submódulos mais
isolados:

1. `math.rs` — já tem funções que recebem os parâmetros
   necessários; extracção mecânica.
2. `operators.rs` — idem.
3. `control_flow.rs` — medianamente isolado.
4. `bindings.rs` — relativamente isolado.
5. `markup.rs` — muitos armos, extracção mais trabalhosa.
6. `closures.rs` — último (mais entrelaçado).
7. `rules.rs` — último (tem mutação cross-cutting de
   `show_rules`).

### 2.3 — Verificação por submódulo

Após extrair os armos de cada submódulo:

```bash
cargo check --package typst-core 2>&1 | tail -10
cargo test --package typst-core 2>&1 | tail -5
```

Ambos devem passar. Se falhar, rollback **desse submódulo** e
reportar.

### 2.4 — Tratamento de `CodeBlock` e `ContentBlock`

Estes dois armos têm scoping cross-cutting (criam `local_styles`,
`local_show_rules`). Duas opções:

- **Permanecer inline** em `mod.rs` — scoping é responsabilidade
  do dispatcher, não de submódulo específico.
- **Extrair para `blocks.rs`** (novo submódulo) — scoping é
  domínio distinto que merece ficheiro próprio.

Recomendação: **permanecer inline**. Razão: são apenas 2 armos;
criar submódulo para dois armos contradiz o princípio da
ADR-0037 (coesão por domínio; submódulo trivial não traz ganho).
Se ficarem longos (> 30 linhas combinados), considerar extracção
para `mod.rs::eval_code_block` e `mod.rs::eval_content_block`
como funções privadas do `mod.rs` — ganham nome próprio sem
criar submódulo.

---

## Tarefa 3 — Criar `modules.rs` se necessário

Se a Tarefa 1.3 identificou que `ModuleInclude` / `ModuleImport`
estão inline em `mod.rs` ou mal colocados noutro submódulo:

1. Criar `01_core/src/rules/eval/modules.rs` com cabeçalho
   análogo aos outros submódulos.
2. Mover lógica para `pub(super) fn eval_module_include` e
   `pub(super) fn eval_module_import`.
3. Adicionar `mod modules;` em `mod.rs`.
4. Substituir armos em `eval_expr` por delegações.

---

## Tarefa 4 — Verificação final

### 4.1 — Tamanho do `mod.rs`

```bash
wc -l 01_core/src/rules/eval/mod.rs
```

Alvo: 450–550 linhas. Se ficar acima de 600, reavaliar armos
médios que ficaram inline — talvez devessem ter extraído.

### 4.2 — Tamanho dos submódulos

```bash
wc -l 01_core/src/rules/eval/*.rs | sort -rn
```

Esperado: crescimento moderado de cada submódulo (absorveu
lógica dos armos). Nenhum ultrapassa 800 linhas; se ultrapassar,
considerar subdivisão (ex: `markup.rs` em `markup/mod.rs` +
`markup/inline.rs` + `markup/block.rs`).

### 4.3 — Dispatcher compacto

```bash
# Contar armos no eval_expr:
grep -n "^\s*Expr::" 01_core/src/rules/eval/mod.rs | wc -l
```

O número continua ~40 armos (cada `Expr::*` tem de aparecer).
A diferença é que agora cada armo é uma linha, não um bloco.

### 4.4 — Testes e linter

```bash
cargo test --workspace 2>&1 | tail -10
cargo run --package crystalline-lint 2>&1 | tail -5
```

Esperado: 746 L1 + 174 L3 + 6 ignorados inalterados. Zero
violations.

### 4.5 — Actualizar nota Regra 6 no `mod.rs`

Se o `mod.rs` caiu abaixo de 800 linhas, **remover** a marca de
excepção Regra 6 do topo do ficheiro (já não é excepção).

Se continua acima, actualizar a justificativa para reflectir o
que permaneceu inline (armos triviais + scoping de blocos).

---

## Critérios de conclusão

- [ ] Cada armo longo do `eval_expr` delega para função de
      submódulo.
- [ ] Armos triviais (1–3 linhas) permanecem inline.
- [ ] `mod.rs` entre 450 e 700 linhas; se acima, justificativa
      actualizada.
- [ ] Submódulos actualizados com as funções extraídas; nenhum
      ultrapassa 800 linhas sem excepção Regra 6.
- [ ] `modules.rs` criado se necessário.
- [ ] `cargo test --workspace` passa com 746 L1 + 174 L3 + 6
      ignorados.
- [ ] `crystalline-lint` → zero violations.
- [ ] Nenhum `unsafe` novo introduzido.
- [ ] ADR-0037 não alterado neste passo (continua `PROPOSTO`;
      Passo 96.3 vai promover com ajustes).
- [ ] DEBT-46 não alterado neste passo (Passo 96.1 marcou o
      checkbox principal; este passo é refinamento).

---

## Ao terminar, reportar

Tarefa 1 (mapeamento):
- Total de armos no `eval_expr`.
- Classificação: triviais, longos, médios.
- Se `modules.rs` foi criado ou se `ModuleInclude`/
  `ModuleImport` já estavam noutro lugar apropriado.

Tarefa 2 (extracção):
- Lista de funções extraídas por submódulo.
- Linhas movidas por submódulo (aproximado).

Tarefa 3 (modules.rs):
- Se criado: tamanho.
- Se não necessário: localização actual de `ModuleInclude` e
  `ModuleImport`.

Tarefa 4 (verificação):
- Tamanhos finais de todos os ficheiros em
  `eval/`.
- Contagem final de armos no dispatcher.
- Testes verdes, zero violations confirmados.
- Estado da nota Regra 6 no `mod.rs` (removida ou actualizada).

Tensões encontradas adicionais para Passo 96.3:
- Armos onde a classificação trivial/longo foi ambígua.
- Armos médios que ficaram inline vs. extraídos — raciocínio.
- Outras fricções com a ADR-0037 que a revisão do Passo 96.3
  deve abordar.

Go/No-Go para Passo 96.3:
- **Go incondicional** se `mod.rs` ficou abaixo de 800 linhas
  e dispatcher compacto. Passo 96.3 promove ADR-0037 com os
  3 ajustes (A, B, C) identificados no Passo 96.1.
- **Go com nota adicional** se a extracção revelou fricção
  nova que merece ajuste suplementar na ADR. Reportar e
  incluir no Passo 96.3.
- **No-Go** se a extracção comprometeu testes ou introduziu
  complexidade desproporcional. Reverter e reavaliar.
