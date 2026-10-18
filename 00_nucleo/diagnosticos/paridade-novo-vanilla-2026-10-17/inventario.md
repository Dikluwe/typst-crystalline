# Inventário de paridade contra bd5ebec28 — 2026-10-17

**Resultado:** há diferenças de linguagem que precisam de correção e diferenças
que devem ser conservadas no Tekt. Não foi alterado código produtivo, API ou L0.
Este documento é diagnóstico; não legitima implementação ou aprova contratos.

A referência é `bd5ebec285505889e8ff1d4f68804ab147a699f3`, árvore
`cac836c07e4dec2a89219370428292393a3216b5`. Estado auditado:
`b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, branch Tekt, com somente este
novo diretório de diagnóstico não commitado. HEAD, UTC, status/diffstat e hashes
estão em `before.json.gz`; receitas e resultados completos nos arquivos de coleta.
A seção de evidência precede as decisões para cumprir ADR-0107/0108.

## 1. Medição antes da classificação

### Identidade dos produtos

| Produto | SHA-256 |
|---|---|
| Vanilla novo, `lab/typst-original/target/release/typst` | `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9` |
| Tekt, `target/debug/typst` | `5cdfaa01cec0b3035ca1069b8d013bf014bf6f93d119f52fc651d46da28ec0f8` |
| Vanilla anterior, `typst-9f2b6e871-p1489` | `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` |

O executável Tekt é o mesmo certificado no re-sync anterior. Os 2105 hashes
produtivos/L0 daquele freeze coincidem com a árvore atual. O `--version` conserva
o HEAD usado no build; a equivalência foi comprovada por hashes, não pela string.

Foram coletadas **703 entradas/1498 invocações CLI** em execuções sequenciais:
627 na coleta principal, 45 suplementares, 13 focais, seis de isolamento,
oito de fronteiras e quatro de include. Existem controles repetidos entre
corpora: não são 703 defeitos nem uma cobertura percentual da linguagem.
Duas entradas iniciais de inteiros negativos foram interpretadas como opções
CLI: suas quatro invocações com exit 2 não recebem crédito de linguagem. A
repetição com expressão entre chaves está em `supplement.json.gz`.
Quatro sondas iniciais de stretch usavam `200%` sem `#` no contexto matemático:
servem como negativos de validação, não como prova geométrica positiva. O corpus
focal repetiu a sintaxe válida da fonte upstream com `#200%`/`#(100%+2em)`.

`measurement.json.gz`, `supplement.json.gz`, `focal.json.gz`, `last.json.gz`,
`edges.json.gz`, `include.json.gz` e `visual.json.gz` preservam entradas,
comandos, erros completos, valores, coordenadas e controles. Os identificadores
abaixo são os campos `group/label` dessas coletas. `evidence.json.gz` preserva `analysis.json`, uma triagem
mecânica inicial: diferença de dump, ordem de extração PDF ou representação
interna não é automaticamente defeito de linguagem.

### Evidências discriminantes

