# Relatório — typst-passo-885: confirmação visual dos 4 achados (seguimento)

**Data:** 2026-07-24T02:39:48Z
**Executor:** Claude (Sonnet 5)
**Commit base:** `3f15cc50ec1dc40e852b41bc92e9ee2895ecaec6` (HEAD do ramo `Tekt`)
**Working tree:** **não limpa** — alterações não commitadas em 31 ficheiros (font/export/shaper/world,
prompts L0 correspondentes, e fixtures `03_infra/fixtures/p307b/reference/*.pdf`), `+1106/-405`. Lista
completa: `git diff HEAD --stat` no momento desta medição. As medições abaixo usam o binário
compilado **desta working tree**, não do commit puro — se `cargo build --release` for repetido após
commitar este trabalho pendente, o binário não muda, mas o hash de proveniência muda.
**Ficheiros fonte usados:** `/tmp/p872-bench/0{4,5,6,7}-*.typ` (estado no momento desta medição — ver
nota de metodologia abaixo, dois deles não são os mesmos ficheiros que geraram os PDFs vanilla de
referência de P872).

Nenhum código foi alterado neste passo — é verificação, conforme pedido em `typst-passo-885.md`.

---

## 1. Resumo

Dos 4 achados não confirmados/reportados em P885, **2 confirmam-se como regressões reais** e **2
são artefacto de comparação com fonte errada** (não são bugs, descartados). Adicionalmente, a
inspecção visual expôs uma anomalia de espaçamento não coberta por nenhum dos 4 achados originais
(secção 5).

| Achado | Veredito | Método |
|---|---|---|
| 1 — `04-math`: fração/segunda equação ausente | **Descartado** — comparação usava `vanilla-04-math.pdf` de fonte `.typ` antiga (P872), diferente da fonte actual | Recompilação dos dois binários a partir do `.typ` actual + inspecção visual |
| 2 — `07-context`: PDF sem texto | **Confirmado** — regressão real | Inspecção do stream de conteúdo (`/Length 0`) + render visual |
| 3 — `05-tables`: linhas da tabela ausentes | **Confirmado** — regressão real | Render visual + contagem de operadores de stroke no content stream |
| 4 — `06-long`: header/footer ausentes | **Descartado** — comparação usava `vanilla-06-long.pdf` de fonte `.typ` antiga (P872) que tinha `#set page(header:, footer:)`; a fonte actual não define header/footer em nenhum dos dois binários | Recompilação do vanilla a partir do `.typ` actual + inspecção visual |

---

## 2. Nota de metodologia — por que achados 1 e 4 caíram

`vanilla-04-math.pdf` e `vanilla-06-long.pdf`, usados na comparação original de P885, são os PDFs
do benchmark de **P872** (`ls -la` confirma timestamps `jul 23 17:00/17:03`, sessão anterior a
qualquer edição destes `.typ`). Os ficheiros-fonte `04-math.typ` e `06-long.typ` em
`/tmp/p872-bench/` foram **reescritos depois disso** (timestamps `jul 23 20:33` e `20:36`,
provavelmente durante os passos de depuração de performance P874–P884) para versões mais simples,
usadas para gerar os PDFs `cristalino-*-p884.pdf`. A comparação de P885 juntou um vanilla antigo
(fonte rica: duas equações + fração; header/footer) com um cristalino novo (fonte simplificada: uma
equação; sem header/footer) — **as duas metades não vêm do mesmo `.typ`**.

Confirmação directa, recompilando os dois binários a partir do `.typ` **actual**:

```
$ pdftotext vanilla-04-math.pdf(P872, antigo) -
∑ 𝑘² = 𝑛(𝑛+1)(2𝑛+1)/6      𝛼+𝛽=𝛾/2      ← fonte antiga (2 equações, índice k)

$ cat 04-math.typ (actual)
#for i in range(100) { $ sum_(i=0)^n i^2 = alpha + beta $ }

$ <vanilla_bin> compile 04-math.typ /tmp/vanilla-04-math-current.pdf
$ <crist_bin>           04-math.typ /tmp/cristalino-04-math-current.pdf
→ ambos renderizam exactamente "∑ i² = α + β", nada em falta dos dois lados.
```

