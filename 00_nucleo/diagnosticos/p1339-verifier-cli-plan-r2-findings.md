# P1339 — achados focais antes do freeze CLI

Entrada examinada: `p1339-ab-cli-plan-r2.json`, ainda plano não selado,
SHA-256 `bb1a716d7071f7a8af6379d0a647b05fb17298e8322267e74d90d6c1e80cf5bf`.
Auditoria concluída em 2026-09-10 03:36:01 UTC sobre o HEAD/working tree
pinados no intake e no focal independente de mutantes.
Verificador `/root/p1311_review`, executado sem atestação de isolamento.
Contrato vigente r3 `c0cd1826679ddaf77e937a677839c5cbe64fdae6ec71bfa34e635d584d9c3e17`.

Mapeamento dos 539 IDs históricos conferido: nenhum ausente, excedente ou
duplicado. Os dezoito IDs de show são mapeados para compile-witness vanilla
sucessor conforme contrato; demais alvos históricos correspondem. Controles
do host mutante vanilla devem ser comparados ao vanilla, não ao alvo legado
do candidato cristalino quando divergem.

Dois achados impedem congelar o plano tal como entregue:

1. `show-strong-direct-control`, `show-emph-direct-control` e
   `show-text-direct-control` estão com `reference_policy=vanilla`.
   `preserved_exceptions.direct_show_controls` exige baseline para esses
   controles; a dívida bare-text é explícita. O candidato não pode ser
   obrigado a corrigir a rota direta alheia para satisfazer o teste novo.
2. `where-l0-empty-vs-bare` contém
   `repr((strong.where()==selector(strong),heading.where()==selector(heading)))`
   e está com alvo vanilla. R3 mantém `selector(strong)` nu indisponível.
   A construção where autorizada deve avançar até o erro nu protegido;
   não se pode exigir a tupla vanilla nem congelar automaticamente o primeiro
   erro baseline, que pode ainda ocorrer na construção where ausente. Exige
   predicado causal e controles separados, além de testemunha válida da
   distinção grupo vazio/bare no domínio admitido, sem promover bare Strong.

Busca dirigida nas expressões/fontes do plano com alvo vanilla encontrou
esse único programa com chamada textual bare selector/counter/query de
Strong/Emph. A busca é triagem, não prova de exaustividade semântica.

Os achados foram enviados ao operador para o autor independente corrigir
seus próprios artefatos antes do freeze; o verificador não altera oráculos,
expected ou contrato. Nenhuma matriz completa foi iniciada por esta auditoria.
