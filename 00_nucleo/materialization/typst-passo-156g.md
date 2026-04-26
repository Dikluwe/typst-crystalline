# Passo 156G — block (Layout Fase 2 sub-passo 1)

**Série**: 156G (passo **substantivo escopo M+ esperado**;
materialização Fase 2 Layout, primeira sub-fase). **Quinto
passo consecutivo** da sequência granular Layout
(P156C+D+E+F+G).
**Padrão**: inventariação-primeiro pré-decisão arquitectural
(formalizado por P156F §15).

**Precondição**: Passo 156F encerrado; ADR-0061 PROPOSTO
(Layout roadmap; mantém PROPOSTO até P156I per decisão
humana); 1230 tests; 61 ADRs; 14 DEBTs abertos; cobertura
Layout 56% (10/18 implementado puro pós-P156F); cobertura
user-facing total 57%.

**Numeração**: P156G segue P156F na convenção de letras
consecutivas. **Não conflita** com P157 (Model Fase 2 table
foundations).

**Natureza**: passo **substantivo escopo M+** (1 feature
container rico com 8+ atributos; **primeira feature de Fase 2
Layout — escopo qualitativamente diferente de P156C-F que
eram Fase 1**; ~15-25 testes adicionados estimados; sem
crates novas; sem ADRs novas; sem DEBTs novos esperados).

**Decisão metodológica P156G** (per resposta humana
2026-04-25): **inventariar primeiro; decidir
arquitectura consoante descoberta empírica**. Spec
**não compromete** com forma final de `Content::Block`.
Apresenta 4 hipóteses; sub-passo 156G.2 escolhe consoante
156G.1.

Razão: P156F validou padrão "spec antecipa; inventário
decide" — descoberta de TransformMatrix já unificada desde
P78 reverteu refactor planeado em aditivo. P156G é primeiro
container rico de Fase 2; **decisão arquitectural não-trivial
que afecta P156H (box) e P156I (stack)**. Inventariar
primeiro reduz risco de comprometer-se com forma errada.

**ADRs aplicáveis**:
- **ADR-0026 + ADR-0026-R1**: Content enum aceita variants
  novos OU `Content::Styled` para atributos via Style. Decisão
  per 156G.2.
- **ADR-0033**: paridade funcional para block.
- **ADR-0036**: atomização — consumer explícito.
- **ADR-0037**: coesão por domínio — Layout permanece em
  `rules/layout/` e `rules/stdlib/layout.rs`.
- **ADR-0054**: perfil observacional graded — block cumprido
  com aproximação aceite (atributos avançados como
  `breakable`, `fill`, `stroke` podem ficar scope-out se
  layouter actual não os suporta).
- **ADR-0061** (PROPOSTO): plano de Layout Fase X. Este
  passo aplica-o pela quinta vez; **primeira aplicação a
  Fase 2 (containers ricos)**.

---

## Contexto

P156F fechou skew (56% cobertura Layout). Próximo natural
é Fase 2 Layout (containers ricos: block, box, stack).
P156G ataca **block** primeiro.

**`block(body, ...)` em vanilla**:
- Body posicional opcional.
- Atributos numerosos (8+):
  - `width: Smart<Rel<Length>>` (largura).
  - `height: Sizing` (altura: Auto, Length, Fr).
  - `breakable: bool` (quebra entre páginas).
  - `inset: Sides<Rel<Length>>` (margem interna).
  - `outset: Sides<Rel<Length>>` (margem externa).
  - `fill: Option<Color>` (cor de fundo).
  - `stroke: Sides<Option<Stroke>>` (bordas).
  - `radius: Corners<Rel<Length>>` (cantos arredondados).
  - `clip: bool` (clipping).
  - `spacing: Length` (entre blocks).
  - `above`/`below: Smart<Length>` (espaços antes/depois).
  - `sticky: bool` (cola com próximo block).

- Semantic: container que ocupa largura completa;
  comportamento de quebra por defeito; pode ser estilizado.
- `#show block: ...` aceita customização.

**Hipóteses arquitecturais (4 opções a decidir empiricamente
em 156G.2)**:

**Opção A — Variant rico**: `Content::Block { body, width,
height, breakable, inset, fill, stroke, radius, clip, ... }`.
Todos os atributos como fields da variant. Símbolo: 12+
fields. Tests proporcionais.

