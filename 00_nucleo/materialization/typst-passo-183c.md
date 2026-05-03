# Passo P183C — C2 equation counter via `flat_counter`

Segundo passo de implementação P183 (após P183A diagnóstico,
P183B bloqueado por gate substancial).
Magnitude **S** se semântica `flat_counter` Introspector
match com `get_flat` legacy; senão **bloqueado** com escalada
para DEBT M4-residual.

P183B revelou que `formatted_counter` Introspector tem
**snapshot final após walk** enquanto `format_hierarchical`
legacy é **mutável durante walk**. Substitution-with-fallback
não corrige porque `Some` do Introspector pré-empta o
fallback. P183C aplica este aprendizado: **`.A` faz auditoria
semântica explícita antes de qualquer migração**. Se
`flat_counter` tiver mesma natureza (snapshot-final), P183C
declara gate substancial igual ao P183B e escala para DEBT.

Migra (se a auditoria semântica passar) consumer C2
(`Layouter::layout_equation` em
`01_core/src/rules/layout/equation.rs:97`) de
`self.counter.get_flat("equation")` legacy para
`self.introspector.flat_counter("equation")` com fallback
legacy. Adiciona método trait `flat_counter(&self, key: &str)
-> Option<Vec<usize>>` (ou similar) ao trait `Introspector`,
impl em `TagIntrospector` delega a sub-store apropriado.

Após P183C (caso de sucesso):
- Trait `Introspector` ganha `flat_counter`.
- Consumer C2 consulta Introspector primeiro; fallback
  legacy.
- Walk arm canonical, write paralelo legacy intocados (M6
  elimina).
- Output observable em produção inalterado.

Após P183C (caso de bloqueio):
- Tudo revertido (igual a P183B).
- DEBT M4-residual aberto literalmente em P183F (passo de
  fecho da série) com lista actualizada de consumers
  bloqueados pela natureza snapshot-during-walk.

**Pré-condição**: P183A concluído. P183B reportado como
bloqueado e revertido. Tests workspace 1.756 verdes; zero
violations.

**Restrições**:
- **Não** modificar walk arm canonical legacy.
- **Não** modificar write paralelo legacy.
- **Não** modificar copy-sites.
- **Não** assumir paridade semântica antes da auditoria
  `.A` confirmar.
- **Não** prosseguir para `.D` se `.B` declarar gate
  substancial — escalar para DEBT em P183F.
- API pública preservada.
- Output observable em produção inalterado.

---

## Aprendizado P183B aplicado

P183B falhou porque a spec assumiu paridade semântica entre
duas primitivas que não a tinham. P183C corrige adicionando
**sub-passo de auditoria semântica explícita** (`.B`) antes
de qualquer migração de código. Se a auditoria revelar
asimetria snapshot-during-walk vs snapshot-final (igual a
P183B), o passo declara gate substancial e termina sem
migrar — sem testes a regredir, sem reverter código.

A categoria de bloqueio descoberta em P183B é
"**consumers que precisam de valor durante o walk, não do
final**". Identificação prévia desta categoria nos
consumers C1–C5:

- **C1 heading prefix** — confirmado bloqueado (P183B).
- **C2 equation counter** — em diagnóstico (este passo).
- **C3 figure auto-number per kind** — provavelmente
  bloqueado (lê `figure_numbers[kind][idx]` durante
  layout; mesma natureza de C1).
- **C4 resolved label** — provavelmente OK (labels são
  identidade, snapshot final = snapshot durante walk).
- **C5 TOC entries** — já bloqueado por lacuna #3
  separada.

P183C diagnostica C2. Se passar, P183 prossegue. Se falhar,
categoria confirma bloqueio em massa e P183 muda de
natureza (DEBT M4-residual cobre C1, C2, C3 todos).

---

## Sub-passos

### .A Auditoria L0

