# Prompt L0 — `stdlib/figure_image` — imagens
Hash do Código: 70d51226

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/figure_image.rs`
**Origem**: fatiado de `rules/stdlib.md` em **P314** (ADR-0104). Convenção e
helpers partilhados: ver `stdlib/_comum.md`.
**Nota de deriva (F4)**: `figure_image.rs` também define `native_figure`, não
specado em `stdlib.md`; preservado como candidato a spec dedicada (não inventado).

---

## Imagens (Passo 71 + P502)

| Função | Assinatura Typst | Implementação |
|--------|-----------------|---------------|
| `native_image` | `image(path, width?, height?, fit?, page?)` | lê bytes via `ctx.world.read_bytes(path)`, cria `Content::Image` |

`fit` é `Str` named opcional (default `"cover"`); valores válidos `"contain"`,
`"cover"`, `"stretch"`.

`page` é `Int` named opcional (P835, #20 — decisão do dono, 2026-07-22):
**aceite como no-op validado** — no vanilla só tem efeito para fontes PDF
(`visualize/image/mod.rs:158-161`), e PDF-como-fonte continua scope-out
(P781/DEBT-68). Inteiro positivo é ignorado; `page: 0` ou negativo →
`number must be positive`; outro tipo → `expected integer, found {tipo}`
(mensagens verbatim do vanilla, medidas em P835).

`native_image` é a única função stdlib com I/O — usa `ctx.world.read_bytes(path)`
para aceder ao ficheiro (Passo 71 — DEBT-24).

### Validação de formato em avaliação (P772p)

Depois de `world.read_bytes`, antes de construir `Content::Image`:

1. Se `path` termina em `.svg`/`.svgz` (case-insensitive) → erro
   `"SVG images are not supported yet"`. Cristalino não decodifica SVG
   (P772k) — a extensão sozinha já garante isto, sem precisar de olhar o
   conteúdo.
2. **P781** — se `path` termina em `.pdf` (case-insensitive) → erro `"PDF
   images are not supported yet"`. Vanilla **suporta**
   `#image("ficheiro.pdf")` (embute uma página do PDF fonte como imagem,
   via `hayro`/`hayro-syntax` para parsing/interpretação + `krilla::
   draw_pdf_page` para escrever a página como Form XObject no PDF
   exportado). Decisão registada em `paridade-producao-p781.md`:
   dependência pesada (`hayro` sozinho arrasta ~15 crates transitivas,
   incluindo o motor de renderização vectorial `vello_common`/
   `vello_cpu` e o parser de fontes `skrifa`/`read-fonts`) e sem
   equivalente de "Form XObject" no exportador cristalino (hand-rolled,
   não usa `krilla`) — scope-out consciente, não implementado. Mesmo
   padrão do SVG: extensão sozinha basta, sem olhar o conteúdo.
3. Senão, `detect_image_format(&data)` (`entities/image-format.md`,
   partilhado com o exportador PDF). Se `Unknown` → erro
   `"unknown image format"` (paridade textual com o vanilla,
   `typst_library::visualize::image::mod.rs:344`, para o caso de
   assinatura binária totalmente não reconhecida).
4. `Png`/`Jpeg`/`Gif`/`WebP` (P833/#17) → prossegue como antes. **Não**
   valida a integridade do payload (só a assinatura) — a corrupção mais
   funda que a assinatura é apanhada em **L3** por
   `validate_document_images` (pipeline, antes do export — P833/#18), que
   falha a compilação com a mensagem do vanilla
   (`failed to decode image ({detalhe})`, span detached).

Situação pós-P833 (revê o registo anterior de divergência): o vanilla
decodifica a imagem inteira em avaliação e erra com
`"failed to decode image (...)"`; o cristalino verifica a assinatura em
avaliação (pureza de L1) e decodifica/valida em L3 no pipeline — erra com
a **mesma mensagem** (`failed to decode image ({detalhe})`, o detalhe vem
da crate `image` em ambos), span detached (nuance: sem a posição do
`#image(...)`). Para assinatura totalmente desconhecida, ambos erram em
avaliação (`"unknown image format"`). Nenhum dos dois omite a imagem em
silêncio — a lacuna de P650 item 2 está fechada desde P833.

**Limitação conhecida — span**: os erros usam `Span::detached()`, igual a
todos os outros erros já existentes em `native_image` (formato de argumento
inválido, ficheiro não encontrado). `Args` (`entities/args.rs`) não
transporta o span de cada argumento posicional — corrigir isto exigiria uma
mudança mais ampla na ABI de chamada de funções nativas (afecta todas, não
só `image()`), fora do âmbito deste passo. Registado para follow-up, não
implementado às pressas.
