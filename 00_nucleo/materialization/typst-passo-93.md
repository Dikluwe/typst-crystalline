# Passo 93 — Diagnóstico do padrão `Constraint` e pagamento parcial do DEBT-45

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/DEBT.md` — entrada DEBT-45 (em aberto; ficará
  parcialmente pago).
- `00_nucleo/adr/typst-adr-0036-*.md` — princípio de atomização
  progressiva (já em vigor desde Passo 91.5).
- `00_nucleo/adr/typst-adr-0033-*.md` — paridade funcional com
  vanilla.
- `01_core/src/entities/world_types.rs` — `Route<'a>` com os 4
  métodos `check_*_depth`. Comentário actual sobre o override
  `<Route<'static> as Validate>::Constraint`.
- `01_core/src/rules/eval.rs` — funções que recebem
  `route: Tracked<'r, Route<'r>>` (refactor do Passo 92).
- Ficheiros de show rules e layout que podem precisar dos
  `check_*`.
- `lab/typst-original/` — pontos onde o vanilla chama
  `route.check_show_depth()`, `check_layout_depth`,
  `check_html_depth`.

Pré-condição: `cargo test` — 744 L1 + 174 L3 + 6 ignorados,
zero violations. Passo 92 concluído (DEBT-44 encerrado, `Route`
estrutural).

---

## Natureza deste passo

Passo único com duas tarefas distintas:

1. **Tarefa A** — Sub-tarefa de governança/documentação:
   produzir diagnóstico curto sobre o padrão
   `<T<'static> as Validate>::Constraint` descoberto no Passo 92
   ao encadear `Tracked` recursivamente. Fica em
   `00_nucleo/diagnosticos/` como referência para materialização
   futura de `Engine<'a>` e outros tipos `comemo-tracked` com
   auto-referência.

2. **Tarefa B** — Construção: ligar 3 dos 4 `check_*_depth`
   do `Route<'a>` aos pontos de chamada correctos do eval/show/
   layout. O quarto (`check_html_depth`) fica pendente até o
   pipeline HTML ser materializado. Paga parcialmente o DEBT-45
   — fecha-se totalmente quando o HTML for adicionado.

Justificação para combinar as duas: o diagnóstico (Tarefa A) é
curto (5-10 minutos) e a construção (Tarefa B) é localizada
(chamadas pontuais, não refactor estrutural). Separar em passos
independentes seria sobredimensionar o processo.

---

## Decisões formalizadas neste passo

- ADR-0033 (paridade funcional) — replicar chamadas vanilla.
- ADR-0036 (atomização progressiva) — funções de eval continuam
  a declarar dependências explicitamente; este passo adiciona
  verificações, não novo estado partilhado.
- DEBT-45 passa a **parcialmente pago** (não encerrado). Mantém
  critério de fecho completo para quando o HTML existir.

---

## Tarefa A — Diagnóstico do padrão `Constraint`

### A.1 — Contexto factual

O Passo 92 descobriu que encadear `Tracked<'a, T>` recursivamente
(como em `Route { outer: Option<Tracked<'a, Self>>, ... }`)
requer override explícito do parâmetro `Constraint`:

```rust
// Sem o override, não compila:
pub outer: Option<Tracked<'a, Self>>,

// Com o override, compila:
pub outer: Option<Tracked<'a, Self, <Route<'static> as Validate>::Constraint>>,
```

Este padrão não é óbvio e vai ser necessário para qualquer tipo
futuro que tenha referência a si próprio via `Tracked`, incluindo
potencialmente `Engine<'a>` quando materializado.

### A.2 — Conteúdo do diagnóstico

Produzir ficheiro
`00_nucleo/diagnosticos/diagnostico-constraint-tracked-recursivo-passo-93.md`
com:

1. **Contexto** — descrição de 3-5 linhas sobre o problema:
   auto-referência em tipos `#[comemo::track]` requer anotação
   explícita do `Constraint`.

2. **Manifestação** — snippet do `Route<'a>` antes (não compila)
   e depois (compila). Mensagem de erro típica do compilador
   (se o Claude Code a tiver capturado no Passo 92 ou conseguir
   reproduzi-la).

