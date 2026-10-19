# P781 — Suporte a `#image()` com fonte PDF (`image::pdf`)

> **Passo:** 781
> **Data:** 2026-07-17
> **Commit-base:** `6636c5ea62aeffe07e3ef027358fe95e4e1e451b` — working tree com
> P780 ainda não commitado no início deste passo (ver
> `paridade-producao-p780.md`).
> **Dependências:** P772w (achado, confirmação de escopo), P772k/P772p
> (metodologia de erro explícito para formato não suportado, reutilizada aqui).

---

## Sonda — mecanismo exacto do vanilla

### Dependências e arquitectura

`lab/typst-original/crates/typst-library/src/visualize/image/pdf.rs` —
`PdfDocument`/`PdfImage` embrulham `hayro_syntax::Pdf` (parsing) +
`hayro_syntax::page::Page`. `Cargo.toml.original:65-67`:

```
hayro       = { version = "0.7.1", default-features = false }
hayro-svg   = { version = "0.7.0", default-features = false }
hayro-syntax = "0.7.2"
```

`typst-pdf/src/image.rs:73-76` — o embutimento real:

```rust
ImageKind::Pdf(pdf) => {
    if let Some(size) = size.to_krilla() {
        surface.draw_pdf_page(&convert_pdf(pdf), size, pdf.page_index());
    }
}
```

`surface.draw_pdf_page` é uma API do **`krilla`** (o escritor de PDF que o
vanilla usa para exportação) — escreve a página do PDF fonte como **Form
XObject** dentro do PDF exportado, reutilizando o grafo de objectos/recursos
da página original directamente (embutimento vectorial nativo, não
rasterização).

### Peso real da dependência (medido, não assumido)

Sonda directa: `cargo add hayro-syntax --manifest-path 03_infra/Cargo.toml`
(dry-run e depois real) confirma que o registo crates.io **está acessível**
neste ambiente (via `cargo`, apesar de `curl https://crates.io` directo dar
403 — proxy/protocolo específico). `hayro-syntax` sozinho compila limpo como
dependência nova.

Mas `hayro-syntax` só faz **parsing** (metadata: nº de páginas, dimensões). O
que `#image()` precisa de facto — **embutir** a página — exige `hayro`
(interpretação/render). `cargo add hayro --manifest-path 03_infra/Cargo.toml`
(depois revertido) mostra a árvore real:

```
hayro v0.7.1
├── hayro-interpret v0.7.0
│   ├── hayro-cmap v0.1.0
│   │   └── hayro-postscript v0.1.0
│   ├── hayro-syntax v0.7.2
│   │   ├── hayro-ccitt v0.3.0
│   │   ├── hayro-jbig2 v0.3.0
│   │   ├── hayro-jpeg2000 v0.3.5
│   │   └── zune-jpeg v0.5.15
│   ├── kurbo v0.13.1
+ vello_common v0.0.8/v0.0.9, vello_cpu v0.0.8, skrifa v0.42.1,
  read-fonts v0.39.2, pic-scale v0.7.10, pxfm v0.1.30, phf* v0.13.1, png v0.18.1, ...
```

`vello_common`/`vello_cpu` (Linebender) é um motor de renderização
vectorial CPU/GPU completo — não uma dependência leve. `skrifa`/`read-fonts`
duplicam parte do parsing de fontes que o cristalino já tem (rota própria,
sem depender de `hayro`). ~15 crates transitivas novas só para a parte de
*render*, sem contar o que ainda faltaria para *escrever* a página como Form
XObject no exportador PDF do cristalino.

### Estado actual do cristalino (antes deste passo)

```bash
grep -n "detect_image_format\|ImageFormat" 01_core/src/entities/image_format.rs
```

`ImageFormat` só tem `Jpeg | Png | Unknown` — sem variante `Pdf`. Testado
directamente: `#image("ficheiro.pdf")` com um PDF real válido (gerado pelo
próprio `lab/typst-original`) já produzia `error: unknown image format`, exit
1 — **já era um erro explícito, não omissão silenciosa** (confirma que a
correcção de P772p já cobria este caso, mesmo sem um braço dedicado). Vanilla,
com o mesmo ficheiro, compila com sucesso (exit 0) e embute a página.

---

## Decisão de âmbito

