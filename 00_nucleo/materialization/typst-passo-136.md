# Passo 136 — Estender `TextStyle` com 5 campos (Fase A)

**Série**: 136 (passo **XS** em L1; Fase A de 5 do roadmap
revisto em 135 para fechar DEBT-1).
**Precondição**: Passo 135 encerrado; 1084 total tests; zero
violations; 54 ADRs activas (ADR-0054 `EM VIGOR`); 12 DEBTs
abertos (DEBT-52 rastreador de gaps de consumer).
**ADRs aplicáveis**:
- **ADR-0033** (paridade funcional).
- **ADR-0054** (critério fecho DEBT-1 inclui consumo integral).
- **DEBT-1** (fecho depende de Fases A+B+C).
- **DEBT-52** (gap 1 de 8: extensão de `TextStyle`).

**Natureza**: passo L1. **Extensão de struct + propagação**.
Nenhum efeito observável no PDF. Infraestrutura pura para
preparar Fase B (consumers).

---

## Contexto

Passo 135 revelou que 5 de 10 campos de `StyleDelta` são
inertes em layout: `weight, tracking, leading, lang, font`.
O bloqueio físico é `TextStyle` (5 campos apenas) + `From<&StyleChain>`
(mapeia apenas esses 5). Propriedades capturadas em `StyleDelta`
não atravessam a ponte para o frame.

Este passo estende `TextStyle` com os 5 campos em falta e
propaga-os via `From<&StyleChain>`. Após 136, as propriedades
atravessam a ponte mas não têm consumer — é pré-requisito para
137+ (consumers).

**Decisão de lote**: 5 campos de uma vez. Razão: cada campo
individual seria XS, mas o pattern de extensão é idêntico para
os 5 — multiplicar enunciados seria inflar sem ganho. Se um
campo tem forma anómala, inventário detecta.

---

## Contexto estratégico

Fase A do roadmap revisto (135):

- **136** (este): Fase A — estender `TextStyle` + propagar.
- **137–139**: Fase B — consumers simples (tracking, leading,
  weight faux-bold).
- **140–143**: Fase C — consumers avançados (font, lang,
  embedding PDF).
- Fases D, E: opcionais, podem ficar fora de DEBT-1.
- **Fecho DEBT-1**: após Fase A+B+C (estimativa: 10-16h total).

---

## Objectivo

Ao fim do passo:

1. `TextStyle` tem 10 campos (5 actuais + 5 novos):
   - `weight: Option<u16>` (ou tipo equivalente ao `StyleDelta`).
   - `tracking: Option<Length>`.
   - `leading: Option<Length>`.
   - `lang: Option<Lang>`.
   - `font: Option<FontList>`.
2. `From<&StyleChain>` (ou equivalente) propaga os 5 novos
   campos de `StyleDelta` → `TextStyle`.
3. `FrameItem::Text.style: TextStyle` carrega os novos campos
   até ao frame (os consumers 137+ leem daí).
4. Nenhum consumer novo no PDF export — campos continuam
   inertes no output.
5. 3-5 testes L1 novos que validam a propagação por campo:
   "set text(X: Y) → TextStyle.X == Y".
6. Tests existentes inalterados (bold/italic/size/fill
   continuam a funcionar).

Este passo **não**:

- Adiciona consumer em layout.
- Altera PDF output.
- Adiciona propriedades a `StyleDelta` (já estão lá).
- Fecha DEBT-52 (resolve apenas 1 de 8 gaps).
- Toca ADRs.

---

## Decisões já tomadas

1. **Lote de 5 campos**: executar todos de uma vez.
2. **Tipos espelham `StyleDelta`**: `Option<u16>` para weight
   (não `Option<FontWeight>` — `FontWeight::to_number()` é
   consumer-side). `Option<Length>` para tracking/leading.
   `Option<Lang>` para lang. `Option<FontList>` para font.