1. Confirmar consumer C2 actual:
   - `01_core/src/rules/layout/equation.rs:97` (per
     P183A §2).
   - Localizar leitura: padrão esperado
     `self.counter.get_flat("equation")` ou similar.
   - Identificar contexto (é dentro de
     `layout_equation`? que valor é retornado? é usado
     para formatar prefixo da equação numerada?).
   - Confirmar tipo de retorno: `Vec<usize>`,
     `Option<Vec<usize>>`, `&[usize]`, ou outra.

2. Confirmar consumer P182D `is_numbering_active`
   migrado em `equation.rs:24` (3 linhas acima):
   - Pode haver impacto cruzado entre os dois consumers
     no mesmo arm. Confirmar empiricamente.

3. Confirmar `get_flat` legacy:
   - `01_core/src/entities/counter_state_legacy.rs` (ou
     similar).
   - Assinatura exacta.
   - Comportamento durante walk vs após walk:
     - Legacy é populado pelo walk como? (`equation_numbers`
       field provavelmente).
     - Quando o Layouter chama `get_flat`, o `state`
       legacy está em que estado — final pós-walk, ou
       mutável durante o layout walk?

4. Confirmar `flat_counter` em trait `Introspector`:
   - **Não existe** ainda (per P183A §3 — método novo a
     adicionar em P183C).
   - Inventariar qual sub-store contém os contadores de
     equation (`CounterRegistry`?).
   - Confirmar API do sub-store: tem método análogo a
     `formatted_counter` mas que retorna o `Vec<usize>`
     em vez de string formatada?

### .B Auditoria semântica explícita

**Esta é a auditoria que faltou em P183B.**

1. Construir documento de teste com 3 equations
   numeradas:
   ```
   #set heading(numbering: ...)  // ou equivalente
   $ a = 1 $  // equation 1
   $ b = 2 $  // equation 2
   $ c = 3 $  // equation 3
   ```

2. Para cada equation, durante o layout, observar:
   - **Path legacy**: `state.get_flat("equation")` retorna
     que valor?
     - Em equation 1: `[1]`?
     - Em equation 2: `[2]`?
     - Em equation 3: `[3]`?

3. Após o walk completar (antes do layout), observar:
   - **Path Introspector**: `intr.flat_counter("equation")`
     retorna que valor?
     - Sempre `[3]` (snapshot final)?
     - Ou tem mecanismo location-aware que retorna
       `[1]`, `[2]`, `[3]` conforme posição consultada?

4. Se path Introspector retorna sempre `[3]` (snapshot
   final): **gate substancial**. Mesma natureza de C1.
   Saltar para `.G` (escalada para DEBT M4-residual).

5. Se path Introspector tem mecanismo location-aware
   (improvável sem mudança no Introspector;
   `flat_counter` é método novo — pode ser desenhado
   para aceitar `location` como argumento): **prosseguir**
   para `.C`.

6. Decisão: **`flat_counter(key)` sem location é a forma
   simples**, mas só serve se C2 não precisar de valor
   por-equation. Se C2 só precisa do contador final
   (improvável — equations são numeradas conforme aparecem),
   `flat_counter(key)` resolve. Se C2 precisa do valor na
   altura da equation, **só `flat_counter_at(key,
   location)` resolve** — e isso traz o problema "Layouter
   não conhece a sua location" (P183B causa raíz).

Output: tabela com observação legacy vs Introspector + 
decisão (`prosseguir` / `gate substancial`).

**Critério de saída e gate de decisão**:
- Se `.B.4` confirma snapshot-final no Introspector:
  cláusula gate **substancial** — saltar para `.G`,
  documentar bloqueio, abrir DEBT em P183F.
- Se `.B.5` confirma location-awareness OU C2 só precisa
  de contador final: prosseguir para `.C`.

### .C Actualizar L0 `entities/introspector.md`

1. Adicionar entrada para método novo `flat_counter`.

2. Hash em branco aguarda recálculo.

(Apenas se `.B` autorizou.)

### .D Adicionar método ao trait + impl

1. Em `01_core/src/entities/introspector.rs`:
   - Adicionar declaração ao trait `Introspector`.
   - Implementar em `impl Introspector for TagIntrospector`.
   - Delegar a sub-store apropriado (provavelmente
     `CounterRegistry`).

