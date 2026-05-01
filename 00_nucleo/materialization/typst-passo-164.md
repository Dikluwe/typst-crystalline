# Passo P164 — Extrair `is_locatable` como função pública (M2)

Passo único de M2 do refactor Introspection. Extrai
`is_locatable(content: &Content) -> bool` como função pura
em `rules/introspect/locatable.rs`, com invariante:
`is_locatable(c) == extract_payload(c).is_some()` para todo
`c: Content`.

Função fica disponível como utilitária para consumers
futuros (M3 `Introspector::from_tags`, M9 features novas).
**Walk não é modificado** neste passo — `is_locatable` é
recurso novo, não alteração de fluxo existente.

**Pré-condição**: M1 concluído (P161 + P162 + P163).
`extract_payload` existe em `rules/introspect/extract_payload.rs`.

**Restrições**:
- Não modificar walk em `rules/introspect.rs`.
- Não modificar `extract_payload`.
- Não tocar features novas (state, metadata, locate).
- Output observable não muda; snapshot tests passam inalterados.

---

## Sub-passos

### .A Inventário

Reverificar (não confiar em P163):

1. `extract_payload` em
   `01_core/src/rules/introspect/extract_payload.rs` existe e
   tem 3 arms locatable (Heading, Figure, Cite). Registar arms
   exactos.
2. `Content` enum tem N variants. Listar literalmente (grep
   ou inspecção do `entities/content.rs`). Comparar com arms
   de `extract_payload`:
   - Arms locatable: variants que retornam `Some(...)`.
   - Arms não-locatable: variants no fall-through `_ => None`
     ou variants ausentes do match (compilador deveria
     forçar exaustividade — verificar).
3. Confirmar que `rules/introspect/mod.rs` existe (criado em
   P162.D). Identificar exports actuais.
4. Localizar L0 actual (se existir):
   `00_nucleo/prompts/rules/introspect/extract_payload.md`.
   Confirmar formato. **Não** existe ainda
   `locatable.md` — será criado em .B.

Output: notas internas. Lista literal de variants
locatable e não-locatable, derivada de `Content` enum +
`extract_payload` arms.

**Critério de saída e gate de decisão**:
- Se `extract_payload` não tem fall-through `_ => None` mas
  match exaustivo explícito: registar para .B (implementação
  de `is_locatable` segue mesma forma).
- Se `Content` ganhou variants novos entre M1 e M2 (improvável
  mas possível): registar para .B (decidir locatability de
  cada novo variant). Cláusula gate trivial aplicável.
- Senão, prosseguir para .B.

### .B Criar L0+L1 de `is_locatable`

1. L0 em `00_nucleo/prompts/rules/introspect/locatable.md`:
   - Cabeçalho com campo "Hash do Código" em branco.
   - Camada L1, ficheiro alvo
     `01_core/src/rules/introspect/locatable.rs`.
   - ADRs: ADR-0033 (paridade), ADR-0066 (Introspection
     contexto).
   - Origem vanilla: nenhuma directa. Vanilla usa marker
     traits (`Locatable`, `Unqueriable`, `Tagged`); cristalino
     prefere função pura com match exaustivo. Documentar
     como divergência consciente: lista única auditável vs
     atributos dispersos por elementos.
   - Restrições estruturais:
     - Função pura, sem efeitos secundários.
     - `pub fn is_locatable(content: &Content) -> bool`.
     - Match exaustivo sobre `Content` (compilador força
       cobertura — adicionar variant novo a `Content`
       força decisão sobre locatability aqui).
     - **Invariante**: para todo `c: Content`,
       `is_locatable(c) == extract_payload(c).is_some()`.
   - Critérios de verificação:
     - `is_locatable(&Content::Heading {..})` retorna `true`.
     - `is_locatable(&Content::Figure {..})` retorna `true`.
     - `is_locatable(&Content::Cite {..})` retorna `true`.
     - `is_locatable(&Content::Text(..))` retorna `false`.
     - Pelo menos 3 outros variants não-locatable retornam
       `false`.
     - Invariante de equivalência com `extract_payload`
       verificada por test exhaustivo.

2. L1 em `01_core/src/rules/introspect/locatable.rs`:
   - Cabeçalho `@prompt 00_nucleo/prompts/rules/introspect/locatable.md`.
   - Implementação:

   ```rust
   use crate::entities::Content;

   pub fn is_locatable(content: &Content) -> bool {
       match content {
           Content::Heading { .. } => true,
           Content::Figure { .. } => true,
           Content::Cite { .. } => true,
           // Adaptar à lista exaustiva confirmada em .A:
           Content::Labelled { .. } => false,
           Content::Text(..) => false,
           Content::Sequence(..) => false,
           // ... outros variants não-locatable, todos com `=> false`
       }
   }
   ```

   Match deve ser **exaustivo** (sem `_ => false` fall-through).
   Razão: compilador força revisão quando variant novo é
   adicionado a `Content`. Se adicionar `_ => false`,
   variants novos são silenciosamente classificados como
   não-locatable, perdendo a invariante.

   - Tests co-localizados em `#[cfg(test)]`:
     - **Tests de cobertura**: para cada variant locatable,
       confirmar `true`; para pelo menos 3 não-locatable,
       confirmar `false`.
     - **Test de invariante**: para cada variant de
       `Content`, construir instância mínima e verificar
       `is_locatable(&c) == extract_payload(&c).is_some()`.

