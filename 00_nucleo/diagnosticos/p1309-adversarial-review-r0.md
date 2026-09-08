# P1309 D — revisão focal do auditor preliminar R0

Entrada: `p1309-audit.py`, API `audit(bundle)`, fornecida por B antes do bundle
fresco. Nenhuma alteração do auditor por D. Esta revisão por leitura não é
execução das 18 mutações e não fornece mutation score. Hipótese dominante:
campos derivados devem ser recalculados a partir de evidência autenticada,
em vez de comparados apenas com outras declarações do mesmo artefato candidato.

## Fronteiras identificadas

1. **Prioridade de seleção:** `calculated` incorpora `cohort["priority"]`
   fornecida. Trocar prioridade, rank_key e vencedor coerentemente pode ocultar
   ordem normativa. A prioridade precisa derivar de classe, rota/carrier e
   evidência; não pode ser confiada ao próprio campo julgado.
2. **Sentinelas:** o subsistema verifica IDs presentes, sem exigir resultado
   bilateral válido nem vincular seus transcripts à classificação/fechamento.
   Um encoder com função publicada e sentinela de comportamento divergente
   precisa bloquear o fechamento, mesmo que o ID da sentinela exista.
3. **Arrays:** `values == list(range(n))` é um controle útil para arrays range,
   mas não autentica os dados efetivamente retornados. Alterar o transcript
   preservando a lista declarada não pode passar. Arrays de cores precisam
   preservar todos os componentes, não somente arrays sintéticos range.
4. **Transições:** só são verificadas linhas que aparecem em `transitions`.
   A exclusão da própria linha de regressão ou ID novo sem cobertura exaustiva
   pode esconder o evento. Exigir o mapa inteiro ID/perfil e relação com a
   matriz atual e o predecessor autenticado.
5. **Fixtures:** matrix confronta `observation.expression` com `row.expression`,
   mas não com a expressão do probe no catálogo e argv executado. A troca
   coerente de fixture precisa ser rejeitada como identidade incompatível.
6. **Canais apagados:** hash/base64/texto internamente coerentes não provam que
   stderr foi preservado. Apagar o mesmo warning em todas as ordens e recalcular
   hashes deve ser confrontado com evidência raw autenticada separadamente.
7. **Núcleos:** dependência ausente entra em `walk` como folha pelo `.get`
   default; órfãos e consumer repetido sob prompts distintos não são tratados.
   Editar pins e hashes candidatos coerentemente precisa ser comparado com
   anchors da fonte, cuja autenticidade será examinada por E.

## Protocolo D preservado

O harness permite que o controle escolha somente um `scope` focal suportado.
Não permite reparar evidência no controle, modificar anchors, nem mudar scope
entre positivo e negativo. Assim, um bundle global bloqueado não vira controle
válido por adulteração: só um subsistema real que já passe pode fornecer
testemunha focal. Erro de parser, timeout e rejeição genérica não são kills.

O predecessor inicial foi invalidado pelo incidente de bytecode. O replay final
deve pinçar os sucessores R2 e seus recibos, mantendo histórico do incidente.
Não houve mutação de produto nem execução das 37 famílias P1307 neste papel.
