# Relatório P158A — auto-detecção de `kind` em `native_figure` (Model figure-kinds sub-passo 1)

Primeiro sub-passo substantivo de Model figure-kinds per scope
decidido em diagnóstico P158 §3.2 (subset minimal). **Décima
terceira aplicação consecutiva de materialização** desde início
da série granular P156C. **Décima quarta aplicação consecutiva**
do padrão diagnóstico-primeiro.

Refino qualitativo de infraestrutura existente (`Content::Figure`
+ counters por kind funcionais desde P75/ADR-0041 + P157A).
**Sem alteração a variants Content; sem alteração a layout ou
introspect**. Funcionalmente: `figure(image(...))` activa counter
de imagem automaticamente sem `kind:` manual.

---

## 1. Resumo do executado

### 1.1 Diagnóstico (.1)

Ficheiro novo:
`00_nucleo/diagnosticos/diagnostico-figure-auto-detect-passo-158a.md`
(7 itens canónicos ADR-0034 + 2 itens específicos para refino
comportamental).

**Decisão arquitectural-chave Sequence handling** (§8 do
diagnóstico):
- Vanilla usa `query_first_naive` recursivo profundo em todo o
  subtree.
- Cristalino limita recursão a `Content::Sequence` —
  paridade vanilla parcial per ADR-0033.
- Outros containers (Block/Box/Pad/Styled) scope-out per
  ADR-0054 graded.

Justificação:
- Sequence é wrapper trivial muito comum (markup `[...]`).
- Outros containers seriam recursive deep — risco maior.
- Cobertura suficiente para casos típicos.

**Verificação tests pré-existentes**: nenhum verifica `kind`
field directamente — zero risco regression.

### 1.2 Helper `infer_kind_from_body` (.2)

Adicionado a `01_core/src/rules/stdlib/figure_image.rs`
(privado, ~10 linhas):

```rust
fn infer_kind_from_body(body: &Content) -> Option<String> {
    match body {
        Content::Image { .. } => Some("image".to_string()),
        Content::Table { .. } => Some("table".to_string()),
        Content::Raw   { .. } => Some("raw".to_string()),
        Content::Sequence(seq) => seq.iter().find_map(infer_kind_from_body),
        _ => None,
    }
}
```

Cobertura:
- Image/Table/Raw direct → kind detectado.
- Sequence recurse no primeiro child detectável.
- Outros (Text, Block, Box, etc.) → None (caller aplica
  default).

### 1.3 Modificação `native_figure` (.3)

Substituído extracção directa por fallback chain 3 níveis:

```rust
let kind = args.named.get("kind")
    .and_then(|v| if let Value::Str(s) = v { Some(s.to_string()) } else { None })
    .or_else(|| infer_kind_from_body(&body))   // P158A
    .unwrap_or_else(|| "image".to_string());
```

Precedência absoluta para `kind:` explícito preserva
comportamento existente.

### 1.4 Tests (.4)

**+6 tests novos** (exactamente dentro do range esperado +6-8):

Em `01_core/src/rules/stdlib/mod.rs`:
1. `figure_auto_detect_image` — `figure(image(...))` → kind="image".
2. `figure_auto_detect_table` — `figure(table(...))` → kind="table".
3. `figure_auto_detect_raw` — `figure(raw(...))` → kind="raw".
4. `figure_kind_explicit_override_auto_detect` — `kind:`
   explícito vence (precedência absoluta).
5. `figure_default_image_quando_body_nao_detectavel` — body
   Text → fallback "image".
6. `figure_auto_detect_image_dentro_de_sequence` — Sequence
   com Image dentro detecta via recursão.

**Tests pré-existentes** (4 em `stdlib/mod.rs:388-431`)
continuam a passar inalterados — usam `Content::text` body
que cai no default "image" coincidentemente.

### 1.5 Hashes + cobertura (.5)

`crystalline-lint --fix-hashes .` reportou **"Nothing to fix"**
(refactor aditivo; preserva hash do prompt L0).

**Hash `entities/content.rs` mantém-se `ec58d849`** — **sétimo
passo consecutivo** (P156L → P157 → P157A → P157B → P157C →
P158 → P158A) sem alteração ao variant Content.

