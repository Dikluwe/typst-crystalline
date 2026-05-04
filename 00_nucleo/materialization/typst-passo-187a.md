# Passo 187A — Instrução Claude Code

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- Tests workspace 1.801 verdes; zero violations.
- M9 ✅ 11/11 (slot 11 livre).
- M5/M4 progresso: 6/12 read-sites migrados.
- DEBT M4-residual cobre apenas C1 + C2.
- ADR-0068 ACEITE. Layouter location-aware via
  `current_location: Option<Location>` (P185C).
- Trait `Introspector` 18 métodos incluindo
  `is_numbering_active_at` + `flat_counter_at` (P185B) +
  `formatted_counter_at` (P177).
- Equation locatable estruturalmente — P186 série fechada
  com gate dormente em produção (separado de P187).

P187 fecha **C1 heading prefix** (`mod.rs:310`). Esta é a
primeira migração que beneficia plenamente de location-aware
Layouter — em contraste com C2 (P188) onde Introspector
fica dormente, em C1 o Introspector path é o **caminho
funcional** porque `Content::SetHeadingNumbering` existe e
popula state em produção (P182C).

C1 tem precedente histórico:
- **P183B falhou** com gate substancial — `formatted_counter`
  snapshot-final preempta fallback legacy mutável durante
  walk (P183B relatório). C1 ficou no DEBT M4-residual.
- **P185 desbloqueou eixo 1** ao introduzir
  `formatted_counter_at(key, location)` location-aware no
  trait + `current_location` no Layouter.
- **P187 finaliza** a migração com substitution-with-fallback
  que P183B tentou, agora com infraestrutura correcta.

Material de partida verificado:

- `00_nucleo/materialization/typst-passo-183b-relatorio.md`
  — relatório de falha P183B; documenta causa raíz e
  blueprint para o desbloqueio.
- `00_nucleo/materialization/typst-passo-185-relatorio-consolidado.md`
  §8 — P187 listado como próximo após P185, blueprint
  literal em test P185D `.E`
  (`pipeline_e2e_is_numbering_active_at_via_current_location`).
- ADR-0068 ACEITE — sincronização Locator
  empiricamente validada em P185D.
- `01_core/src/rules/layout/mod.rs:310` — site C1 actual
  com leitura legacy `self.counter.format_hierarchical(
  "heading")`.

P187A é o passo de diagnóstico que precede a
implementação. Magnitude S esperada — replicação de
padrão P184D com primitiva diferente
(`formatted_counter_at` em vez de `figure_number_at_index`).

---

## Postura do auditor / executor

P187A é passo **L0-puro / diagnóstico-primeiro**, no mesmo
registo de P181A/P182A/P183A/P184A/P185A/P186A.

- **Zero código tocado** em camadas cristalinas.
- **Zero testes** novos ou modificados.
- **Pode criar** ADR `PROPOSTO` se decisão arquitectural
  exigir — improvável (replicação de padrão estabelecido).
- **Pode abrir DEBT** se trabalho identificado for adiado.
- **Não modifica** trait `Introspector`, Layouter consumer,
  `Locator` — P187B+.

**Magnitude diagnóstico**: S. Decisões esperadas são
locais (forma exacta de migração, tratamento do `None` do
Introspector, relação com P183B aprendizado). Sem ADR
substancial.

---

## Escopo

**Primário**: desenhar migração de C1 heading prefix
(`mod.rs:310`) para `formatted_counter_at(key,
current_location)` com fallback legacy.

**Confirmação**: validar que infraestrutura P185 está
acessível no site exacto + que blueprint do P185D `.E` é
aplicável.

**Decisões a tomar** — 6 cláusulas:

1. **Forma exacta da expressão de migração** — variante
   da substitution-with-fallback. Há sub-opções sobre
   tratamento de `current_location: Option<Location>`
   (`unwrap` vs propagar `None` vs fallback).

2. **Tratamento do `None` do Introspector** — se
   `formatted_counter_at` retorna `None` (chave ausente
   ou Location anterior à primeira escrita), cair em
   fallback legacy (Opção A) ou retornar valor default
   (Opção B).

