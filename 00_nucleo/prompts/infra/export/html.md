# Prompt L0 — exportação HTML semântica
Hash do Código: 5b4f56d5

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

## P1173 — whitespace junto a block siblings (MATERIALIZADO EM P1173.1)

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

## P1175 — proteção de espaço junto a inline vazio (MATERIALIZADO EM P1175.1)

Na fixture com dois `picture` vazios e, como contraprova, com dois `span`
vazios, o vanilla ratificado protegeu o `Content::Space` intermediário como
`<span style="white-space: pre-wrap">&#x20;</span>`. O cristalino emitiu espaço
literal. Portanto a diferença antecede P1175 e é transversal, não
particularidade de `picture`.

`convert.rs:33-55` define que um espaço ASCII isolado deve ser protegido quando
não há elementos normais que o sustentem dos dois lados. O passe
`convert.rs:597-681` percorre inline descendants: texto/conteúdo visível e
replaced elements sustentam espaço; inline vazio não sustenta; block e `br`
colapsam a fronteira conforme `tag.rs:496-503`.

Estender o exporter para proteger um `Content::Space` isolado em contexto
block/paragraph quando ele não possui conteúdo visível supportive de ambos os
lados, emitindo exatamente o span `white-space: pre-wrap` e `&#x20;`. A análise
deve atravessar HtmlElem inline, ignorar elementos vazios, respeitar boundaries
block/`br` e não envolver o espaço normal entre siblings inline com conteúdo.
Não alterar `HtmlElem`, constructor, target, pipeline nem a serialização de
espaços dentro de `pre`. É correção interna de paridade e segue RED→GREEN em
fluxo contínuo pelo ADR-0127.

## P1176 — `summary` block e espaço junto a boundary de parágrafo (MATERIALIZADO EM P1176.1)

Fixtures tipadas/genéricas mediram duas divergências internas:

1. dentro de `details`, vanilla emite `<summary>Inside</summary>Body`, enquanto
   o cristalino preserva espaço; `property.rs:144-146` define `summary` block;
2. no topo, espaços antes/depois de `datalist` e `summary` não geram
   parágrafos, enquanto o cristalino pode convertê-los em spans pre-wrap
   isolados. `datalist` não é block: dentro de `div`, espaços ao redor dele
   coincidem e seguem a proteção inline normal.

Acrescentar `summary` à classificação block usada por
`collapses_adjacent_whitespace`. Separadamente, em `block_sequence`, descartar
`Content::Space` cuja procura à esquerda ou direita atinja um HtmlElem
não agrupável antes de encontrar conteúdo do parágrafo; não transformar esse
espaço de boundary em parágrafo nem em span protegido. Preservar a análise
inline de `datalist` dentro de bodies e a proteção entre dois `noscript`
vazios.

Não classificar `datalist` como block, não implementar CSS/display, scripting
ou validação de parent. São correções internas de paridade no exporter
existente, sem contrato, entidade, default ou mudança de fase; seguem
RED→GREEN em fluxo contínuo pelo ADR-0127.

## P1177 — whitespace estrutural dentro de `ruby` (MATERIALIZADO EM P1177.1)

Fixtures diferenciais tipada/genérica mediram que espaços comuns entre filhos
consecutivos `rp` e `rt` dentro de `ruby` são descartados pelo vanilla. Isso
inclui `rp[] rt[]` e `rp[(] rt[T] rp[)]`; o cristalino atualmente preserva o
espaço ou o protege como `white-space: pre-wrap`. Contraprovas mostram que
`ruby[A rt[T]]` preserva o espaço após a base e que
`ruby[A span[X] B]` preserva espaços em torno do `span`. Logo não se deve
remover todo whitespace do body de `ruby`.

Ao serializar o body de um `HtmlElem` cuja tag é `ruby`, descartar
`Content::Space` somente quando os vizinhos não-space imediatos são ambos
elementos `rp`/`rt`. Aplicar a regra antes da proteção de espaço vazio. Não
validar ordem, parent ou quantidade; não mudar agrupamento top-level, entidade,
eval, default ou fase. A regra é correção interna de paridade e foi
materializada em P1177.1 junto dos bindings após o gate público.

## P1178 — composição documental e `title` escapable-raw (MATERIALIZADO EM P1178.1)

### Medição anterior à decisão

`document.rs:262-310` mede a finalização do DOM pelo número de nós não
introspectivos. Se o único nó é `html`, ele vira a raiz diretamente, sem
inserção de `lang`, head ou metadata automáticos. Se o único nó é `body`, ele
é adotado e recebe ao redor o `html lang=locale` e o head automático. Um nó
`html` ou `body` acompanhado por qualquer outro nó falha com
`` `<TAG>` element must be the only element in the document ``. `head` não é
adotado: isolado ou repetido permanece dentro do body automático.

Fixtures ratificadas confirmaram:

- documento explícito completo:
  `<!DOCTYPE html><html><head><title>T</title></head><body>B</body></html>`;
