# Passo P183D — C3 figure auto-number per kind

Terceiro passo de implementação P183 (após P183A diagnóstico,
P183B bloqueado, P183C bloqueado).
Magnitude **S** (caso sucesso) ou **trivial** (caso bloqueio
esperado).

P183B + P183C consolidaram **regra operacional dos 2 eixos**
para migração de consumers de contador (per relatório P183C
§6):

> Auditar antes de migrar em dois eixos:
> 1. **Existem dados** no sub-store correspondente para a
>    chave? (Verificar arm correspondente em `from_tags`.)
> 2. **A semântica temporal coincide**? (Snapshot-final do
>    Introspector vs valor mutável durante walk no legacy.)
> Se qualquer falhar, declarar gate substancial e escalar
> para DEBT.

P183D aplica a regra a C3 (figure auto-number per kind em
`01_core/src/rules/layout/mod.rs:435–439`,
`figure_numbers[kind][idx]` legacy).

**Expectativa empírica** (per análise pós-P183C):
- **Eixo 1** (semântica temporal): C3 lê durante layout
  walk; provável bloqueio mesma natureza de C1/C2.
- **Eixo 2** (existem dados): figures **são** locatable em
  cristalino (P165/P168). `figure_label_numbers` está
  populado em sub-store. Eixo 2 esperado **OK**.

Resultado provável: **bloqueio por eixo 1 apenas** (não eixo
2). DEBT M4-residual em P183F cobre C1+C2+C3 simultaneamente.

Se P183D detectar bloqueio em ambos os eixos (eixo 2 também
ausente — improvável dado P165/P168), DEBT M4-residual
expande para incluir trabalho de promoção de variant.

Após P183D (caso de sucesso, improvável):
- Trait `Introspector` ganha `figure_number_at_index(kind,
  idx)` ou similar.
- Consumer C3 consulta Introspector primeiro; fallback
  legacy.

Após P183D (caso de bloqueio, esperado):
- Tudo revertido (zero código tocado se `.A`+`.B` forem
  apenas leituras, igual a P183C).
- C3 confirmado bloqueado. Categoria "snapshot-during-walk"
  ratificada em terceiro consumer.
- DEBT M4-residual em P183F cobre C1+C2+C3.

**Pré-condição**: P183C concluído e bloqueado. Tests
workspace 1.756 verdes; zero violations. Trait
`Introspector` 15 métodos (sem `flat_counter` nem
`figure_number_at_index`).

**Restrições**:
- **Não** modificar walk arm canonical legacy.
- **Não** modificar write paralelo legacy.
- **Não** modificar copy-sites.
- **Não** assumir paridade semântica antes da auditoria
  `.B` confirmar empiricamente (regra operacional).
- **Não** prosseguir para `.C`+ se `.B` declarar gate
  substancial em qualquer eixo.
- **Não** tocar código de produção em `.A` + `.B` (apenas
  leitura).
- API pública preservada.
- Output observable em produção inalterado.

---

## Aprendizado P183B + P183C aplicado

P183C formalizou regra operacional dos 2 eixos. P183D é a
primeira aplicação **completa** dessa regra desde a sua
formulação.

Antes de qualquer migração, `.B` testa explicitamente:

- **Eixo 1**: para a chave `"<algum_kind_de_figure>"`,
  durante layout walk, o legacy retorna valores diferentes
  conforme posição? Se sim, snapshot-final do Introspector
  é insuficiente.
- **Eixo 2**: o sub-store correspondente tem dados para a
  chave? Se não, gate independente do eixo 1.

Identificação prévia da expectativa:
- C3 figure auto-number — eixo 1 provavelmente falha; eixo
  2 provavelmente OK.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar consumer C3 actual:
   - `01_core/src/rules/layout/mod.rs:435–439` (per P183A
     §2).
   - Localizar leitura: padrão esperado
     `state.figure_numbers.get(kind).and_then(|v|
     v.get(idx))` ou similar.
   - Identificar contexto (arm `Content::Figure` em
     Layouter? que valor é retornado? como é usado?).

