# P1423 — relatório independente

**Veredito: PASS.** Executado sem atestação de isolamento.

O candidato materializa exatamente os 43 constructors residuais: 10 `void`, 2 `raw` e 31 `content`. O módulo totaliza 112 tags tipadas e `html.frame` permanece ausente, reservado ao P1424. Os quatro testes A/B de L1 e o teste focal de raw text em L3 passaram; `cargo build`, `cargo fmt --all -- --check`, `crystalline-lint .` e `git diff --check` encerraram com código 0.

Os quatro oráculos DOM selados (`img`, `script`, `blockquote`, `area`) coincidiram byte a byte com os hashes vanilla em ordem direta, repetição direta e reversa. As sondas adicionais confirmaram ordem e duplicatas em `headers`, duração zero, realização void de `source`, identidade da tag `track` e atributo global em `caption`. Nove negativos executáveis produziram erro conclusivo, diagnóstico esperado e nenhum artefato.

A auditoria dos 26 ataques classificou 26/26 como `Violated` nas três ordens, score 1.0. Os cinco controles positivos ficaram `Preserved`; os seis controles opacos permanecem `Unknown` com crédito zero. `Unknown` nunca foi contado como sucesso.

A cadeia válida é R3. Manifesto, contrato, oráculos e mutantes conservam os hashes selados. O L0 produtivo mudou em relação ao baseline causal apenas no campo mecânico `Hash do Código`; o teste A/B mudou no corpo versionado apenas pelo envelope de seis linhas, e ganhou owner L0 exclusivo conforme ADR-0129. O L0 do exporter conserva o hash R1. Os hashes finais do candidato e dos recibos constam em `p1423-results.json` e `p1423-verification.json`.

Limite: o veredito cobre somente o escopo P1423. Não atesta `html.frame`, outras rotas ou paridade HTML geral.
