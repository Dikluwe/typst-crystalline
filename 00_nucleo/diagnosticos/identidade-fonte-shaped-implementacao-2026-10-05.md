# Fonte efetiva de TextShaped — materialização de 2026-10-05

A fonte que gerou os glifos agora acompanha o carrier até a exportação. Isso
corrige o árabe em ênfase que era desenhado com GIDs da regular na face Oblique.
A origem da aproximação por família foi documentada em P534; esta rodada
corrige a ligação perdida, sem apagar o significado da ênfase ou trocar seus
campos de estilo para induzir seleção de fonte.

## Alteração materializada

- `01_core/src/entities/layout_types.rs:418`: campo ratificado `TextShaped.font: Option<Font>`. Clone do recurso carregado com Arc existente; None preserva construções legadas. Tradução, rebasing, placement, footnotes e equações conservam o campo.
- `03_infra/src/shaper/cache.rs:27` e `shaper/shaping.rs:251`: o resultado memoizado conserva a fonte que produziu os glyphs. Miss e hit fornecem essa fonte ao carrier, sem resolver World novamente na emissão. O RED de carga contou quatro consultas à regular; o GREEN verifica no máximo duas para face/shape frios.
- `03_infra/src/export/font_binding.rs`: binding pela face original e pedido de eixos. Recursos derivados/subsets mantêm o índice associado à origem. Nomes/variantes iguais não tornam duas faces intercambiáveis. Cache local retém clone da fonte, valida bytes na primeira consulta e usa identidade da memória como atalho posterior; não conserva ponteiro de World entre sessões.
- Coleta por fonte distribui os itens antes de subset/CMap: GIDs de faces diferentes não são misturados por simples teste de faixa. Glyph matemático direto conserva seu contrato anterior; não foi acrescentado carrier de fonte a ele.
- PDF, SVG e PNG consomem a associação. SVG/PNG separam bytes originais dos bytes preparados da instância. O caminho existente de fonte única continua válido quando não precisa de instanciação VF. Assinaturas públicas de exportação e FontKey permanecem.

Nenhum unsafe novo, I/O em L1, cache global ou nova fase foi acrescentado.
O unsafe existente de CachedFace não foi removido nesta rodada. Claims
compartilhadas ficam no núcleo `00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml`;
32 consumers possuem seus owners próprios, com resselo restrito.

## Aprovação e proveniência

O humano confirmou o L0 original de layout_types com SHA-256
`f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409`.
[Aprovação](../materialization/identidade-fonte-shaped-aprovacao-2026-10-05.json).
O manifesto original permaneceu congelado como evidência do pacote proposto.
Alterações privadas subsequentes seguem fluxo contínuo ADR-0127.

Estado medido: working tree não commitada sobre
`06848cdcb5450f2c555626bf116bc7acd82bd82b`.
[execution.json.gz](identidade-fonte-shaped-implementacao-2026-10-05/execution.json.gz)
conserva UTC, diff HEAD --stat completo, status/untracked, fontes/L0 exatos,
logs e hashes. `before.json.gz`, `cache-before.json` e `seals.json` conservam
os estados e a linhagem; `first-candidate.json.gz` e `second-candidate.json.gz`
conservam as versões intermediárias. Os binários antes/depois são distintos
por SHA, sem depender da string --version.

Tekt final SHA-256: `7a089e7dc9f0189cd6fa999002ac5652ef05531bfb16d8e3cd3439ef08d9343c`.
Baseline Tekt SHA-256: `3159634c96c3008ffd2b904d2d6a0f3b5ac439717c71b708673eee20e1f8a8ee`.
Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Fontes DejaVu e bibliotecas da sonda têm hashes na auditoria inicial; o controle
VF usa o fixture Ubuntu Sans e o font path explícito registrado no comando.

## Verificação

Testes RED→GREEN de transporte, miss/hit e consulta redundante à fonte.
Testes de metadados iguais com faces distintas, novas alocações dos mesmos
bytes, None, desenho PNG/SVG contra a face correta e coordenadas explícitas
sobrepostas em fonte variável. Fonte carregada no carrier torna a exportação
independente de uma nova consulta ao catálogo do World.

