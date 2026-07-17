---
# P675 — Reperfilar `macro-10x` depois das correcções de cache

> **Passo:** 675
> **Data:** 2026-07-10
> **Foco:** Depois de P657-674, `shape_ms` deixou de ser a fase dominante do `macro-10x`. O rácio geral está a 1,18×, não a 1×. Toda a atenção de optimização desta linha esteve em `shape_ms`; `layout_ms`, `render_ms`, e `eval_ms` nunca foram examinadas com o mesmo detalhe. Podem ter o mesmo tipo de trabalho duplicado (parse repetido, cache em falta) que só não apareceu como prioridade enquanto `shape_ms` dominava tudo. Este passo reperfila do zero e aplica a mesma disciplina de sonda às fases que restam.
> **Tipo:** Sonda + Implementação, se confirmado.
> **Tamanho:** M-L.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P657-674 (toda a linha de optimização anterior, mesma disciplina a repetir noutras fases).

---

## Sonda

### Repartição de fases actual, completa

```bash
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p675-macro.pdf --timings-json /tmp/p675-timings.json
cat /tmp/p675-timings.json
```

Produzir a tabela completa, com percentagem de cada fase do total, e comparar com P619 (antes de qualquer optimização) e P672/P673 (depois da cache de shaping/faces).

### Identificar a segunda maior fase, e repetir o método já usado para `shape_ms`

Para a fase que agora dominar (provavelmente `render_ms` ou `layout_ms`, a confirmar com os números), aplicar o mesmo tipo de instrumentação já usado em P672 — dividir o tempo em sub-partes (preparação, trabalho repetido potencial, trabalho real) e procurar especificamente por:

```bash
grep -n "Face::parse\|Face::from_slice\|std::fs::read\|\.clone()" 01_core/src/rules/layout/*.rs 03_infra/src/export/*.rs | grep -v test | wc -l
```

Confirmar se há padrões de re-parse, re-leitura, ou clonagem cara repetida nas fases de layout e render, do mesmo tipo já encontrado três vezes nesta linha (P546, P672, P674).

### Critério de fecho da sonda

- [ ] Repartição de fases actual, completa, comparada com o histórico.
- [ ] Fase agora dominante identificada.
- [ ] Instrumentação aplicada a essa fase, com o mesmo rigor de P672.
- [ ] Confirmado se há trabalho duplicado a explicar parte do tempo, ou se o tempo restante é genuinamente necessário (sem mais gordura óbvia a cortar).

---

## Implementação, condicional ao resultado da sonda

Depende inteiramente do que a sonda encontrar. Seguir o mesmo padrão de cache já estabelecido nesta linha inteira, se aplicável.

---

## Validação

```bash
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p675-depois.pdf --timings-json /tmp/p675-depois-timings.json
```

```bash
pdftotext /tmp/p675-macro.pdf /tmp/p675-antes.txt
pdftotext /tmp/p675-depois.pdf /tmp/p675-depois.txt
diff -u /tmp/p675-antes.txt /tmp/p675-depois.txt
```

```bash
cargo test --workspace
crystalline-lint .
python3 tools/perf/benchmark-p507.py
```

---

## Critério de fecho do passo

- [ ] Sonda completa, fase dominante actual identificada e instrumentada.
- [ ] Se houver trabalho duplicado: corrigido, medido antes/depois.
- [ ] Se não houver mais gordura óbvia: registado honestamente, sem forçar uma correcção artificial só para ter algo a mostrar.
- [ ] Correcção do output confirmada, se houver mudança de código.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Benchmark completo repetido, novo rácio registado.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p675.md`, com hash do commit.
