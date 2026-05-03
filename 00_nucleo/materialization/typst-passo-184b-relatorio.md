# Relatório P184B — refinamento do arm `Figure` em `from_tags`

**Data**: 2026-05-03
**Passo**: P184B — refinar arm Figure (chave `figure:{kind}`)
**Resultado**: arm refinado; chave global `"figure"` mantida em
paralelo; Δ tests workspace 0 (1.756 inalterado); zero violations
linter.

---

## §1 Resumo

`from_tags.rs:71–112` arm `ElementPayload::Figure` agora popula
`CounterRegistry` com chave per-kind `figure:{kind}` (default
`figure:image` quando `kind == None`), em paralelo à chamada
existente que mantém a chave global `"figure"` (preservada por
simetria com walk legacy até M6 — P184A cláusula 5).

L0 `00_nucleo/prompts/rules/introspect/from_tags.md` actualizado a
descrever a convenção promovida do doc comment de
`element_payload.rs:52` para implementação real.

Sem método trait novo (P184C). Sem consumer migrado (P184D).

---

## §2 Sub-passos executados

| Sub-passo | Estado | Notas |
|-----------|--------|-------|
| `.A` Auditoria L0 | ✅ | Arm Figure confirmado em `from_tags.rs:71-95`; `kind: Option<String>` no payload; `apply_at` aceita `String`; tests existentes (`from_tags.rs:339-396`) sobre `figure_label_numbers`/`kind_index[Figure]` independentes da chave do counter — não regridem. |
| `.B` Actualizar L0 | ✅ | `from_tags.md` linha 34 reescrita; entrada nova no histórico de revisões. |
| `.C` Refinar arm | ✅ | Destructure `kind` adicionado; `apply_at(format!("figure:{}", kind_key), ...)` adicionado antes da chamada global; chamada global preservada com comentário de propósito. `kind_key = kind.as_deref().unwrap_or("image")`. |
| `.D` Verificação regressão | ✅ | `cargo check --workspace` passa; `cargo test --workspace` passa (1.496 + 215 + 24 + 21 = **1.756 verdes**, baseline preservado). `crystalline-lint .` zero violations após `--fix-hashes` (recálculo automático do hash L0 → `d0113a49`). |
| `.E` Encerramento | ✅ | Este relatório. |

---

## §3 Confirmação `.D`

- `cargo check --workspace`: **passa** (warnings pré-existentes não relacionados).
- `cargo test --workspace`: **1.756 passed**, 7 ignored, 0 failed.
  - typst-core lib: 1.496 passed.
  - typst-infra lib: 215 passed (+ 6 ignored).
  - typst-shell lib: 24 passed.
  - typst-shell integration: 21 passed (+ 1 ignored).
- `crystalline-lint .`: ✓ **No violations found**.
- Δ vs P184A baseline: **0 tests** novos ou modificados.

Tests específicos `from_tags`: 20 passed (incluindo
`figura_numerada_com_label_popula_figure_label_numbers`,
`figuras_numeradas_recebem_numeros_sequenciais`,
`sequencia_mista_isola_por_kind`, etc.) — todos os 20 verdes.

---

## §4 Hashes finais L0 modificado

- `00_nucleo/prompts/rules/introspect/from_tags.md` — Hash do
  Código: `7ab14b2f` (anterior `9acddbb4`).
- `01_core/src/rules/introspect/from_tags.rs` — `@prompt-hash`:
  `d0113a49` (anterior `2010372a`).

Sincronizados via `crystalline-lint --fix-hashes .`.

---

## §5 Decisões de execução notáveis

1. **Ordem das duas chamadas `apply_at`**: per-kind vem **antes** da
   global. Sem implicações funcionais (CounterRegistry indexa por
   chave; cada chave tem o seu Vec<usize> independente; ordem de
   inserção entre chaves diferentes é irrelevante para a leitura).
   A escolha é de legibilidade — chave nova primeiro, legacy depois,
   com comentário a documentar o propósito de cada uma.

2. **`String::from("figure")` vs `"figure".to_string()`**: mantido
   `to_string()` por simetria com a chamada original (não introduz
   estilo novo).

3. **Sem alteração à chamada `kind_index`**: per restrição P184B
   "Não modificar consumers existentes de `kind_index[Figure]`". A
   única alteração é a adição do `apply_at` per-kind e o destructure
   de `kind` no pattern.

4. **Sem cleanup do dead code legacy**: per P184A cláusula 5 e
   restrição P184B "Não modificar walk arm Figure em
   `introspect.rs`". O cleanup de `state.figure_numbers` e
   `local_figure_counters` fica para M6 quando `CounterStateLegacy`
   for eliminado.

---

## §6 Estado actual

- **P184 série**: A ✅ B ✅ | C–F pendentes.
- **C3 desbloqueio em curso** (eixo 2 de P183D):
  - Eixo 1 (semântica temporal): já era OK pré-P184 (figures
    pós-walk fixos; snapshot-final apropriado).
  - Eixo 2 (dados em sub-store): **parcialmente atendido** —
    `CounterRegistry` agora recebe entries `figure:{kind}` para
    cada figure processada via `from_tags`. Falta:
    - Método trait `figure_number_at_index` (P184C).
    - Helper `value_at_index` no `CounterRegistry` se necessário
      (P184C avalia).
    - Consumer C3 migrado para usar o método trait (P184D).
- **M9**: 11/11 (inalterado).
- **M5/M4 progresso**: 5 read-sites migrados (sem mudança em
  P184B; consumer C3 migra em P184D).
- **39 passos executados** (P184A + P184B = +2 desde 37 da
  contagem cumulativa em `typst-passo-183d-relatorio.md` §4).

---

## §7 Pendências cumulativas

Inalteradas em relação ao estado pós-P184A:

- Lacuna #3 (TOC entries via Introspector) — bloqueada,
  separada da série P184.
- DEBT M4-residual a abrir em P183F cobrindo C1+C2+C3 (P184F
  reduzirá para C1+C2 após este passo + P184C–E completarem).
- Pendência paralela P182E §5.2 (location-aware Layouter para
  desbloquear C1+C2) — espera M6+.

---

## §8 Próximo passo — P184C

Adicionar método trait `figure_number_at_index(&self, kind: &str, idx: usize) -> Option<usize>` ao `Introspector` + impl em
`TagIntrospector` que delega ao `CounterRegistry`. P184C avalia se
`CounterRegistry` precisa de helper `value_at_index` (acesso por
posição na `history` do counter, em vez de por `Location`).

5 tests unitários (padrão P181F/P182B):
1. Vazio devolve `None`.
2. Após populate retorna `Some(N)`.
3. Kinds distintos isolados (`figure:image` vs `figure:table`).
4. Idx fora de range devolve `None`.
5. Default kind (`None` → `"image"`) — testar via populate com
   kind `None` e leitura via `figure_number_at_index("image", 0)`.

Pré-condição P184C: este passo concluído (P184B); chave
`figure:{kind}` populada no `CounterRegistry`.
