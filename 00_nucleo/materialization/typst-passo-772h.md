---
# P772h — Arqueologia: por que existem `header:`/`footer:` inventados e o código órfão de `align`+`place` em células

> **Passo:** 772h
> **Data:** 2026-07-16
> **Foco:** P772f encontrou dois achados sem correcção imediata: (1) `grid()`/`table()` têm parâmetros nomeados `header:`/`footer:` sem equivalente no vanilla, com conteúdo silenciosamente descartado; (2) um bloco de código não commitado, já presente no working tree antes de P772f começar, sem L0 correspondente, que diverge do vanilla (~13.5pt). Antes de decidir o que fazer com qualquer um dos dois, este passo investiga a origem de ambos — regra 9 do handoff ("registo, não reconstrução"), aplicada aqui como "entender antes de decidir", não como correcção.
> **Tipo:** Sonda arqueológica. Sem implementação — as decisões (§ do relatório P772f) ficam para depois deste passo.
> **Tamanho:** S/M — a Parte B pode não encontrar nada, dado o código ser não commitado.
> **Dependências:** P772f (achados originais, commit `2c7025a9a949ecafda8a2550c8c3692db3c9ad0b`).

---

## Parte A — Origem de `header:`/`footer:` como parâmetros nomeados

### Indício já encontrado (fora do repositório, em conversa anterior — confirmar contra o código real)

Uma conversa de planeamento anterior deste projecto ("Passo 141-156i") listava o passo **P156L "grid refino"** com o escopo: *"gutter, align, stroke, fill, inset, header/footer, colspan/rowspan"* — agrupando `header`/`footer` na mesma lista que atributos nomeados legítimos do vanilla (`gutter`, `align`, `stroke`, `fill`, `inset`). Isto é um indício de que `header`/`footer` foram tratados, no planeamento, como "mais um atributo do grid" sem verificar que no vanilla têm forma de API diferente (elementos-filho, não argumentos nomeados) — não é prova, é uma pista a confirmar contra o código real.

### Confirmar contra o repositório

```bash
git log --all --oneline -S "\"header\"" -- "01_core/src/rules/stdlib/layout.rs" | tail -20
git log --all --oneline -S "\"footer\"" -- "01_core/src/rules/stdlib/layout.rs" | tail -20
```

Identificar o commit exacto que introduziu `header:`/`footer:` como argumentos nomeados de `native_grid`/`native_table`.

```bash
find . -iname "*passo-156*" -o -iname "*p156l*" 2>/dev/null
find 00_nucleo -iname "*grid*" 2>/dev/null
```

Se o prompt/relatório desse passo existir no repositório (não só na conversa), lê-lo e confirmar se a decisão de usar argumentos nomeados para `header`/`footer` foi deliberada (com alguma justificação, mesmo que hoje pareça errada) ou se foi mesmo uma extensão não avaliada, herdada do agrupamento do planeamento.

### Registo na tabela de rastreabilidade (regra 9 — sem reescrever)

| Passo (numeração da época) | Commit | O que decidiu (de facto) |
|---|---|---|
| P156L "grid refino" (planeamento) | — (só conversa, confirmar se corresponde a um passo real no repo) | Agrupou `header`/`footer` com atributos nomeados legítimos, sem distinguir a forma de API do vanilla |
| (passo real de implementação, a confirmar) | a preencher | Implementou `header:`/`footer:` como argumentos nomeados, conteúdo descartado sem erro nem render |
| P772f | `2c7025a9a` | Achado do descarte silencioso, não corrigido, registado para decisão |
| P772h (este passo) | — | Arqueologia da origem |

---

## Parte B — Origem do código órfão não commitado

Este código não tem histórico de commit — `git log` não vai encontrar nada directamente. Procurar noutros lugares:

```bash
git stash list
git reflog --all | head -50
find / -newer 01_core/src/rules/layout/grid.rs -maxdepth 3 -iname "*.md" 2>/dev/null | grep -v /proc
ls -la 01_core/src/rules/layout/grid.rs
```

Confirmar a data de modificação do ficheiro e comparar com os timestamps dos passos mais recentes (P772-P772f) — o código pode ter sido escrito numa sessão anterior de Claude Code que não chegou a commitar nem a gerar relatório, ou pode ser resultado de uma tentativa manual do utilizador.

```bash
grep -B5 -A5 "P772f — aplicar align efectivo" 01_core/src/rules/layout/grid.rs
```

Ler o comentário completo à volta do bloco — pode conter pistas (data, intenção, referência a algum passo) que não foram citadas no relatório de P772f.

**Se nada for encontrado**: registar explicitamente que a origem não é rastreável (nem por git, nem por comentário, nem por L0), e que a decisão sobre reverter/formalizar tem de ser tomada só pelo mérito do código em si (diverge do vanilla ~13.5pt, sem L0), não pela intenção original — que se torna irrecuperável.

---

## Critério de fecho do passo

- [x] Commit de introdução de `header:`/`footer:` como argumentos nomeados identificado — `0af55e792` "Passo 222 - 224" (não era só planeamento em conversa: já estava formalizado no prompt/relatório do repositório, com a inconsistência plano-vs-implementação documentada).
- [x] Tabela de rastreabilidade da Parte A preenchida, sem reescrever passos antigos.
- [x] Origem do código órfão da Parte B investigada em todas as fontes plausíveis (stash, reflog, dangling commits, timestamps, comentários).
- [x] Não encontrada por git — registado explicitamente como não rastreável, com essa conclusão a informar (não substituir) a decisão de mérito sobre reverter/formalizar.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772h.md`.

---

## Próximo passo

Com a arqueologia feita, retomar as duas decisões pendentes de P772f (header/footer, código órfão) com o contexto adicional, e continuar a varredura da stdlib.
