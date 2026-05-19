# Passo 287 — `Content::SmartQuote` + função stdlib

**Frente**: `P-smartquote` (rank 1 do relatório P286 §9).
**Origem**: Tabela C linha 380 — `smartquote` registado como
ausente, bloqueador único `Content::SmartQuote`. Escopo S.
**Pré-requisitos**: nenhum. Infraestrutura lang-aware
(`rules/lang/quotes.rs`) já existe desde P155.

---

## §1 — Objectivo

Materializar a função vanilla `smartquote(double: bool, enabled:
bool, alternative: bool)` como entrada de primeira classe no
sistema. Estado actual divide-se em dois sítios:

- **Implementado**: markup `"foo"` → "foo" via `eval_markup`
  (P155); alternância open/close lang-aware com 6 idiomas + ASCII
  default em `rules/lang/quotes.rs`.
- **Ausente**: função stdlib `#smartquote(...)` — não existe;
  `Content::SmartQuote` variant não existe.

Razão de ser:

1. **Completar a feature**: a função permite invocação programática
   (templates, show rules, callbacks) sem depender do parser de
   markup. Em vanilla typst, `smartquote(double: false)` produz `‘`
   ou `’` conforme o estado open/close — esta capacidade não está
   acessível em cristalino sem materializar a função.
2. **Reaplicação directa do padrão "variant rico com cosméticos
   opcionais"** (N=4 cumulativo desde P156G/H/I + P284). Este passo
   leva a N=5 — **limiar histórico para promoção ADR meta**
   conforme relatório P284 §8.1 e P286 §9.

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória)

Quatro ambiguidades factuais antes de materializar:

### A.1 — Inventário da infraestrutura P155 existente

Listar literalmente em
`00_nucleo/diagnosticos/diagnostico-smartquote-passo-287.md`:

1. **API actual em `rules/lang/quotes.rs`** — assinaturas das
   funções `open_double`/`close_double`/`open_single`/`close_single`
   (ou nomes equivalentes); como recebem o lang corrente; estado
   open/close persistido como?
2. **Integração actual em `eval_markup`** — onde está o
   alternador open/close; que estado mantém entre invocações; é
   per-paragraph, per-document, per-runtime?
3. **Como `text.smartquotes` (atributo `set text`) é consultado** —
   se é. Vanilla typst tem `text.smartquotes: bool` para
   desligar a feature. Se cristalino o tem (parcial ou total),
   identificar; se não tem, registar como divergência.

Output: tabela de 3-4 linhas mapeando símbolos → funções →
ficheiros + linhas exactas.

### A.2 — Estrutura do variant `Content::SmartQuote`

Decisão arquitectural (per ADR-0065 critério #1 — naming e estrutura
do variant):

| Opção | Estrutura | Prós | Contras |
|---|---|---|---|
| **(a)** `SmartQuote { double: bool }` | mínimo vanilla | Simétrico vanilla typst (signature da função) | Não captura o estado open/close — quem decide o glyph emitido? |
| **(b)** `SmartQuote { double: bool, opening: bool }` | estado explícito | Captura semântica completa; permite chamada directa fora de markup-alternation | Precisa de mecanismo para `opening` ser preenchido — pelo parser? pelo Layouter? |
| **(c)** `SmartQuote { double: bool, alternative: bool }` | paridade vanilla completa | Cobre todos os atributos vanilla | `alternative` tem semântica obscura (alterna `« »` vs `‘ ’` em fr; ADR-0054 graded justifica scope-out) |
| **(d)** `SmartQuote { double: bool, enabled: bool, alternative: bool }` | vanilla 1:1 | Paridade total | 3 atributos cosméticos; reaplica padrão "variant rico" (relevante para gatilho N=5) |

Default sugerido: **(d)** se Fase A.1 confirmar que infra existente
permite consultar lang correctamente; **(a)** se A.1 mostrar que
estado open/close é resolvido externamente (e.g. no consumer
Layouter) e o variant apenas carrega o "quero aspas duplas/simples".

**Implicação para padrão N=5**: opções (b)/(c)/(d) qualificam-se
como "variant rico com cosméticos opcionais" (atributos opcionais
para além de body); (a) não qualifica (1 campo `bool` required).
Esta decisão **influencia a promoção do ADR meta** — relevante para
o relatório.

