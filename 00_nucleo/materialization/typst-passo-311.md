# typst-passo-311 — 12 funções math style (`bb`/`cal`/`frak`/...)

**Tipo**: Passo de Execução (planeamento táctico)
**Sub-tipo**: composto — exige Fase 1.5 diagnóstico
**Data**: 2026-05-20
**Magnitude**: M+ (composta de 1 diagnóstico + 1 materialização)
**Pré-requisitos**: nenhum — passo é diagnóstico-primeiro
**Cobertura prevista**: stdlib agregada ~54,5% → ~57%

---

## 1. Motivação

P306+P308 fecharam categoria `calc` (41/41 = 100%). Auditoria
empírica de 2026-05-19 identificou 12 funções math style ausentes
em A.7 Text features:

`bb`, `bold`, `cal`, `frak`, `italic`, `mono`, `sans`, `scr`,
`script`, `serif`, `sscript`, `upright`

Bloco coeso candidato natural pós-P310 (paralelo P306 mas escopo
diferente: text style math vs scalar math).

---

## 2. Diferença crítica vs P306

**P306 (calc)** foi escopo simples:
- Funções escalares puras `Value::Float → Value::Float`.
- Sem `Content`, sem variants novos, sem layout.
- Helpers já existiam (`coerce_to_f64`, `guard_float`).
- Aritmética pura em `f64`.

**P311 (math style)** é escopo arquitectural distinto:
- Funções produzem `Content`, não `Value::Float`.
- Cada função recebe um corpo math e retorna corpo com variant glyph aplicado.
- Mapping ASCII → Unicode variant (tabela ~52 letras × 12 variants).
- Composição entre variants (`bb(cal(x))` — qual prevalece?).
- Integração com `MathLayouter` e `is_single_letter_var`/`is_math_function`.

Não há scaffolding directo. **Exige Fase 1.5 diagnóstico** antes
do L0.

---

## 3. Decisões arquitecturais a fixar em P311a (diagnóstico)

### 3.1 — Como representar a transformação?

Três caminhos viáveis:

**Caminho I — Variant Content dedicado**:
```rust
Content::MathStyled {
    kind: MathStyleKind,  // Bb, Cal, Frak, ...
    body: Box<Content>,
}
```
- Composicional natural (`bb(cal(x))` = nested variants).
- Custo: 1 variant novo em `Content` (24 → 25); 1 enum
  `MathStyleKind` em `entities/`.
- Hash `content.rs` **quebra** (28º consecutivo termina).

**Caminho II — Mapping Unicode directo (eager)**:
Cada função substitui ASCII por codepoint variant logo no
`native_*`. `bb("A")` → `Content::MathText("𝔸")`.
- Sem variant novo; preserva hash `content.rs`.
- Composição não-natural: `bb(cal(x))` exige reparsing.
- Limitado a strings literais; falha em variáveis (`bb(x)`
  onde `x` é binding).

**Caminho III — Style enum extension**:
```rust
Style::MathVariant(MathStyleKind)
```
Aplicado via `Content::Styled(body, [MathVariant(Frak)])`.
- Reusa infraestrutura `Style`/`StyleChain` (ADR-0040).
- `MathLayouter` precisa consultar chain para resolver
  glyph variant.
- Composição via merge de `Style`.

**Recomendação preliminar**: Caminho I se diagnóstico revelar
poucos cross-cutting concerns; Caminho III se DEBT-StyleChain
materializado; Caminho II rejeitado (não-composicional).

### 3.2 — Tabela Unicode variant — onde fica?

Vanilla usa `typst-utils::math_variant` com tabela ~52 letras
× 12 variants. Cristalino tem três opções:

**Opção α — Inline em `entities/math_style.rs`**:
- Cresce ~200 LOC.
- Sem dependência externa.

**Opção β — Crate externa `unicode-math-symbols`**:
- Verificar autorização ADR-0018-style (puro?).
- ~3 KB binário.

**Opção γ — Geração on-the-fly**:
- Unicode planos `MATHEMATICAL BOLD CAPITAL` têm offsets
  regulares.
- Tabela compacta de excepções.
- ~50 LOC + 10 excepções literais.

**Recomendação preliminar**: γ (compacta) se diagnóstico
confirmar regularidade dos offsets; α caso contrário.

### 3.3 — Composição entre variants

`bb(cal(x))` — qual prevalece? Vanilla tem regra específica:
último variant aplicado ganha (right-to-left binding).

P311a diagnóstico inspecciona `lab/typst-original/` para
confirmar e documenta os casos:
- `bb(cal(x))` → `bb` apenas.
- `bold(italic(x))` → ambos (caso especial; bold-italic é
  variant separado).
- `upright(italic(x))` → `upright` ganha.

### 3.4 — Integração com `is_single_letter_var`

