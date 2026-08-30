# P1194 — separar stdlib `state` da entidade State

**Estado:** EXECUTADO — GREEN EM 2026-08-25
**Baseline condicionado:** P1193 GREEN; V15=13.

Manter `compiler/stdlib/state.md` no módulo stdlib e criar `entities/state.md`
para a entidade. Criar `_nuclei/state/semantics.toml` apenas para identidade,
updates e observáveis partilhados; chamadas nativas e representação de domínio
ficam nos owners. Não mudar API, defaults ou fase de introspecção.

GREEN: V15 13→12, V26=0, V5 focal limpo, testes state/introspecção verdes,
build, sources inalterados fora dos headers, dry-run bloqueado por 12 V15.
Fechar `typst-p1194-saneamento-state-owners.md`.