| ID | Coleta e observação nova | Local da fonte que delimita a investigação |
|---|---|---|
| E01 | `relative-render/h-ratio`: Tekt junta `LR`; vanilla posiciona R em x=106,68555pt. `box-ratio`: Z fica em x=8 no Tekt e x=100 no vanilla. Os controles absolutos preservam o deslocamento. Block relativo mantém a geometria nos controles; getter ausente é dívida distinta. | `01_core/src/compiler/stdlib/layout/casting.rs:25-36`; os carriers públicos são delimitados no L0 desse owner. |
| E02 | `supplement-eval/text-field`, `h-amount`, `box-width`, `block-width`: getters válidos são rejeitados; `.fields()` de h/box/block retorna vazio onde há propriedades explícitas no vanilla. Caption omitida/none e metadados de grupos também divergem. | `01_core/src/entities/content/fields.rs`; `01_core/src/compiler/eval/bindings/field_access/content_methods.rs`. |
| E03 | `callback-isolation/state-show-value`: Tekt não escreve o valor, vanilla escreve `1`. Callback de update com `panic("CALLED")` e callback de supplement de Heading terminam com sucesso no Tekt, mas o vanilla executa a função e reporta o panic. `state("audit")` é recusado pelo Tekt, apesar do init opcional no vanilla. | `01_core/src/compiler/introspect/from_tags/state_updates.rs:11-30` distingue replay e caminho legado; `01_core/src/compiler/stdlib/state.rs`. |
| E04 | `counter/math.equation` e alias são rejeitados; o seletor `.where(block:true)` é aceito. `show bibliography` e `show text` são recusados, enquanto `bibliography.where()` permite avançar. | `00_nucleo/prompts/compiler/stdlib/counter.md:65-77` ainda limita a tabela; `01_core/src/compiler/stdlib/counter.rs`; `01_core/src/compiler/stdlib/foundations/selector.rs`. |
| E05 | Bibliografia MLA sem regra: continuação em x=8 no Tekt, x=26 no vanilla. Com `bibliography.where(): set par(hanging-indent:3em)`, vanilla novo usa x=44, anterior continua em x=26; Tekt continua em x=8 e muda para o formato fallback `[1] ... ↑[1]`, apesar de MLA explícito. O parágrafo OUTSIDE é controle de não vazamento. | `01_core/src/compiler/stdlib/structural/par.rs:172-173` ignora hanging-indent; `01_core/src/compiler/layout/convergence.rs:196-245` procura estilo e termina com wildcard; `01_core/src/compiler/layout/bibliography.rs:112-123` escolhe cache ou fallback. |
| E06 | `document-format`: no bundle, Tekt rejeita `format:"html"` e `format:auto`; aceita `format:format.html`. Vanilla aceita string/auto e rejeita a função. Em eval, a guarda de target antecipa a rejeição de valores document no Tekt. | `01_core/src/compiler/stdlib/structural/document.rs:57-69`; L0 vigente ainda declara Func e restrição de construção fora de Bundle. |
| E07 | 105 pares Table foram repetidos: spans/ocupação, ordem lexical, grupos vazios/níveis/repeat, erros escondidos por receitas, metadados e índices de linhas divergem. O SVG `erase-blue` conserva 80 pixels azuis no Tekt e nenhum no vanilla; `erase-half` conserva a linha inteira no Tekt e metade no vanilla. Não se usa simples diferença de tbody como defeito. | `01_core/src/compiler/stdlib/structural/table_grid.rs`; `01_core/src/compiler/realize/table_cells.rs`; `01_core/src/compiler/layout/grid/`; proposta RG1 existente. |
| E08 | Text aceita Color, mas rejeita Gradient e Tiling nomeados/posicionais que o vanilla aceita. O negativo `relative:"self"` também exige erro/hint próprio. | `01_core/src/entities/style.rs:45`, `Fill(Color)`; `01_core/src/compiler/stdlib/text/constructor.rs`. |
| E09 | `scripts`/`scripts-abs`: posição e tamanho de sub/super divergem, inclusive no controle absoluto. Em `heading-affine-child`, B mede 73,2pt, mas Tekt mantém página de 72,5318pt contra 100,33917pt, deslocando Z antes do fim do filho. Na substituição de Heading por função, A fica em 27pt no Tekt contra 36,6pt. | `01_core/src/compiler/layout/heading.rs`; `01_core/src/compiler/layout/paragraph.rs`; os owners de layout de sub/super e de show-set precisam ser delimitados no lote. |
| E10 | `matching/local-cjk`: Tekt produz `水果`, vanilla `X`; controle root produz X nos dois. Duas regras idênticas produzem X no Tekt e Y no vanilla, também em fragmento local. | `01_core/src/compiler/eval/rules/textual/matching.rs`; L0 vigente declara ambas as obrigações ainda abertas. |
| E11 | `positive-overflow-type`: Tekt promove inteiro decimal fora de i64 para float; vanilla erra. A forma negativa entre chaves também vira Float no Tekt; vanilla exige `int.min` em vez do literal manual. Float explícito é controle válido. | `01_core/src/compiler/lexer/code.rs:167-178`; `01_core/src/entities/ast/expr.rs` e `01_core/src/compiler/eval/expression_dispatch.rs` devem ser auditados para não trocar promoção por zero. |
| E12 | `include-morphology/string-render` e `path-render`: arquivo só com definições deixa `L R` no Tekt, `LR` no vanilla. O controle de igualdade com `[]` dá false nos dois: não autoriza canonicalizar todos os wrappers vazios. | `01_core/src/compiler/eval/modules.rs`; `01_core/src/compiler/eval/markup.rs`; origem dos separadores. |
| E13 | `supplement-inequality-file`: Heading com supplement A é considerado igual ao de supplement C no Tekt; vanilla distingue. Controle com mesmo supplement coincide. `std-cross-file-*`: Tekt false e vanilla true, tanto no perfil padrão como HTML. Os 24 controles Styled continuam coincidentes. | `01_core/src/compiler/eval/operators/equality.rs`; `01_core/src/entities/content/morphology.rs`; identidade da biblioteca/import em `01_core/src/compiler/eval/modules.rs`. |
| E14 | Figure perde kind/reflexão explícita, distinção entre função e string e presença de caption. A limitação pública `Option<String>` está declarada no L0 FG1. | `01_core/src/compiler/stdlib/figure_image.rs`; `01_core/src/entities/elements/figure.rs`. |
| E15 | Sonda de símbolos do re-sync, no mesmo binário verificado: 146 MATCH, 16 TEXT_MATCH_CARRIER_DIFF e 36 EXTENSION. O glifo coincide nos 16, mas o carrier público é Text em vez de Symbol. | `lab/parity/tools/p1495_symbols_probe.py:187-208`; recibo em `vanilla-resync-2026-10-17/evidence.json.gz`; `01_core/src/entities/content.rs`/`01_core/src/compiler/realize/bare_math.rs`. |
| E16 | Stretch válido com anexos mantém colocação lateral/avanço diferente no Tekt, em vez do tratamento do eixo nativo do vanilla. No caso horizontal com attachment, Z fica em x=85,497 contra x=76,323pt. O callback de angle funciona na chamada direta de math.cancel, mas vira content e é recusado na chamada matemática com `#compute`. | `01_core/src/compiler/math/layout/stretch.rs`; `01_core/src/compiler/math/layout/attach.rs`; `01_core/src/compiler/eval/math/arguments.rs`. Contagem de pedaços de assembly no PDF não é critério de geometria. |
| E17 | SVG e PNG multipágina gravam apenas `result-{p}.ext` no Tekt. Vanilla grava `result-1.ext` e `result-2.ext`. Controles PNG de 72/144 PPI coincidem em dimensões. | `03_infra/src/pipeline/raster.rs:94` seleciona a primeira página; L0 de CLI/raster deve delimitar migração de nomes/compatibilidade. |
| E18 | Tiling com quatro cores translúcidas: SVG/PNG Tekt coincidem com o controle analítico dentro da tolerância; PDF tem 1160/4800 pixels com erro máximo por canal >2 na escala 0–255, nas junções do período. Não se compara ao corte do vanilla para decidir a composição. | `03_infra/src/export/pdf_tiling.rs`; `00_nucleo/prompts/infra/export/tiling.md:70-88`; `visual.json.gz`. |
| E19 | `html-span-string` é rejeitado no Tekt, enquanto o mesmo body Content funciona nos dois. Vanilla aceita a string. Style raw com UTF-8 deixa de causar panic no novo vanilla; no Tekt a string falha antes na coerção. | `01_core/src/compiler/stdlib/html/elem.rs`; `lab/typst-original/crates/typst-html/src/encode.rs` no delta novo. |
| E20 | Os corpora conservam dez negativos import/include de string/hex com mensagens/âncoras divergentes, três âncoras matemáticas e 20 ocorrências divergentes no corpus de 159 diagnósticos. Também há novos rastros de callback/processamento no vanilla novo. | `01_core/src/compiler/eval/modules.rs`, `01_core/src/compiler/lexer/code.rs`, `01_core/src/compiler/eval/call_dispatch/call_spans.rs`, `01_core/src/compiler/eval/call_dispatch/type_fields.rs` e owners de métodos; a lista de 20 é de ocorrências, não de APIs diferentes. |