Pré-P311: `MathLayouter` aplica itálico automático a variáveis
de uma letra via `is_single_letter_var`.

Pós-P311: `bb(x)` ou `upright(x)` deve **suprimir** este
itálico automático. Diagnóstico identifica sítios afectados.

### 3.5 — Funções não-glyph: `script`/`sscript`

Estas duas não fazem variant glyph — alteram **tamanho**:
- `script`: 70% do tamanho base (subscript-style).
- `sscript`: 50% (sub-sub-script).

Família distinta do resto. Diagnóstico decide:
- Tratar como variant separado (`MathStyleKind::Script`).
- Tratar como caso de `Style::Size` (reuso ADR-0040).

---

## 4. Estrutura proposta — 2 sub-passos

### P311a — Diagnóstico-primeiro (magnitude M documental)

Output: `00_nucleo/diagnosticos/diagnostico-math-style-passo-311a.md`

Estrutura:
```
§1. Inventário vanilla (12 funções com signatures)
§2. Tabela Unicode variant (mapping ASCII → 12 codepoints)
§3. Decisões arquitecturais (§3.1-3.5 desta spec)
§4. Comparação caminhos I/II/III (matriz prós/contras)
§5. Integração com is_single_letter_var
§6. Composição cross-variant (regras de prevalência)
§7. Família script/sscript (tamanho vs variant)
§8. Recomendação operacional para P311b
§9. ADRs candidatas (se Caminho I escolhido)
```

Sem código tocado. Sem ADR criada. Sem L0 alterado.

### P311b — Materialização (magnitude M+ a L)

Depende das decisões fixadas em P311a. Estimativa preliminar:

- **L0**: `00_nucleo/prompts/engine/stdlib.md` + possível L0 novo
  em `00_nucleo/prompts/entities/math_style.md` (se Caminho I).
- **L1**:
  - Caminho I: `01_core/src/entities/math_style.rs` (novo,
    ~80 LOC) + variant em `content.rs` + impl em
    `rules/stdlib/math.rs` (ou similar).
  - Caminho II: 12 funções em `rules/stdlib/math.rs` apenas.
  - Caminho III: extensão em `entities/style.rs` + 12 funções
    nativas.
- **MathLayouter**: ajuste em `is_single_letter_var` para
  consultar variant activo.

Estimativa: ~300-500 LOC L1 + ~30 tests.

---

## 5. Ficheiros tocados (previsão)

### 5.1 — P311a (só leitura + 1 diagnóstico novo)

Leitura:
- `lab/typst-original/crates/typst-library/src/math/style.rs`
- `lab/typst-original/crates/typst-library/src/math/variant.rs`
- `01_core/src/entities/content.rs`
- `01_core/src/entities/style.rs`
- `01_core/src/engine/math/symbols.rs`
- `01_core/src/engine/math/layout/**`
- `01_core/src/engine/stdlib/math.rs` (ou equivalente)

Output novo:
- `00_nucleo/diagnosticos/diagnostico-math-style-passo-311a.md`

### 5.2 — P311b (depende do caminho)

Provável:
- L0 novo (Caminho I): `00_nucleo/prompts/entities/math_style.md`
- L0 actualizado: `00_nucleo/prompts/engine/stdlib.md` (12
  funções novas) + possivelmente `entities/content.md` (variant
  novo se Caminho I).
- L1: novos ficheiros e/ou extensões.

---

## 6. ADRs reusadas + possíveis novas

### Reusadas (independente do caminho):
- ADR-0017 — Estratégia gradual typst-library.
- ADR-0033 — Paridade observable.
- ADR-0036 — Atomização `&Args`.
- ADR-0037 — Coesão por domínio.
- ADR-0054 — Perfil observacional graded.
- ADR-0059 — `Args` como input vehicle.

### Possíveis novas (P311b, depende de P311a):
- **ADR-Math-Style-Mechanism** se Caminho I escolhido (formaliza
  `Content::MathStyled` como variant arquitectural).
- **ADR-Math-Style-Composition** se regra de prevalência for
  decisão consciente (right-to-left binding documentada).
- **ADR-Unicode-Math-Crate** se Opção β e crate externa
  precisar autorização.

P311a recomenda quais ADRs criar; humano decide promoção.

---

## 7. Protocolo de Nucleação — sequência prevista

| Fase | P311a | P311b |
|---|---|---|
| 1. Plano | este doc | aguarda P311a + decisão humana |
| 2. L0 | n/a (só diagnóstico) | depende caminho |
| 3. Hash | n/a | obrigatório |
| 4. Testes | n/a | testes-primeiro |
| 5. Implementação | n/a | materialização |
| 6. Validação | crystalline-lint clean | id. |

**Trava arquitectural**: P311b não pode arrancar antes de P311a
estar fechado e humano ter aceite o caminho recomendado.

---

## 8. Granularidade — fixada