**Cenário confirmado**: dependência pesada. Ainda que `hayro-syntax`
(parsing puro) seja leve e o registo crates.io esteja acessível, a
capacidade que `#image()` precisa de facto — **embutir** a página, não só
ler metadata — exige o motor `hayro` completo (interpretação + `vello`,
~15 crates transitivas) **e** uma capacidade de escrita "Form XObject" no
exportador PDF do cristalino, que é hand-rolled (`03_infra/src/export/`,
sem `krilla`) e não tem hoje nenhum equivalente a
`surface.draw_pdf_page`. Implementar exigiria **ou** (a) construir do zero
a escrita de Form XObject (reler o grafo de objectos do PDF fonte, resolver
recursos/fontes/cores, renumerar referências cruzadas no nosso próprio
escritor) **ou** (b) rasterizar a página via `hayro`/`vello` e embutir como
imagem raster (mais tratável — reaproveita o pipeline de imagem raster já
existente — mas ainda exige a árvore completa de `hayro`, e diverge do
mecanismo vectorial do vanilla; ADR-0107 permite essa divergência de
mecanismo, mas a dependência continua pesada).

**Decisão**: **não implementar neste passo.** Scope-out consciente, registado
com o custo medido (não uma estimativa). Precedente directo: P772k/P772p já
tomaram a mesma decisão para SVG (decodificação própria também é um motor de
renderização inteiro) — este passo aplica o mesmo padrão a PDF.

---

## Implementação — melhoria da mensagem de erro (âmbito pequeno, dentro do padrão já estabelecido)

O critério de fecho do passo, para o ramo "não implementado", pede para
confirmar que o gap já produz um erro explícito (sim, via `detect_image_format
== Unknown` → `"unknown image format"`) — mas essa mensagem é genérica,
partilhada com qualquer assinatura binária não reconhecida, menos específica
que o padrão já usado para SVG (`"SVG images are not supported yet"`, via
checagem de extensão dedicada, `figure_image.rs`, P772p).

Adicionado braço dedicado para `.pdf` (case-insensitive), mesmo padrão do
SVG, em `native_image` (`01_core/src/engine/stdlib/figure_image.rs`):

```rust
if lower_path.ends_with(".pdf") {
    return Err(vec![SourceDiagnostic::error(
        args.span,
        "PDF images are not supported yet".to_string(),
    )]);
}
```

Precede a checagem de `detect_image_format` (mesma ordem do SVG) — a
extensão sozinha basta, sem olhar o conteúdo. L0 (`stdlib/figure_image.md`)
actualizado antes do código, com a decisão e o custo medido documentados.
1 teste novo (`native_image_pdf_gera_erro_nao_suportado`), mirror exacto do
teste SVG existente.

---

## Validação

```bash
#image("ficheiro.pdf")   # PDF real, válido (gerado pelo lab/typst-original)
```

- Vanilla: exit 0, página embutida (confirmado por sonda).
- Cristalino, antes deste passo: `error: unknown image format`.
- Cristalino, depois deste passo: `error: PDF images are not supported yet`
  (mais específico; mesmo padrão do SVG).

```
cargo build --workspace --release   → 0 erros
cargo test --workspace --release    → 4253+647+33+2+29+2 = 4967 passed, 0 failed
crystalline-lint . --fix-hashes     → 1 ficheiro re-hashed (figure_image.rs)
crystalline-lint .                  → 0 drift (só V7 pré-existente, não relacionado)
```

Sonda de dependência (`cargo add hayro-syntax`/`hayro`) foi **revertida**
por completo (`git checkout -- 03_infra/Cargo.toml Cargo.lock`) — nenhuma
dependência nova permanece no workspace; a medição de peso ficou registada
neste relatório, não no `Cargo.toml`.

---

## Critério de fecho do passo

- [x] Mecanismo exacto do vanilla confirmado (`hayro`/`hayro-syntax` para
      parsing/render, `krilla::draw_pdf_page` para escrita como Form
      XObject) — nenhum parâmetro `page:` encontrado em `#image()` (vanilla
      só usa a 1.ª página do PDF fonte, via `PdfImage::new(document, 0)`
      implícito no caminho de detecção; não confirmado um selector
      explícito de página na API pública).
- [x] Decisão de âmbito registada com custo real medido (árvore de
      dependências `hayro`, não estimativa).
- [x] Não implementado: gap documentado com mensagem de erro dedicada e
      específica (`"PDF images are not supported yet"`, antes genérica
      `"unknown image format"` — ambas já eram erros explícitos, não
      omissão silenciosa; a mudança é de especificidade, não de
      comportamento observável fundamental), registado como scope-out
      consciente.
- [x] `cargo test --workspace` verde (4967 passed, 0 failed).
- [x] `crystalline-lint .` zero violações (excepto V7 pré-existente).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p781.md`
      (este ficheiro).

---

## Próximo passo

Restam, dos débitos ainda abertos: fallback de fontes matemáticas (débito
antigo de P772w), splice de `#expr`/field-access bare em modo math (débito
de P780). Ambos candidatos a passo dedicado. Alternativa: parar para um
resumo da série P765a-P781.
