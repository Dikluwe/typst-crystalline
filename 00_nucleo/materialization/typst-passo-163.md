# Passo P163 — Verificação E2E de M1: tests de captura, refino L0 `introspect.md` (M1 sub-passo 3/3)

Terceiro e último passo de M1 do refactor Introspection
(P161 + P162 + P163). Este passo:
1. Refina L0 de `00_nucleo/prompts/rules/introspect.md` para
   reflectir alterações de P162 (assinatura `walk` com 5
   parâmetros; emissão de tags em paralelo). Pendência
   herdada de P162 verificação .H.6.
2. Adiciona tests E2E de consistência entre `Vec<Tag>` e
   `CounterStateLegacy`. Sem isto, M2 (que vai consumir
   tags) pode descobrir lacunas tarde.

Após P163, M1 está concluído. M2 pode começar — extrair
`is_locatable` como função pública (primeiro passo a
consumir o `Vec<Tag>` que P163 verificou estar bem
capturado).

**Pré-condição**: P162 concluído. Walk emite `Vec<Tag>` em
paralelo a `CounterStateLegacy`; tags descartadas na API
pública.

**Restrições**:
- Não criar `Introspector` (M3 — passo futuro).
- Não consumir tags fora dos tests (introspect público
  continua a retornar apenas `CounterStateLegacy`).
- Não adicionar kinds novos a `ElementPayload` (M9).
- Output observable não muda; snapshot tests passam inalterados.

---

## Sub-passos

### .A Inventário

Reverificar (não confiar em P162):

1. Walk em `01_core/src/rules/introspect.rs`:
   - Assinatura actual com 5 parâmetros (incluindo
     `label_from_parent: Option<&Label>`, decisão registada
     em P162.A).
   - Emissão de `Tag::Start` + `Tag::End` em paralelo a
     mutação de `CounterStateLegacy`.
   - Helper `introspect_with_tags` em `#[cfg(test)]` para
     expor `Vec<Tag>` aos tests (criado em P162.G).
2. L0 actual `00_nucleo/prompts/rules/introspect.md`:
   - Ler conteúdo. Identificar secções existentes.
   - Confirmar que reflecte estado **pré-P162** (assinatura
     antiga de walk, sem menção a tags).
3. API pública de `CounterStateLegacy`:
   - Existe método `format_hierarchical()` ou equivalente?
     P162 relatório menciona — confirmar nome exacto e
     output (string formatada? estrutura?).
   - Existe método para contar headings, figures, citations
     individualmente? Se não, identificar como verificar
     contagens por kind.
4. Tests existentes em `rules/introspect.rs`:
   - Tests do walk com tags (P162.G) — confirmar 4 tests
     base passam.
   - Tests de paridade ADR-0033 — localizar conjunto
     completo (snapshot tests).
5. Helper de construção de Content para tests:
   - Existe builder/factory para construir `Content` mínimo
     (e.g. `make_heading("título")`, `make_figure(...)`)?
     Se não, criar helpers locais nos tests E2E em .C.

Output: notas internas. Sem diagnóstico separado a não ser
que algo divirja significativamente do esperado.

**Critério de saída e gate de decisão**:
- Se `walk` actual não tem 5 parâmetros conforme P162: **parar**
  e reportar — P162 incompleto.
- Se `format_hierarchical()` não existe e não há método
  equivalente para verificação cruzada: registar e adaptar
  testes em .C (gate trivial — ver "Notas operacionais"
  sobre cláusula).
- Senão, prosseguir para .B.

### .B Refinar L0 `introspect.md`

Pendência herdada de P162 verificação .H.6: L0 de
`introspect.rs` não foi actualizado em P162 sob justificação
"walk é interno". Decisão revisitada: L0 deve documentar
tudo o que está no L1, incluindo internals (decisão pós-debate
P162).

1. Ler L0 actual `00_nucleo/prompts/rules/introspect.md`.
   Identificar:
   - Secção sobre `walk` (assinatura, papel).
   - Secção sobre `introspect()` pública.
   - Secção sobre `materialize_time` (ou similar) se
     existir.
