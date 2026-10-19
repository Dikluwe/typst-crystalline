# Passo 991 — centralização horizontal errada em delimitador empilhado manual (`(conteúdo \ conteúdo)`), `binom()` como referência

**Precede este passo**: achado externo (2026-08-08) — na construção manual `(n \ k)` (quebra de
linha `\` dentro de parênteses, seção 7 do documento de teste, `(n \ k) = n!/(k!(n-k)!)`), o
delimitador empilhado (`⎛⎜⎝`/`⎞⎟⎠`) já está correto (peças formando um parêntese único e alto,
cobrindo as duas linhas — **não mexer nisso**, é o comportamento certo, inclusive mais correto que
o próprio vanilla neste caso específico, per a nota do achado). O problema é a **centralização
horizontal do conteúdo dentro do delimitador**:

| construção | n: folga esq/dir | k: folga esq/dir |
|---|---|---|
| `binom(n, k)` (referência, correta) | 1.10 / 1.10 | 1.45 / 1.62 |
| `(n \ k)` (manual, empilhado — errada) | 0.00 / 0.00 | 0.70 / 0.17 |

"n" encosta nas duas peças do delimitador sem margem nenhuma; "k" fica deslocado para a direita,
quase encostando na peça direita.

**Pré-condição de árvore**: `git status`. Confirmar P990 presente.

---

## Fase A — comparar os dois caminhos de código dentro do próprio cristalino

1. Confirmar que construção sintática produz `(n \ k)` — provavelmente `Content::MathDelimited`
   envolvendo um `Content::MathAlignPoint`/grelha de linhas via `\`, distinto de como `binom(n, k)`
   é construído (P899, mecânica de matriz per achado de P946).
2. Ler o código de `binom()` (já identificado em P946 como usando mecânica de matriz) e confirmar
   exatamente como ele centra cada linha horizontalmente dentro da largura da coluna/grelha.
3. Ler o código do caminho manual (`(... \ ...)` dentro de delimitador) — confirmar se usa a mesma
   função de centralização de célula/grelha que `binom()`, ou um caminho separado que replica a
   lógica sem a mesma correção (mesma classe de "caminho irmão duplicado" já vista em P988 Parte
   A — assembly horizontal vs variante única).
4. Confirmar se a largura usada para centrar cada linha é a largura da **grelha inteira** (maior
   linha) em ambos os casos, ou se o caminho manual usa uma largura errada (por exemplo, a largura
   do próprio item em vez da largura da coluna) — isso explicaria por que "n" (mais estreito) fica
   sem margem nenhuma e "k" fica deslocado, em vez de ambos centrados numa largura comum.

## Fase B — Implementação (TDD directo se for aplicar a mesma função de centralização já usada por
`binom()`; protocolo de dois agentes se os dois caminhos precisarem de unificação mais profunda)

1. Teste com os valores esperados (folga simétrica, aproximadamente 1.10pt para "n" e média de
   1.45/1.62 para "k", ou a fórmula exata que `binom()` já usa) para o caminho manual.
2. Implementar — preferencialmente reaproveitando a mesma função/caminho que `binom()` já usa
   corretamente, em vez de duplicar a correção.
3. Suíte completa verde, discriminada por crate. Confirmar que `binom()` continua correto (guarda
   de não-regressão).
4. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Medir de novo `(n \ k)` no documento de 30 secções — confirmar folga simétrica, próxima da de
   `binom()`.
2. Confirmação visual a alta resolução, lado a lado com `binom()` na mesma seção.
3. Benchmark completo, 7 cenários canónicos, `depois/antes`, zero regressão.

## Resultado esperado

- Conteúdo de `(... \ ...)` centrado horizontalmente com folga simétrica e confortável, igual ao
  padrão já correto de `binom()`.
- `binom()` inalterado (guarda).
- Se a causa for um caminho de código duplicado: idealmente unificado com o de `binom()`, não só
  corrigido em paralelo — reduz a chance de um terceiro caminho repetir o mesmo bug no futuro.
- Benchmark sem regressão.
