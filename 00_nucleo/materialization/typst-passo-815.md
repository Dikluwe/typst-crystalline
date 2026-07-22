# Prompt — typst-passo-815: `typst_eval::methods` — método inexistente diverge, dict-key-call sem hints (achado #2 de P810)

**Origem**: achado #2 da tabela de P810
**Estado**: aguardando execução

---

## Achado (texto do relatório de P810)

> método inexistente: caminho normal diverge; dict-key-call sem hints

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Comando exacto + saída literal (vanilla vs cristalino) para cada afirmação. Contagem de testes de `typst-core` a bater com os testes novos declarados.

---

## Passo 1 — Sonda

1. Compilar `#"texto".metodo_inexistente()` (método chamado num tipo que não o tem) com os dois binários — registar a mensagem literal de cada um. Confirmar em que exactamente divergem (texto da mensagem, hints sugeridos, tipo mencionado).
2. Compilar um caso de "dict-key-call": um dicionário com uma chave que parece um método mas não existe como tal, chamado como se fosse método (`#dict.chave_que_nao_e_metodo()` ou `#dict.metodo_inexistente()` sobre um `dictionary`) — confirmar que o vanilla emite hints (ex.: sugestão de nome próximo, ou explicação de que dicionários não têm métodos arbitrários) que o cristalino não emite.
3. Localizar no vanilla (`lab/typst-original/`) a rotina de resolução de método (`crates/typst-eval/src/methods.rs` ou equivalente) — texto exacto das mensagens de erro e lógica dos hints (provavelmente usa distância de edição para sugerir nomes próximos, como já visto noutros achados do projecto, ex. `unknown variable` com hint de subtracção em P772r).
4. Localizar o caminho equivalente no cristalino e identificar a divergência.
5. Registar os pontos antes de tocar em código.

## Passo 2 — Implementação

Corrigir a mensagem de erro de método inexistente para bater com o vanilla, e adicionar os hints em falta no caso de dict-key-call, reaproveitando o mecanismo de sugestão por distância de edição já existente no projecto se ele já cobrir esse padrão (ver P772r).

## Passo 3 — Validação

1. Recompilar. Repetir os comandos do Passo 1, mensagens literais batendo com o vanilla.
2. Testes cobrindo método inexistente em pelo menos dois tipos diferentes (string, dict) e o caso de dict-key-call com hint.
3. Suíte `typst-core` completa, comando + contagem antes/depois.

## Passo 4 — Relatório

`00_nucleo/diagnosticos/typst-passo-815-relatorio.md` com: medição antes, código vanilla/cristalino identificados, diff, medição depois, contagem de testes.
