# Passo 138 — Consumer `leading` em layout (Fase B.2)

**Série**: 138 (passo **S** em L1; segundo consumer com efeito
visível — vertical).
**Precondição**: Passo 137 encerrado; 1092 total tests; zero
violations; 54 ADRs activas; 12 DEBTs abertos (DEBT-52 com 6
gaps). `TextStyle.leading` propagado até frame (Passo 136).
Tracking implementado em layouter + exporter (Passo 137).

**ADRs aplicáveis**:
- **ADR-0033** (paridade funcional).
- **ADR-0054** (critério fecho DEBT-1).
- **DEBT-52** fase B.2 — segundo consumer.

**Natureza**: passo L1. Consumer no layouter (eixo vertical).
**Exporter não é tocado** — frame carrega positions correctas.

---

## Contexto

Passo 137 fechou tracking (eixo horizontal). Este passo fecha
leading (eixo vertical). Semântica vanilla a confirmar no
inventário: `leading` pode ser "substituir default line-height",
"somar ao default", ou "definir gap entre linhas".

O efeito visível é: linhas consecutivas ficam mais afastadas
(ou mais próximas) verticalmente conforme o valor.

---

## Decisão estrutural confirmada

**Aplicar só no layouter.** Alinhado com princípio: frame
carrega positions correctas; exporter desenha o que recebe.

Implicação: o pipeline de line-break computa baseline da
linha N+1 somando `line_height` ao baseline da linha N.
`line_height` passa a depender de `style.leading` — fórmula
exacta a decidir em 138.A com base na semântica vanilla.

---

## Contexto estratégico

Fase B.2 do roadmap 135:

- **136** (feito): Fase A — extensão de TextStyle.
- **137** (feito): Fase B.1 — consumer tracking.
- **138** (este): Fase B.2 — consumer leading.
- **139**: Fase B.3 — consumer weight faux-bold.
- **140-143**: Fase C.
- **Fecho DEBT-1** após Fase A+B+C.

---

## Objectivo

Ao fim do passo:

1. **Semântica de leading documentada** no inventário (138.A),
   com referência a `ParElem::leading` vanilla e estado actual
   do código.
2. **Consumer aplicado no layouter**: baseline da linha N+1
   reflecte `leading` quando `Some`.
3. **Fórmula exacta** (substituir/somar/multiplicar) escolhida
   e registada.
4. **Exporter inalterado**: frame carrega positions finais.
5. **3 testes L1 numéricos**: comparação baseline entre linhas
   sem/com leading, caso edge (1 linha não tem inter-line
   spacing, valor de leading irrelevante), regressão (leading
   zero = positioning idêntico a pré-138).
6. **Teste manual visual**: PDF com linhas mais espaçadas
   observável.
7. Canary `hyphenate` preservado.

Este passo **não**:

- Adiciona consumer para outras propriedades (139+).
- Modifica exporter.
- Resolve font embedding.
- Fecha DEBT-52 (resolve gap 3 de 8).

---

## Decisões já tomadas

1. **Aplicar só no layouter**. Exporter não muda.
2. **Usar `Length::resolve_pt(size)`** — mesmo helper usado
   em 137 (paridade de fórmula entre propriedades).
3. **`leading = None` preserva comportamento actual**. Garante
   zero regressão.

## Decisões diferidas (138.A)

4. **Semântica exacta**:
   - **Opção subst**: `line_height = size + leading` (leading
     define o *gap*; substitui qualquer default).
   - **Opção soma**: `line_height = default_line_height + leading`
     (leading aumenta o default).
   - **Opção vanilla**: outra forma específica — a confirmar.

5. **Fórmula do default actual**: o código já calcula
   `line_height` de alguma forma (pode ser `size × 1.2` ou
   similar). Identificar antes de adicionar leading.

6. **Onde vive o cálculo**: função de line-break, de `Cursor`,
   de `flush_line`, ou outro. Depende de estrutura actual.

---

## Escopo

**Dentro**:
- `01_core/src/rules/layout/...` — onde line-break/positioning
  vertical vive (confirmar em 138.A).
- `01_core/src/rules/layout/tests.rs` — 3 testes numéricos.
- `00_nucleo/DEBT.md` — marcar gap 3 resolvido.

