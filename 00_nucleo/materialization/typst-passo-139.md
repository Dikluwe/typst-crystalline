# Passo 139 — Consumer `weight` faux-bold (Fase B.3)

**Série**: 139 (passo **S** em L1 + L3; **terceiro consumer
com efeito visível**. Último da Fase B).
**Precondição**: Passo 138 encerrado; 1095 total tests; zero
violations; 54 ADRs activas; 12 DEBTs abertos (DEBT-52 com 5
gaps). `TextStyle.weight: Option<u16>` propagado até frame
(Passo 136). Tracking (137) e leading (138) activos.

**ADRs aplicáveis**:
- **ADR-0033** (paridade funcional, lida em perfil
  observacional graded ADR-0054).
- **ADR-0054** (critério fecho DEBT-1).
- **DEBT-52** fase B.3 — último consumer simples.

**Natureza**: passo L1 + L3. Consumer em exporter via PDF
primitives `Tr` e `w`. Estratégia "faux-bold" — aproximação
visual, não tipográfica verdadeira (vanilla usa font embedding
real; cristalino ainda não).

---

## Contexto

Passos 137 (tracking) e 138 (leading) fecharam os eixos
horizontal e vertical de posicionamento. Este passo fecha
**visibilidade de weight** — valor que está capturado em
`TextStyle.weight` desde o Passo 136 mas não produz efeito.

Vanilla resolve weight via `FontBook::select(weight, ...)` +
embedding da variante escolhida. Cristalino não tem embedding
real. A estratégia "faux-bold" — stroke adicional a glyphs —
é aproximação até Fase C (140+) materializar embedding.

---

## Estratégia: Opção A — faux-bold via PDF stroke

Decisão tomada fora do enunciado:

1. Weight mapeia para **PDF stroke width** contínuo.
2. Weight 400 → 0pt stroke (regular, sem efeito visível).
3. Weight 700 → stroke visível (bold, faux).
4. Weights intermédios e extremos interpolam linearmente.
5. Stroke escala com `size` para manter proporção visual.

**Contrato obrigatório** (testes asseram):
- `weight 400` produz output idêntico a `sem weight set`.
- `weight 700` produz output visivelmente diferente de 400.

**Aceite**:
- `weight 500` pode produzir output idêntico a `400` se a
  fórmula resolve abaixo do stroke mínimo renderizável.
- Outros valores aproximam — não há garantia de distinção
  fina 500 vs 600.

---

## Fórmula proposta (ponto de partida)

```
factor = max(0, (weight - 400) / 300)     // 0..1.67 para 400..900
stroke_pt = factor × size_pt × K
```

Onde `K` é constante a calibrar — candidato inicial: **K = 0.04**.

Exemplo com `size = 11pt, K = 0.04`:
- Weight 400: `factor = 0`, stroke = 0pt (regular).
- Weight 500: `factor = 0.33`, stroke = 0.15pt.
- Weight 700: `factor = 1.0`, stroke = 0.44pt (bold).
- Weight 900: `factor = 1.67`, stroke = 0.73pt.

**K pode precisar ajuste** após teste visual em 139.F.
Registar decisão final no relatório.

**Weight abaixo de 400** (thin, extralight, light — 100, 200,
300): `factor = 0` (clamp a zero). Não há stroke negativo.
Produz output idêntico a regular. Aceite como limitação da
estratégia faux-bold.

---

## Contexto estratégico

Fase B do roadmap 135 **completa após 139**:

- 136 (feito): Fase A — TextStyle.
- 137 (feito): Fase B.1 — tracking.
- 138 (feito): Fase B.2 — leading.
- **139 (este)**: Fase B.3 — weight faux-bold.
- 140-143: Fase C — font, lang, embedding.
- Fecho DEBT-1 após Fase C.

---

## Objectivo

Ao fim do passo:

1. **Fórmula de stroke validada** no inventário (139.A) contra
   PDF rendering real — K calibrado.
