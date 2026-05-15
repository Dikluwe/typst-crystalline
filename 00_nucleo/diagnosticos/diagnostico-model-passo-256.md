# Diagnóstico Model — Passo 256

**Data**: 2026-05-15
**Tipo**: passo arquitectural de diagnóstico (não materializa código)
**Estrutura**: duas fases registadas (precedente P255 cristalizou
o padrão "auditoria condicional" N=2):
- **Fase A** — inventário empírico das 22 entradas Model
  declaradas em P154A; classificar cada uma como
  `implementado/implementado⁺/parcial/ausente/scope-out`.
- **Fase B** — decisão condicional sobre próximos passos
  baseada em cobertura real apurada.

**Análogo estrutural**: P254A (Introspection actualizado),
P254B (Math), P255 (Math executado).

**Motivação**: o resumo cumulativo pós-P254 cita Model ~50%.
Esse número provém de P157A (2026-04-26). Houve ~6 semanas de
evolução cumulativa em M3-M9 + ADR-0066 + ADR-0072 que podem
ter movido a cobertura — paralelo com o que aconteceu a
Introspection (~17% → ~85% per P254A).

---

## §1 — ADRs e DEBTs relevantes

### ADRs activos para Model

| ADR | Status | Relevância |
|-----|--------|------------|
| ADR-0017 | IMPLEMENTADO | Estratégia gradual typst-library — fundação |
| ADR-0026 + R1 | EM VIGOR | Content como enum; `Arc<[T]>` em Sequence |
| ADR-0033 | EM VIGOR | Paridade observable vanilla |
| ADR-0034 | EM VIGOR | Diagnóstico canónico |
| ADR-0038 | EM VIGOR | `Content::Styled` — rejeitado para Model structural |
| ADR-0054 | EM VIGOR | Perfil graded — scope-outs aceites |
| ADR-0060 | IMPLEMENTADO (Fase 1 fechada P155) | Model structural roadmap — fonte das 22 entradas |
| ADR-0061 | (estado a confirmar) | Layout Fase X — desbloqueou footnote em P156C |
| ADR-0062 | PROPOSTO | Hayagriva autorização (Bloco B Model) |
| ADR-0064 | EM VIGOR | Smart→Option (Casos A-D em Model) |
| ADR-0065 | EM VIGOR | Inventariar primeiro |

### DEBTs

- **DEBT-55**: bibliography+cite XL; **estado actual incerto**.
  P159A contribuiu mas não fechou. Bloco B (hayagriva) não
  confirmado materializado. Refinos P159C-G podem ter encerrado
  parcialmente.

### Histórico de transições ADR-0060

- **2026-04-25** (P154A): PROPOSTO criado com roadmap 3 fases.
- **2026-04-25** (P155): PROPOSTO → IMPLEMENTADO (Fase 1
  fechada — terms/divider/quote).
- **Mantido IMPLEMENTADO** pós-P157/P158/P159 (anotação
  cumulativa); Fase 2 prossegue.

---

## §2 — Inventário declarado (P154A, 2026-04-25)

P154A classificou 22 entradas Model. Estado **declarado** em
2026-04-25:

| Classificação | Contagem | Entradas |
|---------------|----------|----------|
| `implementado` | 3-4 | heading, emph, strong, outline |
| `implementado⁺` | 4 | figure, ref, numbering, heading com ressalva |
| `parcial` | 5 | link, list, enum, par, caption inline |
| `ausente` | 10 | bibliography, cite, footnote, quote, terms, table, document, divider, asset, title |
| `scope-out` | 0 | — |

Cobertura **declarada** P154A: 32-36%.

### Evolução cumulativa declarada (linha temporal)

| Passo | Δ Model | Cobertura agregada |
|-------|---------|---------------------|
| P154B (2026-04-25) | +terms, +divider, +termitem | 32-36% → ~41% |
| P155 (2026-04-25) | +quote | ~41% → ~45% |
| P157A (2026-04-26) | +table | ~45% → **50%** |
| P157B | +TableCell (sub-entrada) | 50% inalterada (qualitativa) |
| P157C | +TableHeader/Footer (sub-entradas) | 50% inalterada |
| P158A-C (figure-kinds) | refinos qualitativos | 50% inalterada |
| P159A | +bibliography, +cite par acoplado | ~50% (refino estrutural) |
| P159C-G | refinos BibEntry (16 fields) | 50% inalterada |

**Estado declarado pós-P159G** (2026-04-27 aprox.): **~50%
agregada** com 24 entradas parciais.

### Refinos pós-P159G não auditados em Model

Entre P160 e P254 (~3 semanas) houve actividade massiva mas
quase toda focada em Introspection (M3-M9, P164-P204). Para
Model, **não há evidência cumulativa** no contexto disponível
de:

- `footnote` materializado (estava desbloqueado por P156C).
- `document` / `title` / `asset` (Fase 3 condicional).
- Hayagriva integration (Bloco B; promoção ADR-0062 PROPOSTO →
  IMPLEMENTADO).
- Refinos `list`/`enum`/`par`/`link` para subir de `parcial`.

---

## §3 — Fase A: Inventário empírico (a executar)

**Análogo a P255 §1**: comandos `grep`/`view` que produzem
evidência factual antes de decisão.

### Entrada por entrada — comandos sugeridos

#### Entradas `implementado` (P154A — verificar manutenção)

```bash
# heading, emph, strong, outline
grep -n "Content::Heading\b\|Content::Emph\b\|Content::Strong\b\|Content::Outline\b" \
  01_core/src/entities/content.rs
```

**Critério**: confirmar que variants existem; nenhum refactor
posterior deteriorou estado. Esperado: 4/4 confirmados
implementado.

#### Entradas `implementado⁺` (P154A — verificar refinos)

```bash
# figure, ref, numbering
grep -n "Content::Figure\b\|Content::Ref\b" 01_core/src/entities/content.rs
# numbering — não é variant Content, é mecanismo
grep -rn "SetHeadingNumbering\|SetEquationNumbering\|format_hierarchical" \
  01_core/src/rules/
```

**Critério**: confirmar que SetHeadingNumbering (P182C) e
SetEquationNumbering (P199B) estão integrados — moveriam
`numbering` de `implementado⁺` para `implementado` real.

#### Entradas `parcial` (P154A — verificar evolução)

```bash
# link, list, enum, par, caption inline
grep -n "Content::Link\b\|Content::List\b\|Content::Enum\b\|Content::Par\b" \
  01_core/src/entities/content.rs
grep -n "caption" 01_core/src/entities/content.rs
```

**Critério**: cada `parcial` pode ter subido para
`implementado` via refinos pós-P154A. Provavelmente não — a
actividade pós-P159G concentrou-se em Introspection.

#### Entradas `ausente` (P154A — verificar materialização)

```bash
# Já materializados (alta confiança): terms, divider, quote,
# table, bibliography, cite
grep -n "Content::Terms\b\|Content::Divider\b\|Content::Quote\b\|Content::Table\b\|Content::Bibliography\b\|Content::Cite\b" \
  01_core/src/entities/content.rs

# Incertos: footnote, document, title, asset
grep -n "Content::Footnote\b\|Content::Document\b\|Content::Title\b\|Content::Asset\b" \
  01_core/src/entities/content.rs
```

**Critério**:
- 6/10 esperados materializados (terms, divider, quote, table,
  bibliography, cite).
- 1/10 incerto pendente verificação (footnote — desbloqueado
  por P156C mas materialização não confirmada).
- 3/10 prováveis ainda ausentes (document, title, asset — Fase
  3 condicional).

#### Listagem de todas as variants Content

```bash
# Contagem real de variants pós-todas as evoluções
grep -c "^\s*[A-Z][a-zA-Z]* *{" 01_core/src/entities/content.rs
grep "^\s*[A-Z][a-zA-Z]* *{" 01_core/src/entities/content.rs | head -60
```

**Esperado**: ≥54 variants (último contado P157B). Provavelmente
60+ pós-M9 + P199B (que adicionou SetEquationNumbering) +
outros.

### Cobertura hayagriva — DEBT-55 estado

```bash
# Hayagriva crate efectivamente usada?
grep -rn "hayagriva\|use hayagriva" 01_core/ 03_infra/
grep "hayagriva" Cargo.toml */Cargo.toml
```

**Critério**:
- Zero hits → ADR-0062 PROPOSTO mantém-se reserva sem ficheiro;
  Bloco B não materializado.
- Hits em `01_core/` → ADR-0062 deveria ter sido promovida a
  IMPLEMENTADO; verificar se transição foi documentada.

### Inconsistências documentais a esperar

Por precedente P255, é esperado encontrar:
- L0 prompts `entities/content.md` desactualizados vs enum real.
- README ADRs com contagens cumulativas obsoletas.
- DEBT-55 não actualizada desde P159A (análogo a DEBT-8).

---

## §4 — Cenários Fase B

### Cenário B1: Cobertura ≥75% confirmada (improvável)

Implicaria que muitos `parcial` subiram a `implementado` ou que
hayagriva foi materializado fora do que vejo no contexto. Pouco
provável.

Acção: relatório de fecho conceptual Model análogo a P255 fecho
DEBT-8. ADR-0060 mantém-se IMPLEMENTADO (sem promoção).

### Cenário B2: Cobertura 55-70% (provável)

