# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/shaper/font_selection.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `3729ae2e0992ad276da045017d2a7b2aad046dc2ad0202f45716b23e27d6187f`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

---
Hash do Código: 188eae9c
prompt: infra/shaper/font_selection
layer: L3
created: 2026-09-26
adr: ADR-0120, ADR-0127, ADR-0109, ADR-0129, ADR-0130
---

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:faf6c20021b467fdb2a864625f4f6dd4386b5ef28c137b946e5cb4e4ce073d52

# Prompt L0 — infra/shaper/font_selection

## Medição anterior à decisão

Extração estrutural de `03_infra/src/shaper.rs:206-263, 688-1055` no HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado. Proveniência da auditoria registada em `00_nucleo/diagnosticos/shaper-atomizacao-2026-09-26.md` (hash da fonte anterior, diff HEAD --stat, untracked e instante). A suíte baseline shaper passou sem alterações. Estas referências indicam a posição pré-extração, não linhas dos arquivos novos.

## Decisão e contrato da unidade

Possuir resolução da FontList, fallback por classe e MATH, cobertura lazy, scoring, índice local denso e segmentação por script/fonte. Centralizar a cadeia primária usada por medição e emissão, sem alterar ordem de consultas ou resultados. A cadeia MATH adicional continua no ramo em que a lista declarada resolve; não acrescentar outro ramo. Não carregar slots globais intermediários. Consumir o núcleo fallback-selection com o pin indicado.

Separação mecânica em L3, sem alteração de contrato público, comportamento por defeito ou fase do pipeline (ADR-0127). A estrutura Rust é mecânica, não uma nova decisão de paridade linguística (ADR-0107/0108). Cada consumer tem este owner exclusivo; referências L0 são canônicas.

## P1404 — fallback esparso para controles sem glifo

Medição focal no estado não commitado de P1404 mostrou que o texto mínimo
`a\u{2060}b` esgotava o teto virtual de 512 MiB, enquanto `ab` terminava. A
causa é interna: `CandidateSet::covering_all` confundia o índice absoluto do
slot no `FontBook` com a posição densa do vetor local `fallback`, preenchendo
todos os índices intermediários quando um caractere sem cobertura (como WORD
JOINER) só aparecia num slot distante.

O vetor local é um interner esparso: cada slot global não primário ocupa
exatamente uma posição local, reutilizada quando repetido. O índice devolvido
continua no namespace `primary.len() + posição_local`; jamais depende da
grandeza numérica do slot global. Um controle sem glifo avança pelo fallback
finito e não causa alocação proporcional ao maior índice do catálogo.

Os controles de formatação usados por `outline.entry.inner` — WORD JOINER
U+2060, RIGHT-TO-LEFT EMBEDDING U+202B e POP DIRECTIONAL FORMATTING U+202C —
permanecem no Content e chegam à análise bidi/quebra. Eles não consultam
cobertura: entram no sub-run corrente ou, quando isolados, no primeiro
candidato primário, sem percorrer o catálogo. Assim o shaper pode emitir o
glifo invisível e preservar o codepoint no `ToUnicode`/texto extraível sem
reabrir a alocação proporcional ao índice global que causou o OOM. Texto
latino, seleção/scoring de fontes e caracteres invisíveis alheios a essa lista
permanecem controles. Os testes focais exigem idempotência do interner, tamanho
linear no número de slots distintos, reconhecimento exato dos três controles e
preservação de U+2060 em extração PDF sob o teto de memória.


## Resolução de fonte

```rust
fn resolve_slot(world: &dyn World, font_list: &FontList) -> Option<usize>
```

Itera `font_list.as_slice()`, para cada família chama
`world.book().select_pattern(&family.name, &FontVariant::default())`.
Verifica que `world.font(idx)` devolve `Some` antes de aceitar o slot.
Mesma lógica de `resolve_font` em `pipeline.rs`.


## §P515 — Font fallback por caractere

**Data:** 2026-06-30

O shaper passa a suportar fallback por caractere quando a fonte principal
não cobre todos os codepoints do texto. Isto é necessário para documentos
com scripts mistos (ex: latino + CJK + emoji).

