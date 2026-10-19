---
# P657 — Porque é que `shape_ms` domina, e há cache de shaping em falta?

> **Passo:** 657
> **Data:** 2026-07-09
> **Foco:** P619 confirmou que `shape_ms` explica 90,66% do tempo de compilação do `macro-10x` (32 577 ms). P548 já resolveu um problema de cache parecido para `layout_ms` (métricas de fonte re-analisadas em cada chamada, corrigido com uma cache de `Face`). `shape_ms` nunca recebeu o mesmo tratamento — a pergunta "há chamadas repetidas ao shaper para o mesmo texto/fonte/tamanho, sem cache de resultado?" nunca foi feita a sério. O nome do documento (`macro-10x`) sugere conteúdo repetido, o que tornaria esta hipótese particularmente promissora.
> **Tipo:** Sonda + Implementação, se confirmado.
> **Tamanho:** M–L.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P548 (precedente do mesmo tipo de correcção, para `layout_ms`), P619 (onde `shape_ms` foi confirmado como dominante, sem investigação da causa interna).

---

## Sonda

### Confirmar se o documento `macro-10x` tem conteúdo repetido

```bash
wc -l tools/perf/corpus/macro-10x.typ
sort tools/perf/corpus/macro-10x.typ | uniq -c | sort -rn | head -20
```

Confirmar quantas linhas/blocos de texto se repetem literalmente, e com que frequência.

### Confirmar se o shaper já tem alguma forma de cache de resultado

```bash
grep -n "fn shape\|cache\|HashMap\|memoiz" 03_infra/src/shaper.rs | head -30
```

Distinguir entre: cache de métricas de fonte (já corrigido por P548), e cache do **resultado do shaping em si** (glifos, posições, para um texto+fonte+tamanho+estilo específico) — são coisas diferentes. Confirmar se a segunda existe.

### Medir directamente quantas vezes o mesmo texto é shapeado

Instrumentar temporariamente (ou usar um contador) para confirmar, no documento `macro-10x`, quantas chamadas ao shaper existem no total, e quantas delas são para exactamente o mesmo texto+fonte+tamanho+estilo já visto antes.

```bash
grep -n "fn shape_text\|pub fn shape" 03_infra/src/shaper.rs
```

Adicionar instrumentação temporária (contador incrementado a cada chamada, e um `HashSet` das chaves já vistas) para medir a taxa de repetição, sem alterar o comportamento.

### Critério de fecho da sonda

- [ ] Confirmado se `macro-10x` tem conteúdo repetido, e em que proporção.
- [ ] Confirmado se existe cache de resultado de shaping hoje, ou só cache de métricas de fonte.
- [ ] Medida a taxa real de chamadas repetidas ao shaper para exactamente o mesmo texto+fonte+tamanho+estilo.
- [ ] Estimativa do ganho potencial de uma cache de shaping, baseada nesta medição, não em suposição.

---

## Implementação, condicional ao resultado da sonda

Se a sonda confirmar uma taxa alta de repetição sem cache: adicionar uma cache de resultado de shaping, chaveada por (texto, fonte, tamanho, estilo, direcção, script), reaproveitando os cuidados já estabelecidos em P591 (cache de `advance_shaped`, que já existe parcialmente) — confirmar se essa cache já existente pode ser estendida, ou se é uma cache diferente, a um nível diferente do pipeline.

### Critério de fecho da implementação

- [ ] Cache de resultado de shaping implementada, se confirmado o ganho.
- [ ] Testada com `macro-10x`, medindo `shape_ms` antes e depois.
- [ ] Testada com documentos sem repetição (para confirmar que não há regressão de desempenho nem de memória nesse caso).
- [ ] Invalidação de cache correcta — confirmar que mudar a fonte, tamanho, ou estilo a meio do documento não usa por engano um resultado da cache que já não se aplica.

---

## Validação

```bash
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p657-depois.pdf --timings-json /tmp/p657-timings.json
cat /tmp/p657-timings.json
```

Comparar `shape_ms` com o valor de referência de P619 (32 577,36 ms).

```bash
cargo test --workspace
crystalline-lint .
python3 tools/perf/benchmark-p507.py
```

Repetir o benchmark completo (já corrigido para terminar dentro do tempo desde P594), confirmando o novo rácio face ao vanilla.

---

## Critério de fecho do passo

- [ ] Sonda completa, com números reais de repetição e cache existente/ausente.
- [ ] Se confirmado ganho: cache implementada, testada, com invalidação correcta.
- [ ] `shape_ms` medido antes e depois, com números.
- [ ] Benchmark completo corrido de novo, novo rácio registado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p657.md`, com hash do commit.
