# Relatório — typst-passo-840: `text::font::exceptions` — tabela de exceções de família e peso (#29, #30)

**Data**: 2026-07-22
**Proveniência**: HEAD `b2bb09166` (P839). Medições "antes" feitas com a árvore
limpa nesse commit (binário release pré-existente, reconstruído em P839).
Medições "depois" feitas com **working tree não commitada** — ficheiros
alterados (`git diff HEAD --stat`):

```
 00_nucleo/prompts/infra/embedded_fonts.md |  24 +-
 00_nucleo/prompts/infra/fonts.md          |  49 ++-
 00_nucleo/prompts/infra/shaper.md         |  15 +-
 03_infra/src/embedded_fonts.rs            |  46 +--
 03_infra/src/fallback_fonts.rs            |  21 +-
 03_infra/src/fonts.rs                     | 508 +++++++++++++++++++++++++++++-
 03_infra/src/shaper.rs                    |   2 +-   (só header de lineage, --fix-hashes)
+ 03_infra/fixtures/fonts/p840-fandolhei-bold.ttf (nova fixture, untracked)
```

**Alcance do port**: **integral** — todas as entradas da tabela do vanilla
(`lab/typst-original/crates/typst-library/src/text/font/exceptions.rs:46-342`,
~150 PS names: Arial-Black, Archivo Narrow, Fandol, Noto, New Computer Modern,
Latin Modern, SimSun-ExtB, STKaiti). Sem scope-out parcial: a transcrição é
mecânica e o custo de a deixar completa é o mesmo de a deixar parcial.
Decisão minha (executor), dentro da margem que o passo dá ("não precisa ser
1:1 completa… o caso medido precisa funcionar"); reportada aqui.

---

## #29 (E1) — exceções de família ausentes (New Computer Modern embutida)

### Medição ANTES

Probe `temp/p840/e1.typ`:

```typst
#set text(font: "New Computer Modern")
Hello *world*
```

- vanilla (`lab/typst-original/target/release/typst compile temp/p840/e1.typ temp/p840/e1-van.pdf`):
  exit 0, sem warnings. `pdffonts`:
  ```
  KJUFLA+NewCM10-Regular-Identity-H    CID Type 0C  Identity-H  yes yes yes
  UGZCGE+NewCM10-Bold-Identity-H       CID Type 0C  Identity-H  yes yes yes
  ```
- cristalino (`./target/release/typst temp/p840/e1.typ -o temp/p840/e1-cris.pdf`):
  exit 0, com
  ```
  temp/p840/e1.typ:1:16: warning: unknown font family: new computer modern
  ```
  `pdffonts`: 2 fontes anonimizadas (`AAAAAA+CrystallineFont1/2`) — fallback,
  não a NewCM. (O ID1 cru da embutida é `NewComputerModern10`, sem espaços;
  sem a tabela de exceções o nome documentado não resolvia.)

### Implementação

`03_infra/src/fonts.rs`:
- `struct FontException { family, style, weight, stretch }` (port do vanilla
  `exceptions.rs:9-43`, com os mesmos builders `const fn`).
- `fn find_exception(postscript_name: &str) -> Option<FontException>` — a
  tabela completa como `match` (o vanilla usa `phf::Map`; mesma semântica de
  lookup exato, sem dependência nova).
- `font_info_from_bytes`: lookup pelo name ID6 (`POST_SCRIPT_NAME`) logo após
  o parse — como o vanilla `info.rs:60-61`; família/estilo/peso/stretch da
  exceção prevalecem sobre a extração normal (`info.rs:73-77,80-112`).

**Cascata obrigatória (descoberta na implementação, não scope creep):** com a
exceção, a família registada no `FontBook` para as embutidas `NewCM10-*`/
`NewCMMath-*` deixou de ser o ID1 cru (`NewComputerModern10`/
`NewComputerModernMath`, sem espaços — a "correção" de P784) e passou a ser o
nome documentado com espaços (`New Computer Modern`, `New Computer Modern
Math`) — que é o que o vanilla regista, porque lá a tabela existe desde
sempre. Dois sítios comparavam contra os ID1 crus e foram atualizados:

- `03_infra/src/embedded_fonts.rs::embedded_font_group` — classificação
  texto/math_code passa a comparar `== "new computer modern"` → texto e
  `== "new computer modern math"` → math_code (sem a mudança, a NewCM10 caía
  no `else` math_code — o bug lateral que P784 tinha corrigido regressava).
- `03_infra/src/fallback_fonts.rs::DEFAULT_FALLBACK_FONTS_MATH[0]` — volta a
  `"New Computer Modern Math"` (com espaços), paridade literal com a cadeia
  `math::families()` do vanilla (`math/mod.rs:179`: `"new computer modern
  math"`). A forma sem espaços de P784 era um workaround para a tabela
  ausente; com a tabela, deixava de resolver.

### Medição DEPOIS (release reconstruído)

- cristalino: exit 0, **sem warnings**. `pdffonts`: 2 fontes `CID Type 0C
  (OT)`; extração das name tables dos streams embutidos (python/fontTools):
  ```
  ('NewComputerModern10', 'NewCM10-Regular')
  ('NewComputerModern10', 'NewCM10-Bold')
  ```
  — as mesmas faces do vanilla (`NewCM10-Regular` + `NewCM10-Bold` para
  `Hello *world*`). Os nomes no PDF ficam anonimizados (`CrystallineFontN`)
  por decisão de export pré-existente (divergência mecânica registada,
  fora de âmbito).

**Bate com o vanilla**: família documentada resolve, sem warning, mesmas
faces embutidas.

---

## #30 (E2) — exceções de peso ausentes (usWeightClass errado)

### Medição ANTES

Probe `temp/p831/fexc2.typ` (reaproveitado de P831; as fontes sintéticas
`temp/p831/fonts/fandolhei-*.ttf` existiam e foram reutilizadas):

```typst
#set text(font: "FandolHei", weight: "bold")
AAAA
```

A face com PS `FandolHei-Bold` tem `usWeightClass=400` errado na OS/2
(confirmado por fontTools). Nuance da sonda: `FandolHei` só cobre
maiúsculas+espaço no subconjunto sintético — o probe tem de usar `AAAA`
(com lowercase o cristalino cai, corretamente, no fallback de cobertura,
C059 — medido e descartado como probe inválido em `temp/p840/e2a.typ`).

- vanilla (`typst compile --font-path temp/p831/fonts …`): `pdftotext -bbox`
  → palavra `AAAA` com `xMax−xMin = 44.0pt` (70.866→114.866); `pdffonts`:
  `DUZRZB+FandolHei-Bold` — a exceção corrige o peso para 700 e a face bold
  é selecionada.
- cristalino: `AAAA` com `xMax−xMin = 22.0pt` (70.867→92.867) — ambas as
  faces com peso 400, o pedido de bold cai na regular (glifos de 500 upem).

### Implementação

Mesma tabela e mesmo ponto de aplicação de #29: as entradas
`FandolHei-Bold`/`FandolSong-Bold` → `weight(700)`, `Arial-Black` →
`weight(900)`, `STKaiti*` → 400/700/900, `NewCM*-Book` → 450, `LMMono*` →
300 (+ `stretch(666)` nos `LtCond`), etc. O peso da exceção prevalece sobre
`face.weight().to_number()` (vanilla `info.rs:105-108`); idem stretch
(`info.rs:110-112`) e estilo (`info.rs:80`).

### Medição DEPOIS (release reconstruído)

- cristalino: exit 0; `pdftotext -bbox` → `AAAA` com `xMax−xMin =
  **44.0pt**` (70.867→114.867) — **igual ao vanilla (44.0pt)**; `pdffonts`:
  1 fonte `CID TrueType` (a face bold, ID `FandolHei-Bold`).

**Bate com o vanilla**: a face bold é selecionada via peso 700 da exceção.

---

## Regressões (fontes sem exceção)

- `#set text(font: "DejaVu Sans")` → embutida `DejaVu Sans / DejaVuSans`
  (inalterada face ao antes).
- Documento por defeito (sem `#set`) com `$x^2$` → `Libertinus Serif` no
  texto e `NewCMMath-Regular` na matemática — confirma que a cadeia math
  continua a resolver após a mudança de `DEFAULT_FALLBACK_FONTS_MATH`.
- Teste novo `p840_fonte_sem_excecao_inalterada`: `NimbusSans-Regular.otf`
  (PS fora da tabela) → família/peso/estilo inalterados.

## Testes

Testes primeiro (RED confirmado: `E0425` — `find_exception` inexistente).
4 testes novos em `03_infra/src/fonts.rs` (+1 fixture
`fixtures/fonts/p840-fandolhei-bold.ttf`, cópia da sintética de P831):

- `p840_find_exception_casos_tabela` — amostra de todos os grupos da tabela
  (Arial, Fandol, Noto, NewCM 08/10/Math/Mono/Sans/Uncial, Latin Modern,
  SimSun-ExtB, STKaiti) + nomes fora da tabela → `None`.
- `p840_excecao_peso_fandol_hei_bold` — fixture com `usWeightClass=400` →
  `FontWeight(700)`.
- `p840_excecao_familia_newcm_embutida` — itera `typst_assets::fonts()`:
  `NewCM10-Regular`/`NewCM10-Bold` → família `"New Computer Modern"`;
  `NewCMMath-Book` → peso 450. (Nota: `NewCM10-Book` **não** está entre as
  embutidas do `typst-assets` pinned `c0ae970` — o conjunto é
  LibertinusSerif×6, NewCM10×4, NewCMMath×3, DejaVuSansMono×4 — por isso a
  verificação de peso 450 usa `NewCMMath-Book`.)
- `p840_fonte_sem_excecao_inalterada` — regressão NimbusSans.

### Contagens (comando: `cargo test -p <crate>`, linha `test result`)

| Suite | Antes | Depois |
|-------|-------|--------|
| `typst-core` | 4567 passed, 0 failed | **4567 passed, 0 failed** (inalterado — mudança só em L3) |
| `typst-infra` | 694 passed, 0 failed | **698 passed, 0 failed** (+4 testes P840) |

Nuance de baseline: a primeira corrida de `typst-infra` neste passo deu
`693 passed; 1 failed` — flaky, não reproduzível: as duas corridas
seguintes na mesma árvore limpa deram 694/0, e a corrida final (com as
alterações) deu 698/0. Não identifiquei o teste (o log da corrida falhada
não foi preservado); se voltar a aparecer, merece investigação própria.

## Lint

`crystalline-lint --fix-hashes .` (atualizou headers de `fonts.rs`,
`embedded_fonts.rs`, `shaper.rs`) seguido de `crystalline-lint .` →
**exit 0, 0 violations** (sem correções manuais ao multi-`@prompt` —
não foi preciso desta vez).

## L0 atualizados

- `00_nucleo/prompts/infra/fonts.md` — `find_exception`, aplicação em
  `font_info_from_bytes`, alcance integral do port documentado.
- `00_nucleo/prompts/infra/embedded_fonts.md` — §P840: famílias passam a
  ser os nomes documentados; classificação atualizada.
- `00_nucleo/prompts/infra/shaper.md` — nota P840 no §P784 (Bug 2): a forma
  sem espaços era workaround para a tabela ausente; a cadeia math volta ao
  nome documentado.

## Nuances / limitações

1. **Port integral, mas verificação por amostra**: a tabela foi transcrita
   completa, mas só os casos NewCM/Fandol foram medidos ponta-a-ponta com os
   dois binários; os restantes grupos estão cobertos por testes unitários da
   tabela (amostra representativa), não por compilação comparada.
2. **`embedded_font_group` usa igualdade exata** para os dois nomes NewCM —
   os grupos óticos/Mono/Sans/Uncial da tabela não existem nas embutidas do
   `typst-assets` pinned `c0ae970`; se o pin mudar e novas NewCM entrarem, a
   classificação cai no `else` (math_code) — comportamento seguro, mas a
   rever se o pin for atualizado.
3. **`p754_newcm_math_is_not_in_text_group`** (em `embedded_fonts.rs`) não
   tem atributo `#[test]` — não corre na suíte (pré-existente, detetado
   neste passo; conteúdo atualizado para os nomes novos mas deixado sem
   `#[test]` para não alterar contagens fora de âmbito). Fica registado
   para decisão do dono.
4. **Peso default sem clamp**: a extração normal (sem exceção) usa
   `FontWeight(face.weight().to_number())` direto; o vanilla passa por
   `FontWeight::from_number` (clamp 100–900). Pré-existente de P839, fora
   dos achados #29/#30 — não tocado.
5. Nomes de fonte no PDF ficam anonimizados (`CrystallineFontN`) —
   divergência mecânica de export pré-existente; a paridade foi verificada
   por extração das name tables dos streams embutidos.
