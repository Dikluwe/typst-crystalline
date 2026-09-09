# P1326 — auditoria do candidato

Predecessor obrigatório `p1326-review-freeze-red.md`: C iniciou somente após
freeze independente e RED R1 validado. Manifesto
`800db40f2300d0fda0275d06365c5f3019ca0848577ee54a9c834975b231146f`.

Fonte candidata SHA-256
`4c0045831976a08e414820c831be0b0c957d522f121d3f9a59ee92e05d67d449`;
hash sem linha @prompt-hash confirmado independentemente
`3a6ed279c9adee74880901578bd16f06b9b5d4ca7c2e201e9bd7a4adb7d67647`.
Testes R1 e sucessor legado estão integrados em uma única cópia literal cada.

Reconstrução independente: retirar snippet R1, substituir sucessor legado
pelo extrato original e retirar somente helper privado, cláusula de seleção
de span e cláusula da mensagem Closure restaura o owner baseline byte a byte
após excluir header de hash e whitespace periférico. Não há delta funcional
adicional escondido entre as intervenções.

O helper percorre With iterativamente e discrimina FuncRepr exaustivamente;
somente Closure retorna true. Não depende de nome, namespace vazio, pointer
ou execução da função. Campos presentes e nomes de nativas continuam nos
mesmos ramos; Plugin/Element mantêm falso e fallback vigente. Span altera
somente no ponto de delegação já contratado. Gates prévios, text contextual,
warnings, ordem de avaliação, Args e demais variants não mudaram.

Não há API pública, entidade, carrier, default, feature ou fase nova; a
classificação ADR-0127 contínua permanece correta. Nenhum achado funcional
no diff examinado. Gates de execução finais ainda pendentes nesta auditoria;
não inferir GREEN pelo tamanho do patch. A/B sem atestação de isolamento.
