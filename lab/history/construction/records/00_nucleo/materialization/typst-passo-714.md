---
# P714 — `array.at(index, default:)` (reconstrução retroactiva)

> **Passo:** 714
> **Data:** 2026-07-10 (reconstruído depois do facto)
> **Foco:** `array.at()` em falta no dispatcher de métodos de `Array`, bloqueando `cetz` em `aabb.typ`. Ver `00_nucleo/diagnosticos/paridade-producao-p714.md` para a sonda e implementação completas.

---

## Nota

Este prompt foi escrito depois da execução — "sem materialização prévia — instrução directa do utilizador", conforme o próprio relatório regista. Este ficheiro fecha o buraco na sequência numerada.

## Resumo do que foi pedido (reconstruído do relatório)

Implementar `array.at(index, default:)` seguindo `Array::at`/`locate_opt` do vanilla — índice negativo conta do fim, `default:` usado se fora de limites, erro exacto se fora de limites sem `default`.

## Resultado (já registado no relatório)

- Braço em falta identificado com precisão (`try_dispatch_collection_method` não tinha entrada para `(Array, "at")`).
- Implementado com paridade exacta, incluindo mensagem de erro verbatim.
- `cargo test --workspace` sem regressão, `crystalline-lint .` limpo.
- `cetz` avançou para o próximo bloqueio (atribuição por desestruturação).
