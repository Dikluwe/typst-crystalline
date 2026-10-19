# P772m — Varredura da stdlib: `typst_library::text::font::*`

> **Passo:** 772m
> **Data:** 2026-07-16
> **Commit-base:** `61b7edee78fdae9b020e458f5989f638cbf04096` (HEAD). Working
> tree neste momento tem as alterações de código de P772l (`eval.md` +
> `01_core/src/engine/eval/*.rs`) já feitas; **nenhum código foi alterado por
> P772m** — só leitura, sondas e este relatório.
> **Medido em:** 2026-07-16T22:09–22:38Z.

---

## 1. Lista real de itens (confirmada contra o inventário — 20, não ~22)

```
book::distance, book::shared_prefix_words, book::similarity
exceptions::Exception, exceptions::find_exception
info::Coverage, info::decode_mac_roman, info::find_name,
  info::InternalBitFlags, info::typographic_family
metrics::FontMetrics, metrics::LineMetrics, metrics::ScriptMetrics,
  metrics::TextEdgeBounds, metrics::VerticalFontMetric
variations::AxisValue, variations::FontAxis, variations::FontVariations,
  variations::StandardAxes, variations::tag_hint_helper
```

A hipótese do prompt (`variations, metrics, info, book, exceptions, case`)
tinha um submódulo a mais (`case` — transformação maiúsculas/minúsculas —
**não existe** em `text::font::*`; é `text::case`, módulo distinto, fora
deste inventário). Confirmado por `awk` directo sobre
`lente-lista-B-2026-07-15.txt`: só 5 submódulos, 20 itens.

---

## 2. Classificação por submódulo

### 2.1 `book` — matching difuso de nome de família (fallback)

| Item | Classificação | Evidência |
|---|---|---|
| `distance` | **Lacuna total** | `01_core/src/entities/font_book.rs` (`FontBook::select`/`select_pattern`, ~183-233) só compara `(weight_dist, style_dist)` por campo; sem distância de `stretch`, sem clamping por eixo variável (não há campo `axes` em `FontInfo`). |
| `shared_prefix_words` | **Lacuna total** | Zero ocorrências de `shared_prefix`/`unicode_words` em `01_core/src`/`03_infra/src`. |
| `similarity` | **Lacuna total** | Sem função equivalente. `FontFlags` do cristalino (`font_book.rs:134-140`) só tem `monospace`/`serif` (faltam `math`/`variable` — ver `info`). |

**Achado mais largo**: `FontBook::select_fallback` do vanilla (coverage +
similaridade, sobre toda a colecção de fontes carregadas) **não existe** no
cristalino — zero ocorrências de `select_fallback`. O fallback real usa
`03_infra/src/fallback_fonts.rs`: uma lista estática (4-5 nomes por classe:
Liberation/DejaVu/Noto/FreeSerif/Arial) escolhida só por heurística
serif/sans sobre o *nome* da fonte primária (`fallback_font_list_for`,
34-41) — **cega a se a fonte de fallback realmente cobre o carácter**
necessário, porque `Coverage` (§2.3) não existe.

### 2.2 `exceptions` — tabela de correcções manuais por PostScript name

| Item | Classificação | Evidência |
|---|---|---|
| `Exception` | **Lacuna total** | Zero ocorrências de `Exception`/`EXCEPTION`/`postscript` em código de fontes do cristalino. |
| `find_exception` | **Lacuna total** | Idem — nenhuma tabela de lookup por PostScript name. |

Confirma a suspeita do prompt do passo: a tabela inteira de correcções
manuais do vanilla (131 entradas: peso errado do Arial-Black antigo, split
de família do Archivo Narrow, peso do FandolHei/Song-Bold, ~15 entradas de
família Noto Sans Display, etc.) está ausente.

**Medição de impacto real, não hipotético** (ADR-0108 — medir antes de
decidir): nenhuma das 33 fontes de teste do projecto
(`03_infra/fixtures/fonts/*.ttf|otf` — Amiri, Cantarell-VF, DejaVu, Liberti-
nus, MPLUS1p, NewCM, Nimbus, Noto\* (11 variantes), Sedgwick, SourceSansPro,
Ubuntu Sans, Yellowtail, etc.) aparece na `EXCEPTION_MAP` do vanilla
(grep cruzado, zero hits). **A lacuna é real e total, mas empiricamente
inerte para o corpus actual de fontes deste projecto** — nenhum teste
existente hoje pode estar a falhar por causa dela. Isto explica também que
o "resíduo mecânico de P772j" **não** vem daqui: verificado no relatório
`paridade-producao-p772j.md` §"Divergência residual" — a causa está
documentada e é outra (medição aproximada de largura de coluna em duas
passagens, `measure_content_constrained` vs `line_content_right`, aceite
desde P233), não a tabela de excepções.