**Fora**:
- Exporter.
- Outras propriedades.
- L2, L3, L4.
- ADR nova.

---

## Sub-passos

### 138.A — Inventário confirmatório

**A.1 — Semântica vanilla de `par.leading`**:

Leitura de `lab/typst-original/crates/typst-library/src/model/par.rs`
(ou caminho real):

- `grep -n "pub leading" lab/typst-original/...`
- Localizar `ParElem.leading`.
- Registar:
  - Tipo exacto.
  - Default value.
  - Como é usado no layout (procurar referências em
    `layout/inline/...` vanilla).
  - Fórmula: `line_height = f(size, leading, top_edge, bottom_edge, ...)`.
- Documentar se o valor é **gap entre linhas** ou **altura
  total da linha**.

**A.2 — Estado actual do cálculo em L1**:

`grep -rn "line_height\|line.*advance\|baseline\|flush_line\|line_break" 01_core/src/rules/layout/`.

Registar:
- Função onde o incremento vertical é calculado.
- Fórmula actual (provavelmente `size × K` para algum K).
- Se há campo dedicado em `TextStyle` ou é derivado.

**A.3 — Aplicabilidade de `Length::resolve_pt`**:

Já confirmado no Passo 137 que o helper existe. Reaproveitar.

**A.4 — Pipeline de line-break**:

Como linhas são acumuladas no frame:
- Se é "cursor avança baseline a baseline", adicionar leading
  ao advance.
- Se é "lista de lines com heights explícitas", ajustar
  `line.height` antes de frame assembly.

**A.5 — Teste harness**:

Confirmar que `text_items_with_pos` (Passo 137) retorna `pos.y`
além de `pos.x`. Se só retorna `x`, estender para aceder a `y`.

**A.6 — Casos edge a considerar**:

- Primeira linha: não aplica leading (só entre linhas).
- Última linha: idem — leading é inter-linha.
- Documento de 1 linha: leading irrelevante.
- `leading = 0pt`: comportamento deve ser igual ao actual (sem
  leading set). Regressão OK.

**A.7 — Tests base**:
- L1: 861.
- Total: 1092.

**Gate 138.A**:

- Se A.1 revela que vanilla usa `leading` como **multiplicador
  de size** (ex: `line_height = size × (1 + leading_factor)`),
  a Opção soma/subst acima não se aplicam; fórmula é outra.
  Escolher conforme vanilla.

- Se A.2 revela que line_height actual é hardcoded (ex: `size × 1.2`)
  e não há lógica de leading: **criar** lógica. Pequeno mas
  real trabalho adicional.

- Se A.4 revela que line-break usa representação complexa
  (ex: glyph-level positions com BiDi runs), adicionar leading
  exige ponto específico — pode ser múltiplos sítios.
  Reportar antes de prosseguir.

- Outros casos: prosseguir.

### 138.B — Decidir fórmula

Com base em A.1 e A.2:

**Caso subst** (leading define gap, substitui default):

```rust
let line_gap = style.leading
    .map(|l| l.resolve_pt(style.size.val()))
    .unwrap_or_else(|| default_line_gap(style.size));
```

**Caso soma** (leading adiciona ao default):

```rust
let line_gap = default_line_gap(style.size)
    + style.leading
        .map(|l| l.resolve_pt(style.size.val()))
        .unwrap_or(0.0);
```

**Caso vanilla específico** (ex: `line_height = top_edge - bottom_edge + leading`):
espelhar literal.

Registar decisão e fórmula real no relatório.

### 138.C — Consumer em layouter

**Ficheiro**: conforme A.2/A.4.

Adicionar leitura de `style.leading` no ponto onde `line_height`
ou baseline-advance é calculado. Pseudo-código:

```rust
// pattern antes (exemplo):
fn flush_line(&mut self) {
    let line_height = self.style.size * 1.2;  // default
    self.cursor_y += line_height;
    // ...
}

// pattern depois (caso soma):
fn flush_line(&mut self) {
    let default_gap = self.style.size.val() * 1.2;
    let leading_pt = self.style.leading
        .map(|l| l.resolve_pt(self.style.size.val()))
        .unwrap_or(0.0);
    let line_height = Pt(default_gap + leading_pt);
    self.cursor_y += line_height;
    // ...
}
```

