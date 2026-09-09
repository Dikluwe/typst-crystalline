# P1332 — nome intrínseco de calc.abs

## Evidência e objetivo

O fechamento P1331, em diagnosticos/p1331-closure.json, deixa diferenças
de nome nos traces. As medições novas p1332-baseline.json e
p1332-name-consumers-public.json mostram que repr já usa abs, mas o nome
intrínseco registrado ainda vaza como calc.abs. Também é interpolado em
erros de gradientes e show; mudar somente o trace mascararia a causa.

Corrigir o nome intrínseco da função registrada em calc.abs para abs,
mantendo a chave de lookup, assinatura, implementação e todas as demais
funções. O L0 proprietário é prompts/compiler/stdlib/calc.md, não este passo.
Correção de nome/paridade em fluxo contínuo ADR-0127, sem nova API ou fase.

## Execução autorizada e limites

1. Registrar baseline, binários ratificados, diff/stat e preservação P1331.
2. Atualizar L0 primeiro; congelar manifesto e autorias A/B sem atestação.
3. Autor independente deriva testes e expectativas antes do candidato.
   Migrar apenas nomes de traces históricos afetados, jamais expressões,
   mensagens primárias de abs, spans, severidades ou controles restantes.
4. Integrar os bytes congelados; confirmar RED compilado. Implementar
   somente o registro legitimado e ressellar a linhagem; confirmar GREEN.
5. Rodar build/workspace/fmt/lint/V5/V15/V26 e CLI normal/repetida/reversa;
   revisão independente, relatório substantivo e fechamento em diagnosticos.

Cobrir identidade de linguagem por lookup/import/alias, diferenças para
outras funções, repr, With/nested/arguments, erros primários e origem,
warnings, quatro perfis e os consumidores indiretos medidos. Igualdade ou
hash Rust não são alvo de paridade; mudanças de sua mecânica não autorizam
colisão semântica de funções. Não introduzir wrapper ou regra especial no
dispatcher. Não corrigir named/aridade, parsing, NaN dimensional, show ou
gradientes neste passo. Unknown obrigatório bloqueia fechamento.

Regime A/B: root escreve L0/candidato; autor de testes sem runtime/patch;
revisor somente lê os artefatos julgados. Filesystem compartilhado não
atesta isolamento. Sem mutation score ou selo de refinamento.
Budget de calibração: duas revisões focais sem ganho exigem reabrir o
desenho antes de outra rodada completa. Evidências antigas são imutáveis.
Usar target dedicado /tmp/p1332-target.tTIpWy: /dev/shm tem 3.3G livres,
insuficientes para o cache anterior. Não apagar targets, commitar ou
escrever o passo seguinte nesta rodada.
