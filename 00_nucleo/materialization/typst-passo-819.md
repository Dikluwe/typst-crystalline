# Prompt — typst-passo-819: `foundations::plugin_` — `plugin.transition` ausente, mensagens L1 divergentes, spans detached (achado #6 de P810)

**Origem**: achado #6 da tabela de P810
**Estado**: aguardando execução

---

## Achado (texto do relatório de P810)

> `plugin.transition` ausente (não registado no L0); mensagens L1 divergentes; spans detached (caminho feliz em paridade total)

Nota: o achado confirma que o caminho feliz (plugin a funcionar correctamente) já está em paridade — o problema é só nos casos de erro/feature específica.

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Comando exacto + saída literal (vanilla vs cristalino) para cada afirmação. Contagem de testes de `typst-core` a bater com os testes novos declarados.

---

## Passo 1 — Sonda

1. Confirmar no vanilla (`lab/typst-original/`) o que é `plugin.transition` — provavelmente uma função relacionada ao ciclo de vida/estado de um plugin WASM entre chamadas. Localizar a assinatura e o comportamento exacto.
2. Testar chamar `plugin.transition` (ou o equivalente) no cristalino — confirmar `unknown` ou erro de ausência.
3. Reproduzir um erro de carregamento/execução de plugin (ex.: plugin WASM inválido, ou função exportada com assinatura errada) nos dois binários — comparar as mensagens L1 (mensagens de baixo nível, próximas ao runtime WASM) e confirmar a divergência.
4. Confirmar os spans `<detached>` em erros de plugin — testar um erro que no vanilla aponta para a chamada `plugin(...)` no documento e comparar com o span do cristalino.
5. Registar os pontos antes de tocar em código.

## Passo 2 — Implementação

Implementar `plugin.transition` replicando a semântica do vanilla. Ajustar as mensagens de erro L1 para bater com o texto do vanilla onde a exactidão da mensagem importa (ver critério já usado no projecto — mensagens que mudam significado vs mensagens só estéticas). Corrigir o span dos erros de plugin para apontar para a chamada no documento, não `<detached>`.

## Passo 3 — Validação

1. Recompilar. Repetir os comandos do Passo 1, saída literal batendo com o vanilla.
2. Testes cobrindo `plugin.transition`, uma mensagem de erro L1 específica, e o span do erro.
3. Suíte `typst-core` completa, comando + contagem antes/depois.

## Passo 4 — Relatório

`00_nucleo/diagnosticos/typst-passo-819-relatorio.md` com: medição antes, código vanilla/cristalino identificado, diff, medição depois, contagem de testes. Actualizar o L0 de plugin (mencionado no achado como estando sem esta função registada).
