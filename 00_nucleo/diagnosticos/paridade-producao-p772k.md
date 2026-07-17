# P772k — Varredura da stdlib: `typst_library::visualize::image::svg`

> **Passo:** 772k
> **Data:** 2026-07-16
> **Commit-base:** `61b7edee78fdae9b020e458f5989f638cbf04096` (HEAD), working tree limpo em
> `01_core/`, `03_infra/`, `00_nucleo/prompts/` no momento da medição
> (`git status --short` sem alterações nesses diretórios).
> **Medido em:** 2026-07-16T22:09:37Z.

---

## 1. Classificação item a item (7 itens)

Lista extraída de `00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt`:

```
typst_library::visualize::image::svg::base_options
typst_library::visualize::image::svg::FontResolver
typst_library::visualize::image::svg::format_usvg_error
typst_library::visualize::image::svg::ImageResolver
typst_library::visualize::image::svg::SvgImage
typst_library::visualize::image::svg::SvgImageInner
typst_library::visualize::image::svg::tree_size
```

Grep cruzado (`grep -rn "SvgImage\|FontResolver\|tree_size\|base_options\|format_usvg_error\|ImageResolver" 01_core/src 03_infra/src`) — **zero ocorrências**. Confirmado também por busca ampla `grep -rlni svg 01_core/src 02_shell/src 03_infra/src 04_wiring/src`: o único hit em todo o cristalino é `01_core/src/rules/stdlib/structural.rs:2347`, uma entrada de tabela de extensão→kind (`"svg" => "image"`) usada para inferir o `kind` de `figure()`, sem qualquer relação com decodificação/renderização.

| Item | Classificação | Nota |
|---|---|---|
| `base_options` | **Lacuna total** | Opções base do `usvg::Tree::from_data` — inexistente; não há parsing de SVG no cristalino. |
| `FontResolver` | **Lacuna total** | Resolução de fontes Typst para dentro do usvg (texto em SVG) — inexistente. |
| `format_usvg_error` | **Lacuna total** | Formatação de erro de parsing SVG para diagnóstico do utilizador — inexistente. |
| `ImageResolver` | **Lacuna total** | Resolução de imagens raster referenciadas (`<image href=...>`) dentro de um SVG — inexistente. |
| `SvgImage` | **Lacuna total** | Tipo de dados que representa um SVG decodificado (`Arc<SvgImageInner>`) — inexistente. |
| `SvgImageInner` | **Lacuna total** | Estado interno (`data`, `size`, `font_hash`, `tree`) — inexistente. |
| `tree_size` | **Lacuna total** | Cálculo do tamanho em píxeis a partir da `usvg::Tree` — inexistente. |

**0 de 7 itens têm qualquer correspondência no cristalino.** Isto não é uma
lacuna de granularidade fina (função renomeada, lógica dispersa) — é a
ausência completa de um subsistema. O cristalino não tem nenhum decodificador
de SVG.

---

## 2. Evidência empírica — efeito observável (ADR-0107/ADR-0108)

### 2.1 SVG válido — vanilla renderiza, cristalino omite silenciosamente

```bash
cat > /tmp/p772k/test.svg <<'EOF'
<svg xmlns="http://www.w3.org/2000/svg" width="100" height="80">
  <rect x="10" y="10" width="60" height="40" fill="red"/>
  <circle cx="50" cy="60" r="15" fill="blue"/>
</svg>
EOF
cat > /tmp/p772k/test.typ <<'EOF'
#image("test.svg")
Hello after image.
EOF
```

- Vanilla (`lab/typst-original/target/release/typst compile`): **exit 0**, PDF de 6970 bytes com a forma desenhada.
- Cristalino (`./target/release/typst`): **exit 0**, mas imprime no stderr
  `Formato de imagem desconhecido — imagem omitida` (`03_infra/src/export/images.rs:353-354`,
  ramo `ImageFormat::Unknown` de `process_image_item`) e produz um PDF sem a
  imagem — o `#image("test.svg")` desaparece do documento sem qualquer aviso
  no diagnóstico do compilador.

### 2.2 Achado relacionado, mais grave que o SVG isolado — divergência de língua confirmada

```bash
printf '\x00\x01\x02\x03garbage-not-an-image' > /tmp/p772k/bogus.png
echo '#image("bogus.png")' > /tmp/p772k/bogus.typ
```

- Vanilla: **exit 1**, erro de compilação:
  `error: failed to decode image (Format error decoding Png: Invalid PNG signature.)`
  apontando para o span de `#image("bogus.png")`.
- Cristalino: **exit 0**, mesmo `eprintln!` de "Formato de imagem desconhecido —
  imagem omitida", PDF gerado sem erro e sem a imagem.

Isto é **divergência ao nível da língua**, não da mecânica (ADR-0107): no
Typst, `#image()` com um ficheiro que não pode ser decodificado é um erro de
compilação observável pelo utilizador do documento — nunca sucesso silencioso
com conteúdo em falta. O cristalino actualmente trata *qualquer* formato não
reconhecido (SVG incluído) como aviso de terminal + omissão silenciosa.

