# Base vanilla de 2019 comparada ao Tekt

## Base e método

A base de entrada da auditoria histórica é o protótipo vanilla `5a600eb354c65ec008cbf020e45705c2f401d669`, de 12/02/2019. Ele será acompanhado desde esse estado, por funcionamento, até o pin upstream ratificado. Esta escolha **não muda o alvo de paridade** `9f2b6e8715237cb086899a42873660fe744622e8` nem exige reproduzir defeitos de 2019.

Comparamos três estados Tekt: início próprio `8260bcd642d6eb55747288067287dbc4aac9d757` (22/03/2026); estado agregado P10–P23 `8658aa0666427406ec0e9a0799c12df55df714f7` (28/03/2026); working tree atual sobre HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`. O primeiro é estrutura inicial; o segundo permite comparar uma fatia funcional. Não tratamos código upstream em quarentena como implementação Tekt.

A evidência é leitura de fonte, não execução dos snapshots antigos. Os blobs históricos foram exportados sem alteração para [comparacao-base-2019-tekt-2026-09-28](comparacao-base-2019-tekt-2026-09-28/proveniencia.json), com commit, blob e SHA-256. A proveniência registra timestamp UTC, working tree não commitado, lista exata de alterações e `git diff HEAD --stat`. Não houve alteração de código produtivo ou Prompt L0.

## Medição antes da comparação

Na base vanilla, `src/lib.rs:6–11` publica parsing, documento, fontes, PDF e utilitários. O teste `src/pdf.rs:353–359` compõe `tokenize → parse → generate → write_pdf`. Em `src/doc.rs:130–155`, a geração reúne palavras numa string, descarta newlines, escolhe Helvetica e cria uma página A4. Toggles de itálico/negrito/math e chamadas de função chegam a `unimplemented!()`. O PDF fixa fonte/tamanho/posição e escreve o texto diretamente em `src/pdf.rs:266–275`; fonte carregada chega a `unimplemented!()` em `:240`. O leitor de fonte retorna nome, não uma medição tipográfica do texto (`src/font.rs:12–23,145–150`). Essas observações delimitam o protótipo; ter todos os módulos não significa linguagem completa.

No início Tekt, `01_core/src/lib.rs:7` exporta somente entities. O L0 histórico `00_nucleo/prompts/core.md:5–33` lista FileId/SyntaxKind/Span e dependências ainda bloqueadas. Shell e infra têm somente headers de linhagem. `04_wiring/src/main.rs:7–10` apenas imprime a mensagem de migração. São resultados explícitos daquele escopo, não evidência de defeito por si mesmos.

No P10–P23 Tekt, `01_core/src/rules/layout.rs:202–205` já oferece layout com FixedMetrics; `:370–420` contém teste com MockWorld, eval e layout. `03_infra/src/layout.rs:18–29` oferece uma entrada separada com fonte real e fallback. `03_infra/src/export.rs:14` exporta PDF; `:161–172` escapa parênteses/barra, mas substitui texto não ASCII por `?`. O wiring ainda contém a mensagem de migração. Todos os caminhos citados estão nos snapshots preservados.

## Lado a lado

| Funcionamento | Vanilla base, 12/02/2019 | Tekt inicial, 22/03/2026 | Tekt P10–P23, 28/03/2026 |
|---|---|---|---|
| Entrada e sintaxe | Tokenização e parsing implementados | Tipos FileId/SyntaxKind/Span; sem parser nas camadas próprias | Lexer, parser, Source e AST disponíveis |
| Transformação em conteúdo | Gera documento simples a partir da árvore | Sem eval ou geração de documento nas camadas próprias | Eval e Content; teste liga fonte ao layout |
| Texto e página | Junta palavras; uma página A4; ignora newlines | Sem layout nas camadas próprias | Layout com posições, quebra de linha e transição de página |
| Tipografia | Helvetica fixa; leitor de nome de fonte separado | Sem infraestrutura de fontes nas camadas próprias | FixedMetrics na entrada padrão; API separada com métricas reais |
| Estilos | Reconhecidos na árvore, geração ainda não implementada | Sem consumer de render nas camadas próprias | Strong/Emph alteram estilo no layouter |
| PDF | Writer disponível e chamado por teste; parâmetros fixos e fonte externa incompleta | Sem exportador nas camadas próprias | Exportador disponível; naquele estado reduz não ASCII a `?` |
| Integração | Teste descreve o percurso fonte → PDF | Executável só anuncia migração | Teste fonte → layout e testes layout → PDF; executável ainda anuncia migração |

A comparação por primeira data mostra **dois pontos de partida diferentes**: vanilla registra uma fatia vertical pequena; Tekt registra primeiro uma estrutura de migração com tipos e contratos. A comparação com P10–P23 mostra que o Tekt logo materializa uma fatia funcional mais ampla. Não se pode derivar produtividade, qualidade ou superioridade de arquitetura das datas ou da quantidade de módulos.

## O que isso revela para a auditoria

### Componente disponível não é comportamento integrado

A API Tekt `layout_with_font` já existia em março. O diff produtivo de `a245f2dbbb4e7cc4305a175f727c4592cfd949b8`, em julho, troca a entrada do pipeline PDF para a variante com `FallbackFontMetrics::new(world)`: [diff preservado](auditoria-historia-vanilla-2026-09-28/diffs/tekt-a245f-pipeline.patch). Portanto, a afirmação correta é **integração posterior de métricas reais nesse caminho de produção**, não ausência total de métricas reais até julho. Outros caminhos históricos precisam de rastreio próprio antes de generalizar.

A mesma distinção vale para 2019: havia leitor de fonte, mas o gerador escolhia Helvetica e o ramo PDF de fonte carregada estava incompleto. Em ambos, a auditoria deve seguir a chamada desde a entrada até a saída, além de procurar a definição da função.

### Teste de uma etapa não fecha as seguintes

O teste Tekt de março usa “Olá mundo”, verifica o layout e aceita que o texto contenha “Olá” **ou** “mundo” (`layout.rs:417–420`). Ele não chama o exportador, cuja conversão ASCII pode perder o acento depois. Essa fonte não prova preservação integral do conteúdo nem preservação no PDF. O teste vanilla de PDF chama o writer e gera arquivo, mas não verifica visualmente o arquivo ou sua extração textual. São evidências de cobertura parcial, não de ausência de todos os testes.

Este é o primeiro cruzamento comportamental útil: para a mesma entrada, registrar conteúdo após parsing/eval, documento após layout e conteúdo após exportação. A aceitação deve acompanhar semântica, sintaxe e morfologia; bytes idênticos e objetos Rust iguais não são requisito geral (ADR-0107).

### Como o Tekt atual se relaciona com a base

O Tekt atual já possui entradas próprias para parsing (`01_core/src/compiler/parse/mod.rs:25`), eval (`01_core/src/compiler/eval/mod.rs:915`), layout com métricas (`01_core/src/compiler/layout/mod.rs:2355`) e exportação (`03_infra/src/export/mod.rs:234`). O wiring atual despacha comandos (`04_wiring/src/main.rs:30`) e sua compilação cria World e chama o pipeline PDF (`04_wiring/src/compile.rs:246,372`). Isso supera claramente o escopo estrutural inicial e a fatia estreita de 2019; existência das entradas não fecha a paridade do produto.

Os limites de smallcaps e quebra local de palavra encontrados na auditoria atual não podem ser atribuídos ao protótipo de 2019: aquela base não implementava esses comportamentos. O cruzamento adequado segue o momento em que cada comportamento nasce no vanilla, depois identifica sua materialização e integração no Tekt. Os exemplos já documentados estão em [histórias lado a lado](historias-vanilla-tekt-lado-a-lado-2026-09-28.md).

## Linha de auditoria adotada

O primeiro eixo é **texto simples → conteúdo preservado → posicionamento/quebra → métricas de fonte → PDF com texto preservado**. Para cada estado histórico, distinguir:

1. Contrato escrito.
2. Implementação local.
3. Implementação usada no caminho escolhido.
4. Teste que realmente percorre esse caminho.
5. Comportamento ainda limitado e decisão posterior que o substitui ou conserva.

Esta sequência compara funcionamento e ordem de integração. A hipótese de que começar por uma fatia vertical completa poderia mudar o desenvolvimento do Tekt permanece uma inferência. O que a refutaria: rastreio histórico mostrar que os caminhos reais e seus testes já eram completos antes das etapas apontadas, ou uma reconstrução controlada com os mesmos contratos não apresentar diferença relevante. Não foi executada reconstrução nesta auditoria.
