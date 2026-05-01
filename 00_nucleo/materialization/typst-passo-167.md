# Passo P167 — Inventário de consumers `CounterStateLegacy` + escolha de primeiro a migrar (M5 sub-passo 1)

Primeiro passo de M5 do refactor Introspection. **Sem código
de produção** — passo de descoberta + decisão. Output:
- Inventário literal de **leitores** de `CounterStateLegacy`
  (campos lidos por cada consumer).
- Mapeamento **campo legacy → equivalente `TagIntrospector`**
  ou identificação de lacuna.
- Escolha do primeiro consumer a migrar em P168 com
  justificação numérica.

Trabalho real de migração fica para P168 (passo seguinte).

**Pré-condição**: M4 concluído (P166).
`introspect_with_introspector` expõe `(CounterStateLegacy,
TagIntrospector)`; `introspect()` é wrapper que descarta o
introspector.

**Restrições**:
- Não modificar código de produção.
- Não criar L0+L1 novos (ficheiros L1).
- Sem alteração de `Introspector`, sub-stores, ou walk.
- Apenas trabalho documental: leitura, análise, escrita de
  diagnóstico.
- Output observable não muda.

---

## Sub-passos

### .A Inventário de leitores

Identificar literalmente todos os call-sites que lêem
**fields** ou chamam **métodos** de `CounterStateLegacy`.

1. Grep por acessos a fields:
   - `grep -rn "\.resolved_labels\|\.headings_for_toc\|\.figure_numbers\|\.figure_supplements\|\.bib_entries\|\.bib_numbers\|\.lang\|\.has_outline\|\.is_readonly\|\.numbering_flags" 01_core/src/ 03_infra/src/`.
   - Ajustar à lista exacta de fields confirmada em P161 .A.
2. Grep por chamadas a métodos:
   - `grep -rn "\.format_hierarchical\|\.next_heading_at\|\.step_hierarchical" 01_core/src/ 03_infra/src/`.
   - Adaptar à API real de `CounterStateLegacy` (a maioria
     destes métodos foi confirmada em P163 .A).
3. Para cada match, registar:
   - **Localização**: ficheiro:linha.
   - **Field/método** lido.
   - **Função enclosing**: nome da função onde o acesso ocorre.
   - **Categoria**: production / test / helper de teste.
4. Agrupar por consumer (função enclosing) e por field
   acessado.

Output: tabela em
`00_nucleo/diagnosticos/inventario-consumers-counter-state-legacy.md`
com formato:

```
## Leitores por consumer

### Consumer: <nome da função/módulo>

| Field/método lido | Localização (ficheiro:linha) | Categoria |
|-------------------|------------------------------|-----------|
| ...               | ...                          | ...       |

### Consumer: <próxima função>
...
```

**Critério de saída**:
- Tabela completa em `00_nucleo/diagnosticos/...md`.
- Cada consumer tem secção própria.
- Pelo menos 1 consumer de production identificado (P166
  .A reportou 2 — Layouter e pipeline).
- Tests não-relevantes para migração (apenas tests do
  próprio `CounterStateLegacy`) podem ser omitidos da
  análise downstream, mas listados.

### .B Mapeamento legacy → Introspector

Para cada (consumer, field-lido) identificado em .A,
determinar se `TagIntrospector` actual cobre, cobre
parcialmente, ou não cobre.

1. Para cada field/método único usado:
   - Identificar equivalente em `TagIntrospector`:
     - `state.resolved_labels` → `introspector.query_by_label`?
     - `state.headings_for_toc` → indexar via
       `introspector.kind_index[Heading]` + `query_by_label`?
     - `state.figure_numbers` → `introspector.counters.value("figure")`?
     - `state.format_hierarchical(key)` → reconstruível
       via `counters.value(key)` + formatação?
   - Categorizar mapeamento:
     - **Directo**: equivalente 1:1 sem perda de informação.
     - **Parcial**: equivalente cobre subset; falta info.
     - **Lacuna**: sem equivalente. `TagIntrospector` não
       tem mecanismo.
