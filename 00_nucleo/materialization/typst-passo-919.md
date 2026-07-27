# Passo 919 — desalinhamento vertical entre elementos adjacentes numa sequência matemática

**Precede este passo**: `typst-passo-917-relatorio.md`, "Achado registado, não corrigido" —
`x^2_i + (1/2)` mostra `(1/2)` desalinhado verticalmente em relação a `x^2_i +`, confirmado
pré-existente (idêntico com/sem as correções dessa sessão, via `git stash`). Hipótese não
confirmada nesse relatório: `apply_axis_offset` recentra cada `MathBox` independentemente antes de
`hconcat`, sem eixo comum entre irmãos.

**Prioridade mais alta dos quatro itens residuais** — afeta qualquer sequência com mais de um
elemento (a maioria dos casos reais), não um caso de nicho.

**Pré-condição de árvore**: `git status`. Confirmar P918 (4 commits) presente.

---

## Fase A — confirmar a hipótese antes de implementar (per `ADR-0123`, ler o vanilla primeiro)

1. Ler `hconcat`/`hconcat_spaced` (`math/layout/mod.rs`) e confirmar exactamente como cada
   `MathBox` irmã é posicionada verticalmente antes de serem concatenadas horizontalmente — a
   hipótese do relatório é que cada uma é recentrada no seu próprio eixo (`apply_axis_offset`)
   independentemente, sem nenhum eixo comum entre elas. Confirmar lendo o código real, não
   assumir que a hipótese já registada está certa.
2. Ler o mecanismo equivalente do vanilla (`typst-layout/src/math/`, provavelmente em torno de
   `MathRun`/`multiline`/junção horizontal de fragmentos) — confirmar como o vanilla garante que
   elementos numa mesma linha partilham um eixo vertical comum, `file:line`.
3. Isolar um caso mínimo (`$ x^2_i + (1/2) $` ou mais simples, `$ a + (b/c) $`) e medir com
   `mutool trace` a posição Y real de cada elemento no cristalino vs. vanilla — confirmar
   quantitativamente o desalinhamento antes de desenhar a correção (mesma disciplina de `L11`:
   recibo antes de remendar).
4. Confirmar se a correcção é: (a) um eixo comum calculado uma vez para toda a sequência antes de
   posicionar qualquer elemento (exige conhecer todos os irmãos antes de `hconcat` — mudança de
   ordem de operações), ou (b) cada elemento já carrega informação suficiente (a sua própria
   `axis_height`) para ser alinhado a posteriori sem recalcular layout. Desenhar antes de
   implementar, com custo/risco de cada abordagem registado.

## Fase A.1 — gate se necessário

Se a Fase A concluir que a correcção exige mudar a ordem de operações de `hconcat`/assinatura de
função pública: parar, editar L0s, sincronizar hashes, aguardar confirmação do dono — mesmo
protocolo de P893/896/906/909/915/918.

## Fase B — Implementação (protocolo de dois agentes de P898 — geometria com potencial efeito
em cascata sobre toda sequência matemática, risco alto)

1. Agente A escreve testes com ground-truth medido do vanilla real (Fase A ponto 3), cobrindo pelo
   menos: dois elementos de alturas diferentes lado a lado devem partilhar eixo; três ou mais
   elementos (não só o caso de dois, para confirmar que a solução generaliza).
2. Agente B implementa.
3. Revisão do orquestrador — caso composto não coberto: sequência com fracção **e** delimitador
   **e** sub/sobrescrito na mesma linha, todos ao mesmo tempo.
4. Suíte completa verde, discriminada por crate.
5. Confirmação visual/geométrica (`mutool trace`, `L11`/`ADR-0123`): recibo real, posições Y de
   cada elemento, comparadas ao vanilla, não "parece alinhado".
6. `cargo run -- .` — zero violations.

## Fase C — Regressão

Benchmark completo, 7 cenários, `--warmup 5 -m 20`, attestation completa (comando, números,
comparação — `L11`).

## Resultado esperado

- Causa confirmada por leitura do vanilla, não suposição.
- Recibo de posição Y antes/depois, comparado ao vanilla.
- Testes cobrindo 2 e 3+ elementos.
- Benchmark completo atestado.