## 2. O que corrigir — direção e dependências

Os IDs Pxx referem-se às evidências Exx acima. Prioridade significa impacto e
sequenciamento proposto; não é autorização para alterar API.

| Ordem de trabalho | Correção | Fronteira da mudança |
|---|---|---|
| A1 — P02/P03/P04 | Recuperar getters/campos já transportados; executar callbacks de State/Heading; init opcional; unificar admissão bare/where/counter pela identidade real dos elementos já suportados. | Começar por reparos internos com L0 primeiro e testes causais. Presença que já foi colapsada exige o contrato correspondente. Não ampliar tabelas por nome de função ou aceitar callback sem executá-lo. |
| A2 — P05 | Preservar CSL sob receitas/carriers, aplicar hanging-indent e manter o isolamento do parágrafo externo. | A troca observada para fallback é funcionalmente errada. A busca incompleta em convergence é hipótese causal forte a confirmar inspecionando a árvore/carrier e o cache. O refutador seria demonstrar que o cache se perde por outro caminho. Acrescentar somente o seletor não fecha o recuo. |
| A3 — P11/P12/P13 | Corrigir overflow/tipo de literais, espaço introduzido por include e igualdade semântica de atributos/identidade std. | Não trocar promoção por `Int::get` default zero; não apagar wrappers indiscriminadamente; não alterar PartialEq Rust/Hash/terminação só para igualar `==` da linguagem. Preservar todos os 24 controles Styled. |
| A4 — P09/P10/P16 | Corrigir geometria de sub/super, reserva de altura/estilos de Heading, precedência de regras idênticas, show local CJK, eixo nativo com attachments e transporte de função em argumentos matemáticos. | Fórmulas/transportes internos podem seguir fluxo contínuo se preservarem contratos e fases. Mover defaults ou validação entre eval/layout exige análise e gate ADR-0127. Não substituir variantes/assembly por escala visual contínua. |
| A5 — P17/P19/P20 | Exportar todas as páginas com templates corretos; aceitar coerções HTML válidas; alinhar mensagem, hint, ocorrência e cadeia causal dos erros. | Avaliar compatibilidade dos nomes de arquivos. Priorizar erro/resultado incorreto antes da diferença de aspas. Não atribuir todos os erros de callback apenas a apresentação de traces. |
| B1 — P01 | RM1: transportar abs+em+percentual em h/v/box, sem truncagem. | Os carriers públicos hoje não comportam a informação completa. Preparar L0s, migrações e revisão ADR-0127. Block relativo já preserva a geometria e não deve regredir. Aceitação de números sem unidade é decisão separada. |
| B2 — P07 | RG1: grupos ordenados, ocupação/spans, validação e efeitos, metadados, x/y de linhas, stroke:none e overhang. Replanejar também RG2/RG3. | RG1 tem proposta pendente de aprovação; rebasear sobre b1b486f3 e a referência nova. Não copiar candidatos antigos nem considerar toda Table resolvida pelo RG1. Campos, repetição multipágina, rich properties e PDF/UA têm frentes próprias. |
| B3 — P08/P14/P15 | Paint textual completo, Figure kind/caption tipados e Symbol com morfologia preservada. | Se a solução mudar payload/campo/variante pública, preparar revisão e obter gate. TS1 e o transporte de FramePaint aprovado não autorizam automaticamente contratos públicos adicionais de texto/FG1/SY1. |
| B4 — P06 | Corrigir formato de document para a linguagem atual: strings e auto, reflexão e fronteira entre construir valor e exportar. | O L0 atual ainda prescreve Func e a guarda antecipada. Atualizar L0 antes do código; mudança de carrier público/fase/compatibilidade exige revisão. A função pode ser estudada como alias legado explícito, mas não substitui suporte à sintaxe do vanilla. |
| C1 — P18 | Resolver junções PDF de tiling com controle analítico e ampliar amostragem subpixel/alpha variável. | Manter composição por contribuições. Sem copiar recortes do vanilla ou tentar alinhar pixels deslocando fase. Qualidade subpixel continua uma frente mesmo quando o processo não cai. |

