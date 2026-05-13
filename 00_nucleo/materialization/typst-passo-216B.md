# Passo 216B — Sub-fase (a) parte 2: `Regions` wrapper minimal + `Layouter::with_regions`

**Série**: 216 (segundo sub-passo materialização Layout
Fase 3; parte B de sub-fase (a) decomposta per P215.div-1).
**Marco**: nenhum (quinto passo pós-M9c; segunda
materialização DEBT-56; fecha sub-fase (a) DEBT-56).
**Tipo**: refactor estrutural **sem mudança observable** —
preserva todos os tests workspace como regression suite.
**Magnitude**: M (~2-3h).
**Pré-condição**: P216A concluído (`Region` tipo em
`entities/region.rs`; Layouter refactored para
`region: Region`; 1943 tests verdes); ADR-0078 PROPOSTO
§"Plano materialização" sub-fase (a) parte 2 documentada;
ADR-0078 anotado com P216A em §"Plano de materialização";
humano fixou continuação Caminho 1.
**Output**: 1 ficheiro relatório curto + código alterado +
extensão L0 `entities/region.md` (sem novo ficheiro L0;
`Regions` co-habita com `Region`) + ADR-0078 anotada
(sem transição de status).

---

## §1 Trabalho

P216A agregou 7 fields dispersos do Layouter num `Region`
único. P216B introduz `Regions` wrapper como struct
intermediária preparatória para sub-fase (b) — consumer
multi-column real (P219). `Regions` em P216B **é minimal**:
contém apenas `current: Region` (single-region preservado).

**Decisão arquitectural central (anti-inflação 11ª aplicação)**:

Estrutura proposta em ADR-0078 e §8 do relatório P216A era
forma rica vanilla:

```rust
pub struct Regions {
    pub current: Region,
    pub backlog: Vec<Region>,
    pub last:    Option<Region>,
}
```

**P216B rejeita** essa forma e fixa minimal:

```rust
pub struct Regions {
    pub current: Region,
}
```

**Justificação literal anti-inflação**:

- `backlog: Vec<Region>` só faz sentido quando há **múltiplas
  regions sequenciais** (multi-column). Em single-column
  (single-region) está sempre vazio.
- `last: Option<Region>` só faz sentido para algoritmos de
  break/overflow que consultam "última region completa".
  Em single-column é sempre `None`.
- **Zero consumers reais** de `backlog` e `last` em P216B
  (consumer único `Layouter` continua single-region).
- Precedente P205D — `SealedLabelPages` foi deferido
  (Caminho B) por "tracking de informação já tracked
  por outra rota" sem consumer real. P216B aplica mesmo
  princípio a `Regions` rica.

`backlog` e `last` adicionados em **P219** (consumer
multi-column real) quando emergir necessidade. Critério de
reabertura: materialização de `Content::Columns` consumer
no Layouter.

**Decisão central de P216B**: introduzir `Regions { current:
Region }` minimal; refactor `Layouter` para usar `regions:
Regions` em vez de `region: Region` (mecânico ~5-15
call-sites); preparar pre-condição estrutural para P219.

**Decisão alternativa rejeitada**: skip P216B; ir directo a
P217 (`Content::Columns` variant) com `region: Region`
preservado. Rejeitada porque P217+P218+P219 cumulativos
introduzem `Regions` em sub-passo onde também aparece
consumer multi-column — viola atomização ADR-0036.
Separação P216B (estrutura) + P219 (consumer) é mais clara.

Reuso de dados (sem recolha nova):

- P216A `Region` tipo estabelecido.
- ADR-0078 PROPOSTO §"Decisão" `Regions` estrutura paridade
  vanilla.
- ADR-0078 anotada com P216A.
- Pattern P205D anti-inflação como precedente literal.

---

## §2 Cláusulas (8)

### C1 — Inventário call-sites `self.region` pós-P216A

Confirmar empiricamente quantos sítios serão afectados pelo
refactor `self.region` → `self.regions.current`:

```
grep -c "self\.region\." 01_core/src/rules/layout/mod.rs
grep -c "self\.region\." 01_core/src/rules/layout/cursor.rs
grep -c "self\.region\." 01_core/src/rules/layout/equation.rs
grep -c "self\.region\." 01_core/src/rules/layout/grid.rs
grep -c "self\.region\." 01_core/src/rules/layout/placement.rs
grep -c "self\.region\." 01_core/src/rules/layout/tests.rs
```

