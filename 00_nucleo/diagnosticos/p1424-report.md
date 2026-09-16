# P1424 R4 — relatório independente

**Veredito: PASS.** Executado sem atestação de isolamento.

O candidato preserva a cadeia R4 e fecha o fragmento contratado de `html.frame`. Os hashes do manifesto (`ba1a9909…`), contrato (`f80e212b…`), oráculos (`e24b3350…`), mutantes (`9432b31d…`), selo (`15c5287b…`) e quatro artefatos de teste coincidem com as entradas congeladas. Os 28 caminhos candidatos estão dentro da allowlist: 14 consumers produtivos e 14 owners L0. Nestes owners, o delta é somente o resselo mecânico de `Hash do Código`, incluindo a primeira inserção no owner novo; `crystalline-lint .` validou a linhagem final.

As suítes seladas passaram nas três ordens com vetor idêntico: 3/3 testes L1 e 4/4 testes L3 em `forward`, `forward_repeat` e execução individual inversa. Os 13 casos positivos são `Preserved`. Os quatro controles opacos continuam `Unknown`, recebem crédito zero e nenhum resultado obrigatório foi promovido a sucesso.

O refinamento R4 discrimina a falha que bloqueou R3. P13/O7 executa `#html.frame([#html.elem("div")[x]])` pela fachada pública HTML e exige `Err` com diagnóstico não vazio. O candidato examina o corpo completo antes do layout intrínseco e devolve explicitamente `html.frame body contains content unsupported by paged layout`; a fixture passou nas três ordens. Assim, M20 não sobrevive como SVG vazio, omissão, bare text ou tag literal. O mesmo O7 chama o exporter sem resolvedor sobre `Content::HtmlFrame` e recebe erro explícito, rejeitando M21.

Resultado adversarial: 21/21 mutantes válidos `Violated`, zero sobreviventes, mutation score `1.0`, igual ao mínimo exigido. Os vetores permaneceram idênticos nas três ordens.

Os gates finais passaram: `cargo fmt --all -- --check`, `cargo build`, `crystalline-lint .` e `git diff --check HEAD`. A regressão do exporter HTML terminou com 16 passes e nenhuma falha. A regressão de pipeline terminou com 60 passes e uma falha em `p1159_location_page_numbering_ve_func_no_relayout`; o teste focal foi recompilado a partir de um `git archive` do baseline causal `79c7a6d60` e falhou com a mesma asserção sobre `plain_text().contains("=>")`. Essa falha fica registrada separadamente como preexistente e não causal, sem contaminar o veredito P1424.

O PASS atesta somente binding/assinatura/carrier, transparência paginada, resolução HTML localizada, SVG inline intrínseco para o subset suportado, geometria/pintura/texto cobertos, nesting, distinção de `html.elem("frame")` e falha fechada registradas no contrato R4. Não atesta paridade HTML/SVG geral, igualdade de bytes/IDs/ordem de atributos nem os itens explicitamente fora de escopo.
