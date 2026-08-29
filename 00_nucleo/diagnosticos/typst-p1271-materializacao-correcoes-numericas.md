# P1271 — materialização das correções numéricas causalmente provadas

**Estado:** EXECUTADO
**Baseline de entrada:** `e055a12faefb2b1773d140fc5fcae3874422cf45`
**Regime:** L0-first, RED→GREEN por causa; sem atestação de isolamento

## Resultado

Os clusters de divergência de produto certificados pelo P1270 foram fechados
sem alterar budgets, cap, quantização, fallback ou promoção produtiva.

- C1 corrigiu apenas o contrato do probe: componentes continuam `f32` e o
  offset passou a `f64`; o witness e o controle causal produziram saída
  byte-idêntica.
- C2 materializou o despacho Luma D65 → XYZ → Oklab do `palette 0.7.6`,
  incluindo clamp de `l` de `FromColor`. A medição de fonte refutou a descrição
  inicial mais cômoda de “sRGB → Oklab genérico”.
- C3 preservou os offsets efetivos em `f64` na entrada de `repeat` e no
  accessor de linguagem. O RED descobriu dois estreitamentos seriais, C3a e
  C3b, selados e atacados separadamente.
- C4 preservou o `Ratio(f64)` do sample Radial até a formação dos pesos `f32`
  nos espaços Oklab e Linear RGB. Conic e os demais espaços ficaram fora.

## Reexecução

O causal P1270 reexecutou 24 fixtures e 28 métricas congeladas. Todas as
variantes serializadas e controles ficaram `Preserved`; somente o
contrafactual `offset-f64-raw`, que omite a serialização normativa do produto,
manteve oito p95 acima do budget e não foi usado como sucesso nem falha do
contrato.

No envelope P1266, grafo, numérico e raster passaram em 96/96; os 192 recibos
de determinismo passaram e 24/24 mutantes foram rejeitados. Os quatro pares
continuam `Unknown-generalization` porque os gates de custo/validade preservam
40 fixtures como válidos e 56 como violados. Isso não reabre os clusters
numéricos e não autoriza mudar o cap.

P1234 e P1236 foram executados duas vezes e deram artefatos byte-idênticos:
P1234 manteve 15 fixtures, 30 probes e zero divergência pública; P1236 manteve
42 pares, luminância máxima zero e zero `Unknown`. P1237 reproduziu o baseline
congelado e P1264 repetiu o certificado focal: os quatro gates pair-local
passaram como `Preserved-fragment`, sem promoção produtiva.

## Ataques e gates finais

Quatro mutantes focais foram rejeitados: constante D65 errada, narrowing na
entrada de `repeat`, narrowing no accessor `stops()` e narrowing do argumento
Radial. O workspace passou na reexecução completa; a primeira execução teve
um único timeout do watcher, que passou isoladamente e na repetição integral.
Também passaram build do workspace, formato, diff-check, V1/V5/V15/V26 e o
lint Tekt completo com zero violações.

Contrato, testes, implementação, ataques e veredito foram executados pela
mesma autoridade `/root` no mesmo workspace. Portanto o veredito proporcional
é: **EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO**.