Passaram 6528 testes do núcleo (5 ignorados), 1170 da infraestrutura e as suítes
de integração aplicáveis. Builds debug/release passaram, assim como diff --check.
Lint: 0 erros, V15/V26 sem resultados, 531 warnings anteriores e 1576 notes.
Comparação de multiset por regra/URI/mensagem com hashes normalizados adicionou
somente V19 informativo do helper novo. Não se declara zero violations global.

A matriz final tem 58 entradas bilaterais. 51 passam texto sem formatação e
caixas agrupadas com tolerância 0.05pt; dois casos antes divergentes passaram
(`emph-mixed` e `arabic-emph-repeat`), sem regressão dos casos antes positivos.
Agrupamento só une caixas contíguas na mesma linha, sem inventar espaços.
[verification.json.gz](identidade-fonte-shaped-implementacao-2026-10-05/verification.json.gz)
conserva também texto exato, extração bruta, caixas estritas, comandos e estado.

Quatro controles — árabe em ênfase, repetição, árabe strong e latim em ênfase —
foram renderizados a 600dpi e inspecionados. Têm mesmas caixas de tinta e zero
pixels escuros sem correspondência dentro de um pixel nos dois sentidos.
Um pixel equivale a 0.12pt; isto não é prova de igualdade subpixel.
[raster.json](identidade-fonte-shaped-implementacao-2026-10-05/raster.json).

## Tempo da CLI completa

A primeira versão correta regrediu: delta pareado +25.36ms em grid320 e cerca
+14ms nos controles RTL/misto. A inspeção mostrou comparações repetidas dos
bytes da fonte. Cache local de associação, atalho de igualdade para referências
compartilhadas, preservação do caminho de fonte única e transporte da fonte no
resultado memoizado removeram a regressão maior. A/B combinado não isola a
contribuição temporal de cada alteração.

Rodada final: 2 warmups, 15 rodadas com ordem aleatória pareada por entrada,
RAYON_NUM_THREADS=1 e SOURCE_DATE_EPOCH=0, sem outro job pesado concorrente.
Os tempos incluem catálogo/fontes, compilação e PDF, não apenas L1.
[benchmark.json](identidade-fonte-shaped-implementacao-2026-10-05/benchmark.json)
conserva amostras, UTC, hashes, estado e comandos; first/second-benchmark guardam
as rodadas intermediárias. IQR abaixo é o dos deltas pareados, em ms.

| Entrada | Tekt antes | Tekt depois | Vanilla | Delta pareado | IQR do delta |
|---|---:|---:|---:|---:|---:|
| grid4 | 151.17 | 151.21 | 125.82 | -0.53 | -1.75…+0.46 |
| grid320 | 194.42 | 194.15 | 146.31 | -0.30 | -1.56…+1.56 |
| rtl160 | 164.38 | 160.47 | 128.06 | -1.18 | -2.00…+1.33 |
| mixed160 | 170.15 | 168.30 | 136.24 | +0.16 | -3.75…+1.76 |

A rodada final não demonstra ganho consistente de tempo na CLI. Os efeitos
pequenos não justificam extrapolar para o gap total com vanilla. Esta rodada
fecha a perda de identidade e o trabalho redundante encontrado na sua correção.

## Limites e defasagens ainda abertas

O controle `variable-axes` mantém exatamente as caixas anteriores e continua
divergente do vanilla, com delta máximo 3.98pt. O teste interno de binding /
instância passa, mas isso não certifica a cadeia fonte→estilo→shaping→métricas
inteira. A origem dessa diferença ainda não foi isolada nesta rodada.

Continuam também Block/Align fora do certificado bidi, extração dos controles
autorais e carrier de espaço literal terminal. A geometria agrupada e a tinta
não substituem a verificação da cópia do PDF. Coverage::from_ranges continua
proposta independente não materializada. Este passo não fecha todos os riscos
de seleção de fonte, dimensões do cache ou memoização entre edições.