Cenário **mais provável**. Reflecte:
- Bloco A (P157-P159G) materializado.
- Bloco B hayagriva NÃO materializado.
- Footnote possivelmente sim ou não.
- Fase 3 NÃO materializada.

Acção: documentar estado factual; identificar 1-3 sub-passos
prioritários:
- **Opção 1**: footnote materialização (se ausente). M; +5-10
  tests.
- **Opção 2**: ADR-0062 hayagriva promoção + bibliography
  refinos reais. L; +15-25 tests.
- **Opção 3**: refinos `parcial → implementado` para
  `list`/`enum`/`link`/`par`. S+ por feature; +20 tests
  cumulativos.

### Cenário B3: Cobertura ≤50% (regressão documental)

Implicaria que classificações declaradas estavam optimistas.
Possível mas improvável.

Acção: re-classificação conservadora; sub-passos de elevação
prioritários.

---

## §5 — Recomendação concreta

### Recomendação primária

**P256-aud — Fase A audit empírico** (XS; ~30 min de leitura
de código). Output: diagnóstico imutável análogo a
`diagnostico-math-fase-a-passo-255.md` com classificação literal
de cada uma das 22 entradas P154A.

### Recomendação secundária (pós-audit)

Depende do cenário Fase B confirmado:

- **B1 raro**: fecho conceptual; passar a outro módulo.
- **B2 provável**:
  - Se footnote ausente → P257 footnote materialização
    (priorização alta — feature visível user-facing).
  - Se prioridade for hayagriva → P257-X ADR-0062 promoção +
    bibliography hayagriva. Magnitude L; ADR transita
    PROPOSTO → IMPLEMENTADO.
- **B3 improvável**: re-classificação primeiro; materializações
  prioritárias depois.

### Recomendação terciária

**Refinos qualitativos `parcial → implementado`** para Model
`list`/`enum`/`link`/`par`. Cada um S+ standalone. Ganho de
cobertura agregada modesto (cada um +5pp), mas paridade
observable melhora.

### Não recomendado

- Atacar **Fase 3 condicional** (asset/document/title) sem
  audit prévio confirmar prioridade — Fase 3 declarada
  condicional em ADR-0060.
- Hayagriva integration **sem** ADR-0062 promovida via passo
  administrativo XS primeiro (padrão `ADR-0062-create` + P160A
  + P255.B).

---

## §6 — Padrões metodológicos aplicados

### ADR-0065 critério #5 — scope determinado por inventário

Aplicação directa. Este passo é diagnóstico-de-diagnóstico
análogo a P254A/P254B.

### Subpadrão "auditoria condicional" N=3

Cumulativo:
- **N=1** P192A (audit M7 fixpoint).
- **N=2** P255 (audit Math DEBT-8).
- **N=3 P256** (audit Model).

Patamar N=3 atinge limiar **formalização ADR meta**
candidato. Decisão de formalizar adiada (consistente com
política de aguardar N=4-5 para promoção formal).

### Subpadrão "diagnóstico imutável precedente à acção"

Cumulativo:
- **N=1** P255 (Fase A imutável).
- **N=2 P256** (este passo recomenda Fase A imutável análoga).

### Política "sem novas reservas"

Preservada. Recomendações §5 são para validação humana, não
compromissos.

---

## §7 — Limitações deste diagnóstico

1. **Cobertura agregada P159G "~50%" pode estar desactualizada**
   por refinos posteriores não auditados no contexto.

2. **Footnote materialização incerta** — desbloqueio P156C
   confirmado; materialização real depende de passo dedicado
   posterior cuja referência não está no contexto.

3. **Hayagriva integration incerta** — ADR-0062 PROPOSTO. Sem
   evidência de promoção a IMPLEMENTADO no contexto.

4. **Variants Content cumulativos** — última contagem
   confirmada N=54 (P157B). Pós-M3-M9 + P199B esperado N≥60
   mas sem contagem auditada.

5. **Refinos `parcial → implementado`** pós-P154A não auditados
   — pode haver evolução qualitativa não reflectida nos números
   agregados.

---

## §8 — Referências

- ADR-0017, ADR-0026, ADR-0033, ADR-0034, ADR-0038, ADR-0054,
  ADR-0060, ADR-0061, ADR-0062, ADR-0064, ADR-0065.
- DEBT-55 (bibliography+cite XL).
- P154A — diagnóstico Model original (origem das 22 entradas).
- P154B → P159G — Bloco A Model materializado.
- P156C — Layout Fase 1 desbloqueia footnote.
- P181D-H — Bibliography integrado com Introspector.
- P182C, P199B — Set*Numbering variants.
- P254A — precedente "actualização cumulativa de módulo".
- P254B → P255 — precedente "auditoria condicional Math".