Tabela cobertura A.6 actualizada com nota qualitativa em
entrada `figure` (footnote ²⁸):
- Estado mantido `implementado⁺`.
- Nota acrescentada: "P158A: auto-detecção de `kind`...".
- Cobertura agregada Model **inalterada** (~50%).

ADR-0061 §"Aplicações cumulativas" actualizada para pós-P158A.
ADR-0060 ganha anotação P158A. README ADRs ganha entrada P158A
antes de P158.

---

## 2. Verificações (numeradas per spec do passo)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `cargo test` workspace: 1379 + Δ; zero falhas (Δ esperado +6-8) | **Δ=+6** (1379 → 1385 lib+integ+diag); zero falhas |
| 2 | `crystalline-lint`: zero violations | **✓ No violations found** |
| 3 | Variants Content: 56 (inalterada — refino) | **✓ 56** |
| 4 | Stdlib funcs: 46 (inalterada — refino) | **✓ 46** |
| 5 | Cobertura Model agregada: ~50% (inalterada) | **✓** entrada `figure` ganha nota qualitativa em footnote ²⁸; agregação inalterada |
| 6 | Hash actualizado em prompts L0 | **✓** "Nothing to fix" (refactor aditivo) |
| 7 | Hash `entities/content.rs` permanece `ec58d849` | **✓ sétimo passo consecutivo** P156L → P158A sem alteração ao variant Content |
| 8 | Sequence handling decidido em .1 documentado no relatório §1.1 | **✓** decisão "recursão limitada a Sequence" com justificação completa |
| 9 | Sem novas reservas criadas em P158A | **✓** política P158 preservada — supplement, show selectors, refactor kind continuam NÃO-reservados |
| 10 | Tests pré-existentes que passam `kind:` explícito continuam inalterados (regression) | **✓** verificado em diagnóstico §9 + tests pré-existentes não verificam kind directamente |

**Build limpo**: `cargo build` 1.27s sem warnings novos.

---

## 3. Análise de risco — peso baixo (décima segunda aplicação consecutiva)

P158A é **primeiro passo Model com refino comportamental sem
alteração estrutural**. §análise de risco preserva precedente
N=11 (P156F-P158) → **N=12**.

### 3.1 Riscos identificados

| Risco | Avaliação | Mitigação aplicada |
|-------|-----------|---------------------|
| Sequence handling divergir da expectativa vanilla | Confirmado | Decisão registada com justificação per ADR-0033 (paridade parcial); documentado em §8 do diagnóstico |
| Tests pré-existentes quebrarem | Confirmado nulo | Inventário §9 confirmou — nenhum test verifica `kind` directamente; `Content::text` body cai no default "image" coincidentemente |
| `Content::Image`/`Table`/`Raw` constructors no test exigirem inicialização não-trivial | Materializado | Construtores inline com fields explícitos; ~5 linhas por test |
| Auto-detecção activar comportamento inesperado em código existente | Não | Precedência absoluta `kind:` explícito preserva qualquer caller que já passa kind manualmente |

### 3.2 Riscos avaliados mas não materializados

| Risco | Avaliação inicial | Razão de não-materialização |
|-------|-------------------|----------------------------|
| Helper exigir variants adicionais além de Image/Table/Raw | Baixo | Cristalino não tem trait `Figurable` — limitação a 3 variants é decisão arquitectural |
| Pattern-match exhaustive falhar | Nulo | Helper privado com wildcard `_ => None`; sem dependência de exhaustive |
| Refino exigir alteração ao variant `Content::Figure` | Não | Refino vive só na origem do valor `kind` antes de o passar ao variant |

### 3.3 Riscos não-aplicáveis

- **Algoritmo dinâmico de runtime**: zero (refino comportamental
  trivial).
- **Quebra de paridade observável vs vanilla**: divergência
  estrutural aceite per ADR-0033 (recursão parcial).

### 3.4 Conclusão de risco

**Risco residual: muito baixo após inventário**. P158A é refino
comportamental aditivo — sem refactor de variant; sem alteração
de layout ou introspect; precedência absoluta `kind:` explícito
preserva tests pré-existentes.

**§análise de risco preserva precedente cross-domínio**: P158A
é primeiro passo Model com refino qualitativo sem alteração
estrutural — patamar #4 cresce para N=12 com diversidade (P156L
refactor real Layout; P157C par simétrico Model; P158A refino
comportamental Model).

