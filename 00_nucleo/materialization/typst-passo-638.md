---
# P638 — O vanilla tem falhas silenciosas nas mesmas áreas já auditadas?

> **Passo:** 638
> **Data:** 2026-07-09
> **Foco:** Toda esta conversa assumiu que "bater com o vanilla" é o critério de correcção. Mas o vanilla é código real, com os seus próprios padrões de supressão de erro. Se o vanilla tiver uma falha silenciosa no mesmo sítio onde o cristalino também a tinha, a comparação nunca a apanharia — os dois "concordariam", e isso pareceria confirmação de que está tudo bem. Este passo aplica a mesma auditoria de P633 ao código do vanilla, limitada às áreas já mexidas nesta conversa (eval, regras `#set`, layout, export), não ao projecto inteiro.
> **Tipo:** Sonda directa. Sem implementação de correcção no vanilla (não é código deste projecto) — só identificar onde o cristalino pode estar a herdar um problema por imitação, não por estar genuinamente certo.
> **Tamanho:** M. Limitado por âmbito às áreas já tocadas, não o vanilla inteiro.
> **ADR-0108 EM VIGOR.** Bater com o vanilla não é prova de estar certo — é prova de bater com o vanilla. As duas coisas só coincidem se o vanilla estiver certo nesse ponto.

---

## Âmbito

Não auditar o vanilla inteiro — seria um projecto à parte, fora do que esta conversa se propôs a fazer. Limitar às áreas já activamente comparadas e corrigidas nesta sequência:

- Avaliação de expressões e regras `#set` (equivalente a `01_core/src/rules/eval/`).
- Layout de texto, RTL, colunas (equivalente a `01_core/src/rules/layout/`).
- Exportação PDF, fontes, metadados (equivalente a `03_infra/src/export/`, `03_infra/src/font*`).

---

## Sonda

### Repetir os seis padrões de P633, agora no vanilla

```bash
grep -rn "\.ok()" lab/typst-original/crates/typst-eval/src/ lab/typst-original/crates/typst-library/src/model/ lab/typst-original/crates/typst-layout/src/ lab/typst-original/crates/typst-pdf/src/ --include="*.rs" | wc -l
grep -rn "_ => {}" lab/typst-original/crates/typst-eval/src/ lab/typst-original/crates/typst-library/src/model/ --include="*.rs"
grep -rn "let _ = " lab/typst-original/crates/typst-eval/src/ lab/typst-original/crates/typst-layout/src/ --include="*.rs"
grep -rn "unwrap_or_default()\|unwrap_or_else(|| " lab/typst-original/crates/typst-eval/src/ lab/typst-original/crates/typst-library/src/model/ --include="*.rs"
```

### Comparar directamente com os 23 casos já confirmados no cristalino

Para cada um dos 23 casos confirmados por P633/corrigidos em P634-637, verificar o equivalente no vanilla:

| Caso do cristalino (já corrigido) | Equivalente no vanilla existe? | Vanilla também falha em silêncio, ou já tinha erro claro? |
|---|---|---|
| `#break`/`#continue`/`#return` fora de contexto | `flow.rs:28-36` (já confirmado em P634) | Vanilla já tinha erro claro — não é caso de herança. |
| `#set page(numbering: 123)` tipo inválido | ? | ? |
| `counter.display` argumento inválido | ? | ? |
| Bibliografia — entrada sem key/título omitida | ? | ? |

Preencher a tabela para os 23 casos, confirmando se cada correcção feita no cristalino era mesmo uma divergência do vanilla (o cristalino estava a fazer pior do que ele), ou se por acaso o cristalino já reproduzia um comportamento que o vanilla também tem, silenciosamente.

### Testar directamente os casos onde a sonda encontrar um padrão suspeito no vanilla

Para qualquer ocorrência nova encontrada no vanilla (não coberta pelos 23 já conhecidos), testar directamente com o binário vanilla, para confirmar se é uma falha silenciosa real, não só um padrão de código que parece suspeito mas tem razão documentada (como muitos dos "Inofensivo" de P633).

---

## Critério de fecho da sonda

- [ ] Os seis padrões varridos nas três áreas do vanilla listadas.
- [ ] Os 23 casos já corrigidos no cristalino, verificados um a um contra o equivalente no vanilla — nenhum ficou por confirmar.
- [ ] Qualquer padrão novo encontrado no vanilla, testado directamente, classificado (inofensivo/suspeito/confirmado), com a mesma disciplina de P633.

---

## Decisão

Se algum dos 23 casos corrigidos no cristalino tinha, na verdade, o mesmo problema no vanilla: isto não desfaz a correcção do cristalino (produzir erro claro continua a ser melhor do que falhar em silêncio, mesmo que o vanilla também falhe) — mas muda a classificação de "correcção de paridade" para "melhoria além do vanilla", e vale a pena registar isso com precisão, não deixar como se fosse simplesmente "bater com o vanilla".

Se for encontrado um padrão novo, exclusivo do vanilla, que o cristalino não reproduz: não é para copiar o problema do vanilla para o cristalino. É para confirmar que a divergência actual (cristalino sem o problema) é uma coisa boa, não um erro de paridade a "corrigir" na direcção errada.

---

## Critério de fecho do passo

- [ ] Sonda completa, três áreas do vanilla varridas.
- [ ] Os 23 casos já corrigidos, verificados um a um.
- [ ] Qualquer padrão novo classificado com a mesma disciplina de P633.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p638.md`, com a tabela completa dos 23 casos.
- [ ] Nota nos relatórios de P634-637: se algum caso corrigido também existe no vanilla, actualizar a classificação de "correcção de paridade" para "melhoria além do vanilla", com a razão.
