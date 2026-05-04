# Passo 185A — Instrução Claude Code

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- Tests workspace 1.769 verdes; zero violations.
- M9 ✅ 11/11 (P182 fechou lacuna #4; slot 11 livre per
  P184 consolidado §5).
- M4-residual em curso. P183: C4 pendente (P183E não
  corrido); C1, C2, C3 bloqueados em P183B/C/D. P184:
  C3 desbloqueado e validado.
- DEBT M4-residual cobre apenas **C1 + C2** (cenário B —
  P183F ainda não correu; nota preventiva registada em
  P184F).
- M5/M4 progresso: 6/12 read-sites migrados.

P185 ataca a pendência paralela P182E §5.2: location-aware
Layouter. Esta pendência foi adiada várias vezes e é
pré-condição para desbloquear C1 (heading prefix) e C2
(equation counter). Sem ela, fallback `||` legacy mutável
durante walk continua a ser o caminho funcional para
casos de re-update.

Material de partida verificado:

- `00_nucleo/materialization/typst-passo-182e-relatorio.md` §5.2
  — descobriu que `Introspector::is_numbering_active` usa
  `state_final_value` (snapshot final); insuficiente para
  re-update. Fallback legacy mutável é o caminho funcional.
- `00_nucleo/materialization/typst-passo-184-relatorio-consolidado.md`
  §4.2 — inversão observable: C3 (P184) fechado com
  Introspector como caminho funcional. C1 e C2 esperam
  location-aware para mesma inversão.
- Trait `Introspector` já tem **`formatted_counter_at(key,
  location)`** (P177). Trabalho location-aware em
  Introspector parcialmente feito. Falta: (1) Layouter
  conhecer `Location` actual em cada consulta; (2) outros
  métodos location-aware se necessário (ex.: `flat_counter_at`,
  `is_numbering_active_at`).

P185A é o passo de diagnóstico que precede a
implementação. Decisão arquitectural substancial — sem
fixação em P185A, P185B+ herda o problema do plano
monolítico.

---

## Postura do auditor / executor

P185A é passo **L0-puro / diagnóstico-primeiro**, no mesmo
registo de P181A/P182A/P183A/P184A.

- **Zero código tocado** em camadas cristalinas.
- **Zero testes** novos ou modificados.
- **Pode criar** ADR `PROPOSTO` se decisão arquitectural
  o exigir — **provável** dado escopo (mecanismo de
  propagação Location é decisão arquitectural).
- **Pode abrir DEBT** se trabalho identificado for adiado.
- **Não modifica** Layouter, walk, trait `Introspector`,
  consumers — P185B+.

**Decisão arquitectural substancial esperada**: P185A é
diferente dos diagnósticos anteriores — escolhe entre
3 mecanismos com perfis de manutenção diferentes. Não há
"caminho óbvio" como em P181A (replica P165) ou P182A
(replica P171). Aqui há decisão genuína.

---

## Escopo

**Primário**: desenhar mecanismo de propagação `Location`
ao Layouter no ponto de consulta de contadores.

**Confirmação**: validar inventário factual — onde estão
os consumers C1+C2; que `Location` é necessária em cada
ponto; que infraestrutura location-aware já existe no
Introspector.

**Decisões a tomar** — 6 cláusulas:

1. **Mecanismo de propagação** — escolha entre 3
   alternativas:
   - **M1** Walk sincronizado: walk de layout produz
     `Location`s sincronizadas com walk de introspect.
   - **M2** Parâmetro propagado: `Location` actual passa
     via parâmetro nos métodos de layout.
   - **M3** Cursor próprio: Layouter mantém cursor
     interno que avança com o seu próprio walk.

2. **Trait methods location-aware necessários** — quais
   já existem (`formatted_counter_at` per P177) e quais
   precisam ser adicionados (`flat_counter_at`,
   `is_numbering_active_at`?). Confirmar empiricamente.

3. **Compatibilidade com `Locator` existente** —
   cristalino tem mecanismo `Locator` para gerar
   `Location`s durante walk. Layouter já usa? Ou
   precisa de Locator próprio?

4. **Forma de migração de C1 + C2** —
   substitution-with-fallback (replica P184D) vs
   alternativa quando location-aware está disponível.

5. **Compatibilidade com walk puro (P189 / M5)** —
   mecanismo escolhido em cláusula 1 deve **não** exigir
   walk impuro. Confirmar empiricamente cada alternativa.

6. **Critério de fecho de P185** — diagnóstico fixa a
   decisão arquitectural; implementação concreta
   (P185B+) é trabalho subsequente.

**Fora de escopo**:

- Implementação concreta do mecanismo (P185B+).
- Promoção `Content::Equation` a locatable (P186).
- Migração C1 (P187).
- Migração C2 (P188).
- Walk puro M5 (P189).
- Eliminação `CounterStateLegacy` M6 (P190).

---

## Critérios objectivos

Para cada decisão das 6 cláusulas, registar:

### O1 — Inputs verificáveis

`grep -rn "Location\|Locator\|formatted_counter_at"
01_core/src/`. Para cláusula 1, inventariar como vanilla
typst resolve a mesma necessidade
(`lab/typst-original/`). Para cláusula 3, confirmar API
do `Locator` actual.

### O2 — Alternativas

Cláusula 1 tem 3 alternativas obrigatórias (M1/M2/M3).
Outras cláusulas têm mínimo 2.

### O3 — Critério de escolha

ADR existente (ADR-0036 atomização, ADR-0037 coesão por
domínio), invariante de walk puro (P163), simetria com
vanilla, custo de implementação, perfil de manutenção
(coerência com ADR-0067 attribute-grammar).

### O4 — Magnitude da decisão

Trivial vs substancial. Cláusula 1 é certamente
substancial. Outras podem ser triviais.

### O5 — Reversibilidade

Reversível ou fixa direcção cara mudar.

---

## Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

Mecanismo escolhido alinha com sub-store pattern
(P165/P169/P171/P177/P181) ou inventa estrutura nova?

### Q2 — Honestidade de magnitude

P185A diagnóstico é S. Mas implementação subsequente
(P185B+) é provavelmente **M-L**. Registar honestamente
a magnitude esperada da implementação.

### Q3 — Compatibilidade com walk puro

P163 invariante walk puro deve ser preservada. Mecanismo
escolhido em cláusula 1 não pode exigir mutação de state
durante walk de layout. Confirmar empiricamente.

### Q4 — Coerência com ADR-0067 (attribute-grammar)

ADR-0067 PROPOSTO sugere attribute-grammar como direcção
para scoping de propriedades. Location-aware Layouter é
trabalho relacionado mas distinto. Mecanismo escolhido
deve **não** bloquear adopção futura de attribute-grammar
nem replicar trabalho que ela faria.

### Q5 — Granularidade dos sub-passos P185B+

Implementação concreta cabe em quantos sub-passos? Cada
um é S/M/L?

---

## Sub-passos de P185A

### Sub-passo 185A.A — Validação do estado actual

Auditor confirma empiricamente:

1. **Trait `Introspector`** — métodos location-aware
   existentes:
   - `formatted_counter_at(&self, key: &str, location:
     Location) -> Option<String>` (P177).
   - Outros métodos `*_at(...)` (verificar empiricamente).
   - Métodos não location-aware que C1+C2 usariam ou
     já usam fallback (`is_numbering_active`,
     `formatted_counter`, `flat_counter` se existir).

2. **Layouter** — uso actual de Location:
   - `grep -rn "Location" 01_core/src/rules/layout/`.
   - Layouter conhece `Location` em algum ponto? Ou
     trabalha apenas com índices/posições estruturais?
   - Se Layouter já usa `Location` parcialmente: onde
     e como?

3. **`Locator`** — API e uso:
   - `01_core/src/entities/locator.rs` (ou similar).
   - Como `Locator` gera `Location`s durante walk de
     introspect? Pode ser reutilizado em walk de layout?
   - `Locator` tem state mutável ou é puro?

4. **Vanilla typst** — solução equivalente:
   - `grep -rn "Location\|Locator" lab/typst-original/crates/`.
   - Como vanilla resolve consultas de contador
     location-aware durante layout?
   - Se vanilla tem `Locator` partilhado entre walks:
     templates para cristalino seguir.

5. **Consumers C1 + C2** — Location necessária:
   - C1 (`mod.rs:310`, heading prefix) — em que ponto
     Layouter pode ter `Location` da heading actual?
   - C2 (`equation.rs:97`, equation counter) — idem
     para equation actual.

6. **Walk de layout vs walk de introspect**:
   - São o mesmo walk em sequência? Walks separados?
   - Se separados: como sincronizar `Location`s entre
     eles?

Output: tabela com item + estado confirmado / linha
actual / observação.

**Critério de saída**:
- Inventário completo. Decisão de cláusula 1 informada
  por dados empíricos.

### Sub-passo 185A.B — Decisão cláusula 1 (mecanismo de propagação)

Esta é a decisão central de P185A.

#### M1 — Walk sincronizado

Walk de layout produz `Location`s sincronizadas com walk
de introspect. Layouter consulta `Introspector` com a
`Location` corrente.

Vantagens:
- Replica modelo vanilla mais directamente.
- Locations exactamente alinhadas entre walks.

Desvantagens:
- Requer que walk de layout use o mesmo `Locator` (ou
  Locator com sequência idêntica) que walk de introspect.