2. **Consumer aplicado no exporter**: dentro de `BT/ET`, emitir
   `2 Tr` + `{stroke} w` quando `style.weight` resolve a stroke
   non-zero.
3. **Resetar state** após o span: `0 Tr` + `0 w` (ou restore
   PDF state saveR/restore `q/Q`).
4. **Weight 400 = zero regressão**: output idêntico a sem
   weight set.
5. **Weight 700 visivelmente diferente**: teste manual confirma.
6. **4 testes L1**: propagação já coberta (136); adicionar
   tests numéricos da fórmula (não requerem render).
7. **Exporter modificado** pela segunda vez (primeira foi 137
   com `Tc`).
8. **Canary preservado**.

Este passo **não**:

- Resolve font embedding real.
- Faz `FontBook::select` em L1.
- Aborda weight simbólico separadamente (já cobre todos via
  fórmula contínua).
- Fecha DEBT-52 (resolve gap 4 de 8; restam 4).

---

## Decisões diferidas (139.A)

1. **`K` exacto**: 0.04 é proposta. Pode precisar 0.03 ou
   0.05 conforme resultado visual.
2. **Weight default**: se `weight = None`, qual o valor tratado
   como "base"? Candidatos:
   - `None` = 400 implícito (regular), stroke 0.
   - `None` = sem consumer (nem resetar state).
   - Se o código de export actualmente não tem state tracking
     de `w`, qualquer opção é equivalente.
3. **Sítio exacto do emit**: antes do `Tj` mas dentro de
   `BT/ET`. Confirmar pattern.
4. **Preservação de state entre spans**: se dois `BT/ET`
   consecutivos, ambos precisam de emit individual (como `Tc`
   no 137) — state PDF não atravessa BT boundaries.

---

## Escopo

**Dentro**:
- `01_core/src/rules/layout/...` — possível helper para resolver
  stroke_pt a partir de weight + size.
- `03_infra/src/export.rs` — emit `Tr` + `w` dentro de BT/ET.
- `01_core/src/rules/layout/tests.rs` — 4 testes numéricos.
- `00_nucleo/DEBT.md` — marcar gap 4 resolvido.

**Fora**:
- Font embedding.
- `FontBook::select`.
- Weight negativo / thin render diferenciado.
- L2, L4.
- ADR nova.

---

## Sub-passos

### 139.A — Inventário confirmatório

**A.1 — Exporter BT/ET actual**:

`grep -n "BT\|/F1\|Tf\|Tj\|Tc\|Tr\|Td" 03_infra/src/export.rs`.

Registar:
- Estrutura actual do format!.
- Como `Tc` foi adicionado (Passo 137) — replicar pattern
  para `Tr` e `w`.
- Se há estado partilhado entre spans ou cada `BT/ET` é
  independente.

**A.2 — Helper de resolução weight → stroke_pt**:

Decidir localização:
- Pode viver em `TextStyle` como método
  (`self.stroke_pt_from_weight(size)`).
- Pode viver em helper standalone em `layout_types.rs`.
- Pode viver inline no exporter.

Recomendação: método em `TextStyle` (encapsulamento).

**A.3 — Fórmula validada**:

Testar mentalmente com `K = 0.04`:
- size 11, weight 700: stroke ~0.44pt — visivelmente bold em
  PDF a 72dpi.
- size 11, weight 500: stroke 0.15pt — provavelmente
  renderizado igual ou quase igual a 0pt (sub-pixel).

K pode ser maior. Teste visual 139.F calibra.

**A.4 — Teste em harness**:

Os testes de propagação (136) já cobrem `TextStyle.weight =
Some(N)`. Neste passo, adicionar tests sobre a **fórmula**:
valor de stroke_pt para inputs típicos.

**A.5 — PDF state reset**:

Confirmar se cada `BT/ET` no exporter actual tem state
independente. Se sim: cada emit de `weight != 400` precisa de
stroke incluído no BT; próximo BT começa de zero.

**A.6 — `Tr` operator**:

PDF spec:
- `0 Tr`: fill only (default).
- `1 Tr`: stroke only.
- `2 Tr`: fill + stroke.
- `3 Tr`: invisible.