**Opção B — Content::Styled per ADR-0026**: atributos como
entries em Style cascade. Variant minimalista
`Content::Block { body }` (ou nem variant — só `Style::Block`
marker). Reusa Style infra. Risco: alguns atributos
(breakable, sticky) podem não encaixar no modelo Style.

**Opção C — Variant minimalista + struct atributos**:
`Content::Block { body, attrs: BlockAttrs }` onde
`BlockAttrs` é struct dedicada com defaults. Análogo a
opção C de P156F que se descartou. Compromisso: variant
fino + atributos isolados em struct própria.

**Opção D — Variant + Style híbrido**: `Content::Block
{ body }` + Style entries para atributos (ex: `Style::BlockSize`,
`Style::BlockFill`). Variant minimalista marca o "block-ness";
Style cascade gere atributos. Padrão emergente em outros
elementos do projecto?

**Hipóteses a confirmar empiricamente em 156G.1** (não
compromisso):

- Estado actual cristalino para "block-like" (algo existe?).
- Se `Content::Styled` já cobre parcialmente (e.g. Strong/
  Emph usam para apply Style cascade).
- Quais atributos vanilla são layout-relevant em cristalino
  actual (e.g. `fill`/`stroke` exigem renderer support).
- Forma de `Sides<Rel<Length>>` vs `Sides<Length>` — Rel
  precisa de tipo `Rel<T>` que pode não existir.
- Forma de `Sizing` (Auto/Length/Fr) — Fr (fraction) não
  está suportado per P156D scope-out.

---

## Objectivo

Ao fim do passo:

1. **Inventário rigoroso** em 156G.1 do estado actual e
   das 4 hipóteses arquitecturais.

2. **Decisão arquitectural** em 156G.2 (escolha entre A/B/C/D
   ou variante derivada) com **justificação empírica**.

3. **Forma final de Block** materializada conforme decisão
   de 156G.2.

4. **Atributos cobertos**: subset realista per ADR-0054
   graded. Provável:
   - **Cobertura Fase 1 (este passo)**: body, width, height,
     inset, breakable.
   - **Scope-out (refino futuro)**: fill, stroke, radius,
     clip, spacing, above/below, sticky, outset.
   - Decisão final em 156G.2 baseada em viabilidade
     técnica (renderer support).

5. **`native_block`** em
   `01_core/src/rules/stdlib/layout.rs` expondo
   `#block(body, width: ?, height: ?, inset: ?, breakable: ?, ...)`.

6. **Cobertura exaustiva de arms** consoante decisão
   arquitectural. Se variant novo → 9 sítios. Se Styled →
   menor.

7. **Layouter block**: suporta width/height/inset; força
   nova linha se necessário; respeita breakable.

8. **Testes** unit + eval (~15-25 testes adicionados
   estimados — depende de quantos atributos forem
   implementados).

9. **L0 prompts** + hashes propagados.

10. **Inventário 148 actualizado**:
    - Tabela A.5 Layout: linha `block` ausente →
      `implementado` (com possível anotação sobre
      atributos parciais per ADR-0054 graded).
    - Cobertura Layout: 10/18 → **11/18 = 61%**.
    - Tabela A linha "Layout": `10/0/3/5/0=18` →
      `11/0/3/4/0=18`.
    - Total user-facing: 57% → **~58%**.
    - Tabela B Content variants: 48 → **49** (se variant
      novo) OU **48** (se Styled).

11. **README dos ADRs actualizado**.

12. **ADR-0061 NÃO actualizada** (per decisão humana).

13. **Sem DEBTs criados/fechados** (esperado; possível
    DEBT se algum atributo descoberto-em-falta for
    bloqueante — improvável).

14. **Relatório do passo** em
    `00_nucleo/materialization/typst-passo-156g-relatorio.md`
    incluindo §análise de risco análoga a P156F.

Este passo **não**:

- Toca outras features Layout além de block.
- Implementa atributos complexos sem suporte renderer
  (fill, stroke, radius — scope-out per ADR-0054).
- Implementa show rules.
- Toca série paridade.
- Modifica ADR-0061.
- Implementa column flow.

---

## Decisões já tomadas

1. **Inventariação primeiro** per padrão P156F. Decisão
   arquitectural diferida para 156G.2.

2. **Granularidade**: 1 container rico num passo. Escopo
   M+ vs S/S+ dos passos anteriores.

3. **Localização canónica**: `01_core/src/rules/stdlib/layout.rs`.

4. **Assinatura natives**: 5-param canónica.

