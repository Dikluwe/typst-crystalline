# Passo 104 — Materializar `Sink` com dedup; 1 consumidor piloto

**Série**: 104 (passo único de construção; sub-passos de inventário
agressivo, ADR e verificação).
**Precondição**: Passo 103 encerrado; `#show` validado + ADR-0041;
795 L1 + 174 L3 + 6 ignorados; zero violations.
**ADRs aplicáveis**: ADR-0036 (atomização), ADR-0037 (coesão),
ADR-0040 (activação de `#set`), ADR-0041 (activação de `#show`).
**ADR nova**: ADR-00NN "Sink materializado em L1" — `PROPOSTO`
em 104.B, `EM VIGOR` em 104.E.

---

## Objectivo

Substituir o stub `Sink(())` (ou estado actual equivalente) por
um tipo real com API mínima:

- `warn(diag: SourceDiagnostic)` — acumula warning; deduplica por
  chave `(span, message)`.
- `into_diagnostics() -> Vec<SourceDiagnostic>` — consome e
  devolve todos.
- `is_empty() -> bool`.
- `new()` / `default()`.

Integrar **um** consumidor piloto — um sítio no eval onde hoje há
comentário de warning silencioso que passa a emitir via `Sink`. A
escolha do consumidor é decidida em 104.A com base no inventário.

O passo **não**:
- Migra DEBT-49 em massa (isso é passo separado).
- Materializa `Introspection`, `Engine<'a>`, outras folhas.
- Muda a arquitectura de `comemo::TrackedMut<Sink>` (se existir na
  assinatura de `eval`).
- Adiciona propagação de `Sink` a funções que não o recebem hoje.

---

## Decisões já tomadas

1. **API mínima**: `warn` + `into_diagnostics` + dedup por
   `(span, message)`.
2. **Dedup**: duas warnings idênticas (mesmo `Span` e mesma
   mensagem) são acumuladas uma só vez. Hints e severity não
   participam na chave.
3. **Consumidor piloto**: decidido em 104.A. Critério: o sítio
   que hoje tem comentário `DEBT-49` ou equivalente e tem call
   site único (baixo raio de explosão).
4. **DEBT-49**: permanece aberto após este passo. O consumidor
   piloto reduz-lhe uma entrada; o resto continua silenciado.

---

## Contexto empírico

Nas últimas sessões, o relatório de continuidade desalinhou três
vezes do estado real do código:

- Passo 99 pensou que `LazyHash` era stub em L1; ADR-0016 já
  estabelecia o oposto.
- Passo 102 pensou que `#set` não estava activo; estava desde o
  Passo 30.
- Passo 103 pensou que `#show` não estava activo; estava desde o
  Passo 70.

Por isso, **104.A é inventário agressivo**. Se o estado actual
de `Sink` revelar que já é um tipo real com API idêntica à
proposta, o passo vira formalização (ADR + testes de dedup +
consumidor piloto) em vez de materialização.

---

## Escopo

**Dentro**:
- `01_core/src/entities/sink.rs` — criar ou estender.
- `01_core/src/entities/world_types.rs` — remover `pub struct Sink(())`
  stub se existir.
- 1 consumidor piloto (eleger em 104.A).
- `01_core/src/entities/mod.rs` — re-export.
- Testes unitários de `Sink`.
- ADR nova.

**Fora**:
- Todos os restantes call sites silenciados (DEBT-49).
- `Routines`, `Traced`, `Engine<'a>`, `Introspection` — continuam
  como estão.
- Integração de `Sink` no L3/CLI (mostrar warnings ao utilizador).
  Isto é passo separado.

---

## Sub-passos

### 104.A — Inventário agressivo

**Parte 1 — Estado actual de `Sink`**:

1. Grep por `pub struct Sink` e `Sink(` em `01_core/src/`. Responder:
   - Sink é `struct Sink(())` stub, ou tem campos reais?
   - Se tem campos, quais? (Possível: `diagnostics: Vec<SourceDiagnostic>`;
     possível: outros, como `counters`, `inspectors`.)
   - Tem métodos: `warn`, `into_diagnostics`, outros?
2. Grep por `TrackedMut<Sink>` e `&mut Sink` no código. Responder:
   - A assinatura de `eval` tem `_sink: TrackedMut<Sink>` (como no
     Passo 12) ou mudou?
   - Algum outro sítio escreve em `Sink`?
