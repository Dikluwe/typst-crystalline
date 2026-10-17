# D06 — preparação de matemática nua materializada

Estado: normalização, identidade, show e escopo de inserção corrigidos no recorte
medido. Matemática estrutural externa passa pelo bridge Equation existente antes
de show/introspecção. Débitos preexistentes de render e morfologia permanecem
explicitamente separados abaixo; não se certifica matemática universal.

## Gate e proveniência

O humano confirmou o pacote `00_nucleo/materialization/math-bare-2026-10-06.md`.
Receipt: `00_nucleo/materialization/math-bare-aprovacao-2026-10-06.json`.
Manifesto ratificado SHA-256
`149d6535b442a9275aa6612647ad79eac460351808ea1858175d09d1cf07e1f6`.
Nove L0s completos e Núcleo foram promovidos antes de testes/implementação.
A assinatura pública realize_math_insertions e a preparação anterior a show/índice
estão cobertas pelo gate ADR-0127 confirmado.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
Todas as evidências ficam em `math-bare-2026-10-06/`. Coletas guardam UTC,
`git diff HEAD --stat`, lista exata de alterações, hashes das fontes, entradas,
argv/cwd/ambiente, outputs e hashes dos executáveis congelados.
Snapshots baseline-state, sealed-state e final-code preservam também conteúdo.

Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Tekt release final SHA-256
`85b173641ecfa8284381f30b1616a69724c284db7c443399aaa6fee84ceb5e69`.
Coleta final: 2026-10-06T21:03:12.988389+00:00 a 21:03:25.753352+00:00.
Final-additional: 21:03:26.116269+00:00 a 21:03:32.368577+00:00.
Final-refuters: 21:03:32.813434+00:00 a 21:03:33.923776+00:00, mesma data UTC.
`final-results.json` registra também proveniência das regressões.

## Medição anterior à decisão

`math-bare-medicao-2026-10-06.md` reúne before/additional/refuters e fonte pinada.
Frac/mat/root/lr/op nus tinham zero Equation no Tekt contra uma no vanilla;
Equation explícita já produzia fração pelo bridge. Texto/math.bold([x]) não
criavam Equation, enquanto bold(frac(...)) tinha inserção estrutural. Alias nu
inserido duas vezes tinha duas ocorrências; corpo dentro de math tinha uma.
Block/alt/numbering eram propriedades do escopo de inserção, e show tinha de
ocorrer depois da preparação. Valor cru frac e seu repr coincidiam antes da
inserção. Label do frac original não era label da Equation automática.

Fonte upstream `typst-realize/src/lib.rs:245–265,302–334` prepara kind antes de
show e distingue Math de conteúdo externo. Tekt `layout/equation.rs:680–689`
emitia plain_text, realize/mod omitira a preparação e eval/mod:908–925 pulava a
realização quando não havia receitas. `mathy-catalog.json` conserva file:line
para o catálogo Mathy real; o prefixo Math não é critério.

Coleta after demonstrou perda de block local e de show em produtor contextual.
CodeBlock restaurava styles sem transportar o delta para a inserção nua;
ContextBlock transportava closure/scope, mas o Engine de execução tinha rules
vazio. Os RED local-set e context-show confirmaram esses observáveis.

## Materialização e refinamentos internos

Novo owner privado compiler/realize/bare_math reconhece variantes estáticas e
prepara a raiz externa, parando em Equation/Located e preservando origens
Realized. Styled/ShowRecipe conservam contexto lexical. MathSequence é fluxo;
MathIdent/MathText e MathStyled de texto isolado não são Mathy. MathStyled com
raiz estrutural transporta sua composição dentro do corpo Equation.
MathUnderover é extensão legada sem counterpart Mathy upstream e conserva
fallback anterior; essa correção do catálogo foi documentada no L0 antes do código.

Helper privado pré-ordem em traversal substitui uma subárvore antes de visitar
filhos; a tabela exaustiva fica nesse owner, sem catálogo de containers no
compiler. Map_content/map_text públicos permanecem com suas semânticas.
Dynamic legado é fronteira opaca do helper privado; math dentro de Dynamic não
é certificado. Métodos públicos dos elementos não foram alterados.

Show usa identidade estática de math.equation direto/alias/where. A preparação
precede matching/Location; produtos novos voltam a realizar, it conserva
identidade e it.body novo alcança o limite de profundidade com diagnóstico
causal. Span da chamada diferida é transportado privadamente no Styles da
receita e só ancora erros detached, sem substituir Source/span conhecidos.

