O ambiente de execução (sandbox) está a falhar neste momento — não consigo criar nem gravar ficheiros nem correr comandos. Em vez de continuar a tentar, deixo o conteúdo do passo aqui directamente, para não bloquear.

```markdown
# Passo 1045 — V18 (2 casos) + V16 remanescente (pós-P1041)

**Tipo**: V18 é verificação directa, pequena. V16 remanescente é o que sobrar depois do
P1041 já ter tratado os 8 DENY e os 132 neutros — não presumir o número, confirmar por
lint antes de planear o trabalho.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1044.

---

## Fase 0 — Confirmar quanto de V16 ainda falta

```
crystalline-lint --checks v16 . | wc -l
```

O baseline original era 195 (8 DENY + 132 neutros + resto não classificado no P1041). O
P1041 tratou 8+132=140. Não presumir que sobram 55 — o número pode ter mudado por passos
posteriores (P1042 tocou math/layout/, pode ter introduzido ou removido casos). Confirmar
a contagem real antes de dimensionar o resto deste passo.

## Fase A — V18 (2 casos): padrão de range numérico fora de lexing/numbering

```
crystalline-lint --checks v18 .
```

Para cada uma das 2 ocorrências:
1. Localizar o `match`/guarda com padrão de range (`0..=6`, `'a'..='z'`, etc.) fora dos
   módulos onde isso é esperado (`lexer/`, `stdlib/numbering.rs`).
2. Confirmar se o range está correcto por comparação directa com o vanilla (`.typ` mínimo
   que force os dois extremos do range e um valor fora dele, comparar output).
3. Se o range é só uma forma alternativa de um `match` de valores discretos (ex.: `1..=6`
   em vez de `1 | 2 | 3 | 4 | 5 | 6`), sem problema de domínio — é estilo, não bug. Se o
   range implica limites que não batem com o vanilla (off-by-one, fronteira errada), é
   achado a corrigir com gate `ADR-0127` se mudar comportamento.

## Fase B — V16 remanescente: classificar, não corrigir em bloco

Para os casos que sobrarem da Fase 0 (número real, não presumido):

1. Cada `_ =>` restante: confirmar se é saturação arbitrária (mesma classe dos 8 DENY do
   P1041 — devia ter sido apanhado lá, mas pode ter escapado) ou hub intencional já
   coberto por `[wildcard_exceptions]` mas ainda a disparar por algum motivo (falso
   positivo do linter a confirmar).
2. Aplicar o mesmo princípio do P1041: nenhum `_ => <default>` se resolve por suprimir o
   lint — decisão semântica por caso, com evidência (a/b/c da mesma taxonomia).
3. Se o número remanescente for pequeno (dezenas, não centenas), tratar caso a caso neste
   mesmo passo. Se for grande, dimensionar como passo próprio e reportar aqui só a
   contagem e classificação preliminar, sem forçar tudo numa sessão.

## Fase C — Validar

```
crystalline-lint --checks v16,v18 .
cargo build --workspace --release
cargo test --workspace
```
Zero regressão nos casos que não mudam comportamento. Qualquer correcção que mude output
observável passa por gate `ADR-0127` antes de codificar, mesma disciplina de sempre.

---

## Resultado esperado

V18 fechado (2/2, confirmado contra vanilla). V16 remanescente classificado com a
contagem real (não presumida), tratado neste passo se pequeno, ou dimensionado como
passo próprio se grande — decisão tomada com número real, não com o "195" original que já
está desactualizado.
```