3. Grep por `#[comemo::track]` perto de `impl Sink`. Responder:
   - Sink tem bloco `#[comemo::track]` que obrigue a API opaca?
   - Se sim, os métodos declarados são compatíveis com a API
     proposta (warn, into_diagnostics, is_empty)?

**Parte 2 — Candidatos a consumidor piloto**:

1. Grep por comentários `DEBT-49`, `silenciad`, `TODO.*warn`,
   `TODO.*sink`, `// warning` em `01_core/src/`. Listar todos.
2. Para cada candidato, anotar:
   - Ficheiro e linha.
   - O contexto (que warning seria emitido: "unsupported property",
     "unknown function", etc.).
   - Se o sítio tem acesso a um `Sink` propagado (via parâmetro
     `sink: &mut Sink` ou `TrackedMut<Sink>`).
3. Ranking de candidatos:
   - **Preferência 1**: DEBT-49 (propriedades `#set text(...)` não
     suportadas). É o sítio mais visível da necessidade de Sink.
   - **Preferência 2**: warning de tipo em `eval_binary_op` ou
     similar, se existir silenciado.
   - **Preferência 3**: qualquer outro com `Sink` já acessível.
4. Eleger o candidato com:
   - Acesso directo a `Sink` (não requer propagar parâmetro por
     muitas funções).
   - Baixo raio: 1 sítio, 1 mensagem, 1 teste possível.
   - Reversível (se alguma coisa corre mal, remover a chamada a
     `sink.warn` não quebra nada mais).

**Parte 3 — Ponto de saída dos warnings**:

1. Grep por `into_diagnostics` em `01_core/src/` e `03_infra/src/`.
   Hoje, os warnings do `Sink` chegam a algum sítio? Ou são
   descartados?
2. Se `eval` retorna `SourceResult<Module>`, os warnings vivem
   separadamente dos erros. Verificar se há um caminho para o
   caller (L3, CLI) extrair warnings do `Sink`.
3. Se não existe caminho, este passo não o cria — apenas
   documenta a lacuna. Os warnings ficam acumulados no `Sink` e
   são descartados no fim do eval. Isto é aceitável como estado
   intermédio; a conexão L1 → L3 é trabalho separado.

**Escrever em** `00_nucleo/diagnosticos/inventario-sink-passo-104.md`:

```
Sink estado actual:
  forma: struct Sink(()) | Sink { diagnostics: ..., ... }
  API: [lista de métodos existentes]
  #[comemo::track]: sim/não
  TrackedMut<Sink> no eval: sim/não

Candidatos a consumidor piloto:
  1. <ficheiro:linha> — <contexto> — acesso directo? sim/não
  2. ...
  Eleito: N.º X (razão)

Saída dos warnings hoje:
  [descrição do caminho, se existe; "descartado" se não]
```

**Gate**: se `Sink` **já** tem a API proposta (warn + dedup +
into_diagnostics), 104.C reduz-se a "adicionar dedup se falta" +
"adicionar consumidor piloto". Se não tem nada, 104.C é
materialização completa.

**Gate adicional**: se `#[comemo::track]` impõe assinatura
específica que colide com a API proposta, parar e reavaliar.
A API tem de ser compatível com o track; se não for, escolher
entre redesenhar ou adiar.

### 104.B — ADR nova

1. Criar `00_nucleo/adr/typst-adr-00NN-sink-materializado.md`
   com `PROPOSTO`.
2. Conteúdo:
   - Contexto: Sink era stub; o eval tem vários pontos de warning
     silenciado; DEBT-49 é sintoma. Passos 100–103 prepararam o
     terreno (`Content::Styled`, `#set`, `#show` funcionais).
   - Decisão: Sink real em L1 com `diagnostics: Vec<SourceDiagnostic>`
     + índice de dedup (HashSet de chaves ou equivalente).
   - API: `new`, `warn`, `into_diagnostics`, `is_empty`.
   - Dedup: chave `(Span, String)` em `HashSet` ou similar.
     Alternativa: varrer `Vec<SourceDiagnostic>` em cada `warn` —
     O(n) por chamada; rejeitado.
   - Razão para dedup: sem dedup, um aviso em hot loop (ex: cada
     nó de markup processado) inunda. Dedup torna o Sink seguro
     para integração em qualquer ponto.
   - Propriedades adiadas na API:
     - `error`: erros continuam a fluir por `SourceResult::Err`,
       não via Sink. Alinhado com vanilla.
     - `hint` multi-linha: `SourceDiagnostic::with_hint` já existe;
       Sink não precisa de o re-expor.
     - `counters`, `inspectors`: responsabilidade de `Introspection`,
       não de Sink.
   - Relação com `comemo::TrackedMut`: se o track impõe assinatura,
     registar; caso contrário, a API acima é definitiva.
   - Plano de migração: DEBT-49 migrado num passo separado com
     muitos call sites; este passo só toca 1.