P311 é **série de 2 sub-passos** (P311a diagnóstico + P311b
materialização). Justificação:

- Decisões arquitecturais não-triviais (§3.1-3.5).
- Magnitude composta M+ a L excede limite passo singular.
- Precedente: P156B → P156G (Layout); P307a → P307b
  (decomposição L3).
- Diagnóstico-primeiro per ADR-0065.

---

## 9. Critérios de fecho

### P311a fechado quando:
- [ ] Diagnóstico publicado com §1-§9 completas.
- [ ] §3.1 caminho I/II/III recomendado com racional.
- [ ] §3.2 tabela Unicode opção α/β/γ recomendada.
- [ ] §3.3 regra de composição documentada.
- [ ] §3.4 sítios afectados em `MathLayouter` listados.
- [ ] §3.5 família `script`/`sscript` resolvida.
- [ ] §8 recomendação operacional para P311b explícita.

### P311b fechado quando:
- [ ] L0 actualizado per §5.2.
- [ ] Hashes propagados; `crystalline-lint .` zero violations.
- [ ] 12 funções implementadas em L1.
- [ ] ~25-35 testes unitários verdes.
- [ ] ~3-5 testes E2E paridade verdes.
- [ ] Integração `is_single_letter_var` confirmada (testes
  específicos).
- [ ] Composição cross-variant funciona (testes específicos).
- [ ] Relatório de passo escrito.

Invariantes a preservar:
- [ ] `export/*` snapshots inalterados (4º consecutivo
  pós-P307).
- [ ] **`content.rs` hash**: depende do caminho.
  - Caminho I: hash quebra (28º consecutivo termina; aceitável
    e necessário per ADR-0033).
  - Caminhos II/III: hash preservado (28º consecutivo).
- [ ] Cobertura calc 41/41 inalterada.
- [ ] Tests pré-existentes inalterados.

---

## 10. Cobertura pós-P311

**Stdlib agregada**:
- Antes: ~54,5% (pós-P308).
- Depois: ~57% (12 funções de 12 ausentes na sub-categoria
  math style; +~2,5pp global).

**Cobertura A.7 Text features**:
- Antes: 57,1% (P305).
- Depois estimativa: ~65% (12 funções math style + sub-features
  já cobertas).

**Cobertura empírica de elementos**:
- Vanilla `MathStyleElem` e variantes: 12 funções `#[func]` + 1
  elemento se Caminho I.
- Cristalino: depende do caminho.

---

## 11. Não-objectivos

P311 **não**:

- Materializa shaping completo (rustybuzz) — scope-out ADR-0054
  perfil graded.
- Cobre `MathTextElem`/`MathStyleElem` granulares vanilla — só
  as 12 funções stdlib.
- Refactoriza `MathLayouter` significativamente — só ajustes
  pontuais em `is_single_letter_var`.
- Toca DEBT-StyleChain — Caminho III aumenta acoplamento mas
  não materializa StyleChain real.

---

## 12. Riscos identificados

| Risco | Probabilidade | Mitigação |
|---|---|---|
| Diagnóstico revela que Caminho I exige refactor M+ adicional | Média | P311a documenta honestamente; humano pode adiar P311b |
| Tabela Unicode opção γ não-regular força α (200 LOC) | Média | P311a verifica regularidade em vanilla antes de recomendar |
| Composição `bb(cal(x))` vanilla é mais complexa que right-to-left | Baixa | P311a inspecciona casos de borda |
| `MathLayouter` ajuste em `is_single_letter_var` quebra tests pré-existentes | Média | P311b testes-primeiro confirmam regressão zero |
| ADR nova necessária para Caminho I bloqueada por discussão | Possível | P311a recomenda; humano decide promoção |
| Hash `content.rs` quebra após 27 consecutivos | Aceitável | Caminho I declara explicitamente; documenta no relatório |

---

## 13. Próxima acção concreta

**Aguardar confirmação humana** para arrancar P311a.

Decisões pendentes:

1. **Arrancar P311a?** Magnitude M documental; baixo risco;
   sem código tocado.
2. **Escopo do diagnóstico**:
   - α) Decisões §3.1-3.5 todas obrigatórias.
   - β) Subset (e.g. só §3.1 + §3.2; outras adiadas).

Recomendação: escopo α (todas as decisões) — diagnóstico
parcial força decisões ad-hoc em P311b.

3. **Caminho preliminar preferido** (sem decisão fixada;
   informa o diagnóstico):
   - I) Variant `Content::MathStyled`.
   - II) Mapping Unicode eager.
   - III) Style enum extension.
   - α) Sem preferência; diagnóstico recomenda.

Sem confirmação, P311 fica em standby. Frentes alternativas
da lista pós-P310 (§9 do relatório P310) ficam disponíveis:
Cat D reforços, DEBT-libm migração agregada, outras categorias
stdlib.