2. Update L0:
   - Assinatura actual de `walk` (5 parâmetros incluindo
     `label_from_parent`).
   - Comportamento dual: walk muta `CounterStateLegacy` E
     emite `Vec<Tag>` em paralelo.
   - Lógica de emissão: chamar `extract_payload`; se
     `Some`, alocar `Location`, construir `ElementInfo`,
     push `Tag::Start`; recurse com `label_from_parent`
     ajustado para variants `Content::Labelled`; se emitiu
     Start, push `Tag::End` com hash do nó.
   - API pública `introspect()` cria `Locator` + `Vec<Tag>`,
     chama walk, **descarta tags** (em M1; M2/M3 começarão a
     consumir).
   - Documentar como divergência consciente face a vanilla
     (que não tem walk explícito; usa fixpoint via
     `comemo`/`convergence`).
3. Não modificar L1 neste sub-passo. Apenas L0.

**Critério de saída**:
- L0 reflecte estado actual de L1.
- Linter passa (sincronização L0↔L1 verificada).
- `cargo check` continua a passar.

### .C Helper E2E e tests de bracketing

1. Confirmar (ou criar se ausente) helper de teste
   `introspect_with_tags` em `rules/introspect.rs`. P162.G
   já criou — verificar acessibilidade aos tests E2E.

2. Tests E2E em `rules/introspect.rs` ou módulo dedicado de
   tests (`#[cfg(test)] mod e2e_paralelismo`):

#### .C.1 Determinismo do walk

Walk duas vezes sobre o mesmo Content produz `Vec<Tag>`
**idêntico** (mesma ordem, mesmas Locations, mesmos hashes).

```rust
#[test]
fn walk_e_deterministico() {
    let content = make_content_complexo();  // headings + figures + citations
    let (_, tags1) = introspect_with_tags(&content);
    let (_, tags2) = introspect_with_tags(&content);
    assert_eq!(tags1, tags2);
}
```

Se este test falha, `Locator` ou `hash_content` têm
não-determinismo. Investigar antes de prosseguir.

#### .C.2 Bracketing válido em sequências complexas

Walk sobre Content com aninhamento múltiplo (heading dentro
de figure dentro de heading) produz tags com bracketing
válido:

```rust
#[test]
fn bracketing_valido_em_aninhamento_complexo() {
    let content = make_heading_com_figure_com_citation();
    let (_, tags) = introspect_with_tags(&content);

    let mut stack = Vec::new();
    for tag in &tags {
        match tag {
            Tag::Start(loc, _) => stack.push(*loc),
            Tag::End(loc, _) => {
                let top = stack.pop().expect("End sem Start correspondente");
                assert_eq!(top, *loc, "End com Location diferente do último Start");
            }
        }
    }
    assert!(stack.is_empty(), "Start sem End correspondente");
}
```

Caso adicional: sequência com múltiplos headings ao mesmo
nível (não aninhados) — verificar bracketing igualmente.

#### .C.3 Hash em End reflecte conteúdo distinto

Dois Contents do mesmo kind mas conteúdos diferentes
produzem `u128` diferentes em `Tag::End`:

```rust
#[test]
fn end_hash_distingue_conteudo() {
    let content_a = make_heading("Título A");
    let content_b = make_heading("Título B");
    let (_, tags_a) = introspect_with_tags(&content_a);
    let (_, tags_b) = introspect_with_tags(&content_b);

    let end_a = tags_a.iter().find_map(|t| match t {
        Tag::End(_, h) => Some(*h),
        _ => None,
    }).unwrap();
    let end_b = tags_b.iter().find_map(|t| match t {
        Tag::End(_, h) => Some(*h),
        _ => None,
    }).unwrap();

    assert_ne!(end_a, end_b);
}
```

**Critério de saída** (sub-passos .C.1 a .C.3):
- 3 tests novos passam.
- `cargo test` — todos os tests passam.
- Linter passa.

### .D Tests de consistência por kind

Walk sobre Content estruturado produz `Vec<Tag>` consistente
com `CounterStateLegacy`. Verificação cruzada por kind.

#### .D.1 Consistência heading

Walk sobre Content com N headings em níveis variados produz:
- N pares `(Tag::Start, Tag::End)` com `ElementPayload::Heading`.
- `depth` (ou `level` conforme nome real) de cada
  `ElementPayload::Heading` corresponde ao nível esperado.
