# Passo P156J — `repeat` (Layout Fase 3, sub-passo granular)

Continuação directa da série P156C-I. Mantém cadência granular
(1-2 features/passo, N=7 validado). Target pós-passo:
Layout 78% (14/18).

---

## Estado actual antes de começar

- Layout 72% (13/18) após P156I (Stack).
- 1296 tests; zero violations linter.
- 51 variants Content; 41 stdlib funcs.
- Hash actual `entities/content.rs`: `b9ca52c4` (P156I).
- ADR-0061 PROPOSTO; §"Aplicações cumulativas" anotada.

**Leituras prévias obrigatórias**:
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  — entrada `repeat` na tabela A.
- `00_nucleo/materialization/typst-passo-156i-relatorio.md` —
  padrão imediatamente anterior (Stack, Arc<[Content]>).
- `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md` —
  §"Aplicações cumulativas".
- `lab/typst-original/crates/typst-library/src/layout/repeat.rs`
  (vanilla, quarentena) — código de referência.

---

## Natureza do passo

**Tamanho**: M.

**Justificação**: 1 feature (repeat). Trivial em superfície
(content + gap + justify), mas requer inventário cuidadoso do
comportamento vanilla (TOC dot leaders) para preservar
paridade observável (ADR-0033).

Granularidade preservada: 1 feature → mantém N=8 do padrão.

---

## Decisões já tomadas

- **Variant Content**: `Repeat { body, gap, justify }`.
- **Naming**: `Repeat` directo (sem conflito com keyword Rust;
  difere de Box→Boxed em P156H).
- **Tipo `gap`**: `Option<Length>` per padrão Smart→Option/default
  (N=5, P156D-I).
- **Tipo `justify`**: `bool` directo (default false per vanilla).
- **Tipo `body`**: `Arc<Content>` (single child, não Sequence).
- **Stdlib func**: `repeat`.

## Decisões diferidas

- Algoritmo de repetição em runtime layout (cálculo de
  quantidade de cópias para preencher largura disponível) —
  fora do scope de P156J; documenta-se como limitação registada
  per ADR-0033 perfil graded (ADR-0054).

---

## Sub-passos

### .1 Inventário pré-materialização (obrigatório)

Diagnóstico em `00_nucleo/diagnosticos/diagnostico-repeat-passo-156j.md`
com 7 itens mínimos (ADR-0034):

1. Assinatura vanilla `RepeatElem` — campos, defaults, tipos.
2. Comportamento observável (TOC dot leaders; espaçamento;
   alinhamento).
3. Decisões Smart<T> → Option<T>/default a aplicar.
4. Variants Content existentes a estender (nenhuma; novo
   variant).
5. Helpers stdlib reusáveis (`extract_length` per N=5 da série).
6. Limitações aceites (algoritmo de repetição em runtime
   diferido).
7. Tests planeados (variant present + stdlib happy path +
   defaults + edge cases).

### .2 Adicionar variant `Content::Repeat`

`01_core/src/entities/content.rs`:
- Adicionar variant `Repeat { body: Arc<Content>, gap: Option<Length>, justify: bool }`.
- Cobrir todos os pattern-match Content existentes (exhaustive).
- Cobrir todos os pattern-match Style existentes se aplicável.

### .3 Adicionar stdlib func `repeat`

`01_core/src/stdlib/`:
- Func `repeat(body, gap: none, justify: false) -> content`.
- Reusar `extract_length` para `gap`.
- Diagnóstico claro em erro de tipo.

### .4 Tests

- Tests unitários do variant (criação, pattern-match).
- Tests de integração da stdlib func (happy path; defaults;
  arg inválido).
- Smoke test de paridade observável onde aplicável.

### .5 Propagação de hashes

`crystalline-lint --fix-hashes` para propagar hash novo de
`content.rs` aos prompts L0 que o referenciam.

---

## Verificação

Numerada para reporte de conclusão:

1. `cargo test` em workspace: **1296 + Δ** tests, zero falhas
   (Δ esperado ≈ +18 a +25 baseado em precedente da série).
2. `crystalline-lint`: zero violations.
3. Contagem variants Content: **52** (51 → 52).
4. Contagem stdlib funcs: **42** (41 → 42).
5. Cobertura Layout: **78%** (14/18) — entrada `repeat` marcada
   implementado puro em
   `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
6. Hash actualizado em prompts L0 (`crystalline-lint --check-hashes`
   passa).

---

## Critério de conclusão

- Verificações 1-6 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-156j-relatorio.md`
  produzido com:
  - Resumo do executado.
  - Confirmação de cada verificação.
  - §análise de risco (padrão N=4; mesmo sendo passo aditivo,
    documentar para preservar precedente).
  - Slope cumulativo actualizado (mesa P156C-J).
  - ADR-0061 §"Aplicações cumulativas" anotada com P156J
    (sem promoção formal — promoção continua diferida).

---

## O que pode sair errado

**Cenários gerais**:
- Inventário .1 revela campo vanilla não antecipado → expandir
  variant antes de avançar para .2.
- Pattern-match exhaustive falha em variant existente fora de
  `content.rs` → grep por `Content::` em todo o crate.

**Cenários específicos**:
- `justify: true` ter interacção subtil com gap em vanilla que
  não foi capturada no inventário → registar como limitação
  graded (ADR-0054) se runtime layout estiver fora de scope.
- Conflito de naming `repeat` com método existente em traits
  Rust (Iterator::repeat, str::repeat) — não afecta variant
  nem stdlib func, mas verificar se algum helper interno usa
  `repeat` como identificador.

---

## Notas operacionais

- Reuso explícito de `extract_length` (P156C; reusado P156D/E/G/H/I)
  — sexta aplicação consecutiva. Documentar no relatório
  como N=6 do helper (subpadrão dentro de "reuso de template
  containers" N=3).
- Cadência granular preserva-se: 1 feature → +6% cobertura.
  Slope esperado consistente com P156E/F/H (passos +6%).
- Não materializar runtime layout do repeat — apenas variant +
  stdlib + paridade estrutural per ADR-0033.

---

## Pós-passo

Após conclusão de P156J, próxima decisão:
- Continuar com P156K-meta (formalizar padrões em ADR meta —
  Smart→Option N=5 + inventariar-primeiro N=4), enunciado
  separado já preparado.
- Ou outra direcção das 7 candidatas se contexto mudar.

ADR-0061 mantém-se PROPOSTO. Promoção a IMPLEMENTADO continua
diferida (3 caminhos documentados; nenhum activado por P156J).
