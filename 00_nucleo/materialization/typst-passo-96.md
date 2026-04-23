# Passo 96 — Governança: abrir DEBT-46 e registar ADR-0037

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/README.md` — índice canónico dos ADRs.
- `00_nucleo/adr/typst-adr-0036-*.md` — ADR predecessora
  (atomização progressiva). O ADR-0037 é complementar.
- `00_nucleo/DEBT.md` — inventário actual. DEBT-39 (`active_guards`)
  foi encerrado no Passo 95; DEBTs em aberto incluem DEBT-1,
  DEBT-42, DEBT-43, DEBT-45.
- Ficheiro do ADR-0037 fornecido pelo utilizador (a colocar em
  `00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md`).

Pré-condição: `cargo test` — 746 L1 + 174 L3 + 6 ignorados,
zero violations. Passo 95 concluído.

---

## Natureza deste passo

Passo único de governança. Não altera código. Abre trabalho
arquitectural significativo:

- **Tarefa A** — Regista o ADR-0037 (`PROPOSTO`) que formaliza
  o princípio de coesão por domínio em ficheiros de L1. Actualiza
  o índice `README.md`.
- **Tarefa B** — Abre DEBT-46 no `DEBT.md` documentando os seis
  ficheiros candidatos à reestruturação, com critério de conclusão
  explícito e referência ao ADR-0037 como base.

Os sub-passos de reestruturação concreta (96.1–96.7) serão
enunciados separadamente, cada um a pagar parte do DEBT-46.

Regra absoluta: **não altera código**, **não altera outros ADRs**,
**não toca em ficheiros acima de 1000 linhas**. Altera apenas:

- `00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md` (novo).
- `00_nucleo/adr/README.md` (índice actualizado).
- `00_nucleo/DEBT.md` (nova entrada DEBT-46).

---

## Decisões formalizadas neste passo

Duas decisões novas:

- ADR-0037 `PROPOSTO` — princípio de coesão por domínio com 7
  regras operacionais. Promovido a `EM VIGOR` depois da primeira
  aplicação concreta (Passo 96.1, validado no Passo 96.2).
- DEBT-46 aberto — inventário concreto da reestruturação
  necessária, com 6 ficheiros listados e critério de fecho por
  ficheiro.

---

## Tarefa A — Registar ADR-0037

### A.1 — Colocar o ficheiro

O conteúdo do ADR-0037 vem de ficheiro fornecido pelo utilizador
(`typst-adr-0037-coesao-por-dominio.md`). Conteúdo esperado:

- Título: `⚖️ ADR-0037: Coesão por domínio — ficheiros limitados
  a uma responsabilidade clara`.
- `**Status**: ` `` `PROPOSTO` ``.
- `**Data**: 2026-04-22`.
- Sem campos de relação (não revoga, não é revisão).
- Secções: Contexto, Decisão (com 7 regras), Alternativas
  Consideradas, Consequências, Relação com ADR-0036, Plano de
  aplicação, Status `PROPOSTO` vs `EM VIGOR`, Referências.

Caminho final:
`00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md`.

### A.2 — Actualizar `00_nucleo/adr/README.md`

Cinco actualizações no índice. **Ordem sugerida: de baixo para
cima**, mantendo as contagens consistentes com a tabela.

#### A.2.1 — Adicionar linha na tabela "Estado por ADR"

Adicionar após a linha do ADR-0036:

```markdown
| 0037 | Coesão por domínio — ficheiros limitados a uma responsabilidade clara | `PROPOSTO` |
```

#### A.2.2 — Actualizar contagem por status

Na "Distribuição de status":

```
- `PROPOSTO`: 13 ADRs  →  14 ADRs
```

(Valor exacto depende da contagem actual; ajustar se o valor
actual diferir.)

#### A.2.3 — Actualizar total de ADRs

```
**Total**: 37 ADRs (36 números únicos...)  →  **Total**: 38 ADRs (37 números únicos...)
```

#### A.2.4 — Actualizar preâmbulo

No topo do `README.md`, linha de preâmbulo:

```
Lista os 37 ADRs em vigor  →  Lista os 38 ADRs em vigor
```

#### A.2.5 — Não adicionar à secção "Meta-regras em vigor"

A ADR-0037 está `PROPOSTO`, não `EM VIGOR`. Só entra nas
meta-regras quando for promovida (Passo 96.2). Se a secção tem
lista com marca de status, adicionar nota "ADR-0037 `PROPOSTO`,
pendente validação Passo 96.1–96.2"; caso contrário, saltar.

---

## Tarefa B — Abrir DEBT-46 no `DEBT.md`

### B.1 — Localização

Inserir na Secção 1 (DEBTs em aberto), após DEBT-45 (última
entrada actual).

### B.2 — Texto proposto

```markdown
## DEBT-46 — Ficheiros de L1 com coesão baixa por tamanho excessivo — EM ABERTO (Passo 96)

Seis ficheiros em `01_core/src/` excedem 1000 linhas e misturam
responsabilidades de domínios distintos. A análise realizada
antes do Passo 96 revelou:

| Linhas | Ficheiro |
|--------|----------|
| 3780 | `01_core/src/rules/eval.rs` |
| 2848 | `01_core/src/rules/layout/mod.rs` |
| 2255 | `01_core/src/rules/parse.rs` |
| 1806 | `01_core/src/rules/math/layout.rs` |
| 1711 | `01_core/src/rules/stdlib.rs` |
| 1250 | `01_core/src/rules/lexer/mod.rs` |

Total: 13.650 linhas em seis ficheiros. O `eval.rs` sozinho tem
368 ocorrências de padrões `match` sobre `Expr::`, `SyntaxKind::`
e `Value::` — é um dispatcher central para toda a lógica de
avaliação, misturando markup, matemática, closures, imports,
regras show/set e controlo de fluxo.

Este DEBT documenta o inventário concreto da reestruturação
necessária pela ADR-0037 (`PROPOSTO` após Passo 96; `EM VIGOR`
esperada após Passo 96.2).

### Motivação

A ADR-0036 (atomização progressiva) reduziu acoplamento dentro
de funções (Passos 92–95) mas não reduziu tamanho dos ficheiros.
O `eval.rs` diminuiu parcialmente após as extracções de `route`,
`styles`, `show_rules`, `active_guards`, mas continua acima de
3700 linhas.

A ADR-0037 complementa a ADR-0036 ao orientar decomposição
**entre ficheiros**, não só dentro de funções.

### Critério de conclusão

Cada ficheiro listado tem checkbox próprio:

- [ ] `eval.rs` reestruturado em submódulos por domínio (markup,
      math, modules, rules, closures, control_flow, bindings).
      Nenhum submódulo > 800 linhas ou cada excepção tem
      justificativa Regra 6 no topo. (Passo 96.1)
- [ ] ADR-0037 promovida de `PROPOSTO` para `EM VIGOR` ou
      ajustada conforme fricção encontrada. (Passo 96.2)
- [ ] `parse.rs` reestruturado por tipo de nó (markup, code,
      math, rules). (Passo 96.3)
- [ ] `stdlib.rs` reestruturado por módulo da stdlib (text,
      layout, math, calc, etc.). (Passo 96.4)
- [ ] `layout/mod.rs` reestruturado (orquestração, medição,
      emissão, sub-frames). (Passo 96.5)
- [ ] `math/layout.rs` reestruturado ou marcado como excepção
      Regra 6. (Passo 96.6)
- [ ] `lexer/mod.rs` reestruturado ou marcado como excepção
      Regra 6. (Passo 96.7)
- [ ] Verificação final: `find 01_core/src -name "*.rs" | xargs wc -l |
      sort -rn | head -10` mostra ficheiros acima de 800 linhas
      só com excepções Regra 6 documentadas. (Passo 96.8 ou
      encerramento implícito em 96.7)

### Dependências

Nenhuma técnica. O trabalho é mecânico (movimentação de código
por domínio), não requer decisões arquitecturais adicionais.

### Nota sobre escopo

O DEBT aplica-se apenas a `01_core/src/`. Ficheiros em `02_shell/`,
`03_infra/`, `04_wiring/` não estão no escopo — se excederem o
limite, abrir DEBT específico por camada.

### Nota sobre encerramento

Este DEBT fecha quando os 7 ficheiros listados tiverem sido
tratados (reestruturados ou marcados como excepção). Encerramento
parcial não é aceitável — a coerência do princípio ADR-0037
requer aplicação consistente.

Se um dos ficheiros resistir à decomposição por razões técnicas
descobertas durante a execução, registar como **excepção Regra 6**
em vez de deixar como dívida pendente. A Regra 6 existe
precisamente para estes casos.

---
```

### B.3 — Verificação após B

Confirmar que `DEBT-46` aparece na Secção 1, após DEBT-45.
Contagem de DEBTs em aberto depois deste passo: **aumenta em 1**.

---

## Critérios de conclusão

- [ ] Ficheiro
      `00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md`
      existe com `Status: PROPOSTO` e data correcta.
- [ ] `00_nucleo/adr/README.md` tem nova linha para ADR-0037,
      contagens actualizadas (PROPOSTO +1, total 37 → 38,
      números únicos 36 → 37), preâmbulo actualizado.
- [ ] Entrada DEBT-46 adicionada em `DEBT.md` Secção 1, após
      DEBT-45.
- [ ] Nenhum outro ADR alterado.
- [ ] Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/`,
      `04_wiring/` alterado.
- [ ] `cargo test` passa com os mesmos 746 L1 + 174 L3 + 6
      ignorados. `crystalline-lint` → zero violations.

---

## Ao terminar, reportar

Tarefa A:
- Caminho e tamanho do ADR-0037.
- 5 actualizações no `README.md` confirmadas.

Tarefa B:
- Linhas exactas onde DEBT-46 começa e termina em `DEBT.md`.
- Tamanho da entrada.

Verificação:
- Contagem de testes inalterada.
- Zero violations.

Go/No-Go para Passo 96.1:
- **Go** se ADR-0037 registado como `PROPOSTO`, DEBT-46 aberto
  com 8 checkboxes. Passo 96.1 = reestruturação do `eval.rs`
  em submódulos por domínio, primeira validação da ADR.
- **No-Go** se algum dos ficheiros `.md` tiver formato que não
  aceita directamente as alterações (ex: preâmbulo do README
  sem número explícito de ADRs). Reportar o formato encontrado
  e esperar orientação antes de prosseguir.