3. **Propagação via `From<&StyleChain>`**: pattern existente,
   não reinventar.
4. **Sem `Default` novo**: campos ficam `None` quando não
   especificados — igual ao padrão actual.

## Decisões diferidas (136.A)

5. **Ordem dos campos em `TextStyle`**: agrupar com os
   existentes (bold, italic, size, fill, heading_level) ou
   separar? Confirmar convenção no ficheiro.
6. **`Clone`/`Debug`/`PartialEq`/`Hash` derives**: se `TextStyle`
   tem derives, os novos campos têm de ser compatíveis. `Lang`
   é `Copy`, `FontList` é `Clone`, `Length` é `Copy` —
   verificar se alguma incompatibilidade surge.
7. **Nome exacto do método de propagação**: `From<&StyleChain>`
   ou método dedicado? Confirmar em 136.A.

---

## Escopo

**Dentro**:
- `01_core/src/entities/...` — onde `TextStyle` vive
  (confirmar em 136.A).
- `01_core/src/entities/style_chain.rs` — `From<&StyleChain>`
  estendido.
- `01_core/src/rules/eval/tests.rs` — 3-5 testes novos de
  propagação.
- `00_nucleo/prompts/entities/text-style.md` (ou equivalente)
  + hash.
- DEBT.md — **actualização parcial de DEBT-52**: marcar gap 1
  ("estender TextStyle") como ✓.

**Fora**:
- Consumer em layout.
- PDF export.
- L2, L3, L4 excepto propagação automática via tipos (se
  acontecer, documentar).
- Novas ADRs.

---

## Sub-passos

### 136.A — Inventário confirmatório

**A.1 — Localizar `TextStyle`**:

`grep -rn "struct TextStyle" 01_core/src/`.

Registar:
- Ficheiro e linhas.
- Campos actuais (esperado: bold, italic, size, fill,
  heading_level).
- Derives (`#[derive(...)]`).
- Se tem `Default` impl.

**A.2 — Localizar `From<&StyleChain> for TextStyle`**:

`grep -rn "impl From.*TextStyle\|From<&StyleChain>" 01_core/src/`.

Registar:
- Ficheiro e linhas.
- Como cada campo é mapeado.
- Se há fallback values (ex: `size.unwrap_or(DEFAULT_SIZE)`).

**A.3 — Confirmar tipos disponíveis em scope**:

Para os 5 tipos novos (`u16`, `Length`, `Lang`, `FontList`):
- Verificar se já importados no ficheiro de `TextStyle`.
- Se não, anotar imports a adicionar.

**A.4 — Verificar derives compatíveis**:

- `Length`: `Copy, Clone, Debug, PartialEq, Eq, Hash` (confirmar).
- `Lang`: `Copy, Clone, Debug, PartialEq, Eq, Hash` (confirmado
  131B).
- `FontList`: `Clone, Debug, PartialEq, Eq, Hash` — **não é
  Copy** (contém `Vec`). Se `TextStyle` é `Copy`, incompatível.

**Se `TextStyle` é `Copy`**: decisão a tomar em 136.A — ou
remover `Copy` de `TextStyle` (ripple em call sites), ou
mudar tipo de font em `TextStyle` para `Option<Arc<FontList>>`
(adiciona indirecção mas preserva Copy). Esperado: `TextStyle`
não é `Copy` porque já tem `fill: Option<Color>` e `Color`
provavelmente não é Copy. Confirmar.

**A.5 — `FrameItem::Text.style` propagação automática**:

`grep -n "FrameItem::Text\|Text.*style: TextStyle" 01_core/src/ 03_infra/src/`.

Confirmar que `FrameItem::Text` tem `style: TextStyle` (ou
equivalente). Se sim, extensão de `TextStyle` propaga
automaticamente. Se há cópia ou conversão noutra camada,
documentar.

**A.6 — Call sites de `TextStyle::from` ou construtores**:

Identificar onde `TextStyle` é construído. Cada construtor
pode precisar de inicialização explícita dos novos campos
(ou `..Default::default()` se `Default` existe).

**A.7 — Tests base**:
- L1: 853.
- Total: 1084.

**Gate 136.A**:
- Se A.4 revela que `TextStyle` é `Copy` e `FontList` quebra
  essa propriedade: **parar e reportar**. Decisão necessária
  (remover Copy vs Arc wrap) excede XS.
- Se A.2 revela que `From<&StyleChain>` é mais complexo que
  mapeamento directo (ex: faz merging com outro source):
  documentar mas prosseguir se mapeamento dos 5 novos é
  trivial.
- Se A.6 revela 10+ call sites de construtor `TextStyle::new`
  (em vez de `Default::default()` + atribuição): o passo
  cresce. Registar mas prosseguir.
- Outros casos: prosseguir para 136.B.

### 136.B — Estender `TextStyle`

**Ficheiro**: confirmado em 136.A.1.

Imports a adicionar (se necessário):

```rust
use crate::entities::length::Length;  // ou caminho real
use crate::entities::lang::Lang;
use crate::entities::font_list::FontList;
```

Campos novos (ordem sugerida — agrupar por fonte de captura):

```rust
pub struct TextStyle {
    // Existentes:
    pub bold: bool,
    pub italic: bool,
    pub size: /* tipo actual */,
    pub fill: Option<Color>,
    pub heading_level: Option<u32>,

    // Novos (Passo 136 — Fase A DEBT-52):
    pub weight: Option<u16>,
    pub tracking: Option<Length>,
    pub leading: Option<Length>,
    pub lang: Option<Lang>,
    pub font: Option<FontList>,
}
```

Comentário no ficheiro antes dos novos campos:

```rust
// Passo 136 (Fase A de DEBT-52): campos propagados de
// StyleDelta para TextStyle. Sem consumer em layout ainda —
// Fases B/C resolvem. Ver ADR-0054.
```

Se `Default` existe, estender:

```rust
impl Default for TextStyle {
    fn default() -> Self {
        Self {
            // existentes ...
            weight: None,
            tracking: None,
            leading: None,
            lang: None,
            font: None,
        }
    }
}
```

Se não há `Default`, construtores directos precisam de
inicialização explícita — ver 136.E.

### 136.C — Estender `From<&StyleChain>`

**Ficheiro**: confirmado em 136.A.2.

Assumindo o pattern actual:

```rust
impl From<&StyleChain<'_>> for TextStyle {
    fn from(chain: &StyleChain<'_>) -> Self {
        Self {
            bold: chain.resolve_bold(),
            italic: chain.resolve_italic(),
            size: chain.resolve_size(),
            fill: chain.resolve_fill(),
            heading_level: chain.heading_level(),

            // Passo 136:
            weight: chain.resolve_weight(),
            tracking: chain.resolve_tracking(),
            leading: chain.resolve_leading(),
            lang: chain.resolve_lang(),
            font: chain.resolve_font(),
        }
    }
}
```

**Nota**: o nome exacto dos métodos de `StyleChain` é
confirmado em 136.A.2. Pode ser `chain.bold()`, `chain.weight()`,
ou outro padrão. Replicar o padrão existente para consistência.

**Se os métodos `resolve_X` não existem em `StyleChain`**:
adicionar neste passo. Cada um é uma linha:

```rust
impl StyleChain<'_> {
    pub fn resolve_weight(&self) -> Option<u16> {
        self.iter().find_map(|delta| delta.weight)
    }
    // ... análogos para tracking, leading, lang, font
}
```

O pattern exacto (ex: `find_map` vs scan manual) depende do
`StyleChain` actual. Espelhar métodos existentes.

### 136.D — Call sites de construtor

**Ficheiro**: conforme 136.A.6.

Se há construtores directos sem `..Default::default()`, cada
um precisa de `weight: None, tracking: None, leading: None,
lang: None, font: None` adicionados.