### Heurística de cobertura

```rust
fn face_covers_char(face: &rustybuzz::Face, c: char) -> bool
```

- Um caractere é coberto se `face.glyph_index(c)` (via ttf_parser) devolver `Some`.
- Alternativa: iterar `face.glyph_count()` — usar `ttf_parser::Face` para o lookup.

### `shape_with_fallback`

Substituir `try_shape` por um pipeline que:

1. Resolve a fonte principal via `resolve_slot` (primeira família de `FontList` que resolva).
2. Para cada run bidireccional (P484), percorre os caractes e quebra o run em sub-runs sempre que a cobertura da fonte actual mudar.
3. Para cada sub-run, tenta resolver uma fonte que cubra todos os caracteres do sub-run, iterando `FontList` e, opcionalmente, fallback genérico do sistema (quando `fontdb` estiver activo).
4. Shape cada sub-run com a sua fonte.
5. Concatena os glifos na ordem visual, preservando `cluster` absoluto no texto original.

### Estrutura do resultado

`FrameItem::TextShaped` continua a ser uma única entidade por `FrameItem::Text` original. Os glifos podem vir de múltiplas faces; o PDF emit (multifont, P515+) usa o `glyph_id` e a fonte correcta para cada glifo.

Para a Fase 1 do P515, o shaper produz **um `TextShaped` por fonte** (i.e., o item original é substituído por múltiplos `FrameItem::TextShaped` consecutivos com o mesmo `pos` base e offsets acumulados) ou mantém um único `TextShaped` com metadados de fonte por glifo. A decisão concreta é delegada ao Prompt L0 `infra/export/font_subset` e ao refactor do `PdfBuilder` para multi-fonte per-glyph.

### Scope-out P515

- Fallback para fontes do sistema quando `FontList` não cobre — requer `fontdb` activo (Prompt L0 `infra/fontdb`).
- Escolha de peso/estilo no fallback — usa `FontVariant::default()`.
- Shape de texto vertical.

### Testes adicionados P515

- `p515_shape_document_latin_only_single_font`: latim coberto → 1 TextShaped.
- `p515_shape_document_mixed_fallback_splits`: latim + caractere ausente → ≥2 TextShaped ou glifos com fontes distintas.
- `p515_try_shape_missing_glyph_does_not_panic`: caractere sem cobertura em nenhuma fonte → preserva Text (não panic).

---


## P534 — Fallback de fonte por script/cobertura (multi-script)

**Data:** 2026-07-02

Correcção de P515: o fallback anterior limitava-se às famílias declaradas na
`FontList`; caracteres fora da cobertura dessas famílias perdiam-se. P534
segmenta o texto por script Unicode e, para cada segmento, procura primeiro nas
famílias declaradas e depois em todas as fontes disponíveis no `FontBook`
(fontdb), escolhendo a primeira que cubra todos os caracteres do segmento.

### Segmentação por script

```rust
fn script_runs(run: &BidiRun) -> Vec<ScriptRun>
```

Usa `unicode-script::{Script, UnicodeScript}`. Caracteres com script `Common` ou
`Inherited` herdam o script do segmento actual (`is_compatible`). Mudanças de
script forçam uma quebra de sub-run, mesmo que a mesma fonte cubra ambos.

### Escolha por cobertura

Para cada segmento de script:

1. Itera as `primary_candidates` (famílias da `FontList`, na ordem declarada).
2. Se alguma cobrir todos os caracteres do segmento, usa-a.
3. Caso contrário, itera `fallback_candidates` (todo o `FontBook`, na ordem de
   descoberta), escolhendo a primeira que cubra todos os caracteres.
4. Se nenhuma fonte cobrir o segmento inteiro, divide o segmento no ponto onde
   a cobertura muda e repete.

Implementação concreta: `split_run_by_font` percorre os caracteres de cada
`BidiRun`, mantendo o script efectivo e o índice da fonte candidata. Em cada
mudança de script ou de fonte, fecha a sub-run actual.

### Caching de cobertura