2. Para cada categoria "Parcial" ou "Lacuna":
   - Identificar qual ficheiro de pendências cobre
     (ex. `m1-lacunas-captura.md` lista as 3 divergências
     conhecidas).
   - Se a lacuna é nova (não estava em `m1-lacunas-captura.md`),
     adicionar ao mesmo ficheiro com nota "detectado em
     P167".

Output: secção em
`inventario-consumers-counter-state-legacy.md`:

```
## Mapeamento por field

| Field/método legacy | Equivalente Introspector | Categoria | Notas |
|---------------------|--------------------------|-----------|-------|
| ...                 | ...                      | Directo   | ... |
| ...                 | ...                      | Parcial   | ver `m1-lacunas-captura.md` divergência X |
| ...                 | (sem equivalente)        | Lacuna    | nova; adicionar a `m1-lacunas-captura.md` |
```

**Critério de saída**:
- Secção completa.
- Cada field do inventário .A tem entrada.
- Lacunas novas registadas em `m1-lacunas-captura.md`.

### .C Análise de migrabilidade por consumer

Para cada consumer identificado em .A, calcular:
- **Migrabilidade total**: todos os fields lidos têm
  mapeamento "Directo".
- **Migrabilidade parcial**: alguns fields têm "Parcial" ou
  "Lacuna"; consumer pode migrar para subset com
  caveat documentado.
- **Migrabilidade bloqueada**: pelo menos 1 field crítico
  é "Lacuna" sem mapeamento parcial viável.

Output: secção em
`inventario-consumers-counter-state-legacy.md`:

```
## Migrabilidade por consumer

| Consumer | Fields lidos | Migrabilidade | Razão |
|----------|--------------|---------------|-------|
| Layouter | resolved_labels, figure_numbers | Total | todos os fields têm equivalente directo |
| pipeline | format_hierarchical, headings_for_toc | Parcial | headings_for_toc tem auto-labels lacuna |
| ... | ... | ... | ... |
```

**Critério de saída**:
- Secção completa.
- Pelo menos 1 consumer marcado como "Total" (caso
  contrário, gate substancial em .D).

### .D Escolha do primeiro consumer + decisão

Com base em .A, .B, .C, escolher o **primeiro consumer a
migrar em P168**.

**Regras de escolha**:
1. Preferir consumer com **migrabilidade Total** (sem
   lacunas).
2. Entre os Total, preferir consumer com **menor número de
   call-sites** (menos disruption).
3. Entre os Total com poucos call-sites, preferir consumer
   de **production** sobre tests (validação real).
4. Entre os Total de production, preferir consumer com
   **menor superfície downstream** (não bloqueia outros
   passos).

Output: secção final em
`inventario-consumers-counter-state-legacy.md`:

```
## Escolha para P168

**Consumer escolhido**: <nome>
**Localização**: <ficheiro:linha>
**Justificação**: <por que este antes dos outros>
**Fields a migrar**: <lista>
**Mapeamento concreto**: <field legacy → método Introspector>
**Riscos identificados**: <se algum>
**Tamanho estimado de P168**: S / M / L
```

**Critério de saída e gate de decisão**:
- Se há pelo menos 1 consumer Total: escolher e prosseguir.
- Se nenhum consumer é Total mas há Parciais aceitáveis:
  cláusula gate trivial — escolher Parcial com lacunas
  não-críticas, documentar caveats.
- Se todos os consumers são Bloqueados (todos têm lacunas
  críticas): **gate substancial**. Parar e reportar.
  Próximo passo precisa de adicionar features ao
  `Introspector` antes de migrar qualquer consumer.

### .E Verificação estrutural

1. `cargo check --workspace` passa (não tocámos código).
2. `cargo test --workspace` passa sem mudança de contagem
   (não criámos tests).
3. `crystalline-lint`: zero violations (não modificámos
   L0/L1 — confirmação).
