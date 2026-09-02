# Prompt L0 — `infra/integration_tests` — suíte E2E L3
Hash do Código: 99985da2


**Camada:** L3, somente `#[cfg(test)]`
**Ficheiro proprietário:** `03_infra/src/integration_tests.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0129

## Responsabilidade

Esta suíte exercita o pipeline real por fronteiras que mocks de L1 não cobrem:
`SystemWorld`, filesystem temporário, eval, introspecção, layout e export. Seus
testes integram módulos produtivos já especificados por seus próprios owners; este
L0 legitima apenas o harness e os testes no consumer proprietário.

Helpers test-only constroem e removem diretórios temporários, criam um
`SystemWorld`, avaliam fontes, atravessam frames e compilam documentos até os
artefatos de export. Filesystem, relógio e world concreto são permitidos aqui por
se tratar de consumer L3 test-only; essa permissão não se transfere para L1.

## Observáveis e paridade

Os testes podem verificar valores produzidos por eval, morfologia e layout,
warnings e mensagens de erro quando são observáveis, além da validade e das
estruturas observáveis dos formatos exportados. Segundo ADR-0107/0108, igualdade
acidental de bytes, estrutura interna ou passos do algoritmo não vira contrato,
exceto quando bytes ou estrutura são precisamente o observável do formato sob
teste. Toda medição decisória registra proveniência reproduzível.

## P1293.final — observação de `FrameItem` em coordenadas globais

### Medição anterior à decisão

O recibo adversarial P1293 de SHA-256
`8c742753c64524578f08151e604668d047be23a526d9acac9088398870f79868`
mediu `cargo test -p typst-infra --lib` em
`2026-09-02T16:38:34-03:00`: `910` testes passaram e `8` falharam nas linhas
`4650`, `4695`, `4737`, `4795`, `4892`, `4957`, `5008` e `5045` do consumer
SHA-256
`6377db547065c4fa912924bfae88d6c0e296b1446fd55571f57c2a43783d0086`.
Todos esses casos obtêm itens pelo helper privado
`03_infra/src/integration_tests.rs:34-51`.

Esse helper atravessa `Semantic`, `Group` e `Link`, mas devolve referências aos
filhos e deixa os consumidores lerem `pos`/`start` locais sem carregar a
transformação ancestral. A fonte de tipos declara em
`01_core/src/entities/layout_types.rs:484-502` que `Group.pos` está no espaço
do pai, `matrix` é a transformação afim e seus `items` estão no espaço local.
O exporter SVG confirma em `03_infra/src/export/svg.rs:1638-1682` a ordem
observável `translate(Group.pos)` seguida de `matrix`. A medição causal do
adversário mostra que o `Group` de Formula conserva o render global e tornou
locais apenas as coordenadas dos filhos; portanto os oito valores locais não
podem ser comparados diretamente às expectativas globais históricas.

Medição, não inferência: o observador perde `Group.pos`/`matrix` durante a
descida. Inferência: compor a transformação como o exporter restaura as
coordenadas de página já esperadas sem alterar layout. Refutador: depois da
composição completa, qualquer um dos oito valores globais ainda divergir,
uma quantidade/ordem de item mudar ou uma transformação não poder ser
representada pelo carrier existente; nesse caso o helper não mascara a falha
e o owner produtivo causal precisa ser reaberto antes de produção.

### Decisão test-only

Os helpers privados que inspecionam frames devem carregar, durante toda a
descida, a transformação afim acumulada do ancestral até o item observado.
Ao entrar em `FrameItem::Group`, devem compor na ordem do exporter a
translação de `Group.pos`, a `Group.matrix` e o transform já acumulado; grupos
aninhados devem compor transitivamente. Pontos de `Text`, `TextShaped`,
`Glyph`, `Image` e `Shape`, extremos de `Line` e qualquer coordenada usada em
asserção devem ser projetados uma única vez nesse referencial global antes da
comparação. `Semantic` continua envelope transparente; `Link` conserva a sua
semântica vigente e não autoriza deslocamento inventado ou dupla aplicação.

Travessias que contam ou classificam itens podem continuar a observar a
identidade da variante, mas toda comparação geométrica usa a projeção global.
As expectativas numéricas, fixtures, fontes, layout produtivo e ordem dos
itens permanecem byte-conceitualmente inalterados. É proibido acomodar a
mecânica local alterando os oito valores esperados, revertendo o `Group` de
Formula ou tocando `compiler/layout/equation.md`/`equation.rs`.

Classificação ADR-0107/0108: coordenadas globais renderizadas são o observável
de layout; referência Rust, recursão e composição do helper são transporte de
teste. Esta decisão decorre das linhas medidas acima. ADR-0127: correção
test-only do observador, sem API pública, default, fase ou compatibilidade;
fluxo contínuo após resselo. ADR-0129: este owner continua 1:1 com
`03_infra/src/integration_tests.rs` e não legitima nenhum módulo produtivo.

## Proveniência P844

P844 acrescentou os testes `p844_a1_...` a `p844_a8_...` e os helpers
`p844_expand_plain_text`/`p844_expand_errors`, seguindo P506/P821. O lote cobriu
query→content, seletor por função de elemento, `state.at`, `state.final`,
`counter.final`, `counter.at(Location)`, repr de array em `#context`,
`counter.display` com numbering do `#set` e pattern real, além da sonda de
`#context` entre headings via expansão e reintrospecção. Essa proveniência não
pretende enumerar ou esgotar a suíte vigente.

## Aceitação estrutural

- o módulo é compilado somente sob `cfg(test)` pela raiz da crate;
- helpers permanecem privados ao harness;
- os testes selecionados da suíte compilam e passam sem alterar contratos
  produtivos para acomodar mecânica de teste.

## Fora de escopo

Não pertencem a este owner os contratos dos módulos produtivos exercitados, unit
tests internos desses módulos, fixtures externas ou comportamento novo do produto.
Cada correção funcional descoberta pela suíte exige seu próprio L0 e classificação
ADR-0127 antes de alterar produção.
