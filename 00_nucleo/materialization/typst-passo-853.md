# Prompt — typst-passo-853: `ref()` não aceita uma label como argumento (achado #63 de P848, incidental)

**Origem**: achado #63 de P848 — incidental, emergiu na triagem do lote 6, não pertence a nenhum dos 7 módulos triados (é do domínio de introspecção/referência)
**Estado**: aguardando execução

---

## Achado (medição de P848)

`#ref(<label>)` — cristalino: `error: ref() espera nome como string, recebeu label` (exit 1); vanilla: aceita uma label diretamente como argumento (e só depois valida se o alvo referenciado tem numeração/é referenciável).

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino). Contagem de testes de `typst-core` a bater com os testes novos.

## Passo 1 — Sonda

1. Testar `#ref(<algumlabel>)` com um label que existe e aponta para um elemento numerado (ex.: um heading com numbering) e com um label que existe mas aponta para algo sem numeração (confirmar o comportamento do vanilla nesse segundo caso — provavelmente erro específico, não o mesmo erro de tipo).
2. Testar também `#ref("string")` (se essa forma existir no vanilla) para confirmar se `ref()` aceita só label, só string, ou ambos com semânticas diferentes.
3. Localizar no vanilla (`lab/typst-original/`) a assinatura de `ref()` — tipo do argumento principal, e a validação que ocorre depois de resolver o label (elemento referenciável ou não).
4. Localizar no cristalino onde `native_ref` valida o argumento e confirmar que rejeita `Value::Label` categoricamente.

## Passo 2 — Implementação

Aceitar `Value::Label` (além do que já for aceito) em `ref()`, propagando para a resolução normal de referência já usada por outros mecanismos do projeto que resolvem labels (query, locate, etc. — reaproveitar, não duplicar resolução de label). Replicar a validação pós-resolução do vanilla (erro específico se o alvo não for referenciável).

## Passo 3 — Validação

1. Recompilar. `#ref(<label>)` funcionando com label existente e numerado, batendo com o vanilla (texto da referência renderizado).
2. Testar o caso de label existente mas não-referenciável, e label inexistente — mensagens batendo com o vanilla.
3. Suíte completa, comando + contagem antes/depois.

## Relatório

`00_nucleo/diagnosticos/typst-passo-853-relatorio.md` com medição antes, código identificado, diff, medição depois, contagem de testes.