Para evitar re-parse da face para cada caractere, `try_shape` mantém um cache
`HashMap<char, usize>` do primeiro candidato (primary+fallback) que cobre cada
caractere. O cache é local a cada chamada de `try_shape`.

### Scope-out P534

- Fontes de cor para emoji (COLR/CPAL) — mecanismo de renderização, não escolha
  de fonte.
- Fundir blocos `BT...ET` consecutivos da mesma fonte — scope-out; cada
  `FrameItem::TextShaped` continua a gerar o seu próprio bloco.
- ~~Ordenação sofisticada de fallback por script (fontique) — usa ordem do
  `FontBook`.~~ **Revogado em P838** — ver secção P838 abaixo.

### Testes adicionados P534

- `p534_split_run_by_font_respects_script_boundaries`: segmentação por script.
- `p534_shape_mixed_script_system_fallback`: texto latim+CJK+árabe com system
  fonts produz múltiplos `TextShaped` com fontes distintas.

---


## P838 — Fallback global com scoring de similaridade (paridade vanilla)

**Data:** 2026-07-22

Correcção do achado #24 de P831: o fallback global escolhia a primeira fonte
do `FontBook` (ordem de índice) que cobrisse o caractere — ex.:
`Droid Sans Fallback` para CJK — enquanto o vanilla escolhe por scoring de
similaridade (`book.rs:94-185`), resultando em `Noto Sans CJK JP` para o
mesmo texto.

### Desenho

- O scoring vive em L1: `FontBook::select_fallback(like, variant, candidates)`
  (ver `entities/font-book.md`), replicando `find_best_variant`/`similarity`/
  `distance` do vanilla.
- `like` = `FontInfo` da primeira primária resolvida (equivalente ao
  `ctx.first()` do vanilla); `variant` = variante do `TextStyle`.
- `CandidateSet` recebe `like` e `variant` no constructor. No passo de
  fallback global de `covering_run`, o vencedor de `select_fallback` é movido
  para a frente da lista de candidatos antes de `best_covering_run` — assim,
  em empate de comprimento de run (caso CJK: todos os candidatos cobrem o run
  inteiro), vence o scoring do vanilla. O critério de run mais longo (P543)
  mantém-se como primário.

**Nota:** o scan de fallback carrega as faces lazy (cache `FaceCache`), tal
como antes; apenas a *ordem* de preferência muda.

---


## P875 — Filtro de fallback por cobertura Unicode (paridade de I/O)

**Data:** 2026-07-23

**Problema medido (P873):** quando um caractere não é coberto pelas fontes
primárias, `CandidateSet::covering_all` percorre **todo** o `FontBook`
(~1086 fontes de sistema) chamando `load_fallback` para cada slot. Para
colecções `.ttc` grandes (`NotoSansCJK`, `NotoSerifCJK`), cada `load_fallback`
lê o ficheiro inteiro (~19–27 MB) para uma única face. Em documentos math,
símbolos matemáticos/gregos não cobertos por CJK fazem com que se leiam
dezenas de MB de ficheiros CJK só para confirmar que nenhum deles tem o
glifo.

**Solução:** usar o bitmap `Coverage` de `FontInfo` (ver `entities/font-book.md`
e `infra/fonts.md` P875) para filtrar candidatos **antes** de carregar a face.

### `CandidateSet::covering_all` (P875)

```rust
fn covering_all(&mut self, c: char) -> Vec<usize>
```

1. Iterar as primárias como antes.
2. Obter os candidatos cujo `coverage` cobre o bloco de `c` via
   `self.world.candidates_for_char(c)` (P880 — `SystemWorld` calcula a
   cobertura lazy; `MockWorld`s usam o `FontBook` directamente).
3. Para cada candidato desta lista filtrada, lazy-load a face e confirmar
   `face_covers_char(...)` (o bitmap é aproximado por bloco).
4. Candidatos cujo bitmap não cobre o bloco de `c` são **ignorados** — não
   são carregados.

Isto reduz drasticamente o número de faces abertas para caracteres cuja
cobertura é claramente ausente no sistema (ex.: símbolos matemáticos não têm
nenhum bit activo nos blocos CJK).