Estimativa P215.div-1: ~30-40 call-sites adicionais. P216A
real foi ~167 (substituições `self.cursor_x` → `self.region.cursor_x`
etc.); P216B fará nova camada de indirecção sobre todos eles
(`self.region` → `self.regions.current`).

**Distinção crítica**: P216A criou 167 sítios `self.region.X`.
P216B refactor é literal: cada `self.region.X` →
`self.regions.current.X`. **Mesmo número de call-sites**
afectados, mas mecânica é simples (substituição uniforme).

Hipótese provável: ~167 substituições (paridade P216A em
volume; trivial em mecânica). Se contagem divergir > 10%:
registar `P216B.div-1`.

### C2 — Estender `entities/region.rs` com `Regions` minimal

Editar `01_core/src/entities/region.rs` adicionando struct
`Regions` ao mesmo ficheiro (cohabitação semântica):

```rust
/// Wrapper sobre regions sequenciais.
///
/// Introduzido em P216B (DEBT-56 sub-fase (a) parte 2) para
/// preparar consumer multi-column em P219. Forma minimal por
/// anti-inflação (11ª aplicação cumulativa pós-P205D) — apenas
/// `current` field. Fields `backlog`/`last` adicionados em P219
/// quando emergir consumer real.
///
/// Paridade vanilla simplificada per ADR-0078 PROPOSTO §"Decisão":
/// vanilla `Regions<'a> { current, backlog: &'a [Abs], last,
/// expand, full, ... }`; cristalino reduz a 1 field até consumer
/// emergir.
#[derive(Debug, Clone)]
pub struct Regions {
    /// Region actual onde Layouter escreve. Single-region em
    /// P216B; multi-region em P219.
    pub current: Region,
}

impl Regions {
    /// Cria `Regions` com 1 region de dimensões dadas.
    pub fn single(width: f64, height: f64) -> Self {
        Self {
            current: Region::new(width, height),
        }
    }

    /// Reset region actual (delega a `Region::reset`).
    pub fn reset_current(&mut self) {
        self.current.reset();
    }
}
```

Re-export em `01_core/src/entities/mod.rs`:

```rust
pub use region::{Region, Regions};
```

Sentinelas mínimas (3 tests unitários em
`region.rs::tests`, adicionados aos 4 P216A):

- `p216b_regions_single_cria_current_com_dimensoes`.
- `p216b_regions_reset_current_delega`.
- `p216b_regions_clone_funciona`.

Total tests pós-P216B em `region.rs::tests`: 4 (P216A) + 3
(P216B) = **7 tests** unitários.

Magnitude: XS isolada (~10min).

### C3 — Actualizar L0 `entities/region.md`

Editar `00_nucleo/prompts/entities/region.md` adicionando
secção `Regions`:

- Histórico: anotar "2026-05-12 (P216B adição — `Regions`
  wrapper minimal; DEBT-56 sub-fase (a) parte 2)".
- Nova secção `## Struct `Regions`` (paridade estrutural
  com secção `Region`).
- Campos: 1 (current).
- Métodos: `single`, `reset_current`.
- Pattern: anti-inflação 11ª aplicação — `backlog`/`last`
  diferidos.
- Consumers: P216B Layouter (`regions: Regions` em vez de
  `region: Region`); P219 (consumer multi-column real).

Hash propagado via `crystalline-lint --fix-hashes`.

### C4 — Refactor `Layouter` struct: `region` → `regions`

Editar `01_core/src/rules/layout/mod.rs`:

**Antes (P216A field)**:
```rust
pub(super) region: crate::entities::region::Region,
```

**Depois (P216B field)**:
```rust
/// P216B: agregação em `Regions` wrapper (single-region em
/// P216B; multi-region em P219 sub-fase (b) consumer).
pub(super) regions: crate::entities::region::Regions,
```

**`Layouter::new` ajuste**:
```rust
regions: {
    let mut r = Regions::single(cfg.width, cfg.height);
    r.current.cursor_x = Pt(cfg.margin);
    r.current.cursor_y = Pt(cfg.margin) + ascender;
    r.current.line_start_x = Pt(cfg.margin);
    r
},
```

