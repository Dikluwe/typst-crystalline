# Auditoria histórica: medição de texto, quebra de linha e paginação

A auditoria encontrou limitações concretas no percurso funcional, mesmo depois das correções anteriores e das suítes verdes: opções de parágrafo/hifenização sem efeito, medição de tracking incoerente com os glifos emitidos, palavra longa não hifenizada na origem da linha e página terminal vazia. O algoritmo otimizado é uma capacidade ausente neste caminho; não é a explicação única das outras falhas.

Este documento é diagnóstico. Não altera L0, código produtivo ou o pin de paridade e não autoriza implementação por si só.

## Medição e proveniência antes da classificação

Estado: working tree não commitada sobre `17f13b1df79b87ea065ee62e78164b18e477da3d`, com as correções anteriores já materializadas. O estado dos arquivos produtivos e prompts permaneceu igual durante a auditoria; hashes, lista exata de alterações, `git diff HEAD --stat`, UTC e binários estão em `auditoria-quebra-paginacao-2026-09-28/proveniencia.json` e `final-receipt.json`.

Oráculo: vanilla `9f2b6e8715237cb086899a42873660fe744622e8`, árvore ratificada `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. As duas fontes upstream usadas para classificar o algoritmo foram comparadas byte a byte com o pin local, em `history.json`. A receita do binário está no diagnóstico P1489.

Foram medidas **63 entradas** fonte→PDF nas duas implementações, em três lotes: 30 sondas iniciais, 12 controles de configuração/origem e 21 controles de tracking/altura. Um caso é deliberadamente inválido: `linebreaks: "nonsense"`. São entradas de auditoria, não 63 testes de paridade aprovados. Mesma Noto Sans versionada, tamanho 12pt e dimensões explícitas; ASCII/latim e ligaduras dessa fonte não exigem outra família. A flag font-path é comum; não se reivindica isolamento universal de fontes do sistema.

Resultados completos: `resultados.json`, `resultados-controles.json`, `resultados-isolamento.json`, fontes `.typ`, PDFs, XMLs e stderr no diretório da auditoria. `pdftotext -bbox-layout` fornece grupos `<line>` do extrator; em texto de tamanhos mistos esses grupos não equivalem necessariamente às baselines físicas. Não usamos sua contagem para declarar um defeito de baseline. Contagem de páginas e presença de uma página branca foram confirmadas independentemente.

A sonda externa `layout-probe.rs.txt` usa as APIs públicas atuais de eval, layout com métricas reais e shaping. Não altera os consumers nem substitui toda a pipeline com callbacks/introspecção. Nos casos simples examinados, reproduz as páginas do PDF e distingue o documento de layout dos efeitos de exportação. Receita, bibliotecas e hash do executável estão em `layout-probe-recipe.json`.

## A ordem de construção, lado a lado

| Responsabilidade | Vanilla: fonte histórica verificada | Tekt: fonte histórica verificada |
| --- | --- | --- |
| Primeira fatia | `5a600eb354`, 12/02/2019: reúne texto e gera uma página A4; não contém planejamento tipográfico de linhas/páginas | `8658aa0666`, 28/03/2026: mede palavras, quebra localmente e avança páginas; entrada simples usa FixedMetrics; há entrada separada de fonte real |
| Medição e quebra local | `0274e9381`, 26/05/2019: TextUnit guarda caracteres/fonte/larguras; TextFinisher faz first-fit e emite ações; não desmembra palavras | `0715427c6`, 24/04/2026: acrescenta hifenização condicionada a lang e ao estouro local, escolhendo o maior prefixo que cabe |
| Construção do layout | `968e12169`, 21/06/2019: evolui os layouters de flex/box/texto; diffs arquivados não comprovam completude de paginação | `a245f2dbbb`, 03/07/2026: integra FallbackFontMetrics no caminho produtivo PDF; não é a primeira existência da API de métricas reais |
| Shaping | `730715c064`, 03/10/2020: marco de reorganização do texto em shaping, com diff arquivado | `0944df74ab`, 07/07/2026: usa largura shaped na decisão de quebra; `7e5f60607`, mesma data: unifica a autoridade text_width |
| Alternativas de algoritmo | `288a926fea`, 14/03/2022: mantém Simple e acrescenta Optimized com custo/predecessores; `711a2d8807`, 18/10/2024: revisão do otimizado sem justificação | O cursor atual ainda fecha a linha no estouro da palavra/fragmento; não existe consumo de linebreaks nessa decisão |
| Tracking | O pin atual é o observável desta auditoria; não foi datada a primeira implementação upstream de tracking | `c147be1af`, 08/07/2026: emission aplica tracking entre clusters; a autoridade text_width conserva contagem de codepoints |
| Superfície de parágrafo | O pin atual realiza parágrafos e respeita configuração de algoritmo | `c98ffc8ac`, 21/07/2026: materializa native_par com limitações; L0 vigente continua legitimando devolução direta do body e propriedades ignoradas |

Os diffs completos, hashes e datas de autor/commit estão em `history/` e `history.json`. A sequência compara responsabilidades; não mede produtividade nem atribui intenção. O marco de fevereiro de 2019 não deve receber retroativamente Knuth–Plass, hifenização ou todas as garantias modernas.

## Achados com refutadores

### 1. Configuração aceita sem produzir o comportamento pedido

**Medição:** `short-100-simple` e `short-100-optimized` usam as mesmas palavras curtas e `lang: "zz"`, eliminando hifenização inglesa como explicação. Tekt produz as mesmas quebras nas duas opções. Vanilla simples coincide com Tekt; vanilla otimizado altera as quebras. Por exemplo:

- Simples/Tekt: `The old fox` / `and the big` / `dog run by the`.
- Otimizado vanilla: `The old fox and` / `the big dog` / `run by the sea.`.

Os dois PDFs têm o mesmo número de grupos de linha nessa entrada; isso não elimina a diferença de comportamento. No controle de largura 115pt também há diferença; outros controles coincidem, demonstrando que comparar apenas um parágrafo não distingue estratégias.

`invalid-linebreaks` termina com erro vanilla `expected "simple", "optimized", or auto`. Tekt compila e avisa que a propriedade ainda não é suportada. Os stderr foram preservados: o caminho `#set` não ignora silenciosamente o pedido.

