# Prompt L0 — export HTML: document
Hash do Código: dabfbe2a

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/export/html/document.rs`
**Atualizado em:** 2026-09-26

## Medição anterior à decisão

No snapshot anterior `03_infra/src/export/html.rs:190-248 compõe o documento; :417-512 adota raiz explícita e agrupa blocos.`
A localização é mecânica L3 (ADR-0109); preservar a morfologia HTML e
mensagens de erro observáveis (ADR-0107), sem inferir intenção do produto.

## Responsabilidade e invariantes vigentes

Possui envelope/doctype automáticos, export_html_with_frame_resolver, explicit_document_root e block_sequence. Mantém a cadeia inicial de estilos, a adoção exclusiva de html/body, atributos e metadata atuais; head continua nó comum. Preserva flush, trim e fronteiras de parágrafo, incluindo tabelas e HtmlFrame block-level. Pretty ocorre somente após produzir a string compacta. Não muda target, feature, eval ou layout.

Módulo privado descendente de export::html; helpers usados entre irmãos
empregam pub(super), sem pub(crate), API nova ou mudança de fase. Mantém as
assinaturas, algoritmos e diagnósticos do snapshot medido, incluindo
adaptação local existente a Content::Styled(styled). A extração é interna
e segue fluxo contínuo ADR-0127; cada consumer possui owner 1:1 ADR-0129.

## Aceitação

Testes funcionais existentes e novas caracterizações passam antes/depois;
comparação de assinatura/corpo detecta deriva na movimentação. Não afirmar
ganho de tempo nem paridade nova a partir desta reorganização.

## Proveniência histórica das regras transferidas

As medições abaixo pertencem aos pins e passos históricos citados; não
constituem nova medição contra upstream atual. Estados de proposta e scope-out
antigos são superados pelos invariantes vigentes acima e cortes posteriores.

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