3. **Explicação** — o `Constraint` é um tipo associado da trait
   `Validate` que o `#[comemo::track]` gera. Cada tipo
   `comemo-tracked` tem o seu próprio `Constraint`. Quando o
   tipo tem campo com `Tracked<Self>`, o Rust infere um
   `Constraint` diferente por instância de lifetime, quebrando
   a cadeia. O override `<T<'static> as Validate>::Constraint`
   força o mesmo `Constraint` em todas as instâncias,
   permitindo o encadeamento.

4. **Quando aplicar** — lista de indicadores:
   - Tipo `#[comemo::track]` com campo `Tracked<Self>`.
   - Tipo `#[comemo::track]` que referencia outro tipo com
     `Tracked` e tem lifetime dependente.
   - Erros de compilação com mensagens sobre `Constraint` não
     corresponderem entre tipos.

5. **Referências** — `Route<'a>` em
   `01_core/src/entities/world_types.rs` (exemplo concreto no
   projecto); documentação do `comemo` 0.4.0 (se acessível);
   documentação vanilla se tiver o mesmo padrão.

6. **Candidatos futuros** — lista de tipos no projecto que
   provavelmente vão precisar deste padrão quando materializados:
   `Engine<'a>` (tem `route: Route<'a>`), potencialmente outros.

### A.3 — Tamanho alvo

40-80 linhas. É diagnóstico técnico pontual, não análise
arquitectural extensa.

### A.4 — Verificação da Tarefa A

```bash
ls -la 00_nucleo/diagnosticos/diagnostico-constraint-tracked-recursivo-passo-93.md
grep -c "^## " 00_nucleo/diagnosticos/diagnostico-constraint-tracked-recursivo-passo-93.md
wc -l 00_nucleo/diagnosticos/diagnostico-constraint-tracked-recursivo-passo-93.md
```

Esperado: ficheiro existe, ao menos 6 secções (uma por item do
A.2), tamanho entre 40 e 100 linhas.

---

## Tarefa B — Integrar `check_show_depth`, `check_layout_depth`, `check_call_depth`

### B.1 — Mapear pontos de chamada no vanilla

```bash
# Pontos onde o vanilla chama check_*_depth:
grep -rn "check_show_depth\|check_layout_depth\|check_html_depth\|check_call_depth" \
    lab/typst-original/ --include="*.rs" | head -30
```

Para cada um dos 3 métodos a integrar, identificar:
- **`check_show_depth`**: chamado onde no vanilla? Tipicamente em
  `apply_show_rules` ou equivalente (início do processamento de
  regras de show).
- **`check_layout_depth`**: chamado onde? Tipicamente no braço
  recursivo do layouter, antes de descer num sub-frame.
- **`check_call_depth`**: chamado onde? Em `apply_func` ou
  `apply_closure` quando a função é chamada. O cristalino já tem
  `EvalContext::check_call_depth` antigo — precisa decidir se
  substitui ou adiciona.

Relatar os pontos encontrados antes de prosseguir para B.2.

### B.2 — Decisão sobre `check_call_depth` duplicado

O cristalino tem duas implementações de limite de profundidade
de chamadas:

- `EvalContext::check_call_depth` — antigo, `max_call_depth: 250`
  (herdado do DEBT-3 via ADR).
- `Route::check_call_depth` — novo, `MAX_CALL_DEPTH: 80` (vanilla).

**Decisão a tomar**:

- **Opção 1** — Manter ambos. `EvalContext` para limite cristalino,
  `Route` para limite vanilla. Complexidade desnecessária.
- **Opção 2** — Remover `EvalContext::check_call_depth`, usar só
  `Route::check_call_depth`. Alinha com vanilla. **Recomendado**.
- **Opção 3** — Remover `Route::check_call_depth`, manter o
  antigo. Contradiz ADR-0033.

A recomendação é Opção 2. Se o Claude Code detectar que
`EvalContext::check_call_depth` é chamada de muitos pontos
(mais de 5), reportar antes de decidir — pode justificar deixar
o ajuste para sub-passo dedicado.

### B.3 — Adicionar chamadas nos pontos correctos do cristalino

Para cada um dos 3 `check_*_depth`:

#### `check_show_depth`

Localizar em `01_core/src/rules/eval.rs` (ou submódulo de show
rules) a função `apply_show_rules` ou equivalente. Adicionar:

```rust
route.check_show_depth()?;
```

Como **primeira** linha após a assinatura, antes de qualquer
outra lógica. A semântica é "antes de processar show rules,
verifica se não estamos demasiado fundo".

Se a função `apply_show_rules` não recebe `route` directamente
(não foi tier 1 nem tier 2 do Passo 92), é provável que receba
o `ctx` de onde se pode obter — mas **atenção**: pelo Passo 92,
o `route` deixou de estar no `EvalContext`. Se `apply_show_rules`
não recebe `route`, é sinal de que a sua assinatura precisa de
ser alterada para incluir `route: Tracked<'r, Route<'r>>`.

Reportar se a assinatura da função tem de ser alterada. Se sim,
é continuação do refactor do Passo 92; alteração mecânica.

#### `check_layout_depth`

Localizar em `01_core/src/rules/layout/` a função de layout
recursivo (provavelmente `layout` em `layout/mod.rs` ou o braço
que desce em sub-frames).

**Importante**: o layout é executado **depois** do eval. A
`Route<'a>` actual vive no eval e termina quando a avaliação
acaba. Duas opções:

- **Opção X** — Propagar o `Route` final do eval para o layout.
  Requer decidir como.
- **Opção Y** — Criar um `Route<'a>` separado para o layout,
  com a sua própria cadeia de chamadas.
- **Opção Z** — Usar o `Route` do eval passado como parâmetro;
  durante layout, `extend` cria novos frames para cada descida
  recursiva.

Verificar o que o vanilla faz — o `Engine<'a>` vanilla inclui
`route` e é passado ao layout. No cristalino, o `Engine<'a>`
é stub; portanto a estratégia exacta depende de como o layout
actual recebe contexto.

**Se a integração no layout revelar que a arquitectura actual
não o suporta naturalmente**, parar e reportar. Pode ser
preciso adiar `check_layout_depth` até o `Engine<'a>` ser
materializado.

#### `check_call_depth` (se Opção 2 em B.2)

Localizar `apply_func` ou `apply_closure`. Substituir chamada
actual a `EvalContext::check_call_depth` (se existir) por
`route.check_call_depth()?`.

Remover `EvalContext::check_call_depth` e o campo
`max_call_depth` do contexto (se não usado por outros sítios).

### B.4 — Testes

Adicionar ao menos **2 testes novos** que exercitam os limites:

- `show_depth_rejeita_regras_infinitas` — show rule recursiva
  (`#show heading: it => heading(it)`) retorna `Err` com
  mensagem de ciclo em vez de stack overflow. Teste análogo
  ao vanilla.
- `call_depth_rejeita_recursao_profunda` — closure que se chama
  recursivamente (ex: factorial sem caso base) retorna `Err`
  antes de stack overflow.

Se `check_layout_depth` foi integrado com sucesso, adicionar
3º teste:

- `layout_depth_rejeita_aninhamento_excessivo` — conteúdo com
  aninhamento muito profundo (ex: tabelas dentro de tabelas ×80)
  retorna `Err` em vez de crashar.

Se `check_layout_depth` não foi integrado (ver B.3), manter só
os 2 testes e nota no reporte.

### B.5 — Verificação da Tarefa B

```bash
# Contagem de testes (744 + 2 ou 3 novos):
cargo test --workspace 2>&1 | tail -10

# Linter:
cargo run --package crystalline-lint 2>&1 | tail -5

# Chamadas dos check_*_depth:
grep -n "check_show_depth\|check_layout_depth\|check_call_depth" \
    01_core/src/rules/eval.rs 01_core/src/rules/ \
    --include="*.rs" -r

# Se Opção 2 de B.2 foi aplicada: EvalContext::check_call_depth
# antigo removido:
grep -n "EvalContext::check_call_depth\|max_call_depth" \
    01_core/src/rules/eval.rs
```

Esperado:
- 746 ou 747 L1 (744 + 2 ou 3 novos testes).
- Zero violations.
- Chamadas dos 3 `check_*_depth` encontradas em pontos
  apropriados.
- Se aplicado: `EvalContext::check_call_depth` antigo zero
  resultados.

---

