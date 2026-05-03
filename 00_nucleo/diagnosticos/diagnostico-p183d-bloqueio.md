# Diagnóstico — P183D bloqueio (C3 figure auto-number per kind)

**Data**: 2026-05-03
**Passo**: P183D — auditoria semântica explícita (regra dos 2 eixos)
**Escopo**: tentativa de migração do consumer C3
(`01_core/src/rules/layout/mod.rs:435–439`,
`self.counter.figure_numbers.get(kind_key).and_then(|v| v.get(idx))`)
para `Introspector::figure_number_at_index(kind, idx)` com fallback.
**Postura**: zero código tocado em L1–L4; zero testes modificados; zero
L0 modificado.

---

## §1 Premissa testada

> `figure_number_at_index(kind, idx)` Introspector tem paridade
> semântica com `figure_numbers[kind][idx]` legacy, suficiente para
> suportar substitution-with-fallback (padrão P168/P181G/P182D)
> sem regressões observáveis.

Aplicação literal da regra operacional dos 2 eixos consolidada em
P183C §6.

---

## §2 Resultado por eixo

### §2.1 Eixo 2 — existem dados no sub-store? **FALHA**

O `TagIntrospector` **não tem qualquer sub-store que registe contadores
de figure por kind**. Inspecção empírica:

1. **`from_tags` arm `ElementPayload::Figure`**
   (`01_core/src/rules/introspect/from_tags.rs:71–95`):
   ```rust
   ElementPayload::Figure { counter_update, is_counted, .. } => {
       intr.kind_index
           .entry(ElementKind::Figure)
           .or_default()
           .push(*loc);
       intr.counters.apply_at(
           "figure".to_string(),
           counter_update.clone(),
           *loc,
       );
       if *is_counted {
           if let Some(label) = &info.label {
               let next_num = intr.figure_label_numbers.len() + 1;
               intr.figure_label_numbers
                   .entry(label.clone())
                   .or_insert(next_num);
           }
       }
   }
   ```
   - **Chave única `"figure"`** no `CounterRegistry` (counter global,
     não por-kind).
   - O campo `kind: Option<String>` do payload é silenciosamente
     ignorado via `..` pattern.
   - `figure_label_numbers` indexa por `Label`, não por `(kind, idx)`.

2. **Comentário em `element_payload.rs:52–53`**:
   ```
   /// Update implícito do contador `figure:{kind}`.
   counter_update: CounterUpdate,
   ```
   Convenção documentada de chave `figure:{kind}` (ex. `figure:image`,
   `figure:table`) — **não está implementada** em `from_tags`.

3. **`CounterRegistry`** (`01_core/src/entities/counter_registry.rs`):
   - `value(key) -> Option<&[usize]>` retorna `Some(&[N_total_figures])`
     para `key = "figure"` (counter global).
   - Não há entry `"figure:image"`, `"figure:table"`, etc.
   - `value_at(key, location)` analogamente vazio para per-kind keys.

4. **Conclusão eixo 2**: `figure_number_at_index("image", 0)`
   (hipotético) não tem fonte de dados. Promoção de variant
   + arm em `from_tags` seria necessária — **trabalho infraestrutural
   substancial**, fora do escopo S de P183D.

### §2.2 Eixo 1 — semântica temporal coincide? **CONCEPTUALMENTE OK**

(Análise teórica, dado que eixo 2 já bloqueia.)

1. **Walk de introspect** (`01_core/src/rules/introspect.rs:391–399`)
   popula `state.figure_numbers["image"] = [1, 2, 3, ...]` em ordem
   de aparecimento. Após walk completo, `figure_numbers` está em
   snapshot final fixo.

2. **Copy-site `layout_with_introspector`** (`mod.rs:1414–1430` no-TOC,
   `mod.rs:1444–1460` TOC fixpoint): **NÃO copia `figure_numbers`**
   nem `local_figure_counters` para o Layouter. Comparar com
   campos copiados (`resolved_labels`, `headings_for_toc`,
   `numbering_active`, `bib_entries`, `bib_numbers`, `known_page_numbers`).

3. **Read site `mod.rs:432–439`**:
   ```rust
   let progress = self.figure_progress.entry(kind_key.to_string()).or_insert(0);
   let idx = *progress;
   *progress += 1;
   let figure_number = self.counter.figure_numbers
       .get(kind_key)
       .and_then(|v| v.get(idx))
       .copied()
       .unwrap_or(idx + 1);
   ```
   - `self.counter.figure_numbers` está **sempre vazio** porque
     `Layouter::new()` cria `CounterStateLegacy::new()` (default
     vazio) e nada é copiado de `initial_state.figure_numbers`.
   - O caminho realmente activado em produção é o `unwrap_or(idx + 1)`.
   - `figure_progress` (campo do Layouter, init vazio em
     `Layouter::new`, `mod.rs:152`) gera 0, 1, 2, ... → `idx + 1`
     dá 1, 2, 3, ... — sequência idêntica à que `figure_numbers`
     legacy teria se fosse copiada.

4. **Análise temporal**: figures recebem números fixos pós-walk
   (não dependem de hierarquia que se reorganiza durante re-walk,
   contrariamente a heading). Snapshot-final é a semântica
   **correcta** para figures. Logo, se eixo 2 fosse OK, eixo 1
   estaria conceptualmente alinhado.

