# Passo 256 — Relatório

**Data**: 2026-05-15
**Tipo**: passo arquitectural de diagnóstico
(**não materializa código**)
**Estrutura**: duas fases registadas (Fase A audit a executar +
Fase B decisão condicional).
**Motivação**: utilizador propôs "Model" como módulo seguinte
após P255 fechar DEBT-8 Math. Estado cumulativo do resumo
pós-P254 cita Model ~50%, mas esse número provém de P157A
(2026-04-26) e pode estar desactualizado.
**Curiosidade respondida**: Color mora em
`01_core/src/entities/layout_types.rs` (divergência histórica
da organização vanilla; ADR-0028 P25 colocou-a junto com tipos
de layout primitivos; ADR-0029 revogou a simplificação mas o
ficheiro nunca foi reorganizado).

---

## §1 Sumário executivo

1. **Diagnóstico Model preparado em duas fases** análogo a
   P254B (Math) seguido por P255 (Math executado). Fase A
   (audit empírico das 22 entradas P154A) recomendada como
   primeiro sub-passo executável; Fase B condicional aos
   resultados Fase A.

2. **Cenário mais provável B2** (cobertura 55-70%) — Bloco A
   Model materializado em série P154B → P159G (terms, divider,
   quote, table foundations, figure-kinds, bibliography+cite
   par acoplado); Bloco B hayagriva e Fase 3 condicional
   ausentes.

3. **Subpadrão "auditoria condicional" cresce N=3** —
   precedentes P192A + P255; este passo é terceira aplicação
   pré-execução. Atinge limiar formalização N=3 candidato.

4. **Inconsistências documentais esperadas** baseadas em
   precedente P255 (Math): L0 prompt `entities/content.md`
   provavelmente desactualizado; DEBT-55 provavelmente
   congelada desde P159G.

5. **Recomendação primária P257-aud**: executar checklist
   Fase A (~30-45 min); decidir cenário B1/B2/B3; materializar
   conforme cenário.

6. **Subpadrão "diagnóstico imutável precedente à acção" N=2**
   confirmado se Fase A for materializada como ficheiro
   imutável.

---

## §2 Curiosidade — onde mora `Color`

Resposta directa: **`01_core/src/entities/layout_types.rs`**.

**Representação actual**:
```rust
enum Color {
    Rgb { r, g, b: u8 },
    Rgba { r, g, b, a: u8 },
}
```

**Histórico**:
- **ADR-0028** (P25, 2026-03-29 — REVOGADO): colapsou Color
  num enum de 2 variantes minimalista; espaços de cor
  avançados (Oklab, CMYK, HSL) → `Value::None`.
- **ADR-0029** (P84.8c, 2026-04-22): revogou ADR-0028;
  estabeleceu "diagnosticar primeiro o vanilla antes de
  materializar"; **enumerou Color, ColorSpace, Gradient,
  Paint, Tiling como tipos tipográficos do vanilla**.

**Estado factual hoje**: representação P25 não-evoluída.
Tipos compostos relacionados (`ColorSpace`, `Gradient`,
`Paint`, `Tiling`, `Stroke<T>`) **não materializados** per
ADR-0029 §enumeração. Pertencem a domínio **Visualize**, não
Model. Não bloqueia Model.

**Funções nativas** (em `01_core/src/rules/stdlib*`):
- `native_rgb(r, g, b)` ou `(r, g, b, a)` → `Value::Color`.
- `native_luma(l)` → `Color::Rgb(l, l, l)` (escala cinza).

**Consumidores actuais**:
- `entities/geometry.rs` — `Stroke { paint: Color, thickness }`.
- `entities/style.rs` — `Style::Fill(Color)`.
- `FrameItem::Text { fill: Color, ... }` (assumido).

**Divergência cosmética**: Color em `layout_types.rs` em vez
de ficheiro dedicado `entities/color.rs`. Sem ganho observable
em mover; candidato informal a refactor cosmético sem
prioridade.

---

## §3 Artefactos produzidos

| Ficheiro | Localização canónica | Conteúdo |
|----------|----------------------|----------|
| `diagnostico-model-passo-256.md` | `00_nucleo/diagnosticos/` | Diagnóstico principal §1-§8 com inventário declarado P154A, evolução cumulativa, cenários Fase B condicionais, recomendação |
| `fase-a-checklist-model-passo-256.md` | `00_nucleo/diagnosticos/` | Checklist executável Fase A: 7 blocos de comandos `grep`/`view`; tabela 22 entradas para preencher; templates Fase B para cenários B1/B2/B3 |
| `typst-passo-256-relatorio.md` | `00_nucleo/materialization/` | Este ficheiro |

---

## §4 Padrões metodológicos aplicados

### ADR-0065 critério #5 — scope determinado por inventário

Aplicação directa. Diagnóstico precede materialização.

### Subpadrão "auditoria condicional"

Patamar N=3 cumulativo:
- **N=1** P192A — audit M7 fixpoint runtime.
- **N=2** P255 — audit Math DEBT-8.
- **N=3** P256 (este passo) — recomenda audit Model análogo.