3. **Tratamento do `None` do `current_location`** —
   `Layouter::new` inicializa `current_location: None`
   (P185C decisão). Se C1 é processado antes de qualquer
   heading set rule, `current_location` é `None`.
   Tratamento: cair em fallback legacy directamente
   (Opção A) ou retornar default (Opção B).

4. **Compatibilidade com aprendizado P183B** — P183B
   demonstrou que substitution-with-fallback simples
   `or_else` falha porque `Some` do Introspector pré-empta
   o fallback. Com `formatted_counter_at` location-aware,
   este problema **não se aplica** porque a primitiva
   retorna o snapshot na Location consultada (não final).
   Confirmar empiricamente em `.A`.

5. **Forma de migração** — substitution-with-fallback
   simétrica a P184D (Figure) e P181G (cite-arm). Replica
   padrão estabelecido.

6. **Critério de fecho de P187** — Opção 3 (infra pronta +
   consumer migrado + tests E2E paridade). Após P187,
   DEBT M4-residual reduz de C1+C2 para C2 apenas (cenário
   B até P188 fechar).

**Fora de escopo**:

- Migração consumer C2 (P188 — equation counter).
- Walk puro M5 (P189).
- Eliminação `CounterStateLegacy` M6 (P190).
- `Content::SetEquationNumbering` materialização (passo
  fora série).

---

## Critérios objectivos

Para cada decisão das 6 cláusulas, registar:

### O1 — Inputs verificáveis

`grep -rn "format_hierarchical\|formatted_counter_at\|current_location"
01_core/src/`. Para cláusula 1, confirmar contexto exacto
da leitura em `mod.rs:310` (arm `Content::Heading`?
`layout_heading`? que valor é retornado e como é usado?).
Para cláusula 2, confirmar comportamento de
`formatted_counter_at` em casos edge.

### O2 — Alternativas

Mínimo 2 quando há margem real. Para cláusulas 2 e 3
(tratamento de `None`), 2 alternativas claras.

### O3 — Critério de escolha

Padrão estabelecido P184D (Figure) replicado literalmente
para C1. Sem decisão arquitectural nova esperada.

### O4 — Magnitude

Trivial vs substancial. Decisões 1-3 são triviais;
cláusulas 4-6 são confirmações.

### O5 — Reversibilidade

Substitution-with-fallback é reversível por construção
(remover Introspector path; legacy continua funcional).

---

## Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

Migração replica P184D Figure literalmente? P181G
cite-arm? P182D heading-arm `is_numbering_active`? Sim em
ambos os casos — substitution-with-fallback simples.

### Q2 — Honestidade de magnitude

P187A diagnóstico é S. P187B+ implementação:
- P187B: migrar consumer + tests E2E + relatório.
- Provável série de **2-3 sub-passos** apenas (B + C/D
  combinado), não 5-6 como P186.

Total agregado: ~10 LOC produção + ~80 LOC tests ≈ S.

### Q3 — Cobertura sem regressão

P185D test `.E`
(`pipeline_e2e_is_numbering_active_at_via_current_location`)
é blueprint. Tests existentes que cobrem heading prefix:
`p182d_heading_numbering`, `p182e_e2e_heading_numbering`.
Migração não deve regredir nenhum.

### Q4 — Inversão observable face a P183B

P183B falhou. P187 deve **passar** com mesma estrutura
substitution-with-fallback porque primitiva mudou:
- Antes (P183B): `formatted_counter` snapshot-final.
  `Some(...)` pré-empta fallback → output errado em
  re-update.
- Agora (P187): `formatted_counter_at(key, current_location)`
  location-aware. `Some(...)` é o valor correcto na
  Location consultada → fallback é redundante mas
  inofensivo.

P187 é a primeira migração onde Introspector se torna
**caminho funcional real** para C1 — não dormente como C2
em P186.

### Q5 — Granularidade dos sub-passos P187B+

3 sub-passos típicos para passo S:
- `.B` migração + L0.
- `.C` tests E2E + relatório.