### C5 — Substituição mecânica de call-sites

Refactor mecânico literal:

| Antes (P216A) | Depois (P216B) |
|----------------|------------------|
| `self.region.cursor_x` | `self.regions.current.cursor_x` |
| `self.region.cursor_y` | `self.regions.current.cursor_y` |
| `self.region.line_start_x` | `self.regions.current.line_start_x` |
| `self.region.current_items` | `self.regions.current.current_items` |
| `self.region.current_line` | `self.regions.current.current_line` |
| `self.region.width` | `self.regions.current.width` |
| `self.region.height` | `self.regions.current.height` |
| `self.region.has_pending()` | `self.regions.current.has_pending()` |
| `self.region.reset()` | `self.regions.current.reset()` (ou helper `reset_current()`) |

Substituição em todos os `01_core/src/rules/layout/*.rs`
(mesmos 6 ficheiros de P216A; mesmos ~167 sítios).

Ferramenta: `sed` com pattern
`s/self\.region\./self.regions.current./g`. Verificação
manual obrigatória após cada ficheiro.

**Sítio especial**: ajuste manual em `Content::SetPage` arm
(`mod.rs:761-763`) que foi adicionado em P216A para
sincronizar `region.width/height`. Em P216B, sincronização
torna-se `self.regions.current.width = ...` etc. (mecânico).

**Pattern adicional emergente em P216B**: substituições uniformes
sobre output de refactor anterior (cadeia P216A → P216B). Possível
documentar como subpadrão "refactor stacking" se P216C/P217 também
fizer camada adicional.

### C6 — Verificação tests workspace

Critério **rígido** (paridade P216A): tests pré-P216B
verdes pós-P216B; sem regressão observable.

```
cargo test --workspace 2>&1 | tail -20
```

Esperado: **1943 + 3 sentinelas P216B = 1946 verdes**.

**Erro tolerado**: zero. Qualquer test red indica regressão.

Hipótese provável: 1946 verdes. Refactor sem helpers
(Opção α P216A preservada) mantém mecânica simples.

### C7 — Verificação lint + ADR-0078 anotação

```
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 0 violations. Hash propagado em `entities/region.md`
(L0) + `entities/region.rs` (L1).

Editar ADR-0078 §"Plano de materialização" anotando bloco
**`### P216B materializado 2026-05-12`**:

```markdown
Sub-fase (a) parte 2 fechada:
- Struct `Regions` adicionada em `entities/region.rs`
  cohabitando com `Region` (mesmo ficheiro; mesma L0).
- Forma minimal anti-inflação (11ª aplicação cumulativa
  pós-P205D): apenas `current: Region`. Fields `backlog`
  + `last` diferidos a P219 (consumer multi-column).
- Layouter struct refactored: `region: Region` → `regions:
  Regions`. ~167 call-sites refactored mecânicamente
  (`self.region.X` → `self.regions.current.X`).
- 0 mudança observable (1946 tests verdes = 1943 + 3
  sentinelas P216B).

Sub-fase (a) DEBT-56 fechada estruturalmente em P216B.
Próximo: P217 `Content::Columns` variant (sem refactor
Layouter; aditivo puro).
```

**Status ADR-0078**: PROPOSTO mantido. Transição IMPLEMENTADO
só em P221 (encerramento Fase 3).

### C8 — Decisão sobre próximo trabalho

P216B fecha sub-fase (a) inteira de DEBT-56. Decisão humana
sobre próxima sessão:

- **Caminho 1** — prosseguir P217 imediatamente
  (`Content::Columns` variant + arms exhaustivos; aditivo).
  Magnitude S+ (~1.5h); zero refactor estrutural.
- **Caminho 2** — pivot para Bloco C opcional P222
  (`measure(body)` stdlib expose; isolado, não bloqueia
  DEBT-56). S+ (~1-2h). Win rápido §A.9 estricto 83% →
  100%.
- **Caminho 3** — adiar Layout; voltar a outro módulo.

