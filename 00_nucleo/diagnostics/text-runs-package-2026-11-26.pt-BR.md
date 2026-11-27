# Proposta de pacote: texto adjacente como um só trecho e aspas no parágrafo

**26 de novembro de 2026 · aprovado pelo dono em 26/11/2026 (“Aprovo A–B”, no chat) sob a [ADR-0127](../adr/adr-0127-l0-gate-stop-vs-continuous-flow.md), em fluxo contínuo (paridade de geometria, sem contrato protegido)**

Escolhido pela revisão de L1 depois do [pacote de caminhos](paths-package-2026-11-26.pt-BR.md). É o maior grupo de diferenças visuais da suíte oficial com uma causa única em L1 que não cai na alteração de outra autoria (gradientes em `layout/decorations`).

## Os problemas

1. **Texto colado do mesmo estilo perde o kerning e as ligaduras** quando vem de nós de conteúdo diferentes. `A#[V]` sai 2,24 pt mais largo que `AV` a 20 pt, e `f#[i]` não forma a ligadura `fi`. No vanilla os dois saem iguais a `AV` e `fi`. A aspa automática é o caso mais comum: em `telephone'`, o par `e’` fica sem kerning.
2. **A linha pode quebrar depois de uma aspa de abertura automática.** Em `xx "yyyyyyyyyzzz` numa página estreita, o Tekt deixa `“` no fim da primeira linha e manda a palavra para a seguinte. O vanilla leva `“yyyyyyyyyzzz` inteiro para a linha seguinte. Com a aspa literal `“`, o Tekt acerta.

## Medição

HEAD `76d5778e4`, árvore com a alteração de outra autoria em `layout/decorations` (`git diff HEAD --stat`: 5 arquivos, nenhum de texto ou parágrafo). Cristalino debug `93de6f29844060f87b19db0ccc4985b920c9252c0ec975832c18be1222c09213`, o mesmo binário da medição do pacote de caminhos. Vanilla `lab/typst-original/target/release/typst`. Sondas de 18:20 a 19:10 de Brasília, extraídas por `pdftotext -bbox`.

| Fonte, 20 pt | Vanilla, fim da palavra | Cristalino |
|---|---|---|
| `telephone’` (aspa literal) | 95,36 | 95,36 |
| `telephone'` (aspa automática) | 95,36 | 95,84 |
| `telephone#[’]`, `#"telephone"#"’"` | 95,36 | 95,84 |
| `A#[V]` | 34,70 | 36,94 |
| `f#[i]` | 21,20, ligadura | 21,62, sem ligadura |
| `A#text(fill: red)[V]`, estilos diferentes | 36,94 | 36,94 |

| Fonte, página de 150 pt | Vanilla | Cristalino |
|---|---|---|
| `xx yyyyyyyyy#[zzz]` | `yyyyyyyyyzzz` na linha 2 | igual |
| `xx “yyyyyyyyyzzz` (aspa literal) | `“yyyyyyyyyzzz` na linha 2 | igual |
| `xx "yyyyyyyyyzzz` (aspa automática) | `“yyyyyyyyyzzz` na linha 2 | `“` na linha 1, `yyyyyyyyyzzz` na linha 2 |

Na suíte oficial (`pdf-paths.json`), `text/smartquote.typ` tem 26 testes com página diferente do vanilla. Os textos extraídos mostram os dois sintomas, por exemplo `‘ | telephone.’` contra `‘telephone.’`.

**Protótipo descartável.** Para medir o alcance da parte A antes de propor, fiz um protótipo numa worktree em RAM: em `word_layout::push_text`, o texto colado ao anterior, com o mesmo estilo e sem trecho bidirecional, entra no mesmo item. O binário tem SHA-256 `2a2cf255f93904d6fd7b1b95c8f61b7de38d1632698f33d5fd41d09f59584219`. A suíte rodou de 19:00:52 a 19:03:20 (`pdf-proto.json`, SHA-256 `876b960df32a2ac17a8fbe65a0f9c6849e756ab6d224e776f225ec75fac3e1c6`).

