# Estado geral do projecto — depois de P695

**Data:** 2026-07-10

---

## Linhas de trabalho fechadas ou em bom estado

### 1. Falhas silenciosas (P633-656)
Três rondas de auditoria independentes (padrões de texto, segunda ronda de padrões, avisos do compilador/clippy). 23+ casos confirmados e corrigidos — `FlowEvent` (`#break`/`#continue`/`#return` nunca funcionavam), regras `#set` a ignorar tipos inválidos, `counter.display`, bibliografia a omitir entradas, grid a renderizar vazio. **Estado: fechado**, com confiança razoável, não absoluta.

### 2. Desempenho (P657-677)
`macro-10x` foi de 6,78× mais lento que o vanilla para ~1,18-1,33× (por vezes mais rápido). Documentos pequenos passaram a ser 2,3-2,5× **mais rápidos** que o vanilla, depois de corrigir leitura duplicada de fontes no arranque. **Estado: muito bom**, sem mais gordura óbvia identificada nas últimas sondas.

### 3. Divergências de linguagem vs implementação (P660-665)
Regra nova estabelecida: nome igual ao vanilla obriga a comportamento igual. `variant: (eixo: valor)` (inventado por engano) revertido; `text.bold`/`text.italic` revertidos, substituídos por `weight`/`style`. **Estado: fechado** para os casos já auditados; a auditoria original (P663) tinha uma limitação de método (só encontrava casos já auto-rotulados) — P664 corrigiu isso testando directamente, mas só para as funções já tocadas nesta conversa, não o resto da linguagem.

### 4. Pendência antiga de fontes variáveis (P525 → P666-669)
Contornos visuais agora correctos para qualquer peso/estilo, nos dois caminhos de export (single-font e multi-font), com erro claro se Python/fontTools não estiverem disponíveis.

### 5. Métodos de `str` (P689-693)
Lista completa de métodos oficiais confirmada contra o código fonte do vanilla. Todos implementados e com assinatura/comportamento corrigidos (`len`/`at`/`slice` para bytes, `find` para substring, `match`/`matches` para `str | regex`, `normalize` novo). **Estado: fechado**, com confirmação de completude (não só "os que já tínhamos, agora correctos").

### 6. `#import`, pacotes offline, `sys` (P678-695)
`#import` de ficheiros locais implementado do zero (não existia). Pacotes `@preview` funcionam se já estiverem na cache local do sistema. Módulo `sys` implementado. Validado contra um pacote real (`cetz`), que revelou uma cadeia de bugs reais ao longo do caminho — cada um corrigido.

---

## O que fica confirmado como pendente, não corrigido

### Bloqueio imediato de `cetz` — plugins WASM
`cetz` 0.5.2 usa um sistema de plugins WASM (`plugin()`), que o cristalino não tem. Confirmado, não inferido. É uma funcionalidade nova e grande — nunca sondada. Se quiser terminar de validar `cetz` até produzir PDF, é o próximo passo.

### Download de pacotes (P-γ do plano original de P678)
Pacotes só funcionam se já estiverem na cache local (`~/.cache/typst/packages`). Se um pacote não estiver descarregado, o cristalino dá erro claro, mas não descarrega sozinho. O vanilla descarrega automaticamente do registo oficial.

### Resolução de versão "mais recente" (P-δ do plano original de P678)
`#import "@preview/nome"` sem versão explícita não está implementado — é preciso sempre dizer a versão exacta.

### Itens nunca tocados nesta conversa (lista original, desde P526/P531)
- Exportação HTML, SVG, PNG.
- IDE/LSP.
- PDF Tagged/PDF-UA (acessibilidade).
- Compressão por object streams.
- Fontes de cor para emoji (COLR/CPAL).
- Fontes Type1/PostScript.
- Quebra de linha para CJK/Thai (scripts sem espaços).
- Escrita vertical (CJK) — confirmado como ausente **nos dois lados**, não é uma disparidade a corrigir, é uma funcionalidade nova para os dois.

### Débitos pequenos, registados mas não urgentes
- Subsetting de fontes variáveis menos agressivo que o vanilla (funciona correctamente, mas produz ficheiros maiores).
- `str.replace`/flags de regex — scope-out parcial de P689, nunca revisto.
- Auditoria de divergências de linguagem (P663/P664) só cobriu as funções já tocadas nesta conversa — o resto da linguagem (muitas outras funções da stdlib) nunca foi varrido da mesma forma sistemática.

---

## Como decidir por onde continuar

Três categorias, por tamanho e tipo de esforço:

1. **Pequeno, fecha uma cadeia já aberta:** nenhum de momento — as cadeias pequenas foram todas fechadas.
2. **Médio, continua uma cadeia em curso:** `plugin`/WASM (termina `cetz`), download de pacotes, resolução de versão.
3. **Grande, começa algo novo:** exportação HTML/SVG, PDF Tagged/UA, ou uma auditoria sistemática da stdlib inteira (não só as funções já tocadas) para o mesmo tipo de divergência de linguagem já encontrado várias vezes.
