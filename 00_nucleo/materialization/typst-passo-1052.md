# Passo 1052 — V20: profundidade de padrão > 2 fora de contexto-tabela (515 ocorrências)

**Tipo**: Triagem primeiro, correcção só onde o custo compensar. V20 é nível `info`, não
`warning`/`error` — sinal a investigar, não lista de bugs presumidos. 515 é grande demais
para tratar item a item como V16 Classe A/B; a abordagem tem de ser proporcional ao
volume, mesma disciplina já aplicada ao catálogo de ambiguidade (P1021/1024).
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1051.

---

## Fase 0 — Entender a regra antes de triar

**Não presumir o que "profundidade de padrão > 2" significa** — ler a definição/
rationale real da regra V20 no `tekt-linter` (mesmo procedimento que já se aplicou a
V16/V17/V18/V19: ler o ADR/spec da regra antes de agir sobre os resultados).

Perguntas a responder, com a definição real, não com suposição:
1. "Profundidade de padrão" conta níveis de destructuring aninhado (`Some(Ok(Some(x)))`
   = profundidade 3), ou outra coisa (nº de variantes num único `match`, profundidade de
   `if let` encadeado, etc.)?
2. "Fora de contexto-tabela" — o que conta como contexto-tabela para esta regra
   especificamente? (presumivelmente hubs de despacho exaustivo, tipo os que `ADR-0104`
   já legitima — confirmar se a definição da regra bate com essa noção antes de aplicar).
3. Confirmar contagem real (`crystalline-lint --checks v20 . | wc -l`) — 515 é o número
   antigo, pode ter mudado com os passos entretanto.

## Fase A — Categorizar por módulo, não por caso individual

```
crystalline-lint --checks v20 . | awk -F: '{print $1}' | sort | uniq -c | sort -rn
```

Produzir distribuição por ficheiro/módulo. Não é preciso ler os 515 um a um nesta fase —
é para ver onde se concentram.

## Fase B — Amostra dirigida, não exaustiva

Dado o volume e o nível `info`, escolher amostra por risco, não cobertura total:

1. **Os 10 casos de maior profundidade** (piores primeiro) — mais prováveis de
   esconder complexidade real a simplificar.
2. **Todos os casos em `math/layout/` e `table`/`grid`** — histórico de bugs reais
   nesta frente concentra-se ali (P1026, P1042, P1050). Não presumir que estes são só
   ruído estatístico só porque a maioria dos 515 provavelmente é.
3. Uma amostra aleatória pequena (5-10) do resto, para confirmar que a maioria é mesmo
   inofensiva e não há um padrão sistemático escondido fora das áreas já suspeitas.

Para cada caso da amostra: confirmar se é (a) lógica de domínio genuína, só
profunda por natureza — aceitável, sem acção; (b) candidato a simplificação (extrair
função, achatar com métodos combinadores) — sem gate, é refactor puro se não mudar
comportamento; (c) esconde um caso não tratado ou tratado incorrectamente — **isto é o
que mais importa encontrar**, mesmo padrão de todas as investigações profundas desta
frente (medir antes de decidir, não presumir que "profundo" é sinónimo de "inofensivo").

## Fase C — Decisão de alcance

Com a amostra da Fase B: se nenhum caso revelar bug real, catalogar os 515 como debt de
baixa prioridade (info, não bloqueante), sem tratamento item a item — desproporcional ao
risco medido. Se a amostra revelar padrão de bug real concentrado nalgum módulo, expandir
a investigação só a esse módulo, não aos 515.

## Fase D — Validar (só se algo foi corrigido)

```
crystalline-lint --checks v20 .
cargo test --workspace
```
Qualquer correcção de comportamento (não só refactor de forma) passa por gate
`ADR-0127`, mesma disciplina de sempre.

---

## Resultado esperado

V20 fechado por decisão informada — não pelos 515 casos individualmente, mas por amostra
dirigida que confirma (ou refuta) que o volume é ruído de baixo risco. Se refutar,
escala-se só onde a evidência apontar, não ao total.
