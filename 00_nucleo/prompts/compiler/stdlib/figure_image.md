# Prompt L0 — `stdlib/figure_image` — imagens
Hash do Código: cd56d3e5

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

## P1141 — fonte `path | str` (gate ADR-0127)

### Medição antes da decisão

O vanilla documenta `image` como consumer de paths e usa a infraestrutura
`PathOrStr`; `foundations/path.rs:15-29,62-81` dá `image(path("..."))` como
caso central. O cristalino `figure_image.rs:159-186` aceita somente string e
chama `World::read_bytes(current_file, &str)`.

### Decisão

O primeiro posicional passa a aceitar `Value::Path | Value::Str`. O mesmo
helper enraizado de P1141 resolve string no caller uma vez e preserva path já
resolvido; leitura usa `World::read_path`. Detecção de formato usa a vpath
normalizada para extensão e os bytes carregados para assinatura. O conteúdo
de imagem pode preservar a representação portátil da vpath, mas não `PathBuf`
físico nem informação capaz de furar a sandbox. Width/height/fit/page e os
scope-outs SVG/PDF não mudam.

Cast e contrato público mudam; implementar somente após o gate.

## P1473 — `figure.caption` (ESCOPO REDUZIDO — construção sem síntese) (MATERIALIZADO)

### Medição anterior à decisão

Sonda bilateral (2026-09-21, vanilla ratificado `586e1bd43fae`):
`type(figure.caption)` → `function` no vanilla, `function figure does not
contain caption` no cristalino (`figure` já é `Value::Func` bare).

Fonte pinada
(`lab/typst-original/crates/typst-library/src/model/figure.rs:511-591`):
`FigureCaption { position: OuterVAlignment (default Bottom, settable),
separator: Smart<Content> (default Auto — depende de idioma/região,
settable), #[required] body: Content, kind/supplement/numbering/counter/
figure_location: #[synthesized] }`. Os cinco últimos só existem depois de
`figure(...)` sintetizar a legenda dentro de si (`.realize()` monta o
texto final combinando `supplement`+numeração via `Counter::display_at`).

Sondas confirmam: `figure.caption()` → `missing argument: body`;
`figure.caption(1)` → `expected content, found integer`;
`figure.caption(body: [x])` → ``the argument `body` is positional``;
`figure.caption([x], position: left)` → `` expected `top` or `bottom`,
found left`` (`left` aqui é o identificador global de alinhamento, **não**
uma string — confirma `position` como `Value::Align`, não `Str`);
`figure.caption([x], position: "top")` → `expected alignment, found
string` (achado de integração, 2026-09-21: a string `"top"`, mesmo
"correta" em conteúdo, é rejeitada — só o identificador bare `top` é
aceite; a redação original da "Decisão" abaixo dizia `Str`, corrigida por
esta medição). `.fields()` omite `position`/`separator` quando não
definidos explicitamente (`(body: [x])`), inclui-os na ordem
`position, separator, body` quando definidos; acessar `.position`/
`.separator` numa caption **não sintetizada** (fora de um `figure(...)`
real) é erro `field "position"/"separator" in caption is not known at
this point` — confirma que estes dois campos só resolvem valor **depois**
da síntese, não na construção.

### Decisão — escopo reduzido

Registrar somente a **construção**: `figure.caption(body, position:,
separator:)`. `body` é `Content` posicional obrigatório (mesmas
coerções/diagnósticos medidos acima). `position` é um valor de
**alinhamento** (`Value::Align`, o mesmo tipo de `top`/`bottom`/`left`
bare já existente no cristalino — `type(top) == "alignment"`), restrito à
componente vertical `top`/`bottom` sem componente horizontal — **não**
`Str`. `None` = não definido (settable, sem default materializado aqui).
`separator` é
`Content` nomeado opcional, `None` = não definido (o default `Auto`
dependente de idioma **não é resolvido** nesta fatia — ver Não-objetivos).
Constrói `Content::FigureCaption(FigureCaptionElem { position, separator,
body })`, nova entidade em `entities/elements/figure_caption.rs` (owner
próprio, `entities/elements/figure_caption.md`, a criar) — sem os cinco
campos sintetizados do vanilla (não fazem sentido fora da síntese que
esta fatia não implementa).

`native_figure` passa a aceitar, além de `Content`/`Str`/`None` já
aceites para `caption:`, também `Value::Content(Content::FigureCaption(_))`
— extrai o `.body` desse valor e usa-o exatamente como usaria um
`Content` bare (mesmo caminho de código, `FigureElem.caption:
Option<Content>` continua bare `Content`, sem novo campo). `position`/
`separator` da caption explícita **não têm efeito** nesta fatia — ficam
preservados dentro do valor `Content::FigureCaption` original (que já não
é mais consultado depois da extração do `body`), não descartados
silenciosamente (o valor original continua acessível a quem o construiu,
só não é lido pelo `native_figure` atual).

**Não implementa nesta fatia:** síntese (`kind`/`supplement`/`numbering`/
`counter`/`figure_location`), `.realize()` (texto final com
supplement+numeração via `Counter`), resolução do `separator` default por
idioma/região, nem qualquer efeito visual de `position`/`separator`
explícitos. `figure.caption.position`/`.separator` não reproduzem o erro
"not known at this point" do vanilla nesta fatia (ausência de sintetizado
é modelada como campo simplesmente ausente da entidade, não como estado
"ainda não sintetizado") — divergência mecânica aceitável (ADR-0107), não
paridade de linguagem alegada.

### Gate ADR-0127

Nova rota qualificada + nova entidade pública + extensão do cast de
`caption:` em `native_figure`. Requer L0 causal (este e o novo
`entities/elements/figure_caption.md`), hashes ressealados, V5/V15/V26,
commit documental e **confirmação humana explícita posterior**.

Confirmação recebida em 2026-09-21, posterior ao commit causal `3ac60a578`.
Agente A (`01a82593d`) implementou `native_figure_caption` e estendeu
`native_figure` sem ver os testes; a redação original deste L0 dizia
`position: Str` — Agente A implementou fielmente essa (incorreta)
especificação. Agente B (`0258eab89`) escreveu 8 testes RED a partir do
mesmo L0, logo também usando `position: "top"` (string). **Correção
descoberta na integração (papel C, 2026-09-21):** medição direta contra o
vanilla (`figure.caption(x, position: "top")`) revelou `expected
alignment, found string` — `position` nunca foi `Str`; é `Value::Align`
restrito à componente vertical `top`/`bottom` (ver "Sondas confirmam"
acima, corrigido). Implementação e os 2 testes afetados corrigidos para
usar o identificador bare `top`/`bottom` (`Value::Align`), não a string;
o teste de tipo errado tornou-se ele próprio um novo caso RED→GREEN
(`position: "top"` agora corretamente rejeitado). Suíte: GREEN, zero
regressão em typst-core (6129/6129 final).