Para faux-bold, **`2 Tr`** (fill + stroke) é correcto. `1 Tr`
produz outline vazio; `3 Tr` esconde.

**A.7 — Tests base**:
- L1: 864.
- Total: 1095.

**Gate 139.A**:
- Se A.1 revela que BT/ET tem state persistente entre blocks:
  ajustar plano para evitar stroke aplicar-se a spans
  subsequentes sem weight. Improvável — PDF spec diz state
  é BT-local para text-specific operators mas `w` é gráfico
  e pode atravessar.
- Se A.5 confirma que `w` atravessa BT (é graphics state, não
  text state): **emit `0 w` para resetar** após o span com
  weight. Ou usar `q/Q` (save/restore graphics state).

### 139.B — Helper weight_stroke

Adicionar método a `TextStyle`:

```rust
impl TextStyle {
    /// Computa stroke width para faux-bold baseado em weight
    /// e size. Weight <= 400 devolve 0 (sem stroke).
    pub fn faux_bold_stroke_pt(&self, k: f64) -> f64 {
        let w = self.weight.unwrap_or(400);
        let factor = ((w as f64 - 400.0) / 300.0).max(0.0);
        factor * self.size.val() * k
    }
}
```

Com `k = 0.04` hardcoded como constante no exporter (ou `pub
const FAUX_BOLD_K: f64 = 0.04` em algum sítio L1).

Alternativa: dois métodos, um sem parâmetro k (usa constante
interna) e outro com, para teste.

### 139.C — Exporter emit

Pattern similar ao `Tc` do 137:

```rust
// dentro do format! de BT/ET:

let weight_stroke_pt = style.faux_bold_stroke_pt(FAUX_BOLD_K);
let bold_ops = if weight_stroke_pt > f64::EPSILON {
    format!("2 Tr\n{:.3} w\n", weight_stroke_pt)
} else {
    String::new()
};

// BT/ET format estendido:
"BT\n/{font_ref} {:.1} Tf\n{tc_op}{bold_ops}{:.1} {:.1} Td\n({safe}) Tj\nET\n"
```

**Reset**: `Tr` volta a default (0) ao sair do BT automaticamente
(text state). `w` pode persistir como graphics state — confirmar
em 139.A.5. Se persiste, emitir `0 w` no final do BT ou usar
`q/Q` para scope.

### 139.D — Constantes e testes da fórmula

Definir `FAUX_BOLD_K` em local apropriado (provavelmente
próximo de `TextStyle::faux_bold_stroke_pt`).

Testes L1 da fórmula (sem render):

```rust
#[test]
fn text_style_faux_bold_400_zero_passo_139() {
    let style = TextStyle {
        weight: Some(400),
        size: Pt(11.0),
        ..Default::default()
    };
    assert_eq!(style.faux_bold_stroke_pt(0.04), 0.0);
}

#[test]
fn text_style_faux_bold_700_positivo_passo_139() {
    let style = TextStyle {
        weight: Some(700),
        size: Pt(11.0),
        ..Default::default()
    };
    let stroke = style.faux_bold_stroke_pt(0.04);
    // 1.0 × 11.0 × 0.04 = 0.44
    assert!((stroke - 0.44).abs() < 0.001);
}

#[test]
fn text_style_faux_bold_100_clamp_zero_passo_139() {
    // Weights abaixo de 400 dão factor negativo.
    // Clamp a zero é aceite.
    let style = TextStyle {
        weight: Some(100),
        size: Pt(11.0),
        ..Default::default()
    };
    assert_eq!(style.faux_bold_stroke_pt(0.04), 0.0);
}

#[test]
fn text_style_faux_bold_escala_com_size_passo_139() {
    let mut style = TextStyle {
        weight: Some(700),
        size: Pt(11.0),
        ..Default::default()
    };
    let s11 = style.faux_bold_stroke_pt(0.04);

    style.size = Pt(22.0);
    let s22 = style.faux_bold_stroke_pt(0.04);

    // Size dobra → stroke dobra.
    assert!((s22 - 2.0 * s11).abs() < 0.001);
}
```

