---
# P677 — Aplicar a mesma disciplina a `layout_ms`

> **Passo:** 677
> **Data:** 2026-07-10
> **Foco:** `shape_ms` (P657, P672, P673) e `render_ms` (P675) já receberam instrumentação detalhada e revelaram trabalho duplicado real em cada caso. `layout_ms` (~1260ms, a segunda maior fase do `macro-10x` depois da correcção de P675) nunca recebeu o mesmo tratamento. Este passo aplica a mesma disciplina — instrumentar, dividir em sub-partes, procurar por padrões já conhecidos de duplicação (parse repetido, leitura repetida, walk do documento repetido).
> **Tipo:** Sonda + Implementação, se confirmado.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P657-676 (toda a linha de optimização, mesmo método a aplicar aqui).

---

## Sonda

### Confirmar a repartição actual, com `layout_ms` já sem o ruído de amostra pequena

```bash
hyperfine --warmup 5 --runs 30 './target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p677.pdf'
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p677-timings.pdf --timings-json /tmp/p677-timings.json
cat /tmp/p677-timings.json
```

Usar 30 execuções desde já, aplicando a lição de P676 — não tirar conclusões de uma amostra pequena.

### Instrumentar `layout_ms` em sub-partes

```bash
grep -n "fn layout_with_introspector\|fn layout_document\|fn layout_page" 01_core/src/engine/layout/mod.rs | head -10
```

Dividir o tempo de layout em partes prováveis: medição de conteúdo (`measure_content_constrained`, já mencionada em P593), posicionamento, gestão de regiões/páginas, footnotes, e qualquer chamada a `text_width`/`shaped_width` que ainda não esteja completamente coberta pelas caches já criadas.

### Procurar especificamente por padrões já conhecidos

```bash
grep -n "Face::parse\|Face::from_slice\|\.clone()\|collect::<Vec" 01_core/src/engine/layout/*.rs | grep -v test | wc -l
```

Confirmar se algum destes padrões, já causadores de problemas reais três vezes nesta linha, aparece também aqui.

### Critério de fecho da sonda

- [ ] Repartição confirmada com amostra de 30 execuções, não menos.
- [ ] `layout_ms` dividido em sub-partes com instrumentação directa.
- [ ] Confirmado se há trabalho duplicado, e onde exactamente, com `file:line`.

---

## Implementação, condicional ao resultado da sonda

Depende do que a sonda encontrar. Seguir o mesmo padrão de correcção já estabelecido (cache, ou eliminar walk duplicado).

---

## Validação

Aplicar a lição de P676 desde o início — usar sempre `hyperfine --runs 30`, nunca menos, para qualquer comparação de tempo nesta linha de trabalho a partir de agora.

```bash
pdftotext /tmp/p677-antes.pdf /tmp/p677-antes.txt
pdftotext /tmp/p677-depois.pdf /tmp/p677-depois.txt
diff -u /tmp/p677-antes.txt /tmp/p677-depois.txt
```

```bash
cargo test --workspace
crystalline-lint .
python3 tools/perf/benchmark-p507.py
```

Medir também o "tempo fora das fases instrumentadas" (a técnica que P676 usou), para confirmar desde já que qualquer poupança aparece no tempo de parede real, não só no JSON.

---

## Critério de fecho do passo

- [ ] Sonda completa, com amostra de 30 execuções.
- [ ] `layout_ms` instrumentado em sub-partes.
- [ ] Se houver trabalho duplicado: corrigido, medido antes/depois com a mesma disciplina de P676.
- [ ] Se não houver mais gordura óbvia: registado honestamente.
- [ ] Correcção do output confirmada.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Benchmark completo repetido, novo rácio registado, com amostra suficiente.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p677.md`, com hash do commit.