- Risco de divergência se walks divergirem (ex.: walk de
  layout salta nodes que walk de introspect visitou).

#### M2 — Parâmetro propagado

`Location` actual passa via parâmetro nos métodos de
layout: `layout_content(&self, content, location)`,
`layout_heading(&self, heading, location)`, etc.

Vantagens:
- Explícito e auditável.
- Não requer cursor/state interno no Layouter.
- Coerente com walk puro (Location é argumento, não
  state).

Desvantagens:
- Cascata de mudanças de assinatura em todos os métodos
  de layout.
- Locator continua a ser quem gera as Locations —
  Layouter precisa de receber Locator ou pre-computed
  Locations.

#### M3 — Cursor próprio

Layouter mantém um cursor interno (`current_location:
Location`) que avança com o seu próprio walk. Sem
parâmetro nos métodos.

Vantagens:
- Mudanças de assinatura mínimas.
- Cursor é local; não cascata para callers.

Desvantagens:
- State mutável no Layouter (não é walk puro do
  Layouter).
- Risco de cursor desincronizar com walk real (bug
  silencioso).

Critério de escolha:

- Coerência com P163 walk puro: **M2 ou M1** preferíveis.
- Coerência com ADR-0036 atomização: **M2** mais
  explícito.
- Coerência com ADR-0067 attribute-grammar (futuro):
  **M2** alinha (parâmetros propagados são forma natural
  de attribute-grammar).
- Custo de implementação: **M3** mais barato a curto
  prazo; **M2** tem cascata mas é mecânico.
- Reversibilidade: **M2** mais fácil de reverter (remover
  parâmetro). **M3** deixa cursor a remover.

Sugestão: **M2** se P185A confirmar que cascata não é
proibitiva. **M1** se Locator partilhado for trivial.
**M3** apenas se M2 e M1 forem impraticáveis.

Output: decisão fixada com justificação literal.

### Sub-passo 185A.C — Decisão cláusula 2 (trait methods)

Inventariar quais métodos location-aware existem e quais
faltam:

- **Existe** (per P177): `formatted_counter_at(key,
  location)`.
- **Falta provavelmente**: `flat_counter_at(key, location)`
  para C2 (equation counter retorna `[N]`, não string).
- **Falta provavelmente**: `is_numbering_active_at(key,
  location)` para correctness em re-update (P182E §5.2
  identificou).

Para cada método em falta:
- Replica padrão `formatted_counter_at` (P177): delega a
  `CounterRegistry::value_at(key, location)`.
- Magnitude trivial (declaração + impl).

Output: lista de métodos em falta + decisão sobre quais
adicionar em P185.

### Sub-passo 185A.D — Decisão cláusula 3 (Locator)

Layouter pode reutilizar `Locator` actual? Ou precisa
de Locator próprio?

**Opção A** — Locator partilhado entre walks. Walk de
introspect e walk de layout usam **o mesmo** `Locator`
em sequência ou em paralelo.

**Opção B** — Locator separado por walk. Layouter tem o
seu Locator que produz `Location`s sincronizadas (mesma
sequência) por construção.

**Opção C** — Pre-compute Locations no walk de
introspect; Layouter consome a sequência.

Critério: depende de cláusula 1.
- Se M1 (walk sincronizado): Opção A ou B.
- Se M2 (parâmetro propagado): Opção C provavelmente.
- Se M3 (cursor próprio): Opção B com cursor que
  consulta o seu Locator.

Output: decisão integrada com cláusula 1.

### Sub-passo 185A.E — Decisão cláusula 4 (forma de migração)

Substitution-with-fallback (replica P184D) ou alternativa
quando location-aware está disponível?

Quando location-aware está activo, fallback `||` legacy
deixa de ser necessário em casos típicos. Mas durante
janela compat M6, fallback continua a ser defensivo.

**Opção A** — Substitution-with-fallback (replica P184D).
Trivial.

**Opção B** — Substituição directa (sem fallback).
Mais limpo mas menos defensivo.

**Opção C** — Substitution-with-fallback **mas** fallback
é heurística (`unwrap_or(idx + 1)` ou similar) em vez
de leitura legacy. Replica observação P184D §5 onde
fallback acabou em heurística por dead code legacy.

Sugestão: Opção A para coerência. M6 elimina fallback
junto com `CounterStateLegacy`.

Output: decisão fixada.

### Sub-passo 185A.F — Decisão cláusula 5 (compat walk puro)

Mecanismo escolhido em cláusula 1 deve **não** exigir
walk impuro. Confirmar:

- M1 (walk sincronizado) — walk de layout itera sem
  mutar state; Locator partilhado é leitura. Compatível.
