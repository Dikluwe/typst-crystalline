# Passo 91 — Abertura de DEBT-44 e DEBT-45 (governança pós-Passo 90)

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/DEBT.md` — inventário actual. DEBT-40 encerrado no
  Passo 90 e movido para Secção 2. DEBT-42 (scanner bloqueado)
  e DEBT-43 (linter crate-level) estão em aberto.
- `00_nucleo/adr/typst-adr-0033-*.md` — paridade funcional com
  vanilla. Relevante porque os dois DEBTs documentam divergências
  estruturais face ao vanilla.
- Reporte do Passo 90 (descrito abaixo) — contexto factual para
  o texto dos DEBTs.
- `01_core/src/rules/eval.rs` — localização do `EvalContext`
  com campo `route: Vec<FileId>` (divergência que o DEBT-44
  regista).
- `01_core/src/entities/world_types.rs` — localização de
  `Route<'a>` com métodos `check_show_depth`, `check_layout_depth`,
  `check_html_depth` (código não usado que o DEBT-45 regista).

Pré-condição: `cargo test` — 747 L1 + 174 L3 + 6 ignorados,
zero violations. Passo 90 concluído.

---

## Natureza deste passo

Passo único de governança. Não altera código. Abre dois DEBTs
que documentam estado real do projecto após o Passo 90:

- **DEBT-44** — O `Route<'a>` foi materializado em L1 mas o
  `EvalContext` não o usa estruturalmente. A API `with_route_id`
  continua a operar sobre `Vec<FileId>`. Divergência face ao
  vanilla persiste; `unsafe` desapareceu (DEBT-40 fechou) mas
  paridade estrutural da detecção de ciclo não foi alcançada.

- **DEBT-45** — `Route<'a>` tem 4 métodos `check_*_depth` mas
  o cristalino só chama `check_call_depth` (via API antiga do
  `EvalContext`, não via `Route`). Os outros 3
  (`check_show_depth`, `check_layout_depth`, `check_html_depth`)
  são código não executado.

Contexto de decisão: foi opção consciente do Passo 90. A escolha
pragmática preservou o valor principal (remover `unsafe`, fechar
DEBT-40) sacrificando convergência estrutural. Os dois DEBTs
registam o sacrifício para que não seja esquecido.

Regra absoluta: **não altera código, não altera ADRs**. Altera
apenas `00_nucleo/DEBT.md`.

---

## Decisões formalizadas neste passo

Nenhuma. Este passo documenta decisões já tomadas implicitamente
no Passo 90 e torna-as explícitas no inventário de dívida.

---

## Tarefa A — Abrir DEBT-44 no `DEBT.md`

### A.1 — Localização

Inserir na Secção 1 (DEBTs em aberto), após DEBT-43 (última
entrada actual da Secção 1 após a movimentação do DEBT-40).

### A.2 — Texto sugerido

```markdown
## DEBT-44 — `EvalContext` não usa estruturalmente `Route<'a>` — EM ABERTO (Passo 91)

O Passo 90 materializou `Route<'a>` em
`01_core/src/entities/world_types.rs` com paridade estrutural
completa face ao vanilla (`outer: Option<Tracked<'a, Self>>`,
linked list imutável, `#[comemo::track]` em `contains`/`within`).

No entanto, `EvalContext` (em `01_core/src/rules/eval.rs`) não
usa `Route<'a>` estruturalmente. Mantém campo
`pub route: Vec<FileId>` como projecção plana — uma lista linear
que imita a cadeia mas não é a estrutura vanilla. A API
`with_route_id(id, span, f)` substitui o `ImportGuard` antigo
com push/pop explícito sobre este `Vec`, sem `unsafe`.

### O que o Passo 90 resolveu

- `unsafe` em `ImportGuard::drop` eliminado (ADR-0032 avança).
- `Route<'a>` existe como tipo em L1 para uso futuro.
- DEBT-40 encerrado (o `unsafe` era o seu critério de fecho).

### O que o Passo 90 não resolveu

- Divergência estrutural face ao vanilla: o `eval` do vanilla
  propaga `Route<'a>` por valor entre frames (linked list
  imutável); o cristalino propaga estado partilhado num `Vec`
  dentro de `EvalContext`.
- `Tracked<'a, Route>` não é usado como parâmetro de funções
  memoizadas, mesmo quando a infraestrutura `comemo` já está
  disponível.

### Razão do adiamento

Opção escolhida no Passo 90 por decisão pragmática: integrar
`Route<'a>` estruturalmente exigiria refactor transversal de
~12 funções `eval_*` (parâmetro `route: &Route<'_>` ou campo
`route: Route<'a>` no `EvalContext<'w>`). Benefício observável
limitado enquanto `Expr::ModuleImport` e `Engine<'a>` continuam
stubs. A decisão preservou o valor principal (fechar `unsafe`,
fechar DEBT-40) e adiou o valor estrutural.

### Critério de conclusão

- [ ] `EvalContext.route: Vec<FileId>` eliminado.
- [ ] Mecanismo de detecção de ciclo usa `Route<'a>` directamente
      (campo no contexto se Classe A do Passo 90, ou parâmetro
      em funções relevantes se Classe B).
- [ ] API pública do eval não muda (comportamento observável
      preservado — ADR-0033).
- [ ] Teste E2E `import_cycle_detectado_retorna_err_sem_panic`
      continua a passar sem alteração.
- [ ] Nenhum novo `unsafe` introduzido.

### Dependências

