# Prompt — typst-passo-802 (achado P798 #5): `utils::listset` — falta warning de label não-anexada

**Origem**: P798 (lote 3 de triagem em lote, corrigido), tabela "Achados de P798, aguardando passo dedicado"
**Handoff**: `00_nucleo/handoff-novo-chat-p798.md`
**Módulo afectado**: `utils::listset`
**Estado**: aguardando execução, ainda não corrigido

---

## Achado (texto exacto do handoff)

> Falta warning de label não-anexada (`query(<lbl>)` sobre label órfã)

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" ou "mecanicamente correto" sem execução mostrada. Cada afirmação do relatório deste passo tem de vir acompanhada do comando exacto e da saída literal, comparando vanilla (`lab/typst-original/target/release/typst`) e cristalino (`./target/release/typst`). A contagem de testes da suíte `typst-core` tem de aparecer no relatório e bater com o número de testes novos declarados.

---

## Passo 1 — Sonda (obrigatória antes de qualquer alteração de código)

1. Reproduzir com um documento `.typ` mínimo que declare uma label não anexada a nenhum elemento (label órfã) e chame `query(<lbl>)` sobre ela. Compilar com os dois binários e registar a saída literal (incluindo warnings) de cada um.
2. Confirmar que o vanilla emite um warning nesse caso e o cristalino não emite nenhum.
3. Localizar no código-fonte do vanilla (`lab/typst-original/`) o ponto que detecta a label órfã e emite o warning — texto exacto da mensagem e condição de disparo.
4. Localizar no código do cristalino o mecanismo de `query()`/introspecção equivalente e confirmar a ausência dessa verificação.
5. Registar os dois pontos (vanilla e cristalino) no relatório antes de tocar em código.

## Passo 2 — Implementação

Adicionar a verificação e o warning no cristalino, replicando a condição de disparo e o texto da mensagem do vanilla (adaptado ao formato de diagnóstico já usado no projecto — ver `00_nucleo/adr/typst-adr-0045-formato-diagnosticos.md`). Não alterar o comportamento de `query()` para labels anexadas.

## Passo 3 — Validação

1. Recompilar o cristalino.
2. Repetir o comando do Passo 1 e mostrar a saída literal, agora incluindo o warning.
3. Adicionar caso de teste cobrindo label órfã com `query()` (e um caso de controlo com label anexada, sem warning).
4. Correr a suíte `typst-core` completa e mostrar o comando e a contagem de testes antes/depois.

## Passo 4 — Relatório

Produzir `00_nucleo/materialization/typst-passo-802-relatorio.md` com:
- Comando + saída literal do Passo 1 (antes da correcção).
- Trecho do código vanilla e do código cristalino identificados no Passo 1.
- Diff da correcção.
- Comando + saída literal do Passo 3 (depois da correcção).
- Contagem de testes antes/depois.
