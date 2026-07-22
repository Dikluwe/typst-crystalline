# Relatório — typst-passo-839: `text::font::info` — achados #25–#28 de P831 (resolução de nome/estilo de fonte)

**Data**: 2026-07-22, medições entre 17:10 e 17:34 (-03:00).
**Proveniência**: commit HEAD `7a0007f6be3b6e92c35532b3bf0c7f00ab660a5f` (P838). Medições "antes" com working tree limpo (`git status --porcelain --untracked-files=no` vazio). Medições "depois" com as alterações deste passo na working tree (não commitadas — ver `git diff --stat` ao fim).
**Binários**: `./target/release/typst` (cristalino, rebuildado antes de cada ronda de medição) vs `lab/typst-original/target/release/typst` (vanilla 0.15.0).
**Fixtures**: fontes sintéticas de P831 reaproveitadas de `temp/p831/fonts{,2,4}/` (fontTools 4.63.0, venv `lab/.venv`); docs `finfo*.typ` copiados para `temp/p839/`. Cinco fontes copiadas para fixtures tracked (`03_infra/fixtures/fonts/p839-*.ttf`) para os testes; `p839-noname.ttf` derivada de `p839-triagx-bold.ttf` com a tabela name esvaziada (fontTools: `font["name"].names = []`). O script auxiliar foi apagado após uso — o linter V1/V8 acusa scripts em `temp/` fora do lineage.

Comandos padrão (da raiz do repo):
```
./target/release/typst temp/p839/<f>.typ -o temp/p839/out/c-<f>.pdf --font-path temp/p831/fonts 2> …
lab/typst-original/target/release/typst compile --font-path temp/p831/fonts temp/p839/<f>.typ temp/p839/out/v-<f>.pdf 2> …
```
Nota de formato (herdada de P831): o CLI cristalino emite diagnósticos em linha única e o vanilla em codespan multi-linha; a comparação é sobre **mensagem verbatim + posição**, não sobre o chrome. O cristalino nomeia as fontes embutidas no PDF genericamente (`CrystallineFont*`) — convenção pré-existente, fora do âmbito.

---

## #25 (I1) — aparo de sufixos do name ID1 (`typographic_family`)

**Antes** (cristalino usava o primeiro registo ID16/ID1 cru, sem aparo):

| Caso | cristalino | vanilla |
|---|---|---|
| finfo1a `#set text(font: "TriagX")` (ID1 = `TriagX Bold`) | `:3:16: warning: unknown font family: triagx` | (vazio — compila; pdffonts: `TriagX-Bold`) |
| finfo1b `#set text(font: "TriagX Bold")` (recíproco) | (vazio) | `warning: unknown font family: triagx bold` (3:16) |
| finfo2a `#set text(font: "TriagDsp Display")` (ID16 `TriagDsp`, ID1 `TriagDsp Display Bold`) | `:3:16: warning: unknown font family: triagdsp display` | (vazio; pdffonts: `TriagDsp-DisplayBold`) |
| finfo2b `#set text(font: "TriagDsp")` | `:3:16: warning: unknown font family: triagdsp` | `warning: unknown font family: triagdsp` (3:16) |

**Depois** (cristalino usa só o ID1 + `typographic_family`, ID16 ignorado — port verbatim do vanilla `info.rs:206-267`):

| Caso | cristalino | vanilla | verdict |
|---|---|---|---|
| finfo1a | (vazio — sem warning; fonte embutida) | (vazio) | ✅ paridade |
| finfo1b | `:3:16: warning: unknown font family: triagx bold` | `warning: unknown font family: triagx bold` (3:16) | ✅ mensagem + posição verbatim |
| finfo2a | (vazio — sem warning) | (vazio) | ✅ paridade |
| finfo2b | `:3:16: warning: unknown font family: triagdsp` | `warning: unknown font family: triagdsp` (3:16) | ✅ paridade (já existia; mantida) |

