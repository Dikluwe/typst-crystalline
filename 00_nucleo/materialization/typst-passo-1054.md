# Passo 1054 — V21: triar e fechar por categoria (não item a item cego)

**Tipo**: Triagem primeiro (Fase A), depois correcção por categoria. A lista de V21
pós-Passo 0067 revela três classes distintas, cada uma com tratamento diferente — não
tratar como 60-80 investigações independentes.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 0067 (tekt-linter) aplicado ao
`crystalline-lint` usado neste repo.

---

## Fase 0 — Confirmar a contagem e separar por categoria

```
crystalline-lint --checks v21 . > /tmp/v21-completo.txt
grep -c 'Escalar contextual fixo' /tmp/v21-completo.txt
grep -c 'Citação obsoleta' /tmp/v21-completo.txt
```

## Categoria A — Citação obsoleta (prioridade 1, pode ser auto-infligida)

`grep 'Citação obsoleta' /tmp/v21-completo.txt` — casos conhecidos já vistos:
`math/layout/frac.rs:173` (→ `fraction.rs:60`), `math/layout/tests.rs:3783` (→
`compiler/layout/metrics.rs:59`), `underover.rs:117` e `:146` (→ `scripts.rs:300/306`).

1. Para cada um: confirmar se o caminho antigo citado corresponde a um ficheiro que
   existia **antes** de algum dos fatiamentos desta frente (P1002, P1013, P1014, P1022,
   P1032, etc.) — procurar no histórico git (`git log --all --full-history -- '*fraction.rs'`
   ou equivalente) se esse ficheiro existiu e foi renomeado/fatiado.
2. Se confirmado: a citação não está errada no conteúdo, só desactualizada no caminho —
   corrigir para o caminho/linha actuais do mesmo conteúdo (não reinvestigar o valor,
   só a localização). Registar como lição: fatiar/mover ficheiro exige grep de citações
   a apontar para o caminho antigo, adicionar isso ao checklist do `auditar-fatiamento.md`.
3. Se não for isso (a citação estava errada desde sempre, não é por causa de mover
   ficheiro): tratar como Categoria C (investigação nova).

## Categoria B — Valor já confirmado legítimo noutro passo, só falta comentário no código

Casos já sabidos: `matrix.rs:26` (`col_gap = 0.5em`) e `:29` (`row_gap = 0.2em`) —
confirmados no P1053 contra `matrix.rs:15-16` do vanilla, nunca escrito como comentário
no ficheiro.

1. Verificar se algum outro dos avisos da lista corresponde a valores já confirmados em
   passos anteriores desta frente (P1042, P1047, P1050, P1053) — cruzar por ficheiro/
   valor antes de tratar como novo.
2. Para cada um confirmado: copiar a citação já existente (do relatório do passo que a
   confirmou) para um comentário no código, no formato já usado (`// P<NNN> — ... —
   file:line`). Sem investigação nova — é só materializar o que já se sabe.

## Categoria C — Genuinamente por investigar

O resto — `columns.rs`, `cursor.rs` (`0.65` de leading — candidato a ser `par.leading`
legítimo, confirmar contra `par.rs:210` já citado no P1053 como canónico, mesma lógica da
Categoria B se bater), `divider.rs`, `frac.rs:53/131/132`, `attach.rs`, `accent.rs`,
`03_infra/src/font_metrics.rs` (`0.6`, duas ocorrências), `03_infra/src/shaper.rs`,
`export/fonts.rs`/`images.rs`/`stream.rs`.

Para cada um: mesmo processo de sempre — confirmar se devia vir de fonte externa,
medir contra vanilla se for geometria, citar se for legítimo, escalar com gate
`ADR-0127` se revelar comportamento a corrigir.

**Nota**: o padrão `0.65` a escalar `leading` aparece repetido em pelo menos 8 sítios
diferentes (`cursor.rs`, `enum_item.rs`, `list_item.rs`, `sequence.rs`, `sub_frame.rs`,
vários em `tests.rs`) — se for confirmado como `par.leading` legítimo (per P1053), é
**uma** citação a propagar a 8 sítios, não 8 investigações separadas.

## Fase B — Decisão de convenção (equation.rs:334 e casos semelhantes)

Confirmar regra: citação completa na primeira ocorrência de um valor reutilizado; remissão
curta (`// mesmo valor que <ficheiro>:<linha>`) nas ocorrências seguintes do mesmo valor
justificado. Registar esta convenção em `auditar-fatiamento.md` (ou onde a emenda do
P1042 já vive) para não voltar a ser dúvida.

## Fase C — Validar

```
crystalline-lint --checks v21 .
cargo test --workspace
```
Zero regressão — Categorias A/B são só comentário/correcção de caminho, sem mudança de
comportamento. Categoria C segue gate `ADR-0127` onde aplicável.

---

## Resultado esperado

Categoria A fechada com lição sobre fatiamento a propagar para o método. Categoria B
fechada por materialização barata de citações já conhecidas. Categoria C reduzida ao
número real de investigações novas (provavelmente muito menor que o total da lista,
dado o padrão de repetição já visível em `0.65`/leading).
