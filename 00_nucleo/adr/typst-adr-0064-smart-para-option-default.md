# ⚖️ ADR-0064: Tradução `Smart<T>` vanilla → `Option<T>`/default cristalino

**Status**: `EM VIGOR`
**Data**: 2026-04-26

---

## Contexto

O Typst vanilla usa o tipo `Smart<T>` (enum `Auto | Custom(T)`)
para representar campos com default contextual ou auto-resolvido
em momento de uso. Aparece em centenas de campos de elements
vanilla (`BlockElem.width`, `BoxElem.height`, `StackElem.spacing`,
`PagebreakElem.to`, etc.) onde `Auto` significa uma das duas
semânticas:

- **Auto contextual**: "computar do contexto envolvente" (ex:
  `BlockElem.width = Smart<Rel<Length>>` com `Auto` = "ocupar
  largura disponível na linha actual").
- **Auto literal fixo**: "usar um valor literal fixo" (ex:
  `Smart<Dir>` com `Auto` = `Dir::TTB`).

O cristalino enfrentou esta escolha **8 vezes consecutivas** na
série P156C-J (materialização Layout Fase 1 + Fase 2 + Fase 3
sub-passo 1):

| Passo | Campo vanilla | Tradução cristalina |
|-------|---------------|---------------------|
| P156D | `weak: bool` (default false) | `bool` directo |
| P156E | `Smart<Parity>` | `Option<Parity>` |
| P156G | `Smart<Rel<Length>>` (Block.width) | `Option<Length>` |
| P156G | idem (Block.height) | `Option<Length>` |
| P156H | `Smart<Rel<Length>>` (Box.width/height) | `Option<Length>` |
| P156H | `Length` (Box.baseline, default zero) | `Length` directo |
| P156I | `Smart<Length>` (Stack.spacing) | `Option<Length>` |
| P156I | `Smart<Dir>` (Stack.dir) | `Dir` directo (Default = TTB) |
| P156J | `Length` (Repeat.gap, default zero) | `Option<Length>` |
| P156J | `bool` (Repeat.justify, default true) | `bool` directo |

A regra que emergiu — **N=6 aplicações empíricas** consecutivas
do mesmo padrão de tradução com **zero reformulações mid-passo**
— é o objecto deste ADR.

---

## Decisão

### Regra de tradução vinculativa

Quando o vanilla expõe campo de element com semântica de "default
contextual" ou "default literal fixo", o cristalino traduz segundo
quatro casos canónicos:

#### Caso A — `Smart<T>` com semântica "auto = computa do contexto"

→ **`Option<T>`** no variant cristalino.
→ `Auto` vanilla traduz-se em `None`.
→ Default contextual resolvido **em momento de uso** (stdlib func
  ou layout arm).

**Exemplos**:
- P156G/H: `Smart<Rel<Length>>` para `width`/`height` →
  `Option<Length>` (default = ocupar largura disponível ou
  computar do body).
- P156I: `Smart<Length>` para `spacing` → `Option<Length>` (None ==
  zero).

#### Caso B — `Smart<T>` com semântica "auto = valor literal fixo"

→ **`T` directo** se `T: Default`.
→ `Auto` vanilla traduz-se em `T::default()` natural.
→ Construtor cristalino aceita o `T` directamente; default natural
  é o valor literal fixo.

**Exemplo**:
- P156I: `Smart<Dir>` → `Dir` directo com `Default::default() ==
  Dir::TTB` (paridade vanilla `Auto = TTB`).

#### Caso C — campo vanilla `T` com default não-`Default::default()`

→ **`Option<T>`** no variant cristalino.
→ `None` representa "usar o default vanilla declarado" (e.g. zero).
→ Default resolvido em momento de uso.

**Exemplos**:
- P156I: `Stack.spacing: Length` (default `Length::zero()`) →
  `Option<Length>` (None ≡ zero).
- P156J: `Repeat.gap: Length` (default `Length::zero()`) →
  `Option<Length>` (None ≡ zero).

#### Caso D — campo vanilla `bool` com default não-`false`

→ **`bool` directo**, NÃO `Option<bool>`.
→ Documentação explícita do default vanilla na declaração do
  variant e no construtor.
→ Construtor stdlib aceita o bool directamente; default vanilla
  literal é o valor não-`false` declarado.

**Justificação**: `Option<bool>` introduz tristado (`None|Some(true)
|Some(false)`) sem semântica vanilla correspondente. O default
não-`false` é apenas convenção; mantém-se `bool` directo com
documentação clara.

**Exemplos**:
- P156D: `HSpace.weak: bool` (default false) → `bool` directo.
- P156G: `Block.breakable: bool` (default true) → `bool` directo.
- P156J: `Repeat.justify: bool` (default true vanilla) → `bool`
  directo, default `true` documentado em prompt L0 e construtor.

### Excepções

Casos onde a regra **não** se aplica:

- `Auto` vanilla com semântica **distinta** de "default" (ex:
  marcador semântico que precisa de discriminação explícita por
  arm) → enum dedicado, não `Option`.
- Campos onde `T` tem `Default` natural alinhado com vanilla mas
  o variant cristalino precisa distinguir "explícito" de "implícito"
  por razões de show rules → `Option<T>` mesmo com `T: Default`.

Excepções documentam-se no diagnóstico do passo (per ADR-0034) e
no comentário do field do variant.

---

## Justificação empírica — N=6 aplicações

| Passo | Aplicação | Reformulação mid-passo? |
|-------|-----------|:------------------------:|
| P156D | `weak: bool` directo (Caso D) | ✗ |
| P156E | `Smart<Parity>` → `Option<Parity>` (Caso A) | ✗ |
| P156G | `Smart<Rel<Length>>` → `Option<Length>` (Caso A) | ✗ |
| P156H | idem Box.width + Box.baseline `Length` directo (Casos A+D variant) | ✗ |
| P156I | `Smart<Length>` → `Option<Length>` (Caso A) + `Smart<Dir>` → `Dir` directo (Caso B) | ✗ |
| P156J | `Length` (default zero) → `Option<Length>` (Caso C) + `bool` directo (Caso D) | ✗ |

**Total**: 6 passos consecutivos com 0 reformulações. Cobertura
de todos os 4 casos canónicos (A, B, C, D) na série. Patamar
empírico forte que justifica formalização como regra vinculativa.

---

## Implicações

### Código mais idiomático Rust

`Option<T>` é primitivo idiomático em Rust; `Smart<T>` exigiria
implementar tipo paralelo com semântica idêntica + boilerplate
(`is_auto`, `unwrap_or_default`, etc.). `Option` traz toda a
ergonomia da stdlib (`map`, `and_then`, `or_else`, etc.).

### Diagnósticos de erro mais claros

Variant cristalino expõe `Option<Length>` directamente; erro
de tipo apresenta o tipo Rust real, não a nossa abstracção.
Stdlib funcs validam com mensagens estruturais ("X(field:)
espera length, recebeu Y").

### Helpers stdlib consolidados

A regra cristalizou em **5 helpers stdlib reusados** em
`stdlib/layout.rs`:
- `extract_length` — coage `Length`/`Float`/`Int` para `Length`.
  **Reusado N=6 vezes consecutivas** (P156C/D/G/H/I/J).
- `extract_parity` — coage `Str` para `Parity` (P156E).
- `extract_dir` — coage `Str` para `Dir` (P156I).
- `extract_weak` — coage `bool` named arg para `bool` (P156D).
- `build_spacing` — combina `extract_length` + `extract_weak`
  para HSpace/VSpace (P156D).

### Subpadrão emergente: promoção de `extract_length`

Reuso N=6 consecutivos de `extract_length` é **subpadrão dentro
deste ADR**: o helper deixou de ser local de `pad`/`hide`
(P156C) para se tornar vocabulário canónico da stdlib layout.
**Promoção a helper público** (`pub fn extract_length(...)` em
`stdlib/mod.rs` ou em `stdlib/coercion.rs` novo) é candidato a
**refactor escopo XS** futuro — não materializado neste ADR
para preservar âmbito (regra de tradução, não refactor de
infraestrutura).

---

## Relação com outros ADRs

### Estende `ADR-0034` (diagnóstico obrigatório)

Cada aplicação da regra Smart→Option/default ocorre dentro do
sub-passo `.1` de inventário per ADR-0034. O diagnóstico
identifica o tipo vanilla, classifica em Caso A/B/C/D, e
documenta a tradução escolhida.

### Compatível com `ADR-0033` (paridade observável)

A regra preserva paridade observável (defaults vanilla são
preservados em momento de uso). Divergência permitida é apenas
**estrutural** (tipo cristalino diferente do vanilla) —
permitida explicitamente por ADR-0033.

### Reforçado por `ADR-0061` §"Aplicações cumulativas"

ADR-0061 §"Aplicações cumulativas" pós-P156J documenta o padrão
como N=6 e referencia este ADR para formalização.

---

## Consequências

### Positivas

- **Sessões futuras citam ADR-0064** em vez de re-justificar
  empiricamente cada passo. Reduz overhead de enunciados.
- Regra mecânica reduz superfície de decisão arquitectural a
  classificação Caso A/B/C/D, deixando o LLM focado em validações
  semânticas (negativos, ranges, conflito de naming).
- Helpers stdlib consolidados (especialmente `extract_length`
  N=6) tornam-se vocabulário compartilhado.

### Negativas

- Casos exóticos (Caso "Auto distinto de default") podem ser
  mal-classificados como Caso A se inventário .1 for superficial
  — exige rigor no diagnóstico per ADR-0034.
- Caso D (bool directo com default não-`false`) é discutível em
  contextos onde o utilizador precisa distinguir "explícito" de
  "implícito" — excepção documentada na regra.

### Neutras

- Regra **não** dita estrutura interna do enum cristalino (variant
  rico vs Style cascade) — essa decisão é separada (P156G).
- Regra **não** se aplica a `Args` named (helper stdlib) onde
  ausência já é representada por `None` natural via
  `args.named.get(key)`.

---

## Alternativas Consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Replicar `Smart<T>` como tipo paralelo em L1 | Paridade estrutural directa com vanilla | Boilerplate; perde ergonomia de Option; viola ADR-0033 ("divergência estrutural permitida") |
| Usar `Option<T>` apenas para Caso A; tipo dedicado para B/C/D | Captura semântica mais fina | Aumenta superfície sem benefício observável (paridade observável já preservada) |
| Documentar como convenção informal sem ADR | Menos overhead documental | N=6 reformulações futuras possíveis se padrão não for citável |
| **Decisão adoptada: 4 casos canónicos vinculativos com excepções documentadas** | **Equilibra rigor + ergonomia + auditabilidade** | **Caso E pode emergir; expansão futura prevista** |

---

## Referências

- ADR-0033 — Paridade funcional vanilla (regra que este ADR
  preserva).
- ADR-0034 — Diagnóstico obrigatório (mecanismo onde a tradução
  é classificada).
- ADR-0061 §"Aplicações cumulativas" pós-P156J — patamar
  empírico documentado.
- Relatórios P156D, P156E, P156G, P156H, P156I, P156J — evidência
  empírica em `00_nucleo/materialization/`.
- `01_core/src/rules/stdlib/layout.rs` — helpers consolidados
  (`extract_length`, `extract_parity`, `extract_dir`,
  `extract_weak`, `build_spacing`).
- `lab/typst-original/.../foundations/values.rs` — definição
  vanilla de `Smart<T>`.