Adaptar conforme código real. Se a lógica é mais intricada
(paragraph break vs line wrap), aplicar nos sítios apropriados.

### 138.D — Testes L1 novos (3)

**Ficheiro**: `01_core/src/rules/layout/tests.rs`.

```rust
#[test]
fn layout_leading_afecta_posicao_linha_seguinte_passo_138() {
    // Compara baseline da linha 2 entre sem e com leading.
    let sem = layout_typst("linha1\nlinha2");
    let com = layout_typst(
        "#set par(leading: 10pt)\nlinha1\nlinha2"
    );

    let sem_items = text_items_with_pos(&sem);
    let com_items = text_items_with_pos(&com);

    // Linha 2 em `com` está mais abaixo (y maior em PDF coord)
    // do que em `sem` por aproximadamente 10pt.
    let y_linha2_sem = sem_items[1].pos.y;
    let y_linha2_com = com_items[1].pos.y;

    // PDF y cresce para cima; linha 2 abaixo da 1 tem y menor.
    // "Mais afastadas" = diferença absoluta maior.
    let gap_sem = (sem_items[0].pos.y - y_linha2_sem).abs();
    let gap_com = (com_items[0].pos.y - y_linha2_com).abs();

    assert!(gap_com > gap_sem,
        "gap com leading ({}) devia ser maior que sem ({})",
        gap_com, gap_sem);
}

#[test]
fn layout_leading_nao_afecta_documento_de_uma_linha_passo_138() {
    // Leading define espaçamento inter-linha. Com 1 linha,
    // valor irrelevante para output.
    let sem = layout_typst("uma linha");
    let com = layout_typst(
        "#set par(leading: 10pt)\numa linha"
    );

    let sem_items = text_items_with_pos(&sem);
    let com_items = text_items_with_pos(&com);

    assert_eq!(sem_items.len(), 1);
    assert_eq!(com_items.len(), 1);

    // Baseline idêntica — leading sem efeito com 1 linha.
    assert!((sem_items[0].pos.y - com_items[0].pos.y).abs() < 0.01);
}

#[test]
fn layout_leading_zero_preserva_comportamento_base_passo_138() {
    // Regressão: leading = 0pt é equivalente a sem set.
    let sem = layout_typst("linha1\nlinha2");
    let com = layout_typst(
        "#set par(leading: 0pt)\nlinha1\nlinha2"
    );

    let sem_items = text_items_with_pos(&sem);
    let com_items = text_items_with_pos(&com);

    for (s, c) in sem_items.iter().zip(com_items.iter()) {
        assert!((s.pos.y - c.pos.y).abs() < 0.01,
            "leading 0pt deve ser igual a sem set");
    }
}
```

**Nota**: o terceiro teste só é válido se a semântica for
**soma** (leading soma ao default, logo `leading = 0` = sem
leading). Se a semântica for **subst** (leading substitui
default), `leading = 0pt` colapsa linhas para zero gap — teste
precisa de adaptação. Decidir no relatório 138.B.

### 138.E — Teste manual visual

```bash
$ cat sem.typ
linha um
linha dois
linha três

$ typst sem.typ -o sem.pdf

$ cat com.typ
#set par(leading: 10pt)
linha um
linha dois
linha três

$ typst com.typ -o com.pdf

$ # Abrir ambos; "com.pdf" tem linhas visivelmente mais afastadas.
```

Critério: espaçamento vertical entre linhas maior em `com.pdf`.
Registar no relatório.

### 138.F — Canary e regressão

- Canary `eval_set_text_hyphenate_canary_passo_132b` passa.
- Testes existentes de layout passam.
- Tracking (Passo 137) continua funcional — não há conflito
  entre horizontal e vertical.

### 138.G — DEBT-52 actualizar

```markdown
- [x] Gap 1 (Fase A). **Passo 136**.
- [x] Gap 2 (Fase B.1): tracking. **Passo 137**.
- [x] Gap 3 (Fase B.2): leading. **Passo 138**.
- [ ] Gap 4 (Fase B.3): weight faux-bold.
- [ ] ...
```

5 gaps restantes.

### 138.H — Verificação

1. `cargo test -p typst-core` — L1: 861 → **864** (+3).

2. `cargo test --workspace` — total ≥ 1095.

