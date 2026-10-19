# Prompt L0 — export HTML: inline
Hash do Código: f1c97acd

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/export/html/inline.rs`
**Atualizado em:** 2026-09-26

## Medição anterior à decisão

No snapshot anterior `03_infra/src/export/html.rs:518-610 faz despacho Content e serialização de HtmlElem, Styled e HtmlFrame.`
A localização é mecânica L3 (ADR-0109); preservar a morfologia HTML e
mensagens de erro observáveis (ADR-0107), sem inferir intenção do produto.

## Responsabilidade e invariantes vigentes

Possui inline: texto, sequência, Strong/Emph, Styled com cadeia acumulada, linebreak, wrappers de célula/header/footer, HtmlElem e HtmlFrame. Preserva o match estático e seu fallback de conteúdo não suportado. Atributos mantêm ordem; valores vazios emitem só nome. Void rejeita body content mesmo vazio e não emite fecho. Title/raw delegam ao encoding, body normal ao whitespace, Table ao table. Frame sem resolvedor falha com diagnóstico atual; com resolvedor insere o SVG recebido sem escape nem layout. Não alterar HtmlElem, StyleChain ou o contrato público HtmlFrameResolver.

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

## P1424 — slot de SVG inline resolvido (APROVADO NO GATE ADR-0127 EM 2026-09-16)

### Medição anterior à decisão

O vanilla codifica `HtmlFrame` como nó SVG inline, sem escape textual e sem tag
`frame`. O exporter cristalino recebe hoje apenas `Content` sem capacidade de
layout; fazê-lo criar layout quebraria a separação L3 e duplicaria a pipeline.

### Contrato proposto

O exporter aceita, por entry point interna explícita, um resolvedor read-only
de ocorrências `HtmlFrame`. Ao visitar uma ocorrência, insere exatamente o SVG
validado fornecido pela pipeline como markup inline, preservando sua posição no
DOM. `export_html*` sem resolvedor conserva a API e retorna diagnóstico
explícito se receber frame não resolvido.

O SVG não passa por escape HTML, não é embrulhado em `<p>` e é block-level por
defeito. Texto/attrs HTML comuns, `html.elem("frame")`, modos de escaping e
pretty externo permanecem inalterados. O exporter não chama layout, World nem
filesystem. A nova integração de fase foi confirmada pelo dono em 2026-09-16
e fica limitada pelo escopo de P1424.