### A.3 — Estado open/close: onde reside?

| Opção | Localização | Prós | Contras |
|---|---|---|---|
| **(α)** Markup parser preenche — `eval_markup` cria `SmartQuote { ..., opening: true/false }` consoante alternância actual | Estado fica na fase parser; consumer Layouter recebe info completa | Função stdlib `#smartquote(...)` não tem acesso a esse estado (não passa pelo parser de markup) — produz sempre o mesmo glyph |
| **(β)** Layouter resolve — consumer rastreia open/close em `Layouter.smartquote_state` (analógo a `decoration_lines_collector` de P286 §8.1) | Função stdlib **e** markup partilham mesma fonte de verdade | Refactor de P155 — `eval_markup` deixa de pré-resolver |
| **(γ)** Híbrido — markup pré-resolve via flag interna ao parser; função stdlib defaulta para `opening: false` (close-quote como vanilla typst quando não há contexto) | Backward-compat literal de P155; função apenas adiciona caso novo | Divergência subtil entre markup e função em casos edge |

Default sugerido: **(γ)** — minimiza touch points em P155, preserva
bit-exact do markup actual, e a função stdlib serve casos novos
sem perturbar os antigos. **(β)** se A.1 revelar que o estado
P155 já é consultável de fora (consolidação natural).
**(α)** rejeitada salvo se A.1 mostrar acoplamento estrutural
necessário com o parser.

### A.4 — Política do glyph emitido

Independentemente de A.3, o consumer Layouter precisa produzir o
glyph correcto. Decisão:

| Opção | Comportamento |
|---|---|
| **(i)** Reusar `rules/lang/quotes.rs` directamente — emitir `Content::Text(quote_glyph, style)` via lookup `(lang, double, opening)` | Single source of truth (padrão N=3 §8.2 P286) — markup e função partilham resolução |
| **(ii)** Variant fica preservado até ao emit; PDF emite glyph directamente via tabela interna | Distingue arquitecturalmente; permite show rules sobre `SmartQuote` |

Default sugerido: **(i)** — reusa infraestrutura P155, aplica
padrão "single source of truth como invariante anti-bug" (§8.2
P286 N=3). **(ii)** se A.1 revelar que show rules sobre smartquote
são funcionalidade vanilla concreta (provável vanilla expõe via
`#show smartquote: it => ...`).

**Interacção com ADR meta candidata**: se A.4 → (i), este passo
**cita o padrão "single source of truth"** (P282 + P285 + P286 =
N=3 → P287 = **N=4**) sem disparar nada — ainda não é a quinta
aplicação. Se a Fase A revelar que outra parte do passo cita o
padrão de outra forma, registar. **Promoção do ADR meta meta
continua a ser não-objectivo** deste passo.

---

## §3 — Materialização

Após Fase A produzir inventário + estrutura variant + política
estado + política glyph:

1. Adicionar `Content::SmartQuote { ... }` em
   `01_core/src/entities/content.rs` conforme decisão A.2.
2. Adicionar `native_smartquote` em
   `01_core/src/rules/stdlib/text.rs` (ou módulo equivalente,
   onde estão `native_underline` etc. pós-P284).
3. Registar em `make_stdlib`.
4. Implementar consumer Layouter:
   - Se A.3 → (γ): `Content::SmartQuote { double, ... }` sem
     `opening` field; consumer consulta
     `Layouter.smartquote_state` (ou equivalente Region/Regions
     pós-P216) e emite glyph; atualiza estado.
   - Se A.3 → (β): refactor P155 para usar mesma resolução.
   - Se A.3 → (α): consumer apenas emite — `opening` veio do
     variant.
5. Aplicar política A.4:
   - Se (i): consumer chama helper de `rules/lang/quotes.rs`
     (paridade ao caminho P155).
   - Se (ii): consumer mantém variant até emit; tabela interna
     no L3.
6. Visitors do `Content` enum estendidos (precedente P284):
   `plain_text`, `is_empty`, `PartialEq`, `map_content`,
   `map_text`, `is_locatable`, `materialize_time`, `walk`.
