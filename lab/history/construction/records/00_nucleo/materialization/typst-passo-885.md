# Passo 885 — Verificação visual pós-P884 (cristalino vs vanilla)

**Data**: 2026-07-24
**Tipo**: Verificação. Nenhum código foi alterado neste passo.
**Motivação**: recomendação 3 do handoff pós-P884 (`00_nucleo/handoff-novo-chat-p884.md`) — confirmar que o estado depois de P884 está estável antes de continuar a construir em cima dele.
**Método**: exportação de 6 dos 7 cenários do benchmark de performance (falta `06-long`) pelos dois binários, seguida de extração de texto dos PDFs resultantes e comparação. Os achados 3 e 4 vieram de inspeção visual directa feita pelo dono do projecto, fora deste método — ver nota em cada um.

**Limite do método de extração de texto**: em especial para conteúdo matemático, a extração pode reordenar ou cortar caracteres mesmo quando o PDF está correto visualmente. Os achados 1 e 2 partem de extração de texto, não de inspeção visual do PDF renderizado, e precisam de confirmação visual antes de virarem achado formal com número. A extração de texto também não detecta desenho vectorial sem texto associado — linhas de grelha de tabela, header, footer — o que explica por que os achados 3 e 4 não apareceram na comparação da secção seguinte.

---

## Resultado por cenário

| Cenário | Cristalino | Vanilla | Resultado |
|---|---|---|---|
| `01-hello` | "Hello World" | "Hello World" | Igual |
| `02-lorem` | parágrafo Lorem ipsum completo | parágrafo Lorem ipsum completo | Igual (texto idêntico nos dois) |
| `03-images` | 5 páginas, espirais Debian, 42 imagens no total | 5 páginas, espirais Debian, 42 imagens no total | Igual |
| `05-tables` | texto (números 0–49) igual ao vanilla; **sem linhas de tabela** | texto igual; com linhas de tabela | Diferente — ver achado 3 |
| `06-long` | não inspeccionado por este passo (ficheiro não incluído) | tem header e footer | Diferente — ver achado 4 |
| `04-math` | `∑𝑛 𝑖=0 𝑖 2 =𝛼+𝛽`, repetido | `∑𝑛 𝑘=0 𝑘 2 = 𝑛(𝑛+1)(2𝑛+1)/6`, seguido de `𝛼+𝛽=𝛾/2` | Diferente — ver achado 1 |
| `07-context` | sem texto extraível (documento aparenta vazio) | ~120 repetições de `(width: 42.85pt, height: 7.24pt)` | Diferente — ver achado 2 |

---

## Achado 1 (não confirmado) — `04-math`: fração e segunda equação ausentes na extração de texto do cristalino

O texto extraído do cristalino não contém `n(n+1)(2n+1)/6` nem `𝛾/2`. O vanilla tem os dois. O cristalino também usa `i` como índice do somatório onde o vanilla usa `k`.

Duas hipóteses, nenhuma verificada aqui:
1. O PDF do cristalino de facto não desenha a fração e a segunda equação (bug de layout de matemática).
2. O PDF desenha tudo, mas a extração de texto não capturou (comum em glifos posicionados sem ordem lógica de leitura, o que é o caso normal de fórmulas em PDF).

**Próximo passo antes de abrir achado**: abrir `cristalino-04-math-p884.pdf` visualmente e confirmar se a fração e a segunda equação estão ou não desenhadas na página.

## Achado 2 (não confirmado) — `07-context`: PDF sem texto extraível

O PDF do cristalino para o cenário `07-context` não retornou texto na extração, enquanto o vanilla retornou ~120 repetições da tupla `(width: ..., height: ...)`. Isto é uma diferença maior do que a do achado 1 — não é uma questão de ordem de extração, é ausência total de conteúdo textual.

Duas hipóteses, nenhuma verificada aqui:
1. A página do cristalino está de facto em branco (o teste de `#context`/`measure()` não está a produzir saída).
2. A página tem conteúdo, mas não é texto (por exemplo, se o teste desenha formas em vez de imprimir números) — o que tornaria a extração de texto vazia por desenho, não por bug.

