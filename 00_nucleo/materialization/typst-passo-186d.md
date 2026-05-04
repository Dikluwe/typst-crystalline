# Passo P186D — Activar `is_locatable(Content::Equation) = true`

Terceiro passo de implementação P186 (após P186A diagnóstico,
P186B variants, P186C `extract_payload` arm).
Magnitude **trivial-S** (mais que trivial pelo trabalho de
restauro e cobertura de invariante).

> **Nota sobre invariante quebrada**: a inversão de ordem
> sugerida originalmente (P186C `extract_payload` antes de
> P186D `is_locatable`) **não eliminou a janela quebrada**
> — apenas inverteu o lado da quebra. Walk de introspect
> gateia em `extract_payload.is_some()` (não em
> `is_locatable`), conforme P186C `.A.6` descobriu
> empiricamente. Resultado: durante P186C↔D, walk emite Tag
> para Equation mas Layouter não avança Locator. P186C
> resolveu pragmaticamente removendo Equation do fixture
> de `gating_locator_apenas_em_locatables` (P185D test);
> P186D **restaura Equation no fixture** quando
> `is_locatable` activar e os dois lados sincronizarem
> novamente.

Modifica arm `Content::Equation` em `is_locatable.rs:60` de
`false` para `true`. Repõe invariante `is_locatable(c) ↔
extract_payload(c).is_some()` para Equation. Restaura
fixture do test P185D. Fecha lacuna identificada pelo
auditor: `build_minimal_for_each_variant` em
`locatable.rs::tests` não cobria Equation.

Após P186D:
- `is_locatable(Content::Equation)` retorna `true`.
- Invariante reposta — Layouter e walk avançam Locator
  sincronizadamente para Equation.
- Fixture P185D restaurado.
- `build_minimal_for_each_variant` cobre Equation.
- `from_tags` stub no-op (P186B) ainda intocado — Equation
  tags são silenciosamente ignoradas; sub-store não recebe
  entries (P186E activa).

**Pré-condição**: P186C concluído. Tests workspace 1.793
verdes; zero violations. Arm `extract_payload` para Equation
funcional. **Janela quebrada activa**: walk emite Tag para
Equation mas Layouter não avança — fixture P185D foi
ajustado em P186C para evitar exposição. P186D fecha a
janela.

**Restrições**:
- **Não** modificar `from_tags` — P186E.
- **Não** modificar `extract_payload` — P186C fechou.
- **Não** modificar walk arm legacy.
- **Não** migrar consumer C2 — P188.
- API pública preservada.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar `is_locatable` actual:
   - `01_core/src/rules/introspect/locatable.rs:60` (per
     P186A §2).
   - Localizar arm `Content::Equation { .. } => false`.
   - Confirmar invariante `is_locatable(c) ↔
     extract_payload(c).is_some()` documentado em
     `locatable.rs:11`.

2. Confirmar L0 `rules/introspect/locatable.md`:
   - Localizar entrada actual sobre `Content::Equation`.
   - Identificar onde actualizar.

3. Confirmar fixture P185D actual:
   - `01_core/src/rules/layout/tests.rs` submódulo
     `p185d_locator_sync` test
     `gating_locator_apenas_em_locatables`.
   - Per P186C `.D`: Equation foi **removida** do fixture
     com comentário inline referenciando P186D para
     restauro.
   - Localizar comentário e linha de Equation removida.

4. Confirmar `build_minimal_for_each_variant`:
   - `01_core/src/rules/introspect/locatable.rs::tests`.
   - Per P186C §"Próximo passo": helper que constrói
     instância mínima de cada variant de Content para
     teste de invariante. **Equation está em falta**.
   - Identificar como adicionar (provável: arm para
     `Content::Equation { body: Box::new(Content::Empty),
     block: true }`).

5. Confirmar test de invariante:
   - Em `locatable.rs::tests`, deve haver test que
     itera todos os variants de Content e verifica
     invariante `is_locatable ↔
     extract_payload.is_some()`.
   - Após P186D, este test deve incluir Equation e
     passar.

6. Confirmar P186C activo:
   - `extract_payload(Content::Equation { .. })` retorna
     `Some(...)`. Verificado em P186C `.E`.

Output: tabela com item + estado + linhas exactas para
edits.

