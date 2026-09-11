# P1339 — preparação mecânica do lote 1

Estado: componentes prospectivos congelados para revisão, não executados. Regime:
executado sem atestação de isolamento. Este documento não é selo, oráculo ou
veredito; não autoriza candidato nem execução focal antes do manifesto agregado.

O driver `p1339-mutant-closed-state-final-runner.py` exige dois testbins reais,
configuração de compilação declarada e auditoria independente de binding aceita.
O wrapper core inclui os adapters protegidos e exige as asserções dos resultados;
o collector pipeline produz somente transporte. O driver invoca também os dois
predicados Python originais, incluindo o predecessor importado pelo checker v2.
Conjuntos exatos de IDs × quatro perfis × três ordens são obrigatórios:
API 312, relação 852, opaco 12, estilo 12, projeções 96 e lifecycle 144.
Essas cardinalidades são derivadas das fixtures pinadas, não medições de sucesso.

## Ligação futura estritamente mecânica

1. Incluir o core-wrapper em módulo `#[cfg(test)]` filho de `compiler::eval`.
   O binding real e o teste `p1339_frozen_core_bound_matrix` devem ficar **dentro
   desse mesmo módulo**, para acessar os tipos privados dos seus filhos.
   O filtro do driver não executa o teste API herdado separadamente; o wrapper já
   o chama e imprime as células uma vez.
2. `ActualRelation` só traduz o discriminante da operação privada real usada pelo
   validador. Não recalcula relação, não usa fixtures e não duplica o algoritmo.
   Hooks style/projection apenas recolhem eventos reais das operações invocadas;
   nenhuma expectativa ou resposta esperada é argumento do hook.
3. Incluir o lifecycle-collector no módulo de teste de `03_infra::pipeline` e
   ligar `p1339_frozen_pipeline_bound_matrix` ao caminho paginado produtivo real.
   O callback constrói o World imutável autorizado, preserva FallbackFontMetrics
   real e observa passivamente o ciclo. Não implementa um segundo ciclo, fold,
   seleção, retenção, invalidação, replay ou sink.
4. **Fronteira entre crates ainda precisa ser resolvida e auditada:** o core
   compilado como dependência do testbin infra não recebe `cfg(test)`. Logo, hooks
   privados presentes somente no testbin core não são acessíveis por esse binário
   infra. O binding final requer exposição/compilação exclusivamente test-only,
   explicitamente autorizada e identificada na configuração auditada, que torne
   observáveis os eventos reais necessários através da fronteira; não se presume
   que isso já exista. Uma prova separada só pode substituir um vínculo se o
   contrato/oráculo/verificador a autorizarem explicitamente, sem perder o DTO ou
   predicado obrigatório. Até então a ligação é precondição aberta, não PASS.
   Não autoriza nova API produtiva/default, estado global, I/O em L1 ou execução
   extra do corpo para inventar eventos de observação.
5. A auditoria final deve fixar o grafo efetivo, todos os owners/ports, configurações
   por crate, ausência no build normal e fontes reais. Hash distinto não demonstra
   validade semântica/arquitetural nem que o binário incorpora essas fontes.

## Identidade e execução futura

O driver calcula o SHA do arquivo testbin executado e fornece somente metadados
por `P1339_TEST_BINARY_SHA256`, `P1339_COMPILED_CONFIGURATION_JSON`,
`P1339_COMPILED_CONFIGURATION_SHA256` e `P1339_BINDING_AUDIT_JSON`.
O binding de teste pode ler tais metadados; eles não transportam resultados.
O driver confere as identidades nas linhas contra o arquivo realmente chamado,
preserva argv, UTC, exit/sinal, stdout/stderr e base64 e desabilita core dumps.
Ausência, duplicação, erro de processo, sinal e saída opaca não geram sucesso.
`PASS_CLOSED_TESTS_ONLY` não fecha CLI, testemunhas estruturais, workspace ou selo.

## Limites de preparação

Nenhum adapter de APIs futuras foi compilado ou executado. Rustfmt valida apenas
sintaxe; tipagem, lifetimes e integração permanecem não demonstrados. Falhas de
binding devem ser registradas como tais, nunca corrigidas ajustando expectativas.
As declarações e o mapa de cobertura são alvos de auditoria, não testemunhas
aceitas automaticamente. O mapa autoral ancestral pinado no recibo permanece
separado do resultado do driver; eventual sucessor exige pin causal no manifesto.

Os vinte mutantes e seus recibos anteriores permanecem imutáveis. O lote 1 não
reinicia os budgets anteriores: zero execução focal nova e zero matriz completa
pré-selo por este papel durante esta preparação. Custos históricos são os do
registry ancestral, incluindo builds falhos/aposentados e focais, sem crédito de
runtime futuro. Contexto P1336 herdado e acesso procedural compartilhado não
constituem atestação de isolamento.
