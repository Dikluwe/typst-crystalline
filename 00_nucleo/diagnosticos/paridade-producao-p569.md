# P569 — Paridade de produção: palavras árabes não coladas em RTL

**Status**: `CONCLUÍDO` (implementação; testes unitários passam; validação manual confirma espaços preservados)  
**Data**: 2026-07-04  
**Scope**: `03_infra/src/layout_bidi.rs`, `00_nucleo/prompts/infra/layout_bidi.md`.

---

## 1. Reformulação da pergunta

A pergunta original era: "corrigir palavras árabes coladas sem espaço em documentos RTL longos".  
Reformulamos em verificações mensuráveis:

1. `pdftotext` extrai as palavras árabes separadas (não `معلوماتقيمة.`).
2. A morfologia do texto árabe é preservada: cada palavra é um objeto legível, com o espaço entre elas visível na extracção.
3. Os testes unitários do módulo `layout_bidi` cobrem o caso mínimo (sufixo LTR, espaço solto, combinação).

---

## 2. Medições

### 2.1 Causa raiz observada

| Ferramenta | Input | Output |
|------------|-------|--------|
| `pdftotext` (antes de P569) | `#set text(lang: "ar")\nمعلومات قيمة.` | `معلوماتقيمة.` — palavras coladas, sem espaço. |
| `mutool draw -F txt` (antes de P569) | mesmo input | `معلومات قيمة.` — visualmente correcto. |

A divergência indica que o observável correcto é a **morfologia** (palavras e espaços enquanto objectos da linguagem), não a ordem mecânica dos bytes no stream PDF. A causa é a interacção entre:

- `03_infra/src/shaper.rs:508` — `bidi.visual_runs(para, line)` separa caracteres neutros (espaços) e sufixos LTR (pontuação) em runs próprios;
- itens de espaço soltos ficam capturados pelo sufixo LTR na ordem do stream, fazendo com que extratores sequenciais percam o espaço entre palavras.

### 2.2 Implementação

O Prompt L0 `00_nucleo/prompts/infra/layout_bidi.md` foi actualizado (hash `603ffda0`).  
A implementação em `03_infra/src/layout_bidi.rs` acrescenta três passos ao pipeline de reordenação bidireccional:

1. **Ordenação do vector por x crescente** (`sort_line_items`) — garante que extratores sequenciais percorrem os operadores PDF da esquerda para a direita.
2. **Coalescência de espaços no item anterior** (`coalesce_space_items`) — anexa cada espaço solto como *trailing space* da palavra visualmente anterior (menor x), colocando o espaço entre palavras na ordem do stream.
3. **Separação de sufixos LTR** (`split_ltr_suffixes_page` / `split_ltr_suffixes_line`) — divide `"قيمة."` em `"قيمة"` + `"."`, posicionando o ponto como item independente imediatamente à esquerda do texto árabe.

Pontos de medição no código:

| Ficheiro | Linha | Observável |
|----------|-------|------------|
| `03_infra/src/layout_bidi.rs:162` | `coalesce_space_items` | anexa espaço ao item anterior (trailing space). |
| `03_infra/src/layout_bidi.rs:187` | `split_ltr_suffixes_page` | chamada final da página, depois de reordenações/reflows. |
| `03_infra/src/layout_bidi.rs:392` | `split_ltr_suffix` | detecta sufixo LTR ignorando trailing spaces e recolando-os ao base. |

### 2.3 Testes unitários

Adicionados em `03_infra/src/layout_bidi.rs`:

```text
p569_ltr_suffix_split          PASS
p569_trailing_space_coalesced  PASS
p569_suffix_and_space          PASS
```

Resultado completo do módulo:

```bash
cargo test -p typst-infra layout_bidi
# running 10 tests
# test result: ok. 10 passed; 0 failed
```

### 2.4 Validação manual com o binário de release

Comando:

```bash
cargo build --release --bin typst
./target/release/typst /tmp/p569-curto-ponto.typ /tmp/p569-curto-ponto.pdf
pdftotext /tmp/p569-curto-ponto.pdf -
mutool draw -F txt /tmp/p569-curto-ponto.pdf 1
```

Input (`/tmp/p569-curto-ponto.typ`):

```typst
#set text(lang: "ar", size: 20pt)
معلومات قيمة.
```

Output `pdftotext` (depois de P569):

```text
‫معلومات‬
‫قيمة ‪.‬‬
```

Output `mutool`:

```text
. ةميق  تامولعم
```

As palavras deixaram de estar coladas. O espaço entre `معلومات` e `قيمة` é agora visível em ambos os extractores.  
A inversão visual da ordem das palavras em `pdftotext` é um comportamento conhecido e fora do scope de P569 (ver Scope-out do L0).

