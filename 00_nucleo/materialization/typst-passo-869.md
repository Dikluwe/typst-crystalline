# Prompt — typst-passo-869: fechar as duas lacunas deixadas por P868

**Origem**: P868 pulou duas coisas que o próprio prompt dele pedia explicitamente. Este passo só resolve essas duas — não é para reabrir a consolidação inteira.
**Estado**: aguardando execução

---

## Lacuna 1 — reconciliar a contagem de testes de verdade

P868 declarou a corrida final (`typst-core = 4682`) como referência sem comparar contra a soma dos testes novos declarados pelos sete relatórios (P862 a P867), como o prompt original pedia.

1. Baseline antes de P862 (fim de P860): `typst-core = 4655`. Final consolidado: `4682`. Diferença real: **+27**.
2. Somar os testes novos declarados em cada relatório individual, um a um, com cuidado especial no P864 — a seção 6 dele diz "7 testes novos", mas a soma das três tabelas de teste na seção 4.2 do mesmo relatório (lista→enum, enum, terms) dá 3+5+2 = 10. Usar a contagem real dos nomes de teste listados, não o número resumido, se os dois divergirem.
3. Se a soma correta não bater com +27, achar onde está a diferença: teste que dois relatórios reivindicam como "novo" mas é o mesmo; teste que foi descrito mas não ficou no código; ou erro de aritmética num dos relatórios. Não aceitar "a soma não bate mas o número final está certo" sem essa explicação — o objetivo é saber por que não bate, não só confirmar que a suíte está verde.

## Lacuna 2 — isolar de verdade o diff do P863

P868 rodou `cargo clean` + teste sobre a árvore **já consolidada** (todos os sete passos juntos), e concluiu que a causa da contradição P863/P864 era só cache. Isso não isola a variável pedida — não prova que P863 sozinho, sem P864 por cima, já passava antes de qualquer interação.

1. Reconstruir uma árvore com **só** o diff de P863 aplicado sobre o commit base (`06a336b0c`), sem P861, P862, P864, P865, P866, P867.
2. `cargo clean` + `cargo test p863_show_par_func_transforma_paragrafo` nesse estado isolado, do zero, sem cache.
3. Se passar isolado: confirma que a explicação de cache do P868 estava certa, e agora com o método correto, não só com um argumento plausível.
4. Se falhar isolado: significa que P863 nunca passou de verdade sozinho, e a explicação de P868 estava errada — nesse caso, investigar o que realmente fazia esse teste depender de algo de outro passo (provavelmente P864, dado que ele fez correções de compilação em código que P863 tinha tocado, conforme o próprio relatório de P864 registrou).

## Relatório

`00_nucleo/diagnosticos/typst-passo-869-relatorio.md` com: a reconciliação de contagem completa (Lacuna 1), com a diferença explicada até fechar, e o resultado do teste isolado de verdade (Lacuna 2), com o veredito final sobre se a explicação de "artefato de cache" do P868 se sustenta.