A reúne reparos inicialmente internos; B reúne frentes com preparação de contratos
públicos provável. A classe final depende da solução concreta e dos L0s donos.
Os jobs de validação devem continuar sequenciais sob o limite de RAM.
RG1 é o único desses pacotes cuja revisão pública já está pronta; permanece sem
aprovação. Os demais precisam de revisão concreta se acionarem ADR-0127.

## 3. O que faz mais sentido preservar no Tekt

### Confirmado por comportamento e fonte

1. **Composição do overlap de tiling.** Com quatro contribuições vermelhas a
   50%, Tekt PDF/SVG/PNG produz (255,15,15), coerente com alpha acumulado de
   93,75%. O novo vanilla continua em (255,127,127), uma contribuição. A fonte
   pinada `visualize/tiling.rs:111-115` descreve overlap; o contrato Tekt e o
   controle analítico sustentam conservar a composição. Não deduzo intenção
   upstream do resultado: a recomendação seria refutada por um contrato que
   exigisse o corte ou por falha do controle analítico. As junções PDF de quatro
   cores continuam dívida separada, portanto não é um passe livre para todo tiling.
2. **Guardas subpixel.** No mesmo input de período 0,1pt/corpo 0,05pt, o PNG do
   vanilla novo termina em panic 101; Tekt conclui. Preservar o tratamento seguro.
   `typst-render/src/paint.rs:299-302` arredonda dimensões e usa unwrap. Sucesso
   do Tekt não prova integração de área correta: no centro ele mede (255,207,205)
   contra média analítica ideal `RGB(255, 191.25, 191.25)`; os renderers PDF/SVG também
   têm resultados diferentes da média. Definir amostragem/qualidade sem copiar o panic.
