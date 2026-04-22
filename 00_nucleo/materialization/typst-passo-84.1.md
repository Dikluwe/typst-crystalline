# Passo 84.1 — Limpeza textual do `DEBT.md` (DEBT-1, DEBT-8, DEBT-21, DEBT-23)

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/DEBT.md` — Ficheiro alvo único. Localização após o Passo 83.5.
- Relatório do Passo 83.5 — Secção 3 "DEBTs com divergência entre ficheiro
  e código".

Pré-condição: `cargo test` — 901 testes (732 L1 + 169 L3, 6 ignorados
pré-existentes), zero violations. Passo 83.5 concluído: `DEBT.md` em
`00_nucleo/`, três secções, texto verbatim das entradas.

---

## Natureza deste passo

Este é um passo de **edição textual pura** em `00_nucleo/DEBT.md`. Não
toca em código-fonte, não altera testes, não fecha DEBTs.

O que o Passo 83.5 deixou intencionalmente por fazer (para não violar a
regra "preservar texto verbatim"), este passo executa agora sob
autorização explícita:

1. DEBT-1: riscar pendências já resolvidas por outros DEBTs.
2. DEBT-8: riscar `MathAlignPoint` da lista de pendências.
3. DEBT-21: remover o bloqueio "requer Rust ≥ 1.85" (Rust 1.92 em uso).
4. DEBT-23: consolidar as duas entradas duplicadas em uma única.

Nenhum DEBT muda de secção neste passo. Nenhum DEBT é encerrado neste
passo. O fecho efectivo de DEBT-21 (com mudança de código) fica para o
Passo 84.3.

---

## Tarefa 1 — DEBT-1: riscar pendências resolvidas

Localização: Secção 1 de `00_nucleo/DEBT.md`, entrada `DEBT-1 — StyleChain`.

### Alterações

Na subsecção `### Pendente` de DEBT-1, remover duas linhas:

- `Scoping de #set por bloco { }` — resolvido implicitamente pelo DEBT-7
  (save/restore de `ctx.styles` em `Expr::CodeBlock`, `Expr::ContentBlock`
  e `apply_closure`). Verificado na auditoria em `eval.rs:343`, `:635`,
  `:1094`.
- `#show rules` — resolvido pelos DEBT-19/20 (Passos 68-70). Verificado
  em `eval.rs:706` (Expr::ShowRule) e `:1408` (apply_show_rules).

Manter as restantes pendências (são dívida real em aberto):

- `Propriedades adicionais (fill, font-family, weight numérico, etc.)`
- `Paridade total com o sistema de styles do original`
- `Remover os wrappers Content::Strong/Emph do layout quando eval os tiver
  totalmente substituído`

Na subsecção `### Divergência intencional`, **manter intacta** a linha
`StyleChain não integrada com #show rules (Passo futuro)`. Esta linha
descreve um estado histórico da decisão de design, não uma pendência
activa. A integração com `#show` aconteceu pelos DEBT-19/20 de forma
independente da `StyleChain` — esta continua a não ser a fonte dos estilos
aplicados por show rules. A divergência intencional permanece válida.

### Adicionar uma nota de proveniência

No fim da entrada DEBT-1, adicionar um parágrafo curto:

```markdown
### Nota — actualização no Passo 84.1

Duas pendências originais ("Scoping de #set por bloco" e "#show rules")
foram riscadas por terem sido resolvidas implicitamente por outros DEBTs
(DEBT-7 e DEBT-19/20 respectivamente). A auditoria do Passo 83.5
confirmou a presença do código correspondente em `eval.rs`. As
pendências remanescentes (propriedades adicionais, paridade, wrappers)
continuam em aberto.
```

---

## Tarefa 2 — DEBT-8: riscar `MathAlignPoint` da lista de pendências

Localização: Secção 1, entrada `DEBT-8 — Motor de equações`.

### Alterações

Na subsecção `**Ainda pendente**:`, a linha actual é:

```markdown
- `MathPrimes`, `MathAlignPoint`
```

Substituir por:

