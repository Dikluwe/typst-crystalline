# Correção da identidade dos eixos na chave de shaping

Corrigido o defeito demonstrado em `reuse-key-followup`: a chave agora representa
cada coordenada de eixo pelos bits exatos do f32, em hexadecimal de largura fixa,
em vez de arredondar para quatro casas decimais. A aplicação das variações ao
shaper continua recebendo o mesmo valor. Valores idênticos conservam a mesma chave;
valores com bits diferentes não são aproximados para fins de reutilização.

O L0 `00_nucleo/prompts/infra/shaper/cache.md` foi atualizado antes da implementação.
A mudança é uma correção interna de identidade sob ADR-0127: não altera API pública,
modo padrão, compatibilidade ou fase do pipeline. Owner e relação 1:1 permanecem.
O hash do prompt e o Hash do Código foram atualizados somente neste par.

## Validação

Antes da correção, os três testes novos falharam pela colisão esperada (RED).
Após a correção, os três passaram (GREEN). Os testes com fontes reais Ubuntu Sans
e Cantarell exigem que AB e BA coincidam com os resultados isolados correspondentes;
verificam também AA. As fixtures precisam produzir avanços frescos diferentes,
para impedir que uma mudança futura as torne controles não discriminantes.
A suíte de shaping terminou com 49 testes passando e nenhum falhando, inclusive
as três regressões, e foi repetida após a finalização da linhagem.
`cargo build --offline` passou. `crystalline-lint .` não apresentou erros ou
violações do consumer alterado; conserva avisos globais V5/V12/V16/V17/V18.
Não foi realizado resselo global nem formatação global.

Comandos e logs completos em `shaping-axis-key-fix/`. Os tempos exibidos pelo
harness de testes não são medições de desempenho. Nenhum ganho de velocidade é
atribuído a esta correção. Tracking e outras dimensões da chave mantêm o contrato
anterior; esta tarefa não estabelece sua suficiência ou segurança geral.

## Proveniência e preservação

HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitada.
`provenance.json`, `status-before.txt` e `incoming.patch` registram o estado de
entrada. Ambos os arquivos corrigidos já existiam como mudanças não commitadas;
as cópias `before/` e `change.patch` distinguem esta correção do trabalho anterior.
A patch tracked permaneceu idêntica. O fingerprint final registra exatamente
esses dois consumers/inputs alterados em relação ao estudo anterior.
Todos os manifests e relatórios selados anteriormente auditados continuam íntegros.
Seus fingerprints antigos descrevem o estado anterior ao reparo, portanto não
devem ser apresentados como medições do código corrigido.

Arquivos funcionais alterados: somente o L0 de cache e
`03_infra/src/shaper/cache.rs`, incluindo seu módulo de testes. Evidência nova:
este relatório e `shaping-axis-key-fix/`. Fontes, benchmarks, outros L0s e APIs
foram preservados. Os arquivos de snapshot usam extensão .txt para não serem
interpretados como novos consumers pelo linter.
