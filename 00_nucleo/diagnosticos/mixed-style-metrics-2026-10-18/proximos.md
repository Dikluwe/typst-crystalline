# Pendências após as métricas de faces

O objetivo integral continua ativo. A aprovação R2-S está registrada em
`sequence-constructor-2026-10-17/approval-r2s.json.gz`; seu relatório já descreve
a execução. Esta etapa é posterior e não substitui esse contrato ou as aprovações
anteriores. Não houve publicação.

1. **Plain + optimized no CSL custom**: conservar os sete casos anteriores
   discriminados no audit-final. Não forçar simple para esconder a seleção distinta.
2. **Containers**: os casos Box/Par em Box, Raw, List e Quote ainda exigem
   análise de dimensão, recorte, espaçamento terminal e quebra, usando os pares
   h0/h10 para separar a causa do recuo. Testes puros não fecham o caminho CLI.
3. **Alias de Content / R3-T1**: a realização textual foi aprovada pelo humano
   nesta sessão, com recibo em
   `00_nucleo/materialization/textual-realization-2026-10-18/approval-receipt.json`.
   A próxima materialização segue a proposta
   `00_nucleo/materialization/textual-realization-2026-10-18/review.md`, manifesto
   `47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302`.
   R3-T1 autoriza essa outra mudança de ordem. Preservar o overlay de
   bibliography-indent e integrar os 12 L0s sobre seus owners atuais, sem
   sobrescrever as cláusulas adicionadas após a proposta. A implementação e
   validação dessa frente continuam pendentes, agora autorizadas.
4. **Admissão regional fora do certificado**: esta etapa ampliou faces com o
   mesmo size/leading e offset zero. Misturas de tamanho/leading em páginas
   finitas, colunas, células, notas e floats precisam de medição própria. Os
   controles existentes continuam obrigatórios; não inferir cobertura universal.
5. **Fontes e PDF**: o audit-final separa bbox PDF de posições de glyphs no SVG.
   Quando SVG coincide e apenas bbox diverge, investigar metadados, precisão e
   extração antes de chamar o efeito de defeito da língua. Os casos Liberation
   com deslocamento também em SVG requerem investigação geométrica própria.
6. **Checkpoint com prefixo em current_line**: segue a suspeita documentada em
   bibliography-hanging/proximos.md. A tentativa inicial perdeu DDDD e conservou
   FFFF após flush/restore; o teste aprovado cobre fronteira já drenada.
7. **Decorações completas**: as origens LTR e ausência de traço fantasma são
   controles deste lote; Y, espessura, cauda e RTL completo continuam abertos.
8. **Tekt**: os erros V16 herdados continuam bloqueando o gate global. Notas
   V19/V20 e seu delta permanecem visíveis em lint-delta.json; não atualizar pin
   ou introduzir supressão para declarar encerramento.
9. **Demais frentes**: conservar RM1/RG/FG/Paint/Symbol e E01–E20 do inventário
   em paridade-novo-vanilla-2026-10-17 e do escopo-geral de sequence-r3.
10. **RAM do produto versus vanilla**: comparar execuções pareadas com os mesmos
    documentos, fontes, cache e limites antes de afirmar aproximação. A remoção
    do vetor temporário de perfis é verificável no código; o pico de cgroup
    desta etapa inclui compilação e não substitui essa medição do produto.

Os números finais, arquivos exatos e limitações estão em relatorio.md,
audit-final.json.gz e final-state.json.gz. Cada contagem é de casos medidos,
com controles repetidos entre matrizes; não representa quantidade de bugs
distintos nem percentual de paridade da linguagem.
