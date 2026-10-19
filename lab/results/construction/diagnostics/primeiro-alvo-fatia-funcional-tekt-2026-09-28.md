# Primeiro alvo: a fatia funcional inicial do Tekt

A frente escolhida é o percurso texto → conteúdo → layout → PDF, equivalente à primeira fatia do protótipo vanilla de 2019. A auditoria já encontrou um alvo concreto dentro dela: **preservação causal dos separadores de texto**, além do avanço visual que eles ocupam.

## Proveniência da medição

- Working tree não commitado sobre HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`. Lista exata de alterações, `git diff HEAD --stat`, hashes das fontes e timestamps UTC: [proveniencia.json](fatia-funcional-tekt-2026-09-28/proveniencia.json).
- Tekt reconstruído com `cargo build --release --offline -p typst-wiring --bin typst`, sucesso. O build emitiu warnings existentes; sucesso de build não prova completude ou ausência de violations.
- Oráculo atual: vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, conforme proveniência ratificada. A base de 2019 determina onde começar a história; as sondas de paridade usam o pin vigente.
- Mesma fonte explícita Noto Sans, fixture `03_infra/fixtures/fonts/NotoSans-Regular.ttf`, mesmos parâmetros de texto e página. Os comandos completos e hashes dos PDFs estão nos resultados. A flag font-path foi compartilhada; não foi usada uma flag de isolamento de fontes do sistema. Os casos não exigem fallback além da fonte escolhida.
- Não há benchmark nesta rodada. Posições/linhas/páginas são observações dos PDFs arquivados, extraídas por `pdftotext -bbox-layout`; não são métricas de velocidade nem prova universal sobre todos os leitores PDF.
- A rodada inicial alterou somente diagnósticos; o código e o L0 usados naquela medição estão registrados na sua proveniência. A materialização posterior está separada no diagnóstico de correção.

## Medição antes da classificação

As [dez sondas iniciais](fatia-funcional-tekt-2026-09-28/resultados.json) cobrem ASCII, acentos, newline, parágrafo, interpolação, caracteres delimitadores de PDF, espaços, quebra de linha, palavra longa e letras de larguras diferentes. Houve coincidência do texto esperado em nove casos e divergência no caso de espaços entre letras curtas. Os [quatro controles](fatia-funcional-tekt-2026-09-28/controles.json) distinguem separação real de fragmentos adjacentes. No conjunto são quatorze entradas, três divergências de extração, associadas ao mesmo candidato de limitação; não são três defeitos independentes.

| Entrada, depois dos mesmos parâmetros de fonte/página | Extração Tekt | Extração vanilla | Resultado |
|---|---|---|---|
| `A  B   C` | `ABC` | `A B C` | Divergência |
| `A B C` | `ABC` | `A B C` | Divergência |
| `#text("A B C")` | `ABC` | `A B C` | Divergência |
| `A#("B")C` | `ABC` | `ABC` | Controle de adjacência passou |
| `ALPHA BETA GAMMA` | `ALPHA BETA GAMMA` | `ALPHA BETA GAMMA` | Dependência do caso/extrator |

No primeiro caso, o stream Tekt contém três glifos e ajustes numéricos de distância entre eles:

```text
[ <0001> 0 -260 <0002> 0 -260 <0003> 0 ] TJ
```

O stream vanilla contém também glifos de espaço no seu texto codificado. Os streams relevantes foram preservados em [Tekt](fatia-funcional-tekt-2026-09-28/07-spaces-tekt.content-stream.txt) e [vanilla](fatia-funcional-tekt-2026-09-28/07-spaces-vanilla.content-stream.txt). Essa diferença de mecânica localiza o candidato; não exige igualdade de streams como aceitação.

O bounding box horizontal do conjunto tem os mesmos extremos na medição: `20.0` até `49.292` pontos. O extrator agrupa o resultado Tekt como uma palavra e o vanilla como três. Essa observação é compatível com separação visual correta e separadores de texto dependentes de inferência do leitor. Os resultados não provam identidade de todos os glifos, visual completo ou paridade geral da geometria.

## Rastreamento até a primeira implementação

