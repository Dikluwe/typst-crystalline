# Passo 149 — Relatório (arqueologia + formalização de `Value::Type` e `Value::Args`)

**Data**: 2026-04-25
**Natureza**: passo **L0-puro / administrativo / arqueológico**.
**Zero código**. **Zero testes**. **2 ADRs criadas** (0058 +
0059); **0 DEBTs abertos**. **Inventário 148 actualizado**.
**Precondição**: Passo 148 encerrado; inventário de cobertura
produzido; 1113 tests; zero violations; 57 ADRs.

---

## 1. Sumário executivo

Inventário 148 §7 sinalizou 2 divergências arquitecturais entre
`Value` cristalino e `Value` vanilla sem ADR/DEBT canónica:

1. **`Value::Type`**: cristalino devolve `Value::Str(type_name)`
   via `native_type`; vanilla tem `Value::Type(Type)` com tipo
   rico (scope + métodos + docstring).
2. **`Value::Args`**: cristalino tem `Args` como struct
   separada em `entities/args.rs`, passado como `&Args` às
   nativas; vanilla tem `Value::Args(Args)` como variant.

**Arqueologia** localizou:
- Passo 13 (materialização inicial do enum `Value`) —
  `Type(Type)` e `Args(Args)` **deixadas comentadas** como
  variantes futuras; comentário invoca ADR-0017.
- Passo 16 (funções nativas) — `Args` struct materializado
  em `entities/args.rs` como input vehicle.

**Classificação**: ambas as divergências são **decisões
conscientes** (ADR-0017 explícita; ADR-0036 implícita via
espírito de atomização). Formalização via ADR, não DEBT — não
há plano de migração nem tests a falhar.

**Outputs**:
- `00_nucleo/diagnosticos/divergencias-value-passo-149.md`
  (arqueologia factual).
- `00_nucleo/adr/typst-adr-0058-value-type-simplificado.md`
  (status `EM VIGOR`).
- `00_nucleo/adr/typst-adr-0059-args-tipo-separado.md`
  (status `EM VIGOR`).
- Inventário 148 actualizado (Tabela B + §7 Top divergências;
  cobertura arquitectural 70% → 72%).
- README dos ADRs actualizado (total 57 → 59; distribuição;
  Passos-chave).
- §9 de `typst-paridade-plano-medicao.md` renumerado (passo
  150 = `frame_dto.rs`; era 149).

**Tests**: inalterados em 1113 (zero código tocado).

---

## 2. Inventário factual (sub-passo 149.1)

### 2.1 — `Value::Type`

- **Cristalino**: variant **ausente** do enum `Value`
  (`01_core/src/entities/value.rs:82` — comentada).
- **Substituto**: `Value::type_name() -> &'static str` +
  `native_type(args)` devolve `Value::Str(type_name)`.
- **Vanilla**: `Value::Type(Type)` em
  `foundations/value.rs`; `Type` é struct rico com scope de
  métodos, docstring, construtores.

### 2.2 — `Value::Args`

- **Cristalino**: variant **ausente** do enum `Value`
  (`value.rs:81` — comentada). `Args` é struct separada em
  `01_core/src/entities/args.rs` com `items: Vec<Value>` +
  `named: IndexMap<...>`, passada como `&Args` a funções
  nativas.
- **Vanilla**: `Value::Args(Args)` como variant; `Args` pode
  ser armazenado em variáveis Typst.

### 2.3 — Correcção do inventário 148

A Tabela B do inventário 148 classificava ambas como
`parcial`. Inspecção empírica revelou que:

- **Não são "parcial"** — são decisões explicitamente
  omitidas do enum (comentário no código).
- **São divergências arquitecturais** face ao vanilla,
  sustentadas por tempo (133 passos) e por comportamento
  observável correcto (1113 tests).

Pós-149: reclassificadas para `implementado⁺` (divergência
intencional documentada por ADR).

---

## 3. Arqueologia (sub-passo 149.2)

### 3.1 — `Value::Type`: Passo 13 + ADR-0017

Enunciado `00_nucleo/materialization/typst-passo-13.md`
linhas 169–174 documenta o enum inicial com `Type(Type)` e
`Args(Args)` **comentados** como "variantes futuras". Razão
registada no próprio `value.rs` (linhas 14–16):

> Subset de Passo 15: 9 variantes (5 primitivos + Array, Dict,
> Module, Datetime). As restantes (~21) são adicionadas quando
> os tipos dependentes migrarem para L1. Não adicionar
> variantes sem ADR e tipo migrado. **Ver ADR-0017.**

**ADR-0017** (Passo 6) formaliza a estratégia geral de
"adiamento de eval() e estratégia typst-library": tipos
migram progressivamente conforme os dependentes entrarem em
L1. `Type` exigiria materializar `typst-library::ty` (scope
+ métodos + docstring). Não priorizado em 133 passos.