3. **SVG bem formado de texto em tiling.** ET e librsvg aceitam o Tekt; o SVG
   novo vanilla dessa fixture é recusado. Preservar a validade do Tekt, sem alterar
   o oráculo ou copiar caractere inválido para atingir igualdade de bytes.
4. **PNG/PPI já correto.** No bundle de 2pt, Tekt e vanilla novo geram 4×4px
   por padrão e 2×2px com 72 PPI. O anterior gerava 288×288 e 144×144. A atualização
   upstream eliminou uma divergência onde o Tekt já fazia sentido; manter o cálculo.
5. **Igualdade Styled separada do render.** Os 24 controles continuam corretos.
   Preservar o tratamento de estilos de render e a distinção de wrappers no nível
   da linguagem. Corrigir o supplement de Heading não autoriza comparar toda a
   estrutura Rust nem apagar todos os estilos/campos.
6. **Mecânica Tekt.** Enum Content fechado, despacho estático, L1 puro, I/O em
   L3, transportes tipados, compartilhamento Arc e vistas lexical/preparada/visível
   podem permanecer. Não há evidência nesta auditoria que exija copiar a árvore,
   vtables, algoritmo de passes ou caches do vanilla. Esta é classificação
   arquitetural, não benchmark de ganho de RAM ou velocidade.

### Extensões/compatibilidade que pedem decisão explícita

- **Int/Float como pt em h/box e outros callers.** A aceitação consta dos L0s
  históricos; o vanilla rejeita. Faz sentido preservar compatibilidade enquanto
  se decide uma política explícita. A conveniência é inferência ergonômica, não
  prova de que seja mais correto que a linguagem Typst. Não remover em massa e
  não atribuir crédito de paridade. Percentual que vira zero não pertence a esta exceção.
- **`document(format:format.html)`.** Hoje é contrato legado Tekt e não linguagem
  do novo pin. Pode ser considerado um alias de compatibilidade, documentado e
  distinguido da sintaxe padrão, se o dono assim decidir. Não justifica rejeitar
  `"html"`/auto nem evitar a revisão do carrier/fase.
- **Nomes internos de fonte/objetos PDF e pequenas diferenças de raster.** A
  matriz vigente manteve dois DIFFERENCE conhecidos: 11 pixels com delta máximo
  um e nome interno de fonte diferente. Não priorizar clonagem de bytes/metadados
  internos quando texto, geometria e semântica do caso coincidem. Isso não
  autoriza tolerar perda de páginas, texto incorreto, clipping ou tinta errada.

## 4. Escopo adiado e pontos ainda por medir

