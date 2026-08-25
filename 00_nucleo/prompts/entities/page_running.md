# L0 — Running matter de página
Hash do Código: 577bd120

## Linhagem e alcance

Este prompt legitima o domínio e a composição marginal de páginas para
`numbering`, `number-align`, `header`, `header-ascent`, `footer` e
`footer-descent`. Não legitima a exposição pública de `page` ou `std.page`, nem
o `supplement` de referências.

Fonte medida: vanilla ratificado `a51e02804`, especialmente
`typst-library/src/layout/page.rs:315-450` e
`typst-layout/src/pages/run.rs:141-233`.

## Domínio

`PageMarginal` preserva três estados:

- `Auto`: produz numeração automática somente quando `numbering` existe e a
  componente vertical de `number-align` aponta para esta margem;
- `None`: suprime conteúdo marginal, inclusive numeração automática;
- `Content(Content)`: usa o conteúdo explícito e tem precedência sobre a
  numeração automática na mesma margem.

O delta de set-rule precisa ainda distinguir argumento omitido de cada estado.
Omitido preserva a configuração anterior; `auto` e `none` nunca colapsam.

`PageNumberAlign` contém alinhamento horizontal e posição vertical. O default é
`center + bottom`. A posição vertical aceita somente top ou bottom; horizon é
erro de linguagem.

`PageMarginalOffset` é comprimento relativo. Seu ratio resolve-se contra a
margem física correspondente: `header-ascent` contra top e
`footer-descent` contra bottom. O default de ambos é 30%. A parte absoluta é
somada depois da resolução percentual, sem constante empírica.

`numbering` preserva ausência e a representação de numbering suportada pela
linguagem. Uma string vazia não é atalho interno para `none`; o parser deve
seguir o contrato público de numbering/none. Funções só podem ser admitidas
quando o tipo de domínio e a chamada contextual estiverem especificados; não
podem ser silenciosamente convertidas em string.

## Transporte

`PageConfig` guarda os seis valores ativos. `Content::SetPage` e
`PageRunElem` guardam deltas opcionais e preservam os três estados marginais.
Clone, hash, igualdade, `map_content`, `map_text` e repr não podem perder nem
fundir estados. Um page-run instala uma cópia local e restaura toda a
configuração anterior em LIFO, inclusive quando vazio ou multipágina.

`Page` recebe snapshot visual resolvido por página: padrão de numbering e
listas distintas de header, body e footer. O número lógico é o controlado pelo
contador de páginas quando esse domínio estiver disponível; o número físico só
pode ser fallback explicitamente marcado e testado, nunca confundido com a
semântica final.

## Composição

Para cada página:

1. resolver margens físicas e offsets;
2. materializar a numeração marginal para top ou bottom;
3. escolher header/footer por precedência Content, None, Auto;
4. layout do header na largura interna e na região top menos ascent;
5. layout do footer na largura interna e na região bottom menos descent;
6. manter body independente e compor visualmente header, body, footer.

Conteúdo marginal explícito correspondente suprime a numeração automática;
conteúdo na margem oposta não a suprime. A numeração horizontal respeita
start/end segundo direção e binding vigentes.

Header, footer e numeração marginal são artefatos visuais repetidos. Não entram
no plain text do body, em query/introspecção como duplicatas, nem criam estrutura
acessível ou MCIDs duplicados. O conteúdo fonte continua semanticamente
observável em sua posição original quando aplicável.

## Atomização e pureza

O owner de domínio é `entities/page_running.rs`. A lógica visual pertence a
`compiler/layout/page_running.rs`, como free functions chamadas pelo núcleo
magro, forma B da ADR-0109. L1 não faz I/O, não consulta target e não introduz
despacho dinâmico.

## Aceitação no nível da linguagem

- default: center+bottom, offsets 30%, marginais Auto;
- top/bottom escolhem corretamente a margem da numeração;
- header/footer Content e None suprimem somente a numeração correspondente;
- percentuais dependem das margens efetivas, inclusive páginas com geometria
  e binding distintos;
- page-run restaura integralmente a configuração exterior;
- ordem e posição visuais são observáveis, mas bytes, estrutura Rust e passos
  internos não são critérios de paridade;
- nenhum argumento reconhecido é ignorado.

## P1160 — morfologia do callback realizado

`realized_numbering_layer` faz layout do `Content` devolvido pelo callback em
sub-frame e posiciona o grupo na margem. Não reduz o resultado a
`plain_text`: strong/emphasis, estilos e estrutura visual permanecem no
conteúdo marginal. `plain_text` é usado somente para estimar a largura de
alinhamento; não substitui os items produzidos pelo layout normal.
