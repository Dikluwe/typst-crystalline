# Relatório — typst-passo-845: texto com `\n` trunca o shaping (achado #55)

**Data:** 2026-07-22
**Passo:** P845 — origem: achado #55 de P831 (lote 5)

## Proveniência

- **Commit HEAD:** `43c252f2b` (chore: P844 — …4625/709, lint 0)
- **Estado da árvore durante as medições "antes":** working tree limpa sobre `43c252f2b` (`git status --porcelain` vazio).
- **Estado da árvore durante as medições "depois":** working tree não commitada com exactamente 5 ficheiros alterados (`git diff --stat`):
  - `03_infra/src/shaper.rs` (implementação + testes)
  - `03_infra/src/font_metrics.rs` (`edge_offset_pt` → `pub(crate)`)
  - `03_infra/src/fallback_fonts.rs` (hashes de linhagem — bug multi-`@prompt` corrigido manualmente)
  - `00_nucleo/prompts/infra/shaper.md` (secção P845; scope-out P484 revogado)
  - `00_nucleo/prompts/infra/font_metrics.md` (nota P845)
- **Binários:** vanilla `lab/typst-original/target/release/typst` (jun 29); cristalino `./target/release/typst` rebuildado antes de cada ronda "depois". Fixtures em `temp/p845/`.

## Baseline (confirmada no arranque)

- `cargo test -p typst-core` → **4625 passed, 0 failed** (2 ignored)
- `cargo test -p typst-infra` → **709 passed, 0 failed** (5 ignored)

## Passo 1 — Sonda (medição "antes")

### Caso mínimo — `temp/p845/minimo.typ` (`#"a\nb\nc"`)

```
$ lab/typst-original/target/release/typst compile minimo.typ vanilla-minimo.pdf
$ ./target/release/typst minimo.typ -o cristalino-minimo.pdf
$ pdftotext vanilla-minimo.pdf -        → a / b / c     (3 linhas)
$ pdftotext cristalino-minimo.pdf -     → a             (só a 1ª linha)
```

### Extensão do problema (os 3 caminhos, como previsto no prompt)

| Caso | Fixture | Vanilla | Cristalino (antes) |
|------|---------|---------|---------------------|
| string literal | `minimo.typ` | `a b c` | `a` |
| `read()` multilinha | `leitura.typ` (`#read("dados.txt")`) | `linha1 linha2 linha3` | `linha1` |
| `#eval` de string | `eval.typ` | `x y z` | `x` |

Confirmado: a causa é do **shaper**, não de `loading` — qualquer texto com `\n` embutido é truncado na primeira linha.

### Causa raiz

`03_infra/src/shaper.rs`, `bidi_runs()` (antes: linha ~865): `let para = &bidi.paragraphs[0];` — o unicode-bidi parte o texto em parágrafos nos separadores de classe B (`\n`, `\r`, U+2028, …), mas só o primeiro era shapeado.

**Esquecimento ou decisão?** Decisão consciente e **documentada**: o L0 `00_nucleo/prompts/infra/shaper.md` tinha no scope-out de P484 a linha *"Múltiplos parágrafos (`paragraphs[0]` assume 1 parágrafo por `FrameItem::Text`)"*. Este passo revoga esse scope-out (linha riscada no L0 + secção nova).

**Vanilla:** `typst-layout/src/inline/linebreak.rs:69` define `Breakpoint::Mandatory` (quebra obrigatória após `\n`); `shaping.rs:675-690` trata o break antes de um `\n` (o `\n` não tem glifo). I.e., no vanilla o `\n` vira quebra de linha real do inline layout. No cristalino, L1 (`layout/text.rs`) divide o texto só por espaços — a "palavra" com `\n` chega inteira ao shaper como um único `FrameItem::Text`; a quebra tem de nascer no shaper (L3), como o prompt manda.