Hipótese provável humano fixou Caminho 1 ("focar no Layout
até onde der") — sugere prosseguir P217 imediatamente
pós-P216B. Mas fica em aberto literal.

---

## §3 Output

1 ficheiro relatório:
`00_nucleo/materialization/typst-passo-216B-relatorio.md`.

Estrutura (~5-7 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Confirmação inventário call-sites pós-P216A (C1).
- §3 `Regions` minimal adicionado a `region.rs` (campos +
  sentinelas; anti-inflação 11ª aplicação documentada).
- §4 Layouter refactor `region` → `regions.current` (C4 +
  C5 contagem real).
- §5 Decisões substantivas (forma minimal vs rica
  vanilla; cohabitação L0; pattern "refactor stacking").
- §6 Resultados verificação (tests + lint).
- §7 ADR-0078 anotação P216B + sub-fase (a) fechada
  estruturalmente.
- §8 Próximo passo (Caminho 1 P217 ou Caminho 2 P222;
  decisão humana).

Código alterado:
- **Editado**: `01_core/src/entities/region.rs` (+ ~30 LOC
  struct `Regions` + 2 métodos + 3 sentinelas).
- **Editado**: `01_core/src/entities/mod.rs` (re-export ajuste).
- **Editado**: `01_core/src/rules/layout/mod.rs` (~167
  substituições `self.region.X` → `self.regions.current.X`).
- **Editado**: `01_core/src/rules/layout/{cursor,equation,grid,
  placement,tests}.rs` (substituições mecânicas).
- **Editado**: `00_nucleo/prompts/entities/region.md` (+ secção
  `Regions`).
- **Editado**: `00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md`
  (+ anotação P216B).

**Sem novos ficheiros**. `Regions` cohabita com `Region` no
mesmo `region.rs` por coesão semântica (ambos descrevem o
mesmo conceito; struct + wrapper).

---

## §4 Não-objectivos

- Adicionar fields `backlog: Vec<Region>` ou `last:
  Option<Region>` ao `Regions` — diferidos a P219 (anti-inflação
  11ª aplicação).
- Materializar `Content::Columns` variant — diferido a P217.
- Materializar `native_columns` stdlib — diferido a P218.
- Consumer multi-column no Layouter — diferido a P219 (sub-fase
  b).
- `Content::Colbreak` — diferido a P220.
- Adicionar helpers `Layouter::with_regions` standalone — não
  necessário em P216B (single-region preservado).
- Adicionar helpers field-projection `Layouter::current_region()`
  etc. — Opção α anti-inflação preservada de P216A.
- Promover ADR-0078 PROPOSTO → IMPLEMENTADO — só após P221.
- Fechar DEBT-56 — só após P221 (sub-fase b consumer multi-column
  + Content::Columns/Colbreak).
- Mudar comportamento observable.
- Tocar em código fora de `01_core/src/rules/layout/*.rs` e
  `01_core/src/entities/`.

---

## §5 Riscos a evitar

1. **Inflar `Regions` com `backlog`/`last` em P216B**: tentação
   óbvia per "paridade vanilla literal". Rejeitada — 11ª
   aplicação anti-inflação cumulativa pós-P205D. Adição quando
   consumer real emergir (P219).
2. **Borrow checker quebras renovadas**: P216A reportou 0
   quebras; mas refactor `self.region.X` → `self.regions.current.X`
   é nova camada. Risco baixo dado precedente P216A, mas
   atenção a sítios com dois acessos.
3. **Confundir P216B (estrutura) vs P219 (consumer)**: P216B
   só adiciona `Regions` wrapper minimal. Single-region
   preservado. Consumer multi-column é P219.
4. **L0 separado para `Regions`**: tentação de criar
   `entities/regions.md` novo. Rejeitada — `Regions` é wrapper
   trivial sobre `Region`; cohabita no mesmo L0 por coesão
   semântica (precedente: `Sides<T>` em sides.md cobre struct +
   helpers).
5. **Helper `Layouter::with_regions` standalone**: spec original
   P215 §C4 sugeriu helper. P216B descobre que helper não é
   necessário em single-region preservado; helper emerge em
   P219 com consumer real. Documentar como decisão Opção α
   reforçada.
6. **Mudança observable acidental**: P216A teve 1 ajuste manual
   em `Content::SetPage` arm para sincronizar dimensões. P216B
   herda essa sincronização mecânicamente (substituição uniforme);
   sem novo ajuste esperado.
7. **Refactor stacking pattern emergente**: P216A criou camada;
   P216B adiciona nova camada sobre. Risco de fadiga
   cognitiva — sítios com `self.region.cursor_x` (P216A) viram
   `self.regions.current.cursor_x` (P216B). Mitigação: aceitar
   verbosidade per Opção α; helpers em P219 se útil.
8. **Tests workspace red**: critério rígido — 1946 verdes
   esperados. Qualquer red indica refactor incorrecto; rollback
   parcial possível.
9. **Subpadrão "refactor stacking" prematuro**: P216B é primeira
   aplicação possível. Não promover a pattern formalizado
   (precisaria N=2-3); apenas registar como observação no
   relatório §5.
10. **`reset_current` helper conveniente vs `regions.current.reset()`
    directo**: P216B inclui `reset_current` por simetria com
    métodos de Region. Manter — utilidade não-conflituosa com
    Opção α (anti-inflação aplica-se a helpers no `Layouter`,
    não em tipos L1).

---

## §6 Hipótese provável

C1 confirmará ~167 call-sites `self.region.X` pós-P216A
(paridade P216A inventário).

C2 estenderá `region.rs` com `Regions { current: Region }`
minimal + 2 métodos + 3 sentinelas em ~30 LOC.

C3 actualizará L0 `region.md` com secção `Regions`; hashes
propagados.

C4 refactor Layouter struct `region` → `regions: Regions`
em ~3 linhas (struct + Layouter::new).

C5 substituição mecânica ~167 sítios em 6 ficheiros (sed
uniforme; possível 1-3 ajustes manuais).

C6 reportará 1946 tests verdes (1943 P216A + 3 sentinelas
P216B).

C7 reportará 0 violations; ADR-0078 anotada cumulativamente.

C8 listará Caminho 1 (P217 imediatamente) + Caminho 2 (P222
pivot Bloco C) + Caminho 3 (adiar Layout); hipótese provável
Caminho 1 per orientação humano.

Custo real: M (~2-3h). Maior parcela em C5 (refactor mecânico +
verificação tests) + C7 (lint + ADR cumulativo). Mais simples
que P216A porque mecânica é substituição uniforme sobre
output já agregado.

Mas é hipótese, não decisão. C1-C8 fixam-se empíricamente.

---

## §7 Particularidade P216B

P216B é estruturalmente distinto na trajectória pós-M9c:

- **Segundo refactor sem mudança observable consecutivo** —
  P216A + P216B formam série DEBT-56 sub-fase (a). Tests
  existentes como regression suite preservados rigidamente
  em ambos.
- **11ª aplicação cumulativa anti-inflação** pós-P205D —
  `backlog`/`last` em `Regions` diferidos por falta de
  consumer real. Pattern P205D replicado literal: estrutura
  rica diferida até consumer emergir.
- **Pattern emergente "refactor stacking" N=1** — P216B
  refactora output de P216A (`self.region.X` → `self.regions.current.X`).
  Possível pattern N=2 se P216C ou P217 também stack sobre
  P216B. Promoção a meta diferida (N=3-4 política consistente).
- **Cohabitação L0 N=2** — `Region` + `Regions` no mesmo
  `entities/region.rs` + mesmo L0 `region.md`. Precedente
  `Sides<T>` em sides.md (struct + métodos cohabitam).
  Pattern preservado em P216B; sem inflação documental.
- **Fecho estrutural sub-fase (a) DEBT-56** — após P216B,
  sub-fase (a) completa. P217-P219 são sub-fase (b) e
  features novas. Marco interno (não marco arquitectónico
  formal — DEBT-56 só fecha em P221).
- **Cumulativo P216A+P216B = ~334 substituições mecânicas
  em 2 sessões** sem mudança observable. Demonstra
  viabilidade do pattern "decomposição empírica de
  magnitude" (P215.div-1) — sub-fases pequenas tractáveis
  vs big-bang refactor L+ original.

Por isso §5 risco 1 (inflar `Regions`) é o mais relevante.
Tentação óbvia é "paridade vanilla literal" mas precedente
P205D + 10 aplicações cumulativas anti-inflação fixam
expectativa: estrutura minimal até consumer emergir.
