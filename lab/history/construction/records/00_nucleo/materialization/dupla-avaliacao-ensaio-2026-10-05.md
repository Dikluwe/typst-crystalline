# Ensaio de avaliação única com realização explícita

## Estado e finalidade

**Ensaio implementado e medido em 2026-10-05.** [Resultado e recibos](../diagnosticos/dupla-avaliacao-implementacao-2026-10-05.md). A avaliação comum ocorre uma vez; a segunda passagem de layout continua. Ganho no cálculo pesado: 42,1%; tabela simples sem ganho demonstrado; tabela com show textual 1,7% mais lenta. As seções abaixo preservam o planejamento e o gate de origem.

Pedido do usuário: implementar a conservação de identidade durante transformação e medir se melhora. Preferências mantidas: código explícito, Rust seguro, enum fechado, domínio puro em L1. A investigação encontrou necessidade de mudar a fase de aplicação de show e acrescentar a fachada pública de realização. Este passo coordena a materialização; não é L0.

Baseline de origem: HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`. Código produtivo e L0 existentes não foram alterados nesta preparação. A especificação nova é `00_nucleo/prompts/compiler/realize/mod.md`, owner futuro exclusivo `01_core/src/compiler/realize/mod.rs`. Não existe consumer materializado ainda; a relação produtiva 1:1 precisa ser fechada na materialização antes do gate final do linter. O protocolo admite preparação de L0 antes do código; não declarar V15/V26 limpos enquanto houver contrato futuro sem consumer.

## Gate antes da implementação

A ADR-0127 exige confirmação humana antes de novo contrato público e de mudança de fase. O gate deste ensaio cobre a nova fachada `realize_document`, seu resultado `RealizedDocument`, o transporte explícito de receitas no Content e a transferência de aplicação de show de eval para realização pura. A opção de mover essa lógica não foi tratada como correção interna de fórmula.

O gate não aprova quebra de linguagem, unsafe, macros de despacho, nova flag ou novo padrão de produto. Não autoriza consertar outros gargalos nem ressellar deriva anterior sem auditoria.

## Matriz de ownership a completar antes de cada alteração de código

| Responsabilidade | L0 proprietário | Mudança necessária |
| --- | --- | --- |
| Resultado/fachada de realização | `00_nucleo/prompts/compiler/realize/mod.md` | Materializar owner L1 exclusivo e sua exportação de módulo |
| Transportes no enum Content | `00_nucleo/prompts/entities/content.md` | Declarar payloads e variantes concretos, com ambientes imutáveis e proveniência |
| Projeções, morfologia, identidade, hashing e travessias | Owners em `00_nucleo/prompts/entities/content/` | Especificar transparência de transportes internos; não renderizar original consumido |
| Coordenação do eval | `00_nucleo/prompts/compiler/eval.md` | Uma passagem; montagem da base uma vez; retirar exigência de duas execuções |
| Captura de receitas/estilos | Owners em `00_nucleo/prompts/compiler/eval/rules/` e `markup_stream.md` | Preservar snapshots lexicais e grupo textual sem aplicar show antecipadamente |
| Matching e terminação | Owners de `selector_matching` e `show_rule_termination` | Reutilizar algoritmo vigente, sem copiar mecânica divergente do vanilla |
| Introspecção | `00_nucleo/prompts/compiler/introspect/walk.md` e owners de populate/locatable | Identificar ocorrências e transportar campos e âncoras do original e dos gerados |
| Query | `00_nucleo/prompts/infra/query-helpers.md` | Usar o índice realizado e retirar escolha por cardinalidade |
| PDF, HTML, bundle, contexto | Owners em `00_nucleo/prompts/infra/pipeline/` | Coordenar realização e snapshot; não reavaliar programa comum |
| Compensações de matrizes | Owners de `call_dispatch/invoke` e `math/native_dispatch` | Testar execução causal antes de retirar isolamento/rollback |

Os owners existentes só recebem mudanças normativas depois do gate desta arquitetura e antes de seu respectivo código. O transporte concreto precisa ser declarado no owner Content antes de materialização; a fachada não legitima alterações em consumers de outros prompts.

## RED e validação de linguagem

Reutilizar os controles atuais da auditoria como ponto inicial: um original produzido em texto, um heading gerado e dois headings gerados. Os dois últimos já são RED no pin atual. Acrescentar casos de receitas que devolvem it, duas inserções, headings com corpos iguais, labels, nested show, where/show-set e estilos locais. Testar identidade e ordem por observáveis de linguagem; não igualdade Rust ou estrutura de wrapper.

Um World de teste registra acessos e callbacks para demonstrar execução real do programa, não apenas valores no scope final. Cobrir erro, warning, loops, includes, imports e matrizes. Conteúdo contextual pode reexecutar contribuições dependentes do snapshot; não confundir isso com a dupla avaliação global.

## Medição antes e depois

Conservar um executável baseline separado, seu SHA-256 e a fonte exata de cada fixture antes de rebuild. Usar os controles da auditoria e fixtures de cálculo, texto e tabela do estudo macro. Medir compilação completa com pares intercalados, warmup e opções iguais; medir CPU da avaliação separadamente quando a sonda permitir. Registrar memória e trabalho repetido quando houver instrumento adequado, sem alegar valores não medidos.

Registrar commit/working tree, arquivos alterados, horários, hashes, comandos e amostras em `.json.gz`. Comparações de desempenho precisam excluir casos com divergência linguística/geométrica conhecida ou qualificá-la explicitamente. Fontes e exportação continuam presentes no tempo da CLI; não atribuir a economia inteira ao eval.

## Critério de encerramento

RED→GREEN das sentinelas, contagem demonstrando avaliação comum única, validação dos targets e dos efeitos, build e lint com ownership/pins/hashes coerentes. Relatar ganho, ausência de ganho ou regressão sem meta arbitrária. Nenhuma frente é considerada concluída só porque a nova variante compila ou uma fixture fica mais rápida.

Gate confirmado pelo usuário em 2026-10-05: SHA-256 `5738955f3265e8062ba864e65e61b3999a1260b695969df5676c6cc9c9da10ab`; implementação e medição autorizadas.


## Fechamento do ensaio

Fachada e transportes materializados em L1, com identidade/original/visual explícitos e sem unsafe novo. Introspecção e query conservam originais e gerados; aliases preparados mantêm identidade e a ordem causal é registrada separadamente. Contextos de receitas usam o piso reservado de IDs. Medidores projetam o visual sem contar o original consumido.

Passaram 6.504 testes de núcleo (5 ignorados), 1.143 de infraestrutura, build release, as sondas de linguagem delimitadas e a geometria pareada das seis fixtures. Linhagem dos 52 consumers alterados sem V5/V6/V15/V26; gate ownership/núcleos limpo. Lint global sem erros, porém com avisos anteriores fora do escopo, portanto o critério global de zero violations não foi alcançado. Não ressellamos esses owners sem auditoria.

O diagnóstico final conserva o estado exato, binários, medições completas, controles inválidos intermediários e limitações. As compensações de matriz e a otimização do segundo layout permanecem para frentes independentes.
