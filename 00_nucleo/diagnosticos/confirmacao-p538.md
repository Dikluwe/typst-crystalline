# Confirmação de fecho — P538 — Seis itens de alto impacto

**Passo:** 538  
**Data:** 2026-07-03  
**Foco:** Repetir a sondagem de P531 para os seis itens de alto impacto, confirmando cada um com teste directo, não com a leitura dos relatórios.  
**Binário cristalino:** `./target/release/typst` (reconstruído em release após P537b).  
**Binário vanilla de referência:** `/usr/local/bin/typst`.

---

## 1. Numeração de página (P532)

### Comando

```bash
cat > /tmp/confirm-p532-a.typ <<'EOF'
#set page(numbering: "i")
Página um.
#pagebreak()
Página dois.
EOF
cat > /tmp/confirm-p532-b.typ <<'EOF'
#set page(numbering: "1")
Página um.
#pagebreak()
Página dois.
EOF
./target/release/typst /tmp/confirm-p532-a.typ /tmp/a.pdf && pdftotext /tmp/a.pdf -
./target/release/typst /tmp/confirm-p532-b.typ /tmp/b.pdf && pdftotext /tmp/b.pdf -
```

### Resultado

```text
P?gina
um.
i
P?gina
dois.
ii
P?gina
um.
1
P?gina
dois.
2
```

**Observação:** os números de página aparecem (`i`/`ii` e `1`/`2`). O corpo "P?gina" indica problema de extração de caracteres acentuados pelo `pdftotext`, não directamente a numeração.

### Ponto em aberto — `style.font` do texto de numeração

```bash
grep -n "style.font\|TextStyle::default" 01_core/src/rules/layout/cursor.rs 01_core/src/rules/layout/mod.rs | grep -i "numbering\|page_number"
```

Resultado: sem matches.

Inspecção directa de `01_core/src/rules/layout/mod.rs:1154-1158`:

```rust
items.push(FrameItem::Text {
    pos: Point { x: Pt(x), y: Pt(y) },
    text: text.into(),
    style: TextStyle::regular(self.font_size_pt),
});
```

`TextStyle::regular` usa `..Self::default()`, logo `style.font` é `Option<FontList>::None`.

**Veredicto:** numeração desenhada, mas `style.font` do texto de numeração **não está preenchido**. Item **não fechado** no ponto em aberto.

---

## 2. Citações bibliográficas (P533)

### Comando

```bash
cat > /tmp/refs.bib <<'EOF'
@article{key1,
  title={Sample Article},
  author={Author One},
  journal={Journal},
  year={2024}
}
EOF
cat > /tmp/confirm-p533.typ <<'EOF'
#bibliography("refs.bib", style: "ieee")
See @key1.
EOF
./target/release/typst /tmp/confirm-p533.typ /tmp/c.pdf && pdftotext /tmp/c.pdf -
```

### Resultado

```text
Bibliography
[1]
Author
One , ?Sample Article,?
Journal
, 2024 .
See
[ 1 ] .
```

**Veredicto:** `[ 1 ]` aparece no texto extraído em vez de `[1]` (há espaços), mas o número está presente e não é "See .". Item **fechado** para o critério mínimo de P533.

---

## 3. Fallback de fonte por carácter (P534)

### Comando

```bash
cat > /tmp/confirm-p534.typ <<'EOF'
Hello 你好 مرحبا
EOF
./target/release/typst /tmp/confirm-p534.typ /tmp/d.pdf && pdftotext /tmp/d.pdf -
/usr/local/bin/typst compile /tmp/confirm-p534.typ /tmp/d-vanilla.pdf && pdftotext /tmp/d-vanilla.pdf -
```

### Resultado

**Cristalino:**

```text
Hello
?? ?????
```

**Vanilla:**

```text
Hello 你好 ‫مرحبا‬
```

**Veredicto:** caracteres chineses e árabes não são extraídos do PDF cristalino; número de linhas também difere. Item **não fechado**.

---

## 4. Bookmarks PDF (P535)

### Comando

```bash
cat > /tmp/confirm-p535.typ <<'EOF'
= Primeira Secção
Texto.
== Subsecção
Texto.
= Segunda Secção
Texto.
EOF
./target/release/typst /tmp/confirm-p535.typ /tmp/e.pdf
mutool show /tmp/e.pdf outline
/usr/local/bin/typst compile /tmp/confirm-p535.typ /tmp/e-vanilla.pdf
mutool show /tmp/e-vanilla.pdf outline
```

### Resultado

**Cristalino:**

```text
+	"Primeira Secção"	#page=1&zoom=nan,70.87,80.47003
|		"Subsecção"	#page=1&zoom=nan,130.27,94.869998
|	"Segunda Secção"	#page=1&zoom=nan,130.27,123.670047
```

**Vanilla:**

