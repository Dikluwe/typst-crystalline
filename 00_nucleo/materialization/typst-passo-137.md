# Passo 137 — Consumer `tracking` em layout (Fase B)

**Série**: 137 (passo **S** em L1/L3; **primeiro consumer com
efeito observável** da série desde Passo 102).
**Precondição**: Passo 136 encerrado; 1089 total tests; zero
violations; 54 ADRs activas; 12 DEBTs abertos (DEBT-52 com 7
gaps restantes). `TextStyle.tracking` propagado até
`FrameItem::Text.style.tracking`.

**ADRs aplicáveis**:
- **ADR-0033** (paridade funcional).
- **ADR-0054** (critério fecho DEBT-1).
- **DEBT-52** fase B — primeiro consumer.

**Natureza**: passo L1 + potencial L3 (se exporter precisa
alteração). Primeiro efeito visível em PDF desde o Passo 102
(`fill`). Fim da série de propriedades inertes.

---

## Contexto

Passo 136 completou Fase A: `TextStyle.tracking` viaja do
eval até ao frame via `FrameItem::Text.style`. Mas layout
actual calcula advance entre glyphs sem consultar o campo.

Este passo adiciona o consumer: advance entre glyphs soma
`tracking` (resolvido contra `size` para Pt), excepto no
último glyph do span.

Vanilla aplica tracking **entre cada par de glyphs**, não só
entre palavras. Decisão desta série alinha — Opção 1
confirmada.

---

## Risco central

**Gate crítico em 137.A**: o exporter PDF actual pode não ter
controlo fino de positioning. Se é assim, `tracking` não
produz efeito visível mesmo aplicado em layout — fica
capturado no frame mas perde-se no export.

Se o exporter **não suporta** positioning glyph-a-glyph:
- Consumer em layout é possível (ajusta frame).
- Mas efeito visível exige estender o exporter — **passo
  separado** de infra PDF, não XS/S.

Decisão de aceitação "efeito visível é requisito" obriga que
o exporter suporte. Se não suporta, parar e escrever passo
dedicado de exporter antes deste.

---

## Contexto estratégico

Fase B do roadmap 135:

- **136** (feito): Fase A — extensão de TextStyle.
- **137** (este): Fase B.1 — consumer tracking.
- **138**: Fase B.2 — consumer leading.
- **139**: Fase B.3 — consumer weight faux-bold.
- **140-143**: Fase C.
- **Fecho DEBT-1** após Fase A+B+C.

---

## Objectivo

Ao fim do passo:

1. Consumer `tracking` aplicado em layout:
   - Em cada par de glyphs consecutivos dentro de um span,
     `advance` efectivo = `glyph_advance + tracking_pt`.
   - Último glyph do span não ganha tracking.
   - `tracking_pt` resolvido de `Length` contra `size` actual.
2. Exporter PDF consome o positioning fino (glyph-a-glyph) ou,
   se já o fazia, reflecte o tracking automaticamente.
3. Teste L1 numérico: `#set text(tracking: 1em)\nAB` produz
   frame com distância entre A e B igual a `advance_A + size`.
4. Teste manual visual: PDF com e sem tracking mostra diferença
   observável (espaçamento entre letras).
5. Canary `hyphenate` preservado.

Este passo **não**:

- Adiciona consumer para outras propriedades (138+).
- Resolve font embedding real (fase C).
- Altera advance dos glyphs base (só adiciona tracking).
- Fecha DEBT-52 (resolve apenas gap 2 de 8).

---

## Decisões já tomadas

1. **Tracking entre cada par de glyphs** (Opção 1 — fiel
   vanilla).
2. **Último glyph sem tracking** (comportamento vanilla; `tracking`
   é *entre*, não *depois*).
3. **Resolução de `Length` contra `size`**: padrão existente
   no código (provavelmente helper `.resolve(size)` ou similar).
4. **Efeito visível é requisito**: teste manual mostra
   diferença no PDF.

## Decisões diferidas (137.A)

5. **Onde aplicar tracking** no pipeline de layout: no cálculo
   de advance inline (mais acoplado), ou como pass separada
   após advance base (mais isolado). Decisão conforme estrutura
   actual.
6. **Se exporter suporta positioning glyph-a-glyph**: resposta
   em 137.A.5. Se não, passo pausa.
