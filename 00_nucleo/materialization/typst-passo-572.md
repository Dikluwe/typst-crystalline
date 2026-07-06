---
# P572 — Decidir o destino do código L1 sem L0 (`layout_space`)

> **Passo:** 572
> **Data:** 2026-07-05
> **Foco:** P571 encontrou código em L1 (`cursor.rs`, `mod.rs`, `text.rs`) que introduz `layout_space()` e um campo `pending_space_width`, sem documento L0 correspondente, e sem estar submetido ao histórico do git. Este código afecta todos os nove snapshots, não só os de RTL. Antes de decidir se escrever o L0 que falta ou descartar o código, é preciso saber para que serve.
> **Tipo:** Sonda. Sem regenerar snapshots, sem escrever L0, sem descartar nada, antes de responder à pergunta.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.** A Trava Arquitectural do projecto — código em L1 sem L0 não avança, mas também não se descarta sem entender o que representa.
> **Dependências:** P571 (onde o código foi encontrado, isolado, e a causa das cinco falhas confirmada).

---

## Sonda

### Ler o código directamente, não assumir a partir do nome das funções

```bash
git diff HEAD -- 01_core/src/rules/layout/cursor.rs 01_core/src/rules/layout/mod.rs 01_core/src/rules/layout/text.rs
```

Ler o diff completo. Perguntas a responder:

1. `layout_space()` resolve um problema geral de espaçamento (por exemplo, um bug em texto latino que ainda não tinha sido encontrado), ou é uma tentativa de resolver o mesmo problema do espaço em árabe que P569 já resolveu de outra forma, em L3?
2. Se for uma tentativa alternativa ao que P569 já fez: as duas abordagens competem, ou complementam-se? Ter as duas ao mesmo tempo pode causar dupla-correcção ou conflito.
3. Existe algum comentário no código, mensagem de commit não finalizada, ou nota em qualquer sítio que explique a intenção de quem escreveu isto?

### Verificar se há relação com algum passo já numerado

```bash
grep -rn "layout_space\|pending_space_width" 00_nucleo/ 2>/dev/null
```

Se não houver nenhuma referência em nenhum relatório já escrito, isto confirma que o código não tem origem documentada — precisa de ser tratado como órfão, não como parte de um trabalho já em curso que só falta terminar.

### Critério de fecho da sonda

- [ ] Propósito do código confirmado pela leitura directa, não por suposição.
- [ ] Confirmado se compete ou complementa a correcção já feita por P569 em L3.
- [ ] Confirmado se há alguma origem documentada, ou se é código sem registo.

---

## Decisão

Três caminhos, dependendo da sonda:

1. **Se o código resolver um problema real, diferente do que P569 já tratou:** escrever o L0 que falta, com a descrição exacta do que resolve e porquê está em L1 (e não em L3, como P569 preferiu para o caso do árabe). Só depois disso commitar e regenerar snapshots.
2. **Se o código for uma tentativa alternativa e redundante ao que P569 já resolveu:** descartar, confirmando que a solução de P569 (em L3, sem tocar no Layouter) já é suficiente sozinha, sem precisar de mudanças em L1.
3. **Se não for possível determinar a intenção com confiança:** não decidir às pressas. Registar como código órfão, sem dono conhecido, e deixar de lado até haver mais informação — não apagar sem certeza, mas também não legitimar sem entender.

---

## Critério de fecho do passo

- [ ] Sonda completa, propósito do código confirmado ou declarado desconhecido com razão.
- [ ] Um dos três caminhos escolhido, com decisão escrita.
- [ ] Se for o caminho 1: L0 escrito antes de qualquer commit.
- [ ] Se for o caminho 2: código descartado, working tree limpo, confirmado que P569 continua suficiente sozinho.
- [ ] Se for o caminho 3: registado como código órfão, sem acção imediata.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p572.md`.
- [ ] Corrigir o relatório de P569 (a frase "pré-existente" incorrecta, já apontada por P571), independentemente do caminho escolhido aqui.

---

## Nota

Só depois deste passo é que faz sentido regenerar os snapshots P307b — a regenerar antes disso, sem saber se o código de L1 deve ficar ou sair, corre-se o risco de fixar no histórico um estado que ninguém decidiu de propósito.