- **Melhoraram 10 testes:** 8 passaram a idênticos e 2 a quase idênticos. Entre eles estão `smartquote-prime`, `-bracket`, `-close-before-letter`, `-nesting-twice` e `-he`, além de `counter-figure`, `show-selector-basic`, `space-collapsing` e `newline-continuation-markup`.
- **Pioraram 3 testes:**
  - `bidi-nesting` ficou com página diferente: juntar texto dentro de um parágrafo RTL trocou a ordem visual.
  - `show-text-isolated-space` e `issue-5536-quote-inline-quotes-false` ficaram com diferenças de 6 e 2 pixels.
- **Parte B fora do protótipo:** ele não toca nela, e os demais testes de `smartquote.typ` seguem com a quebra depois da aspa.

O protótipo foi apagado. Ele não é a implementação proposta.

## Causa

- **Parte A:** cada palavra vira um `FrameItem::Text` próprio (`01_core/src/compiler/layout/word_layout.rs:92-150`), e o shaping acontece por item. Assim, dois nós de texto colados nunca são moldados juntos. O vanilla junta segmentos de texto adjacentes com os mesmos estilos antes do shaping (`lab/typst-original/crates/typst-layout/src/inline/collect.rs:409-415`).
- **Parte B:** o planejador de parágrafo coleta só texto, espaços, sequências, estilos, `strong` e `emph` (`01_core/src/compiler/layout/paragraph_prepare.rs:213-235`). Qualquer outro nó devolve `None`. Com uma aspa automática no parágrafo, ele inteiro cai no caminho palavra a palavra, que dispõe a aspa como palavra isolada (`01_core/src/compiler/layout/smartquote.rs:28`) e decide a quebra antes da palavra seguinte. O vanilla resolve a aspa na coleta, como texto do mesmo segmento (`collect.rs:196-211`), e a quebra segue as regras do Unicode para aspas.

## Escopo proposto

| Parte | Mudança |
|---|---|
| A | Texto colado com o mesmo estilo, vindo de nós diferentes, é moldado como um só trecho, com kerning e ligaduras, no planejador e no caminho palavra a palavra. Trechos bidirecionais e RTL continuam separados até haver medição própria, para não repetir a regressão de `bidi-nesting`. |
| B | O planejador de parágrafo coleta a aspa automática, resolvida pelo mesmo contexto de aspas, como texto do segmento vizinho. Ela não abre oportunidade de quebra onde a aspa literal não abre. |

## Compatibilidade e contrato

- É correção de paridade de geometria e quebra de linha, sem mudar contrato protegido (ADR-0127): nenhuma flag, nenhum padrão, nenhuma fase.
- Documentos com texto colado de mesmo estilo, ou com aspas automáticas, mudam de largura em frações de ponto e podem quebrar linhas em outro lugar, o mesmo lugar do vanilla.

## Fora

- Kerning entre trechos de estilos diferentes, que o vanilla também não aplica.
- Texto RTL e trechos bidirecionais.
- As demais diferenças de `smartquote.typ` que não sejam kerning nem quebra depois da aspa. Por exemplo, `smartquote-uk` muda o tamanho da página e precisa de medição própria.

## Aceitação

1. L0 primeiro, com selos e linhagem limpos: `word_layout`, `paragraph_prepare`, `paragraph_plan` e `smartquote`, conforme os arquivos tocados.
2. RED em L1 para cada linha das duas tabelas de medição, com a mesma largura e a mesma quebra do vanilla.
3. Suíte oficial sem regressão em relação a `pdf-paths.json`: `bidi-nesting`, `show-text-isolated-space` e `issue-5536-quote-inline-quotes-false` continuam idênticos. Os testes de `smartquote.typ` com os dois sintomas passam a idênticos.
4. Suíte completa do projeto, lint e linhagem reversa, com build em RAM.

