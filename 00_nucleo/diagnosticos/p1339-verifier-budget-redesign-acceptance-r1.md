# P1339 — aceitação independente do redesenho de orçamento

Verificador `/root/p1311_review`; 2026-09-10, após decisão da coordenação
03:48:26.642909 UTC. Executado sem atestação de isolamento. Nenhuma execução
semântica, alteração de entrada julgada, selo ou candidatura nesta auditoria.

**Aceito o suplemento tático** `p1339-budget-redesign-r1.json`, SHA-256
`4bdd984d7b9d482d68ebb787e98eeed8333a1d4447c637615d4ceb9c78ed6c19`,
sob suas condições integrais. A pausa de orçamento pode ser levantada para
preparar o lote agregado 1; sua execução focal exige primeiro o manifesto
do lote com os inputs, hashes e correções causais previstos. Isto não
autoriza matriz completa, selo, RED canônico ou implementação.

## Evidência auditada antes da decisão

- Todos os 12 pins de entrada e 62 hashes de arquivos modificados do
  suplemento conferem com a árvore atual, HEAD
  `2f42d64253547734564513a1159ee6b584c1c4b4`.
- Os 31 consumers produtivos modificados diferem de HEAD apenas na linha
  `@prompt-hash`; nenhum corpo produtivo mudou. Nenhum arquivo untracked
  existe nas quatro camadas ou em `tests` nesta checagem.
- Ledger do oráculo `p1339-ab-publication-budget-ledger-r1.json`, SHA-256
  `a21abfaa72ca832b7a3c35650ae1f4e81118184608530081bec2da41e2300823`:
  conferidos todos os 185 hashes do inventário e os sete recibos brutos.
  Recomputados 1472 processos, 36 raw Unknown e soma de duração dos lotes
  61.664848616987 segundos. São custos/observações anteriores com proveniência
  nos recibos, não novos passes nem equivalência.
- Os sete recortes são transporte r1/r2a, ponte metadata, derivados W02,
  suplementos CLI e opacidade r1/r2. Nenhum cobre a matriz discriminatória
  integral com repetição/reordenação. As falhas técnicas e políticas estão
  expressas, sem recodificar os resultados.
- Histórico do adversário permanece no registry SHA-256
  `04d5578d720b4c7fff47954c29d20392b7a221f8b0cb80094a3859409add3677`:
  compilações falhas, branches signum não exercidos, M12 cached inválido e
  seu sucessor constam como tentativas, não crédito. Não se infere duração
  de build ausente. O focal próprio do verificador e a falha E2BIG anterior
  continuam registrados separadamente, sem apagar o custo sem recibo.

O total do ledger `known_technical_aborts_without_product_execution=2`
refere-se aos dois abortos de preparação de produto (transporte/generador).
Há também um aborto adicional de publicação do próprio ledger, expressamente
registrado com 0 processos e 0.158091267 segundos. Portanto esse campo não
será citado como total de todos os incidentes técnicos: são três incidentes
listados, sem converter esta ressalva em uma nova execução/revisão semântica.

## Limite da aceitação

A autorização do usuário até o fim de P1339 e o caráter inicial do budget,
com redesenho expressamente previsto, cobrem este ajuste de coordenação.
Não é autorização nova para mudar intenção, contrato r3, L0 ou scope-outs.
O suplemento preserva o esgotamento anterior e permite no máximo dois lotes
focais **agregados**, não dois por papel/arquivo. O segundo é condicionado a
hipótese causal nova dentro da lista fechada e revisão independente.

Permanecem zero de duas matrizes completas preseal usadas, nenhum reset,
20 mutantes válidos, M12 estrutural real, M20 raw Unknown/mandatory_unknown,
controles opacos reais, todas as cláusulas F e a separação NotDue sem crédito.
Nova insuficiência fora da lista, mudança semântica ou ausência de ganho
aciona a parada prevista. Este aceite e o suplemento devem ser pinados no
selo eventual, junto ao ledger e manifestos dos lotes. Nenhuma pendência
de cobertura fica satisfeita somente por esta decisão de orçamento.

## Proposta prospectiva de executor do observador

É admissível registrar separadamente, **antes do selo**, a autoridade de
`/root/p1312_review` como implementador de
`01_core/src/compiler/eval/mod.rs` e recibos `implementation-observer-*`.
Isso não muda o owner L0 1:1 nem cria módulo. Condições do futuro suplemento:

- Registrar contexto herdado P1312 como não relacionado, allowlists de
  leitura/escrita e identidade do executor; não alegar isolamento atestado.
- Ativar escrita produtiva somente após selo independente válido e RED real.
- Ler a mesma intenção r3/L0 selados; não alterar contrato, oráculos,
  mutantes, baseline, selo ou veredito; não assumir papel de verificador da
  própria implementação. Testes locais do implementador não substituem F.
- Root não editar esse arquivo concomitantemente; coordenar interfaces
  privadas com os demais owners sem ampliar as três APIs aprovadas.
- Pin do suplemento e aceite independente antes da ativação. A proposta
  conversacional não é, por si, concessão de capacidade.

Não há objeção arquitetural à preparação dessa autoridade restrita. O
documento canônico concreto ainda precisa ser auditado e pinado.
