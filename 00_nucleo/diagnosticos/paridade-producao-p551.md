# Paridade de Produção — Passo 551

**Data:** 2026-07-03  
**Repositório:** `typst-crystalline`  
**Binários:**

- Cristalino: `./target/release/typst`
- Vanilla 0.14.2: `/usr/local/bin/typst`
- Vanilla 0.15.0: `lab/typst-original/target/release/typst`
- Ferramentas: `mutool` 1.23.10, `pdftotext`

---

## 1. Objetivo

Duas tarefas independentes:

1. **Materializar a lista informal de ~24 itens** referida em P550 no ficheiro `00_nucleo/diagnosticos/inventario-decisoes-pendentes.md`, distinguindo itens já corrigidos de itens ainda abertos.
2. **Re-verificar directamente** o comportamento das notas de rodapé em colunas, comparando o cristalino com o vanilla 0.15.0 (e 0.14.2), usando imagens renderizadas, não descrição de memória.

---

## 2. Parte 1 — Integração da lista de 24 itens

A lista foi adicionada a `00_nucleo/diagnosticos/inventario-decisoes-pendentes.md` como **secção 2**, separada dos achados de P550 (secção 1). A organização segue o formato já usado no inventário:

- Formatos de exportação — nunca implementados
- Fontes
- PDF — estrutura e metadados
- Numeração de página
- Colunas
- Bibliografia
- Pacotes
- Linguagem

Itens que já tinham sido corrigidos por passos posteriores (p. ex. quebra de linha com fallback, fallback global fixo, padrões compostos de numeração, bugs de CSL, `#for`, `#{expr}` em markup) foram marcados como **"Já corrigido"** com referência ao passo correspondente, em vez de ficarem como pendentes desactualizados.

Itens que permanecem em aberto (exportação HTML/SVG/raster, fontes de cor/Type1/CJK vertical, PDF Tagged, pacotes `@preview`, etc.) mantiveram o estado original.

---

## 3. Parte 2 — Re-verificação das notas de rodapé em colunas

### 3.1 Documento de teste principal (`#set page(columns: 2)`)

O teste solicitado em P551 usa a configuração global de colunas:

```typst
#set page(columns: 2)
#lorem(80)#footnote[Nota da primeira coluna.]
#colbreak()
#lorem(80)#footnote[Nota da segunda coluna.]
```

Comandos:

```bash
./target/release/typst /tmp/p551-footnote-cols.typ /tmp/p551-cristalino.pdf
/usr/local/bin/typst compile /tmp/p551-footnote-cols.typ /tmp/p551-vanilla-014.pdf
lab/typst-original/target/release/typst compile /tmp/p551-footnote-cols.typ /tmp/p551-vanilla-015.pdf

mutool draw -o /tmp/p551-cristalino.png -r 150 /tmp/p551-cristalino.pdf
mutool draw -o /tmp/p551-vanilla-014.png -r 150 /tmp/p551-vanilla-014.pdf
mutool draw -o /tmp/p551-vanilla-015.png -r 150 /tmp/p551-vanilla-015.pdf
```

### 3.2 Resultado visual — posicionamento

As três imagens mostram o mesmo posicionamento de notas:

- **Cristalino:** notas de rodapé no fundo de cada coluna, lado a lado.
- **Vanilla 0.14.2:** notas de rodapé no fundo de cada coluna, lado a lado.
- **Vanilla 0.15.0:** notas de rodapé no fundo de cada coluna, lado a lado.

**Conclusão para `#set page(columns: 2)`:** o posicionamento lado a lado do cristalino **está alinhado com o vanilla 0.14.2 e 0.15.0**. Não é uma divergência.

### 3.3 Resultado textual — numeração

A extração de texto do PDF cristalino revela um problema de numeração:

```text
...elit sed do eiusmod[1]

...elit sed do eiusmod[1]

[1] Nota da primeira coluna.

[1] Nota da segunda coluna.
```

Ou seja, o cristalino **repete `[1]`** nas duas colunas. O vanilla 0.14.2 e 0.15.0 incrementam correctamente:

```text
...philosophia defensa et.¹

...philosophia defensa et.²

¹Nota da primeira coluna.

²Nota da segunda coluna.
```

**Conclusão para `#set page(columns: 2)`:** existe uma **diferença real de comportamento na numeração de notas**. O cristalino não incrementa o contador de notas de rodapé através de `#colbreak()` neste modo de colunas.

### 3.4 Teste adicional — `#columns(2)`

Para esclarecer a divergência relatada em P537, foi feito um teste adicional com o comando `#columns(2)[...]`:

```typst
#columns(2)[
  #lorem(80)#footnote[Nota da primeira coluna.]
  #colbreak()
  #lorem(80)#footnote[Nota da segunda coluna.]
]
```

Resultado visual:

- **Cristalino:** notas lado a lado, no fundo de cada coluna (igual ao caso anterior).
- **Vanilla 0.14.2 e 0.15.0:** notas **empilhadas verticalmente na coluna esquerda**.

**Conclusão para `#columns(2)`:** o cristalino difere do vanilla no posicionamento. A observação de P537 estava correcta, mas aplicava-se ao comando `#columns(2)`, não a `#set page(columns: 2)`.

---

## 4. Decisão

Aplicando ADR-0107 (paridade com a linguagem, não com a mecânica) e ADR-0108 (medir antes de decidir):

1. **Posicionamento em `#set page(columns: 2)`** — alinhado com vanilla. Sem acção.
2. **Numeração em `#set page(columns: 2)`** — diferença real. Deve ser corrigida num passo posterior; foi registada como item aberto no inventário.
3. **Posicionamento em `#columns(2)`** — diferença real. Requer decisão explicitamente documentada (corrigir ou aceite com razão nova); também registada no inventário.

A frase "corresponde à semântica esperada" usada em P537/P550 **não se aplica** sem mais à situação global de colunas: é verdadeira para `#set page(columns: 2)` no que toca a posicionamento, mas falsa para `#columns(2)` e para a numeração.

---

## 5. Ficheiros de verificação

- `/tmp/p551-footnote-cols.typ` — teste principal (`#set page(columns: 2)`)
- `/tmp/p551-footnote-cols2.typ` — teste adicional (`#columns(2)`)
- `/tmp/p551-cristalino.pdf`, `/tmp/p551-cristalino.png`
- `/tmp/p551-vanilla-014.pdf`, `/tmp/p551-vanilla-014.png`
- `/tmp/p551-vanilla-015.pdf`, `/tmp/p551-vanilla-015.png`
- `/tmp/p551-vanilla-014-2.pdf`, `/tmp/p551-vanilla-014-2.png`
- `/tmp/p551-vanilla-015-2.pdf`, `/tmp/p551-vanilla-015-2.png`
- `/tmp/p551-cristalino2.pdf`, `/tmp/p551-cristalino2.png`

Estes ficheiros são temporários e não são commitados.

---

## 6. Conclusão

- O inventário de decisões pendentes foi actualizado com os 24 itens da lista informal.
- A re-verificação das notas de rodapé em colunas mostra que o cristalino **está correto no posicionamento** para `#set page(columns: 2)`, mas apresenta **dois problemas de paridade**:
  - numeração duplicada de notas em `#set page(columns: 2)`;
  - posicionamento lado a lado em `#columns(2)` quando o vanilla empilha.
- Não houve alterações de código de produção neste passo.
