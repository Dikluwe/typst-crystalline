# Passo 105 — Relatório de auditoria global de DEBTs

**Data**: 2026-04-23
**Precondição**: Passo 104 encerrado; 803 L1 + 174 L3 + 6 ignorados;
zero violations.
**ADR**: sem ADR nova (auditoria documental).

---

## Sumário

Auditoria sistemática dos 53 DEBTs em `00_nucleo/DEBT.md`:

- **Secção 1** (abertos/parcialmente): **14** entradas.
- **Secção 2** (encerrados/resolvidos): **39** entradas.

Todos os **14 DEBTs activos classificados como M (Manter)** — nenhum
qualifica para F (Fechar) ou A (Actualizar). O estado descrito no
ficheiro reflecte o estado real do código.

**1 orphan comment removido** — `DEBT-34b` em `entities/content.rs`
dizia "rows ignorado pelo layouter" mas `grid.rs` usa `rows` desde
Passo 83. Comentário obsoleto substituído por nota histórica.

Zero regressão: **803 L1 + 174 L3 + 6 ignorados** (inalterado). Zero
violations.

---

## 105.A+B — Tabela final de classificação

| DEBT | Título | Classif. | Razão empírica |
|------|--------|---------:|----------------|
| DEBT-1 | StyleChain — pendências residuais | **M** | Já tem secções de "Actualização Passo 99/100/101/102/103/104". Estado reflecte código exacto; propriedades adicionais ainda bloqueadas por tipos não materializados. |
| DEBT-2 | Closures eager vs lazy | **M** | Sem mudança desde Passo 31. Comemo tracking não activado. |
| DEBT-8 | Motor de equações | **M** | Pendências (kern, OpenType MATH, MathPrimes, baseline x-height) continuam. Passo 96.8 reestruturou mas sem features novas. |
| DEBT-9 | Cobertura de paridade (contínuo) | **M** | Por design ongoing — baseline 35 preservado, 50/50 passam. |
| DEBT-33 | Bounding Box Bézier | **M** | Grep em `layout/` confirma min/max de pontos de controlo. Cálculo analítico ausente. |
| DEBT-34d | Auto não encolhe antes de matar fr | **M** | `grid.rs:64` confirma Auto greedy sem negociação. |
| DEBT-34e | colspan e rowspan | **M** | `Content::Grid` sem colspan/rowspan; `cells.chunks(num_cols)` linear. |
| DEBT-35b | Cache available_width + SetPage | **M** | `available_width()` inline sem cache. Comentário preventivo preservado. |
| DEBT-42 | `get_unchecked` scanner | **M** | 7 ocorrências confirmadas. Bloqueio benchmark permanece. |
| DEBT-43 | Linter whitelist crate-level | **M** | `crystalline.toml` continua crate-level. Dependência externa. |
| DEBT-45 | `check_*_depth` não chamados | **M** | 2/4 ✓ (show + call); 2/4 pendentes (layout adiado, html aguarda pipeline). |
| DEBT-49 | Propriedades `#set` silenciadas | **M** | Actualizado 104 (1/4 ✓). Propagação pendente. |
| DEBT-50 | Show selector origin (dívida latente) | **M** | Canário passa (`debt_50_...`). Dívida adormecida. |
| DEBT-51 | Warnings Sink → L3 | **M** | Recém-aberto (104). Bloqueia valor prático de DEBT-49. |

**Total**: 14 **M**, 0 **F**, 0 **A**.

---

## 105.C — Acção aplicada

### Orphan comment removido

`entities/content.rs:250-257`:

**Antes**:
```rust
/// Grid de colunas com células posicionadas por ordem de leitura (Passo 80).
///
/// `rows` é armazenado no AST mas ignorado pelo layouter (DEBT-34b).
Grid {
    columns: Vec<TrackSizing>,
    rows:    Vec<TrackSizing>, // DEBT-34b: ignorado — todas as linhas são Auto
    cells:   Vec<Content>,
},
```

**Depois**:
```rust
/// Grid de colunas com células posicionadas por ordem de leitura (Passo 80).
///
/// `rows` é consumido pelo layouter desde o Passo 83 (DEBT-34b encerrado).
/// Comentário obsoleto removido na auditoria do Passo 105.
Grid {
    columns: Vec<TrackSizing>,
    rows:    Vec<TrackSizing>,
    cells:   Vec<Content>,
},
```

**Justificação**: DEBT-34b encerrado no Passo 83. `grid.rs:41`
mostra que `rows` é convertido em `row_tracks` e consumido pelo
algoritmo de layout. Comentários inline anteriores eram
factualmente incorrectos e confundiam leitores — eram os únicos
orphan-comments detectados na auditoria que diziam "pendente" ou
contradiziam o código actual.

### Outros orphan comments (mantidos como notas históricas)

- `counter_state.rs:27`, `content.rs:171,178` (DEBT-10): falam de
  "quando motor de introspecção". DEBT-10 resolvido, mas estes
  comentários referem trabalho futuro **condicionado** à
  materialização de `Introspection`. Mantidos.
- `content.rs` — outras referências a DEBT-10 em docstrings de
  variantes `SetHeadingNumbering`/`CounterDisplay` também mantidas
  como contexto histórico.
- `rules/layout/mod.rs:507` (DEBT-35b): preventivo para DEBT aberto.
  Mantido.
- `rules/layout/mod.rs:531` (DEBT-28): explicitamente diz
  "encerrado". Nota histórica. Mantida.