2. Confirmar tipo de retorno:
   - `Option<&usize>`, `usize`, ou outro.
   - É indexado por `kind` (`String`) e por `idx` (`usize`)?
     Confirma estrutura `HashMap<String, Vec<usize>>` ou
     similar em `state.figure_numbers`.

3. Confirmar sub-store equivalente em `Introspector`:
   - `figure_label_numbers` em `LabelRegistry` ou
     sub-store dedicado (P165/P168 estabeleceu).
   - Confirmar empiricamente: que método existe?
     `figure_number_for_label`, ou outro?
   - Se P168 já expôs algum método relevante para C3,
     identificar.

4. Confirmar walk arm `Content::Figure`:
   - `01_core/src/rules/introspect.rs` arm `Content::Figure`.
   - Como é populado `state.figure_numbers[kind]`?
     Esperado: arm avança contador por kind (1, 2, 3...)
     conforme cada figure aparece.

5. Confirmar se figures são locatable:
   - `01_core/src/rules/introspect/locatable.rs` arm
     `Content::Figure`.
   - Esperado: `true` (P168 estabeleceu).
   - Confirmar `extract_payload` arm produz
     `ElementPayload::Figure {...}` ou similar.

6. Confirmar `from_tags` arm `Figure`:
   - `01_core/src/rules/introspect/from_tags.rs` arm
     `ElementPayload::Figure`.
   - Como popula sub-store? Confirmar que entries para
     kind são registadas.

### .B Auditoria semântica explícita (regra dos 2 eixos)

#### .B.1 Eixo 2 — existem dados no Introspector?

1. Construir documento de teste:
   ```
   Figure 1 (kind: "image")
   Figure 2 (kind: "image")
   Figure 3 (kind: "table")
   ```

2. Pipeline: walk → from_tags → inspeccionar
   `TagIntrospector` interno.
   - Sub-store `LabelRegistry` (ou outro relevante per .A.3)
     contém entries para `"image"` e `"table"`?
   - Há uma forma de obter "qual é o número da figure N do
     kind X"?

3. **Resultado esperado**: eixo 2 OK (P168 popula sub-store
   para figures locatable).

4. Se eixo 2 confirma OK: prosseguir para `.B.2`.

5. Se eixo 2 falha (sub-store vazio para figure kinds):
   gate substancial **eixo 2**. Escalar para `.G` —
   bloqueio mais severo que C1/C2 (precisaria de promoção
   de variant + arm). Documentar e parar.

#### .B.2 Eixo 1 — semântica temporal coincide?

1. No mesmo documento de teste:
   ```
   Figure 1 (image)  // legacy figure_numbers["image"][0] = 1
   Figure 2 (image)  // legacy figure_numbers["image"][1] = 2
   Figure 3 (image)  // legacy figure_numbers["image"][2] = 3
   ```

2. Durante o layout walk, observar:
   - **Path legacy**: `state.figure_numbers["image"][0]`
     retorna `1` na altura de Figure 1, `1` (já fixado)
     em qualquer altura posterior.
     - **Nota**: figures são contador **acumulativo final**
       não mutável intra-figure — quando a Figure N é
       processada, `figure_numbers["image"]` já contém
       todos os valores 1..N (e potencialmente futuros se
       walk completou antes de layout começar).
     - Se assim é, eixo 1 pode **OK**, divergente de C1/C2!

3. Verificar empiricamente:
   - Quando o Layouter consulta `figure_numbers[kind][idx]`,
     o `state` legacy está em snapshot final pós-walk OU
     mutável durante layout?
   - Se snapshot final pós-walk: **eixo 1 OK** — Introspector
     também tem snapshot final; paridade alcançável.
   - Se mutável durante layout: **eixo 1 falha** — mesma
     natureza de C1/C2.

4. **Hipótese alternativa**: figures podem ter contador
   "calculado durante walk e fixado por figure" (como cada
   figure tem o seu número fixo). Diferente de heading
   (cuja numeração depende de hierarquia que pode mudar
   durante re-walk). Se for o caso, C3 é **migrável** com
   primitiva `figure_number_at_index(kind, idx)`.

5. Confirmar empiricamente. Se ambos paths retornam
   sequência idêntica `[1, 2, 3]` para `"image"`: eixo 1
   **OK**. Prosseguir para `.C`.