**N=3 atinge limiar candidato a formalização ADR meta**.
Decisão de formalizar adiada — política consistente N=4-5
para promoção formal de subpadrão.

### Subpadrão "diagnóstico imutável precedente à acção"

Patamar N=1 (P255) + N=1 (este passo recomenda) = **N=2
cumulativo se Fase A P257-aud materializar**.

### Política "sem novas reservas"

Preservada. Recomendações §5 são para validação humana;
recomendações condicionais (Opções 1/2/3 cenário B2) não são
compromissos.

---

## §5 Estado cumulativo pós-P256

### Sem alteração

- Tests: 2304 verdes (passo documental).
- Hashes L0: preservados.
- ADRs: distribuição inalterada.
- DEBTs: contagem inalterada — DEBT-55 mantém estado actual
  (provavelmente desactualizado mas auditoria adiada para
  P257-aud).
- 46 aplicações cumulativas anti-inflação preservadas.

### Alteração metodológica

- **Subpadrão "auditoria condicional" N=2 → N=3** cumulativo
  (P256 recomenda terceira aplicação).
- **Subpadrão "diagnóstico imutável precedente à acção" N=1 →
  N=2** se P257-aud materializar.
- **Curiosidade Color respondida** — documentação cumulativa
  ganha mais um achado factual.

---

## §6 Próximos passos sugeridos

### Sequência primária

1. **P257-aud** (XS; ~30-45 min): executar checklist Fase A
   conforme `fase-a-checklist-model-passo-256.md`. Output:
   tabela 22 entradas preenchida com hits literais; decisão
   B1/B2/B3 tomada com base em evidência factual.

2. **P257-doc** (XS; ~15-30 min; pode ser paralelo a P257-aud):
   actualizar L0 prompts obsoletos descobertos durante audit
   (esperado pelo padrão P255-doc).

3. **P257-fix** ou **P258+** (magnitude variável conforme
   decisão):
   - **B1**: fecho conceptual + relatório.
   - **B2 Opção 1**: footnote materialização — M; +10-15
     tests; +5pp Model.
   - **B2 Opção 2**: ADR-0062 hayagriva promoção +
     bibliography hayagriva — L; +20-30 tests; +10pp.
   - **B2 Opção 3**: refinos parcial→implementado — S+ por
     feature; cumulativo +15-20pp.

### Sequência alternativa

Se prioridade for outro módulo após responder à curiosidade
Color:
- **Visualize** (~54%) — implicaria materializar `Color`
  composto (ColorSpace, Gradient, Paint, Stroke<T>, Tiling)
  + shape primitives + paths + curves. Magnitude alta.
- **Text** (~52%) — implicaria StyleChain refino + DEBT-53
  rustybuzz real. Magnitude alta.

---

## §7 Decisões registadas

1. **Color enumerada em §enumeração ADR-0029 como Visualize
   não Model** — Color, Gradient, Paint, Stroke pertencem ao
   domínio visualize. Materialização desses tipos não é
   pré-requisito para evolução Model.

2. **P256 é diagnóstico-de-diagnóstico** (análogo P254B Math)
   — não executa audit; recomenda audit como P257-aud
   subsequente.

3. **Cenário mais provável B2** com Opções 1/2/3 não
   exclusivas — Opção 1 (footnote) é candidata mais imediata
   por ser feature visível user-facing e ter pré-requisito
   já desbloqueado (P156C).

4. **Hayagriva integration (Opção 2)** é decisão arquitectural
   maior — requer ADR-0062 promoção formal antes de qualquer
   código.

---

## §8 Limitações deste diagnóstico

1. **Cobertura agregada P159G "~50%" não confirmada por audit**
   — número declarado em P157A pode estar desactualizado para
   cima (refinos posteriores) ou para baixo (re-classificação
   conservadora durante audit).

2. **Materialização de footnote incerta** — desbloqueio P156C
   confirmado mas materialização não atestada no contexto.

3. **Hayagriva status incerto** — ADR-0062 PROPOSTO em
   2026-04-25. Sem confirmação de promoção a IMPLEMENTADO.

4. **Variants Content cumulativos** — última contagem audited
   N=54 (P157B). Pós-M3-M9 + P199B + outros esperado N≥60
   sem confirmação factual.

5. **Refinos `parcial → implementado` pós-P154A não auditados**
   — possível evolução qualitativa não reflectida nos números
   declarados.

---

## §9 Referências

- Diagnóstico principal:
  `diagnostico-model-passo-256.md`.
- Checklist Fase A: `fase-a-checklist-model-passo-256.md`.
- ADRs: 0017, 0026, 0033, 0034, 0038, 0054, 0060, 0061,
  0062, 0064, 0065.
- DEBT-55 (bibliography+cite XL).
- P154A — diagnóstico Model original.
- P154B → P159G — Bloco A Model materializado.
- P156C — Layout Fase 1 desbloqueia footnote.
- P181D-H — Bibliography integrado com Introspector.
- P182C, P199B — Set*Numbering variants.
- P254A — precedente "actualização cumulativa de módulo"
  (Introspection).
- P254B → P255 — precedente "auditoria condicional Math"
  (executada com sucesso; subpadrão N=2).
- P192A — precedente N=1 "auditoria condicional".