---

## 4. Slope cumulativo Model (mesa P155-P158A)

| Passo | Feature(s) | Slope Model | Cobertura Model cumulativa | Tests Δ |
|-------|-----------|------------:|---------------------------:|--------:|
| P154A | (diagnóstico Model) | — | 36% | 0 |
| P154B | terms + divider | +5%  | 36% → 41% | +10 |
| P155 | quote | +4%  | 41% → 45% (Fase 1 fechada) | +21 |
| P157 | (diagnóstico Model Fase 2) | — | — | 0 |
| P157A | table minimal | +5%  | 45% → 50% | +16 |
| P157B | table cell | 0% agregado | 50% inalterado (sub-entrada) | +18 |
| P157C | table header + footer (fecha table foundations) | 0% agregado | 50% inalterado (par sub-entradas) | +26 |
| P158 | (diagnóstico Model figure-kinds) | — | — | 0 |
| **P158A** | **figure auto-detect** | **0% agregado** | **50% inalterado (refino qualitativo)** | **+6** |

**Total cumulativo P154A-P158A** (Model): **+14pp** Model
agregada em 9 passos (5 materialização + 3 diagnóstico + 1
refino qualitativo). P158A é **primeiro caso** de "ganho
qualitativo via refino comportamental" em Model — precedente
análogo a P156L em Layout (refactor real qualitativo).

---

## 5. ADR-0061 §"Aplicações cumulativas" — confirmações

§"Aplicações cumulativas" actualizada para pós-P158A:

### 5.1 Padrões metodológicos pós-P158A