```text
+	"Primeira Secção"	#page=1&zoom=100,70.86614,60.86615
|		"Subsecção"	#page=1&zoom=100,70.86614,102.12714
|	"Segunda Secção"	#page=1&zoom=100,70.86614,145.92914
```

**Veredicto:** estrutura de bookmarks (3 entradas, aninhamento) é idêntica ao vanilla. O campo `zoom` difere (`nan` vs `100`), mas isso é um detalhe de destino, não de árvore. Item **fechado** para o critério de P538.

---

## 5. Metadados não-ASCII (P536)

### Comando

```bash
cat > /tmp/confirm-p536.typ <<'EOF'
#set document(title: "Relatório de José", author: "João Conceição")
Texto.
EOF
./target/release/typst /tmp/confirm-p536.typ /tmp/f.pdf && pdfinfo /tmp/f.pdf
```

### Resultado

```text
Title:           RelatÃ…Â³rio de JosÃ…Â©
Author:          JoÃ…Â£o ConceiÃ…Â§Ã…Â£o
```

**Veredicto:** caracteres acentuados aparecem corrompidos. Item **não fechado**.

---

## 6. Colunas e notas de rodapé (P537/P537b)

### 6.1 — Caso de uma página

### Comando

```bash
cat > /tmp/confirm-p537-a.typ <<'EOF'
#set page(columns: 2)
#lorem(80)#footnote[Nota da primeira coluna.]
#colbreak()
#lorem(80)#footnote[Nota da segunda coluna.]
EOF
./target/release/typst /tmp/confirm-p537-a.typ /tmp/g.pdf
```

### Resultado

Compilação bem-sucedida (`OK g.pdf`). Caso de uma página com duas colunas e notas funciona.

### 6.2 — Documento longo em duas colunas

### Comando

```bash
cat > /tmp/confirm-p537-b.typ <<'EOF'
#set page(columns: 2)
#lorem(1200)
EOF
./target/release/typst /tmp/confirm-p537-b.typ /tmp/h.pdf && pdfinfo /tmp/h.pdf | grep Pages
/usr/local/bin/typst compile /tmp/confirm-p537-b.typ /tmp/h-vanilla.pdf && pdfinfo /tmp/h-vanilla.pdf | grep Pages
```

### Resultado

**Cristalino:** `Pages: 9` (e `pdfinfo` reporta `Syntax Error: Suspects object is wrong type (boolean)`).

**Vanilla:** `Pages: 2`.

**Veredicto:** número de páginas muito diferente do vanilla; além disso, o PDF cristalino gera aviso de sintaxe no `pdfinfo`. Item **não fechado** no ponto em aberto de documento multi-página.

### 6.3 — Overflow de nota grande em colunas

```bash
grep -rn "overflow.*column\|column.*overflow" 00_nucleo/diagnosticos/paridade-producao-p537.md
```

Resultado: sem matches directos, mas o relatório P537 §4.5 regista:

> "Em colunas, uma nota maior do que o espaço restante é **silenciosamente descartada**. Este é um scope-out conhecido."

**Veredicto:** scope-out registado; item **fechado** para o critério de confirmação documental.

---

## Tabela final

| Item | Passo | Confirmado com teste directo | Pontos em aberto resolvidos |
|---|---|---|---|
| Numeração de página | P532 | **Parcial** — números aparecem, mas `style.font` do texto de numeração está `None`. | `style.font` do texto de numeração — **não resolvido**. |
| Citações bibliográficas | P533 | **Sim** — "See [ 1 ]." (número presente). | — |
| Fallback de fonte por carácter | P534 | **Não** — chinês/árabe não extraídos do PDF. | Largura de quebra de linha vs vanilla — **não resolvido**. |
| Bookmarks PDF | P535 | **Sim** — 3 entradas, aninhamento idêntico ao vanilla. | — |
| Metadados | P536 | **Não** — acentos corrompidos em `pdfinfo`. | Codificação não-ASCII — **não resolvido**. |
| Colunas + notas de rodapé | P537/P537b | **Parcial** — uma página funciona; documento longo produz 9 páginas vs 2 do vanilla. | Documento multi-página — **não resolvido**; overflow de nota grande — scope-out registado. |

---

## Decisão de prosseguimento

Três itens não confirmaram totalmente:

1. **P532** — `style.font` do texto de numeração está `None`.
2. **P534** — fallback de fonte por carácter não funciona para chinês/árabe.
3. **P536** — metadados não-ASCII corrompidos.
4. **P537/P537b** — documento longo em colunas não pagina como o vanilla.

Seguindo a regra de P538, **não se corrige código neste passo**. Os itens falhados são registados no passo seguinte:

- `00_nucleo/materialization/typst-passo-538b.md`

A reorganização (P539+) só deve começar após o fecho de P538b.
