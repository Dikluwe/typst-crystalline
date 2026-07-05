# Relatório de Sonda — P562: Ordem Visual RTL das Palavras na Linha

**Passo:** 562  
**Data de execução:** 2026-07-04  
**Tipo:** Sonda A.0 (ADR-0114) + proposta de Prompt L0  
**Foco:** Determinar onde e como corrigir a ordem visual das palavras árabes na linha, sem implementar código antes de L0 aprovado.

---

## Comandos executados

```bash
# Localizar referências a bidi/RTL no layout (L1)
grep -n "layout_word\|cursor_x\|advance.*word" 01_core/src/rules/layout/cursor.rs | head -20

# Verificar onde bidi_runs é usado no projecto
grep -R "bidi_runs\|unicode_bidi\|unicode-bidi" . --include="*.rs" | head -30

# Localizar o arm Content::Text no layout monolito
grep -n "Content::Text" 01_core/src/rules/layout/mod.rs

# Confirmar a pipeline: layout → shape → export
grep -n "shape_document\|layout_with_introspector_and_metrics" 03_infra/src/pipeline.rs
```

---

## Resultado da sonda por critério

### 1. O Layouter avança `cursor_x` sempre para a direita?

**PASS** — confirmado sem noção de direcção.

- `01_core/src/rules/layout/cursor.rs:134`:
  ```rust
  self.regions.current.cursor_x += w;
  ```
  Emite o texto em `cursor_x` actual e avança para a direita.

- `01_core/src/rules/layout/cursor.rs:149`:
  ```rust
  self.regions.current.cursor_x += w;
  ```
  `layout_chunk` repete o mesmo padrão LTR.

- `01_core/src/rules/layout/text.rs:191-196`:
  ```rust
  layouter.layout_word(part);
  // ...
  layouter.regions.current.cursor_x += layouter.space_width();
  ```
  As palavras são dispostas na ordem do `split(' '')`, da esquerda para a direita.

### 2. `bidi_runs` está acessível no Layouter (L1)?

**FAIL** — `bidi_runs` só existe em L3.

- `03_infra/src/shaper.rs:121`:
  ```rust
  let runs = bidi_runs(text.as_str());
  ```

- `03_infra/src/shaper.rs:490`:
  ```rust
  fn bidi_runs(text: &str) -> Vec<BidiRun> { ... }
  ```

Nenhuma referência a `bidi_runs`, `unicode_bidi` ou direcção existe em
`01_core/src/rules/layout/`. O `Layouter` não tem campo nem trait para
bidi.

### 3. Existe informação de direcção que sobreviva até ao Layouter?

**FAIL** — não há campo de direcção no `TextStyle` ou no `Layouter`.

- `01_core/src/rules/layout/text.rs:54-60` — o estilo decodifica `lang`
  (`text.lang`), mas `lang` indica o idioma, não a direcção base do
  parágrafo nem a direcção de cada run.
- O `Layouter` trata `Content::Text` como uma sequência LTR pura
  (`01_core/src/rules/layout/mod.rs:727` → `text::layout(self, text)`).

---

## Decisão de desenho

Com base nas medições acima, a implementação deve ser uma **passagem
posterior em L3**, inserida entre `layout` e `shape_document` na pipeline
(`03_infra/src/pipeline.rs:346-374`).

**Razão:**

- O `Layouter` (L1) não tem acesso a `unicode-bidi` nem a bytes de
  fonte; trazer essa lógica para L1 violaria ADR-0120 e complicaria o
  hot path de quebra de linha.
- A pipeline já tem um padrão estabelecido de passagem posterior
  (`shape_document`, P482).
- Reordenar antes do shaping preserva a unidade da palavra e evita
  reconstruir `FrameItem::TextShaped` fragmentados por fallback de
  fonte (P515/P534).

**Módulo proposto:** `03_infra/src/layout_bidi.rs`  
**API proposta:** `pub fn reorder_bidi_document(doc: PagedDocument) -> PagedDocument`  
**Algoritmo resumido:**

1. Agrupar items de cada página por linha visual (mesma baseline y).
2. Para cada linha, concatenar os textos dos `FrameItem::Text`.
3. Se `unicode_bidi::BidiInfo` detectar runs RTL, usar `visual_runs`
   para obter a ordem visual.
4. Reordenar os items de texto da linha e ajustar as coordenadas x
   para refletir a ordem visual, sem alterar y nem a quebra de linha.
5. Itens não-texto (`Shape`, `Image`, etc.) mantêm posição relativa.

---

## Prompt L0 proposto

`00_nucleo/prompts/infra/layout_bidi.md`

O ficheiro foi redigido com a especificação completa, medições que a
fundamentam e casos de teste mínimos. O **hash do código está
pendente** — deve ser calculado após a implementação e guarda pelo
dono, de acordo com o Protocolo de Nucleação.

---

## Estado da sequência de RTL

| Camada | Passo | Estado |
|--------|-------|--------|
| Shaping (formas das letras) | P484, P521 | Fechado |
| Fonte embutida no PDF | P560 | Fechado |
| Ordem visual das palavras na linha | P562 | Sonda completa; aguarda L0 |

---

## Bloqueio para implementação

**A implementação de código L1/L3 não pode prosseguir sem:**

1. Dono guardar o Prompt L0 `00_nucleo/prompts/infra/layout_bidi.md`.
2. Dono calcular e inserir o hash do código no cabeçalho do L0.
3. Confirmar aqui o hash para que a implementação continue.

Até lá, nenhum ficheiro `.rs` foi alterado.
