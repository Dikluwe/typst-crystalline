# Prompt L0 — `stdlib/figure_image` — imagens
Hash do Código: 2fd219ca

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/caption-projection.toml sha256:923f1c9e4c2fd5934316ae2eef3dbb461c3bef3009de1d8931d951e793dda342

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

## P1481 — argumento `image(alt:)` (IMPLEMENTADO; confirmado em 2026-09-23)

Medição bilateral: fixture `p1481-corpus.typ`, SHA-256
`90475d9f6bd2b6964495861203c480af064b7273c14dc0feb492f79444dbd7d4`,
vanilla ratificado compila PDF/UA-1 (veraPDF 106/0, 623/0 checks);
cristalino erra `argumento nomeado inesperado em image(): 'alt'`.
A fonte vanilla `visualize/image/mod.rs:165-181` define `alt` opcional.

Contrato confirmado: `native_image` aceita `alt: str` ou ausência,
valida tipo e transporta `Option<EcoString>` à entidade dona. O valor
não vira texto pintado nem altera tamanho, fit, MIME ou I/O. `alt`
ausente continua representável para que a pipeline PDF/UA emita o erro
específico; outros targets preservam comportamento anterior. Mudança do
campo público da entidade depende do gate ADR-0127.


## Guards de domínio explícitos — V17, 2026-10-16

### Medição anterior à decisão

T241: 01_core/src/compiler/stdlib/figure_image.rs:174; T242: 01_core/src/compiler/stdlib/figure_image.rs:177. Fontes e L0 vigentes lidos antes da prescrição; SHA-256, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e git diff HEAD --stat em `lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz`. O inventário anterior, reconfirmado por hash, registra 28 guards em 21 owners; a regra CI pinada manda desdobrar ou mover a condição para o corpo. A medição é estrutura Rust, não prova de defeito funcional ou de melhor leitura por IA. Nenhuma intenção upstream é inferida.

### Responsabilidade exclusiva e aceitação

Usar padrão estrutural Align2D {h: None, v: Some(Top/Bottom)} para position da caption, mantendo aceitação exclusiva e mensagens/repr de alinhamento inválido. Importar o tipo já público sem criar API ou mudar separator/body.

Manutenção interna em fluxo contínuo ADR-0127: sem contrato público, default, compatibilidade ou fase novos. A responsabilidade permanece somente no consumer exclusivo deste L0. Não criar helper apenas para esconder a conjunção, não adicionar supressões nem mudar a configuração do linter. Executar controles funcionais existentes antes do código e preservar seus esperados; o RED real é o gate V17 com 28 avisos, GREEN exige zero V17. Conferir caminhos de sucesso/fallback/erro, curto-circuito e efeitos; resselo restrito por owner. Revalidar build, suíte workspace completa, Clippy all-targets com warnings e expectativas não cumpridas como erro, formato, Rustdoc com warnings como erro, linhagem V1/V5/V15/V26 estrita/reversa e SARIF sem novos avisos. Executar sob 12 GiB, swap zero e um job. Não alegar ganho de memória/desempenho ou paridade global sem medição separada.

## Domínio de figura e medidas — 2026-10-17

### Medição anterior à decisão

lab/results/construction/diagnostics/tekt-v16-figure-casts-2026-10-17/before.json.gz e
before-expanded-measurement.json.gz fixam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado, UTC, status/diffstat e fontes/L0/config/binários.
São 130 pares CLI. O pin é 9f2b6e8715237cb086899a42873660fe744622e8,
binário 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
figure_image.rs:68-105 silencia body inválido como Empty, caption inválida como
nome do tipo e kind inválido como inferência. O pin rejeita essas entradas;
body precede caption, que precede kind, mesmo se a ordem nomeada for invertida.
kind aceita string/auto/função de elemento, mas não função arbitrária/With.
O pin valida FigureKind em model/figure.rs:666-681 como Element ou EcoString.