Nenhuma técnica. Passo dedicado quando priorizado. O Passo 90
deixou o cenário preparado: `Route<'a>` existe e tem API
funcional; falta apenas ligá-lo ao `EvalContext`.

### Nota sobre escopo

A integração estrutural, quando atacada, pode revelar que o
escopo real é maior do que ~12 funções — por exemplo, se o
`Engine<'a>` for materializado simultaneamente. Nesse caso,
este DEBT pode dividir-se em sub-DEBTs.

---
```

### A.3 — Verificação após A

Confirmar que `DEBT-44` aparece na Secção 1 após `DEBT-43`.
Contagem de DEBTs em aberto: 3 → 4 (DEBT-42, DEBT-43, DEBT-44).

---

## Tarefa B — Abrir DEBT-45 no `DEBT.md`

### B.1 — Localização

Inserir na Secção 1 (DEBTs em aberto), após DEBT-44.

### B.2 — Texto sugerido

```markdown
## DEBT-45 — Métodos `check_*_depth` de `Route<'a>` não chamados pelo eval — EM ABERTO (Passo 91)

O Passo 90 materializou `Route<'a>` com 4 métodos de verificação
de profundidade:

- `check_call_depth` (limite `MAX_CALL_DEPTH = 80`)
- `check_show_depth` (limite `MAX_SHOW_RULE_DEPTH = 64`)
- `check_layout_depth` (limite `MAX_LAYOUT_DEPTH = 72`)
- `check_html_depth` (limite `MAX_HTML_DEPTH = 72`)

Paridade estrutural com vanilla (ADR-0033). Mas apenas
`check_call_depth` tem equivalente chamado no cristalino actual
— via `EvalContext::check_call_depth()` da era pré-Route,
independente do `Route<'a>`.

Os 3 restantes são **código não executado**:

- `check_show_depth` — seria chamado em `apply_show_rules` ou
  equivalente; actualmente não chamado.
- `check_layout_depth` — seria chamado no braço de layout
  recursivo; actualmente não chamado.
- `check_html_depth` — seria chamado no pipeline HTML (que
  não existe no cristalino ainda).

### Impacto

- Zero impacto funcional imediato — os métodos existem mas não
  são invocados; nenhuma profundidade é verificada onde não era
  verificada antes.
- Impacto latente: bugs de recursão infinita em show rules ou
  layout não são capturados pelo limite declarado em `Route`.
  O cristalino continua vulnerável aos mesmos stack overflows
  que o limite do vanilla evita.

### Critério de conclusão

- [ ] `check_show_depth` chamado no ponto correspondente em
      `rules/eval.rs` ou `rules/show.rs`, consistente com o
      vanilla.
- [ ] `check_layout_depth` chamado no ponto correspondente em
      `rules/layout/`, consistente com o vanilla.
- [ ] `check_html_depth` chamado quando o pipeline HTML existir
      (não antes — aguarda materialização do pipeline).
- [ ] `EvalContext::check_call_depth` antigo pode ser removido
      ou re-encaminhado para `Route::check_call_depth` — decisão
      adiada para o passo que atacar este DEBT.
- [ ] Testes que exercitam cada limite passam sem alteração da
      asserção (comportamento observável preservado).

### Dependências

- DEBT-44 preferencialmente resolvido antes — faz mais sentido
  `EvalContext` usar `Route<'a>` estruturalmente antes dos
  `check_*` serem integrados. Resolver DEBT-45 sem DEBT-44 é
  possível mas cria dependência artificial sobre o mecanismo
  antigo.

### Nota sobre escopo

`check_html_depth` só é accionável quando o pipeline HTML for
materializado. Este DEBT pode ser parcialmente pago (3 limites
integrados) e manter o quarto aberto até então.

---
```

### B.3 — Verificação após B

Confirmar que `DEBT-45` aparece na Secção 1 após `DEBT-44`.
Contagem de DEBTs em aberto: 4 → 5 (DEBT-42, DEBT-43, DEBT-44,
DEBT-45).

---

## Critérios de conclusão

- [ ] DEBT-44 adicionado ao `00_nucleo/DEBT.md` Secção 1, após
      DEBT-43.
- [ ] DEBT-45 adicionado ao `00_nucleo/DEBT.md` Secção 1, após
      DEBT-44.
- [ ] Nenhum outro DEBT alterado (42, 43, 40 em Secção 2
      inalterados).
- [ ] Nenhum ADR alterado.
- [ ] Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/`,
      `04_wiring/` alterado.
- [ ] `cargo test` passa com os mesmos 747 L1 + 174 L3 + 6
      ignorados. `crystalline-lint` → zero violations.

---

## Ao terminar, reportar

- Linhas exactas onde DEBT-44 começa e termina em `DEBT.md`.
- Linhas exactas onde DEBT-45 começa e termina em `DEBT.md`.
- Contagem total de DEBTs por secção:
  - Secção 1 (abertos): esperado 5 (DEBT-42, -43, -44, -45; e
    DEBT-1, -2, -8, -9, -33, -34d, -34e, -35b, -39 que já estavam).
    **Confirmar pela contagem exacta após o passo**.
  - Secção 2 (encerrados): inalterada.
- Tamanho de cada entrada em linhas.

Go/No-Go para Passo 92:
- **Go incondicional** para Passo 92 = integrar `Route<'a>` de
  facto no `EvalContext`, pagar DEBT-44.
- Se a contagem de DEBTs abertos revelar inconsistência (ex:
  um DEBT da lista histórica já estava encerrado sem marca na
  Secção 2), reportar como observação lateral — não bloqueia
  o Passo 92.
