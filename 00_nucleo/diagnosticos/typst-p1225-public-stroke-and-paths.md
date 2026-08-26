# P1225 — allowlist pública de Stroke e presença explícita

O dono aprovou somente `Value::Stroke -> string JSON` e manteve proibido o
fallback genérico, salvo novo tipo medido no vanilla. O RED focal retornou
`cannot serialize value to JSON`. A implementação expôs um formatter L1
estreito de Stroke e adicionou uma branch nominal no serializer L2. O teste
shell passou; Stroke em raw e Color em JSON continuam exit 1.

O primeiro A/B revelou que a entidade resolvida perdia a presença sintática:
`stroke(paint: red)` ganhava espessura implícita e miter 2 perdia o `.0`.
Sondas adicionais confirmaram que o vanilla distingue `stroke()` de paint ou
thickness explicitamente fornecidos e conserva até defaults explícitos.

Após o gate humano, `StrokeFields` passou a conservar separadamente presença
explícita e valores resolvidos, sem alterar os valores consumidos pelo layout.
No estado não commitado de `0a0fabe05cd001802c6dfcfffb183f7651ab17d1`, medido
em 2026-08-26T17:59:45-03:00 com as alterações de `git diff HEAD --stat`, os
nove casos públicos ficaram byte-idênticos ao vanilla ratificado: default,
paint-only, thickness-only, cap, join, dois dash, miter e default explícito.
Stroke em raw e Color em JSON continuam rejeitados.

Stroke público fica `Preserved`. Paths gerais continuam `PARTIAL`: o comparador
analítico de P1222 permanece válido, mas arcos sem normalização endpoint→center
continuam `Unknown`; este passo não promove aproximação visual a paridade.

Resultado proporcional:

```text
PUBLIC COMPLEX STROKE PRESERVED — GENERAL PATH FRONTIER PARTIAL
```

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.