Se `Default` existe e call sites usam `..Default::default()`,
nada a fazer.

### 136.E — Testes novos

**Ficheiro**: `01_core/src/rules/eval/tests.rs` (ou onde os
outros testes de `From<&StyleChain>` vivem — confirmar em
136.A.2).

5 testes (um por campo novo):

```rust
#[test]
fn text_style_from_chain_propaga_weight_passo_136() {
    let src = "#set text(weight: 700)\nTexto";
    let frame = /* rodar pipeline até frame */;
    let text_style = /* extrair TextStyle do FrameItem::Text */;
    assert_eq!(text_style.weight, Some(700));
}

#[test]
fn text_style_from_chain_propaga_tracking_passo_136() {
    let src = "#set text(tracking: 0.1em)\nTexto";
    // ...
    assert_eq!(text_style.tracking, Some(/* Length 0.1em */));
}

#[test]
fn text_style_from_chain_propaga_leading_passo_136() {
    let src = "#set par(leading: 0.65em)\nTexto";
    // ...
    assert_eq!(text_style.leading, Some(/* Length 0.65em */));
}

#[test]
fn text_style_from_chain_propaga_lang_passo_136() {
    let src = r#"#set text(lang: "pt")\nTexto"#;
    // ...
    assert_eq!(text_style.lang, Some(/* Lang::from_str("pt") */));
}

#[test]
fn text_style_from_chain_propaga_font_passo_136() {
    let src = r#"#set text(font: "Arial")\nTexto"#;
    // ...
    let font = text_style.font.expect("font propagado");
    assert_eq!(font.as_slice()[0].name, "arial");
}
```

**Nota**: a forma exacta de extrair `TextStyle` do frame pode
exigir helper existente (ex: `eval_and_layout(src)` ou
similar). Confirmar pattern dos testes existentes de
propagação (provavelmente há testes similares do Passo 30 e
102 para bold/italic/size/fill).

Se não existe harness para ir até ao frame em testes L1,
alternativa: testar `TextStyle::from(&chain)` directamente
construindo `StyleChain` manualmente com `StyleDelta` populado.

### 136.F — Prompts L0

Se `00_nucleo/prompts/entities/text-style.md` existe, actualizar
com os 5 campos novos. Correr `crystalline-lint --fix-hashes .`.

Se o prompt tem exemplo de `TextStyle`, incluir os novos
campos com valores `None`.

### 136.G — Actualizar DEBT-52

**Ficheiro**: `00_nucleo/DEBT.md`.

Na entrada DEBT-52, marcar gap 1 como feito:

```markdown
- [x] **Gap 1**: estender `TextStyle` com 5 campos novos +
  propagação via `From<&StyleChain>`. **Resolvido no Passo 136**.
- [ ] **Gap 2**: consumer `tracking` em layout.
- [ ] ...
```

Contagem no topo do DEBT.md permanece 12 — DEBT-52 continua
aberto (7 gaps restantes).

### 136.H — Verificação

1. `cargo test -p typst-core` — L1: 853 → **858** (+5 testes
   novos).

2. `cargo test --workspace` — total ≥ 1089.

3. `crystalline-lint` zero violations.

4. Manual (regressão, não nova funcionalidade):

```bash
$ typst b.typ       # #set text(bold: true)
exit=0, PDF com bold (regressão Passo 30)

$ typst w.typ       # #set text(weight: 700)
exit=0, PDF sem efeito visível (weight capturado, não consumido)
                    # mas TextStyle.weight está populado — verificável em debug

$ typst f.typ       # #set text(font: "Arial")
exit=0, PDF sem efeito visível (font capturado, não consumido)
                    # mas TextStyle.font está populado

$ typst h.typ       # canary hyphenate (regressão 132B)
h.typ:1:11: warning: text: propriedade 'hyphenate' ainda não suportada
exit=0
```

