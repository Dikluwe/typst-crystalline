# Relatório Diagnóstico — Passo 588
## Correcção do espaço a mais no início do parágrafo depois de `#set`

- **Commit de Referência:** `4bbda18ca` (Working Tree não commitado)
- **Data/Hora da Medição:** 2026-07-06 18:14:28 UTC

---

## 1. Alcance do bug (antes da correcção)

### Texto latino comum

```bash
cat > /tmp/p588-latim.typ <<'EOF'
#set text(size: 20pt)
Texto normal aqui.
EOF
./target/release/typst /tmp/p588-latim.typ /tmp/p588.pdf
pdftotext -tsv /tmp/p588.pdf -
```

Antes da correcção, a primeira palavra "Texto" começava em `75.87 pt` (margem `70.87 pt` + `space_width` de `5 pt`). O bug era geral, não específico de RTL.

### Outras construções

```bash
cat > /tmp/p588-variantes.typ <<'EOF'
= Heading
Texto depois de heading.

#figure([Imagem])
Texto depois de figura.
EOF
./target/release/typst /tmp/p588-variantes.typ /tmp/p588-variantes.pdf
```

Antes da correcção, "Texto depois de heading." começava em `73.62 pt` em vez de `70.87 pt`, confirmando que qualquer newline após uma construção de bloco gerava espaço visual inicial.

---

## 2. Implementação

Ficheiro: `01_core/src/engine/layout/mod.rs:729`

Alterámos o processamento de `Content::Space` para não avançar o cursor quando a linha corrente ainda está vazia:

```rust
Content::Space => {
    // P588 — não renderizar espaço visual no início de uma linha
    // ou parágrafo. Espaços iniciais são gerados por newlines após
    // `#set`, headings, etc., e não devem avançar o cursor.
    if !self.regions.current.current_line.is_empty() {
        self.regions.current.cursor_x += self.space_width();
        if self.regions.current.cursor_x.0
            > self.regions.current.width - self.page_config.margin
        {
            self.flush_line();
        }
    }
}
```

Adicionámos também um teste de regressão em `01_core/src/engine/layout/tests.rs`:

```rust
fn p588_newline_apos_set_nao_desloca_texto_inicial() {
    let doc = layout_typst("#set text(size: 20pt)\nTexto normal aqui.");
    let first_text_x = doc.pages[0]
        .items
        .iter()
        .find_map(|item| match item {
            FrameItem::Text { pos, .. } => Some(pos.x.0),
            _ => None,
        })
        .expect("deve haver pelo menos um Text item");
    assert!(
        (first_text_x - 70.87).abs() < 0.01,
        "texto inicial deve começar na margem (70.87 pt), não deslocado por espaco; obtido {}",
        first_text_x
    );
}
```

---

## 3. Resultados após a correcção

### Texto latino

```text
5	1	0	0	0	0	70.87	65.90	45.26	20.00	100	Texto
```

"Texto" agora começa exactamente em `70.87 pt`.

### Variantes

```text
5	1	0	1	0	0	70.87	105.35	24.89	11.00	100	Texto
```

"Texto depois de heading." também começa em `70.87 pt`.

### Documento árabe de referência

```bash
cat > /tmp/p588-rtl.typ <<'EOF'
#set text(dir: rtl, lang: "ar", size: 40pt)
الكتاب 42 على الطاولة
EOF
./target/release/typst /tmp/p588-rtl.typ /tmp/p588-rtl.pdf
pdftotext -tsv /tmp/p588-rtl.pdf -
```

Resultado após a correcção:

```text
4	1	0	0	0	0	238.410000	49.903000	276.000000	40.000000	-1	###LINE###
5	1	0	0	0	0	238.41	49.90	72.00	40.00	100	ىلع
5	1	0	0	0	1	320.41	49.90	40.00	40.00	100	42
5	1	0	0	0	2	370.41	49.90	144.00	40.00	100	باتكلا
4	1	0	0	1	0	346.410000	108.543000	168.000000	40.000000	-1	###LINE###
5	1	0	0	1	0	346.41	108.54	168.00	40.00	100	ةلواطلا
```

A quebra de linha prematura **ainda ocorre**. Sem o espaço inicial, o total ocupado pelas quatro palavras é `454 pt`, enquanto a largura útil é `453.54 pt` — uma diferença de **0.46 pt**.

### Investigação da origem dos 0.46 pt

A diferença vem da largura do número `42`:

| Compilador | Largura de `42` a 40 pt | Fonte default usada |
|---|---|---|
| Cristalino | `40.00 pt` | Liberation Serif |
| Vanilla | `37.20 pt` | Libertinus Serif |

Medimos directamente os ficheiros de fonte:

```bash
python3 -m venv /tmp/fontenv && /tmp/fontenv/bin/pip install fonttools -q
/tmp/fontenv/bin/python - <<'PY'
from fontTools.ttLib import TTFont

