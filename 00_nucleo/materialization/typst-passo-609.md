---
# P609 — As fontes de fallback são embutidas inteiras, sem subsetting?

> **Passo:** 609
> **Data:** 2026-07-05
> **Foco:** P608 mediu 15,7 MB para um documento de uma linha de texto com fallback para CJK e árabe, atribuindo isto a "subsetting, scope-out já conhecido, ver P515/P519". Mas P515 confirmou o subsetting a funcionar (559KB→30KB), e P543 corrigiu o mecanismo de fallback. As duas coisas juntas não explicam 15,7 MB para uma linha. A hipótese a confirmar: o subsetting aplica-se à fonte pedida explicitamente, mas não às fontes de fallback (CJK, árabe), que podem estar a ser embutidas inteiras.
> **Tipo:** Sonda directa.
> **Tamanho:** M, a confirmar depois da sonda.
> **ADR-0108 EM VIGOR.** Uma explicação que cita um passo anterior sem confirmar se a situação é mesmo a mesma não é uma explicação confirmada.

---

## Sonda

### Confirmar o tamanho de cada fonte embutida individualmente

```bash
mutool extract /tmp/p608.pdf
ls -la font-*
```

Para cada ficheiro de fonte extraído, confirmar:
1. Que fonte é (nome, via `fc-scan` ou `fontTools`).
2. Se corresponde à fonte pedida explicitamente no documento, ou a uma fonte de fallback (CJK, árabe).
3. O tamanho do ficheiro extraído.

```bash
for f in font-*; do
  echo "=== $f ==="
  python3 -c "
from fontTools.ttLib import TTFont
try:
    font = TTFont('$f')
    name = font['name'].getDebugName(1)
    num_glyphs = font['maxp'].numGlyphs
    print(f'Nome: {name}, Glifos: {num_glyphs}')
except Exception as e:
    print('Erro:', e)
"
done
```

### Comparar número de glifos incluídos com número de caracteres usados

O documento de teste usa "你好" (2 caracteres CJK) e "مرحبا" (5 caracteres árabes, mais formas contextuais). Se a fonte CJK extraída tiver milhares de glifos (uma fonte CJK completa tem tipicamente 20 000 a 65 000 glifos), confirma que não houve subsetting nessa fonte — só 2 ou 3 glifos eram necessários.

### Localizar no código onde a diferença entre fonte pedida e fonte de fallback é tratada

```bash
grep -n "subset\|fallback" 03_infra/src/export/builder.rs 03_infra/src/export/fonts.rs 03_infra/src/export/subset.rs 2>/dev/null | head -30
```

Confirmar se o caminho de código que decide quais glifos subsetar recebe a lista de caracteres usados de todas as fontes envolvidas (incluindo as de fallback), ou só da fonte primária pedida no documento.

### Critério de fecho da sonda

- [x] Tamanho e número de glifos de cada fonte embutida confirmado individualmente.
- [x] Confirmado se as fontes de fallback (CJK, árabe) estão subsetadas ou completas.
- [x] Localizado, com `file:line`, onde a lista de caracteres a subsetar é construída, e se inclui ou não os caracteres vindos de fontes de fallback.

---

## Implementação

Depende inteiramente da sonda. Se confirmado que fontes de fallback não são subsetadas: estender o mecanismo de subsetting já existente (confirmado a funcionar em P515) para incluir também os caracteres resolvidos por fallback, não só os da fonte primária.

### Critério de fecho da implementação

- [x] Fontes de fallback subsetadas da mesma forma que a fonte primária.
- [x] Documento de teste de P608 (`Hello 你好 مرحبا world...`) produz um ficheiro de tamanho comparável à soma das fontes subsetadas — não os 15,7 MB medidos.
- [x] Testado com mais scripts de fallback (hebraico, devanágari, etc.), não só CJK e árabe.

---

## Validação

```bash
./target/release/typst /tmp/p608-fallback.typ /tmp/p609-depois.pdf
ls -la /tmp/p609-depois.pdf
mutool extract /tmp/p609-depois.pdf
ls -la font-*
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [x] Causa confirmada — fontes de fallback `.ttc` embutidas inteiras por falha silenciosa do subsetter.
- [x] Subsetting estendido às fontes de fallback `.ttc` via extracção da face individual em `FontSlot::get()`.
- [x] Tamanho de ficheiro para o documento de teste medido antes e depois, com números.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p609.md`, com hash do commit.
- [x] Corrigir a entrada de P608 na lista de disparidades, que atribuiu isto a um scope-out já confirmado sem essa confirmação ter sido feita de facto.
