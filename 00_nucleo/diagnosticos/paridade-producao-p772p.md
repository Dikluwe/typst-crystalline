# P772p — L0 + Implementação: formato de imagem inválido/desconhecido deve ser erro de compilação

> **Passo:** 772p
> **Data:** 2026-07-17
> **Commit-base:** `56bbaa7a4f2ef35ee0aa970f72bde8021223f552` (HEAD).
> **Medido/implementado em:** 2026-07-16T23:xx–2026-07-17T00:58Z.
> **Dependência:** P772k (achado, `61b7edee7`), P650 (achado original, prioridade 2, nunca corrigido até agora).

---

## 1. Sonda — mecanismo exacto do vanilla

```
grep -n "fn decode\|failed to decode image\|ImageError" \
  lab/typst-original/crates/typst-library/src/visualize/image/*.rs
```

### 1.1 Onde a decodificação acontece — corrige a premissa do prompt do passo

O prompt do passo assumia "tempo de avaliação de `native_image`". **Medido,
não confirmado**: `Packed<ImageElem>::decode()`
(`typst_library::visualize::image::mod.rs:215`) é chamado de
`typst-layout/src/image.rs:19` — **tempo de layout**, não de avaliação.
`ImageElem` na avaliação só guarda os bytes crus; a decodificação/validação
acontece mais tarde, quando o layout de facto precisa da imagem — mas ainda
assim dentro do compilador (`SourceResult` fallível), nunca na exportação.

**Por que a implementação segue mesmo assim o plano original (avaliação, não
layout)**: `01_core/src/rules/layout/mod.rs::layout_content` do cristalino
devolve `()`, não `SourceResult<()>` — o layout é **estruturalmente
infalível** hoje; não há caminho nenhum para propagar um erro de compilação
a partir daí. Tornar o layout falível seria uma mudança muito maior que este
passo M. Avaliação (`native_image`) é o único ponto do pipeline actual que
já devolve `SourceResult` e pode produzir o erro — diverge do *mecanismo*
exacto do vanilla, não do *observável* (ADR-0107): o utilizador continua a
ver um erro de compilação, só que decidido um pouco mais cedo no pipeline.

### 1.2 Mensagens exactas — dois casos distintos, confirmados

```
$ echo '#image("bogus.png")' > /tmp/p772p-bogus.typ
$ printf '\x00\x01\x02\x03garbage-not-an-image' > /tmp/p772p-bogus.png
$ lab/typst-original/target/release/typst compile /tmp/p772p-bogus.typ
error: failed to decode image (Format error decoding Png: Invalid PNG signature.)
```

Este caso é **extensão-guiada**: `determine_format_from_path` (linha 349-360)
mapeia `.png` → `ExchangeFormat::Png` **antes** de olhar para o conteúdo — o
decoder é chamado, falha na assinatura, e o erro vem do decoder PNG, não de
"formato desconhecido". Só quando **nem a extensão nem o conteúdo** mapeiam
para um formato conhecido (`ImageFormat::detect`, linha 344) é que vanilla
diz `"unknown image format"`.

### 1.3 Profundidade de decodificação — vanilla decodifica tudo

`RasterImage::new_impl` (`raster.rs:49-95`) chama
`image::DynamicImage::from_decoder(decoder)` — **decodifica os píxeis
inteiros**, não só o cabeçalho. Isto é medido, não hipotético: `dynamic:
Arc<DynamicImage>` fica guardado na struct.

---

## 2. Decisão de âmbito (registada antes de implementar)

| Opção | Vanilla faz assim? | Cristalino pode fazer? |
|---|---|---|
| Validar só assinatura/cabeçalho em L1 | **Não** (decodifica tudo) | **Sim** — zero I/O, zero dependência externa |
| Decodificar a imagem inteira em L1 | Sim | **Não** — exigiria a crate `image` (ou equivalente) em L1, violando a pureza de L1 (`CLAUDE.md`, restrição absoluta) |

Nenhuma das duas opções da tabela original do passo é um "match" perfeito:
a opção barata não é o que o vanilla faz; a opção fiel ao vanilla viola uma
restrição arquitectural inegociável. **Escolhida a opção barata**, registando
explicitamente a divergência de mecanismo e a lacuna que ela deixa em aberto
(§5). Isto seguiu o mesmo padrão de P772n: a sonda corrigiu a hipótese do
prompt antes de implementar.

