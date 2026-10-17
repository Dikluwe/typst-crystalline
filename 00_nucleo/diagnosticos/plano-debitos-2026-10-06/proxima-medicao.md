# Próxima medição de watch, incremental e macro/micro

Plano de execução autorizado; nenhuma API de retenção escolhida. Owners
vigentes já inspecionados: 00_nucleo/prompts/wiring/watch.md,
00_nucleo/prompts/wiring/compile.md, 00_nucleo/prompts/infra/system-world.md,
00_nucleo/prompts/infra/system-world/cache.md e
00_nucleo/prompts/infra/system-world/fonts.md. Seus contratos atuais conservam
World por compilação; não ressellar ou alargar a duração sem especificação.

## Watch antes de retenção

Usar PDF watch sem servidor para separar o ciclo de recompilação de HTTP.
Controlar no mesmo processo: edição do principal, import, asset read/imagem,
criação de arquivo antes inexistente, remoção, erro transitório e recuperação.
Registrar fonte inicial, mutação única, dependências observadas, saída e último
artefato válido. Distinguir tempo até a publicação da fase de compilação;
intervalo de polling e escrita do harness não são custo do compilador.

Contar constructors de World, aquisição de catálogo/Font e leituras de slots
no binário congelado, por debugger/trace independente dos tempos. Fonte atual
prevê World novo a cada ciclo, mas a contagem dinâmica precisa de seu receipt.
Medir tempo e memória retida/pico junto das fontes carregadas. A referência
vanilla conserva World e reset; copiar sua estrutura não é critério de sucesso.

Antes de desenhar reset, classificar recursos duráveis e por compilação:
Sources/bytes/dependências; catálogo/slots/coverage; inputs/plugins/packages;
leitura de relógio e callbacks; métricas/shaping; índices/contextos/PageStore.
SourceSlot OnceLock e keys por slot impedem compartilhar indiscriminadamente.
Sondar edições de fontes/projeto/inputs e falhas negativas cacheadas. Se a API
concreta nova for pública, preparar L0 e confirmar ADR-0127 antes de código.

## Incremental e ampliação

Para cada unidade do cache-inventory.md, medir hits/misses/trabalho anterior ao
hit, dependências e memória. Revalidar alterações de conteúdo, origem, estilo,
contexto e recurso. Não usar igualdade visual como identidade ou assumir que
Tracked prova memoização. Para células/layout, registrar a guarda responsável
pelo fallback antes de ampliar seu domínio; guardar negativos ainda excluídos.

## Rebaseline macro/micro

Congelar HEAD+working tree+binários/fontes/ambiente e registrar UTC/receita.
Usar os mesmos documentos nos dois motores, qualificando equivalência de
linguagem antes dos tempos. Separar avaliação pesada, texto/bidi/fontes,
tabelas, contexto/páginas, query e ciclos de watch. Contadores/debugger ficam
fora do benchmark; tempos pareados e memória compartilham a mesma rodada.

Revalidar critérios por target: valores/snapshots para query/eval, DOM para
HTML, texto/páginas/caixas e tinta pertinente para PDF/SVG/PNG. Uma rodada nova
não incorpora denominadores nem ganhos históricos. Ausência de ganho, aumento
de memória ou regressão devem ser reportados com a mesma proveniência.
