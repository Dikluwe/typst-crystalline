# Prompt — typst-passo-824: `loading::read_` — `encoding:` rejeitado, não-UTF8 devolve bytes em silêncio (achado #11 de P810)

**Origem**: achado #11 da tabela de P810
**Estado**: aguardando execução

---

## Achado (texto do relatório de P810)

> `encoding:` rejeitado (língua válida); não-UTF8 devolve bytes em silêncio (comentário "heurística vanilla" **refutado** por medição)

Nota importante: o achado já registou que um comentário existente no código do cristalino, que justificava o comportamento actual como "heurística do vanilla", foi **refutado por medição** — ou seja, o comentário estava errado. Confirmar isso de novo neste passo antes de corrigir, e remover/corrigir o comentário incorrecto junto com o código.

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Comando exacto + saída literal (vanilla vs cristalino) para cada afirmação. Contagem de testes de `typst-core` a bater com os testes novos declarados.

---

## Passo 1 — Sonda

1. Compilar `#read("ficheiro.txt", encoding: "utf8")` (ou outra codificação válida suportada pelo vanilla) com os dois binários — confirmar que o cristalino rejeita o argumento `encoding:` mesmo com um valor válido.
2. Preparar um ficheiro com bytes não-UTF8 e compilar `#read("ficheiro_nao_utf8.txt")` — confirmar que o vanilla erra (ou se comporta de forma documentada e específica) e o cristalino devolve os bytes crus silenciosamente, sem erro nem warning.
3. Localizar no vanilla (`lab/typst-original/`, `crates/typst-library/src/loading/read.rs` ou equivalente) o comportamento real para (a) o argumento `encoding:` e (b) ficheiro não-UTF8 sem `encoding:` explícito — medir exactamente o que faz, para poder confirmar ou refutar de vez a suposição de "heurística" que o comentário antigo do cristalino assumia.
4. Localizar o código do cristalino, incluindo o comentário mencionado no achado, e confirmar a divergência.
5. Registar os pontos antes de tocar em código.

## Passo 2 — Implementação

Adicionar suporte a `encoding:` (pelo menos os valores que o vanilla aceita, replicando a lista). Corrigir o comportamento de ficheiro não-UTF8 para bater com o medido no Passo 1.3 (erro, se for isso que o vanilla faz). Remover ou corrigir o comentário "heurística vanilla" no código, já que a medição o refutou.

## Passo 3 — Validação

1. Recompilar. Repetir os comandos do Passo 1, saída literal batendo com o vanilla.
2. Confirmar que leitura de ficheiro UTF-8 válido sem `encoding:` continua a funcionar sem regressão.
3. Testes novos cobrindo `encoding:` válido e ficheiro não-UTF8.
4. Suíte `typst-core` completa, comando + contagem antes/depois.

## Passo 4 — Relatório

`00_nucleo/diagnosticos/typst-passo-824-relatorio.md` com: medição antes, código vanilla/cristalino identificado (incluindo o comentário incorrecto e a sua remoção/correcção), diff, medição depois, contagem de testes.