**Fonte/L0:** `compiler/eval/rules.rs:2972–3010` transporta leading/spacing, mas avisa e descarta as outras propriedades de par. `stdlib/structural/par.rs:119–126` aceita e ignora parâmetros da chamada nativa. Seu L0, `prompts/compiler/stdlib/structural/par.md`, declara explicitamente `justify` aceito e ignorado e parágrafo implícito. O cursor `layout/cursor.rs:195–251` toma decisões locais sem receber o parágrafo completo; `layout/cursor.md` manda conservar esse wrapping. O vanilla pinado seleciona Simple/Optimized em `typst-layout/src/inline/linebreak.rs:152–160`; resolve o default por justificação em `inline/mod.rs:194–196`.

**Classificação:** algoritmo/estrutura de dados são mecânica. Efeito de linebreaks/justify, fronteira de parágrafo e erro de argumento são observáveis de linguagem. Adicionar apenas um otimizador não fará uma propriedade descartada chegar a ele.

**Refutação possível:** demonstrar um consumer produtivo atual de linebreaks e reproduzir alteração das quebras nos controles. A busca e o rastreio desta rodada não encontraram esse caminho.

### 2. Hifenização depende de lang, sem honrar hyphenate

**Medição:** `hyphen-false` e `hyphen-true` produzem o mesmo resultado Tekt, incluindo `A extraordi-` / `nary interna-` / `tionalization`. No vanilla, false impede essa hifenização e true a permite. A diferença entre hífen literal Tekt e U+00AD extraído do vanilla não é usada sozinha como gate de paridade.

