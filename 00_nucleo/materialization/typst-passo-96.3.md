# Passo 96.3 — Promover ADR-0037 com 4 ajustes, renumerar DEBT-46

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md` — ADR
  `PROPOSTO`. Este passo promove a `EM VIGOR` com ajustes.
- `00_nucleo/DEBT.md` — entrada DEBT-46. Os checkboxes
  precisam de renumeração devido à introdução do Passo 96.2
  (delegação dos armos) que desloca os sub-passos seguintes.
- `00_nucleo/adr/README.md` — índice. Ajusta contagens.

Pré-condição: `cargo test` — 746 L1 + 174 L3 + 6 ignorados,
zero violations. Passo 96.2 concluído (delegação completa).

---

## Natureza deste passo

Passo único de governança. Não altera código. Três tarefas:

1. **Tarefa A** — Actualizar o texto da ADR-0037 com 4 ajustes
   derivados da validação empírica dos Passos 96.1 e 96.2.
2. **Tarefa B** — Promover ADR-0037 de `PROPOSTO` para
   `EM VIGOR`. Actualizar o índice `README.md`.
3. **Tarefa C** — Renumerar os checkboxes do DEBT-46 em
   consonância com a sequência final decidida.

Regra absoluta: **não altera código**, **não toca em
`01_core/`, `02_shell/`, `03_infra/`, `04_wiring/`**. Altera
apenas:

- `00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md`
- `00_nucleo/adr/README.md`
- `00_nucleo/DEBT.md`

---

## Decisões formalizadas neste passo

- ADR-0037 promovido a `EM VIGOR`.
- 4 ajustes no texto da ADR (A, B, C, D) registados abaixo.
- Sequência final dos sub-passos de reestruturação
  (96.3 → 96.8) registada no DEBT-46.

---

## Tarefa A — Ajustar texto da ADR-0037

### Ajuste A — Regra 4 revista (armos triviais ficam inline)

**Texto actual** (Regra 4 — Dispatchers pequenos):

> Funções que fazem `match` exaustivo sobre um enum grande
> (dispatchers) devem **delegar** imediatamente a funções
> especializadas em cada submódulo: [...]
> O dispatcher tem uma linha por armo. A lógica real vive no
> submódulo correspondente.

**Texto revisto**:

> Funções que fazem `match` exaustivo sobre um enum grande
> (dispatchers) devem **delegar** armos com lógica substancial
> a funções especializadas em cada submódulo. Armos triviais
> (construtores simples, literais, valores constantes) permanecem
> inline.
>
> Critério operacional:
> - **1–3 linhas**: inline.
> - **4–7 linhas**: decisão caso a caso com base em coesão
>   semântica (a lógica pertence claramente a um submódulo? →
>   extrai; é glue code específico do dispatcher? → inline).
> - **> 7 linhas**: extrai para submódulo.
>
> Exemplos aceitáveis inline:
> ```rust
> Expr::Int(n) => Ok(Value::Int(n.get())),
> Expr::Ident(id) => resolve_ident(id, scopes),
> _ => Err(unexpected_expr_error(expr)),
> ```
>
> Exemplos que devem delegar:
> ```rust
> Expr::Strong(s) => markup::eval_strong(ctx, route, styles, /* ... */, s),
> Expr::SetRule(r) => rules::eval_set_rule(ctx, route, styles, /* ... */, r),
> ```

### Ajuste B — Regra 3 clarificada (paths entre submódulos)

**Adicionar à Regra 3** (após o bloco actual de hierarquia):

> **Paths entre submódulos**: submódulos acedem a funções de
> outros submódulos via path relativo `super::X::func()`
> (subindo ao `mod.rs` do módulo pai e descendo ao submódulo
> destino). Paths absolutos (`crate::rules::eval::X::func`)
> reservam-se para casos onde o caminho relativo é confuso
> (ex: ficheiro de testes que acede a funções de vários
> módulos distintos).
>
> Nota técnica: o linter V14 (`ForbiddenImport`) aceita
> `super::` como path relativo padrão. Evitar `self::` que pode
> produzir conflitos em configurações específicas do linter.

### Ajuste C — Regra 5 clarificada (testes em ficheiro separado)

**Texto actual** (Regra 5 — Testes seguem o domínio):

> Testes unitários ficam no mesmo ficheiro que a lógica que
> testam [...] ou num submódulo de testes paralelo [...]. Não
> em ficheiro monolítico de testes que cruza domínios.

**Texto revisto**:

> Testes unitários ficam preferencialmente no mesmo ficheiro
> que a lógica que testam (`#[cfg(test)] mod tests` no fim do
> ficheiro) ou num submódulo de testes paralelo
> (`eval/math/tests.rs`).
>
> **Excepção aceite**: testes E2E e testes transversais que
> exercitam múltiplos domínios podem viver em ficheiro
> `tests.rs` dedicado no mesmo módulo, mesmo que exceda o
> limite da Regra 2. Esta é excepção natural que não requer
> marca Regra 6 — é reconhecida pela própria Regra 5.
>
> O princípio operativo: testes coesos com o domínio testado
> são preferidos; testes cross-cutting por natureza não se
> forçam a decomposição artificial.

