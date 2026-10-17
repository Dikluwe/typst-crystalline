# Limites e precisão dos componentes de cor

O saneamento posterior dos atributos de teste e do Clippy está em
[Integração dos módulos de teste e Clippy](cfg-test-ci-2026-10-13.md).
Os resultados abaixo conservam a medição desta rodada de cores.

Esta rodada continua a investigação de [cores e CI](cores-ci-2026-10-13.md).
Corrige a conversão de cores fora do gamut, a quantização prematura dos
componentes de `rgb`, a validação de componentes de `color.linear-rgb` e a
ordem de cálculo observável na conversão polar e nas curvas sRGB.

## Proveniência e medição anterior à decisão

Branch `Tekt`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**, preservando as alterações preexistentes. O diretório
`limites-cor-2026-10-13/` conserva fontes, L0, configuração, hashes, UTC,
status e `git diff HEAD --stat` em `before.json.gz`, `parents-before.json.gz`
e `workspace-pre.json.gz`. Os números descrevem essas árvores, não o HEAD
isolado. Os logs e sondas integrais ficam em `evidence.json.gz`.

Oráculo: upstream pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, binário local ratificado
`lab/typst-original/target/release/typst`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Cada sonda registra também o hash do binário cristalino usado.

As sondas anteriores à implementação mostraram:

- `red.rotate(45deg)` expunha um canal blue negativo; o vanilla limita esse
  caminho de conversão a zero. A reconversão para Oklch também divergia.
- `rgb(10%,20%,30%,40%)` arredondava RGB para bytes antes de armazenar a cor,
  alterando os componentes públicos. `rgb(Color)` repetia a quantização.
- Linear RGB aceitava componentes fora do intervalo; o vanilla rejeitava
  a entrada, inclusive alpha negativo.
- Oklch com chroma negativo produzia uma cor cromática; o vanilla limita
  chroma a zero antes da conversão polar.

A fonte upstream `visualize/color.rs:1825-1895` distingue conversão limitada,
identidade e a transferência linear→sRGB sem clamp adicional. Palette 0.7.6
`convert/from_into_color.rs:53-59` limita o resultado de FromColor;
`oklab.rs:312-368` define a matriz direta e o limite de chroma.
`encoding/srgb.rs:100-120` e `num.rs:718-722` mostram que o encode usa
multiplicação/subtração sem fusão; o antigo requisito P1253 de fmaf nesse
caminho estava incorreto. O decode linear multiplica pelo recíproco.

## Correções e refutadores

Os L0 foram atualizados antes de cada implementação, no fluxo contínuo do
ADR-0127. Nenhuma assinatura pública Rust, campo, binding, modo padrão ou
fase de pipeline foi introduzido. O reparador foi aplicado somente aos
owners afetados; os núcleos não foram alterados.

`rgb` preserva os componentes f32 e a identidade de uma Color sRGB. O caminho
CMYK calibrado preexistente continua separado. Linear RGB reutiliza o caster
de componentes com validação e spans. A conversão Oklab/Oklch→LinearRgb usa
a matriz diretamente, sem encode/decode intermediários. FromColor aplica
os limites exigidos; identidade e transferência linear→sRGB preservam seus
contratos próprios. O helper bruto de componentes não recebeu clamp global.

A primeira execução dos sete novos testes teve seis falhas funcionais e um
controle aprovado. Depois do primeiro reparo, a suíte do core ainda falhou
na precisão de rotate. A tolerância foi preservada. Uma sonda intermediária
isolou o hue: o vanilla produzia `28.442533493041992`, e o cristalino
`28.442523956298828`, com lightness e chroma iguais. A fonte palette
`hues.rs:134-146` usa `PI + atan2(-b,-a)` antes da conversão para graus.

Um segundo teste independente foi acrescentado no interpolador de gradiente,
e a expectativa do teste de rotate passou a observar também o ângulo de
entrada. Os dois falharam antes da correção da normalização. O ajuste usa
a ordem medida e `libm::sincosf` na conversão polar. As expectativas dos
testes são referências medidas, sem chamar o helper corrigido para gerar
o resultado esperado.

O primeiro relatório arquitetural apontou um V16 novo no fallback da
conversão LinearRgb. O L0 foi complementado e o braço passou a enumerar
as variantes atuais, mantendo o mesmo corpo. O warning específico deixou
de aparecer; os demais avisos continuam registrados.

