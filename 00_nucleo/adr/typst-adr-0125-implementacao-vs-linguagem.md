# ADR-0125 — Diferença de implementação vs. diferença de linguagem

**Estado:** `EM VIGOR`
**Data:** 2026-07-09
**Aplica-se a:** qualquer passo que introduza sintaxe, argumento, propriedade, ou comportamento novo que um documento `.typ` possa invocar.
**Histórico de numeração:** sem ficheiro/número próprio até P910; reconciliada nesse passo por
varredura real de `00_nucleo/adr/` (slot `0125` livre, último da remessa depois de `0124`).

---

## A distinção

**Diferença de implementação (aceitável):** o cristalino resolve um problema de forma diferente do vanilla por dentro — cache diferente, algoritmo diferente, estrutura de dados diferente — mas um documento `.typ` válido produz o mesmo resultado visível nos dois lados. O utilizador nunca sabe, nem precisa de saber, que a implementação é diferente.

**Diferença de linguagem (inaceitável, salvo decisão consciente):** a sintaxe, os argumentos aceites, ou o comportamento observável de uma construção divergem entre cristalino e vanilla. Um documento escrito para um pode falhar ou comportar-se de forma diferente no outro. Isto quebra a portabilidade — a razão de ser de um projecto de paridade.

## O que já correu mal

`variant: (eixo: valor)` (P660) foi implementado a partir de um pedido que assumiu, sem confirmar, que era sintaxe real do vanilla. A sonda do próprio P660 confirmou o contrário — o vanilla rejeita essa sintaxe. A implementação avançou de qualquer forma, criando uma extensão de linguagem por engano, não por decisão.

Isto só foi apanhado porque o relatório de P660 foi lido com atenção, não porque nenhuma regra já existente o tivesse impedido de acontecer.

## Regra

Antes de implementar qualquer sintaxe, argumento nomeado, propriedade, ou comportamento que um documento `.typ` possa invocar, confirmar contra o binário vanilla de referência **antes** de escrever código, não depois:

```bash
lab/typst-original/target/release/typst compile <documento-de-teste>.typ saida.pdf
```

Se o vanilla aceitar: a implementação é uma correcção de paridade normal.

Se o vanilla rejeitar: parar antes de implementar, e decidir explicitamente:

1. **É uma extensão deliberada, com valor próprio?** (como `table.numbering`, P459 — conveniência real dentro do espírito da linguagem, mesmo divergindo da arquitectura). Documentar como tal, com razão escrita, e avisar claramente que não é portável para o Typst real.
2. **É um erro de partida?** (como `variant: (eixo: valor)`, P660). Não implementar, ou reverter se já implementado.

A diferença entre os dois casos não é técnica — é se alguém, a olhar para a proposta antes de qualquer código, decidiu conscientemente introduzir a divergência, com razão escrita, ou se a divergência apareceu por a confirmação nunca ter sido feita antes de avançar.

## Como aplicar

No modelo de passo já usado neste projecto, qualquer passo que introduza sintaxe nova adiciona, na secção de sonda, um passo explícito:

```markdown
## Confirmação contra o vanilla (obrigatória antes de implementar)

- [ ] Sintaxe/comportamento testado directamente contra `lab/typst-original/target/release/typst`.
- [ ] Se aceite pelo vanilla: prosseguir como correcção de paridade normal.
- [ ] Se rejeitado pelo vanilla: decisão explícita registada — extensão deliberada (com razão) ou não implementar.
```

## Ligação às regras anteriores

- **Decisão nova obrigatória** (citada por nome; não materializada como ADR própria — ver nota de reconciliação em ADR-0119), **disciplina de verificação** (ADR-0119), **proveniência de medição** (ADR-0121), **paridade de defeitos** (ADR-0122), **checklist de sub-layouts** (ADR-0124) — todas tratam de como verificar bem uma correcção já dentro do âmbito de paridade.
- **Esta regra** trata de uma pergunta anterior a todas essas: antes de sequer começar a implementar, a coisa que se está prestes a construir existe no vanilla, ou está-se prestes a inventar linguagem nova sem se dar por isso?