- `rules/eval/tests.rs:1964, 2007, 2051` (DEBT-19, 20, 23): todas
  dizem "encerrado". Notas históricas em tests que asseguram
  comportamento não regride. Mantidas.

---

## 105.D — Verificação

```
$ cargo test --workspace | grep "test result"
test result: ok. 803 passed; 0 failed; 0 ignored ...
test result: ok. 174 passed; 0 failed; 6 ignored ...

$ crystalline-lint .
✓ No violations found

$ grep -c "^## DEBT-" 00_nucleo/DEBT.md
53   (inalterado — auditoria não fecha nem abre DEBTs)
```

Contagens antes/depois:

| Métrica | Antes | Depois |
|---------|------:|-------:|
| L1 tests | 803 | **803** (inalterado) |
| L3 tests | 174 | 174 |
| DEBTs totais | 53 | 53 (inalterado) |
| Activos (Secção 1) | 14 | 14 (inalterado) |
| Encerrados (Secção 2) | 39 | 39 (inalterado) |
| Violations | 0 | 0 |

---

## 105.E — Conclusões

### Alinhamento confirmado

A auditoria confirma que o estado registado em `DEBT.md` **está
alinhado** com o estado real do código. Isto contrasta com os
passos 99/102/103 onde descobertas empíricas mostraram desalinhamento
(ex: `#set` já activo; `#show` já activo; `LazyHash` já fora de L1).

Razão provável do alinhamento actual:

- Passos 99–104 actualizaram DEBT-1 proactivamente com secções
  "Actualização Passo N" em cada passo — práctica institucionalizada.
- DEBTs novos (49, 50, 51) foram abertos com critérios objectivos e
  referências empíricas desde o início.

### DEBTs que merecem atenção futura (sem acção neste passo)

Nenhum. Todos os 14 DEBTs activos têm critério claro e estado
descrito correctamente.

### DEBTs com critério ambíguo

Nenhum. Todos os critérios encontrados estão operacionalizáveis:

- Checkboxes com condições empíricas.
- Dependências externas documentadas (ex: DEBT-42 aguarda infra
  benchmark; DEBT-43 depende de `crystalline-lint`).
- Dívidas condicionais documentadas com canário (ex: DEBT-50).

---

## Lições

1. **Manutenção preventiva paga dividendos**: o Passo 105 fechou
   zero DEBTs porque Passos 99–104 mantiveram `DEBT.md` sincronizado
   em tempo real. Cada activação/materialização adicionou secção
   "Actualização" aos DEBTs afectados. Resultado: auditoria torna-se
   confirmação, não correcção.

2. **Orphan comments são bug silencioso**: o comentário DEBT-34b
   em `content.rs:255` dizia "rows ignorado" mas o layouter **usa**
   rows desde Passo 83. Leitor casual seria induzido em erro. A
   auditoria detectou 1 orphan factual; outros comentários (DEBT-10,
   DEBT-19, DEBT-20, DEBT-23, DEBT-28, DEBT-35b) são notas históricas
   legítimas e foram preservados.

3. **Gate "se em dúvida, M" é conservador mas correcto**: 14 DEBTs,
   14 classificações M. Nenhum F arriscado. O valor da auditoria não
   está em fechar muitos DEBTs, mas em **confirmar** o alinhamento e
   remover ruído (orphan comments).

4. **DEBT-1 como caso de estudo de "Parcialmente Resolvido
   continuado"**: DEBT-1 tem 6+ secções "Actualização" acumuladas
   desde Passo 22. Cresceu mas permanece legível. Subdivisão
   (DEBT-1a, 1b, ...) foi considerada mas rejeitada: criar DEBTs
   novos está fora do escopo da auditoria.

---

## Estado pós-Passo 105

### DEBT.md alinhado

| Categoria | DEBTs |
|-----------|------:|
| Abertos com trabalho pendente | 11 (DEBT-2, 8, 33, 34d, 34e, 42, 43, 45, 49, 50, 51) |
| Abertos com tracking contínuo | 1 (DEBT-9) |
| Parcialmente resolvidos com actualização viva | 1 (DEBT-1) |
| Preventivos (comentários de risco) | 1 (DEBT-35b) |
| Encerrados | 39 |

### Trabalho futuro identificado

A auditoria não criou DEBTs novos. Candidatos naturais para próximos
passos, por ordem de payoff empírico:

1. **DEBT-51** (warnings Sink → L3) — decidir canal e implementar.
   Desbloqueia valor prático de DEBT-49.
2. **DEBT-49** completo — propagar `sink: &mut Sink` pelas `eval_*`
   (5ª aplicação da ADR-0036) + migrar sítios silenciados. Bloqueado
   por DEBT-51.
3. **DEBT-45** — integrar `check_layout_depth` (2/4 restante).
   Candidato para quando Engine<'a> materializar.
4. **DEBT-2** — integração com `comemo` para tracking semântico real.
   Ligado à materialização de `TrackedWorld` real.
5. **DEBT-8** — motor de equações: kern, fontes MATH. Depende de
   materialização de fonte OpenType real.
6. **DEBT-42** — requer infra de benchmark (pré-requisito externo).
7. **DEBT-43** — requer alteração no `crystalline-lint` externo.

Os DEBTs preventivos (35b) e condicionais (50) não exigem trabalho
proactivo — ficam monitorizados pelo canário em teste ou por revisão
futura.
