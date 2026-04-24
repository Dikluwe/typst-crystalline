# Passo 141 — Array fallback chain (gap 6 DEBT-52; ADR-0055 `IMPLEMENTADO`)

**Série**: 141 (passo **XS** em L3 + edição L0; segunda
materialização da Fase C do roadmap DEBT-1, par com 140B).
**Precondição**: Passo 140B encerrado; 1111 total tests; zero
violations; **55 ADRs** (ADR-0055 em `PROPOSTO`); **13 DEBTs
abertos** com DEBT-52 em 5/8 gaps resolvidos; hash L0 de
`prompts/infra/pipeline.md` = `367f8790`.

**ADRs aplicáveis**:
- **ADR-0055** — decisão 4 materializada neste passo. **Transita
  de `PROPOSTO → IMPLEMENTADO`** ao fim de 141 (par 140B+141
  completa a "paridade básica" da decisão).
- **ADR-0033** (paridade funcional) — array fallback é
  comportamento vanilla directo.
- **ADR-0054** (critério fecho DEBT-1) — após 141, gaps 7 e 8
  são opcionais; DEBT-1 pode fechar.
- **ADR-0053** (FontList materializado) — `as_slice()` é o ponto
  de iteração.

**Natureza**: passo **L3** (alteração de `resolve_font`) +
edição **L0** (`prompts/infra/pipeline.md` — spec do loop de
famílias). **Zero alteração em L1 de domínio**. **Zero crates
novas**.

**Tamanho estimado**: **XS**, ~45min (conforme roadmap 140A e
confirmado pelo relatório 140B).

---

## Contexto

Passo 140B fechou gap 5 (consumer `font` string) com MVP
**single-family**: `resolve_font` só tenta a primeira família
de `FontList::as_slice()`. Comportamento actual para `#set
text(font: ("Inria Serif", "Arial", "sans-serif"))`:

- Se "Inria Serif" resolve → embutida.
- Se não resolve → fallback Helvetica **directo**, sem tentar
  "Arial" nem "sans-serif".

Isto diverge de vanilla, que itera a lista até achar match.
Gap 6 de DEBT-52 existe exactamente para capturar esta
divergência.

A correcção é **cirúrgica**: substituir o `.first()?` em
`resolve_font` por um `for`/`iter().find_map` que tenta cada
família em ordem e devolve `Some(bytes)` na primeira que
resolve. Nenhuma estrutura nova, nenhuma crate nova, nenhuma
mudança de assinatura pública.

Após 141, **ADR-0055 transita para `IMPLEMENTADO`** —
completa a "paridade básica" da decisão (par 140B+141 é a
unidade lógica declarada na própria ADR e no enunciado do 140B).

---

## Objectivo

Ao fim do passo:

1. **`resolve_font` itera** `font_list.as_slice()` em ordem;
   primeira família cuja `FontBook::select` devolve `Some`
   vence. Se nenhuma resolve, devolve `None` (fallback
   Helvetica preservado).

2. **Testes unitários L3** adicionados ao módulo `pipeline`
   cobrindo 4 cenários de posição de match na lista:
   - Match no índice 0 (única ou primeira).
   - Match no índice 1 (primeira falha, segunda vence).
   - Match no índice 2 (duas primeiras falham).
   - Nenhum match (todas falham → `None`).

3. **Teste L3 de integração** adicionado a
   `integration_tests.rs` cobrindo fallback chain end-to-end
   com famílias reais/sintéticas (com `discover_any_system_fonts`
   + early-return como no 140B).

4. **DEBT-52 gap 6 marcado `[x]`** em `DEBT.md`.

5. **ADR-0055 transita `PROPOSTO → IMPLEMENTADO`** com nota
   de materialização identificando Passos 140B + 141.

6. **L0 `prompts/infra/pipeline.md`** actualizado:
   - Spec de `resolve_font` passa a descrever o loop de
     famílias.
   - Hash recalculado; header de `03_infra/src/pipeline.rs`
     actualizado.