## Decisão pedida

Aprovar A–B. A alternativa é aprovar só A, que o protótipo já mostrou útil, deixando a quebra depois da aspa para depois.

## Execução

**26 de novembro de 2026.** HEAD `76d5778e4`, árvore não commitada com A–B e a alteração de outra autoria (`git diff HEAD --stat`: 25 arquivos; mais este diagnóstico, não rastreado). Cristalino debug `8d59ccfa8209d7a22f166fa81e73d891c15beb33e77656da6dc0bbe07180bec4`, compilado em RAM às 19:37. Suíte do projeto 11.005 aprovados, 0 falhas, 7 ignorados; doc-tests 5; lint sem violações (só o V17 anterior em `realize/recipes.rs:301`); linhagem reversa 1.034 consumidores, 0 erros.

**Sondas.** Às 19:39, todas as linhas das duas tabelas de medição saem iguais ao vanilla em largura e em quebra de linha, inclusive `f#[i]`, com 21,20.

**Suíte oficial.** De 19:37:53 a 19:39:48, via PDF (`pdf-runs.json`, SHA-256 `1ae585c3a316cd5802e02939db988c46550e4613c69a5d793b82b3dab1afbaa5`), comparada com `pdf-paths.json`.

- **Melhoraram 13 testes, e nenhum piorou.** Os idênticos ao vanilla passaram de 955 para 966.
- **`text/smartquote.typ`:**
  - Passaram a idênticos: `smartquote`, `-de`, `-apostrophe`, `-close-before-letter`, `-prime`, `-bracket` e `-nesting-twice`.
  - `-fi` e `-ro` ficaram quase idênticos, com 2 e 7 pixels.
- **Outros arquivos:** `counter-figure` passou a idêntico. `show-selector-basic`, `newline-continuation-markup` e `space-collapsing` saíram de quase idênticos para idênticos.
- **Testes citados na aceitação:** `bidi-nesting`, `show-text-isolated-space` e `issue-5536-quote-inline-quotes-false` continuam idênticos.

**Como ficou.**

- **A:**
  - O texto colado ao último texto da linha, com o mesmo estilo e dentro da mesma fronteira de elemento, desloca o cursor pelo ajuste do par medido junto (`word_layout::kern_attached`). O item anterior nunca muda.
  - A primeira versão juntava o texto no item anterior. Ela quebrou o restauro de checkpoints que truncam a linha (`checkpoint.rs:189`), e por isso foi trocada.
  - As fronteiras de elemento são contadas na entrada e na saída de `layout_content` e `layout_content_once`. Só texto, aspas automáticas, espaços, sequências e embrulhos de fonte continuam o trecho.
  - Medido às 19:47 com o binário final, o vanilla também não aplica kerning através de `metadata`, `state`, rótulos e `context`, e o Tekt agora faz o mesmo.
  - Linhas e estilos RTL, tracking e realce ficam como estavam.
- **B:**
  - O planejador de parágrafo coleta a aspa automática como texto da palavra vizinha, resolvida pelo mesmo contexto de aspas, e observa texto e espaços nesse contexto.
  - `try_layout` e o planejador de várias partes guardam o contexto e o restauram quando desistem.
  - Cada segmento depois do primeiro começa de um contexto zerado.
  - Um segmento planejado que resolveu aspas instala, na emissão, o contexto que alcançou.

**Limites que ficam.**

- Ligaduras entre nós diferentes não são formadas. A largura coincide com a do vanilla, mas o glifo `fi` só sairia num item único.
- Com hifenização ativa, uma palavra que contém aspa automática faz o planejador desistir (`paragraph_hyphen.rs:49-67`), como já acontecia com outros sinais.
- `smartquote-he` (RTL) e os demais testes de `smartquote.typ` com outra causa continuam diferentes.
