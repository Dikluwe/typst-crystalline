# Próxima frente: identidade de documento e expressão — 2026-10-06

Este é um passo de execução, não Prompt L0 e não autorização para escrever código.

A evidência está em `00_nucleo/diagnosticos/diagnostic-paths-2026-10-06/identity-next-audit.json`:
Source do documento e Source de parse_code recebem world.main(); o primary usado
pela CLI pode substituir a origem de erro/warning do documento pela expressão.
A especificação vigente de `00_nucleo/prompts/compiler/eval.md` também manda
construir a Source com world.main(). Não presumir que essa cláusula prove intenção
humana de compartilhar identidades no caso contextual.

1. Auditar os owners de FileId, Source, registro/cache de SystemWorld, pipeline de
   expressão e wiring/eval. Confirmar linhagem e lifetime, inclusive a faixa de IDs,
   fontes importadas, stdin e a relação entre id diagnóstico e base de I/O.
2. Especificar nos L0s proprietários a separação das fontes, preservando a base real
   de read/import e os Sources de callbacks. A alocação pertence à infraestrutura;
   o domínio recebe a Source apropriada. Não obter a origem pelo texto do erro,
   igualdade dos textos, outro run do documento ou id arbitrário reservado.
3. Preparar o gate ADR-0127 se a solução precisar de novo método público para
   registro/avaliação de Source. Guardar o L0 canônico e seu hash antes de solicitar
   confirmação; este passo não substitui esse contrato.
4. Testar primeiro os controles já medidos e os refutadores: erro do documento,
   warning do documento + erro da expressão, expressão com contexto válido,
   stdin, import, callback com origem no documento, leitura relativa e ausência
   de segunda avaliação. Conservar todas as mensagens/spans/fontes reais.
5. Implementar e revalidar somente após o gate aplicável. Atualizar o diagnóstico
   sem converter os negativos de origem em positivos por renomeação ou projeção.

Matemática automática, morfologia dos corpos e table.cell continuam em frentes
separadas, registradas no diagnóstico. Não misturar seus gates nesta implementação.

## Pacote L0 pronto para confirmação

Os prompts proprietários foram atualizados; o núcleo expression-source-identity
centraliza invariantes compartilhadas e os owners o pinam. Não foi alterado
código neste pacote. V15/V26 passaram; V5 permanece deliberadamente pendente
até materialização autorizada, quando os headers e Hash do Código serão selados
localmente. Não executar --fix-hashes global na árvore compartilhada.

Novos contratos: SystemWorld::for_stdin, SystemWorld::source_for_expression,
L1 eval_source_with_features_and_introspector e L3
eval_source_with_sink_features/eval_source_with_document_features. As entradas
string permanecem compatíveis; a CLI passa a usar Sources distintas. A exaustão
do contador deixa de sobrescrever fontes. Imports/read conservam a base real.

Manifesto revisável: origem-fontes-expressao-l0-manifest-2026-10-06.json,
com SHA-256 completo de cada L0, do núcleo e seu pin efetivo.
SHA-256 do manifesto: 193a235d0db42ebef4d26056d1afee4ac5687a456e941057fa6cff9118cbabc2

## Materialização concluída

Gate confirmado pelo dono com confirmo, registrado no receipt de aprovação.
Implementação, testes RED→GREEN e medições estão em
00_nucleo/diagnosticos/origem-fontes-expressao-2026-10-06.md.
A próxima diferença deste conjunto é o range do warning de stars, não identidade
de Source. Matemática e table.cell permanecem frentes independentes.