5. **Atributos avançados (fill/stroke/radius/clip) scope-out**
   per ADR-0054 graded. Refino futuro.

6. **Tests adicionados**: alvo 15-25 (ajustável conforme
   atributos implementados).

7. **ADR-0061 NÃO anotada**.

8. **Show rules adiadas**.

9. **Risco de regressão potencial > 0** se decisão for
   modificar `Content::Styled` infra existente (opção B).
   Mitigação: regression tests análogos a P156F.

## Decisões diferidas (resolvidas em 156G.2 baseado em
156G.1)

10. **Forma de Block** (variant vs Styled vs híbrido):
    decidida empiricamente.

11. **Quais atributos implementar agora** (per ADR-0054
    graded):
    - Mínimo viável: body, width, height, inset, breakable.
    - Opcional: outros consoante viabilidade.
    - Decisão consoante código actual + tempo disponível.

12. **`Rel<Length>` para width/inset**: vanilla usa
    `Rel<Length>` (relativo: `50%` ou `Length`). Cristalino
    pode não ter `Rel<T>`. Decisão default: usar `Length`
    apenas; se relativo necessário, converter via
    page_config; se exigir mais, adiar.

13. **`Sizing` (Auto/Length/Fr) para height**: cristalino
    pode não ter Sizing tipo. Decisão default: usar
    `Option<Length>` (None = auto; Some(L) = explícito).
    Fr scope-out.

14. **`breakable: true` semantic real**: layouter actual
    pode não ter mecânica de "block atómico" (não-quebra).
    Decisão default: armazenar como atributo; comportamento
    real defere se exige refactor.

15. **`fill` (cor de fundo)**: requer renderer support para
    rectangle fill. Verificar; se ausente, scope-out.

16. **`stroke` (bordas)**: idem; scope-out provável.

17. **Conflito naming `block`**: `block` é palavra-chave
    Rust em alguns contextos. Em stdlib é função normal.
    Construtor Rust pode precisar `block_content` ou
    similar.

---

## Escopo

**Dentro**:

- Modificação de `01_core/src/entities/content.rs`
  (variant novo OU adições em Style — consoante 156G.2).
- Possível modificação de `01_core/src/entities/style.rs`
  (se opção B/D — Style entries novas).
- Possível criação de `01_core/src/entities/block_attrs.rs`
  (se opção C).
- Modificação de `01_core/src/rules/introspect.rs`.
- Modificação de `01_core/src/rules/layout/mod.rs`.
- Modificação de `01_core/src/rules/stdlib/layout.rs`
  (`native_block`).
- Modificação de `01_core/src/rules/stdlib/mod.rs`
  (re-export).
- Modificação de `01_core/src/rules/eval/mod.rs`
  (registo).
- Tests novos.
- L0 prompts + hashes.
- Inventário 148 + README ADRs.
- Relatório do passo (incluindo §análise de risco).

**Fora**:

- Implementação de outras features Layout.
- Implementação de atributos avançados (fill/stroke/radius/
  clip) — scope-out per ADR-0054.
- Show rules.
- Crates externas.
- ADRs novas.
- DEBTs novos (excepto se descoberta empírica justificar).
- Modificação de ADR-0061.
- Modificação de ADR-0060.
- Trabalho em `lab/parity/`.

---

## Sub-passos

### 156G.1 — Inventário rigoroso pré-decisão arquitectural

**Crítico** para este passo (modelo P156F.1 expandido).

**A.1.1 — Estado actual cristalino para "block-like"**:

```bash
view 01_core/src/entities/content.rs   # confirmar 48 variants pós-P156F
grep -nE "Block|block" 01_core/src/entities/content.rs
grep -nE "fn native_block" 01_core/src/rules/stdlib/
view 01_core/src/entities/style.rs   # listar Style variants
grep -nE "^pub enum Style\b" 01_core/src/entities/style.rs
```

Confirmar:
- `Content::Block` existe? Provável: não.
- `Style::Block` ou similar existe? Provável: não.
- Algum padrão emergente para "container com atributos
  ricos" (ex: como Strong/Emph fazem)?

**A.1.2 — Padrão `Content::Styled`**:

```bash
grep -nE "Content::Styled" 01_core/src/
view 01_core/src/entities/content.rs   # secção Styled
view 01_core/src/entities/style.rs   # variants Style
```

Documentar:
- Forma actual de `Content::Styled`.
- Como Strong/Emph (per P22) usam Style.
- Quais atributos vanilla podem encaixar em Style cascade
  vs quais precisam variant dedicado.

