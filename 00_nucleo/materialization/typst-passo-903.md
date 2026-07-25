# Passo 903 — espaçamento ausente em torno de texto entre aspas em modo matemático

**Precede este passo**: `typst-passo-897.md`, secção "Achado registado, fora de escopo deste passo".
Reler os exemplos concretos lá (`"sujeito a"`, `"is natural"`, `"for all"`, secções 24 e 30) antes
de começar.

**Pré-condição de árvore**: `git status`.

---

## Sintoma (já catalogado, não redescobrir)

Texto literal entre aspas dentro de `$...$` não recebe espaço de nenhum dos lados no cristalino
(`𝑓(𝑥)sujeito a𝑔𝑖(𝑥)`), enquanto o vanilla mantém espaço normal antes e depois
(`𝑓(𝑥) sujeito a 𝑔𝑖(𝑥)`).

## Fase A — diagnóstico

1. Confirmar a hipótese já registada em P897: mecanismo provavelmente parecido com o de P891
   (`compute_gaps`/`spacing_between`, `01_core/src/engine/math/layout/spacing.rs`) — texto literal
   pode não ter uma classe (`MathClass`) reconhecida para efeitos de espaçamento contra vizinhos, do
   mesmo jeito que script-size não tinha antes de P891. Confirmar lendo o código, não presumir que a
   hipótese está certa só porque é plausível — já aconteceu nesta frente (P891/892) de uma hipótese
   plausível estar errada.
2. Confirmar qual `MathClass` (ou equivalente) o texto literal recebe hoje no cristalino, e qual
   deveria receber segundo o vanilla (`lab/typst-original/`) — texto literal normalmente tem uma
   classe própria (`MathClass::Alphabetic` ou similar) que já deveria ter regra de espaçamento
   contra identificadores/operadores adjacentes.
3. Isolar um caso mínimo (`$ a "texto" b $`) e confirmar o sintoma isoladamente antes de mexer em
   código — o caso das secções 24/30 tem mais contexto ao redor, útil confirmar que o mecanismo não
   depende de outra coisa presente ali (por exemplo, estar dentro de uma equação com numeração
   activa, ou perto de outros elementos).

## Fase B — Implementação (TDD, mesmo padrão de P891 — mudança de regra de espaçamento, não precisa
dos dois agentes, é adição de caso a uma tabela de regras já existente, não cálculo geométrico novo)

1. Teste que falhe primeiro: `compute_gaps`/`spacing_between` (ou onde a Fase A localizar) devolve
   o espaçamento correto para o par (texto literal, identificador)/(identificador, texto literal).
2. Implementar a regra.
3. Suíte completa verde, discriminada por crate.
4. Recompilar as secções 24 e 30 e confirmar visualmente que o espaçamento aparece, sem quebrar
   nenhum caso onde texto literal já funcionava (se houver algum caso já correto no ficheiro atual,
   confirmar que continua correto).
5. `cargo run -- .` — zero violations.

## Fase C — Regressão

Benchmark completo, 7 cenários, `--warmup 5 -m 20`.

## Resultado esperado

- Header de linhagem actualizado.
- Teste novo cobrindo o par de classes corrigido.
- Relatório com: causa exacta confirmada (classe errada ou regra ausente), confirmação visual nas
  secções 24/30, benchmark completo.
