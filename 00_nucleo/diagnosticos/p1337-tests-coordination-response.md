# P1337 — resposta independente à coordenação privada pré-C

Solicitação atendida após liberação do root e conclusão do RED original, antes
de qualquer C. Derivação: L0 preserva Array; o recibo baseline CLI original em
array-debt/default/normal registra `array does not contain field "missing"`
e âncora total `(1, 2).missing`, enquanto vanilla mostra o nome genérico array
e apenas o field. A sentinela nova expressa essa dívida, sem crédito de paridade.

Adicionado somente o teste
`p1337_tests::p1337_preserve_array_ast_message_and_total_span_debt`, que usa
o helper AST privado já congelado e compara mensagem, span total, cardinalidade,
severidade, hints, traces e ausência de warnings nos quatro perfis.
O filtro adversarial `p1337_tests` inclui esta função.

Os três testes originais P1337, todas as expectativas antecedentes, migração
original de três tuplas, oráculos/casos/runner/expectativas CLI e L0 não mudaram.
A restauração mecânica do módulo sucessor e da migração original reconstrói
integralmente a fonte baseline do manifesto R1. Os artefatos antigos continuam
byte-idênticos. Nenhuma fonte candidata foi lida; ela ainda não existe.

Sucessores imutáveis:

- `p1337-tests-module-r1.rs.txt`: SHA-256
  `66c306fbfb937b0b1ce816c2297cc40c35243a9a2acabbcf2bd0a5594e966982`.
- `p1337-tests-local-patch-r1.json`: SHA-256
  `ebc55b085053a168213f4fb13d3043f20f2276f483688cdff1ca672f4fe1bd4f`.
- `p1337-tests-freeze-r1.json`: SHA-256
  `6344fe48865ea4318fdca7fe7a3d73963a3a1c9db8b6e47714d7ca2238564124`.

O freeze original permanece a entrada operacional do runner CLI imutável,
pois nenhum de seus casos ou hashes mudou. O freeze R1 adiciona os hashes dos
sucessores Rust e vincula o antecedente, sem repetir uma matriz CLI inalterada.
Novo RED local pendente da execução coordenada pelo root; não executei Cargo.
O novo RED deve mostrar a sentinela Array passando no baseline e manter as
falhas de assertions da obrigação P1337. O RED histórico não é sobrescrito.

Limite mantido: A/B sem atestação técnica de isolamento ou selo de refinamento.