Estes itens não são vantagens do Tekt só por estarem documentados:

- **SVG/PDF como fonte de image** e **MathML em HTML**: as fixtures válidas são
  aceitas pelo novo vanilla e recusadas pelo Tekt. São lacunas de produto
  conscientemente adiadas (DEBT-67 e correlatas), a priorizar conforme os documentos
  alvo. Manter a separação L1/L3 ao implementá-las; falta de suporte não é paridade.
- **Padding negativo**: o Tekt rejeita por política P156C/L, o vanilla aceita.
  Reabrir a decisão de escopo se a meta incluir esses documentos. Não há nesta
  medição demonstração de que a restrição seja superior; não removê-la por troca
  textual sem revisar L0 e compatibilidade.
- **Mappers genéricos de Cell**: a auditoria anterior distingue a vista ativa
  resolved da standalone/propriedades. Ainda falta contraprova de linguagem por
  caller para decidir qual vista cada operação deve transformar. Não trocar a
  prioridade global só porque o vanilla tem estrutura diferente. Ver
  `tekt-v16-bibliography-static-2026-10-17/generic-cell-visual-audit.md`.
- **Cobertura ampla** de repetição Table multipágina/PDF-UA, rich properties,
  alpha variável/extremos, imagens/EXIF e demais formatos continua exigindo
  corpora próprios. O delta de 20 commits inclui ajustes de fontes, marcação PDF
  e fluxo; inspeção de diff não certifica essas superfícies nem cria defeito por si.

Dois débitos históricos **não foram reproduzidos nos controles atuais**:
exposição de std.html sem feature (raiz e alias dão agora o erro correto), e
cor representativa constante no PNG da célula com gradiente parent (scan mede
16 cores nos dois backends e o centro coincide). Não repetir esses itens como
bugs abertos a partir dos relatórios antigos; a amostra não fecha todo HTML ou
qualquer gradiente/alpha possível.

## 5. Limites e próximo passo

O inventário usa a linguagem e observáveis visuais/erros, não igualdade Rust.
Campos/JSON, extração PDF e diferenças de árvore foram classificados, não
promovidos automaticamente a defeitos. Por exemplo, peças de assembly vistas no
bbox não devem ser clonadas; a evidência de stretch aqui usa avanço/posição de
A/Z e attachments, além do contrato de eixo nativo.

Os 106 V16 da última varredura continuam sendo dívida de arquitetura, não 106
falhas de paridade. Há coincidências úteis, como a busca parcial de estilo CSL,
mas não relação 1:1. A suíte de 8932 aprovados e a matriz de seis perfis do
re-sync continuam evidência daquele freeze, idêntico por hashes; não foram
reexecutadas nem apresentadas como novas nesta análise. Elas não cobrem todas
as dívidas E01–E20.

Próximo passo recomendado: fechar o escopo do lote A1/A2 com os L0s donos,
conservar os controles, escrever RED funcional e só então implementar. Em
paralelo de planejamento, rebasear/revalidar a proposta RG1 e preparar os
contratos RM1/FG1/SY1/Paint que precisarem de gate. Nenhuma API mudou nesta etapa.

Os sete jobs de medição tiveram MemoryMax=12 GiB, MemoryHigh=10 GiB,
MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Pico registrado: **495742976 bytes
(0,462 GiB)**, sem eventos max/OOM/OOM-kill. Inclui memória do cgroup, não um
benchmark comparável ao pico da compilação/suíte. Receitas, contador por job e
horários estão em `resources.json.gz` e no pacote de evidências.

## 6. Integridade do diagnóstico

`owners.json.gz` registra os L0s donos e hashes das fontes consultadas.
`evidence.json.gz` preserva fixtures, triagem e recibos brutos de recursos;
`final-state.json.gz` registra o encerramento UTC e a conferência dos 2105
arquivos protegidos e três binários contra o freeze inicial.
`manifest.sha256` sela o relatório, scripts e evidências comprimidas.
Não houve mudança produtiva nem execução de uma nova suíte Rust nesta etapa.

Verificação sem executar os produtos: `python3
00_nucleo/diagnosticos/paridade-novo-vanilla-2026-10-17/verify.py` (na raiz).
