# Passo 1019 — Fechar: `heading` avança contador sem `#set heading(numbering:)`

**Tipo**: Investigar → classificar gate → corrigir se aprovado.
**Achado em P1016** (nota lateral, não investigado em profundidade — este passo faz essa
investigação): cristalino avança o counter de `heading` mesmo sem `numbering:` definido;
vanilla não avança nesse caso.
**Pré-condição**: `git status` limpo. Independente do Passo 1018 (pode correr em
paralelo ou em qualquer ordem — não partilha código).

---

## Fase A — Medir e confirmar antes de desenhar

1. Reproduzir directamente:
   ```typst
   = A
   = B
   #context counter(heading).get()
   ```
   sem `#set heading(numbering:)`. Comparar cristalino vs vanilla.
2. Repetir com `#set heading(numbering: "1.")` definido — confirmar que aí os dois batem
   (o P1016 só mediu o caso sem numbering).
3. Localizar por `file:line` onde o cristalino decide avançar o counter — é incondicional
   (avança sempre que um heading é processado), ou condicional a outra coisa que não
   `numbering`?
4. Confirmar no vanilla a regra exacta: o avanço é condicional à presença de `numbering`,
   ou a outra coisa (ex.: `outlined`, nível do heading)? Não presumir que é só
   `numbering:` sem ler o código vanilla.

## Fase B — Classificar o gate

Mudança de comportamento por defeito (documentos sem `numbering:` definido passam a ter
`counter(heading).get()` a devolver outro valor) → categoria 2, ADR-0127. Escrever L0,
apresentar para aprovação antes de código.

```
Dado heading sem #set heading(numbering:)
Quando #context counter(heading).get() é chamado depois de N headings
Então devolve o valor correcto per a regra vanilla confirmada na Fase A (não presumir (0,)
  — confirmar qual é o valor certo, pode não ser simplesmente "não avança")
```

Não-regressão: todos os testes de `heading`/numbering/outline existentes.

## Fase C — Implementar (só após gate) e validar

```
crystalline-lint .
cargo test --workspace
```

---

## Resultado esperado

`heading` só avança o counter quando a regra vanilla (confirmada na Fase A, não
presumida) manda avançar. Se a Fase A revelar que a causa é partilhada com o item do
Passo 1018 (mesma raiz de `CounterRegistry`), reportar a sobreposição e não duplicar o
fix.