**Implementação**: `03_infra/src/fonts.rs` — `font_info_from_bytes` passa a chamar `find_name(face, FAMILY)` e aplica `typographic_family` (listas SUFFIXES/MODIFIERS/SEPARATORS idênticas ao vanilla, aparo iterativo até fixpoint, case-insensitivo via lowercase ASCII). O filtro `TYPOGRAPHIC_FAMILY || FAMILY` foi removido: o vanilla não usa o ID16 de propósito (`info.rs:62-72` — agrupa variantes para lá de estilo/peso/largura, ex.: Display dos Noto). Let-chain do vanilla reescrita aninhada (crate é edition 2021).
**Testes**: `p839a_aparo_sufixos_estilo_id1` (fixtures triagx-bold → `"TriagX"`, triagdsp → `"TriagDsp Display"`), `p839a_typographic_family_casos_vanilla` (12 casos replicados do `test_trim_styles` do vanilla).

## #26 (I2) — `decode_mac_roman`

**Antes**: finfo3 `#set text(font: "TriagRésumé")` (fonte só com registos Macintosh 1,0,0; é = byte 0x8E):
- cristalino stderr: `:3:16: warning: unknown font family: triagrésumé`; pdffonts: só `CrystallineFont` (fallback).
- vanilla stderr: (vazio); pdffonts: `OMMPIC+TriagResume-Regular` embutido.

**Depois**:
- cristalino stderr: (vazio); pdffonts: `CrystallineFont` embutido (a fonte resolveu — sem warning de família desconhecida). ✅ paridade de resolução.
- Nuance medida (fora de âmbito, ver "Notas laterais"): a fonte sintética só tem glifos para espaço/A/B; as larguras dos glifos em falta divergem do vanilla (que faz fallback por glifo para Libertinus — pdffonts vanilla mostra as duas fontes).

**Implementação**: `find_name` (port do vanilla `info.rs:168-182`) — `entry.to_string()` primeiro; se `None` e registo Macintosh (plataforma 1, encoding 0), `decode_mac_roman(entry.name)`. Tabela de 128 chars portada verbatim (`info.rs:185-203`). Usada para FAMILY e FULL_NAME.
**Teste**: `p839b_decode_mac_roman` (unitário `0x8E → "é"`, ASCII inalterado; fixture triagmac → `Some(info)` com `family == "TriagRésumé"` — antes retornava `None`).

## #27 (I3) — inferência de estilo pelo full name

**Antes** (finfo4 `#set text(font: "TriagSlant", style: "oblique")` + `AAAA`; a face Oblique não tem bits fsSelection nem ângulo, só o full name `TriagSlant Oblique`; largura da palavra via `pdftotext -bbox`):

| font-dir | cristalino | vanilla |
|---|---|---|
| `fonts/` (obl antes de reg) | 44.0pt | 44.0pt (coincidência de ordem) |
| `fonts2/` (reg primeiro) | **22.0pt** (escolheu a regular) | 44.0pt |

**Depois**:

| font-dir | cristalino | vanilla | verdict |
|---|---|---|---|
| `fonts/` | 44.0pt (xMax 114.867 − xMin 70.867) | 44.0pt | ✅ |
| `fonts2/` | **44.0pt** (114.867 − 70.867) | 44.0pt | ✅ face obliqua seleccionada independentemente da ordem |