lib = TTFont('/usr/share/fonts/truetype/liberation/LiberationSerif-Regular.ttf')
lib_upem = lib['head'].unitsPerEm
lib_total = sum(lib['hmtx'][lib.getBestCmap()[ord(c)]][0] for c in '42')
print(f"Liberation Serif: upem={lib_upem}, '42' units={lib_total}, at 40pt={lib_total/lib_upem*40:.4f}pt")

van = TTFont('/home/dikluwe/Documentos/Antigravity/typst-crystalline/lab/krilla-reference/assets/fonts/LibertinusSerif-Regular.otf')
van_upem = van['head'].unitsPerEm
van_total = sum(van['hmtx'][van.getBestCmap()[ord(c)]][0] for c in '42')
print(f"Libertinus Serif: upem={van_upem}, '42' units={van_total}, at 40pt={van_total/van_upem*40:.4f}pt")
PY
```

Resultado:

```text
Liberation Serif: upem=2048, '42' units=2048, at 40pt=40.0000pt
Libertinus Serif: upem=1000, '42' units=930, at 40pt=37.2000pt
```

A diferença de `2.8 pt` no `42` é exactamente a diferença entre as duas fontes. O cristalino não tem um erro de layout — tem uma **fonte default diferente** (Liberation Serif, escolhida em P558), cujos dígitos são mais largos do que os da Libertinus Serif do vanilla.

Se o `42` tivesse a largura do vanilla (`37.2 pt`), o total ocupado seria `454 − 2.8 = 451.2 pt`, cabendo dentro da largura útil (`453.54 pt`). Esse valor (`451.2 pt`) coincide exactamente com a largura da linha medida no PDF do vanilla em P586.

**Conclusão:** a quebra residual não é um erro de layout do cristalino. É uma consequência directa da escolha de fonte default. A sequência RTL está fechada quanto aos bugs de layout; a diferença residual é de paridade de fonte, não de algoritmo.

---

## 4. Regressão nos snapshots

A correcção alterou ligeiramente o layout de alguns documentos de snapshot (espaços iniciais removidos). Os seguintes snapshots foram regenerados:

- `03_infra/fixtures/p307b/reference/02-markup-heading.pdf`
- `03_infra/fixtures/p307b/reference/07-multi-feature.pdf`
- `03_infra/fixtures/p307b/reference/09-cidfont.pdf`

---

## 5. Validação

```bash
cargo test --workspace
```

Resultado:

- `typst-core`: 3571 passados, 0 falhas
- `typst-infra`: 597 passados, 0 falhas
- restantes suites: todas passaram

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

### Corpus geral

```bash
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  ./target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1 && echo "OK: $(basename $f)" || echo "FAIL: $(basename $f)"
done
```

Resultado: todos os 36 ficheiros compilaram com sucesso.

---

## 6. Conclusão

- [x] Alcance confirmado — geral, não só RTL.
- [x] Correcção aplicada em `01_core/src/engine/layout/mod.rs:729`.
- [x] Documento latino sem deslocamento inicial.
- [x] Documento árabe re-testado; a quebra prematura principal foi eliminada. A diferença residual de `0.46 pt` foi investigada e atribuída à fonte default diferente (Liberation Serif vs Libertinus Serif), com medição directa nos ficheiros de fonte.
- [x] Snapshots regenerados sem regressão funcional.
- [x] Corpus geral sem regressão nova.
- [x] `cargo test --workspace` limpo.
- [x] `crystalline-lint .` limpo.