Este passo **não**:

- Toca L1 de domínio (FontBook, Font, World, FontList, StyleChain).
- Adiciona crates novas.
- Implementa multi-font per document (Passo 142, opcional).
- Ataca selecção variant-aware (ADR-0055bis candidata).
- Fecha DEBT-1 (fecho é acção separada após 141, fora deste
  passo — candidato a passo dedicado ou entrada curta
  no DEBT.md).
- Adiciona fixture de fonts em `tests/fixtures/` (limitação 7
  do 140B permanece, ataque em passo dedicado futuro).

---

## Decisões já tomadas (herdadas de 140B + ADR-0055)

1. **Iteração sequencial, primeira-que-resolve-vence**. Vanilla
   semantics. Sem scoring, sem preferência de variant, sem
   fallback score.
2. **`FontVariant::default()` continua**. Variant-aware é
   ADR-0055bis candidata; não entra aqui.
3. **Single-font per document preservado**. Array fallback é
   **dentro de uma** `FontList`; `first_font_from_doc`
   continua a devolver a **primeira** `FontList` do documento.
   Multi-font per document é Passo 142 distinto.
4. **Silent drop** quando nenhuma família resolve → fallback
   Helvetica.
5. **ADR-0055 passa a `IMPLEMENTADO` neste passo**.

## Decisões diferidas (resolvidas neste passo)

6. **Short-circuit ou iteração completa com log**: decisão em
   141.2. Proposta: short-circuit estrito (primeira que
   resolve → return). Zero telemetria. Coerente com decisão 4
   (silent drop). Revisitar se utilizadores reportam
   surpresa.
7. **Preservação da limitação "famílias não tentadas não são
   registadas"**: aceite. Se gap aparecer no futuro, abrir
   DEBT dedicado.
8. **Tipo concreto de iterador**: `for` explícito vs
   `iter().find_map`. Decisão em 141.2, sem impacto
   observável — matéria de estilo.

---

## Escopo

**Dentro**:

- Modificação de `resolve_font` em `03_infra/src/pipeline.rs`.
- 4 testes unitários novos no mesmo módulo.
- 1 teste de integração novo em `integration_tests.rs`.
- Edição de `prompts/infra/pipeline.md` (spec + hash).
- Actualização de header em `03_infra/src/pipeline.rs`
  (novo `@prompt-hash`).
- Actualização de `DEBT.md` (gap 6 → `[x]`).
- Actualização de `ADR-0055` (status + nota de
  materialização).
- Actualização de `00_nucleo/adr/README.md` se tiver índice
  de status (0055 passa de `PROPOSTO` a `IMPLEMENTADO` na
  tabela).

**Fora**:

- Qualquer alteração em L1.
- `first_font_from_doc` (continua a devolver a primeira
  `FontList`; não é tocada).
- `export_pdf_with_font` e `build_cidfont` (assinaturas
  preservadas).
- Multi-font per document (Passo 142).
- Variant-aware (ADR-0055bis).
- Fixture de fonts em CI (passo dedicado futuro).
- Telemetria sobre famílias tentadas-e-falhadas.

---

## Sub-passos

### 141.1 — Confirmar assinatura actual de `resolve_font`

`view 03_infra/src/pipeline.rs` — localizar `fn resolve_font`
(privado ao módulo, assinatura conhecida do relatório 140B):

```rust
fn resolve_font(
    font_list: &FontList,
    font_book: &FontBook,
    world:     &dyn World,
) -> Option<Vec<u8>>;
```

Confirmar:
- Corpo actual usa `font_list.as_slice().first()?` (ou
  equivalente semântico do 140B).
- `FontBook::select(name, variant)` devolve `Option<usize>`.
- `world.font(index)` devolve `Option<Font>`.
- `Font::as_slice()` expõe os bytes.