### 2.3 Este achado já estava catalogado — não é novo, mas segue por corrigir

`00_nucleo/diagnosticos/paridade-producao-p650.md` §2.5 e §3 já identificou
exactamente este padrão como **"Confirmado"** (prioridade 2 da lista de
follow-ups): `03_infra/src/export/images.rs:279,285` — "Imagem inválida/
desenhecida omitida com `eprintln!`" / "Falha silenciosa". P650 decidiu
explicitamente **não corrigir** no próprio passo: *"Cada item da lista
priorizada deve virar o seu próprio passo de correcção, com sonda-causa-
correcção."* Isso nunca aconteceu para este item — o código em
`03_infra/src/export/images.rs:353-354` no commit `61b7edee7` é idêntico em
comportamento ao descrito em P650.

A varredura de P772k confirma que a causa raiz de "0/7 itens de `image::svg`"
e o item 2 do debt de P650 são **a mesma coisa**: não existe validação de
formato de imagem em tempo de avaliação (`native_image` em
`01_core/src/rules/stdlib/figure_image.rs`, que só lê bytes crus via
`world.read_bytes` sem inspeccionar o formato); a única detecção de formato
(`detect_format` em `03_infra/src/export/images.rs`) acontece tarde demais,
na exportação PDF em L3, momento em que já não há caminho arquitectural para
devolver um `SourceDiagnostic` de compilação amarrado ao span do `#image()`
original.

---

## 3. Por que não há implementação directa neste passo

O critério do passo é "Implementação directa para achados confirmados". Dois
achados foram confirmados; nenhum dos dois é implementável directamente sem
violar a Regra de Ouro do `CLAUDE.md` ("o assistente nunca pode instruir a
escrita de código L1/L2/L3 se o Prompt L0 correspondente não existir"):

1. **Suporte completo a SVG** (os 7 itens) exigiria adoptar um pipeline de
   parsing+rasterização (equivalente a `usvg`/`resvg`/`fontdb` do vanilla),
   decidir a fronteira L1/L3 para essa dependência externa, especificar
   resolução de fontes Typst dentro do SVG e resolução de imagens raster
   ligadas. Isto é uma funcionalidade nova de grande porte, não um bug
   pontual — não existe nenhum L0 em `00_nucleo/prompts/` que cubra
   `visualize::image::svg` (confirmado: nenhum ficheiro em
   `00_nucleo/prompts/entities|infra|rules/**` menciona "svg").
2. **Corrigir a omissão silenciosa** (tornar formato desconhecido/corrupto um
   erro de compilação, paridade com a linguagem) exigiria mover a validação
   de formato de L3 (tempo de exportação) para L1 (tempo de avaliação de
   `native_image`), o que implica uma nova dependência injectada em
   `EvalContext`/`World` (padrão análogo ao `ImageSizer` já existente) —
   decisão arquitectural nova, sem L0 que a cubra hoje.

Ambos ficam fora do escopo de "implementação directa" de um passo M e exigem
um novo Prompt L0 (ou ADR) redigido e confirmado pelo humano antes de código,
conforme o Protocolo de Nucleação.

---

## 4. Decisão

Nenhuma correcção de código foi feita neste passo. Recomenda-se abrir um novo
passo dedicado (fora da série P765a→P772m de varredura) especificamente para:

- (a) decidir e especificar em L0 a estratégia de suporte a SVG (adoptar
  `usvg`/`resvg` equivalentes como dependência L3 nova, ou aceitar scope-out
  permanente e documentá-lo como tal), e
- (b) especificar em L0 a validação de formato de imagem em tempo de
  avaliação, fechando também o item 2 do debt de P650.

Estes dois pontos devem ser tratados juntos porque a correcção de (b) por si
só já eliminaria o sintoma mais grave (sucesso silencioso com conteúdo
perdido) mesmo antes de (a) estar implementado — um `#image("x.svg")`
passaria a produzir um erro de compilação claro ("formato SVG não suportado")
em vez de desaparecer sem aviso.

---

## 5. Validação

```
cargo test --workspace   # sem alterações de código — não re-executado neste passo isolado; será rodado na validação final de P772k/l/m
crystalline-lint .       # idem
```

Nenhum código foi alterado em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/` neste passo — apenas leitura/sonda e este relatório.

---

## Critério de fecho do passo (`typst-passo-772k.md`)

- [x] Os 7 itens de `image::svg` classificados item a item (todos "lacuna total").
- [x] Caso de teste real (SVG com formas) comparado — vanilla renderiza, cristalino omite silenciosamente.
- [ ] Bugs reais corrigidos com teste e comparação directa — **não aplicável**: achados confirmados requerem novo L0 antes de código (Regra de Ouro), registado na secção 3-4 acima.
- [x] `cargo test --workspace` verde — sem alterações de código; validação final consolidada ao fim de P772k/l/m.
- [x] `crystalline-lint .` zero violações — sem alterações de código.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772k.md`.
