# Diagnóstico P1484 — matriz do gate `bundle` antes e depois da materialização

**Estado:** baseline pré-gate preservado abaixo; P1484 materializado após confirmação do dono, constructor e target ainda abertos em P1485.
**Proveniência:** 2026-09-23, observações confirmadas até
`2026-09-23T20:26:55-03:00`, HEAD
`bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`. Os binários foram
compilados antes dos edits documentais P1484; nenhum source L1–L4 mudou
durante as sondas. Vanilla `/usr/local/bin/typst` SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
cristalino `target/debug/typst` SHA-256
`58236613b1ba682a55b23074f7500bbea2b21c0b06c989e0df0fca2bb75691c8`.
Pin vanilla: upstream/main `586e1bd43`. Comando-base:
`typst eval 'type(asset)' --format json [--features PERFIL]`.

## Medição e classificação

| Perfil | Vanilla `type(asset)` | Cristalino pré-P1484 | Classificação |
|---|---|---|---|
| default | erro gated `bundle` | `"function"` | divergente; sem crédito ao desligado |
| html | erro gated `bundle` | `"function"` | divergente; sem crédito ao desligado |
| a11y-extras | erro gated `bundle` | `"function"` | divergente; sem crédito ao desligado |
| html+a11y-extras | erro gated `bundle` | `"function"` | divergente; sem crédito ao desligado |
| bundle | `"function"` | `"function"` placeholder | Violated na chamada real |
| bundle+html | `"function"` | `"function"` placeholder | ativo, ainda não validado |
| bundle+a11y-extras | `"function"` | `"function"` placeholder | ativo, ainda não validado |
| bundle+html+a11y-extras | `"function"` | `"function"` placeholder | ativo, ainda não validado |

As oito células de `type(asset)` foram sondadas bilateralmente. No perfil
`bundle`, `repr(asset("x", "data"))` devolve
`"asset(path: \"/x\", data: bytes(4))"` no vanilla e
`"asset(\"x\")"` no cristalino. É inferência que o mesmo placeholder
produzirá essa divergência nas três combinações ativas: a implementação
não consulta as outras duas features. Uma chamada bilateral que varie por
combinação refutaria a inferência; não a promover a MATCH sem a sonda.

Controle de escopo: `{ let asset = 1; asset }` devolve `1` nos dois
compiladores sem feature, então o diagnóstico gated aplica-se ao global,
não a um homônimo local. `type(std.asset)` sem feature falha no vanilla
com `cannot access field` e resolve `"function"` no cristalino; com
`bundle` ambos resolvem função, mas a assinatura ainda diverge.

## Matriz de aceitação P1484

Sem `bundle`, o diagnóstico do global e de `std.asset` deve coincidir com
o gate medido, mas o perfil desligado não recebe crédito de paridade.
Com `bundle`, o placeholder antigo deixa de ser público; a rota permanece
RED/Unknown até P1485 implementar `asset(path, data)` e o target bundle.
`--features bundle` deve produzir um set efetivo distinto de default,
independente de HTML, a11y e formato. A matriz não conclui a
implementação do constructor; as células pós-gate e a proveniência
estão registradas abaixo após testes RED→GREEN e validação final.

## Medição pós-P1484 (2026-09-23T20:53:31-03:00)

Proveniência: HEAD `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`,
working tree não commitada; alterações exatas listadas no Passo 1484,
§6. Binário cristalino `target/debug/typst` SHA-256
`459e434d51e97039491134203b64b8ccd2dbbdb227706f142e54942b92874607`.
As oito sondas usaram o mesmo comando-base acima após `cargo build`.

| Perfil | Cristalino pós-P1484 `type(asset)` | Estado semântico |
|---|---|---|
| default | erro gated `bundle` | desligado, sem crédito de paridade |
| html | erro gated `bundle` | desligado, sem crédito de paridade |
| a11y-extras | erro gated `bundle` | desligado, sem crédito de paridade |
| html+a11y-extras | erro gated `bundle` | desligado, sem crédito de paridade |
| bundle | `unknown variable asset` | RED até P1485 |
| bundle+html | `unknown variable asset` | RED até P1485 |
| bundle+a11y-extras | `unknown variable asset` | RED até P1485 |
| bundle+html+a11y-extras | `unknown variable asset` | RED até P1485 |

Sem `bundle`, `type(std.asset)` também emite o gate de campo com os
hints; com `bundle`, o field está ausente. O homônimo local continua
acessível. O teste inicial falhou antes do código (placeholder resolvia);
depois, os testes direcionados e as suítes L1/L2 passaram.