4. Diagnóstico
   `00_nucleo/diagnosticos/inventario-consumers-counter-state-legacy.md`
   existe com 4 secções (.A leitores, .B mapeamento, .C
   migrabilidade, .D escolha).
5. Se houve lacunas novas detectadas em .B,
   `m1-lacunas-captura.md` foi actualizado.
6. Nenhum L0 modificado (não há sincronização L0↔L1 a
   verificar — passo é puramente documental).

### .F Encerramento

Escrever
`00_nucleo/materialization/typst-passo-167-relatorio.md` com:

- Resumo: inventário de consumers feito; primeiro consumer
  escolhido para P168.
- Confirmação de cada verificação .E.
- Resumo numérico do inventário:
  - Número total de consumers identificados.
  - Distribuição por categoria (Total / Parcial / Bloqueado).
  - Lacunas novas adicionadas a `m1-lacunas-captura.md`
    (se algumas).
- Consumer escolhido para P168 + justificação.
- Estado pós-passo: P167 concluído. P168 desbloqueado —
  começar migração do consumer escolhido.
- Pendências cumulativas (lista crescente desde M1).

---

## Critério de conclusão

Todas em conjunto:

1. .A: inventário literal de leitores em diagnóstico.
2. .B: mapeamento campo-a-campo legacy→Introspector.
3. .C: classificação de migrabilidade por consumer.
4. .D: primeiro consumer escolhido (ou gate substancial
   disparado se todos bloqueados).
5. .E: verificações 1-6 passam.
6. .F: relatório escrito.
7. Sem código de produção tocado.
8. Sem L0+L1 novos.

---

## O que pode sair errado

- **Inventário .A revela API mais larga que esperado**: se
  `CounterStateLegacy` tem fields/métodos que P161-P163 não
  documentaram, registar e continuar. Pode aumentar tamanho
  do diagnóstico.
- **Mapeamento .B revela que muitos fields têm "Lacuna"**:
  significa que `TagIntrospector` está mais incompleto que
  o desenho assumia. Documentar exaustivamente em
  `m1-lacunas-captura.md`. Pode forçar reconsideração da
  ordem de M5/M9.
- **Gate substancial em .D (todos os consumers bloqueados)**:
  parar, reportar, decidir caminho:
  - Adicionar features faltantes ao `Introspector` num
    passo P168 alternativo (mover M9 para a frente).
  - Ou: migrar com caveat documentado, aceitando que
    consumer migrado tem comportamento parcial.
  Decisão fica para conversa com utilizador.
- **Inventário descobre call-sites externos a 03_infra
  além dos 10 já identificados em P166 .A**: workspace
  pode ter mais crates. Expandir `grep -rn ... 02_*/src/
  04_*/src/` se houver. Documentar.
- **`format_hierarchical` é usado mais que esperado**:
  método é wrapper sobre vários counters internos.
  Reconstruir em `Introspector` pode exigir helper novo.
  Documentar como pendência se for o caso.

---

## Notas operacionais

- **Tamanho**: S. Trabalho documental puro. Sem código.
  Tests não mudam (verificação .E.2 é confirmação de
  estabilidade, não Δ).
- **Output principal**: o diagnóstico
  `inventario-consumers-counter-state-legacy.md` é
  referência para P168 em diante. P168 cita-o; M6 cita-o
  para confirmar que todos os consumers migraram.
- **Cláusula gate trivial** (formalizada em P163):
  aplicável a decisões de escolha em .D entre consumers
  comparáveis.
- **Risco de descoberta tarde**: P167 é a primeira
  oportunidade de saber se `Introspector` está bem para
  consumir. Se descobrirmos lacunas substanciais aqui,
  M5 inteiro pode precisar de re-planeamento. Aceitável
  — preferível descobrir agora que em P168 com código a
  meio.
- **`m1-lacunas-captura.md`**: P167 .B pode adicionar
  novas entradas. Se a lista crescer significativamente,
  considerar reorganizar ou consolidar antes de prosseguir.
