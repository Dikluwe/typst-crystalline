# Passo 1043 — V17: disciplina `testcase()` para os 29 guards compostos

**Tipo**: Não é instrumentação de cobertura (essa via foi fechada como não-viável, per a
pesquisa de MC/DC — P1040 e ficheiro `mcdc_ecossistema.md`). É a alternativa recomendada:
para cada guarda booleana composta (`&&`/`||`), escrever testes que exercitam **cada
condição atómica nos dois sentidos**, de forma que uma mudança de qualquer condição
isolada mude o resultado de pelo menos um teste — a essência do MC/DC, sem instrumentação
formal, ao estilo SQLite (`testcase()`), usando só `cargo llvm-cov --branch` para
confirmar cobertura de branch normal no fim.
**Base**: `crystalline-lint --checks v17` — 29 ocorrências, nível `warning`.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1042.

---

## Lição a aplicar desde já (não redescobrir)

Do protótipo caseiro (P1038): para N condições atómicas independentes numa decisão, o
mínimo de casos para cobrir todos os pares independentes é **N+1**. Uma suite "casual"
(escrita para comportamento, não para isolar condição) tipicamente cobre muito menos —
medido no protótipo: 1/3 com testes casuais vs 3/3 com 4 = N+1 vectores desenhados.

Do achado do P1029/pesquisa: `match` com múltiplos braços (incluindo guards) não é
coberto pelo mesmo raciocínio de decisão booleana — para esses, a exigência é cobertura
de **todos os braços**, não pares de independência (o compilador Rust já força
exaustividade; a exigência aqui é só garantir que cada braço tem pelo menos um teste que
o exercite especificamente, não só que a compilação não falha).

## Fase A — Listar as 29 ocorrências, classificar por número de condições

```
crystalline-lint --checks v17 . > /tmp/v17-lista.txt
```

Para cada ocorrência: contar condições atómicas na guarda (`&&`/`||`), calcular N+1.
Ordenar por N decrescente — as guardas com mais condições são as que mais beneficiam
(mais lacunas prováveis na suite actual, per o padrão já visto nos 5 nós do P1038).

## Fase B — Para cada ocorrência, confirmar cobertura actual antes de escrever testes novos

Não presumir que a suite actual não cobre nada — verificar primeiro:
```
cargo llvm-cov --branch --workspace
```
Localizar a guarda específica no relatório. Se já está coberta nos dois sentidos de cada
condição pela suite existente (pode acontecer, per o P1038 mostrar 1/3 já coberto sem
esforço dedicado): registar como já resolvida, não escrever teste redundante.

## Fase C — Escrever os testes em falta, um por par de independência não coberto

Para cada condição não isolada pela suite actual: escrever (ou completar) um teste que
mude só essa condição, mantendo as outras fixas, e confirme que o resultado muda. Nome de
teste no padrão já estabelecido nesta frente (`p1043_<contexto>_<condicao>_isolada` ou
equivalente descritivo).

**Não é preciso instrumentação nem ferramenta nova** — são testes normais, `#[test]`,
que happen a ser desenhados com este objectivo específico em mente.

## Fase D — Validar

```
cargo llvm-cov --branch --workspace
cargo test --workspace
crystalline-lint --checks v17 .
```
Confirmar aumento de cobertura de branch nas guardas tocadas. V17 continua `warning`
(per decisão do P1041, promoção a `error` fica para worklist própria, não este passo).

## O que este passo NÃO faz

- Não introduz nenhuma ferramenta de instrumentação MC/DC (via fechada).
- Não promove V17 a `error` — só reduz a contagem de ocorrências ao resolver a lacuna de
  teste que cada uma representa.
- Não altera comportamento de produção — são testes novos, não mudança de lógica. Se
  algum teste **falhar** ao ser escrito (revelar bug real, mesmo padrão já visto várias
  vezes nesta conversa), tratar como achado — não corrigir inline, escalar per o processo
  normal (gate ADR-0127 se mudar comportamento).

---

## Resultado esperado

29 guardas compostas com cobertura de par de independência confirmada ou completada,
disciplina SQLite aplicada de facto, não só recomendada. Qualquer bug real encontrado
pelo caminho, catalogado como achado próprio, não corrigido às pressas.