Pode ser comprimido em 2 (B + C combinado) se a migração
for trivial e os tests directos.

---

## Sub-passos de P187A

### Sub-passo 187A.A — Validação do estado actual

Auditor confirma empiricamente:

1. Confirmar consumer C1 actual:
   - `01_core/src/rules/layout/mod.rs:310` (per P183A
     §2 e P183B relatório).
   - Localizar leitura: padrão esperado
     `self.counter.format_hierarchical("heading")` ou
     similar.
   - Identificar contexto exacto (arm `Content::Heading`,
     função/método específico, escopo das variáveis
     locais).

2. Confirmar acesso a `self.introspector`:
   - P181G/P182D estabeleceram acesso. P185C estabeleceu
     `current_location`.
   - Confirmar que Layouter no site de C1 tem ambos
     acessíveis.

3. Confirmar API `formatted_counter_at`:
   - `01_core/src/entities/introspector.rs` — assinatura
     `formatted_counter_at(&self, key: &str, location:
     Location) -> Option<String>` (per P177).
   - Confirmar empiricamente.

4. Confirmar `current_location` no site:
   - `self.current_location: Option<Location>` (per
     P185C).
   - Quando é populado relativamente ao site C1?
   - Heading é locatable (per `is_locatable`). Layouter
     processa Heading via `layout_content` que faz
     gating: `current_location` actualiza para Heading
     **antes** do site C1 (que está dentro de
     `layout_content` arm Heading) ser executado.
   - Confirmar empiricamente em `.A` — se ordem for
     diferente, cláusula gate trivial em P187B.

5. Confirmar `format_hierarchical` legacy:
   - `01_core/src/entities/counter_state_legacy.rs` (ou
     similar).
   - Assinatura: `format_hierarchical(&self, key: &str)
     -> String` ou `Option<String>` (verificar
     empiricamente — P183B relatório registou).
   - Comportamento durante walk: mutável; reflecte
     valor "na altura" da chamada.

6. Confirmar tests P185D `.E` blueprint:
   - `pipeline_e2e_is_numbering_active_at_via_current_location`
     em `tests.rs` `p185d_locator_sync` submódulo.
   - Padrão da chamada: `intr.is_numbering_active_at(
     "numbering_active:heading",
     layouter.current_location.unwrap())`.
   - Adaptar para `formatted_counter_at` de forma
     simétrica.

7. Confirmar P183B aprendizado relevante:
   - `00_nucleo/materialization/typst-passo-183b-relatorio.md`
     §"Diagnóstico do gate" — `Some(...)` pré-empta
     fallback. Verificar que com `formatted_counter_at`
     location-aware o problema não se aplica.

Output: tabela com item + estado confirmado / linha
actual / observação.

**Critério de saída**:
- Site C1 localizado.
- `current_location` confirmado disponível antes do
  site.
- `formatted_counter_at` API confirmada.
- Aprendizado P183B confirmado não-aplicável após P185.

### Sub-passo 187A.B — Decisão cláusula 1 (forma de expressão)

Avaliar a forma exacta da migração.

**Opção A** — Inline directo:
```
self.introspector
    .formatted_counter_at("heading", self.current_location.unwrap())
    .or_else(|| self.counter.format_hierarchical("heading"))
```

**Opção B** — Variável intermédia:
```
let prefix = self.current_location
    .and_then(|loc| self.introspector.formatted_counter_at("heading", loc))
    .or_else(|| Some(self.counter.format_hierarchical("heading")));
```

**Opção C** — Match explícito:
```
match self.current_location {
    Some(loc) => self.introspector.formatted_counter_at("heading", loc)
        .unwrap_or_else(|| self.counter.format_hierarchical("heading")),
    None => self.counter.format_hierarchical("heading"),
}
```

Critério: P184D usou inline directo com `or_else`. Replica
padrão. Mas `current_location` é `Option<Location>` (P185C)
— pode requerer Opção B (`and_then`) ou Opção C (match).