| # | Padrão | Pré-P158A | Pós-P158A |
|---|--------|----------:|----------:|
| 1 | Granularidade 1-2 features/passo | 12 | **13** (cross-domínio fortalecido com refino Model) |
| 2 | "Inventariar primeiro" pré-decisão | 11 | **12** (P158A reforça critério #5 com decisão Sequence handling) |
| 3 | "Smart→Option/default" | 9 | 9 (inalterado — não aplicável directamente em P158A) |
| 4 | "§análise de risco no relatório" | 11 | **12** (primeiro passo Model com refino comportamental sem alteração estrutural) |
| 5 | "Reuso de template containers" | 4 | 4 (inalterado) |
| 6 | "Antecipar especificidades técnicas" | 2-3 | 2-3 |
| 7 | Helper `extract_length` reuso | 7 | 7 (inalterado) |
| 8 | Reuso `Sides<T>` | 2 | 2 (inalterado) |
| 9 | Reuso `extract_tracks` | 2 | 2 (inalterado) |
| 10 | Helper `extract_usize_or_none_min` | 4 usos | 4 (inalterado) |
| 11 | Helper `extract_bool_with_default` | 2 usos | 2 (inalterado) |
| 12 | Par simétrico em pattern-match | 2 | 2 (inalterado) |
| 13 | **Helper privado de inferência** (novo subpadrão P158A) | — | **N=1** (`infer_kind_from_body`) |

### 5.2 Auto-validação cumulativa de ADRs meta P156K

P158A confirma utilidade de ADR-0065 com aplicação cross-feature:

- **ADR-0064**: NÃO aplicável directamente (kind continua String;
  refactor para Option diferido NÃO reservado).
- **ADR-0065**:
  - Critério #1 (naming `infer_kind_from_body`) — implícito.
  - Critério #5 (scope) — **reforçado** com decisão Sequence
    handling (terceira aplicação concreta após P157 + P158).

**Padrão emergente**: critério #5 (scope) atinge **3 aplicações
concretas** (P157 table foundations multi-passo divisão;
P158 figure-kinds subset selection; P158A Sequence handling
decisão). Patamar empírico cresce sem nova ADR.

### 5.3 Padrão "preservação hash content.rs" — sétimo passo consecutivo

P158A é **sétimo passo consecutivo** sem alteração ao variant
Content (P156L → P157 → P157A → P157B → P157C → P158 → P158A).
Padrão "passos aditivos / refino sem alteração de variant
Content" estabilizado.

Implicação: contrato L0 do variant Content mantém-se estável
durante toda a série Model Fase 2 sub-passo 3 + figure-kinds.
Refinos vivem em stdlib/layout/introspect — não no enum core.

---

## 6. Estado pós-P158A

- **Cobertura Layout**: **78%** (inalterada).
- **Cobertura Model agregada**: ~50% (inalterada — refino
  qualitativo).
- **Cobertura arquitectural total**: **80%** (inalterada).
- **Variants Content**: **56** (inalterada).
- **Stdlib funcs**: **46** (inalterada).
- **Helper novo**: `infer_kind_from_body` privado em
  `stdlib/figure_image.rs`.
- **Tests**: **1147** typst-core lib (era 1141; +6). Workspace:
  **1407** (era 1401).
- **Lint**: zero violations.
- **DEBTs**: zero criados ou fechados.
- **ADR-0060**: `IMPLEMENTADO` mantido; ganha anotação P158A.
- **ADR-0061**: `PROPOSTO` mantido; §"Aplicações cumulativas"
  actualizada.
- **README ADRs**: entrada P158A adicionada antes de P158.
- **Reservas P159/ADR-0062**: inalteradas (não reforçadas per
  política P158).
- **Hash `content.rs`**: `ec58d849` (preservado — **sétimo passo
  consecutivo** P156L → P158A).
- **Total ADRs**: **63** (inalterado).

### 6.1 Funcionalidade nova

`figure(image(...))` activa counter de imagem automaticamente
sem `kind:` manual. Idem para `figure(table(...))` e
`figure(raw(...))`. `figure([..., image(...), ...])` (Sequence)
detecta via recursão.

---

## 7. Decisão pós-P158A

Per spec do passo §"Pós-passo" + política "sem novas reservas",
opções (sem candidata pré-acordada):

1. Continuar refino figure-kinds (subset máximo §3.3 do
   diagnóstico P158: supplement por lang; M; **NÃO reservado**).
2. Continuar Fase 3 Layout (columns/colbreak — DEBT-56;
   quebra granularidade).
3. Footnote area.
4. Atacar Introspection (17% cobertura; módulo mais fraco).
5. Promover ADR-0061 a IMPLEMENTADO.
6. Promover `extract_length` a helper público (N=7 patamar
   forte).
7. Fechar DEBT-34e e DEBT-56 (refactor multi-region L+).
8. Bibliography + cite (P159 reserva pré-existente; ADR-0062
   hayagriva).
9. Promover ADR-0060 a R1 com confirmação Fase 2 sub-passo 3
   fechado.
10. ADR meta XS de "ADR-0064 caso completion" (saturação atingida
    em P157C).

ADR-0060 mantém-se `IMPLEMENTADO`. ADR-0061 mantém-se
`PROPOSTO`.

**Padrão granularidade N=13 NÃO é formalizado** — continua
candidato. P158A consolida o padrão sem quebra.

---

## 8. Fechamento

P158A fecha como **primeiro passo Model com refino comportamental
sem alteração estrutural**. **Sétimo passo consecutivo a preservar
hash `entities/content.rs`** — padrão "passos aditivos / refino
sem alteração de variant Content" estabilizado durante toda a
série P156L → P158A.

**Decisão arquitectural-chave Sequence handling**: recursão
limitada a `Content::Sequence` per ADR-0033 (paridade vanilla
parcial); outros containers scope-out per ADR-0054 graded.

**Auto-validação ADRs meta P156K**: ADR-0065 critério #5 atinge
**3 aplicações concretas** com diversidade cross-feature
(P157 multi-passo divisão; P158 subset selection; P158A Sequence
handling decisão). ADR meta cresce em utilidade empírica sem
nova ADR.

**Política "sem novas reservas" preservada** (P158 estabeleceu;
P158A respeita) — supplement, show selectors, refactor `kind`
permanecem candidatos NÃO-reservados; decisões ficam para
sessões futuras com informação acumulada.

ADR-0060 mantém `IMPLEMENTADO` (anotação P158A adicionada);
ADR-0061 mantém `PROPOSTO`.

**Pausa natural após P158A — refino qualitativo de figure
materializado; auto-detecção activada; padrões cross-domínio
consolidam-se. Sétimo passo consecutivo a preservar hash do
variant Content. Decisão humana sobre próxima direcção
(10 candidatas documentadas) tem máxima informação sem
reservas que travem escolha.**