3. `crystalline-lint` zero violations.

4. Manual:

```bash
$ typst sem.typ                              # 3 linhas normais
exit=0, PDF compacto

$ typst com.typ                              # 3 linhas com leading 10pt
exit=0, PDF com mais espaço vertical

$ typst com-tracking-leading.typ             # ambos combinados
exit=0, PDF com letras espaçadas E linhas espaçadas

$ typst h.typ                                # canary
h.typ:1:11: warning: ... 'hyphenate' ...
exit=0
```

### 138.I — Encerramento

Relatório `typst-passo-138-relatorio.md`:

- Inventário 138.A (especial A.1 e A.2 — semântica vanilla +
  estado actual).
- Fórmula escolhida (subst/soma/outra).
- Diff por ficheiro.
- Resultado do teste manual.
- Números finais.
- Observação sobre tracking + leading combinados.
- Preparação para 139 (weight faux-bold).

---

## Critério de conclusão

1. Inventário 138.A escrito (7 pontos, especial A.1 e A.2).
2. Fórmula exacta escolhida e registada.
3. Consumer aplicado no layouter (exporter inalterado).
4. 3 testes L1 passam (efeito inter-linha, 1-linha, regressão
   com leading 0).
5. Teste manual confirma diferença visível em 3-linha PDF.
6. Canary preservado.
7. L1 tests: **864** (+3).
8. `cargo test --workspace` passa (≥ 1095).
9. `crystalline-lint` zero violations.
10. DEBT-52 gap 3 marcado resolvido.
11. Relatório 138.I escrito.

---

## O que pode sair errado

- **Default line-height não é `size × constante` mas calculado
  via top_edge/bottom_edge** (como vanilla pode fazer): fórmula
  mais complexa. Se detectado, seguir vanilla literal.

- **`par.leading` não é propriedade simples mas default
  parameterizado** (ex: relative factor): tipo em
  `StyleDelta.leading` pode precisar de revisão. Esperado: não,
  porque captura foi `Option<Length>`.

- **Line-break tem representação não-trivial** (ex: paragraph
  shaping com BiDi): aplicar leading no sítio certo pode ser
  mais intricado. Reportar antes de prosseguir se detectado.

- **Teste "leading = 0pt" falha** porque semântica é **subst**
  (não **soma**): colapsa linhas, diferente do comportamento
  sem set. Adaptar o teste para reflectir a semântica real —
  o teste em si é correcto, só a comparação muda.

- **Exporter precisa de alteração afinal** (ex: hardcoded
  line-height em `Td`): gate 138.A.2/A.4 detecta. Se acontece,
  decisão "só layouter" foi optimista — ajustar.

- **`text_items_with_pos` não expõe `y`**: estender helper
  (trivial).

- **Tracking e leading combinados causam regressão em algum
  teste existente**: improvável mas possível. Executar suite
  completa antes de fechar.

---

## Notas operacionais

- **Segundo consumer com efeito visível** da série. Se sai
  limpo, valida que Fase B é real e o roadmap 135 é executável.

- **Eixo vertical vs horizontal**: tracking (137) foi
  horizontal com primitive PDF dedicado. Leading é vertical
  sem primitive PDF — layouter computa baseline directamente.
  Mais "clean" arquitecturalmente porque não envolve exporter.

- **Semântica de leading tem história**: 128 capturou leading
  em `text` por divergência aceite. 133 activou `par`. 134
  migrou para `par`. 138 dá-lhe finalmente efeito observável.
  Trajectória: 4 passos ao longo desta série só para esta
  propriedade. Documentar no relatório como caso de estudo
  de "ADR-0033 lido literal".

- **Canary continua a valer**: `hyphenate` unknown property
  não é afectado por nenhum consumer até Fase C. Preservação
  é trivial mas importante.

- **Candidato registado 137**: "line-wrapping regression test
  com tracking". Continua pendente. Leading pode criar
  oportunidade de teste combinado (tracking + leading + line
  wrap em coluna apertada) — vale considerar quando infra
  suporta.

- **Ritmo estimado**: S. ≈1-2h. Similar a 137 em tempo mas
  mais simples em escopo (só layouter).

- **`eval_with_warnings`** continua pendente. Cada passo de
  consumer adiciona mais 3 testes; fricção acumula.