**Fonte/L0:** `eval/rules.rs:3407–3420` reconhece o nome hyphenate como válido mas avisa scope-out e não o transporta. `cursor.rs:218–246` decide apenas pela existência de lang e dos pontos fornecidos por `hyphenation.rs`; o L0 Cursor conserva esse fallback. Não há controle efetivo de hyphenate no TextStyle deste caminho.

**Classificação:** a configuração não produz o efeito pedido. O aviso atual é evidência de escopo adiado, não um defeito escondido nem prova de motivação dos autores. A decisão preservada no contrato impede resolver isso apenas rearranjando arquivos.

### 3. Palavra longa na origem escapa do percurso de hifenização

**Medição:** `long-word-alone`, página 80pt com margens 8pt e hyphenate true, emite `internationalization` inteira no Tekt. A sonda de layout mede 110,22pt para uma região útil de 64pt; o texto continua integral nos itens de layout, mas excede a página. A extração recorta a palavra visível. Vanilla a distribui por três linhas com hifenização. O controle `unknown-lang-longword` faz ambas as implementações excederem a largura quando não há padrões; não se exige hifenização onde ela não está disponível.

**Fonte/L0:** `cursor.rs:212–216` só entra na tentativa de hifenização quando cursor_x é maior que a margem esquerda. Na origem, empurra diretamente a palavra inteira. Essa condição já tem antecedente no first-fit Tekt inicial; a hifenização posterior ficou dentro do ramo de estouro com prefixo.

**Classificação:** não se perdeu o conteúdo de eval/layout; a geometria excede a região apesar do pedido válido de hifenização. Não se propõe repartir arbitrariamente todos os codepoints ou copiar um defeito histórico vanilla.

### 4. A largura usada na decisão não é a largura emitida com tracking

**Medição:** Noto Sans, `office`, tamanho 12pt, tracking 3pt:

| Observação da sonda | Largura |
| --- | ---: |
| FontMetrics.text_width | 46,14pt |
| Soma dos advances de quatro glifos shaped | 40,14pt |
| Diferença | 6pt |

Sem tracking, a largura observada é 31,14pt. `layout-probe-results.txt` registra as duas autoridades usando o mesmo World/style. Os controles `ligature-80-0` coincidem nas quebras; com tracking 3pt há diferença entre as implementações.

**Fonte/L0:** `metrics.rs:249–262` acrescenta tracking por pares de codepoints; `shaper/shaping.rs:301–319` acrescenta por mudança de cluster, conservando ligaduras. Ambos os L0 determinam essas políticas locais. A medição shaped em `shaping.rs:38–106` não incorpora tracking; a soma entra depois em text_width. `pipeline/paged.rs:471–472` faz shaping e reparo de posições após o layout; `shaper/reconciliation.md` proíbe recomputar decisões de layout nessa passagem.

**Classificação:** as representações podem divergir, mas a autoridade de extensão usada antes de escolher a quebra deve ser coerente com o conteúdo emitido. O reparo posterior de x não reconsidera quebras/páginas. Não se atribui toda divergência dos PDFs de tracking a esses 6pt: espaçamento nas fronteiras de palavras e propriedades ignoradas são variáveis adicionais. O exemplo comprova a incoerência de extensão, não causalidade exclusiva de todas as quebras.

**Refutação possível:** outra medição com mesmo font/style mostrar estimate==real, ou evidência de que a diferença é uma reserva intencional observável de linguagem. O L0 atual a descreve como largura de texto, não como reserva.

### 5. Fechar a linha terminal cria uma página sem conteúdo

**Medição:** `paragraphs` gera duas páginas Tekt contra uma vanilla; `mixed-size` gera cinco contra quatro. As páginas extras são uniformemente brancas no raster a 72 dpi (`terminal-page-render.json`). A sonda eval→layout reproduz respectivamente Page 2 e Page 5 com **zero items**, antes do shaping/PDF. Remover o newline terminal não elimina o problema. Os controles de altura 110/120pt e de remoção da quebra de parágrafo deixam de produzir a página extra.