7. Testes:
   - L1 unitário variant: construção; PartialEq; `plain_text`
     retorna glyph correcto (e.g. `"\u{201C}"` para `double:
     true, opening: true`).
   - L1 unitário stdlib: `#smartquote()` produz variant default;
     `#smartquote(double: false)` flag muda; `enabled: false`
     produz `Text("\"")` ASCII literal.
   - L1 unitário consumer: alternância open/close em sequência
     repetida.
   - L3 integração PDF: documento com `#smartquote()
     ...#smartquote()` produz dois glyphs distintos no output;
     primeiro open, segundo close.
   - **Regressão bit-exact**: markup `"foo"` pré-P287 produz PDF
     idêntico pós-P287 (caminho P155 inalterado ou redirigido
     transparente).
8. Actualizar L0:
   - `00_nucleo/prompts/rules/stdlib.md` — tabela funções (+1).
   - `00_nucleo/prompts/rules/content.md` — variant (+1).
   - `00_nucleo/prompts/rules/lang.md` — se A.3 → (β) refactor
     P155, propagar hash.
   - Propagar hashes via `crystalline-lint --fix-hashes`.
9. Actualizar Tabela C linha 380 — marcar resolvida
   (`~~strikethrough~~` paralelo P156J e P284).
10. Actualizar Tabela A.3 — adicionar **linha nova** para
    `smartquote` (função stdlib) se não existir, ou estender a
    linha 60 (markup) com nota cruzada P287. **Decisão de
    apresentação na Fase A.1 ponto 3**.

**Sem caps** (per P282 §7). Estimativa de testes: ~8-15 (delta
modesto; passo focado num cluster mínimo: 1 variant + 1 função +
consumer + regression).

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P286: 2 732 testes.
  Esperado: ~2 740 a ~2 747.
- `crystalline-lint` zero violations.
- Hash L0 `content.md` muda (+1 variant).
- Hash L0 `stdlib.md` muda (+1 função).
- Hash L0 `lang.md` muda **se** A.3 → (β) (improvável).
- Hash L0 `export.rs` **preserved** (`66cb8ac3`) **se** A.4 → (i).
  Muda **se** A.4 → (ii) — registar como esperado.
- **Regressão bit-exact validada** para markup `"..."` — caso
  P155 produz mesmo PDF byte-a-byte (sem regressão na markup
  existente).
- Tabela C linha 380 marcada resolvida.
- Tabela A.3 actualizada (linha existente ou nova; decisão A.1.3).
- Cobertura Text features: 10/5/1/5/2 → 11/5/1/4/2 (= 23) **se**
  linha nova; ou inalterada **se** smartquote era subsumido em
  linha existente.
- Diagnóstico A.1+A.2+A.3+A.4 produzido.
- **Padrão "variant rico com cosméticos opcionais" atinge N=5
  cumulativo** se A.2 → (b)/(c)/(d) — relatório regista para
  promoção natural ADR meta no próximo passo que cite o padrão.
  Promoção formal **não é objectivo deste passo**.

---

## §5 — Não-objectivos

- **Não** materializar `text.smartquotes` (atributo `set text(...)`)
  se cristalino não o tem actualmente. Se Fase A.1.3 mostrar que é
  necessário para enabled=false, registar como sub-passo P287.1
  candidato.
- **Não** estender lang-aware quotes além das 6 línguas já
  presentes em P155. Adições são passos próprios.
- **Não** materializar `alternative: bool` se decisão A.2 → (a)/(b).
  Continua ADR-0054 graded para passo futuro condicional.
- **Não** promover ADR meta "variant rico com cosméticos
  opcionais" mesmo se N=5 atingido. Relatório regista limiar; o
  passo seguinte é que dispara naturalmente.
- **Não** alterar a alternância open/close de P155 a menos que A.3
  → (β) o exija explicitamente. Bit-exactness do markup é
  critério de fecho duro.
- **Não** materializar `#show smartquote: ...` integration. Se A.4
  → (ii) facilita, registar oportunidade — mas show rules sobre
  SmartQuote é passo distinto.

---

## §6 — Pendências relacionadas

Resolve:
- **Tabela C linha 380** — `smartquote` ausente.

