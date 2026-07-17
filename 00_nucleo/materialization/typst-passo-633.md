---
# P633 — Auditoria sistemática de falhas silenciosas

> **Passo:** 633
> **Data:** 2026-07-09
> **Foco:** Ao longo desta conversa já foram encontradas várias falhas silenciosas — nota de rodapé descartada sem aviso (P595), fonte de fallback embutida inteira por um `.ok()?` que falha calado (P609), conteúdo de `#for` descartado (P538f). Cada uma foi encontrada por acidente, dentro de outro passo. Este passo procura de forma sistemática, no código inteiro, pelos padrões que costumam esconder este tipo de problema — antes de avançar para funcionalidades novas.
> **Tipo:** Sonda directa, ampla. Sem implementação neste passo — só encontrar e classificar.
> **Tamanho:** L. É uma varredura do projecto inteiro.
> **ADR-0108 EM VIGOR.** Uma falha silenciosa não é um bug pequeno — é um bug que ninguém sabe que existe até tropeçar nele por acaso, como já aconteceu três vezes nesta conversa.

---

## Contexto

Os três casos já confirmados têm um padrão comum: código que, perante um erro ou uma situação inesperada, escolhe um caminho alternativo sem avisar ninguém — nem um erro, nem um aviso, nem sequer um comentário a assinalar a decisão. O utilizador (ou quem lê o código depois) não tem forma de saber que algo correu de forma diferente do esperado.

---

## Sonda — padrões a procurar

### 1. `.ok()` a descartar o erro

```bash
grep -rn "\.ok()" 01_core/src/ 03_infra/src/ 02_shell/src/ 04_wiring/src/ --include="*.rs" | grep -v "test" | wc -l
grep -rn "\.ok()" 01_core/src/ 03_infra/src/ 02_shell/src/ 04_wiring/src/ --include="*.rs" | grep -v "test"
```

Para cada ocorrência: o erro descartado é genuinamente inofensivo (por exemplo, um `Option` que já era esperado ser `None` em certos casos), ou é uma falha real a ser escondida, como o caso do subsetter em P609?

### 2. Braços `_ => {}` ou equivalentes que descartam conteúdo/erro

```bash
grep -rn "_ => {}" 01_core/src/ 03_infra/src/ --include="*.rs"
grep -rn "_ => Ok(Value::None)" 01_core/src/ --include="*.rs"
grep -rn "_ => None" 01_core/src/ 03_infra/src/ --include="*.rs" | grep -v "test"
```

Já se confirmou que este padrão exacto escondeu o bug de `Expr::Escape`/`Expr::Shorthand`/`Expr::Linebreak` (P581) e o de `#for` (P538f). Procurar por mais casos do mesmo formato.

### 3. `let _ = ...` a descartar um `Result`

```bash
grep -rn "let _ = " 01_core/src/ 03_infra/src/ 02_shell/src/ 04_wiring/src/ --include="*.rs" | grep -v "test"
```

Confirmar, para cada um, se o valor descartado podia ser um erro relevante.

### 4. `unwrap_or_default()` / `unwrap_or_else` sem log nem aviso

```bash
grep -rn "unwrap_or_default()\|unwrap_or_else(|| " 01_core/src/ 03_infra/src/ --include="*.rs" | grep -v "test"
```

Um valor por defeito silencioso, quando o valor real falhou a calcular, pode esconder um problema da mesma forma que os três já encontrados.

### 5. `if let Ok(...)` sem braço `else` a tratar o erro

```bash
grep -rn "if let Ok(" 01_core/src/ 03_infra/src/ --include="*.rs" -A3 | grep -B3 "^--$" | grep -v "else"
```

(Este comando é aproximado — confirmar manualmente cada ocorrência relevante, dado que a detecção automática de "sem else" por grep é imprecisa.)

### 6. Funções que devolvem `Option<T>` em vez de `Result<T, E>` em pontos onde um erro seria informação útil

```bash
grep -rn "fn.*-> Option<" 01_core/src/rules/ 03_infra/src/export/ --include="*.rs" | grep -v "test"
```

Uma função que devolve `Option` esconde, por definição, a razão do `None` — confirmar se algum destes casos devia devolver `Result` com uma mensagem, em vez de silêncio.

---

## Classificação

Para cada ocorrência encontrada, classificar em três grupos:

1. **Inofensivo, confirmado** — o caminho alternativo é genuinamente esperado e não esconde um problema (por exemplo, um `Option` que representa "esta propriedade não foi definida", não um erro).
2. **Suspeito, precisa de teste** — parece seguir o mesmo padrão dos três casos já confirmados; precisa de um teste directo para confirmar se esconde alguma coisa.
3. **Confirmado como falha silenciosa** — testado, e confirma-se que há informação a desaparecer sem aviso.

---

## Critério de fecho da sonda

- [ ] Todos os seis padrões varridos no código inteiro (não só uma amostra).
- [ ] Cada ocorrência classificada num dos três grupos.
- [ ] Para o grupo "suspeito": pelo menos um teste directo por ocorrência, confirmando ou refutando.
- [ ] Lista final de "confirmado como falha silenciosa", com prioridade por gravidade (perda de conteúdo do utilizador > degradação de desempenho > detalhe cosmético).

---

## Decisão

Este passo não corrige nada. Produz a lista priorizada. Cada item da lista "confirmado" vira um passo próprio, com a mesma disciplina de sonda-causa-correcção já usada em todos os casos anteriores — não corrigido às pressas dentro desta varredura.

---

## Critério de fecho do passo

- [ ] Sonda completa, os seis padrões varridos.
- [ ] Todas as ocorrências classificadas.
- [ ] Casos suspeitos testados directamente, não deixados como suspeita.
- [ ] Lista final priorizada, pronta para virar passos de correcção.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p633.md`, com a lista completa, não um resumo.
- [ ] Inventário de disparidades actualizado, se algum caso novo for confirmado.