**Critério de saída**:
- Arm `is_locatable` localizado.
- Fixture P185D localizado (com comentário a restaurar).
- `build_minimal_for_each_variant` localizado e lacuna
  Equation confirmada.
- Test de invariante localizado.

### .B Actualizar L0 `rules/introspect/locatable.md`

1. Modificar entrada para `Content::Equation`:
   - Antes: documentado como não-locatable.
   - Depois: locatable. Justificação: produz
     `ElementPayload::Equation { block, counter_update }`
     em `extract_payload` (P186C); permite walk popular
     sub-store (P186E).
   - Cross-reference: P186C arm `extract_payload`.

2. Hash em branco aguarda recálculo.

**Critério de saída**:
- L0 reflecte mudança.

### .C Modificar `is_locatable.rs:60`

1. Arm `Content::Equation { .. } => false`:
   - Mudar para `Content::Equation { .. } => true`.
   - Mover de "Não-locatable" para "Locatable" se a
     organização do ficheiro for por blocos (per P186A
     §1: lista "Não-locatable (53 variants)" em
     `locatable.rs:66`; mover para bloco "Locatable").
   - Forma exacta fica para Claude Code conforme
     convenção do projecto.

2. Confirmar `@prompt-hash` actualiza após edit.

**Critério de saída**:
- `cargo check --workspace` passa.
- Linter passa.
- `is_locatable(Content::Equation)` retorna `true`.

### .D Restaurar fixture P185D

1. Em `01_core/src/rules/layout/tests.rs` submódulo
   `p185d_locator_sync`, test
   `gating_locator_apenas_em_locatables`:
   - Per P186C `.D`: Equation foi removida com
     comentário inline.
   - **Restaurar** `Content::Equation { body: Box::new(...),
     block: true }` (ou similar conforme estava
     originalmente) no fixture.
   - Actualizar contagem esperada: 3 → **4** locatables
     na sequência (assumindo fixture original era
     `[Heading, Text, Figure, Equation, Cite]`).
   - Remover comentário "ajustado em P186C; restaurar em
     P186D" (cumprido).

2. Verificar que outros tests no submódulo `p185d_locator_sync`
   não regridem com a mudança.

**Critério de saída**:
- Fixture restaurado.
- Comentário transitório removido.
- Test passa com contagem actualizada.

### .E Fechar lacuna `build_minimal_for_each_variant`

1. Em `locatable.rs::tests`:
   - Adicionar arm para `Content::Equation` no helper
     `build_minimal_for_each_variant`.
   - Forma sugerida: `Content::Equation { body:
     Box::new(Content::Empty), block: true }` (ou
     equivalente conforme convenção dos outros arms do
     helper).

2. Confirmar test de invariante itera Equation e passa:
   - `is_locatable(Equation) == true` ✓ (P186D).
   - `extract_payload(Equation).is_some() == true` ✓
     (P186C).
   - Invariante satisfeita.

3. Se test de invariante já existia mas Equation estava
   omitida (per achado P186A): adição em
   `build_minimal_for_each_variant` automaticamente
   inclui Equation no test sem alterações adicionais.

**Critério de saída**:
- Equation coberta em `build_minimal_for_each_variant`.
- Test de invariante passa para Equation.

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P186C
   baseline (1.793): 0 a +1 (sem tests novos
   fundamentalmente; ajuste de fixtures + adição em
   `build_minimal` que pode ou não contar como test
   novo dependendo de como teste de invariante itera).
3. `crystalline-lint .` zero violations.
4. `is_locatable(Content::Equation)` retorna `true`.
5. **Invariante reposta**: `is_locatable(Equation) ↔
   extract_payload(Equation).is_some()` (ambos `true`).
6. Fixture P185D restaurado com Equation.
7. `build_minimal_for_each_variant` cobre Equation.
8. Test P185D `gating_locator_apenas_em_locatables`
   passa com 4 locatables.