3. `EM VIGOR` em 104.E.

### 104.C — Implementação

**104.C.1 — Struct `Sink`**:

Forma recomendada (adaptar ao que 104.A descobrir):

```rust
use rustc_hash::FxHashSet;  // já autorizado em L1 por ADR-0018
use crate::entities::{span::Span, source_result::SourceDiagnostic};

pub struct Sink {
    diagnostics: Vec<SourceDiagnostic>,
    seen: FxHashSet<(Span, String)>,
}

impl Sink {
    pub fn new() -> Self { Self::default() }

    pub fn warn(&mut self, diag: SourceDiagnostic) {
        let key = (diag.span, diag.message.clone());
        if self.seen.insert(key) {
            self.diagnostics.push(diag);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub fn into_diagnostics(self) -> Vec<SourceDiagnostic> {
        self.diagnostics
    }
}

impl Default for Sink {
    fn default() -> Self {
        Self {
            diagnostics: Vec::new(),
            seen: FxHashSet::default(),
        }
    }
}
```

Ajustar ao que 104.A revela:
- Se `Span` não é `Hash + Eq`, a chave precisa de serialização
  (ex: `span.debug_string()` ou conversão para tuple de `u32`).
  Verificar e adaptar.
- Se `Sink` tem `#[comemo::track]`, os métodos declarados têm de
  ser no bloco trackeado. Seguir o padrão que já existe no código.

**104.C.2 — Remover stub**:

Se `Sink(())` estava em `world_types.rs` ou equivalente, remover.
Re-export de `sink::Sink` onde for preciso.

**104.C.3 — Consumidor piloto**:

No sítio eleito por 104.A, substituir o silenciamento por chamada
a `sink.warn(SourceDiagnostic::warning(span, "mensagem"))`.

Exemplo, se o eleito for a propriedade "font" em `#set text(...)`:

```rust
// Antes (Passo 102, DEBT-49):
_ => {
    // DEBT: propriedades de #set text não suportadas são
    // silenciosamente ignoradas.
}

// Depois (piloto):
"font" => {
    sink.warn(SourceDiagnostic::warning(
        span,  // confirmar qual span está acessível
        "set text.font: propriedade ainda não suportada (ADR-00NN)",
    ));
}
_ => {
    // DEBT-49 continua a cobrir as restantes.
}
```

A mensagem deve:
- Identificar a propriedade (para o utilizador entender).
- Referenciar a ADR (para auditoria).
- Não prometer comportamento que o eval não tem.

**104.C.4 — Propagação**:

Se o consumidor piloto não tem `sink` acessível no seu frame,
pode ser preciso adicionar o parâmetro a uma função. **Limitar
propagação a uma função** — se a cadeia for mais profunda, o
piloto está mal escolhido (voltar a 104.A).

### 104.D — Testes

**Testes unitários de Sink** (em `entities/sink.rs` `#[cfg(test)]`):

1. `sink_vazio_is_empty` — `Sink::new().is_empty() == true`.
2. `warn_adiciona` — `warn` uma vez; `into_diagnostics` devolve
   um item.
3. `warn_duplicado_e_deduplicado` — mesma `(span, message)` duas
   vezes; `into_diagnostics` devolve um item.
4. `warn_mesmo_span_diferente_message` — dedup não aplica;
   devolve dois.
5. `warn_mesma_message_diferente_span` — idem; devolve dois.
6. `warn_diferente_severity_mesmo_par_dedup` — severity não
   participa na chave; mesmo (span, message) com severity
   diferente conta como duplicado; devolve um (documentar se
   for decidido diferente em 104.B).
7. `warn_diferente_hints_mesmo_par_dedup` — hints não participam.

**Teste do consumidor piloto** (end-to-end):

1. Input Typst com a construção que despoleta o warning (ex:
   `#set text(font: "Arial")`).
2. Verificar que `eval` completa (sem erro) e que o `Sink`
   devolvido contém exactamente um `SourceDiagnostic` com a
   mensagem esperada.
3. Teste de dedup: mesma construção duas vezes (ex:
   `#set text(font: "Arial")\n#set text(font: "Arial")`).
   Verificar um só warning.

