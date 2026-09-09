# P1337 — resposta mecânica R2 pré-C

R2 aplicado somente após liberação operacional do RED-r1. `cargo fmt --all --
--check` retornou exit 0 antes do freeze. O teste e todos os inputs R0/R1,
casos/oráculos/runner CLI, norma e produto foram preservados.

Prova em `p1337-tests-format-proof.json` e script reproduzível homônimo `.py`.
Ressalva explícita: rustfmt acrescentou duas vírgulas finais opcionais em tuplas
já binárias; portanto não se afirma igualdade estrita de todos os tokens ou
mudança exclusivamente whitespace. Nenhuma aridade, literal, assertion, ordem
ou outro token mudou. A sequência literal integral coincide, os tokens restantes
coincidem e as saídas canônicas rustfmt de R1 e R2 são idênticas. Substituir o
módulo R2 pelo R1 recompõe exatamente o hash de toda a fonte pré-C anterior.

Artefatos efetivos R2:

- freeze `p1337-tests-freeze-r2.json`, SHA-256
  `2629ad4e34aa94048370662196b24f7e1a85f1afc7c9f8d67304931a87d46294`;
- módulo `p1337-tests-module-r2.rs.txt`, SHA-256
  `8045f10b9fea8ac26ea10313a0680cf29510adc1607b568afa737d29ace77c1f`;
- patch `p1337-tests-local-patch-r2.json`, SHA-256
  `df951620924bdd85621977726e12e23a421e7bd5c1e0a97dafd3316a17d264a6`;
- prova `p1337-tests-format-proof.json`, SHA-256
  `63cce081c8bba78a036c83e31276a187221a4180776ce3c90ff9bc6a93b14bac`.

Root comunicou aceite mecânico do revisor em
`p1337-review-fmt-transition.md`: RED-r1 é transportado, sem terceiro RED
redundante. GREEN após C ainda deve executar os bytes efetivos R2, junto aos
demais gates. O vínculo adversarial deve referenciar este freeze/módulo.