```markdown
- `MathPrimes` (parseado e evaluado em `eval.rs`, sem lógica de kern/posição no layouter)
```

Razão: `MathAlignPoint` está totalmente implementado (`math/layout.rs:143/180/322`,
`eval.rs:1357`, `layout/mod.rs:478`), conforme verificado na auditoria do
Passo 83.5. `MathPrimes` está parcialmente implementado — fica com nota
explícita do estado actual em vez de aparecer ao lado de algo já resolvido.

### Nota de proveniência

No fim da entrada DEBT-8, adicionar:

```markdown
### Nota — actualização no Passo 84.1

`MathAlignPoint` foi removido da lista de pendências — verificação no
Passo 83.5 confirmou implementação completa em `math/layout.rs`,
`eval.rs` e `layout/mod.rs`. A entrada de `MathPrimes` foi clarificada
para indicar o estado parcial (parseado mas sem lógica de layout
dedicada).
```

---

## Tarefa 3 — DEBT-21: remover o bloqueio de versão

Localização: Secção 1, entrada `DEBT-21 — Resolução de NodeKind por string`.

### Alterações

Texto actual:

```markdown
## DEBT-21 — Resolução de NodeKind por string — **MITIGADO (Passo 70)**

`Func::name()` continua a ser usado. Aliasing não é detectado. Resolução
completa por ponteiro adiada (requer Rust >= 1.85 para `fn_addr_eq` estável).
Mensagens de erro melhoradas para closures anónimas (None → Err explícito).
```

Substituir por:

```markdown
## DEBT-21 — Resolução de NodeKind por string — **MITIGADO (Passo 70), desbloqueado (Passo 84.1)**

`Func::name()` continua a ser usado. Aliasing não é detectado. Resolução
completa por ponteiro (`fn_addr_eq`) está agora disponível — a toolchain
do projecto usa Rust 1.92 (verificado no Passo 83.5). A mitigação pode
ser substituída por resolução definitiva num passo de código dedicado
(candidato FÁCIL para o Passo 84.3).

Mensagens de erro melhoradas para closures anónimas (None → Err explícito).
```

O estado continua "MITIGADO". Neste passo apenas remove-se o bloqueio
técnico do texto — a substituição do código de `Func::name()` por
`fn_addr_eq` fica para o Passo 84.3.

---

## Tarefa 4 — DEBT-23: consolidar entradas duplicadas

Localização: Secção 2 (encerrados) de `00_nucleo/DEBT.md`. Ambas as
entradas DEBT-23 estão nesta secção, consecutivas.

### Alterações

O ficheiro tem actualmente duas entradas consecutivas:

```markdown
## DEBT-23 — Travessia múltipla em apply_show_rules (Passo 69)

`apply_show_rules` percorre a árvore uma vez por `ShowRule` activa: O(R×N)
com R regras e N nós. Resolução: `map_content` chamado uma única vez, com
todas as regras testadas por nó dentro da closure. Requer mudança de
assinatura de `map_content` para aceitar uma lista de regras em vez de uma
closure genérica.

---

## DEBT-23 — Travessia múltipla O(R×N) — **ENCERRADO (Passo 70)** ✓

`apply_show_rules` chama `map_content` uma única vez para todas as regras
`NodeKind`. Dentro da closure bottom-up, todas as regras activas são testadas
por nó antes de prosseguir — custo reduzido de O(R×N) para O(N).
```

Substituir as duas entradas por uma única entrada consolidada:

```markdown
## DEBT-23 — Travessia múltipla em apply_show_rules — **ENCERRADO (Passo 70)** ✓

**Registado no Passo 69.**

`apply_show_rules` percorria a árvore uma vez por `ShowRule` activa: O(R×N)
com R regras e N nós. Resolução planeada: `map_content` chamado uma única
vez com todas as regras testadas por nó dentro da closure, implicando
mudança de assinatura de `map_content` para aceitar uma lista de regras em
vez de uma closure genérica.

**Resolvido no Passo 70.** `apply_show_rules` chama `map_content` uma
única vez para todas as regras `NodeKind`. Dentro da closure bottom-up,
todas as regras activas são testadas por nó antes de prosseguir — custo
reduzido de O(R×N) para O(N).
```

