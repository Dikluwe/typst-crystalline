# Passo 1047 — Completar `cases()`: `delim`, `reverse`, `gap` ponta a ponta

**Tipo**: Investigar (já feito, ver diagnóstico) → gate (`ADR-0127`, categoria 2/3 — muda
comportamento por defeito e output visual) → corrigir. Fecha a parte de `cases` do
**Achado C (P998)** — `mat`/`vec` já confirmados corrigidos (P1030/verificação do P1046);
`cases` é o último dos três.
**Base**: diagnóstico completo do P1046 (4 pontos da cadeia desconectados: set-rule,
avaliação inline, entidade AST, layout).
**Aplicar a emenda do P1042**: `gap = style.size * 0.2` em `cases.rs` é uma constante
geométrica sem proveniência explícita no L0 — mesmo que o valor (`0.2em`) coincida com o
default real do vanilla, o código precisa de **ler o campo `gap` do elemento quando
definido pelo utilizador**, não usar a constante sempre. Tratar como a mesma classe de
correcção da emenda, não só "adicionar parâmetro em falta".
**Pré-condição**: `git status` limpo.

---

## Fase A — Confirmar os defaults e o mecanismo exacto antes de codificar

1. Confirmar por `file:line` no vanilla (`matrix.rs:238-265`, já citado) os três defaults:
   `delim` = `{`, `reverse` = `false`, `gap` = `0.2em`.
2. Confirmar o mecanismo de `reverse` no vanilla — "inverte a posição do delimitador para
   a direita dos ramos" — ler exactamente como isto afecta o layout (o delimitador muda
   de lado, o conteúdo mantém-se à esquerda? confirmar geometria exacta antes de
   implementar, não presumir "espelhar tudo").

## Fase B — Os 4 pontos da cadeia, na ordem

1. **Entidade AST** (`entities/elements/math_cases.rs`): adicionar `delim`, `reverse`,
   `gap` a `MathCasesElem`. **Mudança de campo público — gate `ADR-0127` ponto 1.**
2. **Avaliação inline** (`eval/math.rs:940-970`): parar de descartar argumentos nomeados
   — ler `delim:`/`reverse:`/`gap:` da chamada `cases(...)`.
3. **Set-rule** (`eval/rules.rs:225-285`): adicionar `("cases", "delim")`,
   `("cases", "reverse")`, `("cases", "gap")` a `MATH_SET_LIGADOS`, mesmo padrão já usado
   para `mat`/`vec`.
4. **Layout** (`math/layout/cases.rs:20-90`):
   - Delimitador: usar o campo do elemento em vez de `'{'` fixo.
   - Gap: usar o campo do elemento (com fallback ao default `0.2em` só quando não
     definido pelo utilizador) em vez de `style.size * 0.2` sempre.
   - `reverse`: implementar o posicionamento lateral confirmado na Fase A.

## Fase C — Critérios de verificação

```
Dado #set math.cases(delim: "[") seguido de cases(1, 2)
Quando renderizado
Então usa "[" como delimitador, batendo com vanilla

Dado cases(1, 2, gap: 0.5em) inline
Quando renderizado
Então espaçamento vertical entre ramos é 0.5em, não o default 0.2em

Dado cases(1, 2, reverse: true)
Quando renderizado
Então delimitador aparece do lado confirmado na Fase A, batendo com vanilla

Dado cases(1, 2) sem nenhum parâmetro nomeado
Quando renderizado
Então comportamento inalterado (delim "{", gap 0.2em, sem reverse) — guarda de
  não-regressão directa contra os testes já existentes de cases
```

Não-regressão: todos os testes de `cases` já existentes (incluindo os do P1042, que
confirmaram largura a 0.00pt — confirmar que continuam a bater depois desta mudança).

## Fase D — Implementar e validar

```
crystalline-lint .
cargo test --workspace
```
Decalque contra `00_nucleo/corpus-docs/math/cases.typ` e o corpus canónico.

---

## Resultado esperado

`cases()` com `delim`/`reverse`/`gap` funcionais nos dois caminhos (inline e set-rule),
completando o Achado C do P998 para os três elementos (`mat`/`vec`/`cases`). Constante de
`gap` corrigida para ler o campo do elemento, não hardcoded, per a emenda do P1042.
