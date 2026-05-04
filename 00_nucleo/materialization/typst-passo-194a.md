# Passo 194A — Instrução Claude Code

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- Tests workspace 1.821 verdes; zero violations.
- M9 ✅ 11/11.
- M4-residual fechado funcionalmente (P188B).
- M5 incremental (P189B): 1 arm migrado + 6 excepções.
- P193B abriu sub-store `ResolvedLabelStore` + método
  trait `resolved_label_for(&Label) -> Option<&str>`.
- DEBT M5-residual: 3 pré-requisitos restantes (1 dos 4
  avançado).
- Trait `Introspector` 19 métodos.
- `TagIntrospector` 8 sub-stores.

P194 é **passo 2 da sequência §9 P189**: migração
consumer C4. Substitution-with-fallback em
`references.rs:53` (per P193A §2.4 confirmado
empiricamente):

```rust
// Antes (legacy):
let display_text = match layouter.counter.resolved_labels.get(target) {
    Some(text) => text.clone(),
    None       => format!("@{}", target.0),
};

// Depois (esperado):
let display_text = match layouter.introspector
    .resolved_label_for(target)
    .or_else(|| layouter.counter.resolved_labels.get(target).map(String::as_str))
{
    Some(text) => text.to_string(),
    None       => format!("@{}", target.0),
};
```

P194 não migra walk arms (P195+). Em produção, sub-store
P193B fica vazio (nenhum populate via Tag), logo
Introspector path retorna sempre `None` → fallback legacy
é caminho funcional permanente **até P195+ activarem
populate**. **Diferente de P188B** (Equation): aqui o
estado dormente é **temporário**, não permanente.

Material de partida verificado:

- `00_nucleo/materialization/typst-passo-193-relatorio-consolidado.md`
  §8 — P194 listado como próximo; blueprint
  substitution-with-fallback per P184D/P187B/P188B.
- `00_nucleo/materialization/typst-passo-187-relatorio-consolidado.md`
  + `00_nucleo/materialization/typst-passo-188-relatorio-consolidado.md`
  — padrão de migração consumer.
- P193A §2.4 + §11.3 — site C4 confirmado em
  `references.rs:53` com forma exacta.

P194A é o passo de diagnóstico que precede a
implementação. Magnitude esperada **S** — replicação de
padrão estabelecido com 2 sub-decisões locais (ownership
do String + tratamento copy-sites Layouter).

---

## Postura do auditor / executor

P194A é passo **L0-puro / diagnóstico-primeiro**, padrão
estabelecido em 15 aplicações.

- **Zero código tocado** em camadas cristalinas.
- **Zero testes** modificados.
- **Pode criar** ADR `PROPOSTO` — improvável (replicação
  de padrão).
- **Pode abrir DEBT** — improvável.
- **Não modifica** `references.rs`, `mod.rs`,
  `from_tags`, walk — P194B+.

**Magnitude diagnóstico**: S. Decisões esperadas são
locais (ownership, copy-sites, forma exacta).

**Regra dos 2 eixos aplicável** (P183C §6) — confirmar
empiricamente que migração não exige variante
location-aware face a uso real do consumer.

---

## Escopo

**Primário**: desenhar migração consumer C4 em
`references.rs:53` para substitution-with-fallback usando
`intr.resolved_label_for`.

**Confirmação**: validar contexto exacto, ownership,
tratamento de copy-sites Layouter (P193B identificou
`mod.rs:1481, 1512` mas não tocou).

**Decisões a tomar** — 6 cláusulas:

1. **Forma exacta da expressão de migração** — variante
   da substitution-with-fallback. 2 sub-decisões:
   ownership de String (clone vs map vs to_string) e
   forma do match.

2. **Tratamento do `None` do Introspector** — em
   produção real até P195, **sempre** `None`. Fallback
   legacy é caminho funcional. Diferente de C2 (P188B
   permanente) porque temporário.

