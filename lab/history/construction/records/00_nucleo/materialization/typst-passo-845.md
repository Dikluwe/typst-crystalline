# Prompt — typst-passo-845: `loading` — texto com `\n` trunca o shaping (achado #55)

**Origem**: achado #55 de P831 (lote 5)
**Estado**: aguardando execução

---

## Achado (medição de P831)

`#"a\nb\nc"` (uma string com newlines, exibida diretamente, não via `read()` de arquivo) — cristalino mostra só `a` (para no primeiro `\n`); vanilla mostra `a|b|c` (as três linhas, cada `\n` virando quebra de parágrafo/linha visível). Causa: `03_infra/src/shaper.rs:830` usa só `bidi.paragraphs[0]` — o texto é dividido em parágrafos pelo algoritmo bidi, mas só o primeiro é processado. Vanilla: `typst-layout/src/inline/linebreak.rs:69`, `shaping.rs:675-690`.

**Nota importante**: apesar do módulo ser `loading` na tabela de P831, a causa raiz não é de carregamento de arquivo — é do shaping de texto em geral. Qualquer texto com `\n` embutido (vindo de `read()`, de uma string literal, de `eval`, etc.) deve ser afetado, não só o caminho de `loading`.

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino). Contagem de testes de `typst-core` a bater com os testes novos.

## Passo 1 — Sonda

1. Reproduzir o caso mínimo (`#"a\nb\nc"`) e confirmar a extensão do problema: testar também `read()` de um arquivo com múltiplas linhas (o caminho original do achado) e uma string vinda de `#eval`.
2. Confirmar no cristalino (`03_infra/src/shaper.rs:830`) como `bidi.paragraphs` é populado e por que só o índice 0 é usado — é uma limitação de implementação (esquecimento de iterar) ou uma decisão consciente que não foi documentada?
3. Confirmar no vanilla como múltiplos parágrafos dentro do mesmo elemento de texto são tratados no shaping (`linebreak.rs:69`, `shaping.rs:675-690`).

## Passo 2 — Implementação

Corrigir o shaper para iterar sobre todos os parágrafos de `bidi.paragraphs`, não só o primeiro, produzindo a quebra de linha visível para cada `\n` interno.

## Passo 3 — Validação

1. Recompilar. Repetir os casos do Passo 1 (string literal, `read()`, `eval`), todos mostrando as linhas completas, batendo com o vanilla.
2. Testar um caso com muitas linhas (5+) e com linhas vazias consecutivas (`\n\n`) para confirmar que a correção é geral, não só para o caso de 3 linhas.
3. Confirmar que texto sem `\n` (a maioria dos casos) não regride.
4. Suíte completa, comando + contagem antes/depois.

## Relatório

`00_nucleo/diagnosticos/typst-passo-845-relatorio.md` com medição antes, código identificado, diff, medição depois, contagem de testes.