**Implementação**: `infer_style(ttf_italic, ttf_oblique, full_lower)` — port do vanilla `info.rs:80-103`: `italic = (style() == Style::Italic) || full.contains("italic")`; `oblique = is_oblique() || full.contains("oblique") || full.contains("slanted")`; italic tem precedência. Substituído `face.is_italic()` por `face.style() == Style::Italic` — o vanilla evita `is_italic()` porque também consulta o ângulo itálico (falsos positivos em oblique, typst/typst#7479).
**Teste**: `p839c_estilo_inferido_do_full_name` (7 combinações da heurística pura + fixture triagslant-obl → `FontStyle::Oblique`).

## #28 (I4) — FontBook ↔ font_slots emparelhados

**Antes** (finfo5 `#set text(font: ("TriagCovA", "TriagCovB"))` + `AB`; largura esperada 5.5+11 = 16.5pt):

| font-dir | cristalino | vanilla |
|---|---|---|
| `fonts/` (triagmac antes das cov) | **22.0pt** (face errada) | 16.5pt |
| `fonts4/` (prova causal: só mac+cov, mac primeiro) | **11.0pt** (slot da triagmac usado para "TriagCovA") | 16.5pt |

Mecanismo: `discover_fonts` criava slot incondicional; `build_font_book` só fazia push com info — a triagmac (sem nome decodificável pré-I2) entrava nos slots mas não no book e todos os índices seguintes apontavam para o slot errado. O mesmo padrão existia em `fontdb.rs` (latente registado por P838).

**Vanilla** (confirmado antes de escolher a abordagem): `typst-kit/src/fonts.rs:172-189` — `with_db` faz `filter_map` sobre as faces e só produz o par `(FontPath, FontInfo)` quando `FontInfo::new` retorna `Some`; `FontStore::push` (`:39-43`) insere slot e book juntos. **Fonte sem info não ganha slot.** Foi esta a abordagem replicada (não a alternativa "inserir info parcial no book").

**Depois**:

| font-dir | cristalino | vanilla | verdict |
|---|---|---|---|
| `fonts/` | **16.5pt** (87.367 − 70.867); 2 fontes embutidas (CovA+CovB) | 16.5pt | ✅ |
| `fonts4/` | **16.5pt** (87.367 − 70.867); 2 fontes embutidas | 16.5pt | ✅ |

**Implementação**:
- `fonts.rs`: `build_font_book(&[FontSlot]) -> FontBook` substituído por `pair_slots_with_book(Vec<FontSlot>) -> (Vec<FontSlot>, FontBook)` — slot só é mantido quando a info é extraída. `discover_fonts` continua lazy (cria slots para tudo; validação em `get()`), o emparelhamento acontece na composição (`SystemWorld::with_fonts`, `with_fonts_and_system`).
- `fontdb.rs`: `load_system_fonts` e `load_fonts_from_dir` — `slots.push` movido para dentro do `if let Some(info)` (push emparelhado).
- Callers actualizados: `world.rs` (2 builders + 2 testes que codificavam o comportamento antigo: `..._preserva_ordem` passa a usar fonte válida; `..._invalid_slot_returns_none` comentário corrigido), `integration_tests.rs` (5 call sites + import), teste antigo `build_font_book_com_slots_invalidos` renomeado/reescrito.

**Testes**: `p839d_slots_e_book_emparelhados` (fonts.rs: dir com NimbusSans + fake.ttf → `slots.len() == book.len() == 1`; antes: 2 vs 1), `p839d_fontdb_slots_book_emparelhados` (fontdb.rs: NimbusSans + `p839-noname.ttf` → invariante `slots.len() == book.len()`).
**Nuance medida**: o fontdb 0.21 **já exclui ele próprio** a `p839-noname.ttf` (`Database::faces()` reporta 1 face para um dir com as 2 fontes) — o teste do fontdb é guarda do invariante estrutural, não reprodução RED. O vector medido do achado é o caminho `--font-path`/`discover_fonts`, coberto pelo teste de fonts.rs e pelas medições de binário acima. Com o decode mac roman implementado, não se conhece fonte que o fontdb aceite e `font_info_from_bytes` rejeite; o fix torna o emparelhamento estruturalmente garantido independentemente disso (como o `filter_map` do vanilla).

---

## Contagens

| Suite | Antes | Depois |
|---|---|---|
| `cargo test -p typst-core` | 4567 passed, 0 failed | 4567 passed, 0 failed |
| `cargo test -p typst-infra` | 688 passed, 0 failed | **694** passed, 0 failed (+6: p839a×2, p839b, p839c, p839d×2) |

(Contagens agregadas de todos os targets de teste via `grep "^test result"`. As suites "depois" correram antes dos bumps cosméticos de `@updated` nos headers — comentários doc apenas.)

## Regressão com fontes normais

- Doc com `#set text(font: "Libertinus Serif")` (embutido, com bold e itálico): sem warnings; texto correcto no pdftotext. Testes `p753/p784/p797` de embedded fonts verdes na suite.
- Doc com `#set text(font: "DejaVu Sans")` (sistema, sem `--font-path`): sem warnings; renderiza.
- Suite completa verde (inclui `p838_*` de panose/ttc e os font_wiring de integration_tests).

## Lint / linhagem

- `crystalline-lint --fix-hashes .` → `03_infra/src/fonts.rs` → hash `80651f49` (fontdb.rs não tem linha `@prompt-hash` — nada a fixar; sem bug multi-`@prompt` observado desta vez).
- `crystalline-lint .` → **exit 0** (só warnings V7 de prompts órfãos pré-existentes, sem relação com este passo).
- L0 actualizados: `00_nucleo/prompts/infra/fonts.md` (nova API `pair_slots_with_book`, campos de extracção P839, funções internas, critérios, histórico) e `00_nucleo/prompts/infra/fontdb.md` (revogada a instrução "faces sem info mantidas como slots" — codificava o achado #28).

## Ficheiros tocados

- `03_infra/src/fonts.rs` — I1+I2+I3 (`find_name`, `decode_mac_roman`, `typographic_family`, `infer_style`, rewrite de `font_info_from_bytes`) + I4 (`pair_slots_with_book`) + 5 testes p839.
- `03_infra/src/fontdb.rs` — I4 (push emparelhado nos 2 loops) + 1 teste p839d.
- `03_infra/src/world.rs` — `with_fonts`/`with_fonts_and_system` usam `pair_slots_with_book`; 2 testes actualizados.
- `03_infra/src/integration_tests.rs` — 5 call sites + import.
- `03_infra/fixtures/fonts/p839-{triagx-bold,triagdsp,triagmac,triagslant-obl,noname}.ttf` — novas fixtures (4 copiadas de `temp/p831/fonts/`, 1 derivada).
- `00_nucleo/prompts/infra/{fonts,fontdb}.md` — L0 actualizados.
- `temp/p839/` — probes e saídas (não tracked; o gerador da fixture noname foi apagado — ver nota de fixtures acima).

## Divergências residuais / notas para passos futuros (decisões minhas, registadas — não são scope do passo)

1. **Fallback "qualquer nome" sem equivalente vanilla**: `font_info_from_bytes` mantém o fallback pré-existente "primeiro registo name decodificável" quando não há ID1; o vanilla **descarta** a fonte nesse caso (`find_name(FAMILY)?`). Mantido por minimalidade — fontes reais têm ID1; remover o fallback é decisão de paridade a medir num passo próprio.
2. **Fallback por glifo para as fontes default**: medido em finfo1a/finfo3 — o vanilla embute Libertinus Serif **além** da fonte pedida para cobrir glifos em falta (pdffonts mostra as duas); o cristalino embute só a fonte pedida (glifos em falta ficam .notdef, larguras divergentes: "Texto" 28.127pt cris vs 25.124pt van). Pré-existente (P831 já notara que o fallback por cobertura funciona *dentro* da lista do utilizador); a extensão às fontes default fora da lista é candidata a achado novo.
3. **`with_fonts` re-emparelha**: callers que já emparelharam (integration_tests) passam por uma segunda extracção de info (idempotente; mesmo custo de I/O que o código anterior, que também relia os bytes duas vezes).
4. **Excepções vanilla (E1/E2 de P831)** — `find_exception`/EXCEPTION_MAP continua inexistente no cristalino (ex.: "New Computer Modern" embutido). Fora do âmbito de P839 (#25–#28); fica para passo próprio.
5. **fontdb.rs sem linha `@prompt-hash`** no header (estado pré-existente) — o linter não verifica drift desse prompt. Registado; não alterado neste passo.

## Proveniência final (estado da árvore no fim do passo)

```
HEAD = 7a0007f6be3b6e92c35532b3bf0c7f00ab660a5f (P838) — SEM commit deste passo
M  00_nucleo/prompts/infra/fontdb.md
M  00_nucleo/prompts/infra/fonts.md
M  03_infra/src/fontdb.rs
M  03_infra/src/fonts.rs
M  03_infra/src/integration_tests.rs
M  03_infra/src/world.rs
?? 03_infra/fixtures/fonts/p839-*.ttf (5 ficheiros)
```