### Preservação de semântica

- O critério de escolha final continua a ser `select_fallback` (P838) sobre os
  candidatos que de facto cobrem o caractere.
- A ordem de preferência não muda: `candidates_for_char` preserva a ordem do
  `FontBook`, e o vencedor do scoring é movido para a frente em
  `covering_run` como antes.
- O fallback para caracteres com cobertura dispersa (ex.: emoji, CJK real)
  continua a funcionar, porque o bitmap inclui os blocos correctos.

### Testes adicionados P875

- `p875_candidates_for_char_excludes_uncovered_block`: fonte sem cobertura de
  um bloco não aparece em `candidates_for_char` para esse caractere.
- `p875_covering_all_does_not_load_irrelevant_fonts`: documento math não abre
  faces de `.ttc` CJK para símbolos matemáticos (medido via contagem de
  `openat`/`read` ou instrumentação equivalente).

---


## P555 — Fallback preserva a classe da fonte (serifa / sem serifa)

**Data:** 2026-07-03

P554 mudou a fonte por defeito do cristalino para `FreeSerif`. Se `FreeSerif`
não estiver disponível no ambiente, o shaper deve tentar outras serifas antes
de recair em sans-serif, preservando a classe visual escolhida.

### Catálogo por classe

As listas ordenadas e a heurística nominal pertencem a
`infra/fallback_fonts.md`. O shaper consome `fallback_font_list_for` e possui
a decisão dinâmica de anexar a cadeia antes do fallback global.

### Heurística de classe

A classe é inferida a partir do nome da primeira família declarada em
`style.font`:

- Se o nome (case-insensitive) contiver `"serif"` → usar `DEFAULT_FALLBACK_FONTS_SERIF`.
- Se contiver `"sans"` → usar `DEFAULT_FALLBACK_FONTS_SANS`.
- Caso contrário → manter comportamento anterior (`DEFAULT_FALLBACK_FONTS_SANS`).

Nomes em regex ou sem indicação de classe usam a lista sans por defeito.

### Implementação

- O módulo `03_infra/src/fallback_fonts.rs` centraliza as constantes e a
  função `fallback_font_list_for(name: &str) -> &'static [&'static str]`, sob
  owner próprio.
- `shaper.rs` e `font_metrics.rs` importam deste módulo e removem as suas
  definições locais duplicadas.
- Em `try_shape`, quando as fontes primárias não resolvem, itera a lista
  escolhida antes de recair no fallback carácter-a-carácter do `FontBook`.

### Testes adicionados P555

- `p555_fallback_font_list_serif_for_freeserif`: `FreeSerif` → lista serif.
- `p555_fallback_font_list_sans_for_dejavu_sans`: `DejaVu Sans` → lista sans.
- `p555_fallback_font_list_default_for_unknown`: nome sem indicação → lista sans.

---


## §P783/P784 — Cadeia de fallback específica de modo matemático

**P783** implementou o mecanismo (nunca documentado aqui — débito próprio,
fechado retroactivamente por este L0). **P784** fez a verificação visual
real que P783 nunca fez, encontrou dois bugs, e corrigiu.

### Mecanismo (`try_shape`/`shaped_width`, após resolver `primary`)

Depois de resolver as primárias declaradas (`font_list`), decide-se se se
adiciona `fallback_fonts.rs::math_fallback_font_list()`
(`New Computer Modern Math → Libertinus Serif → fontes de emoji`, paridade
`math::families()` vanilla) como **primárias adicionais** — com prioridade
sobre o fallback global lazy (`CandidateSet::covering_all`, todo o
`FontBook` em ordem de índice):

```rust
let primary_has_math = /* primária já resolvida tem tabela MATH própria */;
if style.math || primary_has_math {
    for family in math_fallback_font_list() { /* resolve e adiciona a `primary` */ }
}
```

