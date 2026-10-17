# Tiling: pivô de rotação e fase periódica — 2026-10-14

## Resultado e proveniência

Corrigida a interpretação do offset na rotação da grade: o offset é o pivô,
não um vetor a girar em torno de zero. A compensação da origem Parent
fornecida pelo caller fica no referencial do alvo. A redução de períodos
usa a base da grade rotacionada. Esta mudança não resolve o transporte do
pai composto por Place e outros grupos nem o tiling de contornos.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. before-state,
before/results, red-state, candidate-final, pre-validation, workspace-pre
e final-receipt preservam UTC, diffstat, fontes/owners e hashes dos estados.
Referência pinada 9f2b6e8715237cb086899a42873660fe744622e8, árvore/receita P1489,
binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
A recoleta ocorreu de 2026-10-14T09:31:47.586182+00:00 a 2026-10-14T09:31:50.078408+00:00; binário Tekt
SHA-256 `f99f61f7f804cb79209d2dbec39bb5c796970b790af738e8ac8eb27b72e54901`.

## Evidência anterior à decisão

O SVG vanilla para offset(3pt,5pt), angle90, emitiu a transformação
(0,1,-1,0,3,5). O Tekt emitiu (0,1,-1,0,-5,3). No pin,
visualize/tiling.rs:173-174 descreve a rotação em torno do offset;
:375-377 e layout/transform.rs:334-343 implementam T*R. O L0 anterior e o
teste P1254 tinham interpretado a frase "offset antes de angle" como R*T.
É falha conjunta de especificação e implementação, não drift de hash.
A obrigação é o ponto geometricamente observado, não copiar chamadas Rust.

A comparação apenas PDF seria enganosa. O caso quarter sem offset tem
Tekt PDF = Tekt SVG = vanilla SVG pixel a pixel no raster registrado, mas
vanilla PDF difere de seu SVG em 414 pixels com diferença de canal >16.
Preservou-se o Tekt nesse controle; não se convergiu para a discrepância
PDF do oráculo. Isto é evidência deste corpus/pin/renderizadores, sem
atribuir ainda a causa interna ao vanilla nem declarar superioridade geral.

## Contrato e materialização

O núcleo `00_nucleo/prompts/_nuclei/tiling-placement.toml` contém as claims
compartilhadas de pivô e periodicidade, com digest
`9c88c9c3bde8b005623fa70b5213400655a9a741b1a140b6b76c7225fc16edb4`.
Ele é consumido/pinado pelos owners de entities/tiling, stdlib/tiling e
layout/tiling. Cada prompt continua com um consumer produtivo; código não
aponta ao núcleo. Os L0 foram corrigidos antes do código, inclusive a
ambiguidade histórica e a nota do raio diagonal já substituído no lote anterior.

`layout/tiling.rs` compõe translate(offset - origem Parent) * rotate(angle).
O helper privado de periodicidade converte a translação pela inversa da parte
linear, reduz pelos pitches e retorna à base original, conservando a pintura.
Normalizar x/y da página diretamente é incorreto com rotação: a 90°, um
período vertical vira deslocamento horizontal. Cobertura, recortes, corpo
vazio, size:auto, layout único e limite de células permanecem.

Somente esse owner tem mudança funcional. Em entities/tiling.rs mudou a
documentação de angle e o selo; stdlib/visualize.rs mudou somente o selo.
final-comparison verifica essas relações após normalizar o header, e o
recibo final comprova que nenhum outro Rust foi alterado neste lote.
Não houve nova API, campo público, fase, flag/default ou dependência;
correção de paridade interna em fluxo contínuo ADR-0127.

O teste histórico agora exige o pivô correto. Três testes novos verificam
a tinta da célula ancorada, a compensação Parent independente da rotação e
a equivalência de períodos nos eixos rotacionados. Os quatro falharam
funcionalmente antes da implementação (red.log); os 39 testes filtrados de
tiling passaram depois (green.log). Não se usou erro de compilação como RED.

## Recoleta de SVG e PDF

Todos os renders usam o canvas inteiro 240×192, fundo branco. SVG foi
rasterizado por librsvg; PDF por Poppler a 144 dpi. O limiar >16 é uma
estatística preservada, não uma tolerância nova de aceitação. Os recibos
registram também qualquer diferença e máximo por canal.

| Caso | SVG bilateral antes: pixels >16 | Depois | Máximo depois |
|---|---:|---:|---:|
| quarter-offset | 1764 | 0 | 0 |
| oblique-offset | 1930 | 4 | 19 |
| oblique-ratio | 1884 | 4 | 19 |
| parent-offset direto | 1085 | 4 | 18 |
| parent-place | 2200 | 2205 | 201 |

quarter-offset agora coincide exatamente também entre Tekt PDF e vanilla
SVG, confirmado por pixels e inspeção visual. A matriz Tekt pode diferir da
matriz literal vanilla por períodos inteiros; isto é equivalência geométrica,
não erro. Controles plain, offset sem ângulo, quarter e oblique sem offset
mantiveram seus pixels anteriores nos dois formatos: oito comparações locais
idênticas. As diferenças pequenas nos oblíquos não foram apagadas nem usadas
para declarar paridade completa. O parent composto continua errado; o aumento
da diferença em parent-place está exposto, sem seleção favorável de casos.

## Limites e sequência do trabalho

- Parent dentro de Place ainda recebe origem local insuficiente. Corrigir a
  fórmula local não transporta a transformação composta perdida pelo caller.
- Contornos com tiling continuam no fallback sólido e o teto de 10000 células
  continua inadequado para alguns padrões densos. O lote tiling-cobertura
  conserva as provas públicas desses defeitos. Não aumentar o teto ou inventar
  identificação de grupos por forma para fingir suporte completo.
- O contrato vigente exclui ResolvedTiling público. Uma eventual mudança de
  transporte L1→L3 exige L0 concreto e gate ADR-0127, sem mover eval/layout
  para o exporter. carrier-candidate-inventory é apenas inventário textual
  de consumers potenciais com owners/hashes, não design aprovado nem contagem
  de mudanças necessárias. O pacote Raw já pendente é independente.
- Diferenças PDF/SVG, targets/texto, corpo auto inválido, regras de recorte e
  os demais débitos de produção, CI/release e linhagem global seguem abertos.

## Validação

Suíte completa 2026-10-14T09:33:03.669683+00:00 a 2026-10-14T09:38:40.566994+00:00:
**8485 passaram, 0 falharam, 7 ignorados**.
Fontes e owners ficaram estáveis; ambiente com /dev/shm, vanilla pinado e
linter CI. Build offline locked, rustdoc -D warnings, check all-targets,
fmt, linhagem estrita V1/V5/V15/V26, reversa e diff-check passaram.
Clippy em inventário --cap-lints warn permanece em 181,
sem delta; não é aprovação de -D warnings. Linter completo: zero erros,
382 warnings e 1626 notas,
portanto não zero violations. Os 40 PDFs passaram no qpdf.

Scripts/receitas, logs, fontes de medição, PDFs, SVGs, rasters e recibos
estão em evidence.json.gz, com SHA-256/conteúdo por entrada e releitura
verificada. O objetivo global continua ativo; nada foi commitado/publicado.