casting.rs:24-53 conserva Rel na largura de block, mas extract_length reduz
Relative à parte absoluta e Ratio a zero. h/box aceitam números onde o pin
rejeita; .fields() também é incompleto. Isto mede dívida real, não prova
intenção upstream nem autoriza trocar percentuais por erro. Os carriers
públicos Spacing::Absolute(Length), BoxedElem.width:Option<Length> e
FigureElem.kind:Option<String> não transportam todos os dados de linguagem.
Essa evolução pública e eventual síntese de figure exigem proposta ADR-0127;
não as simular com nomes, valores neutros ou deslocamento informal de fase.

### Decisão exclusiva — rejeições de native_figure

Corrigir body: Content/LocatedContent, Str e None são conteúdo (None vazio);
outros valores erram expected content, found {vanilla_type_name}. Ausência
continua erro, agora missing argument: body. Caption aceita None (ausência),
Content/LocatedContent e Str; inválido erra expected content or none, found
{tipo}. Preservar a extração existente de FigureCaption.body. Não mudar
native_figure_caption, imagens nem o transporte de metadados de caption.

Kind: Str preserva o nome; Auto conserva o estado automático histórico.
Func é válida somente se reconhecida como elemento pela tabela já existente
native_element_fields ou Func::element_name. Não comparar Func pelo nome.
Função arbitrária/With erra expected element. Outros Value, incluindo None,
erram expected function, string, or auto, found {tipo}; não caem em inferência.
Funções de elemento válidas ainda conservam o caminho histórico de inferência
até o contrato público próprio; não alegar kind:table equivalente a string
ou identidade tipada correta nesta fatia. Essa dívida permanece aberta.

Diagnósticos de tipo apontam para value_span da ocorrência correspondente:
primeiro body posicional, último caption/kind nomeado conforme a view de Args.
Fallback: span da ocorrência, depois args.span; não buscar origem por igualdade
Value. Ordem de validação body→caption→kind, sem reavaliar expressões nem
alterar Args/ocorrências. Ausência usa diagnóstico detached como as nativas
existentes para o caller atribuir origem. Não expandir argumentos suportados
nem mudar prioridade de extras/nomes desconhecidos nesta correção.

Não encobrir infer_kind_from_body: seu V16 e suas limitações de recursão e
fase ficam abertos para a proposta de síntese/identidade pública. Não listar
containers como folhas neutras apenas para fazer o linter passar.

### Aceitação

Correção interna/paridade em fluxo contínuo ADR-0127, sem assinatura pública,
default deliberado de produto, compatibilidade Rust ou fase novos. L0 primeiro,
testes funcionais RED para rejeições reais e controles válidos; GREEN exige
mensagens e âncoras causais e precedência sem avaliação repetida. Testes dos
casts controlam comprimentos afins, block relativo e rejeições de famílias;
não cristalizar truncagem como semântica final. Sondas estruturais extraem
padrões produtivos para provar variante futura rejeitada nos dois matches;
payloads stub/RHS apagado não constituem equivalência funcional.

Repetir os 130 pares, distinguir correções intencionais, controles e dívidas
remanescentes, sem declarar todo JSON/fields/reflexão equivalente. Inspecionar
os consumers dos carriers e registrar inventário antes de propor API. V15/V26
válido antes de resselo apenas destes dois owners. Build, suíte workspace,
Clippy -D warnings/-D unfulfilled_lint_expectations, fmt, Rustdoc -D warnings
com quatro índices, linhagem estrita/reversa, diff e SARIF diferencial.
12 GiB rígido, high 10 GiB, swap zero, jobs=1; execução pesada sequencial.
Sem release, merge ou commit/push do Typst. RG1 permanece independente.

### Incompletude e passos de conclusão nomeados

Esta correção não fecha a construção integral de figura nem medidas relativas.
FG1 (lab/results/construction/diagnostics/tekt-v16-figure-casts-2026-10-17/follow-ups.md)
completa identidade tipada de kind e decide síntese/recursão após auditoria dos
owners e gate público; RM1 no mesmo documento completa o transporte de medidas
relativas e a revisão de casts por consumer. São trabalhos pendentes, não L0
materializável nem autorização de API. Não sobrescrever esta etapa/TS1/RG1 com
candidatos antigos ao concluir os passos.

