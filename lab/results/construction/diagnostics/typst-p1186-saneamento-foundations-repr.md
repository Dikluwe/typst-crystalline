# P1186 — fechamento

`compiler/eval/repr.rs` ganhou owner próprio e `foundations.md` ficou restrito
ao hub, sem Núcleo nem mudança funcional. V15 21→20, V26=0; os consumers focais
ficaram limpos após respeitar o hash efetivo legado de `foundations.md`.
Hashes de código: `333feb71` e `c19e6bc9`. Testes de repr: 51 passed.
