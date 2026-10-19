# Tiling: preparação única da célula — 2026-10-14

O layout agora prepara um TilingFrame com uma única célula,
sem depender da área do alvo. Resolve size, spacing, offset, angle e relative,
conserva Color/Gradient integralmente e passa Image pelo layout uma vez.
A célula usa origem geométrica e referencial local explícito; erros causais
chegam ao documento e os mapas de labels/posições e transações do pai são
restaurados. O contrato público aprovado e seu manifesto permanecem intactos.

**Estado intermediário de T1c:** o adaptador materialize_fill consome a célula
preparada, mas ainda cria a grade legada de Groups. Shape.fill/stroke ainda
usam Paint/Stroke, o limite10000 permanece e stroke ainda não usa a célula.
A preparação não encerra T1c/T2/T3 e não certifica produção nem CI/release.

## Proveniência e testes

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. before,
candidate-final, workspace-pre e pre-validation guardam UTC, fontes, hashes,
status e diffstat. Antes de reconstruir, o binário Tekt da sonda foi comprovado
idêntico ao raster-mascaras/after-final: SHA256
8f0f4f1a6dd04ba797db7d04e0d963448151274bf59401f87930bb5b7cb947c8.
O pin vanilla é 9f2b6e8715237cb086899a42873660fe744622e8, árvore
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489; SHA256 do binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
Sondas conservam fontes, argv, códigos de saída, stderr e hashes.

RED inicial: os casos de diagnóstico, ponte Color/Gradient e referencial local
falharam com a implementação anterior. fixture-compile.log registra um erro
anterior do próprio teste (Rect usa w/h), excluído do RED comportamental.
A revisão encontrou que o checkpoint de fluxo apaga seu próprio envelope;
isolation-red.log reproduz a perda da transação externa. O reparo suspende e
restaura pending/active/rejected/checkpoint/replay/depth do pai. Testes adicionais
cobrem auto, empty, placement, Image uma vez, labels locais, erros anteriores
preservados e rejeição antes de executar body. Dez testes foram acrescentados.

Suíte final: **8548 passaram, 0 falharam, 7 ignorados**,
UTC 2026-10-14T15:11:42.559735+00:00 → 2026-10-14T15:17:41.713934+00:00; fontes verificadas inalteradas durante execução.
Build, rustdoc com -D warnings, check all-targets, fmt, diff check, linhagem
V1/V5/V15/V26 estrita e reverse lineage passaram. Clippy é inventário, não gate
-D warnings: 181 → 181, delta por código
{}. Arquitetura completa: {'warning': 382, 'note': 1631}.
Não há alegação de CI remoto verde ou de publicação.

Após a suíte e as sondas, Clippy apontou type_complexity na declaração da
matriz de casos de teste. A única alteração posterior é um alias desse tipo,
mais hashes de linhagem: test-alias-audit.json verifica por igualdade exata
do restante do arquivo e do L0. before-test-alias/candidate-final conserva
o código da suíte/sondas; candidate-final conserva a árvore final. Os dez
testes focados foram executados novamente e build/doc/check/fmt/Clippy/
linhagem foram revalidados. Não se atribui uma nova execução da suíte inteira
ao alias; a implementação produtiva é a mesma que passou a suíte completa.

## Sondas e limites observados

Canvas inteiro 480×280, PNG explícito --ppi144; SVG librsvg72dpi/zoom2;
PDF Poppler144dpi. Sem recorte, realinhamento ou aumento de tolerância.
O limiar >16 é métrica descritiva do delta, não critério suficiente de aceitação.
A primeira chamada pdftoppm usou argumento inválido -scale-to -1; os arquivos
parciais permanecem em before-renders-interrupted e não entram na comparação.
Todos os PDFs emitidos com sucesso têm o retorno de qpdf --check registrado.

Os renders válidos do Tekt permaneceram idênticos na amostra.

Auto vazio passa a falhar com `tile size must be non-zero` e hint explícito,
em vez de emitir preenchimento preto. Pitch zero passa a produzir
`tile pitch must be positive and finite`. O vanilla pinado sofre panic nesse
caso em PDF/PNG; essa falha não é requisito do Tekt. Body com label ausente
continua erro causal nos dois compiladores. comparison.json conserva cada
resultado, inclusive falhas, sem excluir controles desfavoráveis.

As sentinelas dense-open e stroke-open continuam divergentes: exigem a troca
efetiva de Shape.fill/stroke e patterns nativos. Gradientes raster, composição
afínica de Parent, recursos recursivos, anotações decorativas, clipping da célula
nativa e integrações de Place/Rotate continuam gates abertos. O checkpoint
não prova isolamento universal de toda feature; a evidência cobre os casos
nomeados. Não se mediu desempenho/memória nem se alega produção pelo número
de objetos de teste.

## Continuação da migração autorizada

shape-field-migration-inventory.json.gz conserva spans/expressões de cada
construção Shape e hashes dos arquivos. A migração deve manter reconstruções
pass-through e converter os produtores reais: shape, Box/Block, contornos de
grid/table, cell_tail_commit e formas Color-only. Grid ainda compara Stroke
antes de fundir segmentos; preparar no emit conserva essa decisão, sem dar
PartialEq a FrameStroke. PageFill é Paint declarativo separado: não substituir
imports globalmente nos exporters. PDF/SVG/PNG, coletores de imagens/fontes/
spots/gradientes e shaper precisam percorrer a célula; FrameVisitor genérico e
plain_text da página não devem fazê-lo. Não adicionar From<Paint> total ou
fallback sólido para obter compilação. T1c/T2/T3 e o objetivo geral permanecem
ativos, sem nova necessidade de aprovação do contrato já ratificado.

Arquivo reaberto e verificado: 258 entradas,
SHA256 `4743e878231e9fbd78cf6408f18102eac36faee7f0505564e3c580ee3fbc09aa`. Sem commit, push, PR ou release.