Sugestão: **Opção B** se `current_location` é `Option`
(esperado per P185C); **Opção A** se site garante
`current_location` foi populado (vide cláusula 3).

Output: decisão fixada com base em `.A.4`.

### Sub-passo 187A.C — Decisão cláusula 2 (tratamento `None` do Introspector)

`formatted_counter_at(key, location)` pode retornar
`None` se:
- Chave ausente em `CounterRegistry`.
- Location anterior à primeira escrita na chave.

**Opção A** — Cair em fallback legacy:
```
.or_else(|| self.counter.format_hierarchical("heading"))
```

**Opção B** — Retornar string vazia:
```
.unwrap_or_default()
```

**Opção C** — Mistura: tentar fallback; se também `None`,
default.

Critério: P184D usou Opção A (`or_else` com legacy). Replica
padrão. Em produção real, com `Content::SetHeadingNumbering`
populando state, Introspector path retorna `Some(...)`
para headings esperados — fallback é defensivo, raramente
disparado.

Sugestão: **Opção A**.

Output: decisão fixada.

### Sub-passo 187A.D — Decisão cláusula 3 (tratamento `None` do `current_location`)

`current_location: Option<Location>` é `None` antes do
primeiro locatable processado. Per P185D `.D` test, isso
acontece para Text/Space antes de qualquer Heading.

Mas C1 está **dentro** do arm `Content::Heading` em
`layout_content`. Heading é locatable; gating
`advance_locator_if_locatable` (P185C) **precede** o
match arm. Logo no site de C1, `current_location` deve
estar `Some(loc_da_heading_actual)`.

Confirmar empiricamente em `.A.4`.

**Opção A** — Assumir `Some` e usar `unwrap()`:
```
self.introspector.formatted_counter_at("heading",
    self.current_location.unwrap())
```

Risco: panic se invariante violada.

**Opção B** — Defensivo com `and_then`:
```
self.current_location.and_then(|loc|
    self.introspector.formatted_counter_at("heading", loc))
```

**Opção C** — Match com fallback explícito para `None`:
```
match self.current_location {
    Some(loc) => ...,
    None => self.counter.format_hierarchical("heading"),
}
```

Critério: Opção B é mais defensiva e não panica. Opção A
é mais limpa mas frágil. Opção C é explícita mas verbose.

Sugestão: **Opção B** combinada com cláusula 2 dá:
```
self.current_location
    .and_then(|loc| self.introspector
        .formatted_counter_at("heading", loc))
    .or_else(|| self.counter.format_hierarchical("heading"))
```

Output: decisão fixada.

### Sub-passo 187A.E — Decisão cláusula 4 (validar P183B aprendizado)

Confirmar empiricamente que aprendizado P183B
(`Some(...)` pré-empta fallback → output errado em
re-update) **não se aplica** após P185.

Razão: `formatted_counter_at(key, location)` é
location-aware. Para sequência `H1, H2, H1` no documento:
- Em H1 (loc=10): `formatted_counter_at("heading", 10)`
  retorna snapshot na location 10 = `Some("1")`.
- Em H2 (loc=20): retorna `Some("1.1")`.
- Em H1 segunda (loc=30): retorna `Some("2")`.

Diferente de P183B onde `formatted_counter("heading")`
retornava sempre o snapshot **final** (`"2"`) para todos.

Confirmar em `.A.7` que test P185D `.E` empiricamente
demonstra esta correctness.

Output: confirmação registada.

### Sub-passo 187A.F — Decisão cláusula 5 (forma de migração)

Substitution-with-fallback per P184D padrão. Replica
literal.

Output: confirmação.

### Sub-passo 187A.G — Decisão cláusula 6 (critério de fecho)

P187 fecha quando:
- Consumer C1 migrado.
- Tests E2E confirmam paridade observable (output
  Layouter idêntico legacy vs Introspector path).
- DEBT M4-residual actualizado: cobre apenas C2 (era
  C1+C2).

Output: critério literal.

### Sub-passo 187A.H — Validação do plano de sub-passos

Tabela esperada:

| Sub-passo | Escopo | Magnitude | Depende |
|-----------|--------|-----------|---------|
| `.B` | Migrar consumer C1 com substitution-with-fallback + L0 + tests E2E + actualização DEBT M4-residual | S | — |

Sub-passo único agregado P187B (não série B-F como em
P186). Razão: migração + tests + actualização DEBT é
trabalho coeso e pequeno.

**Alternativa**: dividir em 2 sub-passos (B = migração;
C = tests + DEBT). Decisão depende de magnitude empírica
de cada peça em `.A`.

Sugestão: **passo único P187B** se `.A` confirmar que
trabalho cabe em ~80-150 LOC total. Caso contrário,
dividir.

Output: tabela final.

### Sub-passo 187A.I — ADR

Avaliar:

- Substitution-with-fallback é padrão estabelecido P184D
  — não ADR.
- `formatted_counter_at` já existe (P177) — não ADR.
- `current_location` já existe (P185C) — não ADR.
- Não há decisão arquitectural nova.

Conclusão esperada: **não cria ADR**.

### Sub-passo 187A.J — DEBT

P187 não abre DEBT novo. Mas **actualiza DEBT M4-residual**:
- Antes P187: cobre C1 + C2.
- Após P187: cobre apenas C2.
- Sub-passo `.B` deve incluir actualização do DEBT como
  parte do trabalho.

Cenário a confirmar em `.A`:
- **Cenário A**: P183F já correu (DEBT formal aberto).
  P187B edita o DEBT removendo C1.
- **Cenário B**: P183F ainda não correu (nota preventiva
  registada em P184F). P187 actualiza nota indicando que
  quando P183F correr, abre DEBT cobrindo apenas C2.

Output: cenário identificado.

### Sub-passo 187A.K — Outputs

Produzir 3 ficheiros (padrão P181A–P186A):

1. **`00_nucleo/diagnosticos/diagnostico-c1-heading-prefix-passo-187a.md`**
   — diagnóstico com 8 secções:
   - §1 Validação estado actual.
   - §2 Decisões cláusula 1–6 (formato O1–O5).
   - §3 Plano de sub-passos sem condicionais.
   - §4 Magnitude consolidada.
   - §5 ADR avaliação.
   - §6 DEBT avaliação (actualização M4-residual).
   - §7 Relação com P183B falha (validar não-aplicação
     após P185).
   - §8 Próximo sub-passo (P187B com escopo concreto).

2. **`00_nucleo/materialization/typst-passo-187a-relatorio.md`**
   — relatório com 14 secções (padrão P181A/etc.).

3. **Sem ADR e sem DEBT novo esperados**.

---

## Restrições

- **Zero código tocado** em qualquer ficheiro fora de
  `00_nucleo/`.
- **Zero testes** modificados.
- **Não criar reservas** de identificadores.
- **Não migrar consumer C1** — P187B.
- **Não modificar trait `Introspector`** — P185B fechou.
- **Não modificar Layouter struct** — P185C fechou.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como bandeira
  retórica.
- **Honestidade obrigatória**: registar P183B aprendizado
  literalmente, sem revisionismo. P183B falhou; P187
  passa porque infraestrutura mudou (P185), não porque
  P183B estava errado em todos os aspectos.
- **Sem cláusulas condicionais nos sub-passos `.B`+ do
  plano**.

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-c1-heading-prefix-passo-187a.md`
  com 8 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-187a-relatorio.md`
  com 14 secções produzido.
- 6 cláusulas fechadas com decisão literal.
- Plano de 1-2 sub-passos sem condicionais (B único, ou
  B + C).
- Magnitude S agregada confirmada.
- Critério de fecho C1 fixado.
- ADR avaliada (esperado: não criada).
- DEBT M4-residual cenário identificado (A ou B per
  P184F).
- Aprendizado P183B confirmado não-aplicável após P185.
- Nenhum ficheiro de cristalino tocado.
- Tests workspace 1.801 inalterados.
- `crystalline-lint .` zero violations.

P187A é instrumento. Migração concreta de C1 começa em
P187B.