Dois gatilhos independentes (OR):
- **`style.math`** (P784) — `true` sempre que o texto vem do motor de layout
  matemático (`layout/equation.rs`, ver `entities/layout_types.md` §P784).
  **Este é o gatilho que importa na prática** — a fonte de corpo por omissão
  (`Libertinus Serif`) não tem tabela MATH, então `primary_has_math` sozinho
  nunca disparava no caso comum.
- **`primary_has_math`** (P783 original) — a fonte já resolvida declara MATH
  própria (ex.: utilizador define `font: "New Computer Modern Math"`
  explicitamente, mesmo fora de `$...$`). Mantido como gatilho adicional,
  não removido.

### Bug 1 (P783, corrigido por P784) — gate nunca disparava

P783 só tinha `primary_has_math` — **nunca verificado com um glifo
matemático real ausente em `Libertinus Serif`** (o caso de teste de P783,
`frac(a,b)`/`x^2_1`, só usa ASCII já coberto pela primária, então o
`mutool trace` nunca mudou — "correcto mecanicamente" não era evidência).
P784 mediu com `⨿` (U+2A3F, ausente em `Libertinus Serif`, presente em
`NewComputerModernMath`) e confirmou visualmente: o glifo embutido era
**errado** (`uni27F8`, uma seta dupla, de uma fonte de sistema aleatória —
`MathJax_Main-Regular` — apanhada pelo fallback global, porque a cadeia
math nunca chegava a ser tentada). Corrigido com `style.math` (acima).

### Bug 2 (independente, também descoberto por P784) — nome de família errado

Mesmo com `style.math` a disparar o bloco, `resolve_candidates` continuava
a devolver **zero candidatos** para `"New Computer Modern Math"` (o nome
que P783 hardcoded, com espaços). Medido por leitura directa da tabela
`name` do ficheiro embutido (`fontTools`/`ttf_parser`, nameID 1 `FAMILY`):
o nome real é **`"NewComputerModernMath"`, sem espaços** — inconsistência
do próprio ficheiro de fonte upstream (`Libertinus Serif` tem espaços;
`New Computer Modern *` não tem — confirmado nos dois). Corrigido em
`fallback_fonts.rs::DEFAULT_FALLBACK_FONTS_MATH` (`"NewComputerModernMath"`)
e em `embedded_fonts.rs::embedded_font_group` (ver `embedded_fonts.md`
§P784, mesma causa-raiz, mesma correcção).

**P840** — a correcção de P784 era um *workaround* para a ausência da
tabela de exceções (`fonts.rs::find_exception`, port do vanilla
`exceptions.rs`, achados #29/#30 de P831). No vanilla, a exceção para os
PS names `NewCMMath-*` regista a família `"New Computer Modern Math"`
(com espaços) — é por isso que a cadeia `math::families()` do vanilla usa
o nome com espaços e resolve. Com a tabela portada, o FontBook regista o
mesmo nome documentado, e `DEFAULT_FALLBACK_FONTS_MATH[0]` volta a
`"New Computer Modern Math"` (com espaços) — paridade literal com
`math::families()` (`typst-library/src/math/mod.rs:179`). O ID1 cru sem
espaços já não chega ao FontBook para estas fontes.

### Critério de verificação (P784, medido)

```
$⨿$ (U+2A3F)
  → antes: glyph="uni27F8" (seta, errado), font embutido de MathJax_Main-Regular
  → depois: glyph="uni2A3F" (correcto), adv=".75" — bate exactamente com o
    adv=".75" do vanilla (mesma fonte, mesmo glifo)
  → confirmado visualmente: mutool draw mostra o glifo "⨿" correcto (não tofu,
    não glifo errado), mesma forma que o render vanilla
```




## P1466 — índice local denso e slot global no fallback

`CandidateSet::covering_all` devolve índices no vetor local de candidatos.
Depois de P1404, esse índice não é o slot global de `FontBook`; o consumer deve
resolver o candidato local e então observar seu slot/fonte associado. O teste
de coverage mantém três slots globais, torna apenas o slot 1 elegível e exige
que o único candidato local resolva para esse slot, sem exigir que o índice
local seja numericamente `1`.

Não enfraquecer a prova para mera cardinalidade: carregar o slot errado deve
continuar falhando.
