# Passo 927 — Opção 6: disparar o scan de coverage só quando o documento tem carácter não coberto

**Precede este passo**: `typst-passo-926-relatorio.md` — Opção 1 confirmada com regressão real
(1.12-1.48× nos 7 cenários canônicos); Opção 5 (thread de fundo) não ajuda porque o fallback é
tipicamente necessário antes da thread terminar. Opção 6 proposta como mais promissora: escanear
os codepoints do documento antes do layout; só disparar o scan caro de fontes do sistema se
encontrar um bloco não coberto pela fonte primária.

**Pré-condição de árvore**: `git status`. Confirmar que a reversão de P926 (Opção 5) está limpa —
`world.rs` e o L0 de volta ao estado de `da18ea9f3`.

---

## Fase A — desenhar o scan antes de implementar

1. Confirmar onde é mais barato interceptar os codepoints do documento antes do layout — o parser
   já produz uma árvore de sintaxe com todo o texto; confirmar se dá para percorrer essa árvore
   (barato, já em memória) em vez de re-ler o ficheiro fonte.
2. Confirmar o que conta como "coberto pela fonte primária" — mesmo bitmap de coverage por blocos
   de 256 codepoints já usado em `FontBook`/`coverage_cache` (P925 A.1), reaproveitar a
   representação existente, não inventar uma nova.
3. **Caso importante a não esquecer**: texto que só existe depois de avaliação (`context`,
   interpolação de variáveis, conteúdo vindo de `read()`/dados externos, `#for` sobre uma lista
   computada) não está disponível só lendo a árvore de sintaxe bruta — só existe depois do `eval`.
   Confirmar se isto é uma lacuna aceitável (scan cobre o caso comum — texto literal — e casos
   dinâmicos continuam a pagar o custo lazy original, sem regressão face a hoje) ou se precisa de
   scan depois do `eval`, antes do layout (mais caro, mas mais completo). Decidir e registar —
   não implementar assumindo que "só o source literal" é suficiente sem confirmar que é aceitável.
4. Confirmar o custo do próprio scan do documento (percorrer a árvore/texto e verificar coverage
   contra a fonte primária) — deve ser desprezível comparado ao custo de abrir dezenas/centenas de
   ficheiros de fonte, mas medir, não presumir.

## Fase A.1 — gate se necessário

Se a Fase A concluir que o scan precisa de acontecer pós-`eval` (não só no source bruto): isso
pode mudar a ordem do pipeline (scan entre eval e layout) — parar, editar L0s, confirmar com o
dono antes da Fase B. Se o scan de source bruto for suficiente: TDD directo, sem gate adicional
(não muda contrato público, só adiciona uma verificação antes de decidir se dispara o caminho
já existente).

## Fase B — Implementação (TDD directo se não houver mudança de pipeline; dois agentes se houver)

1. Teste com medição real: documento latino puro deve ter **zero** aberturas de ficheiro de fonte
   além da fonte primária (mesmo padrão de contagem de P890/925/926); documento com CJK deve
   disparar o scan exactamente como antes (sem regressão face ao comportamento actual para esse
   caso, só evitando o custo quando não é preciso).
2. Implementar.
3. Suíte completa verde, discriminada por crate.
4. Recompilar os 7 cenários canônicos **e** os 4 casos de bloco Unicode de P923/925/926.
5. `cargo run -- .` — zero violations.

## Fase C — Regressão (9 cenários, mesmo conjunto de P926)

Benchmark completo:
- 7 canônicos, `depois/antes` — confirmar **zero regressão** desta vez (era o objetivo de existir
  a Opção 6; se ainda regredir, a causa precisa de nova investigação, não aceitar como "normal").
- `utf8-latin`/`utf8-greek` (sem fallback esperado) — confirmar tempo igual ou melhor que o
  original (nenhum scan disparado).
- `utf8-cjk`/`utf8-emoji` (fallback esperado) — confirmar que o scan ainda dispara e resolve o
  outlier (mesma melhoria que a Opção 1 mostrou isoladamente, já que quando dispara é o mesmo
  mecanismo).

Attestation completa (`L11`) para os 9.

## Resultado esperado

- Scan de coverage do documento implementado, disparando o caminho caro só quando necessário.
- Caso latino puro: zero custo adicional, confirmado por contagem de aberturas de ficheiro, não só
  tempo.
- Caso CJK/emoji: mesma melhoria que a Opção 1 (~8.6s → ~0.2s), sem o custo de arranque
  incondicional.
- Decisão registada sobre o caso de texto dinâmico (`context`/dados externos) — coberto ou
  scope-out explícito, não esquecido.
- Benchmark completo, 9 cenários, atestado, zero regressão no caso comum.
