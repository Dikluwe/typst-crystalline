# Prompt L0 — `stdlib/figure_image` — imagens
Hash do Código: 8c24ef26

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib/figure_image.rs`
**Origem**: fatiado de `rules/stdlib.md` em **P314** (ADR-0104). Convenção e
helpers partilhados: ver `stdlib/_comum.md`.
**Nota de deriva (F4)**: `figure_image.rs` também define `native_figure`, não
specado em `stdlib.md`; preservado como candidato a spec dedicada (não inventado).

---

## Imagens (Passo 71 + P502)

| Função | Assinatura Typst | Implementação |
|--------|-----------------|---------------|
| `native_image` | `image(path, width?, height?, fit?)` | lê bytes via `ctx.world.read_bytes(path)`, cria `Content::Image` |

`fit` é `Str` named opcional (default `"cover"`); valores válidos `"contain"`,
`"cover"`, `"stretch"`.

`native_image` é a única função stdlib com I/O — usa `ctx.world.read_bytes(path)`
para aceder ao ficheiro (Passo 71 — DEBT-24).

### Validação de formato em avaliação (P772p)

Depois de `world.read_bytes`, antes de construir `Content::Image`:

1. Se `path` termina em `.svg`/`.svgz` (case-insensitive) → erro
   `"SVG images are not supported yet"`. Cristalino não decodifica SVG
   (P772k) — a extensão sozinha já garante isto, sem precisar de olhar o
   conteúdo.
2. Senão, `detect_image_format(&data)` (`entities/image-format.md`,
   partilhado com o exportador PDF). Se `Unknown` → erro
   `"unknown image format"` (paridade textual com o vanilla,
   `typst_library::visualize::image::mod.rs:344`, para o caso de
   assinatura binária totalmente não reconhecida).
3. `Png`/`Jpeg` → prossegue como antes (sem alteração). **Não** valida a
   integridade do payload (só a assinatura) — corrupção mais funda que
   a assinatura ainda pode escapar para o exportador PDF (P650 item 2,
   ainda aberto; ver `entities/image-format.md` §Decisão de âmbito).

Divergência conhecida e aceite (registada, não escondida): o vanilla
decodifica a imagem inteira neste ponto e por isso apanha corrupção do
payload; o cristalino só verifica a assinatura (pureza de L1 impede
decodificação completa aqui). Para o ficheiro `"bogus.png"` cheio de bytes
aleatórios (o caso medido em P650/P772k), o vanilla erra com
`"failed to decode image (Format error decoding Png: ...)"` — mensagem
diferente da nossa `"unknown image format"` — mas ambos os compiladores
erram, exit 1, nenhum omite a imagem em silêncio. A mensagem exacta diverge;
o comportamento observável essencial (erro vs sucesso silencioso) converge.

**Limitação conhecida — span**: os erros usam `Span::detached()`, igual a
todos os outros erros já existentes em `native_image` (formato de argumento
inválido, ficheiro não encontrado). `Args` (`entities/args.rs`) não
transporta o span de cada argumento posicional — corrigir isto exigiria uma
mudança mais ampla na ABI de chamada de funções nativas (afecta todas, não
só `image()`), fora do âmbito deste passo. Registado para follow-up, não
implementado às pressas.
