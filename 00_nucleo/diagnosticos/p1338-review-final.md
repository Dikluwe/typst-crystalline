# P1338 — parecer independente final

**PASS_SCOPED**, sem violations ou Unknown obrigatório. Auditor read-only `p1338-review-final-audit.cjs` executado sobre a working tree não commitada do HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`; saída integral, UTC e 85 inputs pinados em `p1338-review-final.json`. O relatório e o algoritmo de fechamento foram lidos integralmente e estão entre os pins.

O recorte é correção interna ADR0127 de mensagem e origem do erro Array ausente que chega ao lookup. A reconstrução independente confere os testes primeiro e somente a sucessão autorizada da sentinela antiga; C acrescenta exclusivamente Array à seleção AST e troca a mensagem do erro ausente. Todos os demais bytes de testes ficam intactos. O L0 normativo R1 e os freezes permanecem idênticos.

Evidência confirmada: RED 4 pass/3 assertion failures; GREEN release 7/7; workspace release 6749 pass/0 fail/3 ignored; CLI 540/540 envelopes integrais estáveis nos quatro perfis e três ordens (120 convergências, 300 preservações de paridade, 120 preservações de dívida). Controle instrumental passou 6/6; quatro mutantes recompilados falham por suas testemunhas específicas. Fontes, patches, canais e cinco executáveis distintos foram rehashados; timestamps estritamente frescos, caminho de Compiling e conclusão release conferidos. Não foi necessário replay adicional.

Linhagem calculada independentemente: A `7e89a8cef2687b94212b3c9549807a2993e43f9f76755cb1b49e61504c37114d`, B `a4267e575abdc7774fef97b7650e4371c19eca2c1f3926dc19a2ff03457f233e`. O falso-negativo externo de reparo B permanece documentado; a verificação recíproca compensa este par, não corrige o linter externo. Gates V5/V15/V26, fmt, diff, build e suíte têm exit zero. Multiset completo do lint idêntico ao P1337 após retirar apenas coordenadas: 240 warnings/1148 infos/0 errors, nenhuma adição ou remoção.

Preservados 3910 arquivos produtivos fora do par proprietário, 5203 históricos e 499 temporários antecedentes. Inventário total de 3912 paths sem inclusão/remoção; HEAD e staged inalterados. Sem leitura/listagem de materialization/context, sem stage/commit/push e sem modificar inputs julgados.

Limites mantidos: regime A/B sem atestação técnica de isolamento ou selo de refinamento, com contexto anterior de revisão retido. A skill de materialização segregada orientou o gate pré-C, a sucessão documental explícita e a verificação independente dos discriminadores; não houve autoria de testes/produto pelo revisor. Perfil adversarial typst-core opt-level=0 é discriminação instrumental pareada, não equivalência a release normal. Dívidas len/first/last, pré-despacho, Type::Array, Length, ordem e Content.text não foram quitadas. Não se alega paridade geral Array, fields, introspecção, PDF, acessibilidade ou warnings.

Fechamento autorizado somente mediante algoritmo pinado e conferência posterior read-only do recibo. Este parecer não afirma que closure já foi executada.