**Rastreio causal:** `mod.rs:1924–1940` fecha a última linha via flush_line; `line_commit.rs:696–703` pode chamar new_page quando o avanço para a próxima baseline excede o fundo, depois de drenar a linha. O finish então recompõe uma página atual vazia pelo caminho normal. O L0 LineCommit contém uma proteção específica para linha fracionária terminal, mas não uma garantia geral para o texto terminal comum. O L0 do hub e a correção da página inicial vazia não resolvem esse caso posterior.

**Classificação:** página adicional é observável de linguagem/layout. É independente do writer PDF e não é evidência de perda das palavras. Separar tinta/linha atual de uma próxima baseline hipotética é o candidato causal; a correção ainda exige L0 e refutadores de continuação, pagebreak explícito, colunas e floats.

## Atomização por responsabilidade e alternativas

A segmentação de scripts (`word_segment`) e geração de pontos de hifenização (`hyphenation`) já têm owners próprios. Não é preciso reagrupá-las com o algoritmo para ganhar legibilidade. As fronteiras úteis a desenvolver, com contratos medidos antes de código, são:

| Unidade de funcionamento | Responsabilidade a tornar explícita | Relação com o que existe |
| --- | --- | --- |
| Preparação/medição do trecho | Estilo, extensão e oportunidades coerentes com o shaping; distinguir espaços, clusters e pontos de quebra | Metrics, shaper e Space hoje conservam partes da política em momentos diferentes |
| Seleção de quebras | Receber oportunidades/extensões/restrições e devolver quebras; simple e optimized podem compartilhar o mesmo uso com ganhos/custos distintos | Cursor hoje mede, escolhe prefixo, fecha e emite no mesmo percurso |
| Composição da linha | Baseline, ascent/descent, justificação e transporte dos itens usando as quebras já escolhidas | LineCommit já é owner separado; não deve reimplementar seleção |
| Progresso regional | Comitar conteúdo que cabe, avançar somente quando houver necessidade real de continuidade | Cursor/new_page e fechamento terminal precisam de fronteira causal mais clara |
| Integração de configuração | Transportar pedidos de linguagem; wiring compõe a implementação sem calcular custos, métricas ou paginação | Propriedades descartadas devem ser resolvidas antes da troca de peças |

Isso é proposta de investigação para futuros L0, não um novo contrato autorizado. Respeitar ADR-0109: funções/unidades na camada dona, despacho estático e exaustivo, sem dyn/vtable/PropMap nem lógica de render nas entidades. Cada consumer materializável terá owner 1:1 (ADR-0129). Compartilhamento de invariantes, se necessário, usa núcleo pinado; não um prompt dono de vários módulos.

A ideia de duas peças com o mesmo uso é sustentada pelo histórico vanilla Simple/Optimized e pelos controles atuais. Não há benchmark nesta auditoria: não se declara vencedor, desempenho de estado da arte ou troca automática superior. Alterar contrato público, default ou fase requer gate ADR-0127; correções internas comprovadas seguem L0 primeiro e RED→GREEN.

## Ordem de correção recomendada e cobertura restante

1. Refutar e corrigir página terminal vazia, preservando continuação e pagebreak explícito.
2. Unificar a autoridade de extensão/tracking, incluindo fronteiras e ligaduras, antes de otimizar a seleção.
3. Transportar/validar hyphenate e configurações de parágrafo; tratar a palavra longa também na origem quando houver oportunidade válida.
4. Definir a preparação do parágrafo e contratos separados de seleção/composição; só então avaliar simple/optimized e sua composição no wiring.

Concluiu-se a auditoria deste recorte com achados reproduzíveis e candidatos de fronteira. As limitações encontradas **não foram corrigidas nesta rodada**. Permanecem fora do fechamento: UAX #14 completo, CJK/Thai/RTL com fontes controladas, penalidades globais/orphans/widows, shapes/links/caixas no planejamento do parágrafo, paginação de floats/footnotes/colunas e benchmark comparativo. Parsing/erros e o exportador completo também continuam pendentes na auditoria da primeira fatia. Suítes verdes anteriores não certificam esses comportamentos.