**Detalhe de implementação (fonte unicode-bidi 0.3.18, `lib.rs:347-360`):** "The paragraph separator is kept with the previous paragraph" (regra P1 do UAX#9) — o range de cada parágrafo inclui o `\n` final, que tem de ser excluído do range shapeado.

## Passo 2 — Implementação

### Diff (resumo; `git diff` completo na árvore)

1. **`bidi_runs(text) -> Vec<Vec<BidiRun>>`** (antes `Vec<BidiRun>`): itera TODOS os `bidi.paragraphs`; um entry por parágrafo, na ordem; parágrafo vazio (`\n\n`) → `Vec` vazio. O separador de classe B final é excluído do range shapeado (loop com `chars().next_back()` + `bidi_class(last) == BidiClass::B`, correcto para separadores multi-byte como U+2028).
2. **`try_shape`**: por parágrafo, `x` reinicia e `y = pos.y + para_idx × line_advance`. `line_advance = top-edge + |bottom-edge| + leading` — a MESMA fórmula do avanço de linha do Layouter em L1 (P762, `cursor.rs:362-370`) e os mesmos edges por omissão (`cap-height`/`baseline`), reutilizando `font_metrics::edge_offset_pt` (agora `pub(crate)`); leading = `style.leading` resolvido, default 0,65em; métricas da primeira primária resolvida. Calculado antes de mover `primary`/`face_cache` para o `CandidateSet` (borrow checker).
3. **`shaped_width`**: largura multilinha = **max** das larguras de linha (não a soma) — cada parágrafo é uma linha independente; para texto de 1 parágrafo o valor é idêntico ao anterior.
4. Testes P484 adaptados ao novo tipo de retorno (semântica inalterada).

**Nota de iteração:** a primeira versão usava `(ascender − descender)` cru — avanço de 19.69pt vs 14.388pt do vanilla e das linhas normais do documento. Corrigido para os mesmos edges do P762; o avanço passou a bater com o vanilla ao arredondamento (ver medições).

### Testes primeiro (RED)

5 testes novos escritos contra a API nova; RED confirmado por (a) reprodução PDF literal acima e (b) falha de compilação dos testes contra a assinatura antiga (`Vec<BidiRun>`). Testes: `p845_bidi_runs_tres_linhas_tres_paragrafos`, `p845_bidi_runs_linhas_vazias_consecutivas`, `p845_bidi_runs_sem_newline_um_paragrafo`, `p845_try_shape_multilinha_empilha_linhas`, `p845_shaped_width_multilinha_max_das_linhas`.

## Passo 3 — Validação (medição "depois")

### Casos do Passo 1 + casos gerais (conteúdo `pdftotext`)

| Caso | Vanilla | Cristalino (depois) | Bate? |
|------|---------|---------------------|-------|
| `minimo.typ` (`#"a\nb\nc"`) | `a b c` | `a b c` | ✅ |
| `leitura.typ` (`read()` 3 linhas) | `linha1 linha2 linha3` | idem | ✅ |
| `eval.typ` (`#eval` de string) | `x y z` | idem | ✅ |
| `cinco.typ` (6 linhas via `#"a\n…\nf"`) | `a b c d e f` | idem | ✅ |
| `seislinhas.typ` (`read()` de 6 linhas) | `l1…l6` | idem | ✅ |
| `vazias.typ` (`#"a\n\nb"`) | `a b` (linha vazia no meio) | idem | ✅ |
| `simples.typ` (sem `\n`) | `sem newline aqui` | idem | ✅ |

### Posições verticais (`pdftotext -bbox`, yMin)

```
vanilla  minimo: a@68.270160  b@82.658160  c@97.046160   (avanço 14.388pt)
cristalino:      a@68.271000  b@82.659000  c@97.047000   (avanço 14.388pt)
vanilla  vazias: a@68.270160  b@97.046160  (linha vazia salta 1 avanço)
cristalino:      a@68.271000  b@97.047000  (idêntico)
```

Posições e espaçamento batem com o vanilla ao arredondamento de impressão do `pdftotext`.

### Sonda de regressão — texto sem `\n`

Binário "antes" reconstruído (revertendo temporariamente os 2 ficheiros de código, com cópias de segurança, e rebuildando) sobre os fixtures `normal.typ` (parágrafo corrido com quebra automática), `normal2.typ` (`#set text(size:)` + bold/italic) e `simples.typ`:

```
$ diff <(pdftotext -bbox antes-N.pdf - | grep '<word') <(pdftotext -bbox depois-N.pdf - | grep '<word')
→ IDENTICO nos 3 fixtures (todas as palavras e posições)
```

### Suítes completas (comando + contagem)

| Suíte | Antes | Depois | Delta |
|-------|-------|--------|-------|
| `cargo test -p typst-core` | 4625 passed / 0 failed | **4625 passed / 0 failed** | = |
| `cargo test -p typst-infra` | 709 passed / 0 failed | **714 passed / 0 failed** | +5 (testes P845) |

### Lint / L0

- L0 actualizado: `00_nucleo/prompts/infra/shaper.md` (secção P845; scope-out P484 riscado como revogado) e `00_nucleo/prompts/infra/font_metrics.md` (nota de visibilidade).
- `crystalline-lint --fix-hashes .` → actualizou hashes; **o bug multi-`@prompt` manifestou-se** em `fallback_fonts.rs` (2 `@prompt`): o fixer escreveu o hash de `font_metrics.md` na linha do hash de `shaper.md` e deixou o antigo na segunda — corrigido manualmente (`97627a17` / `16a8a639`).
- `crystalline-lint .` → **exit 0**; restam apenas warnings V7 de prompts órfãos pré-existentes (não relacionados com este passo).

## Decisões tomadas pelo executor (não estavam no prompt)

1. **Altura da linha empilhada:** mesma fórmula do P762 (`top-edge + |bottom-edge| + leading`, edges default `cap-height`/`baseline` via `edge_offset_pt`), NÃO `ascender−descender` cru — medido que a primeira opção dava 19.69pt vs 14.388pt do vanilla; a segunda bate exactamente e é consistente com as linhas normais do próprio documento.
2. **`shaped_width` multilinha = max das linhas** (não soma) — semântica de largura de linha; para 1 parágrafo é bit-idêntico ao anterior.
3. **Separador classe B excluído do range shapeado** — evita glifo `.notdef` e `\n` cru no texto extraível (ToUnicode/pdftotext).
4. **`edge_offset_pt` promovida a `pub(crate)`** em vez de duplicar a lógica de edges no shaper.

## Limitações registadas (fora de escopo, a rever antes de fechar o tema)

- **Fluxo vertical em L1:** o Layouter continua a tratar o texto com `\n` como UMA palavra — a altura do frame e o cursor vertical não reflectem as linhas extra. Conteúdo seguinte pode sobrepor-se se as linhas excederem a linha reservada, e o salto de página não é disparado por elas. A correcção é de **render** (todas as linhas visíveis e bem espaçadas), não de fluxo. Cobrir isto exigiria partir o texto em linhas no L1 (`layout/text.rs`) — decisão de escopo para passo próprio.
- Texto terminado em `\n` não gera uma linha vazia final extra (o unicode-bidi não cria parágrafo vazio terminal) — irrelevante visualmente no caso medido.
- O caminho de fallback `FrameItem::Text` não shapeado (fonte ausente) continua a emitir o texto cru numa só linha.
- `cargo fmt -p typst-infra` reformata 11 ficheiros não relacionados (o repo não está rustfmt-clean) — NÃO aplicado; a formatação das regiões alteradas foi ajustada à mão ao estilo envolvente.

## Ficheiros tocados

- `03_infra/src/shaper.rs` — implementação + 5 testes novos + header P845
- `03_infra/src/font_metrics.rs` — `edge_offset_pt` → `pub(crate)` (+ comentário)
- `03_infra/src/fallback_fonts.rs` — hashes de linhagem (correcção manual do bug multi-`@prompt`)
- `00_nucleo/prompts/infra/shaper.md` — secção P845; scope-out P484 revogado
- `00_nucleo/prompts/infra/font_metrics.md` — nota P845
- `temp/p845/` — fixtures e PDFs de medição (não commitados)