- `body[B]` único conserva o envelope/head automáticos e substitui apenas o
  body gerado;
- `head[title[T]]` único é serializado dentro do body gerado;
- attrs do `html`/`body` adotado são preservados; `html(lang: "pt")` impede o
  `lang="en"` automático porque a raiz inteira é explícita;
- dois `head` são permitidos como nós comuns; dois `body`, ou `html` com
  siblings, acionam o erro de exclusividade.

`convert.rs:176-185,279-282` realiza o body de `title` com whitespace Pre.
`encode.rs:154-158,252-283` exige somente filhos textuais e usa codificação
escapable-raw. As sondas preservaram dois espaços e newline; linebreak virou
newline; `&` e `<` foram escapados, enquanto `>` e `"` permaneceram literais.
Um `span` filho falhou com `HTML raw text element cannot have non-text
children`. Chamada vazia emitiu `<title></title>`.

### Decisão proposta

Antes de gerar o envelope normal, classificar os nós top-level ignorando
somente tags introspectivas, conforme a fonte medida:

1. único `HtmlElem("html")`: emitir esse elemento como raiz depois do doctype,
   sem envelope, lang ou metadata adicionais;
2. único `HtmlElem("body")`: gerar `html` + head automático e usar esse nó
   como body, preservando attrs/body;
3. qualquer `html`/`body` quando há outro nó: retornar exatamente o erro de
   exclusividade medido;
4. `head`/`title` seguem como nós comuns e não são extraídos.

Ao serializar `title`, realizar seu body em modo pre: `Content::Space` é
literal, `Linebreak` vira newline e absorve um `Content::Space` imediatamente
seguinte (separador sintático medido em `A\\ B`); texto não sofre
trim/colapso. Aceitar
somente conteúdo que resulte em texto; HtmlElem ou outro nó não textual retorna
o diagnóstico medido. Escapar no texto somente caracteres inválidos no
contexto escapable-raw (`&` e `<` entre os ASCII medidos), preservando `>` e
aspas. O tratamento é contextual a `title`; não alterar escape normal de
elementos ou atributos.

`html` e `body` continuam block boundaries; `head` e `title` continuam
não agrupáveis por display none. Não acrescentar estado a `HtmlElem`, não
validar parent no eval e não mover fase. Estas são regras internas de paridade
L3 e foram materializadas em P1178.1 após o gate dos bindings públicos.

## P1293.reopen-C — modos explícitos de serialização e escaping contextual (PROPOSTO; STOP ADR-0127)

### Medição anterior à decisão

O recibo residual P1293/C SHA-256
`4545df3baa07d09c5c004a77002d18eeaedb47a22aa4883dc2bc3ba12323676a`
mede em `03_infra/src/export/html.rs:190-194,211-218,506-511` que o exporter
vigente usa uma função única e escapa `& < > "` tanto em texto quanto em
atributo. Para a fixture congelada, produz
`value="x&amp;&quot;&lt;&gt;"` e texto `&lt;&amp;&gt;`. A fonte ratificada separa
contextos em `encode.rs:99-133` e `charsets.rs:21-49`: no contexto normal,
atributo escapa `&` e `"`, texto escapa `&` e `<`, produzindo
`value="x&amp;&quot;<>"` e texto `&lt;&amp;>`. A validação WHATWG registrada pelo
coordenador confirma que ambas são HTML válido: a primeira é conservadora e
permanece o padrão cristalino; a segunda é a alternativa de paridade vanilla.

### Contrato público proposto

Este owner define o enum L3 único:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtmlSerializationMode { Crystalline, Vanilla }

pub fn export_html_with_serialization(
    content: &Content,
    mode: HtmlSerializationMode,
) -> Result<String, SourceDiagnostic>;
```

`export_html(content)` permanece API compatível e equivale exatamente a
`export_html_with_serialization(content, HtmlSerializationMode::Crystalline)`.
O modo `Crystalline` preserva os bytes atuais e o escape conservador `& < > "`
nos contextos normais. O modo `Vanilla` usa a seleção contextual observada:
atributo normal escapa `&` e `"`; texto normal escapa `&` e `<`. Isto não
autoriza copiar whitespace, IDs ou mecânica interna do vanilla.

Todos os demais contratos do exporter são comuns aos dois modos: DOM ordenado,
atributo Presence, void, nesting, whitespace estrutural, documento explícito,
`title` escapable-raw, diagnósticos e subset suportado. O modo não muda
`Content`, `HtmlElem`, feature, target nem fase e não pode ser inferido por tag,
texto, path ou variável global.

A API pública nova e o default user-facing preservado exigem confirmação
humana ADR-0127. Após confirmação, REDs devem distinguir default/Crystalline da
forma Vanilla com a fixture medida, provar ambos como DOM equivalente e manter
regressões de void/whitespace/title. Qualquer necessidade de campo em entidade,
estado global ou caminho pelo layout paginado refuta esta proposta e obriga
nova parada.
