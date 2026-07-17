---
# P658 — Estender a cache de P657 a `shaped_width`

> **Passo:** 658
> **Data:** 2026-07-09
> **Foco:** P657 criou uma cache de resultados de shaping para `shape_document` (a passagem final de desenho), com ganho real medido. `shaped_width` (P591), usada durante a decisão de quebra de linha para árabe/devanágari, continua a chamar o shaper sem cache nenhuma — um caminho diferente, sem partilhar o trabalho já feito. Este passo confirma se vale a pena estender, e implementa se sim.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P657 (cache criada, infra-estrutura disponível), P591 (`shaped_width`, o caminho ainda sem cache).

---

## Sonda

### Confirmar se há repetição relevante em documentos com scripts contextuais

Testar com um documento que combine repetição (como `macro-10x`) com texto árabe/devanágari, para medir se `shaped_width` é chamada de forma repetida no mesmo documento:

```bash
python3 -c "
for i in range(500):
    print('الكتاب على الطاولة يوم جميل')
" > /tmp/p658-arabe-repetido.typ
sed -i '1i #set text(lang: \"ar\", dir: rtl, font: \"DejaVu Sans\", size: 20pt)' /tmp/p658-arabe-repetido.typ
./target/release/typst /tmp/p658-arabe-repetido.typ /tmp/p658-baseline.pdf --timings-json /tmp/p658-baseline-timings.json
cat /tmp/p658-baseline-timings.json
```

Confirmar a fase que domina o tempo (provavelmente `layout_ms`, já que `shaped_width` é chamada durante o layout, não durante `shape_document`).

### Confirmar se a chave da cache de P657 pode ser reutilizada directamente

```bash
grep -n "struct ShapeCache\|fn shaped_width" 03_infra/src/shaper.rs 01_core/src/rules/layout/metrics.rs
```

Confirmar se `shaped_width` tem acesso à mesma `ShapeCache` criada por P657 (passada ao `Layouter` desde o início da compilação), ou se precisaria de outro mecanismo de partilha.

### Critério de fecho da sonda

- [ ] Confirmado, com números, se há repetição relevante que justifique a extensão.
- [ ] Confirmado se a cache de P657 pode ser partilhada directamente, ou se precisa de adaptação.

---

## Implementação, condicional ao resultado da sonda

Se confirmado o ganho: `shaped_width` passa a consultar a mesma `ShapeCache`, com a mesma chave (texto, face, direcção, variações, tracking) já estabelecida por P657, antes de chamar o shaper de novo.

### Critério de fecho da implementação

- [ ] `shaped_width` consulta a cache antes de chamar o shaper.
- [ ] Testado com o documento árabe repetido, medindo `layout_ms` antes e depois.
- [ ] Testado que a decisão de quebra de linha continua correcta (reaproveitando os testes já existentes de P590/591/592, sem regressão).

---

## Validação

```bash
./target/release/typst /tmp/p658-arabe-repetido.typ /tmp/p658-depois.pdf --timings-json /tmp/p658-depois-timings.json
```

Comparar `layout_ms` antes e depois.

```bash
cargo test --workspace
crystalline-lint .
python3 tools/perf/benchmark-p507.py
```

---

## Critério de fecho do passo

- [ ] Sonda completa, com números de repetição e viabilidade de partilha da cache.
- [ ] Se confirmado ganho: implementado e medido.
- [ ] Testes de RTL (P590-592) sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p658.md`, com hash do commit.
