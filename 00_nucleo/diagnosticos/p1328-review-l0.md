# P1328 — revisão do amendment L0 anterior a C

Revisor `/root/p1328_review`, `2026-09-09T11:38:13Z`. Regime A/B executado
sem atestação técnica de isolamento. Somente leitura de produto, L0 e
medição; escrita deste diagnóstico. Nenhum candidato C inspecionado.

Entradas verificadas com SHA-256:

- `p1328-baseline.json`:
  `e049418db46ea039235b336254bfdbffd492253ac66c38782ac6bc56564f2ffb`.
- `00_nucleo/prompts/compiler/stdlib/calc.md`:
  `2c7494c628038975cd52d6e093f66dc7e78a435d69effdc17ee6eb312213367e`.

O baseline fixa HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working
tree não commitado, diff/stat integral em `state`, inventário produtivo,
argv e UTC por observação. O binário cristalino é identificado por
`75e8b97b3788c0feaf457cb4c06b1c2c6735cff5ef3b9804a75ec57f8b148e31`,
o vanilla por `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
As testemunhas frescas qualificadas, bare, markup, alias, With e spread
confirmam mensagem antiga/detached contra texto vanilla e origem resolvida.
O mesmo registro distingue diferenças preservadas de named/aridade,
String/Symbol, Length/Angle, integer mínimo e sqrt.

`entities/value.rs:348` publica Content e LocatedContent como `content`;
`:403` também mapeia ambas para `Type::Content`. Symbol publica `symbol`
em `:367` e `Type::Symbol` em `:421`. Portanto incluir LocatedContent é
cobrir a classe da linguagem já selecionada; limitar-se à variante Rust
Content, como sugerido preliminarmente, seria um recorte mecânico incompleto.
Não é heurística de identidade, comparação de payload, nem expansão da API.
O local armazenado no wrapper não se torna origem do argumento: continua
valendo exclusivamente a ocorrência causal de Args.

O amendment mede antes de decidir, fixa texto/severidade/hints e origem,
preserva guards/valores válidos/dívidas, mantém math no pipeline atual e
nomeia refutadores. `call_dispatch.rs:1031–1044` já suprime trace contido;
portanto a redução do trace redundante observada em código decorre da
correção da âncora, sem exigir mudança nesse owner.

O preflight registrado em `p1328-l0-preflight.json` reporta somente V5
esperado de calc; `p1328-l0-dry-run.json` propõe somente o par calc com
hash-a `5d8d8333` e hash-b `4f58eb82`. Exit zero do preflight não é
afirmado como ausência de drift: V5 está explicitamente pendente de resselo.

Veredito: amendment coerente para seguir ao freeze A/B e RED, sem novo gate
público ADR-0127. Isso não aprova implementação nem fechamento. A revisão
do conjunto congelado e da falha RED continua pendente; LocatedContent
precisa constar nos testes de classe/âncora, inclusive quando o wrapper
possui localização distinta da origem do argumento.