**A.1.3 — Tipos relativos**:

```bash
grep -nE "pub struct Rel|pub enum Sizing" 01_core/src/entities/
ls 01_core/src/entities/sizing.rs 2>/dev/null
```

Confirmar:
- `Rel<T>` existe?
- `Sizing` existe?
- `Sides<Rel<Length>>` é construível?

**A.1.4 — Renderer support para atributos avançados**:

```bash
grep -nE "fill|stroke|radius" 01_core/src/rules/layout/
grep -nE "fn render|emit" 03_infra/src/export.rs 2>/dev/null
```

Confirmar:
- Layouter cristalino emite rectangle fills?
- PDF export suporta cor de fundo?
- Stroke (bordas) suportado?

**A.1.5 — Layouter actual: como blocks-like se comportam**:

```bash
view 01_core/src/rules/layout/mod.rs   # arms para Strong/Emph/Heading
grep -nE "force_line_break|new_line|cursor_y" 01_core/src/rules/layout/
```

Confirmar:
- Strong/Emph emitem linha nova?
- Heading existe e emite "bloco" próprio?
- Mecânica actual de "container que ocupa linha completa"?

### 156G.2 — Decisão arquitectural

Consoante 156G.1, escolher uma das 4 hipóteses + ajustes:

**Critérios para escolha**:

| Critério | Opção A (variant rico) | Opção B (Styled) | Opção C (variant + struct) | Opção D (híbrido) |
|----------|------------------------|------------------|----------------------------|-------------------|
| Coerência com vanilla | alta | média | alta | média |
| Reuso de infra | baixo | **alto** | médio | médio |
| Flexibilidade futura (mais atributos) | baixa (mudar variant) | **alta** (Style entry novo) | média | alta |
| Tests proporcionais | muitos | poucos | médios | médios |
| Risco de regressão | baixo (aditivo) | **médio** (toca Style) | baixo | **médio** (toca Style) |
| Decisão para box+stack | repete | repete | repete | requer outro passo decisão |

**Default sugerido (a ajustar consoante 156G.1)**:

- Se Style cascade já usa para Strong/Emph com sucesso e
  block-like atributos encaixam: **opção B**.
- Se atributos são tão diversos que Style fica artificial:
  **opção C** (variant + struct).
- Se decisão é difícil ou descoberta revela hipótese não
  prevista: **pausa-e-consulta** humano.

### 156G.3 — Aplicar decisão arquitectural

Materialização específica consoante 156G.2:

**Se opção B (Content::Styled + Style entries)**:

```rust
// Style enum estendido
pub enum Style {
    // ... existentes
    BlockWidth(Length),
    BlockHeight(Option<Length>),
    BlockInset(Sides<Length>),
    BlockBreakable(bool),
}

// native_block constrói Content::Styled wrapping body
pub fn native_block(...) -> SourceResult<Value> {
    // extrai atributos
    let body = ...;
    let mut styles = vec![];
    if let Some(w) = width { styles.push(Style::BlockWidth(w)); }
    // ...
    Ok(Value::Content(Content::Styled {
        body: Box::new(body),
        styles,
    }))
}
```

**Se opção C (variant + BlockAttrs struct)**:

```rust
pub struct BlockAttrs {
    pub width:     Option<Length>,
    pub height:    Option<Length>,
    pub inset:     Sides<Length>,
    pub breakable: bool,
    // ... (com defaults)
}

// Content enum
Block {
    body:  Box<Content>,
    attrs: BlockAttrs,
},
```

**Se opção A (variant rico)**:

```rust
Block {
    body:      Box<Content>,
    width:     Option<Length>,
    height:    Option<Length>,
    inset:     Sides<Length>,
    breakable: bool,
},
```

### 156G.4 — Cobertura exaustiva de arms

Consoante decisão. Se variant novo, ~9 arms tocados.
Se Styled, arms existentes para Styled cobrem.

### 156G.5 — `native_block`

Atributos extraídos consoante decisão arquitectural.
Helpers reusados: `extract_length` (P156C). Helper
candidato a criar: `extract_sides` (para inset; aceita
`auto`, número uniforme, ou dict `{ left, right, top, bottom }`).

### 156G.6 — Layouter block

Consoante decisão. Comportamento mínimo:
- Força nova linha antes (block ocupa largura completa).
- Aplica width: limita largura disponível para body.
- Aplica inset: reduz área disponível (análogo a pad
  per P156C).