**Nota**: o teste manual não valida efeito visível do weight/font
— eles continuam inertes. Apenas valida que regressões não
existem.

### 136.I — Encerramento

Relatório em `typst-passo-136-relatorio.md`:

- Confirmações 136.A (TextStyle Copy?, From<&StyleChain> pattern,
  call sites).
- Diff por ficheiro.
- Números finais.
- DEBT-52 gap 1 marcado como feito (7 restantes).
- Preparação para 137 (primeiro consumer — provavelmente
  tracking).

---

## Critério de conclusão

1. Inventário 136.A escrito (7 pontos verificados).
2. `TextStyle` ganha 5 campos (`weight`, `tracking`, `leading`,
   `lang`, `font`).
3. Derives compatíveis (não quebra Copy se existia, ou
   decisão documentada se quebra).
4. `Default` (se existe) estendido.
5. `From<&StyleChain>` propaga os 5 novos.
6. Métodos `resolve_X` em `StyleChain` existem (novos ou
   pré-existentes).
7. Call sites de construtor adaptados (se aplicável).
8. 5 testes novos de propagação passam.
9. Testes existentes (bold/italic/size/fill) inalterados.
10. L1 tests: **858** (+5).
11. `cargo test --workspace` passa (≥ 1089).
12. `crystalline-lint` zero violations.
13. DEBT-52 gap 1 marcado como resolvido.
14. Relatório 136.I escrito.

---

## O que pode sair errado

- **`TextStyle` é `Copy` e `FontList` quebra Copy**: decisão
  escala o passo. Opções: (a) remover Copy de TextStyle
  (ripple em call sites), (b) usar `Option<Arc<FontList>>`
  (indirecção extra). Se detectado em 136.A.4, parar.

- **`From<&StyleChain>` é método complexo com merging**: se
  o método não é mapeamento directo mas inclui lógica (ex:
  herança entre níveis do chain), os 5 novos campos podem
  precisar de replicar essa lógica. Parte do pattern
  existente — seguir.

- **Métodos `resolve_X` em `StyleChain` não existem**: adicionar
  neste passo. Cada um é 1-2 linhas. Baixo custo.

- **Harness de teste não vai até ao frame**: testes alternativos
  construindo `StyleChain` manualmente. Menos realista mas
  válido.

- **Call sites de construtor são > 10**: se há muitos usos
  directos de `TextStyle { ... }` sem `..Default::default()`,
  o passo cresce. Provável que `Default::default()` seja
  pattern comum.

- **`Heading_level` como precedente diferente**: `heading_level`
  foi adicionado noutro contexto (talvez via show rule). Se
  o pattern dele é distinto do mapeamento directo, não replicar
  — seguir pattern dos campos bold/italic/size/fill.

---

## Notas operacionais

- **Este é o primeiro passo da Fase A**. Se sai limpo, valida
  o roadmap revisto (135). Se encontra bloqueios estruturais
  (ex: TextStyle Copy que quebra), a estimativa de
  "4-8 passos" pode crescer.

- **Campos ficam inertes intencionalmente**. Utilizador que
  faz `#set text(weight: 700)` vê zero efeito no PDF. A
  justificação é o plano: 137+ adicionam consumer.

- **Regressão é a verificação principal**: o passo não adiciona
  efeito visível, mas não deve tirar nenhum existente. Tests
  bold/italic/size/fill continuam.

- **Pattern 131B/132B** (inverter/adaptar testes existentes)
  não se aplica: nenhum teste existente precisa de mudar
  semântica — só são adicionados novos.

- **Ritmo estimado**: XS real. ≈1h. A maior parte é confirmação
  em 136.A (tipos Copy, pattern do From). Depois do inventário,
  edits são mecânicos.

- **Candidato `eval_with_warnings`** continua pendente. Após
  137 (primeiro consumer) o harness ganha complexidade.
  Priorizar após Fase B.