1. `01_core/src/compiler/eval/mod.rs:1473–1477` conserva espaços lexicais como `Content::Space` ou `SpaceFromNewline`.
2. `01_core/src/compiler/layout/mod.rs:1516` despacha ambos para o adapter Space.
3. `01_core/src/compiler/layout/space.rs:9–23` transforma o espaço em avanço de cursor e oportunidade de justificação; não acrescenta item textual que identifique o separador.
4. O tratamento de espaços dentro de `Content::Text`, em `01_core/src/compiler/layout/text.rs:229–274`, também avança cursor entre fragmentos. Portanto o caso `#text("A B C")` precisa de inspeção junto com o adapter; corrigir apenas Space não cobriria todos os caminhos medidos.
5. `03_infra/src/export/stream/text.rs:655,696` permite agrupamento de runs compatíveis e representa distâncias absolutas entre fragmentos por ajustes TJ. Esse owner recebe posições e fragmentos, mas a distância sozinha não distingue espaço lexical, tracking, kerning ou posicionamento.

A transformação de Space somente em avanço já aparece no snapshot Tekt P10–P23 de março: [layout.rs:96](comparacao-base-2019-tekt-2026-09-28/tekt-p10-p23/01_core/src/rules/layout.rs.txt). Na base vanilla de 2019, o gerador conserva um caractere espaço entre palavras na string do documento: [doc.rs:136](comparacao-base-2019-tekt-2026-09-28/vanilla-2019/src/doc.rs.txt). Esta é uma diferença de construção da primeira fatia, independentemente de quando o agrupamento posterior tornou a extração mais sensível.

**Limite causal:** a fonte localiza onde os separadores deixam de ser itens textuais, e as sondas comprovam a divergência atual. Não foi feito bisect dos binários históricos para datar a primeira manifestação do problema no PDF. O agrupamento posterior e as heurísticas do leitor podem contribuir; a perda da proveniência não deve ser atribuída exclusivamente ao exportador por conveniência.

## Por que os testes locais podem esconder isso

`Page::plain_text()` e `Frame::plain_text()` coletam textos dos itens e aplicam `parts.join(" ")` em `01_core/src/entities/layout_types.rs:903–934`. Esses espaços são inseridos pelo helper. Logo ele pode produzir `A B C` a partir de fragmentos cujo payload não conserva separadores reais; também pode inserir espaços entre fragmentos semanticamente adjacentes.

O teste `pipeline_parse_eval_layout` continua em `01_core/src/compiler/layout/tests.rs:1515`, com aceitação parcial de “Olá” ou “mundo”. Ele termina no layout. Os testes e a extração de PDF são fronteiras de evidência diferentes.

**Classificação (ADR-0107/0108):** fragmentação de buffers, quantidade de glifos, operadores TJ e junção do helper são mecânica. Preservação das palavras e dos separadores de conteúdo para uso do documento é o observável a investigar. A divergência com um extrator foi reproduzida; a generalização para toda leitura/cópia/acessibilidade ainda exige outras verificações. Não se deve concluir que todos os espaços ou todo o Unicode atual estão perdidos: os controles contradizem essa generalização.

## Primeiro trabalho de materialização a derivar desta auditoria

O alvo é preservar a proveniência do separador ao atravessar layout, shaping e exportação, mantendo a geometria medida. A necessidade não é inferir um caractere a partir de qualquer distância positiva: o controle de adjacência exige distinguir separação lexical de fragmentação interna.

Os critérios para avaliar uma proposta concreta são:

- Fonte com separadores conserva separação de palavras na saída; fragmentos adjacentes continuam adjacentes.
- Espaços vindos de markup, newline e strings percorrem casos próprios, incluindo mudanças de estilo, início/fim de linha e quebra de região.
- Geometria, justificação, tracking, kerning, conteúdo semântico e fontes mantêm suas responsabilidades causais.
- A verificação atravessa fonte até PDF e identifica a fase da divergência; o helper que insere separadores não substitui essa prova.

Antes de materializar uma solução, ler e atualizar os L0 dos consumers que a decisão realmente afetar. Foram lidos nesta rodada `compiler/layout/space.md`, `compiler/layout/text.md`, `infra/export/stream/text.md` e `infra/pipeline/pdf.md`. O L0 Space especifica avanço e supressão inicial; o L0 Text preserva divisão/avanço atuais; o L0 Stream especifica agrupamento e fronteiras TJ. Esta análise é diagnóstico, não L0 que legitima código.

Uma correção interna comprovada de paridade segue fluxo contínuo conforme ADR-0127. Se a solução exigir novo campo/variante público, assinatura ou mudança de fase, sua especificação concreta passa pelo gate correspondente. Na conclusão da auditoria inicial, ainda não havia solução de representação materializada. Após aprovação explícita do contrato público, a correção e a atomização foram implementadas; ver [resultado da materialização](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/atomizacao-separadores-2026-09-28.md). A evidência inicial abaixo permanece histórica.
