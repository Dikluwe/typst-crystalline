---
# P713 — `Length / Length` (reconstrução retroactiva)

> **Passo:** 713
> **Data:** 2026-07-10 (reconstruído depois do facto)
> **Foco:** `Length / Length`, `Length / Int`, `Length / Float` em falta em `eval_binary_op`, bloqueando `cetz` em `canvas.typ:37-38`. Ver `00_nucleo/diagnosticos/paridade-producao-p713.md` para a sonda e implementação completas.

---

## Nota

Este prompt foi escrito depois da execução — "sem materialização prévia — instrução directa do utilizador", conforme o próprio relatório regista. Este ficheiro fecha o buraco na sequência numerada; não é uma nova sonda, é reconstrução do que já foi pedido e feito.

## Resumo do que foi pedido (reconstruído do relatório)

Implementar `Length / Length` seguindo o `Length::try_div` do vanilla (`layout/length.rs`), incluindo o gate de divisão por zero já existente para outros tipos numéricos, e as combinações `Length / Int`/`Length / Float`. Decidir explicitamente sobre as combinações `Ratio`/`Relative` mistas (implementar ou scope-out, com razão medida, não assumida).

## Resultado (já registado no relatório)

- Implementado com paridade exacta, incluindo mensagem de erro para o caso incomensurável.
- `Ratio`/`Relative` mistos: scope-out confirmado como não-alcançável por sintaxe de utilizador actual (`Ratio` não é produzível directamente).
- `cargo test --workspace` sem regressão, `crystalline-lint .` limpo.
- `cetz` avançou para o próximo bloqueio (`array.at()`).
