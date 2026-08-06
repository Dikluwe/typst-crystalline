# Passo 986 — linha diagonal manual (`a+b` com peças de canto) na posição errada — efeito de risco virou sublinhado

**Precede este passo**: achado da auditoria (2026-08-06, §7.3) — o mais sério dos três desta
rodada, porque não é diferença de alguns pontos, é **mudança de efeito visual/semântico**. No
vanilla, a linha diagonal cruza *por dentro* do texto (início 0.68pt abaixo do topo do texto, fim
10.3pt abaixo — efeito de risco/tachado). No cristalino, a linha fica *inteira abaixo* do texto
(início 8.8pt abaixo, fim 17.3pt abaixo — efeito de sublinhado). Deslocamento vertical de ~8pt.

**Pré-condição de árvore**: `git status`. Confirmar P985 presente.

---

## Fase A — confirmar a causa

1. Reproduzir isoladamente o construto (linha diagonal manual via peças de canto — confirmar a
   sintaxe exacta usada no documento de teste, seção 10) e medir a posição vertical da linha
   relativa ao texto.
2. Ler o mecanismo de posicionamento vertical deste tipo de linha/decoração manual — confirmar se
   é o mesmo tipo de erro de convenção de baseline (`y=0=topo` vs `y=0=baseline`) já visto três
   vezes nesta frente (P901 radical, P919 axis, P972 parênteses em fração) — se for o quarto caso
   da mesma família, considerar se vale, desta vez, fazer a varredura mais ampla que P973 já tinha
   feito para `FrameItem` em `math/layout/`, mas agora incluindo este tipo de construto
   (possivelmente fora desse directório).
3. Confirmar a posição vertical esperada real do vanilla para este construto — ler o mecanismo
   correspondente, `file:line`.

## Fase B — Implementação (TDD directo se for correção de offset pontual; considerar protocolo de
dois agentes se a causa revelar mais de um ponto a corrigir, mesma disciplina de P952/959)

1. Teste com a posição vertical esperada da linha (início e fim), derivada da fórmula real do
   vanilla.
2. Implementar.
3. Suíte completa verde, discriminada por crate.
4. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Medir a posição da linha de novo — confirmar que cruza o texto como o vanilla, não fica
   inteira abaixo.
2. Confirmação visual a alta resolução — confirmar o efeito de risco/tachado, não sublinhado.
3. Se a Fase A confirmar que é a mesma classe de erro de convenção de baseline já vista três
   vezes: considerar varredura mais ampla (nota da Fase A.2) como candidato a passo seguinte,
   não necessariamente resolvida aqui.
4. Benchmark completo, 7 cenários canónicos, `depois/antes`, zero regressão.

## Resultado esperado

- Linha diagonal manual posicionada como o vanilla (cruzando o texto, efeito de risco), não
  deslocada ~8pt abaixo (efeito de sublinhado).
- Confirmação se é a quarta ocorrência da família de erro de convenção de baseline já vista nesta
  frente, com decisão registada sobre varredura mais ampla.
- Benchmark sem regressão.