2. 5 tests unitários (padrão P181F / P182B):
   - Vazio devolve `None`.
   - Após populate retorna `Some(Vec)`.
   - Keys distintas isoladas.
   - Casos edge.

(Apenas se `.B` autorizou.)

### .E Migrar consumer C2

1. Em `01_core/src/rules/layout/equation.rs:97`:
   - Substitution-with-fallback.

2. Confirmar trait import local.

3. Cabeçalho `@prompt-hash` actualiza após edit do L0.

(Apenas se `.B` autorizou.)

### .F Tests E2E

1. Submódulo `p183c_equation_counter` em `tests.rs`.

2. 3 tests sugeridos:
   - Pipeline completo via Introspector.
   - Pipeline via fallback legacy.
   - Paridade `layout()` legacy vs `layout_with_introspector`.

3. Tests existentes não regridem.

(Apenas se `.B` autorizou.)

### .G Escalada para DEBT (caso de bloqueio)

Se `.B` declarou gate substancial:

1. Reverter qualquer experimentação (se houver — `.A`–`.B`
   são apenas leituras, não devem ter modificado código).

2. Documentar achados em
   `00_nucleo/diagnosticos/diagnostico-p183c-bloqueio.md`
   (curto; padrão P183B reverso):
   - Premissa testada: `flat_counter` Introspector tem
     paridade semântica com `get_flat` legacy.
   - Resultado: falsa (mesma natureza snapshot-final
     descoberta em P183B).
   - Categoria confirmada: "consumers que precisam de
     valor durante o walk, não do final".
   - Implicação: C1, C2 e provavelmente C3 ficam
     bloqueados. C4 ainda em aberto (categoria diferente
     — labels). DEBT M4-residual em P183F deve cobrir
     C1+C2+C3.

3. **Não** abrir DEBT em P183C — DEBT é aberto
   formalmente em P183F (passo de fecho da série),
   acumulando todos os bloqueados.

4. Saltar para `.I` (encerramento adaptado).

### .H Verificação estrutural (caso de sucesso)

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P183A
   baseline (1.756): +8 (5 unit em `.D` + 3 E2E em
   `.F`).
3. `crystalline-lint .` zero violations.
4. `flat_counter` declarado no trait; impl delega
   correctamente.
5. Consumer C2 (`equation.rs:97`) consulta Introspector
   primeiro; fallback legacy.
6. Walk arm canonical legacy **NÃO modificado**.
7. Trait `Introspector` ganhou 1 método novo.
8. Sub-store **NÃO modificado** (apenas exposto via
   trait).
9. Snapshot tests ADR-0033 verdes.
10. Linter passa final.

### .I Encerramento

Escrever
`00_nucleo/materialization/typst-passo-183c-relatorio.md`
com:

**Caso de sucesso**:
- Resumo: C2 equation counter migrado; método trait
  `flat_counter` adicionado; substitution-with-fallback;
  semântica confirmada compatível em `.B`.
- Confirmação `.H` (10 verificações).
- Δ tests vs baseline P183A (esperado +8).
- Hashes finais de L0s modificados.
- Estado actual:
  - P183 série: A ✅ B ❌ (bloqueado, escalado)
    C ✅ | D-F pendentes.
  - **M5/M4 progresso**: 5+1 = 6 read-sites migrados
    (P168 + P181G ×2 + P182D ×2 + P183C ×1; **C1
    permanece legacy** até DEBT M4-residual ser tratado).
  - 36 passos executados.
- Próximo passo: P183D (C3 figure auto-number — auditoria
  semântica obrigatória; provável bloqueio).

**Caso de bloqueio**:
- Resumo: P183C bloqueado por gate substancial em `.B`.
  Categoria confirmada (snapshot-final). Tudo revertido.
  Baseline 1.756 mantido.