**Próximo passo antes de abrir achado**: abrir `cristalino-07-context-p884.pdf` visualmente e confirmar se a página está mesmo em branco. Se estiver, isto é relevante em conjunto com o item já aberto no handoff pós-P884 sobre `measure()`/`Content::Context` (achado #34, fechado em P860) — vale confirmar se a saída visual de `measure()` continua correta depois de tudo que mudou em P872–P884 (frente de performance mexeu em fontes e export, não devia ter mexido em `measure()`, mas isso nunca foi confirmado depois de P884).

## Achado 3 (reportado pelo dono, não confirmado por mim) — `05-tables`: linhas da tabela ausentes no cristalino

O dono do projecto reportou que o PDF do cristalino para `05-tables` tem o mesmo conteúdo textual do vanilla (a grelha de números 0–49), mas não tem as linhas da tabela. O vanilla tem as linhas.

Este achado não apareceu na comparação por extração de texto acima porque linhas de tabela são desenho vectorial (path/stroke no PDF), sem texto associado — o método usado neste passo não as veria de qualquer forma, correcto ou não. Não vi os PDFs para confirmar directamente.

**Próximo passo antes de abrir achado**: confirmar visualmente qual é a especificação Typst usada no ficheiro-fonte de `05-tables` (se `table()` com `stroke:` definido, ou default). Se o vanilla desenha borda por default e o cristalino não, é divergência de comportamento default de `table()`, não só de um caso configurado.

## Achado 4 (reportado pelo dono, não confirmado por mim) — `06-long`: header e footer ausentes no cristalino

O dono do projecto reportou que o PDF do vanilla para `06-long` tem header e footer, implicando que o do cristalino não tem (ou é diferente). O ficheiro do cristalino para este cenário não foi incluído neste passo, então não há como este relatório confirmar directamente.

Isto cruza com um item já registado como aberto no handoff pós-P884, secção "Estado actual — o que ficou aberto", item 2: "conteúdo marcado / acessibilidade PDF" e a nota geral de que o cristalino não emite certas estruturas que o vanilla emite. Header/footer por página é um mecanismo diferente desse (é `#set page(header:, footer:)`, não marcação de acessibilidade), mas seria bom confirmar se está registado nalgum DEBT ou se é descoberta nova.

**Próximo passo antes de abrir achado**: carregar `cristalino-06-long-p884.pdf` e `vanilla-06-long.pdf` e comparar directamente. Confirmar se `#set page(header:, footer:)` está implementado no cristalino e, se estiver, por que não aparece neste output.

---

## O que este passo não fez

- Não comparou bytes ou tamanho de ficheiro entre cristalino e vanilla.
- Não correu a suíte de testes (`cargo test`).
- Não abriu os PDFs visualmente — os achados 1 e 2 partem só de texto extraído e precisam de confirmação visual antes de contarem como achado formal. Os achados 3 e 4 vieram de inspecção visual do dono, não deste relatório.
- Não teve acesso aos ficheiros de `06-long` — o achado 4 depende inteiramente do relato do dono.

## Recomendação para o passo seguinte

1. Confirmar visualmente os achados 1 e 2 (abrir os dois PDFs do cristalino em questão).
2. Confirmar achado 3 olhando para o ficheiro-fonte `.typ` de `05-tables` — se o `stroke` da tabela está definido explicitamente ou se depende de um default que o cristalino não está a aplicar.
3. Confirmar achado 4 comparando os dois PDFs de `06-long` lado a lado.
4. Dos quatro, abrir como achados formais numerados os que forem confirmados. Prioridade sugerida: achado 3 (tabela sem linhas é uma regressão visual clara e provavelmente fácil de localizar — provavelmente no exportador PDF de `table()`, não em layout) e achado 2 (`07-context` vazio, se confirmado, por ser ausência total de conteúdo).
5. Se algum dos quatro for descartado (PDF correto, achado era erro de leitura), registar isso explicitamente aqui ou num passo seguinte, para não ficar sem resposta.