Não resolve (continua aberto):
- `smallcaps` (Tabela A.3 linha 99) — bloqueado por rustybuzz
  (DEBT-53).
- `text.region`/`text.dir`/`text.script` (linhas 96-98) —
  bloqueados por shaping XL.
- `lorem` (linha 102) — escopo S; passo dedicado.
- Soft hyphen Unicode (linha 61) — passo dedicado.
- Objecto `Stroke` rico (Tabela A.7 linha 201) — passo distinto.

---

## §7 — Risco residual

Risco principal: bit-exactness da markup `"..."` (P155). Se A.3 →
(β) refactor consolidar a alternância no Layouter, o caminho do
parser muda. Mitigação: criar teste regression dedicado **antes**
do refactor — capturar bytes PDF de 3-5 documentos representativos
de P155, verificar identidade após refactor. Se A.3 → (γ) (default
sugerido), risco é nulo — caminho P155 preservado literalmente.

Risco secundário: A.4 → (ii) (tabela interna L3 em vez de reusar
P155) cria divergência entre markup e função stdlib. Se aparecer,
documentar como divergência ADR-0054 graded mas preferir A.4 → (i)
salvo razão estrutural forte.

Risco terciário: estado open/close em chamadas programáticas. Em
`#for i in range(10) { smartquote() }`, todas as chamadas estão na
mesma "linha"; vanilla typst alterna open/close em cada chamada
(estado persistente per-document). Mitigação: A.3 deve clarificar
explicitamente o scope do estado (per-document; não per-paragraph).
Se A.1 revelar que P155 mantém o estado per-paragraph e reset em
`flush_line`, registar divergência potencial e decidir
explicitamente o comportamento da função.

Risco quaternário: gatilho ADR meta N=5 acidentalmente disparado
se este passo citar o padrão "variant rico" e simultaneamente o
padrão "single source of truth". Mitigação: §5 explicita
não-objectivo de promoção; relatório do P287 distinguirá citações
de gatilhos.

---

## §8 — Ponteiros

- Vanilla: `lab/typst-original/crates/typst-library/src/text/smartquote.rs`
  + `text/quotes.rs`.
- Cobertura referência: Tabela A.1 linha 60 (markup já
  implementado P155); Tabela C linha 380 (função ausente).
- Infraestrutura existente: `01_core/src/rules/lang/quotes.rs`
  (resolução lang-aware P155); `01_core/src/eval/markup.rs` (ou
  caminho equivalente) — alternador open/close em `eval_markup`.
- Precedente directo "variant rico com cosméticos opcionais":
  P156G/H/I (Block/Boxed/Stack) + P284 (Underline/Strike/Overline)
  — N=4 cumulativo pré-P287; este passo qualifica como N=5 **se**
  A.2 → (b)/(c)/(d).
- Precedente "consumer Layouter consulta estado mutável": P286
  `decoration_lines_collector` (campo opcional `None` default).
  Aplicação simétrica candidata: `smartquote_state`.
- ADR processual: ADR-0065 (inventariar-primeiro — 4 critérios
  cobertos por A.1/A.2/A.3/A.4).
- ADR scope: ADR-0054 graded (justifica não-objectivos §5).
- ADR cultural: P273.17 §0 anti-padrão over-formalização
  (justifica **não** promover ADR meta neste passo mesmo se N=5
  atingido).

---

*Spec P287 produzida 2026-05-19 pós-P286 (cluster decorações
P284-P285-P286 COMPLETO). Frente `P-smartquote` — completa Tabela
C linha 380 com `Content::SmartQuote` + função stdlib +
integração consumer Layouter. Reusa infraestrutura lang-aware
existente desde P155. Fase A obrigatória (4 ambiguidades:
inventário P155, estrutura variant, política estado open/close,
política glyph emit). Critério de fecho inclui validação
bit-exact para markup `"..."` existente. Padrão "variant rico com
cosméticos opcionais" atinge N=5 cumulativo se A.2 → (b)/(c)/(d) —
gatilho histórico para promoção ADR meta; promoção **não é
objectivo deste passo**, fica para passo subsequente que cite o
padrão. Sem caps LOC ou magnitude (P282 §7).*
