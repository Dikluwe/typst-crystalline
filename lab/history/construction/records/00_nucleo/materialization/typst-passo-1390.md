# Passo 1390 — wrap único entre wrappers de tamanhos desiguais

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Completar a célula imediatamente adjacente a P1389: dois wrappers
`Styled(Text)` size-only, ambos acima do tamanho exterior e desiguais, cada
body com uma palavra, com exatamente um wrap na fronteira entre fragmentos.

## Medição anterior à decisão

Medição bilateral em 2026-09-14T11:48:21-03:00, HEAD
`906e2ed5a744779ab1a109d123f965f827500240`, working tree com apenas o
diretório novo `00_nucleo/diagnosticos/p1390-fixtures/` ainda não rastreado.
Binários: vanila `eb60986b...`; cristalino `221784f...`.

Nos pares 21→24, 24→21, 22→30 e 30→22, ambos quebram antes de `More`. O
vanila conserva uma página e `After=Tail=220,166`; o cristalino produz duas
páginas. As posições verticais vanila de `More` são, respectivamente,
25,304; 27,986; 24,546 e 31,698pt. No cristalino são 31,804; 38,410;
31,696 e 49,312pt. A medição foi serial sob 2 GiB/45 s.

## Decisão

1. Atualizar primeiro os L0 proprietários de Sequence e dos testes.
2. Certificar somente dois wrappers size-only desiguais, ambos maiores que o
   exterior, uma palavra por body e um único wrap entre eles.
3. Reutilizar o carrier e o fechamento exterior privado de P1389.
4. Reservar separadamente ascent, descent e leading exterior da primeira e
   da segunda linha; não usar duas vezes o máximo global.
5. Preservar `Unknown` para tamanho igual (dono P1389), tamanho ≤ exterior,
   multiword, delta adicional, terceiro fragmento, wrapper aninhado, wrap
   interno/múltiplo e reserva vertical insuficiente.

Correção interna de paridade em fluxo contínuo pelo ADR-0127. Não há mudança
de contrato público, comportamento por defeito ou fase do pipeline.

## RED → GREEN, ataques e memória

O RED exige uma página e `After=Tail` nas duas ordens 21→24 e 24→21. Os
ataques retiram a nova admissão desigual, reutilizam o máximo global nas duas
linhas, omitem uma linha da reserva, concedem tamanho exterior/multiword ou
regredem os controles P1389/P1388.

Execução serial: teste focal e link sob 4 GiB, debug desligado e ficheiro
máximo 512 MiB; matriz sob 2 GiB/45 s; check/lint sob 3 GiB. Duas revisões sem
ganho na mesma causa reabrem o desenho.