- Aplica height: se Some, reserva pelo menos esse
  espaço; se None, calcula a partir do body.
- breakable: se false, layouter tenta evitar quebra mid-
  block (semantic real defere se exige refactor).

### 156G.7 — Tests adicionados (alvo 15-25)

Distribuição dependente de decisão:
- Construtor (1-3 tests).
- native_block defaults (1).
- native_block cada atributo (~5-8).
- native_block edge cases (3-5).
- Layout E2E (2-4).
- **Regression tests** se opção B/D modifica Style infra (3+).

Tests cumulativos: **1230 → ~1245-1255**.

### 156G.8 — L0 prompts + hashes

Consoante decisão arquitectural:
- Se variant novo: secção em `entities/content.md`.
- Se Style entries: secção em `entities/style.md`.
- Se BlockAttrs struct: ficheiro novo `entities/block_attrs.md`.

Recomputar hashes via `crystalline-lint --fix-hashes`.

### 156G.9 — Inventário 148 actualizado

**Tabela A.5 Layout**: `block` ausente → **implementado**
(possível anotação "atributos parciais per ADR-0054
graded").

**Tabela A linha "Layout"**: `10/0/3/5/0=18` →
`11/0/3/4/0=18`.

Cobertura Layout: 56% → **61%**.

**Tabela B Content variants**: 48 → **49** (se variant)
OU **48** (se Styled).

**Total user-facing**: 57% → **~58%**.

### 156G.10 — README ADRs actualizado

Entrada nova "Passos-chave" para P156G com:
- Decisão arquitectural escolhida + justificação empírica.
- Atributos implementados vs scope-out.
- Tests cumulativos.
- Cobertura.

### 156G.11 — Relatório do passo

Modelo P156F com §análise de risco. Secções:
1. Sumário executivo.
2. Inventário 156G.1 (resumo expandido).
3. Decisão arquitectural escolhida + justificação.
4. Forma final implementada.
5. Cobertura arms.
6. native_block + helpers.
7. Layouter diff.
8. Tests adicionados.
9. L0 prompts + hashes.
10. Inventário 148.
11. README ADRs.
12. Próximo passo (P156H = box).
13. Limitações registadas.
14. Verificação final.
15. **Análise de risco de regressão** (modelo P156F §15).

---

## Verificação

1. ✅ `cargo build --workspace`: clean.
2. ✅ `cargo test --workspace --lib`: ~1245-1255 passed.
3. ✅ `crystalline-lint .`: zero violations.
4. ✅ Hashes propagados consistentes.
5. ✅ `Content::Block` (ou equivalente) em produção.
6. ✅ Stdlib `#block(...)` invocável (38 → 39 funcs).
7. ✅ Cobertura arms exaustiva consoante decisão.
8. ✅ Inventário 148 reflecte cobertura aumentada (56% →
   61%).
9. ✅ README ADRs entrada P156G.
10. ✅ Sem ADR criada / revogada / revisada.
11. ✅ Sem DEBT criado / fechado (ou justificação se algum
    aberto).
12. ✅ ADR-0061 inalterada.
13. ✅ ADR-0060 inalterada.
14. ✅ **Sem regressão** em tests existentes (especial
    atenção a Strong/Emph se opção B/D toca Style).
15. ✅ Sem regressão geral.
16. ✅ Relatório do passo escrito (incluindo §análise de
    risco).

---

## Critério de conclusão

| # | Critério | Estado |
|---|----------|--------|
| 1 | Inventário 156G.1 produzido | ✅ |
| 2 | Decisão arquitectural justificada empiricamente | ✅ |
| 3 | Forma final compila + tests passam | ✅ |
| 4 | Stdlib `#block(...)` invocável com atributos mínimos | ✅ |
| 5 | Layouter block aplica width/inset/breakable | ✅ |
| 6 | Atributos scope-out documentados em limitações | ✅ |
| 7 | Inventário 148 reflecte cobertura 61% Layout | ✅ |
| 8 | Próximo passo (156H = box) tem âncora | ✅ |
| 9 | Sem regressão | ✅ |
| 10 | §análise de risco no relatório | ✅ |
| 11 | Relatório do passo escrito | ✅ |

---

## O que pode sair errado

### Cenários gerais (independentes de decisão arquitectural)

- **Inventário 156G.1 revela que Block parcial existe**:
  align/move/rotate/scale são "implementados via Content::
  Transform unificado" (per inventário 148). Pode haver
  algo análogo para block-like. Verificar; se sim,
  simplificar plano (padrão P156F).

- **Atributos scope-out crescem mais que previsto**: se
  layouter actual não suporta width/inset (improvável
  dado P156C pad existir), refactor exige passo extra.
  Decisão default: registar limitação; cobertura mínima
  ainda atinge "implementado" per ADR-0054.

- **Decisão arquitectural difícil**: se 156G.1 não dá
  pista clara, **pausar e consultar humano**. P156F
  estabeleceu precedente.

- **Volume tests excede 25**: aceitável; ajustar relatório.

- **Volume tests inferior a 15**: investigar; block tem
  múltiplos atributos; cobertura deve ser substancial.

### Cenários por opção arquitectural

**Se opção A (variant rico)**:
- Variant gordo (12+ fields) é antipadrão. Mitigação:
  manter só atributos implementados (5-6 fields).
- Tests cobertura proporcional a fields.

**Se opção B (Content::Styled)**:
- Style infra pode ter contagem máxima de entries (improvável
  mas verificar).
- Strong/Emph devem continuar a funcionar — **regression
  tests críticos**.
- Style cascade ordering: novos entries não interferem
  com existentes (verificar).

**Se opção C (variant + BlockAttrs)**:
- BlockAttrs struct precisa Default trait + Clone + PartialEq.
- BlockAttrs específica de Block ou genérica para outros
  containers (box, stack)? Decisão local.

**Se opção D (variant + Style híbrido)**:
- Risco de redundância entre variant fields e Style entries.
- Confusão sobre qual prevalece em conflito.

### Cenários específicos a Fase 2

- **Block é primeiro container Fase 2**: decisão afecta
  P156H (box) e P156I (stack). Se block escolhe opção A,
  box e stack provavelmente repetem padrão. Inventariar
  P156G com isso em mente.

- **Slope esperado +5-6%**: per projecção P156F. Se
  cobertura final < 60% (vs target 61%), pode ser sinal
  de scope-out maior que esperado. Aceitável.

---

## Notas operacionais

- **Padrão "passos granulares" — quinta aplicação
  consecutiva**. P156C+D+E+F+G. P156G é primeiro Fase 2.
  Hipótese da granularidade testada com escopo M+ pela
  primeira vez.

- **Padrão "inventariar primeiro" formalizado**: P156G é
  primeira aplicação explícita pós-P156F. Em P156F a
  inventariação foi defensiva (esperar refactor); em P156G
  é deliberada (decidir arquitectura). Refinamento
  metodológico.

- **Decisão arquitectural impacta P156H/I**: se block escolhe
  modelo X, box e stack provavelmente seguem mesmo modelo.
  Considerar ao decidir em 156G.2.

- **ADR-0061 mantém PROPOSTO**: per decisão humana.
  Anotação cumulativa após P156I.

- **Variants count**: 48 → **49 OU 48** (consoante decisão
  arquitectural). Após P156I: 50-52.

- **Stdlib funcs**: 38 → **39** (+1).

- **Pós-156G**:
  - 8 features Layout implementadas total.
  - Cobertura Layout: 56% → **61%**.
  - Cobertura user-facing total: 57% → ~58%.
  - **Próximo**: P156H (box) ou alternativa humana.

- **Slope Fase 2 espera-se ~+5-6% por passo**:
  - P156G (block): +5%.
  - P156H (box): +5-6%.
  - P156I (stack): +5-6%.
  - **Total Fase 2**: 56% → ~72% target em 3 passos.

- **Hipótese metodológica em refinamento**: "inventariação
  rigorosa pré-código mantém passos granulares aditivos".
  P156F validou retroactivamente; P156G valida
  prospectivamente. Se P156G mantém slope esperado +5-6%
  e zero regressão, padrão consolida.

- **Quarentena vanilla**: continua opção 3.

- **Série paridade**: continua suspensa em P153.

- **Hash do código `entities/content.rs`** (após P156F):
  `4321258d`. Após P156G: novo (consoante decisão).

- **Padrão emergente "Smart<T> simplificado para Option<T>"**:
  P156E (Smart<Parity> → Option<Parity>) e P156F implícito
  (default 0 para ângulos). Block tem `Smart<Rel<Length>>`
  para width — mais um caso candidato a simplificação.
  Consistência arquitectural se mantida.