## Complemento de casts de figure — Type e Symbol, 2026-10-17

Medição anterior à decisão: type-kind-probe.json e more-cast-probe.json em
lab/results/construction/diagnostics/tekt-v16-figure-casts-2026-10-17 registram o candidato
que passou 5/6 testes mas falhou em kind:int. foundations/value.rs:647-658 do
pin 9f2b6e8715237cb086899a42873660fe744622e8 mede Func aceitando Type.constructor
ou Symbol.func; content/element.rs:210-214 exige to_element; symbol.rs:133-137,
math/accent.rs:189-214 e math/lr.rs:144-198 classificam a conversão simbólica.
Há 37 tipos exercitados no CLI; Location sem construtor foi conferido na
fonte pinada. type-kind-classes.json guarda a classificação completa dos 38.
Símbolo aceito como corpo/caption é convertido pelo pin a SymbolElem; o nosso
Content não tem essa variante e o markup atual conserva o valor como Text.
Esse carrier/morfologia exige evolução pública SY1, não pode ser declarado
resolvido pela preservação do texto nesta etapa.

Decisão exclusiva deste owner:

Concluir o cast de kind: Value::Type com construtor erra expected element;
sem construtor erra type {nome longo} does not have a constructor. Classificar
Type exaustivamente conforme a medição, sem executar o construtor. Bool usa
boolean; Relative usa relative length. Value::Symbol callable como acento
conhecido ou delimitador LR erra expected element, pois wrappers não são
funções de elemento; símbolo não callable erra symbol {valor} is not callable.
Delegar a classificação de acentos/LR aos seus owners via bridges privados,
sem duplicar tabelas nem criar função/trait público. Preservar as âncoras do
kind. Body/caption Symbol conservam o grapheme efetivo como Content::Text,
igual ao cast de markup vigente; não continuar a convertê-los em vazio/erro.
SY1 no follow-ups.md completará SymbolElem, reflexão/morfologia após proposta
e gate público; esta preservação textual é explicitamente incompleta.

Aceitação: L0 primeiro, RED funcional de Type/Symbol/valor textual antes do código,
GREEN e revalidação integral desta etapa; 12 GiB/swap zero/jobs1. Fluxo contínuo
ADR-0127, sem contrato Rust público, fase ou default de produto novo. Resselo só
dos owners após V15/V26; preservar testes de matemática e classificadores originais.

## Completar casts de conteúdo de figure.caption — 2026-10-17

Medição anterior: caption-shape-expanded-before.json fixa os sete casos novos
junto aos oito controles anteriores, com fontes reais e fonte de build do
binário separadas, UTC e SHA de binários. O pin aceita None/Str/Symbol como
body e None/Str como separator; o Tekt os rejeita. foundations/value.rs:646-650
mede o cast Content de None/Str/Symbol. A rota já aprovada de caption armazena
Content e pode conservá-los sem contrato Rust novo. Localizados são Content,
como medido/testado no constructor Figure desta etapa.

Native_figure_caption deve usar cast puro de Content/LocatedContent/Str/None/
Symbol para body e separator, preservando corpo/grapheme/origem. None vira
Empty conhecido, inclusive separator None explícito; Str vira texto, Symbol
usa o grapheme como Text provisório (SY1 permanece). Erro de cast conserva
expected content, found {tipo} e value_span causal. Extrair o helper privado
figure_argument_value_span da closure já testada de Figure e reutilizá-lo nas
duas rotas, sem reavaliar args; os outros erros/ordem não mudam.

Separator auto permanece incompleto no FG1: o carrier Option<Content> não
separa auto explícito de omissão. Não introduzir sentinel nem campo público;
preparar proposta de presença/tipo antes de implementá-lo. Isso não fecha o
constructor inteiro. Posição, síntese, phase, assinatura e structs não mudam.
L0 antes de teste RED de coerções/âncora e GREEN, seguido de todos os gates da
etapa. Correção interna/paridade em fluxo ADR-0127 sob 12 GiB/swap zero/jobs1.