- M2 (parâmetro propagado) — Location é argumento, não
  state. Compatível.
- M3 (cursor próprio) — cursor é state mutável **dentro**
  do Layouter, não no walk de introspect. Tecnicamente
  compatível com P163 (que cobre walk de introspect),
  mas viola espírito de "componentes puros".

Output: confirmação ou ressalva.

### Sub-passo 185A.G — Decisão cláusula 6 (critério de fecho de P185)

P185 fecha quando:

- **Opção 1** — diagnóstico (P185A) + implementação
  trait methods location-aware (P185B) + integração
  Layouter (P185C+).
- **Opção 2** — Opção 1 + tests E2E confirmando
  consumers C1+C2 ainda funcionam via fallback.

Critério: Opção 2 dá rigor mas C1+C2 só são migrados em
P187/P188 (passos dedicados após P185). Opção 1 é
suficiente para P185 fechar.

P185 só fecha **infra** location-aware — migração dos
consumers (C1, C2) fica para P187, P188 conforme plano
agregado P184F §13.

Output: critério literal verificável.

### Sub-passo 185A.H — Validação do plano de sub-passos P185B+

Tabela esperada (depende de cláusula 1):

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Adicionar trait methods location-aware em falta | S |
| `.C` | Implementar mecanismo cláusula 1 (M1/M2/M3) | M-L |
| `.D` | Tests E2E mecanismo location-aware funciona | S |
| `.E` | Relatório consolidado P185 | S |

Output: tabela final com magnitudes específicas.

### Sub-passo 185A.I — ADR

Avaliar:

- Mecanismo de propagação Location é decisão
  arquitectural substancial — **provavelmente ADR
  PROPOSTO**.
- Se a decisão for replicação directa de mecanismo
  vanilla: ADR pode referenciar e ficar `ACEITE`.
- Se a decisão for cristalino-original: ADR `PROPOSTO`
  até validação P185B+.

Conclusão esperada: **ADR PROPOSTO**. Diferente de
P181A/P182A/P183A/P184A que não criaram ADR.

### Sub-passo 185A.J — Outputs

Produzir 3 ficheiros (padrão P181A–P184A):

1. **`00_nucleo/diagnosticos/diagnostico-location-aware-layouter-passo-185a.md`**
   — diagnóstico com 8 secções:
   - §1 Validação estado actual.
   - §2 Inventário (Layouter, Locator, Introspector
     methods, vanilla equivalente).
   - §3 Decisões cláusula 1–6 (formato O1–O5 + opção
     escolhida + justificação literal).
   - §4 Plano de sub-passos P185B+ sem condicionais.
   - §5 Magnitude consolidada (S diagnóstico + M-L
     implementação).
   - §6 ADR avaliação.
   - §7 DEBT avaliação.
   - §8 Próximo sub-passo (P185B com escopo concreto).

2. **`00_nucleo/materialization/typst-passo-185a-relatorio.md`**
   — relatório com 14 secções (padrão P181A/P182A/P183A/P184A).

3. **ADR `PROPOSTO`** se decisão substancial:
   - `00_nucleo/adr/typst-adr-NNNN-location-aware-layouter.md`
     (NNNN a atribuir; próximo após ADR-0067).
   - Documenta mecanismo escolhido em cláusula 1 +
     justificação + alternativas rejeitadas.
   - Status `PROPOSTO`; transita para `ACEITE` quando
     P185B+ implementação confirmar viabilidade.

---

## Restrições

- **Zero código tocado** em qualquer ficheiro fora de
  `00_nucleo/`.
- **Zero testes** modificados.
- **Não criar reservas** de identificadores.
- **Não modificar Layouter** — P185B+.
- **Não adicionar trait methods** — P185B.
- **Não modificar Locator** — P185B+.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como bandeira
  retórica.
- **Honestidade obrigatória**: se mecanismo escolhido for
  substancialmente mais caro que estimado, registar como
  tal — não disfarçar magnitude.
- **Sem cláusulas condicionais nos sub-passos `.B`+ do
  plano** P185B+.

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-location-aware-layouter-passo-185a.md`
  com 8 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-185a-relatorio.md`
  com 14 secções produzido.
- 6 cláusulas fechadas com decisão literal.
- Plano de sub-passos P185B+ sem condicionais —
  tabela com escopo + magnitude + dependência.
- Magnitude consolidada (S diagnóstico; M-L implementação
  esperada).
- Critério de fecho de P185 fixado em palavras
  verificáveis.
- ADR avaliada (provavelmente `PROPOSTO`).
- Nenhum ficheiro de cristalino tocado.
- Tests workspace 1.769 inalterados.
- `crystalline-lint .` zero violations.

P185A é instrumento. Implementação concreta de
location-aware Layouter começa em P185B.
