# Passo 900 — bug `Str + Content` (causa real do crash 2 catalogado em P894)

**Precede este passo**: `typst-passo-894-relatorio.md`, catalogação do crash 2. Ler antes de
começar — este prompt assume que a causa exacta ainda precisa de ser confirmada por leitura de
código; se P894 já a isolou com precisão (confirmar releitura do relatório antes de escrever
código), a Fase A abaixo pode ser mais curta do que está descrita.

**Pré-condição de árvore**: `git status`. Confirmar estado de P899 (se já em curso/commitado) antes
de começar — ficheiros provavelmente não se sobrepõem (P899 é `stdlib`/`rules/math`, este passo é
provavelmente `eval`/`ops` ou equivalente, confirmar na Fase A), mas verificar.

---

## Sintoma (conforme catalogado em P894 — reler o relatório para o caso mínimo exacto antes de
começar, este prompt não repete os detalhes)

Um crash de compilação relacionado com uma operação `Str + Content` (concatenação/soma entre um
valor `Str` e um valor `Content`) num contexto de modo matemático — confirmar contra o relatório de
P894 qual construção exacta do `.typ` dispara isto.

## Fase A — diagnóstico

1. Reler `typst-passo-894-relatorio.md` para o caso mínimo exacto e a mensagem de erro/panic
   completa já capturada nesse passo.
2. Confirmar onde a operação `+` entre `Value::Str` e `Value::Content` é avaliada
   (`01_core/src/engine/eval/` — operadores binários) e o que acontece hoje: falta um braço no
   `match` de tipos, ou existe um braço mas com lógica errada que causa panic (unwrap em `None`,
   índice fora de limites, etc)?
3. Confirmar como o vanilla trata `Str + Content` (`lab/typst-original/`) — é erro de tipo
   controlado (mensagem de erro amigável, sem panic), coerção implícita (Str vira Content de texto
   antes de somar), ou outra coisa? Não presumir — ler o código.
4. Confirmar se este é um caso isolado ou se há outras combinações de tipo em operadores binários
   com o mesmo problema (`Content + Str` na ordem inversa, `Int + Content`, etc — grep pelo `match`
   de tipos do operador `+` e ver quais braços faltam comparado ao vanilla).

## Fase B — Implementação (TDD)

1. Teste que falhe primeiro: reproduzir o caso mínimo confirmado na Fase A, esperando o
   comportamento correcto identificado (erro controlado ou resultado válido, conforme a Fase A
   determinar) em vez de panic.
2. Implementar a correcção.
3. Se a Fase A ponto 4 encontrar outras combinações de tipo com o mesmo problema: decidir se
   corrigem-se todas aqui (mais consistente) ou só a reportada (mais contido) — registar a decisão.
4. Suíte completa verde, discriminada por crate.
5. Recompilar a secção 26 (ou a secção relevante confirmada em P894) do `.typ` de 30 secções e
   confirmar que já não crasha, com output visual correcto.
6. `cargo run -- .` — zero violations.

## Fase C — Regressão

Benchmark completo, 7 cenários, `--warmup 5 -m 20`.

## Resultado esperado

- Header de linhagem actualizado.
- Teste(s) novo(s) cobrindo o caso e, se aplicável, as combinações de tipo relacionadas.
- Relatório com: causa exacta confirmada, comportamento correcto implementado (erro controlado vs
  coerção, conforme vanilla), confirmação visual, benchmark completo.