**Substituto funcional** (Passos 13–14): `v.type_name() ->
&'static str` + `native_type` devolve `Value::Str(type_name)`.
Permite `if type(x) == "int" { … }` idiomatic. Teste unitário
directo em `01_core/src/rules/stdlib/mod.rs:149-152`.

### 3.2 — `Value::Args`: Passo 16 + ADR-0036 (implícita)

Passo 16 introduziu funções nativas com invocação via
`Args`. Passo não tem registo explícito da decisão de excluir
`Args` do enum `Value`.

**Razão inferida** (sem passo/ADR explícito que justifique):

1. **Args é input vehicle, não valor**: nas assinaturas de
   `native_*`, `Args` é parâmetro; o resultado é `Value`. Raro
   armazenar `Args` em variável.
2. **ADR-0036 (atomização progressiva)** espírito: funções
   declaram todas as dependências explicitamente. `&Args` na
   assinatura é mais explícito do que `Value::Args(...)`.
3. **Evita bloat do Value enum**: 18 variants é significativo;
   adicionar uma pouco-usada piora match exaustivo em ~20
   consumidores.
4. **Sem código que peça `Value::Args`**: eval passa `Args`
   directo; stdlib recebe-o directo; nunca empacotado.

Esta é **decisão de facto** consistente por construção ao
longo de ~120 passos.

### 3.3 — Tabela comparativa

| Divergência | Passo | Razão registada | Razão inferida |
|-------------|-------|-----------------|----------------|
| `Value::Type` | 13 (comentada); 14 (substituto) | ADR-0017 (adiamento) + comentário value.rs:14-16 | subset MVP; `Type` rico fora do priorizado |
| `Value::Args` | 13 (comentada); 16 (struct materializado) | nenhuma explícita | ADR-0036 (atomização); input vehicle; evita bloat |

---

## 4. Decisão de classificação (sub-passo 149.3)

Critério aplicado:

| Encontrámos | Classificação |
|-------------|---------------|
| Razão explícita "decisão consciente, forma final" | **ADR** |
| Sem razão registada + comportamento estável + tests OK | **ADR** (formaliza decisão de facto) |

### 4.1 — `Value::Type` → **ADR-0058**

Critério primário: "razão explícita" (ADR-0017 + comentário
no código); 1113 tests verdes; forma estável. **ADR
formaliza; zero DEBT**.

- **ADR-0058** "Tipo simplificado — `type()` devolve
  `Value::Str` em vez de `Value::Type(Type)`".
- Status: `EM VIGOR`.
- Plano condicional: se métodos-sobre-tipos forem
  priorizados futuramente, abrir passo dedicado (escopo
  M-L). ADR-0058 não bloqueia migração; formaliza a forma
  actual.

### 4.2 — `Value::Args` → **ADR-0059**

Critério primário: "sem razão registada + estável + tests OK";
razão inferida consistente com ADR-0036. **ADR formaliza
decisão de facto; zero DEBT**.

- **ADR-0059** "`Args` como tipo separado, não-variant de
  `Value`".
- Status: `EM VIGOR`.
- Plano condicional: se feature futura exigir `Args` como
  valor primeira-classe, abrir passo dedicado (escopo S-M).

### 4.3 — Resumo de outputs

- **2 ADRs** criadas.
- **0 DEBTs** abertos.
- **0 referências cruzadas apenas** (ambas justificaram
  ADR).

---

## 5. Outputs criados (sub-passo 149.4)

### 5.1 — Documento arqueológico

`00_nucleo/diagnosticos/divergencias-value-passo-149.md` —
factual, 4 secções (inventário, arqueologia, decisão, notas).

### 5.2 — ADRs

Ambas com cabeçalho canónico P145 (`**Status**: \`EM VIGOR\``,
título com `⚖️`, `**Validado**: Passo 149 — …`):

- `00_nucleo/adr/typst-adr-0058-value-type-simplificado.md`
  (~130 linhas: Contexto, Decisão, Alternativas (tabela),
  Consequências positivas/negativas/neutras, Plano de
  evolução, Referências).
- `00_nucleo/adr/typst-adr-0059-args-tipo-separado.md`
  (~130 linhas; mesma estrutura).

### 5.3 — Sem DEBTs

Nenhuma DEBT aberta. `DEBT-53` (referenciado como candidato
em relatórios anteriores mas não aberto) **continua
disponível** para uso futuro.

---

## 6. Inventário 148 actualizado (sub-passo 149.5)

**Tabela B** (arquitectural) — linhas de `Value::Type` e
`Value::Args`:

```diff
- | `Args` | ... | `parcial` | tipo separado | `Args` em entities; não Value variant |
- | `Type` | ... | `parcial` | string-based | `type()` devolve string, não Value::Type |
+ | `Args` | ... | `implementado⁺` | Passo 16; **ADR-0059** (P149) | divergência intencional: ... |
+ | `Type` | ... | `implementado⁺` | Passo 13-14; **ADR-0058** (P149) | divergência intencional: ... |
```

