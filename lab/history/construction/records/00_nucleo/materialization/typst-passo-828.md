# Prompt — typst-passo-828: consolidar a working tree de P813 a P827 e rodar a suíte completa uma vez só

**Origem**: auditoria dos 15 relatórios de P813 a P827 — cada um foi executado por um subagente (Kimi Code) partindo de um snapshot diferente e parcial da working tree, não de uma árvore que crescia sequencialmente. Não existe hoje um estado único que contenha todos os 15 diffs ao mesmo tempo, validado de uma vez.
**Sintoma concreto que motivou este passo**: P824 (adenda) corrigiu `integration_tests::read_binario_pipeline` (`typst-infra`: 659→661 passed, 1→0 failed). P827, rodando sobre uma árvore diferente que não continha essa correção, mediu `typst-infra` em 658→659 passed com o mesmo teste ainda falhando (1 failed) — e registrou isso corretamente como "pré-existente e alheio a este passo". As duas medições estão certas para a árvore que cada uma viu; nenhuma reflete o estado combinado dos 15 passos.
**Estado**: aguardando execução — **prioridade sobre qualquer novo achado**, porque nenhuma contagem de teste subsequente é confiável até isto estar feito.

---

## Por que isto importa mais do que parece

Cada relatório de P813 a P827 prova corretamente que a correção dele funciona **na árvore onde ele rodou**. Isso não é o mesmo que provar que os 15 juntos funcionam. Dois riscos concretos:

1. **Conflito de merge silencioso**: se dois passos tocaram o mesmo arquivo (ex.: `01_core/src/engine/eval/math.rs` foi tocado por P799, P809, P812, P820, P825 — todos mexendo em `eval_math_callee`/resolução de identificador em modo math), um merge ingênuo pode perder a alteração de um dos dois, ou os dois podem ter feito suposições incompatíveis sobre o estado do arquivo antes deles.
2. **Contagem de teste inflada ou por engano**: sem saber quantos testes cada árvore parcial já tinha antes de cada passo, não dá para simplesmente somar os "+N testes novos" de cada relatório e confiar no total — alguns desses N podem já ter sido contados por outro passo que tocou o mesmo arquivo, ou podem se perder no merge.

---

## Passo 1 — Levantar o estado real

1. Confirmar com quem está rodando os subagentes: os 15 diffs de P813 a P827 existem hoje como 15 branches/working-copies separadas, ou já foram todos aplicados/commitados em cima uns dos outros em algum momento? Não assumir — perguntar antes de agir, dado o padrão já confirmado de sessões paralelas neste projeto.
2. Se existirem como cópias separadas: listar o `git diff <commit-base-comum> --stat` de cada uma das 15, identificar os arquivos tocados por mais de um passo. A lista já conhecida, só pelos relatórios lidos:
   - `01_core/src/engine/eval/math.rs` — P799, P809 (indiretamente via layout), P812, P820, P825.
   - `01_core/src/engine/stdlib/loading.rs` — P823, P824.
   - `03_infra/src/world.rs` — P808 (medição, sem código), P827; conferir se P824 também tocou (a adenda menciona só `03_infra/src/integration_tests.rs`).
   - `01_core/src/engine/eval/tests.rs` — praticamente todos os passos que tocam `eval` (P814, P815, P816, P817, P818, P820, P821, P825) — arquivo de alto risco de conflito por ser um único arquivo de testes crescendo em paralelo.
   - Outros arquivos "hub" mencionados repetidamente: `01_core/src/engine/stdlib/mod.rs`, `00_nucleo/prompts/engine/eval.md`.

## Passo 2 — Consolidar

1. Escolher um commit-base único (o mais recente comum a todos, ou o que o dono indicar) e aplicar os 15 diffs nele, um de cada vez, na ordem numérica (P813→P827), resolvendo conflitos manualmente quando aparecerem — **não usar merge automático sem revisão** nos arquivos "hub" listados acima.
2. Para cada conflito real (duas alterações incompatíveis no mesmo trecho, não só linhas adjacentes): decidir qual versão prevalece com base na medição, não na ordem de chegada — se os dois passos mediram o mesmo comportamento vanilla de forma consistente, a integração deve ser trivial; se mediram de forma inconsistente, isso é um achado novo (contradição entre dois relatórios) e precisa ser resolvido antes de prosseguir, não silenciado pelo merge.
3. Prestar atenção especial ao caso já identificado: garantir que a correção de `read_binario_pipeline` (da adenda de P824) sobrevive à consolidação, e que P827 não reintroduz a mensagem antiga por aplicar o diff dele sobre uma cópia de `03_infra/src/world.rs` anterior à correção.

## Passo 3 — Validar o estado consolidado

1. `cargo build --release` limpo.
2. `cargo test --workspace` — uma corrida só, sobre a árvore consolidada. Registrar o comando exato e a saída completa (não só o resumo final), para as contagens de `typst-core`, `typst-infra`, `typst-shell`, `04_wiring`/CLI.
3. Comparar a contagem final contra a soma ingênua dos "+N" declarados em cada um dos 15 relatórios — se não bater, investigar a diferença (teste perdido no merge, teste duplicado, ou teste que dois passos declararam como "novo" sendo na verdade o mesmo).
4. `crystalline-lint .` sobre a árvore consolidada — confirmar que os 6 warnings V7 de "prompt órfão" mencionados repetidamente nos 15 relatórios continuam sendo os mesmos 6 (não crescem por causa da consolidação) e que não aparece nenhum V3/V4/V5/V13/V14 novo.
5. Rodar pelo menos um caso de teste manual (`.typ` de fixture) de cada um dos passos que tocaram o mesmo arquivo (a lista do Passo 1.2), para confirmar que a consolidação não regrediu nenhum deles — não confiar só na suíte automatizada para essa checagem cruzada.

## Passo 4 — Commit e relatório

1. Só depois do Passo 3 fechar limpo: commitar o estado consolidado (mensagem de commit referenciando P813–P828).
2. Produzir `00_nucleo/diagnosticos/typst-passo-828-relatorio.md` com: a lista de conflitos encontrados e como foram resolvidos, a saída completa de `cargo test --workspace` (comando + contagem final por suíte), a comparação contra a soma ingênua dos "+N" dos 15 relatórios, e confirmação de que os casos manuais do Passo 3.5 continuam corretos.
3. Depois deste passo, e só depois dele, as contagens de teste dos relatórios P813–P827 devem ser tratadas como históricas (o que cada um provou na sua árvore parcial), não como o estado atual do projeto — o estado atual passa a ser o que este relatório de P828 documentar.