### Igualdade de linguagem preservada

A sonda posterior encontrou uma regressão que os testes anteriores não
cobriam: `rgb(50%,0%,0%) == rgb("#800000")` ficou falso depois de preservar
corretamente os componentes f32. Requantizar os componentes apagaria a
correção original. A leitura do owner de igualdade confirmou a separação
entre igualdade da linguagem e `PartialEq` estrutural do Rust.

O upstream `visualize/color.rs:2064-2083` compara duas cores sRGB por quatro
componentes u8. Sua projeção em `:1780-1783` usa `round`, com empate afastado
de zero, diferentemente do ties-even usado por `to-hex`. A sonda medida de
90.5/255 torna a distinção observável: hexadecimal `#5a0000`, igualdade com
rgb(91,0,0) verdadeira e com rgb(90,0,0) falsa.

O L0 de `compiler/eval/operators/equality` foi atualizado primeiro; dois
testes novos falharam antes da implementação. `values_eq` agora refina o
par sRGB/sRGB usando essa projeção. A recursão existente propaga a regra
para arrays, dictionaries e pertença. Alpha participa; canais distintos,
LinearRgb e espaços diferentes têm controles negativos. `PartialEq`,
Hash, componentes armazenados e hexadecimal permanecem intactos.

Os recibos `equality-before.json.gz` e `equality-pre-validation.json.gz`
conservam fontes, L0, estado da árvore e medições desse reparo adicional.
O resultado intermediário com igualdade incorreta permanece em
`after-probes.json`, sem sobrescrita; a medição final usa outro arquivo.

## Validação e limites da evidência

| Verificação | Resultado |
| --- | --- |
| Workspace completo, antes do refinamento nominal e da correção de igualdade | 8351 passaram, zero falhas, 7 ignorados; 32 resumos; fontes verificadas sem alteração durante a execução |
| Conversão de cores, após exaustividade nominal | 78 passaram, zero falhas |
| Dois novos testes de igualdade antes do reparo | Ambos falharam funcionalmente |
| Core completo, após a correção de igualdade | 6701 passaram, zero falhas, 5 ignorados |
| Build final do workspace | Passou |
| Gates estritos V1/V5/V15/V26 e reverso após o último reparo | Passaram; 965 consumers, zero erros |
| Formatação e diff check | Passaram |
| Auxiliares de CI e actionlint | 16 testes passaram; workflows válidos |
| Arquitetura completa final | Zero erros; 381 warnings; 1619 notes |
| Clippy core, lib e testes, com `-D warnings` | 577 diagnósticos distintos; continua bloqueante |

Nenhum diagnóstico Clippy pertence aos quatro consumers de implementação
alterados nesta rodada. A deduplicação usa código, mensagem e spans primários
completos `(file, byte_start, byte_end)`, sem somar lib e testes. A contagem
não certifica os demais pacotes: o gate do workspace continua bloqueado.

A suíte inteira do workspace não foi repetida após o último reparo de
igualdade. A validação desse estado inclui o core completo, build e sondas
reais da CLI. As contagens acima não são somadas entre estados.

As 16 expressões de conversão têm a mesma classificação de sucesso/erro
nos dois binários. Das 13 positivas, 12 têm saída coincidente; permanece
a diferença de chroma da amostragem Oklch em 25%:
`0.214685320854187` no vanilla e `0.2146853357553482` no cristalino.
As 12 sondas adicionais de igualdade coincidem, incluindo o empate que
distingue igualdade de hexadecimal. Todas as saídas brutas, mensagens e
spans ficam conservados. As três entradas inválidas não são contadas como
prova de igualdade de diagnóstico apenas porque possuem stdout vazio.

## Alcance

Esta rodada não certifica todos os espaços de cor nem o modelo ICC geral.
A precisão dos pesos de amostragem dos gradientes é uma investigação separada.
As diferenças de diagnóstico das formas inválidas devem ser lidas nas sondas
integrais: dois erros com stdout vazio não são diagnósticos iguais.

CI/release continuam exigindo os gates existentes. A validação remota,
Clippy, avisos arquiteturais e os demais itens da auditoria original
permanecem no objetivo. Nenhum commit, push ou publicação foi realizado.
