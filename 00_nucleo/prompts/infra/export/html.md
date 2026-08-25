# Prompt L0 — exportação HTML semântica
Hash do Código: 64f391f4

**Camada:** L3  
**Ficheiro alvo:** `03_infra/src/export/html.rs`  
**ADR:** ADR-0128

## Medição anterior à decisão

O vanilla ratificado gera 175 bytes para `Hello, parity.`. Não existe exporter
HTML cristalino; SVG/PDF recebem `PagedDocument`, forma inadequada para HTML.

## Contrato inicial incompleto

```rust
pub fn export_html(content: &Content) -> Result<String, SourceDiagnostic>;
```

Emite doctype/head/body compactos e converte semanticamente o subconjunto da
ADR-0128. Texto de topo é agrupado em `<p>`. Escapa `& < > "`. Heading vira
`h1..h6`; strong/emph preservam morfologia. Variante fora do subconjunto retorna
erro explícito.

Testes: fixture plain byte-idêntica; escape; heading; strong/emph; variante não
suportada falha. A spec permanece incompleta até os grupos futuros da ADR-0128.

## P1165 — serialização de `HtmlElem` (RASCUNHO; ADR-0127)

**Medição:** com feature ligada, o vanilla representa
`html.elem("article", attrs: (lang: "pt"))[Olá]` como nó `elem` e o exporter
preserva tag/atributos/body. O exporter cristalino não possui variante HTML.

Após aprovação do contrato L1, `export_html` serializa `Content::HtmlElem`
diretamente: valida nome já tipado, escapa valores de atributo e texto, mantém
ordem de atributos, respeita elementos void/raw conforme a tabela aprovada e
recursa no body. O primeiro corte garante `html.elem`; tags tipadas, frame,
CSS, MathML e expansão completa permanecem fora e devem falhar explicitamente
quando ainda não representáveis.

## P1167 — primeiro lote tipado (APROVADO NO GATE ADR-0127 EM 2026-08-25)

Medição ratificada da fixture aninhada confirmou que `div`, `span`, `p`,
`h1..h6`, `strong`, `em` e `ul` usam serialização normal de abertura, body e
fecho; nenhum é void/raw. Como os constructors produzem o mesmo `HtmlElem` e
os casts ocorrem em L1 antes do exporter, P1168 não altera a assinatura nem o
algoritmo público deste owner: recursão/escape/ordem existentes devem emitir
os nós diretamente, sem wrapper `<p>` adicional.

`br` fica fora de P1168. P1171 deve introduzir uma tabela declarativa void e
então exigir `<br>` sem end tag. O estado P1166 `<br></br>` fica registado como
gap conhecido, não como comportamento legitimado.

### P1168 — serialização de atributo vazio

Medição diferencial posterior confirmou que tanto
`html.elem("div", attrs: (hidden: ""))` quanto `html.div(hidden: true)` são
emitidos pelo vanilla como `<div hidden>`, sem `=""`. O exporter deve portanto
serializar qualquer valor de atributo vazio somente como o nome; valores não
vazios conservam `name="escaped"`. Esta é correção interna de paridade do
exporter, sem novo contrato, e segue em fluxo contínuo pelo ADR-0127.

## P1171 — tags void e whitespace local (MATERIALIZADO EM 2026-08-25)

### Medição

O vanilla `tag.rs:123-141` possui 13 tags void. Sondas confirmaram `<br>` e
`<br id="b">`, sem end tag ou slash. Um `HtmlElem` void com body pode existir
após eval genérico, mas o export falha com
`HTML void elements must not have children`. O cristalino atualmente emite
`<br></br>` e `<br>X</br>`.

Repr diferencial mostrou que newlines entre expressões já são
`Content::Space` nos dois compiladores. Portanto o gap formatado não nasce no
parser/eval. Na expansão vanilla, espaços de borda dentro de elemento normal
são removidos, espaço entre dois inline é preservado, espaço entre blocos não
é emitido e whitespace em torno de `br` no caso medido resulta `A<br>B`.

### Decisão proposta

Manter tabela interna única com as 13 tags void. Ao serializar `HtmlElem` void,
body unset/none emite somente `<tag attrs>` e body content retorna o diagnóstico
medido. Atributo vazio, escaping e ordem seguem as regras existentes.

Normalizar whitespace de `HtmlElem` no boundary L3: remover espaços somente
nas bordas do body e os adjacentes a void `br`, preservar espaço semântico
entre siblings inline e não criar whitespace entre siblings block. Não aplicar
`trim()` indiscriminado ao interior de texto/raw. Isto não muda `HtmlElem`,
eval nem pipeline. Agrupamento phrasing de topo permanece fora.

## P1172 — agrupamento phrasing no topo (MATERIALIZADO EM P1172.1)

### Medição

`typst-html/src/tag.rs:290-350` enumera phrasing content e `:504-546` decide
agrupar tags phrasing cujo display default não seja none. As exclusões
display-none relevantes vêm de `property.rs:62-76`: `area`, `datalist`,
`link`, `meta`, `script`, `template`. `typst-realize/src/lib.rs:1043-1065`
trata HtmlElem groupable como trigger de parágrafo.

Sondas confirmaram que `html.a`, `html.elem("a")` e `html.span` isolados no
topo viram filhos de `<p>`. A origem do constructor não importa. Sequência
`span/div/a` produz `<p><span>…</span></p><div>…</div><p><a>…</a></p>`;
parbreak separa dois parágrafos; os mesmos phrasing dentro de `div` não ganham
wrapper. O vanilla agrupa `a` incondicionalmente mesmo com body block, conforme
a intenção explícita da fonte.

Repr/Content vanilla e cristalino coincidem. O gap está em L3:
`block_sequence` cristalino trata todo `HtmlElem` como boundary block.

### Decisão

Adicionar classificação pura `should_group_into_paragraph(tag)` no owner L3,
copiando o conjunto de phrasing visível medido. No contexto top-level,
HtmlElem groupable alimenta o buffer de parágrafo; HtmlElem não groupable
fecha o buffer e é emitido diretamente. Parbreak também fecha. Dentro do body
de HtmlElem, preservar serialização inline existente sem criar `<p>`.

Tags desconhecidas/custom não são groupable por default. Não adicionar campo
a `HtmlElem`, não mover realização para eval e não importar `lab`. Whitespace
continua regido por P1171.1. Esta é correção interna de paridade no exporter
existente e segue sem gate pelo ADR-0127.

## P1173 — whitespace junto a block siblings (APROVADO EM 2026-08-25)

Na fixture `article[address … aside …]` formatada, vanilla emitiu os dois
block siblings sem espaço; o cristalino preservou `Content::Space` entre eles.
A versão compacta coincide e repr preserva Space nos dois, logo a causa
continua na normalização L3.

Estender a classificação de display block no exporter para remover
`Content::Space` imediatamente antes/depois de HtmlElem block dentro de bodies,
como P1171.1 já faz para void `br`. No recorte P1173.1, validar `address`,
`article` e `aside`, sem inferir custom tag como block. Preservar espaço entre
phrasing siblings e bordas já fechadas. É correção interna de paridade, sem
mudança de contrato ou fase.