**Resumo agregado** actualizado:

```diff
- | `Value` variants | 18 | 0 | 4 | 9 | 0 | 31 |
+ | `Value` variants | 18 | 2 | 2 | 9 | 0 | 31 |
- | **Total arquitectural** | **63** | **11** | **7** | **23** | **1** | **105** |
+ | **Total arquitectural** | **63** | **13** | **5** | **23** | **1** | **105** |
- **Cobertura arquitectural total**: (63 + 11) / 105 = **70%**.
+ **Cobertura arquitectural total**: (63 + 13) / 105 = **72%**
+ (era 70% pré-Passo 149; Value::Type e Value::Args reclassificadas
+ de parcial para implementado⁺ após formalização por ADR-0058 e ADR-0059).
```

**§7 Top divergências** — entradas 1 e 2 reescritas para
indicar formalização por ADR-0058 e ADR-0059 respectivamente.

---

## 7. README dos ADRs actualizado (sub-passo 149.6)

- Cabeçalho: 57 ADRs → 59 ADRs (56 números únicos → 58).
- Tabela "Estado por ADR": 2 linhas novas (ADR-0058, ADR-0059
  ambas `EM VIGOR`).
- Distribuição de status: `EM VIGOR` 24 → 26;
  `IMPLEMENTADO` inalterado (18); outros inalterados.
- Entradas "Passos-chave da história dos ADRs": fundidas
  Passos 147+148 (série paridade) e adicionada entrada
  Passo 149 (arqueologia + formalização).

---

## 8. §9 dos documentos de paridade actualizado (sub-passo 149.7)

`00_nucleo/diagnosticos/typst-paridade-plano-medicao.md` §9:
renumeração dos itens.

```diff
  1. **Passo 148** — Inventário de cobertura ...
- 2. **Passo 149** — Implementar `frame_dto.rs` ...
- 3. **Passo 150+** — Implementar P2 ...
- 4. **Passo 151+** — Implementar P4 ...
- 5. **Decisão sobre corpus** ...
+ 2. **Passo 149** — Arqueologia + formalização de
+    Value::Type e Value::Args (este passo). Produziu
+    ADR-0058 + ADR-0059. Zero código; zero DEBTs.
+ 3. **Passo 150** — Implementar `frame_dto.rs` ...
+ 4. **Passo 151+** — Implementar P2 ...
+ 5. **Passo 152+** — Implementar P4 ...
+ 6. **Decisão sobre corpus** ...
```

Matriz honesta passa a estar disponível a partir do **Passo
150** (era 149 antes deste passo).

---

## 9. Próximo passo: 150

**Passo 150** — Implementar `lab/parity/src/frame_dto.rs` com
`LayoutTolerance` e modo `text_content=true`. Adicionar
`lab/parity/tests/layout_parity.rs` invocando corpus filtrado
pelo subconjunto "implementado" + "implementado⁺" + "parcial"
do inventário 148 (pós-149). Gerar primeiro relatório
agregado em `lab/parity/reports/latest.md` — a **matriz
honesta** que o utilizador pediu.

Sequência restante: 151+ (P2 — value_dto + eval_parity); 152+
(P4 — pdf_compare + export_parity).

---

## 10. Verificação final

| Item | Estado |
|------|--------|
| Inventário factual produzido (149.1) | ✅ |
| Arqueologia executada com passos materializadores identificados (149.2) | ✅ |
| Classificação justificada para cada divergência (149.3) | ✅ |
| ADR-0058 criada (cabeçalho canónico P145) | ✅ |
| ADR-0059 criada (cabeçalho canónico P145) | ✅ |
| 0 DEBTs abertos (classificação "ADR formaliza") | ✅ |
| Inventário 148 actualizado (Tabela B + §7 + totais) | ✅ |
| Cobertura arquitectural 70% → 72% | ✅ |
| README dos ADRs actualizado (total 57 → 59; `EM VIGOR` 24 → 26; Passos-chave) | ✅ |
| §9 de `typst-paridade-plano-medicao.md` renumerado | ✅ |
| Nenhum ficheiro de código tocado em L1/L2/L3/L4 | ✅ |
| `cargo test --workspace --lib` inalterado (1113) | ✅ |
| `crystalline-lint .` zero violations | ✅ |
| Cabeçalho canónico aplicado a ambas as ADRs (P145) | ✅ |
| Relatório do passo escrito | ✅ |

**Pós-149**: inventário 148 tem **referências canónicas**
(ADR ou DEBT) para todas as divergências identificadas em §7
que tinham essa lacuna. Passo 150 tem âncora documental
clara para materializar `frame_dto.rs`.
