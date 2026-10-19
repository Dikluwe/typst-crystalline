# Prompt L0 — export HTML: whitespace
Hash do Código: 088d8a43

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/export/html/whitespace.rs`
**Atualizado em:** 2026-09-26

## Medição anterior à decisão

No snapshot anterior `03_infra/src/export/html.rs:737-898 normaliza bordas e espaços inline/top-level/ruby; :70 define o span protegido.`
A localização é mecânica L3 (ADR-0109); preservar a morfologia HTML e
mensagens de erro observáveis (ADR-0107), sem inferir intenção do produto.

## Responsabilidade e invariantes vigentes

Possui inline_html_body, is_ruby_annotation, space_between, top_level_space_between, SpaceSupport e seus predicados. Preserva bordas de Sequence, descarte junto a block/br, distinção Boundary/EmptyInline/Supportive e span protegido exato. Ruby só descarta espaço entre vizinhos imediatos rp/rt, como no código atual. Predicados atravessam Styled e wrappers inline; replaced tags sustentam espaço. SpaceFromNewline recebe projeção idêntica a Space; não introduzir heurística CJK no exporter. Usa classificação estática do módulo tags e chama inline para filhos.

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

## P1494 — projeção neutra do espaço inferido (RATIFICADO; gate ADR-0127 confirmado em 2026-09-24)

### Medição anterior à decisão

No HEAD 6da06e849d75972651a342ad6aad337313451c79, o consumer
03_infra/src/export/html.rs não tem diff. Ele reconhece Content::Space
na escolha da raiz explícita (:420-425), no agrupamento de parágrafo
(:499), na emissão inline (:520), em raw/title (:609,703-710), e nos
predicados de espaços adjacentes e bordas (:738-877). O caso bilateral
P1489-S-001 mostra que o vanilla emite CJK_JOINED para ideogramas
separados por newline e CJK_EXPLICIT_SPACE para espaço explícito,
enquanto o cristalino atual só emite a segunda sentinela. A medição de
eval/mod.rs:6970 e rules.rs:982-1055 situa a perda de origem e o
matching por folha antes da exportação. Inferência: uma correção de
whitespace CJK apenas no encoder esconderia o erro de show textual;
seria refutada por equivalência da morfologia L1 antes do exporter.

### Decisão ratificada

Em todos os ramos produtivos citados, SpaceFromNewline recebe exatamente
a mesma projeção local que Space: emissão de espaço, tratamento de borda,
adjacência block/void/ruby, raiz explícita, raw e title. Uma chamada
direta ao exporter com a variante ainda não normalizada conserva essa
equivalência. A normalização linguística do newline CJK e o matching
show ocorrem antes do exporter, no owner L1; este owner não examina os
caracteres CJK vizinhos para descartar espaço. As regras estruturais
P1171/P1173 e os modos Crystalline/Vanilla permanecem como estão.
Standalone HTML e HTML no bundle recebem a mesma morfologia já realizada;
o SVG de html.frame continua resolvido pela pipeline.

Testes distinguem projeção direta das duas variantes em top-level,
inline, raw/title e junto a block/void, além do caso integrado CJK.
O gate ADR-0127 da variante pública e da fase P1494 foi confirmado
pelo dono em 2026-09-24; esta seção secundária está autorizada.