- Diagnóstico em `diagnostico-p183c-bloqueio.md`.
- Estado actual:
  - P183 série: A ✅ B ❌ C ❌ (ambos bloqueados pela
    mesma causa).
  - DEBT M4-residual será aberto em P183F cobrindo C1
    + C2 + (provavelmente) C3.
  - 36 passos executados.
- Próximo passo: **decisão humana**:
  - Saltar P183D + P183E e ir directamente a P183F com
    DEBT cobrindo C1, C2, C3.
  - OU: P183D primeiro para confirmar C3 bloqueado;
    depois P183E (C4 — categoria diferente, deve passar);
    depois P183F.

---

## Critério de conclusão

Caso sucesso (todas em conjunto):

1. `.A` produziu auditoria sem disparar gate em pré-
   condição.
2. `.B` confirmou paridade semântica.
3. L0 `entities/introspector.md` actualizado.
4. Trait + impl `flat_counter`.
5. 5 tests unit + 3 tests E2E passam.
6. Consumer C2 migrado.
7. Verificações `.H` passam (10/10).
8. Relatório `.I` escrito (caso sucesso).
9. Output observable em produção inalterado.

Caso bloqueio:

1. `.A` + `.B` executados.
2. `.B` declarou gate substancial.
3. Diagnóstico em
   `diagnostico-p183c-bloqueio.md` escrito.
4. Tudo revertido (zero código tocado em produção).
5. Tests workspace 1.756 inalterado.
6. Relatório `.I` escrito (caso bloqueio).
7. P183F vai abrir DEBT M4-residual.

---

## O que pode sair errado

- **`get_flat` legacy retorna assinatura diferente**:
  cláusula gate trivial — adaptar.
- **Sub-store correcto não é `CounterRegistry`** mas outro:
  cláusula gate trivial — auditoria `.A.4` revela.
- **`.B` confirma gate substancial**: caso esperado dado
  o aprendizado P183B. Procede para `.G` em vez de `.C`.
  **Não é falha de execução** — é o output válido do
  diagnóstico.
- **`.B` retorna ambíguo** (legacy lê valor durante walk
  mas Layouter consulta após walk): investigar mais antes
  de prosseguir. Pode ser que C2 seja diferente de C1
  (equation tem 1 leitura por equation; heading tem
  hierarquia).
- **Tests E2E `.F` divergem**: indica que mesmo após `.B`
  passar, há divergência runtime. Cláusula gate
  substancial atrasada — reverter como P183B fez.
  **Esperado raro** se `.B` for honesta.
- **Linter divergência**: cláusula gate trivial.

---

## Notas operacionais

- **Tamanho**: S no caso de sucesso (~80-150 LOC).
  Trivial no caso de bloqueio (apenas leituras).
- **Sem dependências externas novas**.
- **Sub-passos `.B` é o crítico** — auditoria semântica
  explícita que P183B não fez.
- **Não tocar código de produção em `.A` + `.B`**. Só
  `.D`+`.E` tocam código.
- **Não escrever tests `.F` antes de `.D`+`.E`**. Tests
  só fazem sentido após migração; em caso de bloqueio,
  tests não são escritos.
- **Padrão replicado** (caso sucesso): P181F + P182B
  (método trait novo + impl + tests).
- **Aprendizado P183B**: paridade semântica entre
  primitivas Introspector e legacy é **falsa por defeito
  para contadores**. Auditoria semântica é obrigatória
  antes de qualquer migração de consumer que leia
  contadores.
- **Pendência paralela P182E §5.2** (location-aware
  Introspector em M6) é a solução final: quando `flat_counter_at(key,
  location)` existir e Layouter conhecer a sua location,
  C1+C2+C3 desbloqueam. Isso é trabalho substancial
  (M6+ ou M+).
- **Recomendação para P183 inteiro**: assumir que C1,
  C2, C3 vão para DEBT M4-residual; C4 (labels) é
  candidato a prosseguir. P183 inteiro pode acabar como
  "1 consumer migrado (C4) + 1 DEBT M4-residual com 3
  consumers + 1 DEBT separado para C5". Magnitude real:
  S em vez de S-M cumulativo.
