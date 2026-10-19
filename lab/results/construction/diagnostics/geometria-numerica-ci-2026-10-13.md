# Geometria: backend numérico puro e potências pequenas

Substituídas 19 chamadas sinalizadas pelo Clippy em oito consumers. As
operações trigonométricas e normas usam libm 0.2.11, já fixada no projeto;
potências inteiras de grau dois/três usam multiplicação explícita. Não
foram adicionados traits, tipos públicos, flags ou supressões de lint.

## Medição antes da decisão

Branch `Tekt`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**. `geometria-numerica-ci-2026-10-13/before.json.gz` contém
fontes, L0, Cargo.lock, configuração, fonte local de libm, rustc, UTC,
status e `git diff HEAD --stat`. A entrada era de 68 diagnósticos distintos
no Clippy core lib/testes. Os spans selecionados constam do mesmo recibo.

Antes da decisão, uma sonda compilada com o mesmo rustc e o artefato libm
usado pelo projeto comparou as operações anteriores e candidatas. São
4.968 pares: sin/cos/tan com 734 casos cada, hypot/atan2 com 169 cada e
quadrado/cubo com 1.214 cada. A amostra inclui ângulos inteiros de -360 a
360 graus, valores grandes/subnormais, zeros com sinal, infinitos e NaN.
Houve 69 diferenças de até 1 ULP, somente em sin/cos/tan/atan2. Hypot e
potências coincidiram na amostra; não houve mudança de classificação
numérica ou sinal de zero. Isso descreve este host, não todos os floats
nem equivalência universal entre plataformas.

`host-math-summary.json` conserva comandos, resultados, libc e hashes do
binário/artefato; `host-math.csv` contém os pares completos. O código da
sonda está nas receitas. A medição orienta a troca por funções puras
pinadas e as verificações geométricas; não constitui comparação bilateral
com vanilla nem critério de identidade Rust para paridade da linguagem.

## Decisões e implementação

Os oito owners foram atualizados antes do código. Em tiling e cancel,
`libm::hypot` preserva a norma escalada sem substituí-la por uma soma de
quadrados que poderia transbordar. Cancel conserva atan2(width,height),
centro baseline-relativo, length, sinais, cross/inverted/background e o
protocolo de callbacks. Line conserva projeção de length/angle e a
precedência de start/end. TransformMatrix conserva rotação Y-down,
coeficientes de skew e ordem de composição. Apenas hypot foi acrescentado
à whitelist; nenhuma versão de dependência ou pin do oráculo mudou.

Nas fórmulas de custo de parágrafo, a multiplicação externa por 100 e a
ordem da soma final permanecem. Em Bézier, os quadrados são calculados uma
vez e os cubos usam mais uma multiplicação; a ordem dos termos Bernstein,
coeficientes e algoritmo de raízes não mudam. A fixture NonAdditive mantém
avanço quadrático no número de caracteres. O teste de skew usa a identidade
analítica tan(15°)=2-sqrt(3), preservando a tolerância 1e-9 e o controle de
45 graus; deixou de formar a expectativa com a função trigonométrica do
candidato.

Também foi retificada a justificativa antiga do L0 de geometry: a curva
Bézier está contida na caixa dos pontos de controle. Os pesos Bernstein
são não negativos e somam um; os extremos da derivada dão uma caixa mais
justa, não impedem uma fuga da curva da caixa conservadora. A própria
fonte já afirmava essa inclusão e o teste p277_bezier_bbox_curva_tighter_em_y
já a discriminava. Essa correção posterior é somente documental mais
resselo do header. `reference-correction.json.gz` comprova que o corpo
Rust permaneceu idêntico e registra novo build/linhagem/formatação.

## Exemplos de geometria antes/depois

O script `svg_probe.py` compila fontes fixas pelo CLI e registra comandos,
fontes, estado Git, horário e hash do binário em cada rodada. O primeiro
ensaio de cancel usava true sem # dentro de math e foi recusado; a sintaxe
foi corrigida para #true antes de qualquer edição Rust. O resultado inicial
foi preservado; a baseline válida é `before-refined`.

Foram comparadas estrutura XML, ordem de elementos, texto, atributos não
geométricos e componentes numéricos de paths/coordenadas/transformações.
A tolerância foi 1e-9 nas coordenadas serializadas; stderr permaneceu igual.

| Exemplo | Elementos SVG | Maior diferença absoluta |
| --- | --- | --- |
| lines | 5 | 0 |
| rotate | 4 | 0 |
| skew | 4 | 0 |
| cancel | 30 | 0 |
| optimized | 180 | 0 |

Esses exemplos não cobrem todos os casos. O de parágrafo conserva o aviso
preexistente de suporte parcial à seleção global; sua saída não prova que
todos os caminhos globais foram exercitados. Os seletores, caixas Bézier
e demais operações também são verificados pela suíte core. Não houve nova
certificação de paridade contra o vanilla neste lote.

## Validação final

Rodada principal concluída em `2026-10-14T00:20:06.542865+00:00`; documentação concluída em
`2026-10-14T00:20:54.734610+00:00`. As fontes permaneceram inalteradas durante cada rodada.
`pre-validation.json.gz`, `reference-correction.json.gz` e `after.json.gz`
delimitam os estados. Logs, SVGs, fontes, receitas, diffs e recibos estão
reunidos em `evidence.json.gz`.

| Verificação | Resultado |
| --- | --- |
| Clippy core lib/testes com -D warnings | 68 → 49 diagnósticos distintos; ainda falha |
| disallowed_methods | 19 removidos; resta round_with_precision em repr/measure |
| Check workspace com todos os targets | passou |
| Suíte core | 6.705 passaram, zero falhas, cinco ignorados |
| Registro de testes | 6.710 nomes, idênticos antes/depois |
| Build, formatação e diff check | passaram |
| Rustdoc workspace sem dependências, -D warnings | passou |
| Linhagem estrita V1/V5/V15/V26 | passou |
| Linhagem reversa | 965 consumers, zero erros |
| Linter arquitetural completo | zero erros, 380 avisos, 1611 notas |

A CI completa e a matriz remota de release continuam pendentes. O helper
de arredondamento de repr tem uma fórmula e limites próprios a investigar;
a troca deste lote não o certifica. As outras pendências funcionais da
auditoria permanecem no escopo do objetivo geral. Nenhuma release foi publicada.