### 139.E — Teste de regressão em frame

```rust
#[test]
fn layout_weight_400_identico_a_sem_set_passo_139() {
    let sem = layout_typst("texto");
    let com = layout_typst("#set text(weight: 400)\ntexto");

    let sem_items = text_items_with_xy(&sem);
    let com_items = text_items_with_xy(&com);

    assert_eq!(sem_items.len(), com_items.len());
    for (s, c) in sem_items.iter().zip(com_items.iter()) {
        // Posições idênticas; TextStyle pode diferir em weight
        // (None vs Some(400)) mas stroke resolve a zero.
        assert_eq!(s.pos, c.pos);
    }
}
```

Não asserta PDF bytes (exporter emit diferente mesmo com stroke
0 porque `weight: Some(400)` passa pelo arm de computação).
Asserta que **frame** (antes do exporter) é equivalente em
positions — prova que weight 400 não altera layout.

Para validação do exporter, teste manual 139.F.

### 139.F — Teste manual visual

```bash
$ cat regular.typ
HELLO

$ cat w400.typ
#set text(weight: 400)
HELLO

$ cat w700.typ
#set text(weight: 700)
HELLO

$ cat w900.typ
#set text(weight: 900)
HELLO

$ typst regular.typ -o regular.pdf
$ typst w400.typ -o w400.pdf
$ typst w700.typ -o w700.pdf
$ typst w900.typ -o w900.pdf

$ grep -a "Tr\|w " w*.pdf regular.pdf
```

**Verificações**:
- `regular.pdf` e `w400.pdf` **não têm** `Tr` ou `w` operators
  (ou têm `0 Tr / 0 w` que é default).
- `w700.pdf` tem `2 Tr` + `~0.44 w`.
- `w900.pdf` tem `2 Tr` + `~0.73 w`.
- **Abrir visualmente**: w700 deve parecer bold; w900 mais
  bold; w400 idêntico a regular.

Se w700 não parece bold, **calibrar K** para cima (ex: 0.06,
0.08). Registar valor final no relatório.

### 139.G — Reset de graphics state

Confirmar em 139.A.5. Se `w` persiste fora do BT, duas opções:

**Opção A — emit `0 w` no final**:
```
BT /F1 11 Tf 2 Tr 0.44 w 70 700 Td (HELLO) Tj 0 Tr 0 w ET
```

**Opção B — save/restore graphics state**:
```
q BT ... ET Q
```

Opção A é mais leve. Opção B é mais robusta mas envolve outros
operators.

Escolher com base no que é mais limpo no código actual.

### 139.H — Canary

`eval_set_text_hyphenate_canary_passo_132b` continua a passar
(weight não afecta mecanismo de warning).

Tests de 137 (tracking) e 138 (leading) passam — não há
conflito.

### 139.I — DEBT-52 actualizar

```markdown
- [x] Gap 1 (Fase A). Passo 136.
- [x] Gap 2 (Fase B.1 tracking). Passo 137.
- [x] Gap 3 (Fase B.2 leading). Passo 138.
- [x] Gap 4 (Fase B.3 weight faux-bold). **Passo 139**.
- [ ] Gap 5-7 (Fase C): font string, font array, lang.
- [ ] Gap 8 (opcional): font dict.
```

4 gaps restantes. Fase B **completa**.

### 139.J — Verificação

1. `cargo test -p typst-core` — L1: 864 → **869** (+5: 4
   unitários da fórmula + 1 de regressão 400).

2. `cargo test --workspace` — total ≥ 1100.

3. `crystalline-lint` zero violations.

4. Manual:

```bash
$ typst regular.typ
PDF sem Tr/w stroke

$ typst w700.typ
PDF com "2 Tr 0.44 w" — glyphs visualmente bold

$ typst h.typ
canary hyphenate warning
```

### 139.K — Encerramento

Relatório `typst-passo-139-relatorio.md`:

