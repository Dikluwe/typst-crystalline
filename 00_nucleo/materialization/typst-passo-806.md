# Prompt — typst-passo-806 (achado P798 #12): `model` — `#par[...]` como função dá `unknown variable: par`

**Origem**: P798 (lote 3 de triagem em lote, corrigido), tabela "Achados de P798, aguardando passo dedicado"
**Handoff**: `00_nucleo/handoff-novo-chat-p798.md`
**Módulo afectado**: `model`
**Estado**: aguardando execução, ainda não corrigido

---

## Achado (texto exacto do handoff)

> `#par[...]` como função dá `unknown variable: par` — vanilla aceita

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" ou "mecanicamente correto" sem execução mostrada. Cada afirmação do relatório deste passo tem de vir acompanhada do comando exacto e da saída literal, comparando vanilla (`lab/typst-original/target/release/typst`) e cristalino (`./target/release/typst`). A contagem de testes da suíte `typst-core` tem de aparecer no relatório e bater com o número de testes novos declarados.

---

## Passo 1 — Sonda (obrigatória antes de qualquer alteração de código)

1. Compilar `#par[conteúdo de teste]` com os dois binários. Registar a saída literal de cada um — o cristalino deve estar a devolver `unknown variable: par`.
2. Localizar no código-fonte do vanilla (`lab/typst-original/`) onde `par` é registado como função invocável na stdlib (`make_stdlib` ou equivalente), e confirmar a assinatura completa (argumentos nomeados, se existirem — ex.: `leading:`, `justify:`, `linebreaks:`).
3. Localizar no código do cristalino se `par` já existe como construção (`Content::Par`, já usado noutros contextos do projecto — ver `00_nucleo/adr/typst-adr-0060-model-structural-roadmap.md`) mas não está registado como função invocável na `Scope` global, ou se está totalmente ausente.
4. Registar os dois pontos (vanilla e cristalino) no relatório antes de tocar em código.

## Passo 2 — Implementação

Registar `native_par` (ou equivalente) na stdlib do cristalino, ligando-a à representação de `Content::Par` já existente se ela existir (evitar duplicar mecanismo — ver `00_nucleo/prompts/entities/style_chain.md` e ADR-0060 para o estado actual de `Par`). Se `Content::Par` não existir ainda, isso é uma lacuna maior do que o achado original sugeria — registar essa descoberta no relatório antes de decidir a extensão da implementação.

## Passo 3 — Validação

1. Recompilar o cristalino.
2. Repetir o comando do Passo 1 e mostrar a saída literal, agora igual à do vanilla.
3. Adicionar casos de teste cobrindo `#par[...]` chamado como função, incluindo pelo menos um argumento nomeado se a assinatura do vanilla os tiver.
4. Correr a suíte `typst-core` completa e mostrar o comando e a contagem de testes antes/depois.

## Passo 4 — Relatório

Produzir `00_nucleo/materialization/typst-passo-806-relatorio.md` com:
- Comando + saída literal do Passo 1 (antes da correcção).
- Trecho do código vanilla e do código cristalino identificados no Passo 1.
- Nota explícita se `Content::Par` já existia ou teve de ser criada.
- Diff da correcção.
- Comando + saída literal do Passo 3 (depois da correcção).
- Contagem de testes antes/depois.
