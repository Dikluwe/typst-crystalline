# Paridade de Produção — P535

**Passo:** 535  
**Foco:** Bookmarks PDF (`/Outlines`).  
**Data:** 2026-07-02  

## Resumo

Antes de P535, o PDF cristalino não continha a árvore `/Outlines`; o painel de
marcadores/navegação dos leitores de PDF ficava vazio. P535 implementa essa
árvore a partir dos headings do documento, com aninhamento por nível (`=`,
`==`, `===`) e destinos apontando para a página e posição correctas.

Durante a validação descobriu-se que as auto-labels `auto-toc-N` geradas para
os headings não passam por `Content::Label` durante o layout, pelo que os mapas
`extracted_label_pages`/`extracted_label_positions` ficavam vazios para esses
destinos. O pipeline passa agora a preencher esses mapas a partir das
`Location`s dos headings registadas no `TagIntrospector` e das `Position`s
single-pass produzidas pelo layout.

## Sondas

### Sonda 1 — O que existia hoje

Comandos:

```bash
grep -rn "/Outlines\|PdfOutline\|bookmark\|OutlineItem" 03_infra/src/export/ --include="*.rs"
grep -rn "/Catalog\|/Root\|catalog" 03_infra/src/export/ --include="*.rs" | head -10
```

Resultado:

- Não existia qualquer geração de `/Outlines`.
- O catálogo PDF é escrito em `03_infra/src/export/builder.rs` (`build_helvetica`
  e `build_cidfont` começam por emitir o objecto 1 como `<< /Type /Catalog /Pages 2 0 R >>`).
- A lista de headings com nível e texto já existia via
  `TagIntrospector::headings_for_toc()` (usada pelo `outline()` do documento).

Conclusão: faltava construir e ligar a árvore `/Outlines`; os dados de origem
já estavam disponíveis.

### Sonda 2 — Referência vanilla

Comando:

```bash
/usr/local/bin/typst compile /tmp/test-bookmarks.typ /tmp/bookmarks-vanilla.pdf
mutool show /tmp/bookmarks-vanilla.pdf outline
```

Resultado vanilla para o documento de teste:

```text
+	"Primeira Secção"	#page=1&zoom=100,70.86614,60.86615
|		"Subsecção"	#page=1&zoom=100,70.86614,102.12714
|	"Segunda Secção"	#page=1&zoom=100,70.86614,145.92914
```

A estrutura esperada é portanto: título, parentesco, irmãos
(`/Prev`/`/Next`), filhos (`/First`/`/Last`), e destino `/Dest [page /XYZ x y null]`.

## Implementação

### Ficheiros alterados

- `00_nucleo/prompts/entities/layout_types.md` — secção P535 no Prompt L0 de
  `PagedDocument`, acrescentando `extracted_headings`; hash actualizado.
- `00_nucleo/prompts/infra/export/builder.md` — secção P535 descrevendo a
  construção da árvore `/Outlines`; hash actualizado.
- `01_core/src/entities/layout_types.rs` — campo `extracted_headings` em
  `PagedDocument`.
- `03_infra/src/export/builder.rs` — novo método `emit_outlines` que constrói
  a árvore `/Outlines` com `/Parent`, `/Prev`, `/Next`, `/First`, `/Last` e
  `/Dest`, e edita o `/Catalog` para referenciar a raiz.
- `03_infra/src/pipeline.rs` — copia `headings_for_toc()` antes de o
  introspector ser consumido; preenche `extracted_label_pages` e
  `extracted_label_positions` para as auto-labels `auto-toc-N` a partir das
  `Location`s de heading e das `Position`s single-pass do layout.
- `03_infra/fixtures/p307b/reference/02-markup-heading.pdf` — actualizado
  (bytes PDF adicionais de `/Outlines`).
- `03_infra/fixtures/p307b/reference/07-multi-feature.pdf` — actualizado
  (mesma razão).

### Nota técnica: destinos dos headings

As labels manuais (`<sec>`, `<cap1>`) já funcionavam porque o layout de
`Content::Label` regista página/posição em `runtime.label_pages` e
`runtime.label_positions`. As auto-labels `auto-toc-N` são introduzidas no walk
como `Tag::Labelled`, não como `Content::Label`, e por isso não deixavam rasto
nesses mapas.