Mesmo padrão para `06-long.typ`: a fonte actual é só `heading + lorem(200) + pagebreak`, sem
`#set page(header:, footer:)`; recompilar o vanilla a partir dela produz um PDF **sem** header/footer,
idêntico em estrutura ao cristalino. O texto literal "Header"/"Footer" visto no PDF antigo do vanilla
não existe na fonte actual — não há nada para o cristalino "perder".

**Estes dois achados ficam descartados, não fechados por correcção** — não houve bug para corrigir,
houve descompasso entre o par de ficheiros comparados. Registo explícito conforme pedido no item 5
da recomendação de P885.

---

## 3. Achado 2 (confirmado) — `07-context`: `#context measure[...]` não produz conteúdo

Fonte: `07-context.typ` — `#for i in range(200) { context measure[lorem(10)] }`.

Inspecção directa dos bytes de `cristalino-07-context-p884.pdf` (2100 bytes, idêntico byte-a-byte
em tamanho ao `cristalino-07-context.pdf` original de P872 — não é regressão introduzida entre P872
e P884, o problema já existia nos dois):

```
4 0 obj
<< /Length 0 >>
stream

endstream
endobj
```

O content stream da página é **literalmente vazio** (`/Length 0`) — não é um problema de ordem de
extracção de texto (a hipótese 2 do achado original, "página tem conteúdo mas não é texto", está
**refutada**: não há nenhum operador de desenho, nem `Tj`/`TJ` nem `re`/`l`/`S`). Confirmado também
visualmente — render a 150 dpi da página é uma folha em branco.

O vanilla, com a mesma fonte, produz `(width: 42.85pt, height: 7.24pt)` repetido ~120 vezes — o
dicionário retornado por `measure()` é convertido em conteúdo textual visível quando é o valor de
retorno de um bloco `context` a nível de markup. O cristalino calcula (presumivelmente) o valor mas
não o converte em `Content`/texto visível — o resultado é descartado silenciosamente.

**Cruzamento com histórico**: o próprio `typst-passo-885.md` liga isto ao achado #34 do handoff
pós-P884 ("`measure()`/`Content::Context`", fechado em P860). Não abri `00_nucleo/handoff-novo-chat-
p884.md` neste passo para verificar os detalhes desse achado #34 — fica como próximo passo
verificar se isto é uma regressão de #34 ou um caso nunca coberto por ele (o comentário do próprio
P885 já suspeitava disto: "a frente de performance mexeu em fontes e export, não devia ter mexido em
`measure()`, mas isso nunca foi confirmado depois de P884").

**Localização para correcção futura**: não tracei o código responsável (fora do âmbito deste passo,
que é só confirmação visual) — mas dado que o sintoma é "resultado computado, não emitido", o ponto
de partida razoável é onde `context` avalia o bloco e decide se o valor de retorno vira `Content`
(provavelmente em `01_core/src/engine/eval/` ou `01_core/src/engine/introspect/`, não confirmado).

---

## 4. Achado 3 (confirmado) — `05-tables`: sem linhas de grelha

Fonte: `05-tables.typ` — `table(columns: 5, rows: 10, ..range(50).map(str))`, **sem** `stroke:`
explícito. Em Typst, `table()` desenha `1pt + black` por omissão quando `stroke` não é passado
(diferente de `grid()`, que não desenha nada por omissão) — a fonte depende desse default.

Render a 150 dpi, lado a lado:
- **Vanilla**: grelha completa, todas as células com borda visível nas 4 páginas.
- **Cristalino** (`cristalino-05-tables-p884.pdf`): números idênticos (`0`–`49`, mesma disposição
  5×10), **nenhuma linha visível** em nenhuma das 4 páginas.

Corroborado por contagem de operadores no content stream (streams descomprimidos, regex sobre
operadores PDF):