### Ajuste D — Nota nova na Regra 1 (cruzamentos entre submódulos)

**Adicionar à Regra 1** (após o bloco actual de coesão por domínio):

> **Coesão não implica isolamento**: submódulos coesos por
> domínio podem (e frequentemente devem) referenciar-se
> mutuamente via `super::X::func()` quando a semântica o
> justifica. Exemplos observados:
>
> - `closures::eval_func_call` pode consultar
>   `bindings::eval_counter_method` para tratar chamadas a
>   métodos de contador.
> - `rules::eval_set_rule` chama `super::eval_expr` para
>   avaliar os argumentos do `#set`.
>
> A divisão por domínio facilita navegação e manutenção; não
> cria silos fechados. Cruzamentos entre submódulos são
> expectáveis e saudáveis quando reflectem dependências
> semânticas reais.

### A.5 — Actualizar metadados

No topo do ficheiro da ADR:

```markdown
**Status**: `EM VIGOR`
**Data**: 2026-04-22 (`PROPOSTO`) / 2026-04-22 (`EM VIGOR` após validação nos Passos 96.1–96.2)
```

Ajustar a data de `EM VIGOR` para a data real da execução deste passo.

### A.6 — Remover secção "Status `PROPOSTO` vs `EM VIGOR`"

A secção explicativa sobre o uso de `PROPOSTO` já não é
necessária após a promoção. Substituir por nota breve:

> **Nota histórica**: esta ADR começou como `PROPOSTO`
> (2026-04-22) e foi validada empiricamente nos Passos 96.1
> (reestruturação do `eval.rs`) e 96.2 (completar delegação
> dos armos). Promovida a `EM VIGOR` no Passo 96.3 com 4
> ajustes (A: Regra 4 revista; B: Regra 3 clarificada sobre
> paths; C: Regra 5 clarificada sobre testes; D: nota na
> Regra 1 sobre cruzamentos).

---

## Tarefa B — Actualizar `00_nucleo/adr/README.md`

Quatro actualizações:

### B.1 — Tabela "Estado por ADR"

Alterar a linha do ADR-0037:

```markdown
| 0037 | Coesão por domínio — ficheiros limitados a uma responsabilidade clara | `EM VIGOR` |
```

### B.2 — Actualizar contagens

```
- `PROPOSTO`: 14 ADRs  →  13 ADRs
- `EM VIGOR`: 8 ADRs   →  9 ADRs
```

(Valores exactos dependem do estado actual; ajustar se o valor
prévio diferir.)

### B.3 — Meta-regras em vigor

Adicionar entrada nova. Localizar onde o ADR-0036 foi adicionado
(entrada 7 pelo Passo 91.5). Adicionar como entrada 8:

```markdown
8. **Coesão por domínio** — ADR-0037. Ficheiros em L1 agrupam
   código por domínio conceptual ou técnico. Limite orientativo
   de 800 linhas; excepções Regra 6 documentadas no topo do
   ficheiro. Primeira aplicação concreta: Passo 96.1–96.2
   (reestruturação do `eval.rs`). Trabalho restante: DEBT-46.
```

### B.4 — Preâmbulo

Se o preâmbulo menciona "14 ADRs `PROPOSTO`" ou similar, ajustar
para reflectir a nova distribuição.

---

## Tarefa C — Renumerar DEBT-46

### C.1 — Decisão sobre numeração

A sequência final dos sub-passos é:

| Sub-passo | Trabalho | Estado |
|-----------|----------|--------|
| 96 | Governança (ADR `PROPOSTO` + DEBT-46) | Concluído |
| 96.1 | Reestruturação `eval.rs` | Concluído |
| 96.2 | Completar delegação dos armos | Concluído |
| 96.3 | Promover ADR-0037 com ajustes | **Este passo** |
| 96.4 | Reestruturação `parse.rs` | Pendente |
| 96.5 | Reestruturação `stdlib.rs` | Pendente |
| 96.6 | Reestruturação `layout/mod.rs` | Pendente |
| 96.7 | Reestruturação `math/layout.rs` | Pendente |
| 96.8 | Reestruturação `lexer/mod.rs` ou excepção | Pendente |
| 96.9 | Encerramento DEBT-46 (ou fecho em 96.8) | Pendente |

