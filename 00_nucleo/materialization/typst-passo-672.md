---
# P672 — Porque é que 99,76% de acerto na cache só reduziu `shape_ms` em 29%?

> **Passo:** 672
> **Data:** 2026-07-10
> **Foco:** P657 mediu um hit ratio de 99,76% na cache de resultados de shaping, mas `shape_ms` só caiu 29% (34,2s → 24,1s). Se quase todas as chamadas acertam na cache, a redução devia aproximar-se muito mais de 99%. A discrepância sugere que há trabalho caro a acontecer **antes** da consulta à cache — possivelmente resolução de fonte, análise de `Face`, ou construção da chave em si — que não é evitado mesmo quando o resultado do shaping já está guardado.
> **Tipo:** Sonda + Implementação, se confirmado.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P657 (onde a cache foi criada e o hit ratio medido, sem se questionar a discrepância), P546/P548 (precedente do mesmo tipo de problema com `Face::from_slice`).

---

## Sonda

### Instrumentar o tempo dentro e fora da consulta à cache

```bash
grep -n "fn try_shape\|ShapeCache\|cache.get\|cache.insert" 03_infra/src/shaper.rs | head -30
```

Adicionar instrumentação temporária (não commitada) que meça separadamente:

1. Tempo a construir a chave da cache (formatar texto, calcular hash de face, etc.).
2. Tempo da consulta em si (`cache.get(&key)`).
3. Tempo do trabalho que acontece **antes** da consulta (resolução de fonte, `Face::from_slice`, ou qualquer outra preparação), se algum acontecer antes de verificar se já está em cache.
4. Tempo do `rustybuzz::shape` real, só quando a cache falha.

```bash
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p672-instrumentado.pdf
```

Correr com a instrumentação e recolher os números.

### Confirmar se `Face::from_slice` (ou equivalente) acontece antes da consulta à cache

Dado que P546 já identificou este exacto padrão (recriar `Face` a cada chamada, sem cache), confirmar se esse trabalho está a acontecer de novo aqui, desta vez à volta da cache de resultado de shaping, não dentro dela.

### Critério de fecho da sonda

- [ ] Tempo dividido nas quatro partes acima, com números reais.
- [ ] Confirmado qual parte explica a discrepância entre 99,76% de acerto e só 29% de redução de tempo.
- [ ] Se for `Face::from_slice` ou resolução de fonte repetida: confirmado que não está a usar a cache de `Face` já estabelecida por P548.

---

## Implementação, condicional ao resultado da sonda

Se confirmado que trabalho caro acontece antes da consulta à cache: mover a consulta à cache para o ponto mais cedo possível, antes de qualquer resolução de fonte ou análise de `Face`, usando só a informação já disponível (texto, identificador de fonte, tamanho, estilo) para construir a chave e verificar a cache primeiro.

### Critério de fecho da implementação

- [ ] Consulta à cache movida para antes do trabalho caro, se confirmado que estava depois.
- [ ] `shape_ms` medido de novo, comparando com os 24,1s de P657.
- [ ] Testado que a correcção não introduz nenhum problema de correcção (mesma disciplina de P657 — `diff` do texto extraído antes/depois).

---

## Validação

```bash
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p672-depois.pdf --timings-json /tmp/p672-timings.json
cat /tmp/p672-timings.json
```

Comparar `shape_ms` com P657 (24 147 ms) e P670/P671 (~30 000 ms, dentro da variação normal).

```bash
pdftotext /tmp/p672-antes.pdf /tmp/p672-antes.txt
pdftotext /tmp/p672-depois.pdf /tmp/p672-depois.txt
diff -u /tmp/p672-antes.txt /tmp/p672-depois.txt
```

```bash
cargo test --workspace
crystalline-lint .
python3 tools/perf/benchmark-p507.py
```

---

## Critério de fecho do passo

- [ ] Sonda completa, tempo dividido em partes, discrepância explicada com números.
- [ ] Se confirmado: correcção implementada, `shape_ms` medido de novo.
- [ ] Correcção do output confirmada (diff idêntico).
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Benchmark completo repetido, novo rácio registado.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p672.md`, com hash do commit.