| PDF | Operador `S` (stroke) | Operador `Tj`/`TJ` (mostrar texto) |
|---|---|---|
| `vanilla-05-tables.pdf` | 371 | 200 (`Tj`; texto por `Tj` simples) |
| `cristalino-05-tables-p884.pdf` | 3 | 0 (`Tj`; cristalino usa outro operador de show de texto, não `Tj` — não afecta a conclusão sobre `S`) |

3 operadores de stroke no documento inteiro (4 páginas, 20 tabelas, 5×10 células cada = potencialmente
centenas de segmentos de borda esperados) é consistente com "essencialmente nenhuma borda desenhada",
não com uma diferença de estilo pontual.

**Localização para correcção futura**: não tracei o código do exportador de `table()` neste passo
(fora do âmbito — só confirmação visual). Fica como próximo passo natural, conforme já sugerido na
recomendação original de P885: localizar onde `table()` aplica o `stroke` default e confirmar se o
default não está a ser aplicado, ou se está a ser calculado mas descartado na exportação (mesmo
padrão do achado 2 — "computado mas não emitido" — vale a pena verificar se há uma causa comum entre
os dois antes de tratá-los como bugs independentes).

---

## 5. Observação nova, fora do escopo dos 4 achados — espaçamento anómalo em texto com conteúdo interpolado

Não pedida pelo passo, mas visível nos mesmos renders usados para os achados 1 e 4: em dois locais
distintos onde o texto mistura conteúdo estático com um valor interpolado (`#i` num heading; um
índice de sub/sobrescrito numa equação), o cristalino insere um espaço visível extra que o vanilla
não tem:

- `06-long`, heading: vanilla renderiza `Section 0` (espaço normal); cristalino renderiza
  `Section    0` (gap muito maior entre a palavra e o número interpolado).
- `04-math`: vanilla renderiza `i=0` e `i²` colados; cristalino renderiza `i  =0` e `i  2` com gap
  antes do `=` e antes do expoente.

Não investiguei a causa — é só uma observação de inspecção visual, sem medição de código associada,
registada aqui para não se perder (mesmo padrão dos achados 3/4 originais: reportado por observação
directa, precisa de passo dedicado antes de virar achado formal com número). Hipótese não verificada:
puntos de junção entre `FrameItem::Text` de runs adjacentes (um run estático, um run do valor
interpolado) a somar um avanço/espaço que não deveria estar lá — mas isto é especulação, não
medição.

---

## 6. O que este passo não fez

- Não abriu `00_nucleo/handoff-novo-chat-p884.md` para cruzar o achado 2 com o achado #34 histórico.
- Não localizou o código exacto responsável pelos achados 2 e 3 (só confirmou visualmente que são
  reais; localização fica para o próximo passo, como a própria recomendação de P885 já previa).
- Não investigou a anomalia de espaçamento da secção 5 além de a registar.
- Não tocou em nenhum ficheiro de código — só leu PDFs, `.typ` de benchmark, e recompilou binários
  já existentes contra ficheiros temporários em `/tmp`, fora do repositório.

## 7. Recomendação para o passo seguinte

1. Localizar a causa do achado 2 (`context`/`measure` sem output) — começar por confirmar se é
   regressão de #34 ou caso novo, depois localizar no código de avaliação de `context`.
2. Localizar a causa do achado 3 (stroke default de `table()` ausente) — começar pelo exportador de
   tabela; verificar se partilha causa raiz com o achado 2 (padrão "computado mas não emitido") antes
   de tratar como dois bugs separados.
3. Decidir se a anomalia de espaçamento da secção 5 justifica um passo de investigação dedicado antes
   ou depois dos itens 1–2.
4. Ao gerar novos PDFs de referência para comparação vanilla/cristalino em passos futuros, gerar os
   dois **do mesmo `.typ`, no mesmo passo** — a causa raiz de achados 1 e 4 caírem foi comparar PDFs
   de sessões diferentes com fontes diferentes sem perceber. Considerar nomear os PDFs de benchmark
   com hash do `.typ` de origem (ou regenerar sempre os dois lados a cada passo) para evitar repetir
   este erro.
