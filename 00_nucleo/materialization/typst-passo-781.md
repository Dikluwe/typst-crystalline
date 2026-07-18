---
# P781 — Suporte a `#image()` com fonte PDF (`image::pdf`)

> **Passo:** 781
> **Data:** 2026-07-17
> **Foco:** P772w confirmou `image::pdf` (4 itens) como gap real dentro do escopo do vanilla 0.15.0 (verificado contra `Cargo.toml.original`, não uma feature futura) — o cristalino não suporta `#image("arquivo.pdf")`, incorporando uma página de PDF como imagem dentro de outro documento. Registado como maior (nova dependência L3) e deferido. Este passo confirma o mecanismo exato do vanilla e decide se implementar cabe num só passo.
> **Tipo:** Sonda de arquitetura + Decisão registada (regra 1) + Implementação condicional.
> **Tamanho:** L — nova capacidade de decodificação, potencialmente nova dependência L3.
> **ADR-0108 EM VIGOR** — confirmar o mecanismo exato antes de estimar esforço.
> **Regra de ouro do CLAUDE.md** — L0 antes de código, dado ser capacidade nova.
> **Dependências:** P772w (achado, confirmação de escopo), P772k/P772p (metodologia já estabelecida para gaps de formato de imagem — reutilizar o padrão de erro explícito quando não suportado, já implementado para SVG).

---

## Sonda — mecanismo exato do vanilla

```bash
grep -n "PdfImage\|pdf.*image\|image.*pdf" lab/typst-original/crates/typst-library/src/visualize/image/*.rs 2>/dev/null
```

Confirmar:
1. Qual crate/dependência o vanilla usa para decodificar/renderizar uma página de PDF como imagem (provavelmente algo baseado em `pdfium` ou similar — confirmar exatamente).
2. Que parâmetros `#image()` ganha especificamente para PDF (ex: `page:` para escolher qual página do PDF fonte embutir, se o PDF tiver múltiplas páginas).
3. Se há alguma limitação documentada do próprio vanilla (ex: só a primeira página por omissão, sem suporte a formulários interativos, etc.).

```bash
cat > /tmp/p781-pdf-image-test.typ <<'EOF'
#image("test.pdf")
EOF
lab/typst-original/target/release/typst compile /tmp/p781-pdf-image-test.typ 2>&1
```

Gerar um PDF de teste simples primeiro (ex: com o próprio compilador vanilla) e confirmar o comportamento.

### Estado atual do cristalino

```bash
grep -n "detect_image_format\|ImageFormat" 01_core/src/entities/image_format.rs
```

Confirmar que hoje `.pdf` cai no caminho de "formato desconhecido" corrigido por P772p (erro explícito, não omissão silenciosa) — se já for esse o caso, a prioridade deste passo é só adicionar suporte real, não corrigir um sintoma de omissão silenciosa (já resolvido).

---

## Decisão de âmbito

| Cenário | Decisão |
|---|---|
| Vanilla usa uma dependência leve/já disponível no ecossistema Rust para renderizar PDF como imagem | Avaliar se cabe neste passo |
| Vanilla usa uma dependência pesada (ex: bindings para `pdfium`, `mupdf`) | Decisão maior — avaliar custo de adicionar em L3, decidir se compensa vs manter como gap documentado |

Registar a decisão com base no que a sonda encontrar, não assumir de antemão.

---

## Implementação (se decidido prosseguir)

1. Escrever L0 cobrindo a nova dependência (se houver), a fronteira L1/L3 (decodificação de PDF é I/O-like, fica em L3 com trait em L1, mesmo padrão de `ImageSizer`).
2. Implementar a decodificação/renderização da página do PDF fonte para os dados de imagem que o exportador já sabe embutir.
3. Ligar em `native_image`, com o parâmetro `page:` se o vanilla tiver.

---

## Validação

```bash
./target/release/typst compile /tmp/p781-pdf-image-test.typ /tmp/p781-cristalino.pdf
lab/typst-original/target/release/typst compile /tmp/p781-pdf-image-test.typ /tmp/p781-vanilla.pdf
mutool draw -o /tmp/p781-cristalino.png -r 300 /tmp/p781-cristalino.pdf
mutool draw -o /tmp/p781-vanilla.png -r 300 /tmp/p781-vanilla.pdf
compare -metric AE /tmp/p781-vanilla.png /tmp/p781-cristalino.png /tmp/p781-diff.png
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Mecanismo exato do vanilla confirmado (dependência, parâmetros, limitações).
- [ ] Decisão de âmbito registada com base no custo real.
- [ ] Se implementado: L0 escrito antes do código, `#image("arquivo.pdf")` funciona, validado por comparação visual.
- [ ] Se não implementado: gap documentado explicitamente (mensagem de erro clara, já garantida por P772p — confirmar que continua assim), registado como scope-out consciente, não abandonado silenciosamente.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p781.md`.

---

## Próximo passo

Fallback de fontes matemáticas (débito antigo de P772w), splice de `#expr`/field-access bare em modo math (débito de P780), ou parar para um resumo.
