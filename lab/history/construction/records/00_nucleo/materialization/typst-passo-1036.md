# Passo 1036 — Numeração hierárquica de `heading` perde níveis no corpo

**Tipo**: Investigar → gate → corrigir. Achado #4 do P1031, prioridade Alta — afecta
qualquer documento com `#set heading(numbering: "1.")` e mais de um nível de heading, que
é um caso de uso comum.
**Medição do P1031**: `#set heading(numbering: "1.")` + `=`/`==`/`===`:
- Vanilla: `1.` / `1.1.` / `1.1.1.`
- Cristalino: `1.` / `1.` / `1.`

**O outline acerta** (per o relatório) — a numeração hierárquica funciona correctamente
no índice, só o corpo do documento perde os níveis. Isto é um dado importante: o
mecanismo correcto **existe** algures no código (o outline usa-o), só não está a ser
usado no render do corpo.

**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1033. **Relação com o Passo 1019**
(heading avança contador só com `numbering:` activo): confirmar se este achado é o mesmo
código tocado nesse passo, ou área adjacente — não presumir, verificar.

---

## Fase A — Confirmar por que o outline acerta e o corpo não

1. Localizar o código que gera a numeração no **outline** — como calcula `1.1.1.`
   correctamente (provavelmente usa o `CounterKey::Selector` por nível, ou soma dos
   contadores pai).
2. Localizar o código que gera a numeração no **corpo** (o prefixo visível antes do texto
   do heading) — confirmar se usa o mesmo mecanismo ou um caminho diferente.
3. Se forem caminhos diferentes: essa é a causa — o corpo precisa de ser corrigido para
   usar o mesmo cálculo hierárquico que o outline já usa correctamente.
4. Confirmar se isto tem sobreposição com o trabalho do Passo 1019 (gate de
   `numbering_active`) — se o `numbering_active` só controla se avança, mas não como se
   compõe o número hierárquico, são preocupações distintas e não há conflito.

## Fase B — Gate (ADR-0127, categoria 2/3)

```
Dado #set heading(numbering: "1.") com = , == , === aninhados
Quando renderizado
Então corpo mostra 1. / 1.1. / 1.1.1., batendo com vanilla e com o outline já correcto

Dado o mesmo documento
Quando #outline() é gerado
Então continua correcto (guarda de não-regressão — não quebrar o que já funciona)
```

Não-regressão: todos os testes de heading/outline/numbering existentes.

## Fase C — Implementar e validar

```
crystalline-lint .
cargo test --workspace
```
Decalque visual — qualquer documento com headings numerados de múltiplos níveis muda de
aparência (correctamente).

---

## Resultado esperado

Corpo do documento usa o mesmo mecanismo de numeração hierárquica que o outline já usa
correctamente. Sem regressão no outline.
