# Passo 987 — número de equação (`(1)`, `(2)`) flutua ~200pt longe do conteúdo em página auto-width

**Precede este passo**: achado mais grave de toda a auditoria (§8.7, 2026-08-06) — no vanilla,
`(1)` fica a ~7pt da equação (`𝐸 = 𝑚𝑐² (1)`). No cristalino, o número fica a **~206pt de
distância**, quase na margem direita da página (x=496.47 contra a equação terminando em ~x=288,
numa folha de 538.88pt de largura). Já tinha sido notado de passagem em `P975` ("registada aqui
para investigação futura, sem passo ainda") — nunca chegou a ser corrigido. Categoricamente
diferente dos outros achados desta frente (desvio de dezenas/centenas de pontos, não alguns
pontos).

**Pré-condição de árvore**: `git status`. Confirmar P986 presente.

---

## Fase A — confirmar a causa

1. Reproduzir isoladamente: documento com `#set page(width: auto)` (ou equivalente que produza
   página de largura automática, como o documento de teste usa) e uma equação numerada — confirmar
   a posição do número reproduz o problema fora do documento de 30 secções.
2. Ler o mecanismo do vanilla para posicionamento do número de equação — já parcialmente
   mencionado em P952 ("o vanilla coloca o número logo após o conteúdo — a linha colapsa para a
   largura do conteúdo"). Confirmar exactamente como o vanilla decide a largura da linha/coluna
   onde o número é alinhado à direita, quando a página em si é `auto`.
3. Ler a implementação actual do cristalino — candidato à causa: o número está a ser alinhado à
   direita da **largura computada da página** (que pode ser grande, dado `width: auto` só limita
   depois de todo o conteúdo ser medido), em vez de à direita da **largura da linha/conteúdo**
   local onde a equação está. Confirmar isto lendo o código de posicionamento do número
   (`equation.rs` ou `flow`/layout de bloco numerado).
4. Confirmar se isto afeta só páginas `auto`-width, ou também páginas de largura fixa (o
   documento de 30 secções usa `auto`, então pode ser que o bug só apareça nesse caso
   específico) — testar com página de largura fixa para isolar.

## Fase B — Implementação (protocolo de dois agentes de P898 — geometria com efeito potencialmente
amplo, qualquer documento com equação numerada é afectado)

1. Agente A escreve testes cobrindo: equação numerada em página `auto`-width (o caso do bug);
   equação numerada em página de largura fixa (guarda de não-regressão, se já estiver correcto
   nesse caso); equação numerada dentro de uma coluna/container mais estreito que a página.
2. Agente B implementa a correcção.
3. Revisão do orquestrador — confirmar com um documento de várias equações numeradas em sequência,
   larguras de conteúdo diferentes, que cada número acompanha a respectiva equação, não uma
   largura de página global.
4. Suíte completa verde, discriminada por crate.
5. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Medir a posição do número no documento de 30 secções (seção 20 e qualquer outra com equação
   numerada) — confirmar proximidade ao conteúdo, próxima dos ~7pt do vanilla.
2. Confirmação visual — número visivelmente associado à equação, não flutuando solto.
3. `compare.py`/comparação de posição no documento completo.
4. Benchmark completo, 7 cenários canónicos, `depois/antes`, zero regressão.

## Resultado esperado

- Número de equação posicionado próximo ao conteúdo em página `auto`-width, não à direita da
  largura computada da página inteira.
- Confirmado que páginas de largura fixa não regridem.
- Benchmark sem regressão.
