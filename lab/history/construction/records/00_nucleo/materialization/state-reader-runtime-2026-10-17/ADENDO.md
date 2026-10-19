# Precisão interna de A1-S2 — erros posteriores

A revisão aprovada permanece imutável. A sonda state-future-error em
00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/target-controls.json.gz
refuta aplicar somente ações anteriores ao limite: o valor de get/at é do
prefixo, mas o vanilla calcula a sequência completa da key e propaga erro em
callback posterior. Upstream state.rs:419–437 e 477–520 confirmam a origem.
Os L0s do resolvedor/registry foram precisados antes do código, conservando
somente acumulador e resultado escolhido em memória. Não muda assinatura,
contrato público ou fase aprovada; é correção interna de paridade em fluxo
ADR-0127. Não é aprovação reaproveitada para nova API. Sem leitura ou para outra
key, não executar a função. A implementação não copiará o vetor de todos os
estados intermediários do vanilla.