### C.2 — Actualizar checkboxes do DEBT-46

Texto actual tem 8 checkboxes; precisa de 10 (dois novos para
96.2 e 96.3, mais encerramento). Reformular assim:

```markdown
### Critério de conclusão

- [x] `eval.rs` reestruturado em submódulos por domínio.
      Passo 96.1. **Concluído** — 7 submódulos criados
      (markup, math, modules, rules, closures, control_flow,
      bindings), mod.rs + tests.rs documentados como
      excepções Regra 6 no momento da conclusão.
- [x] Delegação completa dos armos longos do dispatcher
      `eval_expr`. Passo 96.2. **Concluído** — mod.rs caiu
      para 520 linhas, excepção Regra 6 removida.
- [x] ADR-0037 promovida de `PROPOSTO` para `EM VIGOR` com
      ajustes validados nos Passos 96.1–96.2. Passo 96.3.
      **Concluído neste passo.**
- [ ] `parse.rs` reestruturado por tipo de nó (markup, code,
      math, rules). Passo 96.4.
- [ ] `stdlib.rs` reestruturado por módulo da stdlib (text,
      layout, math, calc, etc.). Passo 96.5.
- [ ] `layout/mod.rs` reestruturado (orquestração, medição,
      emissão, sub-frames). Passo 96.6.
- [ ] `math/layout.rs` reestruturado ou marcado como
      excepção Regra 6. Passo 96.7.
- [ ] `lexer/mod.rs` reestruturado ou marcado como excepção
      Regra 6. Passo 96.8.
- [ ] Verificação final: nenhum ficheiro em `01_core/src/`
      acima de 800 linhas sem justificativa Regra 6
      documentada no topo. Passo 96.9 (ou fecho em 96.8 se
      lexer for excepção).
```

### C.3 — Verificação da renumeração

Confirmar que:
- Os 3 primeiros checkboxes estão `[x]` com referência correcta
  aos passos executados.
- Os 6 seguintes estão `[ ]` com referência aos passos 96.4–96.9.
- A sequência de números é contínua, sem saltos.

---

## Critérios de conclusão

- [ ] ADR-0037 actualizada com 4 ajustes (A, B, C, D).
- [ ] Status do ADR-0037 alterado para `EM VIGOR` com data
      actualizada.
- [ ] Secção "Status `PROPOSTO` vs `EM VIGOR`" substituída por
      nota histórica.
- [ ] `README.md` actualizado: status da linha ADR-0037,
      contagens, meta-regras em vigor (nova entrada 8),
      preâmbulo.
- [ ] DEBT-46 com 9 checkboxes (3 marcados `[x]`, 6
      pendentes).
- [ ] Nenhum outro ADR alterado.
- [ ] Nenhum ficheiro de código alterado.
- [ ] `cargo test` passa com os mesmos 746 L1 + 174 L3 + 6
      ignorados. `crystalline-lint` → zero violations.

---

## Ao terminar, reportar

Tarefa A:
- Confirmar os 4 ajustes aplicados no texto da ADR.
- Linhas do ficheiro final da ADR.
- Confirmar secção "Status" substituída por nota histórica.

Tarefa B:
- 4 actualizações no `README.md` confirmadas.
- Valores de contagem antes/depois.

Tarefa C:
- Confirmar número final de checkboxes no DEBT-46 (esperado: 9).
- Confirmar que os 3 primeiros estão `[x]` e os 6 seguintes
  estão `[ ]`.

Verificação:
- Contagem de testes inalterada.
- Zero violations.

Go/No-Go para Passo 96.4:
- **Go incondicional** — ADR-0037 está `EM VIGOR`, os 4 ajustes
  validados empiricamente estão no texto, DEBT-46 está
  renumerado. Passo 96.4 aplica ADR-0037 a `parse.rs` (2255
  linhas, divisão por tipo de nó: markup, code, math, rules).
- **No-Go** se algum dos 4 ajustes revelar inconsistência
  interna ao ser escrito (ex: Ajuste A contradiz Ajuste D).
  Nesse caso, reportar a inconsistência e esperar orientação
  antes de promover.