- `CounterStateLegacy` tem o mesmo número de headings
  registado (verificável via `format_hierarchical()` ou
  método equivalente confirmado em .A número 3).

```rust
#[test]
fn headings_capturados_em_paralelo() {
    let content = make_content_headings_levels(&[1, 2, 2, 3]);
    let (state, tags) = introspect_with_tags(&content);

    let heading_levels: Vec<usize> = tags.iter()
        .filter_map(|t| match t {
            Tag::Start(_, info) => match &info.payload {
                ElementPayload::Heading { depth, .. } => Some(*depth),
                _ => None,
            },
            _ => None,
        })
        .collect();

    assert_eq!(heading_levels, vec![1, 2, 2, 3]);
    // Verificação cruzada com CounterStateLegacy:
    // (depende do método identificado em .A número 3)
    // Exemplo:
    // assert_eq!(state.heading_count(), 4);
    // ou:
    // assert_eq!(state.format_hierarchical().lines().count(), 4);
}
```

Adaptar verificação cruzada à API real de
`CounterStateLegacy` confirmada em .A.

#### .D.2 Consistência figure

Análogo a .D.1 mas para figures.

#### .D.3 Consistência citation

Análogo a .D.1 mas para citations. Verificar que cada
citation no input produz exactamente um
`Tag::Start(_, ElementPayload::Citation { key })` com a
key correcta.

**Critério de saída** (sub-passos .D.1 a .D.3):
- 3 tests novos passam.
- `cargo test` — todos os tests passam.
- Linter passa.

### .E Documentar lacunas detectadas

Se algum teste de .C ou .D detectou lacunas (e.g.
`CounterStateLegacy` regista informação que tags não
capturam, ou vice-versa), criar nota em
`00_nucleo/diagnosticos/m1-lacunas-captura.md` listando:

- Qual campo de `CounterStateLegacy` não tem equivalente em
  tags.
- Qual informação em tags não tem equivalente em
  `CounterStateLegacy` (provavelmente nada em M1, mas
  registar).
- Decisão sobre cada lacuna: corrigir agora (estender
  `ElementPayload` ou ajustar `extract_payload`) ou adiar
  para M2/M3.

Se não houver lacunas, ficheiro **não é criado**. Documentar
isso no relatório de conclusão (.G).

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace` — todos os tests passam.
   Contagem aumenta vs baseline P162 (smoke V2 dos tests
   E2E em .C e .D). Documentar Δ.
3. `crystalline-lint`: zero violations.
4. L0 `00_nucleo/prompts/rules/introspect.md` reflecte
   walk com 5 parâmetros + emissão de tags.
5. Helper `introspect_with_tags` acessível aos tests
   (P162.G + reusado em P163).
6. Tests .C.1, .C.2, .C.3 passam.
7. Tests .D.1, .D.2, .D.3 passam.
8. Se houve lacunas detectadas, ficheiro
   `00_nucleo/diagnosticos/m1-lacunas-captura.md` existe.
9. Snapshot tests de paridade ADR-0033 passam inalterados.
10. Linter passa em verificação final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-163-relatorio.md` com:

- Resumo: L0 de `introspect.rs` refinado; 6 tests E2E
  adicionados; M1 inteiro concluído.
- Confirmação de cada verificação .F.
- Hash actualizado de
  `00_nucleo/prompts/rules/introspect.md` (preenchido pelo
  linter).
- Decisões registadas em .A:
  - API real de `CounterStateLegacy` para verificação
    cruzada (`format_hierarchical()` ou equivalente).
  - Helpers de Content criados (se algum).
- Resultado das verificações de captura: lacunas detectadas
  ou "captura completa".
- Estado pós-passo: M1 concluído. M2 (extrair `is_locatable`
  como função pública e iniciar consumo de `Vec<Tag>`)
  desbloqueado.

---

## Critério de conclusão

Todas em conjunto:

1. .A produziu inventário sem disparar gate (ou gate trivial
   resolvido localmente — ver "Notas operacionais").