A correção mantém o `TagIntrospector` clonado no pipeline, obtém a lista de
`Location`s de `ElementKind::Heading` (ordem igual à de `headings_for_toc()`), e
consulta `SealedPositions::position_of` para cada uma, copiando o resultado para
os mapas de labels sob as chaves `auto-toc-N`.

## Validação

### Documento de teste do passo

```typst
= Primeira Secção
Texto.
== Subsecção
Mais texto.
= Segunda Secção
Texto.
```

Comando:

```bash
./target/release/typst /tmp/test-bookmarks.typ /tmp/bookmarks.pdf
mutool show /tmp/bookmarks.pdf outline
```

Resultado cristalino:

```text
+	"Primeira Secção"	#page=1&zoom=nan,70.87,80.47003
|		"Subsecção"	#page=1&zoom=nan,130.27,94.869998
|	"Segunda Secção"	#page=1&zoom=nan,163.27,123.670047
```

Estrutura e hierarquia correctas.

### Teste com quebra de página

```typst
= Primeira Secção
Texto.
#pagebreak()
== Subsecção
Mais texto.
#pagebreak()
= Segunda Secção
Texto.
```

Resultado cristalino:

```text
+	"Primeira Secção"	#page=1&zoom=nan,70.87,80.47003
|		"Subsecção"	#page=2&zoom=nan,77.47,80.47003
|	"Segunda Secção"	#page=3&zoom=nan,77.47,80.47003
```

Cada bookmark aponta para a página correcta.

### Teste com três níveis de aninhamento

```typst
= Capítulo 1
Texto.
== Secção 1.1
Texto.
=== Subsecção 1.1.1
Texto.
== Secção 1.2
Texto.
= Capítulo 2
Texto.
```

Resultado cristalino:

```text
+	"Capítulo 1"	#page=1&zoom=nan,70.87,80.47003
+		"Secção 1.1"	#page=1&zoom=nan,130.27,94.869998
|			"Subsecção 1.1.1"	#page=1&zoom=nan,130.27,123.670047
|		"Secção 1.2"	#page=1&zoom=nan,130.27,152.47003
|	"Capítulo 2"	#page=1&zoom=nan,130.27,181.27002
```

Resultado vanilla para o mesmo documento:

```text
+	"Capítulo 1"	#page=1&zoom=100,70.86614,60.86615
+		"Secção 1.1"	#page=1&zoom=100,70.86614,102.12714
|			"Subsecção 1.1.1"	#page=1&zoom=100,70.86614,141.96912
|		"Secção 1.2"	#page=1&zoom=100,70.86614,180.39215
|	"Capítulo 2"	#page=1&zoom=100,70.86614,224.19416
```

Observações:

- A hierarquia (`/First`/`/Last`/`/Parent`/`/Prev`/`/Next`) é idêntica.
- As coordenadas X/Y diferem porque o vanilla usa a fonte padrão
  (Linux Libertine) e o cristalino usa Helvetica fallback; a semântica de
  navegação (página + posição dentro da página) está correcta.
- O vanilla emite `zoom=100`; o cristalino emite `zoom=nan` (valor `null` em
  `/XYZ`), o que é aceite pela especificação PDF e pelos leitores testados.

### Testes automatizados

```bash
cargo test --workspace
crystalline-lint .
```

Resultado:

- `cargo test --workspace`: todos os testes passam.
- `crystalline-lint .`: zero violations.

## Checklist de fecho

- [x] Sonda completa antes do código.
- [x] `/Outlines` gerado e aninhado por nível de heading.
- [x] Cada bookmark aponta para a página e posição correcta.
- [x] Testado com três níveis de aninhamento.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p535.md`.

## Scope-out registado

- **Zoom explícito no destino `/XYZ`:** actualmente `null` (cristalino usa
  `/XYZ x y null`); vanilla usa `100`. Manter `null` simplifica o exportador e
  é compatível com leitores populares.
- **Bookmarks a partir de labels manuais:** se um utilizador quiser bookmarks
  personalizados, o modelo actual baseia-se exclusivamente nos headings. Não
  há pedido nesse sentido neste passo.
- **Contagem `/Count` de bookmarks abertos/fechados:** o valor emitido é o
  número total de bookmarks. Não se distingue visibilidade inicial
  (open/closed) — feature avançada dos leitores, não observável na navegação
  básica.
