# Prompt — typst-passo-868: consolidar P861 a P867, investigar a contradição P863/P864, e trazer a decisão pendente de P866

**Origem**: os relatórios de P861 a P867 foram executados em paralelo, sem commit, na mesma working tree — mesmo padrão que motivou P828. Além disso, há uma contradição direta: P863 mostra o teste `p863_show_par_func_transforma_paragrafo` passando com saída literal; P864, rodando a suíte completa depois, mostra esse mesmo teste falhando, tratado como "falha pré-existente do P863" sem explicação de como um teste que passou virou falha pré-existente. P866 também pulou uma instrução do prompt original (apresentar a decisão de escopo de PNG/SVG ao dono antes de implementar).
**Estado**: aguardando execução — **prioridade sobre qualquer achado novo**, mesmo motivo do P828: nenhuma contagem ou afirmação dos sete relatórios (P861-P867) é confiável até este passo fechar.

---

## Passo 1 — Investigar a contradição P863/P864 antes de qualquer outra coisa

Isto não é opcional nem pode ser resolvido só rodando a suíte de novo e vendo o que dá. Precisa de explicação real:

1. Confirmar se `cargo check -p typst-core` em 0,14 segundos (tempo reportado por P863) é fisicamente possível para uma recompilação real desse crate com as mudanças de P863 aplicadas, ou se é sinal de que o comando rodou contra um binário em cache que não refletia as mudanças. Rodar um `cargo clean -p typst-core` (ou equivalente) e medir o tempo real de compilação do zero, para ter uma referência do que "compilação real" custa neste projeto.
2. Reconstruir, na ordem certa, o que aconteceu: aplicar só o diff de P863 (isolado, sem P861/862/865/866/867 junto) sobre o commit base, e rodar `cargo test p863_show_par_func_transforma_paragrafo` nesse estado isolado. Se passar isolado e falhar quando P864 é aplicado por cima, a causa é uma interação entre os dois passos (P864 quebrou algo que P863 tinha funcionando) — não uma "falha pré-existente".
3. Se o teste já falhava mesmo isolado (ou seja, P863 nunca de fato passou e o relatório dele estava errado, possivelmente por causa do cache do item 1): isso é um problema de proveniência do relatório de P863, não um bug de código — registrar como tal, separado da investigação técnica.
4. De um jeito ou de outro, o teste `p863_show_par_func_transforma_paragrafo` precisa terminar este passo **passando de verdade**, numa compilação confirmada como real (sem cache suspeito), ou com uma explicação técnica concreta registrada de por que ele não pode passar ainda (não "pré-existente" sem mais detalhe).

## Passo 2 — Consolidar a árvore

1. Confirmar o estado real: as alterações de P861 a P867 existem hoje como uma única working tree (o que os relatórios sugerem) ou como cópias separadas. Não assumir — checar `git status`/`git stash list`/branches, mesmo procedimento do P828.
2. Se for uma única árvore com tudo misturado: isso não é necessariamente errado (pode ter sido intencional, os passos são incrementais uns sobre os outros), mas precisa de uma corrida de validação **limpa e única**, não sete corridas parciais que cada relatório fez sobre um estado ligeiramente diferente da mesma árvore em momentos diferentes.
3. Prestar atenção aos arquivos "hub" que aparecem alterados em múltiplos relatórios com contagens de diff diferentes para o mesmo arquivo (`01_core/src/engine/eval/rules.rs`, `01_core/src/engine/layout/mod.rs`, `01_core/src/engine/eval/tests.rs`, `01_core/src/engine/layout/tests.rs` — todos aparecem em pelo menos três dos sete relatórios) — confirmar que as mudanças de cada passo estão de fato todas presentes e não se sobrescreveram.

## Passo 3 — Validação única e real

1. `cargo build --release` limpo (do zero, sem cache suspeito, dado o que o Passo 1 pode ter revelado sobre confiabilidade de builds em cache neste ambiente).
2. `cargo test --workspace` — uma corrida só, log completo, não só o resumo.
3. Comparar a contagem final contra a soma dos testes novos declarados nos sete relatórios (mesmo método do P828) — se não bater, investigar a diferença.
4. `crystalline-lint .` — confirmar zero violações novas além do V7 já conhecido.

## Passo 4 — Trazer a decisão pendente de P866 (PNG/SVG)

P866 decidiu sozinho rejeitar `-o arquivo.png`/`.svg` com mensagem de erro clara, em vez de implementar rasterização — sem apresentar a decisão de escopo ao dono antes, como o prompt original pedia (mesmo padrão já usado no projeto para SVG-como-imagem e PDF-como-imagem). A implementação em si (recusar com erro claro, não gerar PDF disfarçado) parece razoável e não é do tipo que precisa ser desfeita — mas a formalização da decisão de escopo (dependências pesadas envolvidas: `typst-render`/`typst-svg` do vanilla, ou equivalente) precisa ser trazida explicitamente para o dono confirmar, e só depois formalizada como dívida (mesmo padrão DEBT-66/67/68), não deixada como decisão silenciosa de quem executou.

## Passo 5 — Commit

Só depois dos passos 1 a 4 fecharem limpos: commitar o estado consolidado, mensagem referenciando P861–P868.

## Relatório

`00_nucleo/diagnosticos/typst-passo-868-relatorio.md` com: a explicação concreta da contradição P863/P864 (Passo 1, incluindo se foi problema de cache, de interação entre passos, ou de proveniência do relatório de P863), a lista de conflitos de consolidação encontrados e como foram resolvidos (Passo 2), a saída completa de `cargo test --workspace` com a comparação contra a soma ingênua (Passo 3), e a decisão de escopo de PNG/SVG apresentada explicitamente para o dono decidir (Passo 4) — não implementada nem formalizada neste passo, só trazida à mesa.