2. L0 de `introspect.rs` reflecte estado actual de L1.
3. 6 tests E2E novos (3 em .C + 3 em .D) passam.
4. Walk emite tags determinísticas, com bracketing válido,
   consistentes com `CounterStateLegacy` para os 3 kinds
   (Heading, Figure, Citation).
5. Verificações .F 1-10 passam.
6. Relatório .G escrito.
7. M1 inteiro concluído (P161 + P162 + P163).

---

## O que pode sair errado

- **Test de determinismo (.C.1) falha**: `Locator` ou
  `hash_content` produz output não-determinístico. Prováveis
  causas: ordem de iteração sobre `HashMap` em
  `hash_content`, ou determinismo do `Locator` quebrado
  pela mudança de assinatura em P162. Corrigir antes de
  prosseguir.
- **Bracketing válido (.C.2) falha**: indicaria bug em walk
  — provável early return em alguma arm que pula emissão
  de `Tag::End`. Investigar.
- **Hash em End indistinguível (.C.3) falha**: `hash_content`
  produz mesmo hash para Contents diferentes. Pode ser que
  `format!("{:?}", ...)` (decisão P162.A) colapse
  diferenças. Reabrir decisão sobre forma de hash.
- **Consistência heading (.D.1) detecta divergência**: número
  de headings em tags difere de
  `CounterStateLegacy.heading_count()` (ou equivalente).
  Provável causa: walk arm para Heading trata casos
  especiais (e.g. headings em outline são ignorados em
  counter mas emitem tag, ou vice-versa). Decidir: corrigir
  `extract_payload` ou registar como divergência conhecida
  em .E.
- **`CounterStateLegacy` não expõe método para verificação
  cruzada**: campo "heading count" pode ser interno. Se
  precisar de adicionar getter para tests, fazer
  `pub(crate)`. Documentar.
- **Lacunas listadas em .E são muitas**: se
  `CounterStateLegacy` tiver muitos fields sem equivalente
  em `ElementPayload`, pode indicar que M1 estava incompleto
  e era preciso cobrir mais kinds. Decidir: expandir M1
  (P163.5 adicional) ou registar como dívida para M2.
- **Linter detecta divergência L0↔L1 no `introspect.md`
  refinado**: ao actualizar o L0 em .B, podem aparecer
  divergências subtis (ex. nomenclatura de campos, ordem
  de parâmetros). Ajustar conforme erro reportado.

---

## Notas operacionais

- **Tamanho**: M. Sem L0+L1 novos; apenas refino de L0
  existente + 6 tests E2E + helpers de teste. Mais leve
  que P161 e P162.
- **Pré-condição M2**: M1 inteiro concluído. M2 vai extrair
  `is_locatable` como função pública e começar a consumir
  `Vec<Tag>`. Sem P163, M2 não pode começar com confiança
  na qualidade da captura.
- **Cláusula sobre gates triviais (adoptada a partir deste
  passo)**: gates em sub-passos `.A` que detectem divergências
  pequenas e localmente resolúveis (ex. nome de método
  ligeiramente diferente, mecanismo idiomático equivalente)
  podem ser resolvidos imediatamente sem parar o passo,
  desde que:
  - A decisão seja documentada no relatório de encerramento.
  - A solução seja local (não tem efeito em outros passos
    nem rompe restrições do passo actual).
  - O critério de saída do passo continua a poder ser
    cumprido.
  Gates substanciais (mudança arquitectural, decisão entre
  Opção A vs Opção B, descobrir que pré-condição está
  ausente) continuam a parar o passo e reabrir decisão.
  Esta cláusula formaliza o que P162.A fez com o mecanismo
  de label (resolveu localmente o `Content::Labelled`
  wrapper sem parar). Aplica-se a P163 e a passos seguintes.
- **Tests só leem tags via helper**: `introspect_with_tags`
  é `pub(crate)` e só visível em `#[cfg(test)]`. API pública
  `introspect()` continua a descartar tags. Garante que
  tests não viciam consumers reais que ainda não consomem
  tags.
- **`format_hierarchical()` ou alternativas**: .A número 3
  precisa identificar o método real. Se o relatório P162
  mencionou `format_hierarchical()`, usar. Se não bate com
  L1 actual, usar o método equivalente.