### 2.3 `info` — metadados de fonte (coverage, nomes, flags)

| Item | Classificação | Evidência |
|---|---|---|
| `Coverage` | **Lacuna total** | `FontInfo` (`font_book.rs:143-151`) não tem campo `coverage`; sem bitset de intervalos de código em lado nenhum. |
| `find_name` | **Parcial, forma diferente** | Extracção de nome de família parece feita inline via `ttf_parser` no carregamento, não como helper genérico reutilizável por `name_id`. |
| `decode_mac_roman` | **Lacuna total** | Zero ocorrências de `mac_roman`/`MacRoman`/`Macintosh`. Fontes cuja única entrada de nome usável é Mac-Roman (algumas fontes CJK/antigas) perdem o nome de família no cristalino onde o vanilla o decodifica. |
| `InternalBitFlags` | **Não é item real** | Não existe nem no código-fonte vanilla (só em artefacto de build `.json` em cache) — ruído de macro `bitflags!` apanhado pela ferramenta que gerou o inventário. Recomenda-se excluir de rondas futuras. |
| `typographic_family` | **Lacuna total** | Sem lógica de "aparar sufixo de estilo do nome de família" (`"Noto Sans Light"` → `"Noto Sans"`); o cristalino usa o nome cru da tabela `FAMILY`. |

`FontFlags` do vanilla tem `MONOSPACE | SERIF | MATH | VARIABLE`; o
cristalino só tem `monospace`/`serif` — falta `MATH` (selecção de fonte
matemática por flag) e `VARIABLE` (moot: não há scoring que a use, §2.1).

### 2.4 `metrics` — divergência de mecanismo (ADR-0107), não de comportamento

| Item | Classificação | Evidência |
|---|---|---|
| `FontMetrics` | **Implementado, forma diferente** | `01_core/src/engine/layout/metrics.rs:22` — **trait**, não struct de dados: `advance`, `vertical_metrics`, `cap_height`, `text_edges`, `math_constants()`. Valores reais lidos via `ttf_parser` em `03_infra/src/font_metrics.rs`. |
| `LineMetrics` | **Parcial / não confirmado** | Sem tipo `LineMetrics` nomeado; emissão de strikethrough/underline existe no exportador PDF — não verificado neste passo se lê `strikeout_metrics()`/`underline_metrics()` reais por fonte ou usa constantes fixas (follow-up). |
| `ScriptMetrics` | **Lacuna total como dado derivado da fonte** | Zero ocorrências de `subscript_metrics`/`superscript_metrics`/`ScriptMetrics`; posicionamento de sub/sobrescrito, se existir, não vem das tabelas OpenType `sub`/`sup` reais da fonte. |
| `TextEdgeBounds` | **Implementado, forma diferente** | Coberto por `text_edges(size, style) -> (Pt, Pt)` — API de valor directo em vez de enum de 3 modos (Zero/Glyph/Frame); comportamento observável presente. |
| `VerticalFontMetric` | **Implementado, forma diferente** | Chaveado por string (`"cap-height"`, `"x-height"`, etc.) em vez de enum Rust; os 5 casos nomeados no vanilla parecem cobertos (não verificados numericamente um a um neste passo). |

Este submódulo é o único dos 5 onde a maior parte do comportamento parece
**presente**, só sob mecanismo diferente — paridade de língua (ADR-0107),
não lacuna. Não medido numericamente linha a linha contra o vanilla neste
passo (ficaria fora do orçamento de tempo) — marcado como inferência a
confirmar por sonda dedicada se algum bug de métrica for suspeitado no
futuro.

### 2.5 `variations` — eixos de fonte variável

| Item | Classificação | Evidência |
|---|---|---|
| `FontAxis` | **Lacuna total** | `FontInfo` não tem `axes: Vec<FontAxis>` — metadado de eixo nem chega a ser capturado ao nível do FontBook/L1. |
| `AxisValue` | **Lacuna total** | Sem tipo nem clamping ao intervalo declarado do eixo. |
| `StandardAxes` | **Lacuna total** | Zero ocorrências de `StandardAxes`/`slnt`/`opsz`. |
| `FontVariations` (resolve) | **Parcial — MVP** | `03_infra/src/font_variant.rs` (`axis_variations_for_font_variant`) só mapeia `wght` (sem clamping ao min/max real da fonte) e `ital` (flag 0/1). Comentário do próprio ficheiro admite o escopo: "`wdth` só será mapeado quando `TextStyle` expuser stretch; `slnt` requer `FontStyle::Oblique(angle)`, que o modelo actual não tem." Sem `opsz`. |
| `tag_hint_helper` | **Lacuna total** | Sem API de eixos custom exposta ao utilizador; item moot sem essa superfície. |

