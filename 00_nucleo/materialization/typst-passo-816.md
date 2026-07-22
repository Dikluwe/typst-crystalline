# Prompt — typst-passo-816: `typst_library::diag` — `#set` com propriedade inválida vira warning silencioso em vez de erro (achado #3 de P810, prioridade alta)

**Origem**: achado #3 da tabela de P810, marcado prioridade alta no handoff (`handoff-novo-chat-p810.md`)
**Estado**: aguardando execução

---

## Achado (texto do relatório de P810)

> `#set` prop inválida → warning (exit 0!) em vez de erro; sem warning "unknown font family"; `set text(size: 12)` aceite

---

## Por que é prioridade alta

Um documento com um `#set` de propriedade inválida compila sem erro no cristalino (`exit 0`) quando deveria falhar. Isto é silêncio quando devia errar — a mesma categoria de gravidade que motivou boa parte dos achados fechados em P787-P797 (handoff anterior a P798).

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Comando exacto + saída literal (vanilla vs cristalino) para cada afirmação. Contagem de testes de `typst-core` a bater com os testes novos declarados.

---

## Passo 1 — Sonda

1. Compilar um `#set` com propriedade que não existe na função alvo (ex.: `#set text(propriedade_que_nao_existe: 1)`) com os dois binários — confirmar que o vanilla erra (código de saída ≠ 0) e o cristalino só avisa (`warning`) e sai com `exit 0`. Registar os comandos e códigos de saída exactos (`echo $?` depois de cada compilação).
2. Testar `#set text(size: 12)` — sem unidade, um `Int` onde o vanilla espera `Length`. Confirmar que o vanilla rejeita (`expected length, found integer` ou equivalente) e o cristalino aceita.
3. Testar `#set text(font: "Fonte Que Não Existe")` — confirmar que o vanilla emite `warning: unknown font family` (ou mensagem equivalente) e o cristalino não emite nada.
4. Localizar no vanilla (`lab/typst-original/`) a rotina de validação de `#set` (provavelmente em `typst-eval` ou na validação de argumentos nomeados da função-alvo) — como decide entre warning e erro, e como valida tipo de valor mesmo dentro de `#set`.
5. Localizar o caminho equivalente no cristalino e identificar por que aceita nomes de propriedade inválidos e tipos errados sem erro.
6. Registar os pontos antes de tocar em código.

## Passo 2 — Implementação

Corrigir `#set` para validar nome de propriedade (erro, não warning, se a propriedade não existir na função-alvo) e tipo de valor (mesma validação de tipo já usada em chamadas normais de função). Adicionar o warning de "unknown font family" no ponto correcto (resolução de fonte, não em `#set` em si — confirmar no vanilla onde exactamente esse warning é emitido, pode ser só no momento do layout/shaping, não na avaliação do `#set`).

## Passo 3 — Validação

1. Recompilar. Repetir os três comandos do Passo 1, código de saída e mensagens batendo com o vanilla.
2. Confirmar que `#set` com propriedades válidas continua a funcionar sem regressão (teste de controlo).
3. Testes novos cobrindo propriedade inválida (erro), tipo errado (erro), fonte inexistente (warning).
4. Suíte `typst-core` completa, comando + contagem antes/depois.

## Passo 4 — Relatório

`00_nucleo/diagnosticos/typst-passo-816-relatorio.md` com: medição antes (incluindo códigos de saída), código vanilla/cristalino identificados, diff, medição depois, contagem de testes.