### 141.2 — Modificar corpo de `resolve_font`

Substituição central:

```rust
fn resolve_font(
    font_list: &FontList,
    font_book: &FontBook,
    world:     &dyn World,
) -> Option<Vec<u8>> {
    let variant = FontVariant::default();
    for family in font_list.as_slice() {
        if let Some(index) = font_book.select(family.name(), &variant) {
            if let Some(font) = world.font(index) {
                return Some(font.as_slice().to_vec());
            }
        }
    }
    None
}
```

(Forma final pode usar `iter().find_map` — decisão em 141.2).

Semântica: primeira família com `select` a devolver `Some` **e**
`world.font` a devolver `Some` vence. Se `select` devolve `Some`
mas `world.font` devolve `None` (cenário patológico: índice
stale), **continua a tentar** as famílias seguintes.

### 141.3 — Testes unitários (4)

Usar o mesmo mock `FontMockWorld` do 140B. Adicionar os testes
ao módulo `pipeline::tests`:

1. `resolve_font_lista_match_indice_0`
   - `FontList::from(["A", "B", "C"])`.
   - Mock resolve "A".
   - Assert: devolve bytes de "A".

2. `resolve_font_lista_match_indice_1`
   - `FontList::from(["X", "B", "C"])`.
   - Mock resolve "B" mas não "X".
   - Assert: devolve bytes de "B".

3. `resolve_font_lista_match_indice_2`
   - `FontList::from(["X", "Y", "C"])`.
   - Mock resolve "C" mas não "X" nem "Y".
   - Assert: devolve bytes de "C".

4. `resolve_font_lista_sem_match_devolve_none`
   - `FontList::from(["X", "Y", "Z"])`.
   - Mock não resolve nenhuma.
   - Assert: `None`.

Os 3 testes unitários do 140B (`match_primeiro`, `nao_match`,
`font_book_vazio`) continuam válidos e **não são alterados** —
`match_primeiro` corresponde ao novo caso "lista de
tamanho 1, match imediato".

### 141.4 — Teste de integração L3

Ficheiro: `03_infra/src/integration_tests.rs` (mod
`integration`, junto aos 4 testes `font_wiring_*` do 140B).

```
font_wiring_array_fallback_primeira_falha_segunda_vence
```

- Usa `discover_any_system_fonts()` do 140B para descobrir
  uma família real.
- Constrói documento: `#set text(font: ("FontQueNaoExiste",
  "<família-real>")); Hello`.
- Compila via `compile_to_pdf_bytes`.
- Assert: PDF contém marker `CrystallineFont` (Type0) →
  segunda família embutida.
- Early-return com `eprintln!("[skip] ...")` se probe devolve
  `None` (mesmo padrão do 140B).

### 141.5 — Actualizar DEBT-52

Diff em `00_nucleo/DEBT.md`:

```diff
- - [ ] Consumer `font` array (fallback chain).
+ - [x] Consumer `font` array (fallback chain).
+       **Resolvido no Passo 141** — `resolve_font` itera
+       `FontList::as_slice()`; primeira família que
+       `FontBook::select` resolve vence. Semântica vanilla
+       directa. Zero estrutura nova.
```

Actualizar contagem no cabeçalho de DEBT-52 se existir (5/8 →
6/8 gaps resolvidos). Gaps restantes: 7 (hyphenation, opcional)
e 8 (font dict, opcional).

### 141.6 — Actualizar ADR-0055

Em `00_nucleo/adr/typst-adr-0055-font-consumer-cidfont.md`:

```diff
- **Status**: `PROPOSTO`
+ **Status**: `IMPLEMENTADO`
```

Adicionar secção no final:

```markdown
## Materialização

- **Passo 140B** (2026-04-24) — wiring single-font
  (decisão 3). `compile_to_pdf_bytes` despacha para
  `export_pdf_with_font` quando primeira família resolve.
  Gap 5 DEBT-52 fechado.
- **Passo 141** (2026-04-24) — array fallback chain
  (decisão 4). `resolve_font` itera todas as famílias até
  primeira resolver. Gap 6 DEBT-52 fechado. **Paridade
  básica da ADR completa**.

Decisões 5 (multi-font), 6 (lang hyphenation), 7 (rustybuzz)
permanecem scope-out conforme definido originalmente. Não
bloqueiam fecho desta ADR.
```

Actualizar `00_nucleo/adr/README.md` se tiver tabela de status
de ADRs:

```diff
- | 0055 | Font consumer via pipeline CIDFont existente | `PROPOSTO` |
+ | 0055 | Font consumer via pipeline CIDFont existente | `IMPLEMENTADO` |
```

### 141.7 — Edição L0 `prompts/infra/pipeline.md`

Modificar a secção "Helpers privados de dispatch" (introduzida
em 140B):

```diff
- ### `resolve_font(font_list, font_book, world) -> Option<Vec<u8>>`
-
- Pega na primeira família de `font_list.as_slice()`. Consulta
- `font_book.select(name, &FontVariant::default())`. Se match,
- chama `world.font(index)` e devolve os bytes. MVP
- single-family; array fallback é Passo 141.
+ ### `resolve_font(font_list, font_book, world) -> Option<Vec<u8>>`
+
+ Itera `font_list.as_slice()` em ordem. Para cada família,
+ consulta `font_book.select(name, &FontVariant::default())`; se
+ devolve `Some(index)`, chama `world.font(index)`; se devolve
+ `Some(font)`, devolve os bytes. Primeira família a completar
+ ambos os passos vence. Se nenhuma completa, devolve `None`
+ (pipeline cai em fallback Helvetica).
+
+ Paridade com vanilla: semântica "primeira-que-resolve" do
+ `#set text(font: (...))`.
```

Actualizar nota de MVP se existir:

```diff
- MVP single-family; array fallback é Passo 141.
+ Array fallback materializado no Passo 141. Multi-font per
+ document (Passo 142, opcional) e variant-aware (ADR-0055bis,
+ candidata) permanecem fora do escopo actual.
```

### 141.8 — Recalcular hash + actualizar header

```
sha256sum 00_nucleo/prompts/infra/pipeline.md
```

Anotar primeiros 8 chars. Actualizar header de
`03_infra/src/pipeline.rs`:

```diff
- //! @prompt-hash 367f8790
- //! @updated 2026-04-24
+ //! @prompt-hash <novo>
+ //! @updated 2026-04-24
```

Confirmar com `crystalline-lint .` — zero `PromptDrift` (V5).

### 141.9 — Relatório

Ficheiro:
`00_nucleo/materialization/typst-passo-141-relatorio.md`.

Secções:
1. Sumário executivo.
2. Alteração em `resolve_font` (diff central).
3. Testes adicionados (4 unit + 1 integração).
4. DEBT-52 gap 6 fechado.
5. ADR-0055 transita a `IMPLEMENTADO`.
6. Hash L0 recalculado.
7. Limitações preservadas (variant-aware, multi-font, etc.).
8. Próximo passo: **encerrar DEBT-1** (passo curto separado ou
   entrada directa em DEBT.md) ou arrancar 142 opcional.
9. Verificação final.

---

## Verificação

1. ✅ `resolve_font` itera toda a `FontList`.
2. ✅ 4 testes unit cobrem matches em índices 0/1/2 e
   nenhum match.
3. ✅ 1 teste de integração L3 valida fallback end-to-end.
4. ✅ Os 3 testes unit do 140B + os 4 testes de integração
   `font_wiring_*` do 140B continuam verdes sem alteração.
5. ✅ `cargo test --workspace --lib`: 1111 → **1116** (+5).
6. ✅ `crystalline-lint .`: zero violations.
7. ✅ L1 intacto (git confirma sem diff em `01_core/`).
8. ✅ DEBT-52 gap 6 marcado `[x]`.
9. ✅ ADR-0055 `IMPLEMENTADO` com secção "Materialização".
10. ✅ `prompts/infra/pipeline.md` com spec actualizada;
    hash propagado a `03_infra/src/pipeline.rs`.

---

## Critério de conclusão

1. `resolve_font` com loop de famílias em produção.
2. Testes unit e integração verdes.
3. `cargo test --workspace` passa; contagem registada.
4. `crystalline-lint .` zero violations.
5. ADR-0055 `IMPLEMENTADO`.
6. DEBT-52 6/8 gaps fechados (gap 7 e 8 opcionais).
7. Hash L0 propagado.
8. Relatório 141 escrito.

---

## O que pode sair errado

- **Semântica divergente de vanilla em caso de múltiplos
  matches**: se `FontBook::select` for case-insensitive e
  devolver match "aproximado", famílias subsequentes podem
  nunca ser tentadas. Comportamento é correcto (primeira
  vence), mas pode surpreender se utilizadores esperam
  correspondência exacta. **Não é bug** — registar no
  relatório se testes revelarem casos específicos.

- **Iterador devolve `Some` em `select` mas `None` em
  `world.font`**: cenário patológico (índice stale). O corpo
  proposto **continua** a tentar famílias seguintes (via `for`
  + returns condicionais, **não** via `?`). Confirmar em
  141.2 que a forma final não curto-circuita em `None` de
  `world.font`.

- **`iter().find_map` mais elegante mas menos legível**: `for`
  explícito tem cada ramo visível. Decisão de estilo — sem
  impacto semântico.

- **Hash L0 muda mas `crystalline-lint --fix-hashes` não é
  invocado**: erro humano. Forçar verificação manual em 141.8
  e no relatório.

- **ADR-0055 transita antes de confirmar que 141 passa**: não
  fazer. Transição de status em 141.6 **só após** testes
  verdes em 141.3 e 141.4.

- **DEBT-1 fecha automaticamente?**: não. ADR-0054 exige
  passo explícito de fecho (com relatório a confirmar que
  todos os critérios são cumpridos). O fecho é **acção
  separada** — pode ser passo curto 141.5 ou entrada directa
  em DEBT.md, a decidir. Este enunciado **não** fecha
  DEBT-1.

---

## Notas operacionais

- **Par 140B+141 agora completo**. ADR-0055 paridade básica
  materializada em ~3h cumulativas, dentro da estimativa do
  roadmap 140A (2 passos, ~3h).

- **Fase C em 2 passos, não 4-5**. Estimativa 135 revisada no
  140A: realidade confirmou ganho. `rustybuzz` declarado-
  sem-uso continua — candidato DEBT-53 permanece por abrir
  até priorização.

- **Limitação variant-aware** é o candidato mais imediato
  para ADR-0055bis se paridade avançada for priorizada.
  Enquanto faux-bold (Passo 139) for aceitável, sem urgência.

- **Fecho de DEBT-1**: decisão pós-141. Candidatos:
  - Passo curto dedicado com relatório de fecho formal.
  - Entrada directa em `DEBT.md` movendo DEBT-1 para secção
    "encerrados" (Secção 2).
  - Adiar fecho até multi-font (142) ou hyphenation (143) se
    priorizados brevemente.
  Recomendação: passo curto dedicado, para registar claramente
  o cumprimento de ADR-0054.

- **Tests de fontes em CI** permanece limitação 7 do 140B.
  Quando CI reprodutível sem fonts do sistema for priorizado,
  abrir passo dedicado para fixture (DejaVu preferível;
  licença permissiva; README de proveniência).

- **`FontVariant::default()` permanece fixo**. Selecção
  variant-aware é ADR-0055bis. Não é urgente enquanto
  utilizadores usarem `weight` simulado pelo faux-bold
  (Passo 139).
