# P1263 — máximos interiores não saturados de cor e alpha

**Estado:** EXECUTADO — DEZ FRONTEIRAS FECHADAS, PROMOÇÃO NÃO APLICADA  
**Regime:** protocolo Tekt completo, executado sem atestação de isolamento

## Causa medida e owner

As dez fronteiras congeladas não eram causadas por falta de refinamento. O
candidato e o vanilla já produziam as mesmas contagens nos stopsets ainda
abertos: 36 stops em `alpha-mid` e 40 em `alpha-last`, tanto para Linear como
para Radial. A primeira divergência estava no writer SVG: LinearRgb aplicava
`Ratio::repr` apenas ao último intervalo, enquanto o vanilla o aplica a todos
os offsets adaptativos.

A reconstrução com a ordem aritmética exata do P1262 refutou a restrição
provisória registada naquele passo. Generalizar `Ratio::repr` para todos os
offsets adaptativos LinearRgb fecha as quatro fronteiras de cor e as seis de
alpha sem reabrir `alpha-first`. O único owner produtivo alterado foi
`03_infra/src/export/svg.rs`; `adaptive.rs`, `Color`, o seletor PDF P274,
limiar, cap e API pública permaneceram intactos. O L0 SVG foi atualizado antes
da implementação e o fluxo seguiu continuamente conforme ADR-0127.

## RED → GREEN e envelope

O teste RED confirmou 36 stops no witness `alpha-mid`, mas encontrou offsets
interiores crus, incluindo `0.566874981` e `0.606249988`. Depois da alteração
mínima do seletor de representação, Linear e Radial passaram a serializar os
36 offsets pela forma percentual vanilla; o teste ficou GREEN.

As dez fronteiras P1260 fecharam simultaneamente máximo e p95. Nos LinearRgb
`alpha-mid`, os observáveis finais foram `color_max=0.006397917149869239`,
`color_p95=0.0039718205474433286`,
`alpha_max=0.0038690223152602066` e
`alpha_p95=0.0027857516067549826`. Nos `alpha-last`, foram
`color_max=0.005767424895772103`, `color_p95=0.004064092154347521`,
`alpha_max=0.003760411707839584` e
`alpha_p95=0.002629030015028233`. Cor premultiplicada e alpha foram medidos
separadamente em ponto flutuante antes de `u8`.

## Custo, regressão e elegibilidade

O custo incremental é zero stops e zero decisões adaptativas em todas as 24
fixtures. A reexecução P1237 passou 24/24 sem regressão; Linear/Oklab,
Radial/Oklab, Linear/LinearRgb e Radial/LinearRgb medem 6/6 gráfico e 6/6
numérico. Os quatro pares são elegíveis, mas nenhuma promoção foi aplicada:
alterar o fallback por defeito exige passo próprio sob ADR-0127. O caso opaco
permanece `Unknown` e não conta como sucesso.

## Ataques, determinismo e gates

Foram rejeitados 9/9 mutantes executáveis: ignorar alpha, observar alpha só
no RGB premultiplicado ou após `u8`, quartos globais, pontos guiados pela
aprovação, máximo sem p95, p95 sem máximo, partilha sem execução individual e
remoção de coincidência/right-continuity. `mutation_score=1.0`; `Unknown` foi
excluído do numerador. Duas execuções completas produziram os mesmos sete
artefatos byte a byte, e a ordem direta/inversa foi determinística.

Passaram o teste RED→GREEN focal, as regressões P1261/P1262, 44 testes do owner
SVG, `cargo fmt --check`, `cargo build`, `git diff --check` e
`crystalline-lint .` com zero violações. Os warnings Rust e informativos
V16–V20 preexistentes não são violações do linter.

Contrato, adversário, implementação e integração do veredito foram exercidos
sob a mesma autoridade `/root`. Os oráculos P1231 estavam congelados, mas não
há separação forte de capacidades. Veredito proporcional:
**EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO**.
