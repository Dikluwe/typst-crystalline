# P1339 — integração da interface de observações aprovada

Estado: `L0_INTEGRATION_PARTIAL; PUBLIC_GATE_APPROVED; NO_IMPLEMENTATION`.

A resposta **“Autorizado”** aprovou os três métodos públicos de EvalContext
e a restrição à construção externa por literal causada pelo armazenamento
privado. O recibo `p1339-observation-interface-approval.json` conserva o
texto da decisão, os hashes submetidos e o gate predecessor. Não falta nova
autorização para essa interface.

## Medição e proveniência

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`; working tree não commitado.
O recibo `p1339-observation-integration-receipt.json` conserva UTC, diff/stat
integral, hashes das fontes auditadas e L0, saídas dos checks e comparação
com o inventário A0. As referências abaixo são do código produtivo intacto,
não de uma implementação candidata. Esta inspeção não é execução funcional.

| Entrada real | Owner / fonte | Lacuna que o registro precisa evitar |
|---|---|---|
| Query / locate | foundations/query.rs:53–81 | Locations iguais não garantem fields/conteúdo iguais; vazio também é resultado. |
| Here | foundations/query.rs:99–105 | Location pertence à tentativa, não ao snapshot candidato. |
| State get/display/at/final | stdlib/state.rs:61–182 | Registrar a entrada antes da callback de display, mantendo seu erro posterior. |
| State global legado | stdlib/state.rs:348–395 | Preservar None próprio da rota; não trocar por init de State. |
| Label de state.at | eval/bindings/value_methods.rs:127–153 | O lookup pode falhar antes do helper de state e deixar dependência invisível. |
| Counter get/at/final/display | stdlib/counter.rs:305–546 | Guardar demanda antes de erro de lookup/fold, prefixo e total usados por both. |
| Counter global legado | stdlib/counter.rs:608–640 | Resultado string não equivale ao array dos métodos modernos. |
| Location page/position/page-numbering | eval/call_dispatch.rs:1007,1541,1725–1755 | Comparar a projeção pedida; numbering cru pode conter Func. |
| Estilos e recursos | entities/engine.rs:43–57; pipeline.rs:244–268 | São inputs externos ao snapshot e exigem preservação explícita da tentativa. |

A busca por `.introspector` e `current_location` em eval/stdlib foi conferida
nas fontes produtivas, separando os testes embutidos em stdlib/mod.rs. Também
foram inspecionados measure/layout e text.size/lang: Engine.styles é mutável,
portanto a chain final do corpo não substitui a chain do instante da leitura.
Esta é uma auditoria de acessos diretos; não prova sozinha completude de
dependências transitivas, serviços World ou comparação de valores opacos.

## Decisão incorporada aos L0

Os owners de leitura agora declaram registro e replay das operações efetivas.
Eval conserva armazenamento privado e dispatch fechado; pipeline conserva
os inputs da tentativa e invalida descendentes quando troca seu produtor.
State/query/location não acionam o bit filtrado: apenas demanda de counter
com Element seleciona o bloco. Isso permite registrar leituras anteriores
à demanda sem reexecutar todos os blocos legados.

Registrar label original, não só sua Location resolvida, cobre ausência que
vira presença. Em display, validar as entradas não reaplica a callback de
numbering/formatação. O replay de CounterUpdate::Func continua parte distinta
da leitura completa demandada e permanece no seu owner de introspecção.

## O que ainda impede materializar

- Completar e revisar o comparador de observações: Func/Content/valores opacos
  de query/state/page-numbering não ficam provados por repr, Debug, igualdade
  da linguagem ou Location. Nenhuma implementação permissiva foi escrita.
- Fixar a correspondência entre snapshots/tentativas e saída final, incluindo
  as precondições externas ao snapshot e diagnósticos de não convergência.
- Concluir os L0 das outras rotas P1339 e a observabilidade dos perfis de
  features; compilar com sucesso não converte os Unknown anteriores em prova.
- Contrato, oráculos, mutantes válidos, selo e RED independentes continuam
  pendentes. A última tentativa de revisão por agente retornou limite de uso;
  não há revisão sucessora produzida nem autorização para dispensar o gate.

A skill `tekt-materializacao-segregada` exige essas autoridades separadas:
o autor desta integração não assume também contrato e veredito para contornar
o limite. Regime: **executado sem atestação de isolamento**. Não foram feitos
rebuild, teste funcional, resselo de headers, commit ou declaração de paridade.
Checks de ownership/núcleos e diff são registrados no recibo; V5 pendente é
documentado, não apresentado como lint global limpo.