A consolidação preserva ambos os registos (o problema descrito no Passo
69 e a resolução no Passo 70) numa única entrada com a estrutura
"Registado → Resolvido" que é usada noutros DEBTs do ficheiro (por
exemplo DEBT-7, DEBT-10, DEBT-11).

### Nota sobre posicionamento

A entrada consolidada fica no mesmo ponto onde as duas entradas
originais estavam. Não mover para outro ponto da Secção 2.

---

## Tarefa 5 — Verificação final

Depois de aplicar as quatro tarefas, executar:

```bash
# Contagem de DEBTs em cada secção (deve manter-se igual ao Passo 83.5
# excepto pela consolidação de DEBT-23: 27 → 26 entradas na Secção 2).
grep -c "^## DEBT-" 00_nucleo/DEBT.md      # total único
awk '/^## Secção 1/,/^## Secção 2/' 00_nucleo/DEBT.md | grep -c "^## DEBT-"
awk '/^## Secção 2/,/^## Secção 3/' 00_nucleo/DEBT.md | grep -c "^## DEBT-"

# Confirmar que DEBT-23 aparece uma única vez.
grep -c "^## DEBT-23" 00_nucleo/DEBT.md    # esperado: 1 (era 2)

# Confirmar que nenhum código-fonte foi alterado.
git diff --stat -- '01_core/**/*.rs' '03_infra/**/*.rs'    # esperado: sem alterações

# Regressão de testes e linter.
cargo test
crystalline-lint 00_nucleo/DEBT.md
crystalline-lint .
```

---

## Critérios de conclusão

- [ ] DEBT-1: linhas "Scoping de `#set` por bloco" e "`#show` rules"
  removidas da subsecção `### Pendente`. Nota de proveniência
  adicionada.
- [ ] DEBT-1: linha sobre `#show` rules na subsecção
  `### Divergência intencional` **mantida intacta**.
- [ ] DEBT-8: linha `MathPrimes, MathAlignPoint` substituída por linha
  que menciona apenas `MathPrimes` com clarificação de estado parcial.
  Nota de proveniência adicionada.
- [ ] DEBT-21: cabeçalho actualizado para
  `MITIGADO (Passo 70), desbloqueado (Passo 84.1)`. Parágrafo sobre
  Rust 1.92 substituiu o bloqueio de versão. Menção explícita de que a
  substituição de código fica para o Passo 84.3.
- [ ] DEBT-23: duas entradas consecutivas substituídas por uma única
  entrada consolidada com estrutura "Registado no Passo 69 → Resolvido
  no Passo 70".
- [ ] Contagem de entradas DEBT-23: exactamente 1 (era 2).
- [ ] Contagem total de entradas DEBT-\*: diminuiu em 1 face ao Passo
  83.5 (26 em vez de 27 em Secção 2; 13 em Secção 1; 1 em Secção 3).
- [ ] Nenhuma linha de código-fonte alterada. `git diff --stat` para
  `*.rs` mostra zero ficheiros tocados.
- [ ] `cargo test` passa com os mesmos números do Passo 83.5
  (732 L1 + 169 L3, 6 ignorados).
- [ ] `crystalline-lint` sem violações.

---

## Ao terminar, reportar

- Número de entradas em cada secção depois das alterações. Comparar com
  os números do Passo 83.5 (Secção 1: 13, Secção 2: 27, Secção 3: 1).
- Confirmação de que DEBT-23 aparece uma única vez.
- Confirmação de que nenhum código-fonte foi tocado (output de
  `git diff --stat` para ficheiros `.rs`).
- Confirmação de que os números de testes não mudaram.

Não há Go/No-Go para o sub-passo seguinte — o Passo 84.2 (DEBT-38,
cache de sub-frames no Grid) não depende deste passo textual, mas é
conveniente executar este primeiro para que o `DEBT.md` esteja limpo
quando o Passo 84.3 vier fechar efectivamente DEBT-21.
