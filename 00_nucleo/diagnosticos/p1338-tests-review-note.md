# P1338 — entrega privada pré-C para revisão

Freeze final concluído em `2026-09-09T20:16:24.487855+00:00`, contra manifesto
R1 `14a36a87a116dee46464c32d358f3019702c201ea74809c90cc8bf81cdeeb843`.
Nenhum C existia e nenhum Cargo funcional foi executado por este autor.

- Freeze `p1338-tests-freeze.json`:
  `bd9f6a580251ba5dce783770657165354ab427736ceb4d2a4b87506958be04e3`.
- Runner `p1338-tests-runner.py`:
  `f7951d62700e143ff59e41349f9fef99b9ab8012464bb7bb2e330866d06a2fc2`.
- Módulo `p1338-tests-module.rs.txt`:
  `2f19227ce32aab7066bca98f5200559c0e17d6e631593e37f054bc618fbfd8c4`.
- Patch/migração `p1338-tests-local-patch.json`:
  `82f63591f9e66b24fb3611bb895607bf59798e6f2cb0ec9b479ac9b0083f98c8`.
- Fonte após somente testes:
  `802bde00a7570a041385a25ae74249cba4225180b8ae865bcfbb2629bf2336e2`.

Matriz baseline: 45 casos × quatro perfis × dois produtos × três ordens =
1080 execuções com canais integrais. As 180 relações caso/perfil foram
classificadas antes de C: 40 convergências obrigatórias, 100 preservações de
paridade e 40 preservações de dívida. Zero Unknown obrigatório; normal/repeat/
reverse estáveis. As dez dívidas são len/first/last como valor, len vazio,
first/last vazios no pré-despacho, panic no argumento, Type Array, Length e
Content.text. Preservação dessas dívidas não conta como convergência.

Todos os seis testes novos estão sob p1338_tests: erro puro completo, lookup
puro len/first/last incluindo vazios, AST ausente, pré-despacho vazio com erro
detached, Length/Type e categorias previamente corrigidas, e métodos/chamadas
reais. A migração autorizada fica no módulo antecedente e é coberta pelo filtro
mais amplo p1338_. O adversário deve confirmar seus discriminadores contra o
módulo efetivamente selecionado. Nenhum teste antigo adicional mudou.

Formatação passou antes do freeze. O comentário histórico da sentinela foi
preservado literalmente; somente nome/âncora/mensagem mudaram, com dobragem
mecânica da chamada pelo rustfmt, sem trocar comparadores ou caso.
O script p1338-tests-prefreeze.py prova a integração exata sobre baseline R1.

Limites e regime em p1338-tests-scope.md. A sequência de integração provisória,
retirada antes de R1 e reintegração está registrada em
p1338-tests-operational-sequence.md. Não se alega isolamento técnico nem selo
de refinamento; contexto de autoria P1337 foi retido explicitamente.
