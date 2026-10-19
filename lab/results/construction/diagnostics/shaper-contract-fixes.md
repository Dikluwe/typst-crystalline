# Correções dos contratos e consumers de shaper

Corrigidos os quatro defeitos demonstrados pela auditoria anterior: tracking
arredondado na chave, dependência omitida do tamanho, scoring de fallback contra
slots errados e perda de fronteiras de script quando a face é a mesma.

## Comportamento resultante

A emissão calcula uma vez o incremento i32 de tracking em unidades da fonte,
com a mesma fórmula anterior: resolve o Length pelo tamanho, converte usando
units_per_em/tamanho e arredonda. A chave recebe esse mesmo incremento, e os
glifos recebem exatamente esse valor. Valores de tracking com a mesma impressão
decimal mas incrementos distintos não colidem; tamanhos distintos também não
colidem se mudam o incremento. Tracking em em pode reaproveitar o mesmo resultado
quando o incremento efetivo é igual. Não mudou a regra de aplicação por cluster,
o tratamento de tracking negativo ou o último glifo de cada sub-run.

A seleção traduz índices locais de candidatas para slots globais antes de
FontBook::select_fallback e traduz o vencedor de volta para a posição local.
Essa tradução consulta só a associação existente, sem carregar faces dos slots
intervenientes e sem alocar proporcionalmente ao maior slot. Prioridade das
primárias e preferência pelo maior trecho continuam iguais.

A segmentação inicializa o script ativo a partir do primeiro script não genérico
do run bidi quando houver um. Common/Inherited iniciais acompanham esse script;
transições concretas atualizam o script e fecham o sub-run, mesmo usando a mesma
face. Controles de formato conservam o caminho existente, sem consultas globais
adicionais. Clusters emitidos continuam relativos ao texto de cada sub-run.

Os L0s foram editados antes do código e os quatro pares foram ressellados
localmente, incluindo os pins existentes de núcleos no hash efetivo. A correção
é interna sob ADR-0127: não muda contrato público, modo padrão, compatibilidade
ou fase. Não foi feito resselo ou formatação global. Nenhum núcleo foi alterado.

## Validação

Os quatro testes de regressão falharam antes da implementação pelos defeitos
esperados (RED) e passaram depois (GREEN). Novos controles obrigatórios usam
fontes embutidas, sem depender de SKIP por fontes ausentes no sistema.

- Tracking: isolados versus AB/BA e AA, inclusive valores negativos.
- Tamanho: tracking absoluto em 12/24pt versus resultados frescos.
- Reutilização equivalente: tracking em em em tamanhos distintos com glifos iguais.
- Fallback: slots globais esparsos, metadados decoy nos índices locais e slots
  intervenientes sem fontes; vencedor esperado pelo scoring global.
- Scripts: Latin/Greek na mesma face, nas duas ordens, com Common inicial e
  caractere Inherited; exige glifos válidos e fronteiras do contrato.

A suíte final de shaping passou: 54 testes, nenhuma falha, incluindo as três
regressões anteriores dos eixos. O build offline do workspace passou.
O linter não apresentou erros. Permanece no módulo shaping um aviso V16 no
match de ssty, byte-identicamente presente no resultado de lint anterior à
correção; demais avisos globais são V5/V12/V16/V17/V18. Não se declara zero
violations global. Comandos e logs estão neste diretório diagnóstico.

## Escopo preservado e pendências

Alterados os L0s `infra/shaper/cache`, `font_selection`, `shaping` e `tests`, e
seus quatro consumers homônimos. As cópias before e change.patch distinguem
estas mudanças do trabalho preexistente, inclusive a correção anterior de eixos.
A patch tracked de entrada permaneceu idêntica; os arquivos destes owners já
eram adições não commitadas. Fingerprints dos inputs/consumers mudaram exatamente
nos oito arquivos autorizados. Datasets e relatórios selados anteriores ficaram
intactos; seus fingerprints continuam descrevendo o estado anterior.

Língua no buffer, identidade do script math na chave, confirmação de cobertura
no fallback e a divergência documental sobre clone da face base continuam
pendências do diagnóstico anterior. Não foram resolvidas especulativamente
nesta correção. Nenhum novo ganho de desempenho ou paridade global é declarado.
ADRs, fontes, benchmarks, outros L0s e fases do pipeline foram preservados.

## Proveniência

HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitada.
provenance.json/status-before.txt/incoming.patch/diff-stat.txt registram o estado
exato e horário de entrada. completion-audit.json registra o horário final,
hashes dos arquivos e do relatório e a diferença exata de fingerprints.
MANIFEST.sha256 sela os novos logs, protocolo e snapshots. Os tempos impressos
pelo harness de testes não são amostras de desempenho.