7. **Forma exacta do helper de resolução `Length → Pt`**:
   nome varia conforme convenção do código.

---

## Escopo

**Dentro**:
- `01_core/src/rules/layout/...` — onde advance é calculado.
- `01_core/src/rules/layout/tests.rs` — 1 teste numérico.
- `03_infra/src/export/...` — **possivelmente** se exporter
  precisa de alteração.
- `00_nucleo/DEBT.md` — marcar gap 2 resolvido.

**Fora**:
- Outras propriedades (138+).
- Font embedding (fase C).
- Infra nova de exporter (se não existe, **parar**).
- L2, L4.
- ADR nova.

---

## Sub-passos

### 137.A — Inventário confirmatório

**A.1 — Localizar cálculo de advance**:

`grep -rn "advance\|glyph.*position\|layout.*text" 01_core/src/rules/layout/`.

Registar:
- Função onde advance é calculado por glyph.
- Tipo de dados (Pt? f32? Length?).
- Se há helper para "soma positional".

**A.2 — Resolução `Length → Pt`**:

`grep -rn "\.resolve\|\.to_pt\|Length.*resolve" 01_core/src/`.

Procurar helper/método que converte `Length` (com componentes
abs + em) para `Pt` absoluto dado um `size` de referência.

Se existe, registar assinatura.
Se não existe, adicionar método a `Length` neste passo (trivial).

**A.3 — Tests existentes de positioning**:

`grep -rn "advance\|position.*assert\|frame.*text.*offset" 01_core/src/rules/layout/tests.rs`.

Registar:
- Existem testes que assertam offset numérico de glyphs?
- Forma das assertions (exacta em Pt?).
- Serão base para o novo teste.

**A.4 — Último glyph sem tracking**:

Verificar se existe convenção no código actual para "última
iteração é especial" em loops de glyphs. Se sim, replicar.
Se não, condicional explícito `if i < glyphs.len() - 1`.

**A.5 — Gate crítico: exporter PDF e positioning**:

`grep -rn "export\|pdf.*write\|text.*show\|Td\|TJ" 03_infra/src/`.

Registar:
- Como texto é escrito para PDF.
- Se usa operador PDF `TJ` (text-showing com kerning individual)
  ou `Tj` (simples).
- Se aceita positioning individual por glyph ou só strings.

**Gate**: se exporter usa apenas `Tj` com strings planas e não
tem controlo de positioning individual, **parar e reportar**.
Consumer tracking em layout não produz efeito visível. Passo
dedicado de exporter é pré-requisito.

**A.6 — Spans e iteração**:

Como é um `FrameItem::Text`? Contém:
- Uma string inteira + TextStyle global?
- Slice de glyphs com positions individuais?

Registar. Determina onde aplicar tracking (a nível de frame,
a nível de glyph, a nível de advance).

**A.7 — Tests base**:
- L1: 858.
- Total: 1089.

**Gate 137.A**:
- Se A.5 revela exporter sem positioning fino: **parar e
  escrever passo dedicado de exporter** antes deste.
- Se A.2 revela que `Length::resolve(size) -> Pt` não existe
  e é não-trivial (ex: depende de contexto): registar e
  adicionar método simples; se complexo, escala.
- Se A.6 revela que `FrameItem::Text` é string plana sem
  glyph-level: decisão — aceitar aproximação Opção 2 (entre
  palavras via space expansion) ou reformar representação
  (grande; excede S).
- Outros casos: prosseguir.

### 137.B — Adicionar helper `Length::resolve` se necessário

Se A.2 revela que não existe, em `01_core/src/entities/length.rs`:

```rust
impl Length {
    /// Resolve `Length` para `Pt` dado um `size` de referência.
    /// `abs + em * size` em Pt.
    pub fn resolve(&self, size: Pt) -> Pt {
        self.abs + Pt::from_raw(self.em.to_raw() * size.to_raw())
        // Ajustar conforme API de Pt real.
    }
}
```

Se existe, não tocar.

### 137.C — Consumer em layout

**Ficheiro**: conforme A.1.

Local exacto depende da estrutura. Esperado: função que itera
glyphs e acumula positions. Pseudo-código:

```rust
// pattern antes:
let mut x = start_x;
for glyph in glyphs {
    place_glyph(glyph, x, y);
    x += glyph.advance;
}

// pattern depois:
let tracking_pt = style.tracking
    .map(|t| t.resolve(style.size))
    .unwrap_or(Pt::ZERO);

let mut x = start_x;
let n = glyphs.len();
for (i, glyph) in glyphs.iter().enumerate() {
    place_glyph(glyph, x, y);
    x += glyph.advance;
    if i < n - 1 {
        x += tracking_pt;
    }
}
```

**Alternativa mais idiomática** (se suportada):

```rust
for glyph in &glyphs[..n-1] {
    place_glyph(glyph, x, y);
    x += glyph.advance + tracking_pt;
}
if let Some(last) = glyphs.last() {
    place_glyph(last, x, y);
    x += last.advance;
}
```

Escolher a que se alinha melhor com o estilo existente.

### 137.D — Exporter adaptação (se necessário)

Se A.5 revela que o exporter já faz positioning fino: **nada
a fazer aqui**. Consumer em layout produz frame com positions
correctas; exporter já as consome.

Se o exporter não faz: **parar**. Escrever passo dedicado (137A
ou 137.5) para estender exporter com positioning fino. Depois
re-enunciar este como 137B.

### 137.E — Teste L1 numérico

**Ficheiro**: `01_core/src/rules/layout/tests.rs`.

```rust
#[test]
fn layout_tracking_aplica_entre_glyphs_passo_137() {
    let src = "#set text(tracking: 1em, size: 12pt)\nAB";
    let frame = layout_sample(src);

    // Frame tem texto "AB". Advance do A é conhecido (Helvetica
    // hardcoded no export). Tracking = 1em = 12pt (porque
    // size = 12pt).
    //
    // Posição esperada do B: x_A + advance_A + 12pt.

    let text = extract_text_item(&frame);
    let positions = positions_dos_glyphs(&text);
    let distancia_a_b = positions[1].x - positions[0].x;
    let advance_a = advance_de_char('A', 12.0);

    let esperado = advance_a + 12.0;  // em Pt
    assert!((distancia_a_b - esperado).abs() < 0.01,
        "distância A-B esperada {} Pt, real {} Pt", esperado, distancia_a_b);
}

#[test]
fn layout_tracking_zero_nao_altera_positioning_passo_137() {
    // Regressão: sem set text(tracking), positioning igual a
    // antes do 137.
    let src = "AB";
    let frame = layout_sample(src);
    let text = extract_text_item(&frame);
    let positions = positions_dos_glyphs(&text);
    let distancia_a_b = positions[1].x - positions[0].x;
    let advance_a = advance_de_char('A', 11.0);  // size default
    assert!((distancia_a_b - advance_a).abs() < 0.01);
}

#[test]
fn layout_tracking_ultimo_glyph_nao_ganha_passo_137() {
    // Comprimento total de "AB" com tracking = advance_A + tracking + advance_B.
    // Não é advance_A + tracking + advance_B + tracking.
    let src = "#set text(tracking: 1em, size: 12pt)\nAB";
    let frame = layout_sample(src);
    let text = extract_text_item(&frame);
    let comprimento = comprimento_total(&text);
    let esperado = advance_de_char('A', 12.0) + 12.0 + advance_de_char('B', 12.0);
    assert!((comprimento - esperado).abs() < 0.01);
}
```

Helpers `layout_sample`, `extract_text_item`,
`positions_dos_glyphs`, `advance_de_char`, `comprimento_total`
podem já existir ou precisar de ser adicionados.

Se A.3 revela que harness não expõe positions numéricas,
adicionar método/teste access pequeno.

### 137.F — Teste manual visual

Gerar dois PDFs e comparar:

```bash
$ cat sem.typ
AB

$ typst sem.typ -o sem.pdf
$ cat com.typ
#set text(tracking: 1em)
AB

$ typst com.typ -o com.pdf
$ # Abrir ambos e comparar visualmente.
```

Critério: espaçamento entre A e B visivelmente maior em `com.pdf`.
Registar screenshots ou diff de bytes no relatório (PDF binário
diff é grande; screenshot basta).

### 137.G — Canary e regressão

- Canary `eval_set_text_hyphenate_canary_passo_132b` passa.
- Testes existentes de layout passam (sem regressão).
- Efeito visível só em inputs com `tracking` explícito.

