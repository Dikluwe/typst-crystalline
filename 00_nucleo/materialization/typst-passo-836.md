# Prompt — typst-passo-836: `text::font::variations` — parâmetro `variations:` de `#text` ausente (achado #21)

**Origem**: achado #21 de P831 (lote 5)
**Estado**: aguardando execução

---

## Achado (medição de P831)

`#text(variations: (wght: 250))` — cristalino `error: text() argumento nomeado desconhecido: 'variations'` (exit 1 em 5/5 fixtures testadas); vanilla compila os valores válidos e dá erros específicos com hints nos inválidos (ex.: `tag must be one to four characters in length` + `found 5 characters`). Vanilla: `text/mod.rs:850`, `variations.rs:217-236`. Cristalino: `01_core/src/engine/eval/stdlib/text.rs:92-100` (qualquer named arg diferente de `fill` é erro hard).

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino). Contagem de testes de `typst-core` a bater com os testes novos.

## Passo 1 — Sonda

1. Testar `variations:` com valores válidos (tags de 1-4 caracteres, ex. `wght`, `ital`, `opsz`) e inválidos (tag maior que 4 caracteres, tag vazia, valor fora de faixa se o vanilla validar isso) nos dois binários.
2. Confirmar no vanilla a estrutura completa: `variations:` é um dicionário de tag→valor numérico; localizar a validação de tag (`variations.rs:217-236`) e o texto exato dos erros/hints.
3. Confirmar como as variações se propagam até a seleção/instanciação de fonte variável (o cristalino já tem alguma infraestrutura de eixo de variação, tocada em achados anteriores como P772o/P772u do handoff antigo — verificar se pode ser reaproveitada).

## Passo 2 — Implementação

Adicionar o argumento nomeado `variations:` em `native_text` (ou onde `text()` for implementado), com validação de tag replicando o vanilla, propagando para a mesma infraestrutura de eixo de variação já existente.

## Passo 3 — Validação

1. Recompilar. Casos válidos e inválidos batendo com o vanilla (incluindo hints).
2. Confirmar visualmente/geometricamente (mesmo método usado em achados anteriores de fonte variável) que a variação tem efeito real no glifo renderizado, não só que o argumento é aceito sem erro.
3. Suíte completa, comando + contagem antes/depois.

## Relatório

`00_nucleo/diagnosticos/typst-passo-836-relatorio.md` com medição antes, código identificado, diff, medição depois, contagem de testes.
