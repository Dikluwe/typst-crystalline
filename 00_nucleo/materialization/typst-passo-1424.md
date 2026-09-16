# Passo 1424 — `html.frame` como ilha paginada em SVG inline

## Estado medido antes da decisão

O P1423 materializou os 112 constructors derivados da tabela HTML e reservou
`html.frame`, que não é uma tag. A fonte vanilla ratificada confirma a unidade
separada:

- `typst-html/src/format.rs:268-281` define `FrameElem { body: Content }`, com
  body posicional obrigatório;
- `typst-html/src/convert.rs:140-154` muda o target interno para `Paged`, faz
  layout do body numa região ilimitada e cria um frame HTML block-level;
- `typst-html/src/rules.rs:88-91` torna o elemento transparente no target
  paginado, devolvendo o body;
- `typst-html/src/encode.rs:399-422` emite o frame resolvido como SVG inline.

Sondas executadas em 2026-09-16T13:44:19-03:00, no HEAD
`90ebf6643dbd9070c1018f8dd8748b867ec12520`, tree
`eb838a3b7440fa10d9732e4eecfa7cf893bd1a69`, working tree limpa antes das
sondas. Vanilla `/usr/local/bin/typst`, SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`,
reportou `typst 0.15.1 (586e1bd4)`:

| Fixture | Target | Resultado |
|---|---|---|
| `#html.frame([x])` | HTML | exit 0, 1817 bytes, SHA-256 `56873e737cb3bcc4ffdd23ea131357013f19395065152e2ac809da0a13d208da`, raiz útil `<svg ... style="... display: block">` |
| `#html.frame([#rect(width: 10pt, height: 5pt, fill: red)])` | HTML | exit 0, 480 bytes, SHA-256 `bea5cb973cec51717db5dc547fbe7649c91d4650c1f27066a4501599bd31112c`, `viewBox="0 0 10 5"` |
| `#html.frame([#html.frame([x])])` | HTML | byte-idêntico ao frame simples |
| `#html.frame([x])` | SVG paginado | byte-idêntico a `x` sem wrapper próprio |
| `#html.frame()` | HTML | erro `missing argument: body` |
| `#html.frame(body: [x])` | HTML | erro ``the argument `body` is positional`` |
| `#html.frame([x], [y])` | HTML | erro `unexpected argument` |

No mesmo estado, o binário cristalino release SHA-256
`66dc3120c2b97c69fcc7be941d385d05cbd947a67e951464654789f4f70ba6c3`
falha a fixture válida com `module html does not contain frame`. A alternativa
`html.elem("frame")` permanece uma tag literal e não satisfaz a morfologia.

## Decisão proposta

Materializar `html.frame(body)` como primitiva pública feature-gated, distinta
de `HtmlElem`:

1. L1 preserva `HtmlFrameElem { body: Content }` em
   `Content::HtmlFrame(Arc<HtmlFrameElem>)` e cobre todos os braços exaustivos;
2. `html.frame` aceita exatamente um Content posicional obrigatório, sem named;
3. layout paginado delega transparentemente ao body, inclusive em nesting;
4. a pipeline HTML, depois da preparação contextual, resolve cada ocorrência
   sob target interno `Paged`, com estilos efetivos da ocorrência e região
   intrínseca não paginada;
5. o resultado é SVG inline transparente, block-level por defeito, sem tag
   `<frame>` e sem wrapper de página A4;
6. a emissão reutiliza o renderer SVG cristalino e sua resolução de fontes;
   não duplica um segundo renderer no exporter HTML.

O observável de paridade é a linguagem: presença do binding, assinatura,
transparência paginada, SVG inline, geometria/dimensões, pintura, texto e
nesting. IDs de símbolos, ordem mecânica de atributos SVG e bytes não são
igualdade exigida, salvo diagnósticos públicos e fixtures que deliberadamente
congelam sintaxe.

## Limites explícitos

P1424 cobre o modo block-level padrão. A supressão de `display: block` quando o
frame é envolvido por `box`, CSS gerado, âncoras/link targets dentro do SVG,
bundle, pretty específico dentro do SVG e igualdade textual de paths/IDs ficam
fora; qualquer um deles exige medição e passo próprio. Conteúdo que o layout ou
SVG cristalino ainda não suporta permanece erro/Unknown, nunca sucesso vazio.

## Gate ADR-0127

Este passo adiciona função pública, tipo/campo/variante públicos, uma entry
point de layout de fragmento e a fase HTML
`preparação semântica → resolução de frames paginados → export`. Incide nos
itens 1, 3 e 4 do ADR-0127.

Fluxo obrigatório:

1. atualizar os L0 proprietários antes do código;
2. commitar esta proposta;
3. **PARAR sem alterar L1/L3 nem headers de linhagem**;
4. continuar somente após confirmação explícita do dono para P1424;
5. após confirmação, produzir contrato, oráculos, ataques e testes A/B
   segregados, selar, obter RED e só então implementar.

## Regime posterior ao gate

Usar protocolo completo `tekt-materializacao-segregada`, declarando
“executado sem atestação de isolamento” no workspace partilhado. O contrato
deve distinguir `html.frame` de `html.elem("frame")`, proteger as assinaturas,
os dois targets, nesting, geometria e falhas. Mutation score exigido: `1.0`;
`Unknown` nunca recebe crédito.

## Gates de fecho após confirmação

- RED do binding e da variante/carrier;
- RED dos três diagnósticos de assinatura;
- GREEN L1 focal para construção, walkers, repr e paged transparency;
- GREEN integrado HTML para texto, rect 10pt×5pt e nesting;
- controle literal `html.elem("frame")` preservado;
- repetição direta, repetição inversa e ausência de deriva das entradas seladas;
- `cargo fmt --all -- --check`, testes focais, `cargo build`,
  `crystalline-lint .` e `git diff --check`.

O fecho atesta somente o fragmento P1424 e não paridade HTML/SVG geral.