## Tarefa C — Actualizar DEBT-45 no `DEBT.md`

DEBT-45 **não é movido** para Secção 2 — fica parcialmente pago.
Actualizar o texto na Secção 1:

- Marcar `[x]` os checkboxes correspondentes aos 3 `check_*`
  integrados.
- Marcar `[ ]` no `check_html_depth` (permanece pendente).
- Adicionar secção "Estado actual" no final da entrada:

```markdown
### Estado actual (após Passo 93)

**Parcialmente pago.** 3 dos 4 `check_*_depth` integrados:

- [x] `check_show_depth` — chamado em `apply_show_rules`
  (ou equivalente cristalino).
- [x] `check_layout_depth` — chamado em [ponto integrado
  conforme Tarefa B.3, ou "adiado" se não foi integrado].
- [x] `check_call_depth` — chamado em `apply_func`/
  `apply_closure`, substitui `EvalContext::check_call_depth`
  antigo.
- [ ] `check_html_depth` — pendente. Integração só é possível
  quando o pipeline HTML for materializado (trabalho futuro).

Encerramento completo do DEBT-45 fica para quando o pipeline
HTML existir.
```

Ajustar o texto conforme a realidade do que foi integrado na
Tarefa B (ex: se `check_layout_depth` não foi possível agora, o
checkbox fica vazio e há nota explicando).

---

## Critérios de conclusão

**Tarefa A**:
- [ ] Ficheiro
      `00_nucleo/diagnosticos/diagnostico-constraint-tracked-recursivo-passo-93.md`
      existe com ao menos 6 secções e tamanho razoável.

**Tarefa B**:
- [ ] `check_show_depth` chamado em ponto apropriado do processo
      de show rules.
- [ ] `check_call_depth` chamado em `apply_func`/`apply_closure`;
      se Opção 2, `EvalContext::check_call_depth` antigo removido.
- [ ] `check_layout_depth` chamado **ou** nota explícita no
      reporte justificando adiamento.
- [ ] 2-3 testes novos que exercitam os limites passam.
- [ ] Teste E2E de ciclo de imports
      (`import_cycle_detectado_retorna_err_sem_panic`) continua
      a passar sem alteração.

**Tarefa C**:
- [ ] DEBT-45 actualizado com "Estado actual (após Passo 93)"
      e checkboxes correctos.
- [ ] DEBT-45 permanece na Secção 1 (parcialmente pago, não
      encerrado).

**Geral**:
- [ ] `cargo test` passa com 746 ou 747 L1 + 174 L3 + 6 ignorados.
- [ ] `crystalline-lint` → zero violations.
- [ ] Nenhum `unsafe` novo introduzido.
- [ ] Nenhum ADR alterado.

---

## Ao terminar, reportar

Tarefa A:
- Tamanho do diagnóstico (linhas).
- Confirmação de que as 6 secções foram preenchidas.

Tarefa B:
- Quantas chamadas de cada `check_*_depth` foram adicionadas e
  onde.
- Decisão tomada em B.2 (Opção 1, 2 ou 3). Se Opção 2, confirmar
  remoção do `check_call_depth` antigo.
- Se `check_layout_depth` foi ou não integrado, com justificação.
- Testes novos adicionados.
- Contagem final L1.

Tarefa C:
- Confirmação de actualização do DEBT-45.
- Estado final (quantos `[x]`, quantos `[ ]`).

Go/No-Go para Passo 94:
- **Go** se todas as tarefas concluídas, DEBT-45 parcialmente
  pago. Passo 94 a decidir em conversa — próximas prioridades
  são provavelmente:
  - Materializar `Style` ou `LazyHash` (dependências folha de
    `Styles`).
  - Materializar `Introspection` (desbloqueia `Sink`).
  - Novo ciclo de extracção do `EvalContext` guiado por
    ADR-0036 (próximo campo candidato: `styles` ou
    `show_rules`).
- **No-Go parcial** se `check_layout_depth` não foi possível e
  algum outro limite também não foi integrado. Nesse caso:
  - Pagar DEBT-45 ficaria ainda mais fragmentado (só 1-2 de 4).
  - Talvez reverter e adiar para quando o `Engine<'a>` for
    materializado (que alinha layout+eval num só contexto
    estrutural).