Apesar do MVP ser parcial, a instanciação de `wght`/`ital` **funciona** em
princípio — confirmado (§3) com fonte variável real via pipeline P530
(`fontTools` instancer). O achado novo e mais importante deste passo não é
um item do inventário: é um **bug real na instanciação em pesos altos**,
descrito a seguir.

---

## 3. Achado novo, fora do inventário nomeado: espaço entre palavras colapsa em fontes variáveis com peso alto

### 3.1 Reprodução

Ambiente: `TYPST_CRYSTALLINE_PYTHON` apontado para venv local com
`fonttools` 4.63.0 instalado (`python3 -m venv` + `pip install fonttools`
— o ambiente de sondagem não tinha `fontTools` por defeito; sem isto, a
compilação com fonte variável falha directamente com "Python/fontTools não
está disponível", impedindo qualquer teste de VF). Fonte:
`03_infra/fixtures/fonts/UbuntuSans-Variable.ttf` (eixo `wght`
100–400(default)–**800**, `wdth` 75–100).

```
#set text(font: "Ubuntu Sans", size: 24pt, weight: N)
Weight test here
```

Bissecção do valor de `N` (5 repetições em cada ponto crítico para excluir
flakiness):

| `weight:` | Saída (`pdftotext`) | Nota |
|---|---|---|
| 300, 500, 700 | `Weight test here` | correcto |
| 760 | `Weight test here` | correcto, limite |
| 770 | `Weighttest here` | 1ª palavra já colada |
| 780 | `Weighttest here` | |
| 790 | `Weighttesthere` | todas coladas |
| 799, 800 (= máximo do eixo), 801, 900 | `Weighttesthere` | consistente em 5/5 execuções |

Confirmado que **não é ruído de anti-aliasing**: comparação visual
(`mutool draw -r 300` + crop) a peso 800 mostra os glifos de "Weight" e
"test" **tocando-se directamente**, sem largura de espaço nenhuma — não é
só um espaço visualmente pequeno.

**Vanilla, mesma fonte, mesmos pesos (300–900 incl. 800): sempre
`Weight test here`, sem excepção.** Confirma que é 100% específico do
cristalino, não uma característica peculiar da fonte em si nem do
`fontTools`/`rustybuzz` upstream.

**Segundo ponto de dados — não generaliza a todas as fontes variáveis**:
`Noto Sans` variável (eixo `wght` 100–400–**900**, ficheiro
`NotoSans_variable.ttf`) testado nos mesmos moldes (400/700/850/900 = seu
próprio máximo) — **sempre correcto**, nenhum colapso, em nenhum peso
incluindo o máximo do eixo. Ou seja: o bug depende de dados específicos da
fonte (Ubuntu Sans), não é uma falha universal do pipeline de VF.

### 3.2 Causa provável (marcada como hipótese — não confirmada por debug interno)

`03_infra/src/shaper.rs` linha ~261 (comentário `P568`): espaços entre
palavras são **deliberadamente não shapeados** — `shaped_width_of_frame`
retorna `None` para texto só-espaço, para que o exportador escreva o
carácter de espaço literal na stream do PDF (acessibilidade/copy-paste), em
vez do glifo. Consequência: a largura do espaço não passa pelo caminho de
shaping com variações aplicadas (`rb_face.set_variations(&axis_vars)` em
`shaper.rs:196/332`); em vez disso, `space_width()`
(`01_core/src/engine/layout/cursor.rs:44-46`) chama
`FontMetrics::advance(" ", ...)`, cuja implementação para o caminho de
fallback (`FallbackFontMetrics::advance`,
`03_infra/src/font_metrics.rs:704-746`) lê `face.glyph_hor_advance(g)` de
um `ttf_parser::Face` **cacheado** (`cached_face`) — não confirmado neste
passo se esse cache reflecte a instância correcta do eixo `wght` pedido ou
a instância por omissão (400) da fonte variável.

Isto identifica **dois caminhos de largura de glifo desacoplados** para
fontes variáveis — o caminho shapeado (glifos de texto normal, usa
`rustybuzz` com variações ao vivo) e o caminho não-shapeado (só espaço,
usa `ttf_parser` sobre uma face cacheada) — como candidato estrutural ao
bug. **Não foi confirmado** qual exactamente dos dois diverge nem por que
só acontece com esta fonte e não com Noto Sans — precisaria de
instrumentação (print/trace do valor de `space_width()` e da face
resolvida) para fechar a causa exacta. Marcado como inferência, não facto.

### 3.3 Por que não foi corrigido directamente

A causa raiz não está isolada com confiança suficiente para uma correcção
pontual e seria irresponsável "tentar às cegas" um patch num caminho de
shaping que já tem disciplina de coordenadas estabelecida (P568, P525,
P530) sem entender exactamente onde os dois caminhos divergem — isto viola
directamente a disciplina anti-deriva (ADR-0108: medir antes de decidir).
Requer sonda dedicada com instrumentação (comparar `space_width()` chamado
via o caminho de fallback vs a largura que `rustybuzz` calcularia para o
mesmo glifo com as mesmas variações, para a mesma fonte, no mesmo peso) —
fora do orçamento deste passo de varredura.

**Severidade**: alta quando ocorre (palavras coladas = ilegível), mas
**âmbito ainda não determinado** — pode ser específico de `UbuntuSans-
Variable.ttf` (dados de `gvar` incomuns) ou pode afectar outras fontes
variáveis não testadas aqui. Recomenda-se passo dedicado, com mais de duas
fontes variáveis testadas, antes de classificar como "geral" ou "isolado".

---

## 4. Decisão

1. Nenhum código foi alterado neste passo. Os itens de `book`/`exceptions`/
   `info`/partes de `variations` são lacunas totais reais mas de grande
   porte (novo subsistema de fallback/coverage/correcções manuais) — exigem
   L0 novo (Regra de Ouro), não implementação directa num passo de
   varredura.
2. `metrics` está, na melhor avaliação possível sem medição numérica
   exaustiva, funcionalmente coberto sob mecanismo diferente (ADR-0107) —
   não classificado como lacuna accionável.
3. **Prioridade recomendada para um próximo passo dedicado**: o colapso de
   espaço em fontes variáveis de peso alto (§3) — é um bug de
   comportamento (não lacuna de feature), com impacto visual directo, em
   pelo menos uma fonte já presente no corpus de testes do projecto.
4. `exceptions` (§2.2), apesar de lacuna total confirmada, está
   empiricamente inerte para o corpus actual — prioridade baixa a menos que
   se pretenda alargar o corpus de fontes de teste no futuro.

---

## 5. Validação

Nenhuma alteração de código neste passo — `cargo test --workspace` e
`crystalline-lint .` permanecem no estado validado ao fim de P772l (4172 +
644 + 33 + 2 + 29 + 2 testes, 0 falhas; 0 violações de lint).

Ambiente de sondagem: venv temporário
(`/tmp/p772m/venv`, `pip install fonttools`) usado só para permitir os
testes de fonte variável — não afecta o repositório nem é dependência do
projecto.

## Critério de fecho do passo (`typst-passo-772m.md`)

- [x] Lista real de submódulos/itens de `text::font::*` confirmada (20, não
      a hipótese de ~22 do prompt — `case` não pertence a este módulo).
- [x] Itens classificados; atenção especial a `exceptions` — medido e
      confirmado como lacuna total mas **empiricamente inerte** para o
      corpus de fontes actual (não é a causa do residual de P772j).
- [ ] Bugs reais corrigidos com métricas/coordenadas comparadas
      directamente — **nenhum corrigido neste passo**: os achados de
      lacuna total exigem L0 novo (subsistema grande); o bug de
      espaço-colapsado (§3) tem causa não isolada com confiança
      suficiente para correcção segura — registado como prioridade para
      passo dedicado.
- [x] Sem regressão nos testes de fonte já existentes (P666-669, P753,
      P761-762) — nenhum código alterado.
- [x] Volume grande — tratado por classificação em lote por submódulo,
      não forçado a implementação num passo só.
- [x] `cargo test --workspace` verde (estado herdado de P772l, sem
      alterações neste passo).
- [x] `crystalline-lint .` zero violações (idem).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772m.md`.

---

## Próximo passo

Com `image::svg` (P772k), `foundations::scope` (P772l) e `text::font::*`
(P772m) cobertos, reconfirmar a lista `lacuna-inventario` restante (segunda
rodada, mesmo padrão de P772e) e decidir se vale continuar a varredura
sistemática ou mudar para os dois follow-ups de maior severidade
identificados nestes três passos:

1. Mutação silenciosa de bindings da stdlib (`cannot_mutate_constant`,
   P772l §2.3) — corrupção silenciosa, sem diagnóstico.
2. Colapso de espaço em fontes variáveis de peso alto (P772m §3) — bug de
   comportamento com impacto visual directo.
