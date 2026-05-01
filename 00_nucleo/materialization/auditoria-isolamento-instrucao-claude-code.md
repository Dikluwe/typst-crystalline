# Auditoria arquitectural — isolamento cristalino vs vanilla

## Contexto

A meta original do projecto Typst Cristalino é **isolamento
melhor que vanilla**: re-implementação que produz o mesmo
output observable mas com estruturas mais isoladas, mais
testáveis isoladamente, com responsabilidades mais claras.

Análise anterior (`analise-degradacao-metodologica.md`)
identificou que entre 2026-04-25 e 2026-04-26 aconteceu um
bloco de inflação metodológica (P156a–P160a). A inflação
mascarou possíveis derivas arquitecturais — decisões justificadas
por métricas inventadas em vez de pela meta original.

Esta auditoria avalia se as estruturas cristalinas actuais
cumprem a meta. **Não** propõe refactor; **não** classifica
passos como contaminados; **não** cria nova ADR. Apenas mede
e reporta.

---

## O que fazer

Inspeccionar todas as estruturas (structs e enums) em
`01_core/src/entities/` e ficheiros principais em
`01_core/src/rules/`. Para cada uma, comparar com equivalente
em `lab/typst-original/` segundo 4 critérios combinados.

Output: ficheiro markdown único em
`00_nucleo/diagnosticos/auditoria-isolamento-vs-vanilla.md`
com lista plana de cada estrutura classificada em três grupos:
**pior que vanilla**, **igual a vanilla**, **melhor que vanilla**.

---

## Critérios de avaliação

Cada estrutura é avaliada nos 4 critérios. **A classificação
final é a pior das 4** — se uma estrutura é melhor em 3
critérios mas pior em 1, classifica como "pior".

### Critério A — Fan-in/fan-out

**Fan-in**: número de ficheiros que importam ou referem este tipo.
**Fan-out**: número de tipos distintos que este referencia.

Comparar com equivalente vanilla:
- **Melhor** se fan-in e fan-out cristalino ≤ vanilla.
- **Igual** se diferença ≤ 20% em ambos.
- **Pior** se fan-in ou fan-out cristalino > 120% do vanilla.

Mensurável por `grep` + contagem.

### Critério B — Single responsibility

Pergunta literal: **"quantas razões distintas existem para
modificar este tipo?"**

Razão = mudança numa funcionalidade ortogonal. Por exemplo,
`CounterState` muda quando se adiciona feature de lang, quando
se adiciona feature de bibliography, quando se adiciona feature
de figure counting — cada uma é razão distinta.

Avaliar manualmente lendo o ficheiro:
- **Melhor** se cristalino tem ≤ razões que vanilla.
- **Igual** se número de razões é semelhante.
- **Pior** se cristalino acumula mais razões que vanilla.

### Critério C — Testabilidade isolada

Pergunta literal: **"este tipo pode ser testado sem instanciar
outros tipos não relacionados?"**

Inspeccionar tests existentes:
- **Melhor** se tests deste tipo usam apenas dependências
  conceptualmente relacionadas.
- **Igual** se requer dependências comparáveis às do vanilla.
- **Pior** se tests precisam de instanciar estruturas globais
  (como `CounterState` completo) para testar uma feature
  pequena.

### Critério D — Composição vs concentração

Pergunta literal: **"a estrutura é composta de tipos menores
isolados ou concentra tudo internamente?"**

Inspeccionar declaração:
- **Melhor** se cristalino tem composição (campos são tipos
  isolados em ficheiros próprios) onde vanilla concentra.
- **Igual** se estrutura é semelhante.
- **Pior** se cristalino concentra (muitos campos primitivos
  ou tipos genéricos como `HashMap`/`Vec`) onde vanilla isola
  em tipos.

---

## O que inspeccionar

### Lista mínima obrigatória

Inspeccionar pelo menos:

- `01_core/src/entities/content.rs` (variant Content).
- `01_core/src/entities/counter_state.rs` (CounterState).
- `01_core/src/entities/value.rs` (Value).
- `01_core/src/entities/style.rs` ou similar (Styles, StyleChain).
- `01_core/src/entities/bib_entry.rs`, `citation_form.rs`, e
  outros tipos entity adicionados ao longo do projecto.
- `01_core/src/rules/introspect.rs` (pipeline de walk).
- `01_core/src/rules/layout/` (estruturas de layout).
- `01_core/src/rules/stdlib/` (funcs stdlib).

### Lista alargada

Se a inspecção mínima for rápida, alargar a:
- Outros ficheiros em `01_core/src/entities/`.
- Outros ficheiros principais em `01_core/src/rules/`.

Equivalentes vanilla a procurar em `lab/typst-original/crates/`:
- `typst-library/src/foundations/` (Value, Style).
- `typst-library/src/model/` (Content, model elements).
- `typst-library/src/introspection/` (counters, state, etc.).
- `typst-library/src/layout/` (layout primitives).

---

## Output esperado

Ficheiro `00_nucleo/diagnosticos/auditoria-isolamento-vs-vanilla.md`
com 4 secções:

### Secção 1 — Resumo

Tabela com totais:
- Estruturas inspeccionadas: N.
- Pior que vanilla: X (porcentagem).
- Igual a vanilla: Y (porcentagem).
- Melhor que vanilla: Z (porcentagem).

### Secção 2 — Pior que vanilla

Lista de estruturas classificadas como "pior". Por estrutura:
- Nome e ficheiro.
- Critério(s) onde falha.
- Comparação numérica curta com vanilla quando aplicável.
- Linha única de razão (ex: "concentra 14 fields onde vanilla
  isola em 6 tipos").

### Secção 3 — Igual a vanilla

Lista de estruturas classificadas como "igual". Por estrutura:
- Nome e ficheiro.
- Linha única confirmando equivalência.

### Secção 4 — Melhor que vanilla

Lista de estruturas classificadas como "melhor". Por estrutura:
- Nome e ficheiro.
- Critério(s) onde excede.
- Linha única de razão (ex: "isola Sides em ficheiro próprio
  onde vanilla inline em Pad").

---

## Restrições

- **Não escrever ADR** sobre os achados.
- **Não criar reservas** de identificadores futuros.
- **Não propor refactor** ou plano de acção.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo".
- **Não classificar como passo PNNN**: é auditoria, não
  passo de migração.
- **Sem código novo escrito**: trabalho puramente de leitura.

---

## Sobre a comparação com vanilla

Vanilla pode ter padrões que **também** falham em isolamento.
Nesse caso, "igual a vanilla" é classificação válida mas não
é vitória — apenas significa que cristalino não regrediu.

A meta **"melhor que vanilla"** só se cumpre na secção 4. As
outras secções identificam onde isso falhou ou foi neutro.

---

## Critério de conclusão

- Ficheiro `auditoria-isolamento-vs-vanilla.md` produzido.
- 4 secções presentes.
- Lista mínima obrigatória inteiramente inspeccionada.
- Cada estrutura listada com classificação clara nos 4
  critérios.
- Sem ADR nova criada.
- Sem reservas estabelecidas.
- Sem propostas de refactor.

A auditoria é instrumento. Decisões sobre o que fazer com os
resultados ficam para depois.