6. Se path Introspector retorna sequência diferente
   (truncada, modificada, ou sempre o último valor): gate
   substancial eixo 1. Escalar para `.G`.

#### .B.3 Decisão

- **Ambos eixos OK**: caso sucesso. Prosseguir `.C`–`.H`.
  C3 é migrável; pode ser que figures tenham natureza
  diferente de heading/equation.
- **Eixo 1 falha (provável)**: gate substancial. Escalar
  para `.G`. C3 vai para DEBT M4-residual com C1+C2.
- **Eixo 2 falha (improvável)**: gate substancial mais
  severo. Escalar para `.G` com nota adicional.

Output: tabela com observações + decisão.

### .C Actualizar L0 `entities/introspector.md`

(Apenas se `.B` autorizou prosseguir.)

1. Adicionar entrada para `figure_number_at_index(&self,
   kind: &str, idx: usize) -> Option<usize>` (ou similar
   conforme `.A.3`).

2. Hash em branco aguarda recálculo.

### .D Adicionar método ao trait + impl

(Apenas se `.B` autorizou prosseguir.)

1. Em `01_core/src/entities/introspector.rs`:
   - Declaração no trait.
   - Impl em `TagIntrospector` delega a sub-store
     identificado em `.A.3`.

2. 5 tests unitários (padrão P181F / P182B):
   - Vazio devolve `None`.
   - Após populate retorna `Some(...)`.
   - Kinds distintos isolados.
   - Índices fora de range devolvem `None`.
   - Casos edge.

### .E Migrar consumer C3

(Apenas se `.B` autorizou prosseguir.)

1. Em `01_core/src/rules/layout/mod.rs:435–439`:
   - Substitution-with-fallback.

2. Trait import local se necessário.

3. Cabeçalho `@prompt-hash` actualiza.

### .F Tests E2E

(Apenas se `.B` autorizou prosseguir.)

1. Submódulo `p183d_figure_auto_number` em `tests.rs`.

2. 3 tests sugeridos:
   - Pipeline completo via Introspector.
   - Pipeline via fallback.
   - Paridade legacy vs migrated.

3. Tests existentes não regridem.

### .G Escalada para DEBT (caso de bloqueio — esperado)

Se `.B` declarou gate em qualquer eixo:

1. **Não há código a reverter** (`.A` + `.B` foram apenas
   leituras + observação).

2. Documentar achados em
   `00_nucleo/diagnosticos/diagnostico-p183d-bloqueio.md`
   (curto; padrão P183C):
   - Eixo 1 (semântica temporal): resultado.
   - Eixo 2 (dados em sub-store): resultado.
   - Categoria confirmada: "snapshot-during-walk" — para
     C3 confirma 3 consumers nessa categoria.
   - Se eixo 2 também falhar: registo adicional de
     bloqueio infraestrutural.

3. **Não** abrir DEBT em P183D — DEBT é aberto formalmente
   em P183F (passo de fecho da série), acumulando
   C1+C2+C3 (e quaisquer outros bloqueados).

4. Saltar para `.I` (encerramento adaptado).

### .H Verificação estrutural (caso de sucesso)

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P183A
   baseline (1.756): +8 (5 unit em `.D` + 3 E2E em `.F`).
3. `crystalline-lint .` zero violations.
4. `figure_number_at_index` declarado e implementado.
5. Consumer C3 (`mod.rs:435–439`) consulta Introspector
   primeiro; fallback legacy.
6. Walk arm canonical legacy **NÃO modificado**.
7. Trait `Introspector` ganhou 1 método novo.
8. Sub-store **NÃO modificado** (apenas exposto).
9. Snapshot tests ADR-0033 verdes.
10. Linter passa final.

### .I Encerramento

Escrever
`00_nucleo/materialization/typst-passo-183d-relatorio.md`.

**Caso de sucesso** (formato P182X bem-sucedido):
- Resumo: C3 migrado; figures têm semântica diferente de
  heading/equation; eixos 1+2 OK.
