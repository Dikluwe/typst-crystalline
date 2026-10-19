# Prompt — typst-passo-850: `Duration` sem representação com sinal (resto do achado #59 de P832) — decisão do dono

**Origem**: achado #59 de P831, fechado em P832 para `Angle`/`Ratio`/`Fraction` — `Duration` ficou de fora, registrado como bloqueio real, não decisão inventada
**Estado**: aguardando decisão do dono — **não implementar sem essa decisão**

---

## O que já foi medido (P832, não precisa refazer)

`#repr(-duration(seconds: 3))` — vanilla `duration(seconds: -3)`; cristalino `error: cannot apply Neg to duration`.

Causa: a entidade `Duration` do cristalino é representada como `u64` de nanossegundos (`01_core/src/entities/duration.rs:17-19`) — **sem sinal, por construção**. O vanilla representa duração com sinal e produz valores negativos normalmente. Não dá para implementar `Neg` sem antes migrar a representação inteira da entidade.

## Escopo real da mudança, se for feita

Migrar `Duration` para uma representação com sinal toca, no mínimo:
- O construtor (`duration(seconds: -3, ...)` — hoje provavelmente já rejeita ou trunca valores negativos nos argumentos, não só no resultado de operações; confirmar).
- Toda a aritmética existente sobre `Duration` (soma, subtração, comparação, se já implementadas).
- O `repr()` de `Duration` (que P843 já reescreveu para o formato nomeado do vanilla — confirmar que a mudança de sinal não quebra o que P843 fez).
- Qualquer lugar que hoje assuma `Duration` não-negativo implicitamente (por exemplo, cálculo de diferença de tempo, se existir).

## Passo 1 — Apresentar a decisão ao dono

1. Confirmar com uma sonda rápida se o **constructor** de `duration()` já aceita componentes negativos ou não (`duration(seconds: -3)` direto, sem passar por `Neg`) — isso muda o tamanho real do problema: se o constructor já aceita negativo e só a entidade internamente já suporta sinal de alguma forma, a mudança pode ser mais barata do que parece.
2. Se o constructor também rejeita negativo (provável, dado que a representação é `u64`), apresentar ao dono: migrar para `i64` (ou tipo com sinal equivalente) é mudança de tipo que atravessa vários pontos já implementados (listados acima) — pedir decisão sobre se vale a pena agora ou se fica como débito.

## Passo 2 — Implementar conforme a decisão

Se decidido migrar: trocar a representação, atualizar todos os pontos que a tocam (constructor, aritmética existente, repr — não assumir que são só esses, confirmar por grep de usos de `Duration` no código antes de considerar completo), adicionar `Neg`.

## Passo 3 — Validação (se implementado)

`#repr(-duration(seconds: 3))` → `duration(seconds: -3)`, batendo com o vanilla. Confirmar que os testes de `Duration` já existentes (de P843 e de antes) continuam passando sem regressão — a mudança de representação interna não deve alterar nenhum comportamento já correto. Suíte completa, comando + contagem antes/depois.

## Relatório

`00_nucleo/diagnosticos/typst-passo-850-relatorio.md` — se a decisão for manter como está: formalizar como débito (nova entrada), com o critério de reabertura. Se for migrar: relatório completo com os pontos tocados listados.
