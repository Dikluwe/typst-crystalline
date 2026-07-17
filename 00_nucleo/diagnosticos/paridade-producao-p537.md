# P537 — Relatório de Paridade: Notas de Rodapé em Layout de Duas Colunas

> **Passo:** 537
> **Data:** 2026-07-02
> **Foco:** Notas de rodapé posicionadas no fundo da coluna respectiva em
> `columns(2)`, em vez de no fundo da página inteira.
> **Autor:** assistente de IA (execução de `00_nucleo/materialization/typst-passo-537.md`)

---

## 1. Resumo Executivo

Implementou-se colunas reais lado a lado para `Content::Columns`, dividindo o
body pelos `colbreak()` e renderizando cada segmento como uma coluna independente
na mesma página. As notas de rodapé são agora flushed no fim de cada coluna,
usando coordenadas locais da coluna, o que posiciona cada nota abaixo da coluna
onde foi referenciada.

- **Resultado:** cada nota aparece no fundo da sua coluna, lado a lado com a
  nota da outra coluna, numa única página.
- **Limitação identificada:** notas de rodapé maiores do que o espaço restante
  da coluna são silenciosamente descartadas no contexto de colunas (numa coluna
  única, o mecanismo P305 ainda as distribui pela página). Foi registada como
  scope-out conhecido.
- **Nota sobre `#set page(columns:)`:** o cristalino ainda não converte
  `#set page(columns: 2)` num envolvimento `columns(2, body)`. O teste usou a
  função explícita `#columns(2)[...]`, que é o construtor suportado.

---

## 2. Sonda

### 2.1 Perguntas e medições

1. **Buffer de notas pendentes** (`pending_footnote_bodies`) — medição em
   `01_core/src/engine/layout/cursor.rs:412` confirma que é um único buffer por
   página, sem separação por coluna.
2. **Evento de "fim de coluna"** — não existia. O `colbreak()` era downgrade
   literal a `new_page()` (`01_core/src/engine/layout/colbreak.rs`).
3. **Cálculo da área disponível** — `flush_pending_footnote_bodies` usava
   `page_w` e `page_h` sem noção de coluna (`cursor.rs:415-422`).

### 2.2 Teste directo antes da implementação

Documento:

```typst
#set page(columns: 2)
#lorem(80)#footnote[Nota da primeira coluna.]
#colbreak()
#lorem(80)#footnote[Nota da segunda coluna.]
```

Resultado: `#set page(columns:)` é ignorado pelo cristalino; o documento
renderizou como uma única coluna. Ambas as notas apareciam juntas no fundo da
página, confirmando o problema descrito em P531 Grupo 8.1.

---

## 3. Implementação

### 3.1 Ficheiros alterados

| Ficheiro | Alteração |
|----------|-----------|
| `01_core/src/engine/layout/columns.rs` | Reescrito para dividir `body` por `colbreak()`, renderizar cada segmento numa coluna, e posicionar footnotes localmente. |
| `01_core/src/engine/layout/cursor.rs` | `flush_pending_footnote_bodies` ganhou modo coluna (`column_mode`/`column_origin_x`/`column_width`) para medir e posicionar notas no fundo da coluna actual. |
| `01_core/src/engine/layout/mod.rs` | Campos `column_mode`, `column_origin_x`, `column_width` adicionados a `Layouter` (já existiam no estado resumido da sessão). |
| `01_core/src/engine/layout/tests.rs` | Teste P220 actualizado (colbreak dentro de columns separa colunas reais); testes P537 adicionados. |

### 3.2 Algoritmo de `columns::layout`

1. Calcula `column_width` e `gutter` a partir da largura útil actual.
2. Divide `e.body` em `count` segmentos pelos `Content::Colbreak`.
3. Para cada segmento:
   - Salvaguarda `current_items`, `current_line`, cursor e footnotes pendentes.
   - Configura a region com a largura da coluna e `cursor_x`/`cursor_y` no topo
     útil da coluna.
   - Activa `column_mode` no `Layouter`.
   - Renderiza o segmento, faz `flush_line()` e `flush_pending_footnote_bodies()`.
   - Recolhe os items produzidos (texto + nota da coluna).
   - Restaura o estado global e translada os items horizontalmente para a
     origem absoluta da coluna.
4. Adiciona todos os items acumulados à página actual e avança o cursor para a
   altura máxima consumida.

### 3.3 Coordenadas das footnotes

`flush_pending_footnote_bodies` em modo coluna:

- Mede e posiciona as notas usando `avail_w = column_width - 2*margin`.
- Posiciona-as em coordenadas **locais da coluna** (`left_x = margin`), de modo
  que a translação horizontal final em `columns.rs` as coloque na posição
  absoluta correcta (`column_origin_x + relative_x`).
- A coordenada Y continua a ser `page_h - margin` (fundo útil da página),
  alinhando visualmente as notas das várias colunas na mesma linha de rodapé,
  como no vanilla.

---

## 4. Validação

### 4.1 Documento de teste

```typst
#columns(2)[
  #lorem(80)#footnote[Nota da primeira coluna.]
  #colbreak()
  #lorem(80)#footnote[Nota da segunda coluna.]
]
```

### 4.2 Resultado cristalino

- **Páginas:** 1
- **Estrutura:** duas colunas de texto lado a lado.
- **Notas:**
  - Coluna 1: `[1] Nota primeira` em `x ≈ 70–190`, `y ≈ 763`.
  - Coluna 2: `[1] Nota segunda` em `x ≈ 380–500`, `y ≈ 763`.

Ambas as notas estão no fundo da página, cada uma alinhada à sua coluna.

### 4.3 Comparação com vanilla

O vanilla disponível (`/usr/local/bin/typst`, 0.14.2) renderizou o mesmo
documento com as duas notas também no fundo da página. A diferença principal é
que o cristalino posiciona as notas horizontalmente alinhadas a cada coluna,
enquanto o vanilla 0.14.2 colocou as notas uma abaixo da outra na coluna
esquerda. O passo refere paridade com vanilla 0.15.0; o comportamento cristalino
(uma nota por coluna, lado a lado) corresponde à expectativa semântica de
"nota no fundo da coluna onde foi referenciada".

### 4.4 Caso de uma coluna (sem regressão)

Teste unitário `p537_footnote_uma_coluna_sem_regressao` confirma que footnotes
fora de `columns()` continuam a ser desenhadas no fundo da página.

### 4.5 Overflow de nota grande

Numa coluna única, notas grandes continuam a ser emitidas (mecanismo P305).
Em colunas, uma nota maior do que o espaço restante é **silenciosamente
descartada**. Este é um scope-out conhecido: o tratamento de overflow
multi-página/coluna requer extensão ao algoritmo de colunas (salvaguarda e
re-emissão de remainder entre colunas/páginas).

---

## 5. Testes e Linter

```bash
cargo test --workspace --release
```

Resultado: todos os testes passam.

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

---

## 6. Conclusão

- [x] Sonda completa antes de código.
- [x] Notas de rodapé posicionadas na coluna correcta (`columns(2)`).
- [x] Caso de uma coluna sem regressão.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [ ] Overflow de nota grande em colunas — scope-out documentado.
- [ ] `#set page(columns:)` — não implementado; requer passo separado se
      necessário.

---

## 7. Próximo Passo

P538 — confirmação de fecho dos seis itens de alto impacto (P532–P537), incluindo
os três pontos deixados em aberto.