3. Update `01_core/src/rules/introspect/mod.rs`: re-export
   `is_locatable`.

**Critério de saída**:
- `cargo check` passa.
- `cargo test` — tests novos passam.
- L0 e L1 existem com cabeçalhos correctos.
- Linter passa (sincronização L0↔L1 verificada).

### .C Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace` — todos os tests passam. Δ vs
   baseline P163 (1555). Tests novos provavelmente entre
   5 e 10 (depende do número de variants em `Content`).
3. `crystalline-lint`: zero violations.
4. L0 `locatable.md` existe e tem cabeçalho correcto.
5. L1 `locatable.rs` existe com `pub fn is_locatable`.
6. `is_locatable` re-exportado em
   `rules/introspect/mod.rs`.
7. Walk em `rules/introspect.rs` **não modificado** (verificar
   que não há diff acidental).
8. Snapshot tests de paridade ADR-0033 passam inalterados.
9. Linter passa em verificação final.

### .D Encerramento

Escrever
`00_nucleo/materialization/typst-passo-164-relatorio.md` com:

- Resumo: `is_locatable` extraído como função pública;
  invariante com `extract_payload` verificado por test
  exaustivo; walk não tocado.
- Confirmação de cada verificação .C.
- Hash final de `00_nucleo/prompts/rules/introspect/locatable.md`
  (preenchido pelo linter).
- Decisões registadas em .A:
  - Lista de variants locatable + não-locatable (literal).
  - Forma do match em `is_locatable` (exaustivo vs
    fall-through).
- Estado pós-passo: M2 concluído. M3 desbloqueado — pode
  começar `Introspector::from_tags` que consome
  `Vec<Tag>` (consumer real das tags emitidas em P162).

---

## Critério de conclusão

Todas em conjunto:

1. .A produziu inventário sem disparar gate (ou gate
   trivial resolvido localmente).
2. L0+L1 de `is_locatable` criados com cabeçalhos correctos.
3. Tests de cobertura passam.
4. Test de invariante
   `is_locatable(c) == extract_payload(c).is_some()` passa
   exaustivamente.
5. Match em `is_locatable` é exaustivo (sem `_ =>` para
   forçar revisão futura).
6. Walk em `introspect.rs` não modificado.
7. Verificações .C 1-9 passam.
8. Relatório .D escrito.

---

## O que pode sair errado

- **Match exaustivo em `is_locatable` não compila**: alguns
  variants de `Content` podem ter campos com tipos genéricos
  ou tipos privados que tornam pattern matching com `..`
  problemático. Solução: usar `_` em campos individuais se
  necessário, mas manter exaustividade nos discriminants do
  enum.
- **Invariante falha**: se `is_locatable` lista variant como
  `true` mas `extract_payload` não o cobre (ou vice-versa).
  Test exaustivo detecta. Corrigir o que estiver errado:
  - Se variant é genuinamente locatable, adicionar arm em
    `extract_payload`.
  - Se variant é genuinamente não-locatable, retirar de
    `is_locatable`.
- **Variant novo em `Content` desde M1**: improvável mas
  possível. Cláusula gate trivial aplicável: decidir
  locatability local, documentar no relatório.
- **`Content::Cite` vs `Content::Citation`**: nome do variant
  no cristalino é `Cite` (decisão registada em P162.A). Não
  inventar `Citation`. Confirmar em .A.
- **Linter detecta divergência L0↔L1**: ao escrever L0 e L1,
  verificações de assinatura podem falhar. Ajustar conforme
  erro reportado.

---

## Notas operacionais

- **Tamanho**: S. Função única + tests + L0+L1. Mais leve
  que qualquer passo de M1.
- **Walk não tocado**: optimização "walk consulta
  `is_locatable` antes de `extract_payload`" deliberadamente
  adiada. Razão: `extract_payload` já faz match em `Content`
  e retorna `None` para não-locatable; chamar `is_locatable`
  primeiro é trabalho duplo no caso comum. Optimização real
  só vale se profiling mostrar que vale.
- **Pré-condição M3**: `is_locatable` está disponível como
  utilitária para `Introspector::from_tags` consultar quando
  decidir o que indexar. Sem isto, `Introspector` precisaria
  de duplicar lógica de classificação.
- **Cláusula gate trivial** (do P163): aplicável a este
  passo. Gates em .A que detectem divergências localmente
  resolúveis podem ser resolvidos sem parar, com decisão
  documentada no relatório.
- **Match exaustivo é decisão dura**: não usar `_ => false`
  como atalho. Compilador forçar revisão quando variant
  novo for adicionado é parte do mecanismo de isolamento.