Input longo (`/tmp/p569-longo.typ`):

```typst
#set text(lang: "ar", size: 14pt)
هذا نص طويل باللغة العربية. يحتوي على معلومات قيمة. نأمل أن يعمل بشكل صحيح.
```

Output `pdftotext`:

```text
‫نأمل أن‬
‫يحتوي على معلومات قيمة ‪.‬‬
‫هذا نص طويل باللغة العربية ‪.‬‬
‫يعمل بشكل صحيح ‪.‬‬
```

Output `mutool`:

```text
 نأ لمأن. ةميق تامولعم ىلع يوتحي. ةيبرعلا ةغللاب ليوط صن  اذه
. حيحص لكشب لمعي
```

Nenhuma palavra está colada; os espaços entre palavras são preservados. Os pontos finais aparecem como items LTR separados (visíveis como `‪.‬` em `pdftotext`), o que é o comportamento pretendido da separação de sufixos.

---

## 3. Decisão / Classificação

| Questão | Decisão | Base de medição |
|---------|---------|-----------------|
| Separar sufixos LTR do final de items RTL? | **Sim** | `pdftotext` perde o espaço quando o ponto fica colado ao run árabe no stream. |
| Coalescer espaços no item anterior (trailing space)? | **Sim** | Colocar o espaço como leading space do item seguinte faz com que o ponto capture o espaço quando se separa o sufixo. |
| Fazer a correcção no shaper (`visual_runs`)? | **Não** | Experiências anteriores mostraram que alterar a ordem dos glifos no shaper quebra a renderização visual. A correcção é feita ao nível de `FrameItem::Text`, antes do shaping. |
| Aceitar inversão visual na extracção `pdftotext`? | **Sim** | A paridade é com a **linguagem** (espaços e palavras preservados), não com a ordem mecânica dos bytes (ADR-0107). |

---

## 4. Inferências e riscos

1. **Inferência**: a perda de espaço era causada pela combinação de espaços neutros soltos e sufixos LTR; separar ambos resolve o problema.  
   **O que a refutaria**: `pdftotext` continuar a colar palavras — não observado.

2. **Risco baixo**: o algoritmo de sufixo usa heurística LTR (ASCII não-letra + classes bidi L/EN/AN). Pontuação árabe (e.g. `،`) não é separada, o que é o comportamento correcto.  
   **Excepção**: texto que termine em pontuação árabe seguida de ponto ASCII pode ser separado de forma inesperada; não observado nos casos de teste.

3. **Risco a monitorar**: `cargo test -p typst-infra` falha em 17 testes fora do scope de P569 (ver secção 5).  
   **Actualização (P571):** a afirmação de que as 5 falhas em `p307b_snapshot_tests` eram "pré-existentes, anterior a P569" está incorrecta. P571 confirmou que o commit P569 isolado só apresenta a falha preexistente em `07-multi-feature`; as falhas `01`, `02`, `03` e `09` foram causadas por código órfão em L1 (`layout_space` em `01_core/src/engine/layout/cursor.rs`, `mod.rs`, `text.rs`) que existia no working tree quando P569 foi validado, mas que nunca foi commitado nem tem Prompt L0. Esse código L1 foi descartado em P572.

---

## 5. Estado dos testes

Testes directamente afectados por P569 (todos PASS):

```bash
cargo build --release --bin typst
cargo test -p typst-infra layout_bidi
crystalline-lint .
```

Resultado do linter:

```text
✓ No violations found
```

Testes do crate `typst-infra` com falhas **fora do scope** (não causadas pelo commit P569):

- 12 `integration_tests` de `align`, `grid` e `place`;
- 1 `p307b_snapshot_test` (`07-multi-feature`), preexistente ao commit P569 e corrigido posteriormente em P570.

**Nota (P571/P572):** as falhas adicionais `01`, `02`, `03` e `09` observadas no working tree durante a validação de P569 eram causadas por código órfão em L1 (`layout_space`), não pelo commit P569. Esse código L1 foi descartado em P572.

Total no momento da escrita de P569: 17 falhas em 591 testes (574 passaram, 5 ignorados).

```bash
cargo test -p typst-infra
# test result: FAILED. 574 passed; 17 failed; 5 ignored
```

---

## 6. Conclusão

P569 está **activo e validado**. A separação de sufixos LTR e a coalescência de espaços no item anterior eliminam a colagem de palavras árabes observada em extratores sequenciais, enquanto preservam a morfologia do texto. O linter reporta zero violations; os testes unitários do módulo `layout_bidi` passam.  
**Actualização (P571/P572):** a solução de P569 em L3 provou-se suficiente sozinha; uma tentativa paralela em L1 (`layout_space`) foi identificada como órfã e redundante, e descartada.