### 137.H — DEBT-52 actualizar

```markdown
- [x] **Gap 1** (Fase A): TextStyle + propagação. **Passo 136**.
- [x] **Gap 2** (Fase B.1): consumer tracking. **Passo 137**.
- [ ] **Gap 3** (Fase B.2): consumer leading.
- [ ] ...
```

### 137.I — Verificação

1. `cargo test -p typst-core` — L1: 858 → **861** (+3 testes
   novos).

2. `cargo test --workspace` — total ≥ 1092.

3. `crystalline-lint` zero violations.

4. Manual:

```bash
$ typst sem.typ      # sem tracking
exit=0, PDF normal

$ typst com.typ      # com tracking 1em
exit=0, PDF com letras espaçadas — visualmente diferente

$ typst h.typ        # canary hyphenate
h.typ:1:11: warning: ...'hyphenate'...
exit=0
```

### 137.J — Encerramento

Relatório `typst-passo-137-relatorio.md`:

- Inventário 137.A (especial A.5 — exporter).
- Se passo foi partido em "137A exporter + 137B consumer",
  registar.
- Diff em layout + exporter (se aplicável).
- Resultado do teste manual (diff visual).
- Números finais.
- Preparação para 138 (leading).

---

## Critério de conclusão

1. Inventário 137.A escrito, especial A.5 explícito.
2. `Length::resolve` existe (novo ou pré-existente).
3. Consumer tracking aplicado em layout entre pares de glyphs.
4. Último glyph não ganha tracking.
5. Exporter PDF reflecte o tracking (directamente ou porque
   já suportava positioning fino).
6. 3 testes L1 numéricos passam (entre, zero, último).
7. Teste manual confirma diferença visível.
8. Canary preservado.
9. L1 tests: **861** (+3).
10. `cargo test --workspace` passa (≥ 1092).
11. `crystalline-lint` zero violations.
12. DEBT-52 gap 2 marcado resolvido.
13. Relatório 137.J escrito.

---

## O que pode sair errado

- **Exporter sem positioning fino (A.5 gate disparar)**: parar
  e escrever passo dedicado de exporter. Este enunciado fica
  para re-uso depois.

- **`FrameItem::Text` é string plana sem glyph-level
  positioning**: representação actual não permite consumer
  fine-grained. Decisão maior — ou aceitar Opção 2 (entre
  palavras) ou reformar representação (grande).

- **`Length::resolve` não existe e é complexo**: depende de
  contexto não-trivial (ex: parent element size, em baseado
  em diferente referência). Registar e adicionar com
  pragmatismo; se complexo, sub-passo.

- **Advance dos glyphs é hardcoded sem granularidade**
  (ex: todos chars iguais a X Pt): teste numérico pode ter
  valores inesperados. Adaptar conforme real.

- **Existem optimizações que assumem advance fixo**
  (ex: ligatures, paragraph justification): adicionar
  tracking pode quebrá-las. Verificar regressão.

- **Teste manual difícil de automatizar**: PDF diff binário
  não é reprodutível byte-a-byte. Screenshot a olho é
  suficiente para critério de aceitação.

---

## Notas operacionais

- **Primeiro consumer com efeito visível** desde Passo 102
  (fill). Marca transição de "infra capturada" para "paridade
  observável crescente".

- **Se exporter gate pára o passo**: é o primeiro sinal de
  que o roadmap 135 subestimou infra de export. Registar
  honestamente; roadmap revisto no relatório 137.

- **Teste numérico é tanto regressão quanto contrato**: valor
  concreto em Pt. Se advance base mudar (ex: quando font
  embedding real chegar), teste precisa de adaptação. Aceite
  como parte do fecho DEBT-1.

- **Efeito "é observável mas não mensurável automaticamente"**:
  teste manual + assertion numérica cobrem juntos. Quando
  infra de golden-file testing existir, adicionar regressão
  binária.

- **Candidato `eval_with_warnings`** continua pendente. Após
  137 o harness de layout/tests ganha mais 3 testes. Fricção
  acumula.

- **Ritmo estimado**: S. ≈1-2h se exporter suporta. Se não
  suporta, pausa e reavalia — pode virar M ou L.