3. **Tratamento dos copy-sites Layouter** — `mod.rs:1481, 1512`
   copiam `state.resolved_labels` para Layouters
   secundários. Após P194 o consumer C4 acede via
   Introspector (que não é copiado entre Layouters via
   esses sites — Introspector é partilhado). Decidir:
   - Manter copy-sites (legacy continua a ser usado por
     fallback).
   - Remover copy-sites (legacy fica morto após P194).
   - Avaliar se Layouters secundários têm acesso ao
     Introspector original.

4. **Acesso a `layouter.introspector` em
   `references.rs:53`** — confirmar que `layouter` neste
   contexto tem o field `introspector` acessível.

5. **Documentação inline** — comentário sobre estado
   temporário (vazio em produção até P195) ou apenas
   cross-reference a P194 sem detalhar?

6. **Critério de fecho de P194** — consumer migrado +
   tests E2E + relatório consolidado. Sem fechar
   excepções P189B (E2-E6 continuam até P195+).

**Fora de escopo**:

- Migração walk arm Labelled (P195).
- Migração walk arm Heading (P196).
- Sub-store `headings_for_toc` (lacuna #3).
- `SetEquationNumbering` materialização.
- Eliminação `CounterStateLegacy.resolved_labels` (M6).

---

## Critérios objectivos

### O1 — Inputs verificáveis

`grep -rn "resolved_labels\|resolved_label_for"
01_core/src/`. Para cláusula 1, confirmar contexto
exacto de `references.rs:53` (escopo, tipo de `target`,
tipo final esperado pelo caller). Para cláusula 3,
confirmar comportamento dos copy-sites em
`mod.rs:1481, 1512`.

### O2 — Alternativas

Mínimo 2 quando há margem real. Para cláusula 1
(ownership), 3 alternativas (clone vs map vs to_string).
Para cláusula 3 (copy-sites), 2-3 alternativas.

### O3 — Critério de escolha

Padrão P184D/P187B/P188B substitution-with-fallback.
Decisão sobre copy-sites depende de reuso real.

### O4 — Magnitude

Trivial vs substancial. Cláusulas 1, 4-6 são triviais;
cláusula 3 (copy-sites) pode ser substancial se forçar
mudança de assinatura.

### O5 — Reversibilidade

Substitution-with-fallback é reversível por construção.
Copy-sites: remoção é reversível mas exige cuidado se
houver outros consumers.

---

## Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

Migração replica P184D Figure / P187B C1 / P188B C2
literalmente? Diferença esperada: primitiva
`resolved_label_for` (sem location parameter — per P193).

### Q2 — Honestidade de magnitude

P194A diagnóstico é S. P194B+ implementação:
- Provável passo único agregado: migração + tests +
  relatório.
- Magnitude S esperada.

Total agregado: ~5-15 LOC produção + ~80 LOC tests +
relatório consolidado ≈ S.

### Q3 — Cobertura sem regressão

Tests existentes que cobrem cross-references: identificar
em `.A`. Migração não deve regredir.

**Importante**: como sub-store P193B fica vazio em
produção (até P195), Introspector path retorna `None`
imediatamente → fallback legacy chamado → resultado
idêntico ao actual. **Output observable preservado por
construção**.

### Q4 — Estado temporário (não permanente)

Diferente de C2 (P188B), aqui o estado dormente do
Introspector é **temporário**:
- P194: Introspector vazio; fallback funcional.
- P195: walks Labelled emitem Tag → Introspector
  populated → caminho Introspector activa.
- P196+: idem para Heading auto-toc.

Após P195+P196 (sequencialmente), Introspector torna-se
**caminho funcional real**; fallback fica redundante.
Janela compat até P200 (M6) eliminar
`CounterStateLegacy.resolved_labels`.

Documentação inline pode ser mais leve que P188B (que
era permanente) — referência simples a sequência §9 P189
basta.

### Q5 — Granularidade

Sub-passo único P194B agregado:
- Migração consumer.
- Tests E2E (preservação observable).
- L0 actualizado se aplicável.
- Decisão sobre copy-sites materializada.
- Relatório consolidado.

---

## Sub-passos de P194A

### Sub-passo 194A.A — Validação do estado actual

Auditor confirma empiricamente:

1. Confirmar consumer C4 actual:
   - `01_core/src/rules/layout/references.rs:53` (per
     P193A §2.4). Verificar empiricamente.
   - Localizar match completo (forma per P193A §11.3).
   - Identificar contexto exacto (função/método; tipo de
     `target`; tipo final retornado).

2. Confirmar acesso a `layouter.introspector`:
   - Tipo de `layouter` em `layout_ref` (assumido per
     P184D pattern: `&Layouter<M, S>` ou similar).
   - Confirmar que `layouter.introspector` é acessível.
   - Confirmar tipo (`Box<dyn Introspector>` ou similar).

3. Confirmar copy-sites Layouter:
   - `mod.rs:1481, 1512` (per P193A §2.5).
   - Forma exacta da cópia.
   - Identificar **se** Layouters secundários têm
     `introspector` próprio ou partilhado com o pai.
   - Aplicar regra dos 2 eixos: o consumer dos
     Layouters secundários precisa de
     `state.resolved_labels` populated, ou pode
     consultar o Introspector partilhado?

4. Confirmar tipo de retorno actual:
   - `match Some(text) => text.clone()`. Tipo é
     `String`.
   - `format!("@{}", target.0)` é também `String`.
   - Tipo final é `String`.

5. Confirmar API `resolved_label_for` (P193B):
   - `fn resolved_label_for(&self, label: &Label) -> Option<&str>`.
   - Retorna `&str`. Para clone, `.to_string()` ou
     `String::from`.

6. Confirmar tests existentes:
   - `grep -rn "layout_ref\|references.*test\|cross.ref" 01_core/src/`.
   - Identificar tests que cobrem o caminho.
   - **Esperado**: `layout_resolved_labels_nao_interfere_entre_documentos`
     ou similar. Devem manter-se inalterados (output
     observable preservado).

7. Confirmar nota DEBT M5-residual (per P193 §7):
   - Após P194: 3 → 2 pré-requisitos restantes.
   - Restantes: sub-store `headings_for_toc`,
     `SetEquationNumbering`.
   - Mas... aguarda P195+ para activar Introspector
     path.

   **Nota**: P194 reduz pré-requisitos para "C4
   migration" mas não destranca excepções E2-E6 — essas
   só fecham com P195+. Auditor decide nuance da
   actualização.

Output: tabela com item + estado + linhas exactas.

**Critério de saída**:
- Site C4 confirmado com contexto.
- Acesso a `layouter.introspector` confirmado.
- Copy-sites avaliados.
- Tipo de retorno preservado.
- Tests existentes inventariados.

### Sub-passo 194A.B — Decisão cláusula 1 (forma da expressão)

Avaliar a forma exacta da migração.

**Opção A** — Match com fallback inline:
```rust
let display_text = match layouter.introspector
    .resolved_label_for(target)
    .map(|s| s.to_string())
    .or_else(|| layouter.counter.resolved_labels.get(target).cloned())
{
    Some(text) => text,
    None       => format!("@{}", target.0),
};
```

**Opção B** — Match com fallback usando `or_else` para
texto literal:
```rust
let display_text = layouter.introspector
    .resolved_label_for(target)
    .map(|s| s.to_string())
    .or_else(|| layouter.counter.resolved_labels.get(target).cloned())
    .unwrap_or_else(|| format!("@{}", target.0));
```

**Opção C** — `Option<&str>` propagado, clone só no
final:
```rust
let display_text = layouter.introspector
    .resolved_label_for(target)
    .or_else(|| layouter.counter.resolved_labels.get(target).map(String::as_str))
    .map(String::from)
    .unwrap_or_else(|| format!("@{}", target.0));
```

Critério: P184D/P187B/P188B usaram `or_else` puro. Opção
B está mais alinhada com padrão. Opção C é mais elegante
para tipo (`&str` propagado) mas requer cuidado com
ownership.

Sugestão: **Opção B** — `or_else` chain + `unwrap_or_else`
para fallback literal. Coerente com P188B
(`unwrap_or_else` quando legacy não retorna `Option`).
Mas legacy aqui retorna `Option` (`get`), portanto
podemos usar `or_else` puro chained. Decisão final em
`.A`.

Output: decisão fixada com base em `.A.4` ownership.

### Sub-passo 194A.C — Decisão cláusula 2 (`None` do Introspector)

Em produção real até P195, `intr.resolved_label_for`
retorna sempre `None` (sub-store vazio per P193B). Fallback
chamado consistentemente.

**Opção A** — Aceitar comportamento; sem alteração de
lógica.
**Opção B** — Comentário inline explicando temporariedade.

Critério: simplicidade. Opção A com comentário leve em
`.E` cobre.

Sugestão: **Opção A** + comentário leve.

Output: decisão fixada.

### Sub-passo 194A.D — Decisão cláusula 3 (copy-sites Layouter)

`mod.rs:1481, 1512` copiam `state.resolved_labels` para
Layouters secundários. Após P194:

**Cenário 1** — Layouters secundários têm
`introspector` próprio:
- Cada Layouter constrói o seu Introspector
  independentemente.
- Copy-sites do `state.resolved_labels` continuam
  necessários para fallback dos Layouters secundários.
- **Não tocar copy-sites**.

**Cenário 2** — Layouters secundários partilham
Introspector com o pai:
- Único Introspector populated; Layouters secundários
  acedem via referência.
- Copy-sites do `state.resolved_labels` ficam
  redundantes (mas inofensivos durante janela compat).
- **Decidir**: remover copy-sites em P194 (cleanup) ou
  esperar até M6.

Critério: depende empiricamente do cenário em `.A.3`.
**Sugestão preliminar**: manter copy-sites independentemente
do cenário durante janela compat M5; remover em M6 quando
`CounterStateLegacy` for eliminado.

Output: cenário identificado + decisão de manter/remover.

### Sub-passo 194A.E — Decisão cláusula 4 (acesso a `layouter.introspector`)

Confirmar empiricamente em `.A.2` que `layouter.introspector`
é acessível em `layout_ref`. 3 cenários:

**Cenário α** — Acesso directo: `layouter.introspector.resolved_label_for(target)`.

**Cenário β** — Acesso via referência: `layouter.introspector.as_ref().resolved_label_for(target)`.

**Cenário γ** — Sem acesso directo (introspector é
private): cláusula gate trivial — adicionar getter.

Sugestão: **Cenário α** esperado per P184D pattern (que
acede `layouter.introspector` directamente).

Output: cenário identificado.

### Sub-passo 194A.F — Decisão cláusula 5 (documentação inline)

Per Q4: estado temporário, não permanente. Documentação
mais leve que P188B.

**Opção A** — Sem comentário inline (decisão é simples e
óbvia para quem conhece sequência §9 P189).

**Opção B** — Comentário curto:
```rust
// Introspector path activa após P195 (walk Labelled
// migrated). Durante janela compat, fallback legacy é
// caminho funcional.
```

**Opção C** — Comentário extenso com cross-references.

Sugestão: **Opção B** — comentário curto com cross-reference
ao P194. Não tão leve como sem comentário (que poderia
deixar leitores confusos sobre porquê o fallback) nem tão
extenso como P188B (que era permanente).

Output: decisão fixada.

### Sub-passo 194A.G — Decisão cláusula 6 (critério de fecho)

P194 fecha quando:
- Consumer C4 migrado (`references.rs:53`).
- Tests E2E confirmam paridade observable (preservada
  trivialmente — sub-store vazio).
- DEBT M5-residual actualizado: 3 → 2 pré-requisitos
  restantes.
- Excepções E2-E6 ainda activas (P195+ destranca).

Output: critério literal verificável.

### Sub-passo 194A.H — Validação do plano de sub-passos

Tabela esperada:

| Sub-passo | Escopo | Magnitude | Depende |
|-----------|--------|-----------|---------|
| `.B` | Migrar consumer C4 + tests E2E + decisão copy-sites + L0 + actualização DEBT + relatório consolidado P194 | S | — |

Sub-passo único agregado (similar a P187B/P188B).

Output: tabela final.

### Sub-passo 194A.I — ADR

Avaliar:

- Substitution-with-fallback é padrão estabelecido —
  não ADR.
- `resolved_label_for` já existe (P193B) — não ADR.
- Copy-sites: decisão local; não decisão arquitectural —
  não ADR.
- Sem semântica nova.

Conclusão esperada: **não cria ADR**.

### Sub-passo 194A.J — DEBT

P194 não abre DEBT novo. Actualiza nota DEBT M5-residual:
- Antes P194: 3 pré-requisitos pendentes.
- Após P194: 2 pré-requisitos restantes
  (`headings_for_toc`, `SetEquationNumbering`).
- C4 migration ✅ feita; mas excepções E2-E6 continuam
  até P195+.

**Cenário B continua** (sem DEBT formal aberto).

Output: cenário identificado.

### Sub-passo 194A.K — Outputs

Produzir 3 ficheiros (padrão P181A–P193A):

1. **`00_nucleo/diagnosticos/diagnostico-c4-resolved-label-passo-194a.md`**
   — diagnóstico com 8 secções:
   - §1 Validação estado actual.
   - §2 Decisões cláusula 1–6 (formato O1–O5).
   - §3 Plano de sub-passos sem condicionais.
   - §4 Magnitude consolidada.
   - §5 ADR avaliação.
   - §6 DEBT avaliação.
   - §7 Estado temporário (não permanente) face a P188B.
   - §8 Próximo sub-passo (P194B com escopo concreto).

2. **`00_nucleo/materialization/typst-passo-194a-relatorio.md`**
   — relatório com 14 secções (padrão P181A/etc.).

3. **Sem ADR e sem DEBT formal esperados**.

---

## Restrições

- **Zero código tocado** em qualquer ficheiro fora de
  `00_nucleo/`.
- **Zero testes** modificados.
- **Não criar reservas** de identificadores.
- **Não migrar consumer C4** — P194B.
- **Não modificar trait `Introspector`** — P185B fechou.
- **Não modificar `TagIntrospector`** — P193B fechou.
- **Não tocar copy-sites Layouter** — P194B (decisão de
  cláusula 3).
- **Não modificar walk arms** — P195+.
- **Não modificar `from_tags`** — P195+.
- **Sem inflação retórica**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como bandeira
  retórica.
- **Honestidade obrigatória sobre estado temporário**:
  P194 introduz Introspector path mas que está
  vazio em produção até P195+. Diferente de P188B
  (permanente). Esta diferença deve ser registada
  honestamente.
- **Sem cláusulas condicionais nos sub-passos `.B`+ do
  plano**.

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-c4-resolved-label-passo-194a.md`
  com 8 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-194a-relatorio.md`
  com 14 secções produzido.
- 6 cláusulas fechadas com decisão literal.
- Plano de 1 sub-passo (B agregado) sem condicionais.
- Magnitude S agregada confirmada.
- Critério de fecho C4 fixado.
- ADR avaliada (esperado: não criada).
- DEBT M5-residual cenário identificado (B continua).
- Estado temporário (não permanente) honestamente
  documentado.
- Decisão sobre copy-sites Layouter fixada.
- Nenhum ficheiro de cristalino tocado.
- Tests workspace 1.821 inalterados.
- `crystalline-lint .` zero violations.

P194A é instrumento. Migração concreta de C4 começa em
P194B.