- Confirmação `.H` (10 verificações).
- Δ tests vs baseline P183A (esperado +8).
- Hashes finais L0s modificados.
- Estado actual:
  - P183 série: A ✅ B ❌ C ❌ D ✅ | E-F pendentes.
  - **M5/M4 progresso**: 5+1 = 6 read-sites migrados.
  - 37 passos executados.
- Próximo passo: P183E (C4 resolved label).

**Caso de bloqueio** (formato P183C):
- Resumo: P183D bloqueado em `.B`. Categoria
  "snapshot-during-walk" confirmada em terceiro consumer.
- Diagnóstico em `diagnostico-p183d-bloqueio.md`.
- Estado actual:
  - P183 série: A ✅ B ❌ C ❌ D ❌ | E-F pendentes.
  - DEBT M4-residual em P183F cobre C1+C2+C3.
  - 37 passos executados.
- Próximo passo: P183E (C4, categoria diferente — labels
  não são contadores incrementais).

---

## Critério de conclusão

**Caso sucesso**:

1. `.A` produziu auditoria sem disparar gate em pré-
   condição.
2. `.B` confirmou ambos eixos OK.
3. L0 `entities/introspector.md` actualizado.
4. Trait + impl `figure_number_at_index`.
5. 5 tests unit + 3 tests E2E passam.
6. Consumer C3 migrado.
7. Verificações `.H` passam (10/10).
8. Relatório `.I` (sucesso) escrito.
9. Output observable em produção inalterado.

**Caso bloqueio** (esperado):

1. `.A` + `.B` executados.
2. `.B` declarou gate em eixo 1 (e/ou eixo 2).
3. Diagnóstico em `diagnostico-p183d-bloqueio.md` escrito.
4. **Sem código tocado** (`.A`+`.B` são leituras).
5. Tests workspace 1.756 inalterado.
6. Relatório `.I` (bloqueio) escrito.
7. P183F vai abrir DEBT M4-residual cobrindo C1+C2+C3.

---

## O que pode sair errado

- **Eixo 1 surpreendentemente OK**: figures podem ter
  natureza de "contador fixado por figure após walk
  completar" diferente de heading/equation (que recalculam
  durante layout). Se sim, P183D **passa** e C3 é migrável.
  Documentar a descoberta.
- **Eixo 2 falha** (improvável): figure kinds não estão
  populadas no sub-store actualmente. Cláusula gate
  substancial mais severa que C1/C2 — exige promoção de
  variant ou trabalho infraestrutural. DEBT M4-residual
  em P183F regista o achado.
- **Sub-store correcto não é claro** em `.A.3`: cláusula
  gate trivial — pode ser `LabelRegistry` (figure por
  label) ou `CounterRegistry` (figure por kind). Se
  ambíguo, P183D documenta as duas hipóteses e testa
  empiricamente em `.B`.
- **`.B.2` ambíguo** (legacy é populado pré-layout mas
  Layouter não reflete location consciente): cláusula
  gate substancial — recuar como C2.
- **Test E2E `.F` diverge mesmo após `.B` OK**: gate
  substancial atrasado — reverter como P183B fez.
  Esperado raro.
- **Linter divergência**: cláusula gate trivial.

---

## Notas operacionais

- **Tamanho**: trivial (caso bloqueio, esperado) ou S
  (caso sucesso, ~80-150 LOC).
- **Sem dependências externas novas**.
- **Sub-passo `.B` é o crítico** — auditoria semântica
  dos 2 eixos.
- **Não tocar código de produção em `.A` + `.B`**.
- **Não escrever tests `.F` antes de `.D`+`.E`**.
- **Padrão replicado** (caso sucesso): P181F + P182B
  (método trait novo + impl + tests).
- **Aprendizado P183C aplicado** literalmente: regra
  operacional dos 2 eixos como sub-passo `.B` explícito.
- **Possível descoberta favorável**: figures podem ter
  natureza diferente de heading/equation. Se eixo 1
  passar, P183 ganha 1 consumer migrado e DEBT
  M4-residual cobre apenas C1+C2 em vez de C1+C2+C3.
- **Pendência paralela P182E §5.2** continua relevante:
  mesmo que C3 passe, C1+C2 esperam location-aware
  Introspector em M6. Pendência inalterada.
