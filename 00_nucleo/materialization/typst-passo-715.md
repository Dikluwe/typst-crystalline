---
# P715 — Desestruturação, atribuição por desestruturação, atribuição simples/composta (reconstrução retroactiva)

> **Passo:** 715
> **Data:** 2026-07-10 (reconstruído depois do facto)
> **Foco:** `destructuring assignment` era stub deliberado; a sonda revelou que `let (a, b) = ...` já estava quebrado (só ligava o primeiro nome) e que atribuição simples (`x = 5`) não tinha braço nenhum. Ver `00_nucleo/diagnosticos/paridade-producao-p715.md` para a sonda e implementação completas.

---

## Nota

Este prompt foi escrito depois da execução — "sem materialização prévia — instrução directa do utilizador", conforme o próprio relatório regista. Este ficheiro fecha o buraco na sequência numerada.

## Resumo do que foi pedido (reconstruído do relatório)

"Avança para a desestruturação e corrija o bug" — pedido directo do utilizador, sem sonda prévia escrita por mim. A sonda subsequente (feita durante a execução) revelou o alcance maior do que o bloqueio reportado sugeria.

## Resultado (já registado no relatório)

- `let (a, b) = ...`: bug pré-existente confirmado e corrigido (só o primeiro nome era ligado, ao valor inteiro, não ao elemento correspondente).
- Atribuição simples/composta (`x = v`, `x += v`, etc.): implementada do zero.
- Atribuição por desestruturação (`(a, b) = expr`, distinta de `let`): implementada.
- Mecanismo de mutação de bindings existentes (`Scopes::get_mut`) construído como base partilhada para as três correcções.
- `cargo test --workspace` sem regressão (21 testes novos), `crystalline-lint .` limpo.
- `cetz` avançou para o próximo bloqueio (`Access` genérico — `FieldAccess`/`FuncCall` como alvo de atribuição, ex. `arr.at(i) = valor`), correctamente identificado como scope-out, não implementado neste passo.