- Inventário 139.A (BT/ET pattern, state reset).
- K calibrado (0.04 ou outro).
- Diff layout_types.rs + export.rs + tests.rs.
- Resultado do teste manual (screenshots ou descrição).
- Números finais.
- **Fase B completa**. Fase C (140+) preparada.
- Observação: weight 400 = regular mesmo com set explícito —
  intencional.

---

## Critério de conclusão

1. Inventário 139.A escrito (7 pontos).
2. Helper `faux_bold_stroke_pt` em `TextStyle`.
3. `FAUX_BOLD_K` constante definida.
4. Exporter emit `Tr` + `w` quando stroke > 0.
5. Graphics state resetado (`0 w` ou `q/Q`).
6. K calibrado contra teste visual.
7. 4 tests unit da fórmula + 1 test regressão 400 passam.
8. Teste manual: w700 visualmente bold, w400 idêntico a
   regular.
9. Canary preservado.
10. L1 tests: **869** (+5).
11. `cargo test --workspace` passa (≥ 1100).
12. `crystalline-lint` zero violations.
13. DEBT-52 gap 4 marcado resolvido. Fase B completa.
14. Relatório 139.K escrito.

---

## O que pode sair errado

- **`w` operator atravessa BT e afecta stroke de outros elementos
  (ex: rules, borders)**: detectado em A.5. Reset obrigatório.
  Se Opção A (`0 w`) não é suficiente, usar `q/Q`.

- **Fórmula K = 0.04 produz stroke demasiado fino em 700**:
  glyphs não parecem bold. Aumentar K. Calibrar visual.

- **Stroke demasiado grosso em 900**: glyphs parecem
  artificiais. Diminuir K ou alterar fórmula para não-linear
  (sqrt, por exemplo).

- **Helvetica base do PDF (F1) já é "Helvetica-Bold" e não
  regular**: improvável mas validar em A.1. Se sim, fórmula
  precisa inverter.

- **`TextStyle.size` é Pt directo ou struct wrapper**: ajustar
  `.val()` conforme tipo real.

- **Export actual já tem algum handling de bold via F2**: se
  F2 = Helvetica-Bold, consumer weight em exporter pode colidir.
  Gate A.1 detecta. Se colidir, decidir: remover lógica F2
  (só usar stroke) ou combinar (F2 para >=600 + stroke para
  fine-grading).

- **Weight 400 com `Some(400)` produz PDF diferente de
  `None`** porque emit condicional: `weight_stroke_pt > 0` é
  o gate, e 400 dá 0. Então `Some(400)` = `None` no output.
  Regressão OK — validada em 139.E.

---

## Notas operacionais

- **Terceiro consumer com efeito visível**. Fase B completa.
  Roadmap 135 validado empiricamente — Fases A e B executadas
  em 4 passos (136-139).

- **Faux-bold é aproximação consciente**. ADR-0054 perfil
  graded aceita. Quando embedding real chegar (Fase C /
  pós-DEBT-1), faux-bold pode ser removido ou mantido como
  fallback quando variante bold não está disponível — decisão
  futura.

- **K como constante é escolha pragmática**. Alternativa: K
  configurável por documento via propriedade nova. Não
  justifica complexidade sem consumer que peça.

- **Pattern exporter-assistido consolida**: 137 (Tc), 139
  (Tr + w). Ambos usam primitives PDF dedicados em vez de
  refactor de representação. Candidato para formalização:
  "usar primitives PDF apropriados sempre que a semântica case".

- **Gap 5-7 da Fase C são mais complicados**. Font envolve
  `FontBook::select` + embedding real. Lang envolve
  hifenização (crate externa). Cada um exige diagnóstico
  próprio. O candidato `eval_with_warnings` ganha urgência
  antes de Fase C para reduzir fricção acumulada nos
  harnesses de teste.

- **Ritmo estimado**: S. ≈1.5-2h. K calibration pode precisar
  iteração.

- **Canary `hyphenate`** continua a valer até ao fim da Fase C
  (quando lang ou outra propriedade capturar hyphenate).