9. Test de invariante passa para Equation.
10. Walk arm legacy intocado.
11. `from_tags` stub no-op intocado.
12. Snapshot tests ADR-0033 verdes.
13. Linter passa final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-186d-relatorio.md`
com:

- Resumo: arm `is_locatable(Content::Equation)` activado;
  invariante reposta; fixture P185D restaurado; lacuna
  `build_minimal_for_each_variant` fechada.
- Confirmação `.F` (13 verificações).
- Δ tests vs baseline P186C: 0 a +1.
- Hashes finais L0 (`locatable.md`).
- Decisões de execução notáveis:
  - Inversão de ordem P186C/D não eliminou janela
    quebrada — apenas inverteu sentido. Walk gateia em
    `extract_payload`, não em `is_locatable`.
  - Pragmatismo P186C (ajustar fixture) preservou tests
    existentes durante janela.
  - P186D fecha janela e restaura fixture.
- Estado actual:
  - P186 série: A ✅ B ✅ C ✅ D ✅ | E-F pendentes.
  - **Invariante reposta** — sincronização Locator
    Layouter ↔ walk íntegra.
  - Walk emite Tag para Equation com payload válido.
  - Sub-store ainda **não** populado (P186E activa).
  - 51 passos executados.
- Pendências cumulativas: inalteradas (janela quebrada
  fechada).
- Próximo passo: P186E (substituir stub no-op em
  `from_tags` por arm funcional com gate `block &&
  state-active`).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria com linhas exactas para edits.
2. L0 `locatable.md` actualizado.
3. Arm `is_locatable` activado.
4. Fixture P185D restaurado (Equation re-incluída;
   contagem 3→4).
5. `build_minimal_for_each_variant` cobre Equation.
6. Tests existentes não regridem.
7. **Invariante reposta** explicitamente confirmada
   em `.F`.
8. Verificações `.F` passam (13/13).
9. Relatório `.G` escrito.

---

## O que pode sair errado

- **Fixture P185D foi alterado de forma diferente do
  esperado** em P186C: cláusula gate trivial — auditar
  estado actual e restaurar correctamente.
- **`build_minimal_for_each_variant` não existe ou tem
  nome diferente**: cláusula gate trivial — investigar
  test de invariante e adaptar.
- **Test de invariante itera de forma diferente** (não
  via `build_minimal_for_each_variant`): cláusula gate
  trivial — adaptar adição de Equation.
- **Walk emite Tag para Equation mas `from_tags` stub
  no-op causa panic**: improvável (P186B confirmou stub
  benigno). Se acontecer, cláusula gate substancial —
  investigar.
- **Outros tests regridem inesperadamente** por activação
  de Equation locatable: `grep` amplo em `.A` deve
  apanhar; ajustar se aparecer.
- **Snapshot tests divergem**: walk emite agora Tag para
  Equation. Algum consumer pode iterar `kind_index` e
  ver Equations onde antes não via. Cláusula gate
  trivial — investigar consumer específico. Per P186A
  Q3, esperado raro.
- **Linter divergência V13/V14**: cláusula gate trivial.

---

## Notas operacionais

- **Tamanho**: trivial-S. ~1 LOC produção (mudança de
  `false` para `true`) + ~5 LOC fixture restaurado +
  ~5 LOC `build_minimal` + edits L0.
- **Sem dependências externas novas**.
- **Janela quebrada fecha aqui**: P186C abriu janela
  pragmaticamente (ajustando fixture); P186D fecha e
  restaura. Estado pós-P186D é deployable em qualquer
  ponto.
- **Pré-condição P186E**: este passo concluído.
- **Padrão**: P181D (Bibliography) + P182C
  (SetHeadingNumbering) também activaram `is_locatable`
  após `extract_payload` arm — replicação consciente.
- **Cláusula gate trivial**: aplicável a divergência
  estrutural de fixtures, ajustes de helpers de teste,
  recálculo de hashes.
- **Cláusula gate substancial**: aplicável apenas se
  `from_tags` stub no-op causar panic ou se algum
  consumer não-inventariado regredir.
- **Achados de execução P186C valiosos**:
  - Walk gateia em `extract_payload`, não `is_locatable`.
    Documentação L0 deveria registar este facto
    explicitamente — fora de escopo P186 mas vale
    registo informal para passos futuros.
  - `build_minimal_for_each_variant` tinha lacuna
    silenciosa (Equation omitida). P186D fecha. Outros
    variants podem ter mesma lacuna — verificação fora
    de escopo.
- **Contagem P185D fixture**: 3→4 assume fixture original
  `[Heading, Text, Figure, Equation, Cite]`. Confirmar
  empiricamente em `.A.3` — P186C alterou fixture; P186D
  reverte para forma original.
