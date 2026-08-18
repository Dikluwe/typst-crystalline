# Passo 1059 — Margin collapsing parágrafo→bloco, reconciliação de numeração, decalque do corpus

**Tipo**: Investigar → gate (`ADR-0127`) → corrigir, só se confirmado. Junta as três
pendências deixadas em aberto pelos Passos 1057/1058, para fechar os dois de vez.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1058.

---

## Parte 1 — Testar margin collapsing parágrafo→bloco (a mais importante)

**Contexto**: o L0 do P1058 descreve `par.spacing` como colapsável via `max(S1, S2)`
entre blocos adjacentes com espaçamentos diferentes. O código implementado no P1057
(`extra_spacing = (spacing_pt - leading_pt).max(0.0)`) é **aditivo**, só resolve
parágrafo→parágrafo, e não interage com o espaçamento de um bloco seguinte diferente.
Se o vanilla faz mesmo *collapsing*, este é o ponto onde a implementação actual
provavelmente diverge.

### Fase A — Medir, não presumir

Construir e comparar contra vanilla, pelo menos:

```typst
Parágrafo normal de teste.

= Heading
```

```typst
Parágrafo normal.

#block(spacing: 2em)[Bloco com espaçamento diferente]
```

```typst
#block(spacing: 0.5em)[Bloco pequeno]

Parágrafo normal.
```

Medir a distância vertical entre os dois elementos nos dois binários (mesmo `pdftotext
-bbox-layout` de sempre). Se baterem: não há bug, o collapsing já acontece por outro
mecanismo — documentar onde, fechar sem código. Se não baterem: acende exactamente a
previsão da explicação do P1058.

### Fase B — Se confirmado, localizar a causa e classificar o gate

Confirmar se é falta de collapsing mesmo, ou outra coisa (ex.: heading pode ter o seu
próprio espaçamento fixo, não relacionado com `par.spacing`/`block.spacing` de todo —
não presumir que a causa é a mesma do parágrafo→parágrafo só porque a hipótese aponta
para lá). Gate `ADR-0127` categoria 2/3 antes de qualquer código — muda output visual em
qualquer documento com parágrafo seguido de heading/lista/bloco, o que é praticamente
todo documento real.

### Fase C — Corrigir (só após gate) e validar

Se confirmado e aprovado: implementar `max(S1, S2)` no ponto de transição entre
elementos de bloco distintos, não só a fórmula aditiva actual de `Parbreak`. Testar os
3 casos da Fase A, mais os já existentes de par-a-par (não regressão).

---

## Parte 2 — Reconciliar numeração: "Passo 1056" citado no relatório do P1057

O relatório do P1057 abre com *"No Passo 1056, identificou-se e corrigiu-se..."*, mas
não existe relatório separado de um P1056 neste registo. Confirmar:
1. Houve mesmo um passo 1056 executado à parte (investigação), com relatório próprio
   nunca enviado? Se sim, enviar agora, para o registo ficar completo.
2. Ou foi só imprecisão de numeração no texto, e é tudo o mesmo passo (1057)? Se sim,
   corrigir a frase no relatório do P1057 para não referenciar um passo que não existe.

Não é bloqueante para o código — é higiene de registo, mas fica por resolver há dois
passos.

## Parte 3 — Decalque do corpus canónico (pendente desde o P1057)

O Passo 1057 original pedia explicitamente "decalque contra o corpus canónico completo,
não só um caso isolado — dado o alcance" (qualquer documento com 2+ parágrafos). O
relatório do P1057 só mostrou o teste unitário sintético + suite completa, sem menção a
corpus. Correr agora:

```
<comparar cada documento do corpus canónico entre vanilla e cristalino, cristalino
pós-P1057, mesmo procedimento de sempre>
```

Reportar qualquer divergência nova introduzida pelo P1057 (não esperada, dado que só
corrige um mecanismo, mas confirmar, não presumir).

---

## Validação final (as três partes)

```
crystalline-lint .
cargo test --workspace
```

---

## Resultado esperado

Margin collapsing parágrafo→bloco confirmado correcto ou corrigido com gate. Numeração
do registo reconciliada. Corpus canónico decalcado sem divergência nova. Com isto, os
Passos 1057 e 1058 ficam definitivamente fechados.