Adicionalmente, a arquitectura proposta pelo prompt ("novo trait injectado em
`EvalContext`/`World`, padrão análogo a `ImageSizer`") revelou-se
desnecessária: detecção por assinatura binária é computação pura, sem I/O —
ao contrário de `ImageSizer` (que precisa da crate `imagesize`), não precisa
de injecção de dependência nenhuma. Implementada como função livre em L1.

---

## 3. L0 — escrito antes do código

- `00_nucleo/prompts/entities/image-format.md` (novo) — `ImageFormat`/
  `detect_image_format`, movidos de L3 para L1 (não duplicados).
- `00_nucleo/prompts/rules/stdlib/figure_image.md` — nova secção
  "Validação de formato em avaliação (P772p)": mecanismo, mensagens,
  divergência aceite, limitação de span.
- `00_nucleo/prompts/infra/export/images.md` — `ImageFormat`/`detect_format`
  passam a ser importados de L1, não redefinidos.

---

## 4. Implementação

1. **`01_core/src/entities/image_format.rs`** (novo, L1): `enum ImageFormat
   { Jpeg, Png, Unknown }` + `fn detect_image_format(data: &[u8])
   -> ImageFormat` — exactamente a lógica que vivia em
   `03_infra/src/export/images.rs::{ImageFormat, detect_format}`
   (`pub(super)`), movida byte a byte.
2. **`03_infra/src/export/images.rs`**: `detect_format`/`ImageFormat`
   locais removidos; `pub(super) use
   typst_core::entities::image_format::{ImageFormat, detect_image_format};`
   no seu lugar. `mod.rs`, `builder.rs`, `tests.rs` actualizados para o
   nome novo (`detect_format` → `detect_image_format`) — mesmo
   comportamento, zero duplicação.
3. **`01_core/src/rules/stdlib/figure_image.rs::native_image`**: depois de
   `world.read_bytes`, antes de construir `Content::Image`:
   - `path` termina em `.svg`/`.svgz` (case-insensitive) → erro `"SVG
     images are not supported yet"`.
   - senão, `detect_image_format(&data) == Unknown` → erro `"unknown image
     format"`.
   - `Png`/`Jpeg` → prossegue sem alteração.

### Teste pré-existente corrigido (não regressão, fixture desactualizada)

`native_image_retorna_content_image` (`01_core/src/rules/stdlib/mod.rs`)
usava bytes arbitrários (`vec![1, 2, 3]`) como conteúdo de "foto.png" — só
passava porque não havia validação nenhuma. Corrigido para usar a
assinatura PNG real; o teste continua a verificar o mesmo comportamento
(`native_image` devolve `Content::Image`), agora com um fixture honesto.

### Testes novos

`native_image_formato_desconhecido_gera_erro`,
`native_image_svg_gera_erro_nao_suportado`,
`native_image_jpeg_valido_sem_regressao` (`01_core/src/rules/stdlib/mod.rs`).

---

## 5. Validação

### 5.1 Casos do passo

```
#image("bogus.png")  (bytes: \x00\x01\x02\x03garbage-not-an-image)
  → error: unknown image format          exit 1  ✓ (vanilla: mensagem diferente, mesmo exit 1 — ver §5.3)

#image("test.svg")
  → error: SVG images are not supported yet   exit 1  ✓ (antes: omissão silenciosa, exit 0)

#image("valid.png")  (PNG 1×1 real)
  → exit 0, PDF gerado normalmente        ✓ sem regressão
```

### 5.2 Suite completa

```
cargo test --workspace
  4183 (typst-core, +3 novos, 1 fixture corrigida) + 645 + 33 + 2 + 29 + 2, 0 falhas
crystalline-lint .
  0 violações (mesmo warning V7 pré-existente, não relacionado)
```

### 5.3 O que fica por fechar (registado, não escondido)

- **Mensagem exacta para PNG com extensão certa mas conteúdo corrompido**:
  vanilla diz `"failed to decode image (Format error decoding Png: Invalid
  PNG signature.)"`; cristalino diz `"unknown image format"` para o mesmo
  caso (`bogus.png` com bytes aleatórios) — porque a detecção é só por
  assinatura de conteúdo, não por extensão-depois-decode como o vanilla.
  Ambos erram, exit 1 nos dois, nenhum omite a imagem — mas o texto exacto
  diverge. Corrigir isto exigiria replicar a precedência extensão→decode
  completo do vanilla, o que reabre a restrição de pureza de L1 (§2).
- **Corrupção mais funda que a assinatura**: um PNG com assinatura válida
  mas payload truncado/corrompido a meio **não é apanhado** por esta
  validação (só olha os primeiros bytes) — continua a cair no caminho
  antigo (L3, `eprintln!`, omissão silenciosa). Item 2 do debt de P650
  **parcialmente** fechado: o caso medido e testado (formato totalmente
  não reconhecido) fecha; o caso "formato certo, payload corrompido" não.
- **Span**: todos os erros de `native_image` (incluindo os já existentes
  antes deste passo — ficheiro não encontrado, argumento inválido) usam
  `Span::detached()`. `Args` (`entities/args.rs`) não transporta o span de
  cada argumento posicional — corrigir isto é uma mudança na ABI de
  chamada de funções nativas em geral (afecta todas, não só `image()`),
  fora do âmbito deste passo M. Critério de fecho do passo pedia "span
  correcto" — **não cumprido**, registado explicitamente, não escondido
  atrás de "concluído".

---

## Critério de fecho do passo (`typst-passo-772p.md`)

- [x] Mecanismo do vanilla confirmado (ponto de validação — layout, não
      avaliação —, mensagens exactas por caso, decodificação completa).
- [x] Decisão de âmbito registada — nenhuma das duas opções da tabela original
      encaixava sem ressalva; escolhida a validação leve, com a divergência
      de mecanismo e a lacuna residual explicitamente registadas.
- [x] L0 escrito antes do código.
- [x] Detecção movida para L1 (não um trait novo injectado — desnecessário,
      é computação pura), reaproveitando `detect_format` de L3 sem duplicar.
- [x] `native_image` liga a validação.
- [ ] Erro amarrado ao span correcto — **não cumprido**, `Span::detached()`
      mantido (mesma convenção das outras mensagens de `native_image`);
      motivo estrutural registado em §5.3.
- [x] Formato inválido/corrompido (ao nível da assinatura) dá erro de
      compilação, não omissão silenciosa.
- [x] SVG (ainda não suportado) dá erro claro, não omissão silenciosa.
- [x] PNG/JPEG válidos sem regressão.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772p.md`.

---

## Próximo passo

Seguir com §2.2 (mensagem de mutação de variável capturada, P772l) ou §2.4
(hint de subtracção em `unknown_variable`), ou tratar a lacuna de span geral
em `Args`/chamada de funções nativas como o seu próprio passo dedicado
(afecta mais do que `image()` — beneficiaria todas as mensagens de erro de
argumento de funções nativas), ou reconfirmar `lacuna-inventario`.