Se o caminho para extrair warnings do `Sink` não existe (104.A
revela isto), o teste end-to-end pode precisar de acesso interno
via `pub(crate)` ou equivalente. Aceitável para este passo.

### 104.E — Encerramento

1. Grep: `struct Sink(())` retorna zero matches em L1 (stub
   removido).
2. Grep: `pub struct Sink` em `entities/sink.rs` com campos
   reais.
3. `cargo test --workspace`: ≥ linha de base + testes novos
   (795 + 7~10 = ~805).
4. `crystalline-lint` zero violations.
5. ADR promovida a `EM VIGOR`.
6. DEBT-49: actualizar. Uma entrada paga (a propriedade eleita);
   as restantes permanecem. Registar o número de call sites que
   ainda silenciam.
7. DEBT novo (se aplicável): "Warnings do Sink não chegam a L3/CLI
   — caminho de extração pendente". Se 104.A revelou que o
   caminho já existe, este DEBT não é necessário.
8. Relatório `typst-passo-104-relatorio.md`:
   - Estado de Sink antes/depois.
   - Escolha do consumidor piloto + razão.
   - Resultado do teste end-to-end (warning chega e deduplica).
   - Lacuna identificada (warnings → L3) ou caminho existente.
   - Tamanho do DEBT-49 restante.

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 104.A escrito.
2. ADR-00NN criada e promovida.
3. `Sink` com API `new`/`warn`/`is_empty`/`into_diagnostics` e
   dedup funcional.
4. Stub `Sink(())` removido (se existia).
5. Um consumidor piloto migrado de silenciamento para
   `sink.warn`.
6. Testes unitários de dedup passam.
7. Teste end-to-end do piloto passa (warning chega).
8. `cargo test --workspace` passa.
9. `crystalline-lint` zero violations.
10. DEBT-49 actualizado (−1 entrada).
11. Relatório 104.E escrito.

---

## O que pode sair errado

- **`Span` não é `Hash + Eq`**. Se `Span` é apenas `Copy + Debug`,
  a chave de dedup tem de ser construída manualmente. Verificar
  cedo em 104.C.1.
- **`Sink` com `#[comemo::track]` impõe API opaca**. Se o bloco
  tracked define métodos incompatíveis com `warn` / `into_diagnostics`,
  a integração pode não ser directa. 104.A detecta antes de
  chegar a 104.C.
- **Dedup por (span, message) colide com span detached**. Testes
  que usam `Span::detached()` podem ter `span` idêntico em
  múltiplos warnings diferentes. Regra: dedup por `(span, message)`
  colide quando ambos coincidem — é o comportamento desejado.
  Mas se a intenção é dedup só em pipeline real (não em testes
  artificiais), o teste tem de usar spans reais.
- **Piloto não tem `Sink` acessível**. Se o único sítio com
  warning silenciado está 5 funções abaixo de onde `Sink` é
  propagado, o piloto está mal escolhido. Reescolher em 104.A.
- **`into_diagnostics` consome self**. Se o caller precisa do
  `Sink` depois (improvável, mas), a API consome. Alternativa:
  `drain_diagnostics(&mut self) -> Vec<...>` que esvazia sem
  consumir. Decidir em 104.B; documentar.
- **Ordem dos warnings**. `Vec` preserva ordem de inserção;
  dedup não altera ordem (o primeiro a aparecer fica). Teste de
  regressão: warnings são devolvidos na ordem em que foram
  adicionados.
- **Warnings silenciosos tornam-se visíveis e quebram testes**.
  Se existem testes que passam hoje porque warnings são
  silenciados, e o consumidor piloto torna um desses warnings
  visível, o teste pode falhar (ex: esperava `Ok(module)` com
  `sink.is_empty()`). Detectar em 104.D e actualizar.

---

## Notas operacionais

- Este passo não activa o caminho warnings → L3/CLI. Os warnings
  ficam no `Sink`; se o caller (L3) quer mostrá-los, precisa de
  chamar `into_diagnostics` — passo separado.
- Este passo não toca visibilidade nem reestrutura ficheiros.
- Se 104.A revelar que o consumidor piloto exige mudar a
  assinatura de mais de uma função para propagar `Sink`, **parar
  e reescolher**. A propagação larga é trabalho do DEBT-49
  completo.
- Se 104.A revelar que `Sink` já tem a API e dedup implementado
  (e só falta formalização), este passo torna-se "ADR + testes +
  consumidor piloto" — seguir o padrão do Passo 103 (validação +
  documentação).