Eval continua executando fonte/argumentos comuns uma vez. Seleciona a realização
existente também para math externo sem show; valores/Scope crus não viram
Equation. Set equation.block é avaliado e transportado lexicalmente. Um delta
local de bloco de código acompanha apenas os produtos matemáticos pertinentes.
Label do Mathy original não é promovido para a equação criada.

Produtor contextual delega à fachada pública ratificada com seu Engine/ctx
existentes. Snapshot de show é capturado por carriers/Value/Styles existentes
somente quando há regra Equation ativa; rules/guards/file são instalados e
restaurados no percurso de contribuição. Não há campo/trait/enum/flag novo,
segunda avaliação global, I/O em L1 ou mudança do orçamento contextual.

A primeira suíte ampliada revelou duas regressões HTML: preparar identidades
em contribuições sem math deslocava a descoberta/posição contextual. A fachada
agora retorna esse Content antes de observar/alocar/aplicar show. Os seis
controles p1493 passaram, e a suíte L3 inteira foi reexecutada depois do reparo.

Essas ligações ampliaram o conjunto a 13 owners exclusivos, dentro da fase e
fachada ratificadas: eval/mod, rules/set, expression_dispatch e call_dispatch/invoke
receberam L0 primeiro. Pins foram ordenados e grafo V15/V26 reparado antes do
resselo restrito. Não foi usado --fix-hashes global.

## Validação

RED de linguagem: valid-red.log, local-set-red.log e context-show-red.log.
As falhas de compilação intermediárias não são usadas como prova RED.
GREEN: seis testes focados L1, dois de integração e seis controles p1493;
suítes completas finais incluem esses casos.

- final.json.gz: 38/41 controles integrais coincidem.
- final-additional.json.gz: 18/20 controles integrais coincidem.
- final-refuters.json.gz: 4/4 controles coincidem.
- Regressão D05: 29/29 em equation-where-2026-10-06/final-math-bare.json.gz.
- Regressão query: 36/36 em query-compilada-2026-10-06/final-math-bare.json.gz.
- Núcleo: 6572 aprovados, zero falhas, cinco ignorados (final2-core.log).
- Infraestrutura: 1194 aprovados, zero falhas (final2-infra.log).
- CLI query: dez aprovados, zero falhas, 116 filtrados (final2-cli-query.log).
- Builds debug/release aprovados (final2-debug.log e final2-release.log).
- Linter: zero erros, 529 warnings, 1592 notes (final2-lint.sarif).
  Warnings não são certificados como resolvidos.

Não somar as três coletas como percentual global. Status/stdout/stderr/texto
extraído são os observáveis comparados; bytes PDF não são critério. As duas
sondas adicionais com variável f desconhecida nos dois permanecem controles
de rejeição, não prova de valor cru; final-refuters usa bloco direto válido.

geometry-pairs.json verifica bbox/texto dos números da fração nua versus
Equation explícita: coincidem dentro de cada binário. A diferença bilateral
preexistente de largura da bbox também existe na Equation explícita; não se
certifica geometria completa com essa comparação. Raiz/negrito têm controles
explícitos separados em limits.json.gz, com fonte/estado/binários preservados.

seals.json registra 13 owners/consumers 1:1. final-preservation.json confirma
27 arquivos de fonte/L0/núcleo alterados nesta frente, nenhum fora do conjunto
pertinente, e todos os selos atuais. Alterações anteriores permanecem preservadas.
final-checks.json guarda hashes dos receipts/logs, e final-code fixa o estado
produtivo final. git diff --check passou.

## Negativos separados que permanecem

Os cinco negativos integrais finais são: render da raiz; aviso de arquivo vazio
no eval cru; nome/morfologia de mat (D07), junto do mesmo aviso; render de fração
em negrito; query do Mathy original rotulado. Os controles de raiz/negrito com
Equation explícita reproduzem as diferenças, isolando-as da criação automática.
Query Equation.where(label:<f>) de frac nu continua zero nos dois; query(<f>) do
frac original tem diferença própria. O wrapper automático não deve ocultá-la.

Snapshot morfológico amplo de Styled, HTML/math, Dynamic, callback/rich-numbering
não medidos e catálogo/render completo não foram certificados. D07 é a próxima
frente do plano; não foi materializado nesta etapa.