5. **Conclusão eixo 1**: conceptualmente OK. Inviabilizado apenas
   pelo bloqueio em eixo 2.

### §2.3 Observação adicional — leitura legacy é dead code

Decorrência colateral do eixo 1: a leitura
`self.counter.figure_numbers.get(kind_key).and_then(...)` em
`mod.rs:435–438` é **dead code em produção**. `figure_numbers` está
sempre vazio na altura da consulta. O `idx + 1` fallback é o único
caminho activo. Esta observação é independente do bloqueio P183D —
diz respeito à higiene do código legacy e pode ser tratada num
passo separado de simplificação (remover ramo morto, documentar
explicitamente que `idx + 1` é a fonte autoritativa).

---

## §3 Categoria confirmada

C3 confirma a categoria **"snapshot-during-walk" pela perspectiva
oposta** — não porque a semântica temporal falha (não falha), mas
porque os dados não chegam ao sub-store. **Bloqueio por dados
ausentes**, comparável a C2 §2.2 (P183C) onde `from_tags` também
não tinha arm `Equation`.

Inferência consolidada após P183B + P183C + P183D:

| Consumer | Eixo 1 (semântica) | Eixo 2 (dados) | Estado |
|----------|--------------------|----------------|--------|
| C1 heading prefix | ❌ snapshot-during-walk | ✅ arm Heading popula | bloqueado P183B |
| C2 equation counter | ❌ snapshot-during-walk | ❌ sem arm Equation | bloqueado P183C |
| C3 figure auto-number per kind | ✅ snapshot-final adequado | ❌ chave global em vez de per-kind | **bloqueado P183D** |
| C4 resolved label | n/a (identidade) | ? (pendente P183E) | pendente |
| C5 TOC entries | n/a | n/a | bloqueado lacuna #3 separada |

C3 é o primeiro consumer onde **eixo 1 passa mas eixo 2 falha**.
Diferenças entre os 3 bloqueios:

- **C1**: precisa de Layouter location-aware + `flat_counter_at`
  no trait (M6+).
- **C2**: precisa de tudo de C1 + emissão de Tag `Equation` +
  arm `Equation` em `from_tags` (trabalho duplo).
- **C3**: precisa de promoção da convenção `figure:{kind}` (já
  documentada em `element_payload.rs:52`) para implementação real
  em `from_tags` arm Figure + método trait
  `figure_number_at_index(kind, idx)`. **NÃO precisa de
  Layouter location-aware** — eixo 1 já está alinhado.

---

## §4 Implicação para P183 e M+

### §4.1 Imediato

P183D **não migra C3**. Tudo revertido (zero código tocado em
`.A` + `.B`, ambos apenas leituras).

### §4.2 Para P183E (próximo passo da série)

P183E avança para C4 (resolved label, `references.rs:53`).
Categoria diferente: labels são identidade, não contadores
incrementais. Provável sucesso (sem auditoria de 2 eixos
necessária — mas o passo deve documentar essa diferença).

### §4.3 Para P183F (fecho da série)

DEBT M4-residual a abrir cobre **C1 + C2 + C3** — três consumers
com três sub-categorias distintas dentro da meta-categoria
"snapshot-during-walk" / "dados ausentes":

- **C1**: precisa de location-aware Introspector.
- **C2**: precisa de location-aware + emissão Tag + arm `Equation`.
- **C3**: precisa de promoção `figure:{kind}` + arm refinado +
  método trait `figure_number_at_index`.

C3 é o **mais barato dos três** para desbloquear (não precisa de
location-aware, que é um cross-cutting M6+ change). Pode ser
trabalhado isoladamente assim que a prioridade for justificada.

---

## §5 Estado pós-P183D

- **Código produção**: zero linhas tocadas em L1–L4. Linter
  inalterado.
- **Tests**: workspace baseline P183A mantido — 1.756 verdes; zero
  violations. (Sem alteração de código → sem possibilidade de
  regressão.)
- **L0**: zero prompts modificados. Trait `Introspector` continua
  com 15 métodos.
- **DEBT**: P183D **não abre DEBT formalmente**. DEBT M4-residual
  será aberto em P183F cobrindo C1+C2+C3.

---

## §6 Aprendizado meta — extensão da regra dos 2 eixos

P183B descobriu o eixo 1. P183C ratificou eixo 1 e revelou eixo 2.
P183D mostra que **os eixos são ortogonais**: pode passar um e falhar
o outro. C3 é o caso "eixo 1 OK, eixo 2 falha".

Refino operacional para passos M4-residual e M5+:

> A regra dos 2 eixos é uma **conjunção** — ambos os eixos têm de
> passar para a migração proceder. Eixos podem falhar
> independentemente:
>
> - **Falha só em eixo 1** (C1): solução é Layouter location-aware
>   + método `*_at(key, location)` no trait. Cross-cutting M6+.
> - **Falha só em eixo 2** (C3): solução é refinar o arm relevante
>   em `from_tags` + adicionar método trait dedicado. Localizado,
>   tratável isoladamente.
> - **Falha em ambos** (C2): solução requer trabalho em ambas as
>   frentes — pré-requisito de eixo 2 (emissão de Tag + arm) antes
>   de eixo 1 (location-aware) poder ajudar.
>
> A natureza do bloqueio dita a estratégia de desbloqueio. Não
> tratar todos os bloqueios "snapshot-during-walk" como
> equivalentes — o eixo que falha condiciona o trabalho necessário.
