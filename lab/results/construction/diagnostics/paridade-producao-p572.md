# P572 — Decisão sobre o código L1 sem L0 (`layout_space`)

**Status:** `CONCLUÍDO` (código L1 órfão descartado; relatório P569 corrigido)  
**Data:** 2026-07-05  
**Scope:** `01_core/src/engine/layout/cursor.rs`, `01_core/src/engine/layout/mod.rs`, `01_core/src/engine/layout/text.rs`, `00_nucleo/diagnosticos/paridade-producao-p569.md`.

---

## 1. Resumo executivo

P571 encontrou código em L1 que introduz `layout_space()` e um campo `pending_space_width`, sem Prompt L0 correspondente e sem estar commitado. Este passo determinou o destino desse código.

**Decisão:** descartar o código L1. Ele é **redundante** face à solução já implementada e commitada por P569 em L3 (`03_infra/src/layout_bidi.rs`), e está **incompleto** (`pending_space_width` declarado mas nunca lido). P569 resolve o problema do espaço entre palavras em RTL sem tocar no Layouter, conforme o L0 vigente de `layout_bidi.md`.

**Acções:**

- Código L1 revertido para HEAD (`01_core/src/engine/layout/cursor.rs`, `mod.rs`, `text.rs`).
- Snapshots P307b regenerados com L1 revertidos para o estado de HEAD.
- Relatório P569 corrigido para remover a classificação incorrecta das 5 falhas como "pré-existentes".
- Nenhum L0 novo foi escrito, porque o código não justifica a sua própria existência.

---

## 2. Sonda

### 2.1 Código encontrado

```bash
git diff HEAD -- 01_core/src/engine/layout/cursor.rs 01_core/src/engine/layout/mod.rs 01_core/src/engine/layout/text.rs
```

O diff mostrou três alterações em L1:

**`01_core/src/engine/layout/cursor.rs`**

- Adiciona `pub(super) fn layout_space(&mut self)`.
- O comentário afirma: "P568/P569 — preserva o espaço entre palavras como conteúdo textual" e explica que anexa o espaço ao último `FrameItem::Text` (trailing space) para que o `layout_bidi` posterior o coloque correctamente em RTL.
- Altera `flush_line()` para fazer `self.pending_space_width = Pt(0.0)`.

**`01_core/src/engine/layout/mod.rs`**

- Adiciona campo `pub(super) pending_space_width: Pt` na struct `Layouter`.
- Inicializa o campo a `Pt(0.0)`.
- Altera o braço `Content::Space` de:
  ```rust
  self.regions.current.cursor_x += self.space_width();
  if self.regions.current.cursor_x.0 > self.regions.current.width - self.page_config.margin {
      self.flush_line();
  }
  ```
  para:
  ```rust
  self.layout_space();
  ```

**`01_core/src/engine/layout/text.rs`**

- Substitui `layouter.regions.current.cursor_x += layouter.space_width();` por `layouter.layout_space();`.

### 2.2 Origem documentada

```bash
grep -rn "layout_space\|pending_space_width" 00_nucleo/ 2>/dev/null
```

Resultado: apenas referências no relatório P571 e no próprio passo P572. **Não existe nenhum outro documento** (L0, relatório, ADR, materialization) que explique ou legitime este código.

### 2.3 Avaliação do propósito

| Aspecto | Observação |
|---|---|
| Problema que resolve | Preservar o espaço entre palavras em texto RTL para extratores sequenciais. |
| Já resolvido por P569? | **Sim.** P569 em L3 faz ordenação por x crescente, coalescência de espaços no item anterior e separação de sufixos LTR. |
| Local da solução | L1 (`Layouter`) vs L3 (`layout_bidi.rs`, pós-layout). |
| L0 vigente | `00_nucleo/prompts/infra/layout_bidi.md` (hash `603ffda0`, P569) rejeita explicitamente mudar o Layouter. |
| Completo? | **Não.** `pending_space_width` é declarado, inicializado e resetado, mas **nunca lido**. A implementação actual faz trailing space, enquanto o comentário de `mod.rs` fala em leading space. |
| Impacto mecânico | Altera todos os 9 snapshots P307b, não só os de RTL. |

**Conclusão da sonda:** o código é uma tentativa alternativa/redundante ao que P569 já resolveu, e está incompleta.

---

## 3. Decisão

**Caminho escolhido:** 2 — descartar o código L1.

**Razão:**

1. P569 já resolve o problema do espaço em RTL em L3, sem violar a fronteira L1/L3 nem o L0 vigente.
2. O código L1 é incompleto (`pending_space_width` não utilizado) e afeta negativamente todos os snapshots.
3. Não há documentação de origem que justifique mantê-lo.
4. Escrever um L0 novo para este código seria legitimar uma abordagem que compete com a já escolhida por P569.

---

## 4. Execução

### 4.1 Descarte do código L1

```bash
git checkout HEAD -- 01_core/src/engine/layout/cursor.rs 01_core/src/engine/layout/mod.rs 01_core/src/engine/layout/text.rs
```

### 4.2 Reversão dos snapshots P307b

Os snapshots no working tree tinham sido regenerados com o código L1 presente. Sem ele, deixaram de ser consistentes, pelo que foram revertidos para o estado de HEAD:

```bash
git checkout HEAD -- 03_infra/fixtures/p307b/reference/
```

### 4.3 Correcção do relatório P569

Ficheiro: `00_nucleo/diagnosticos/paridade-producao-p569.md`

- Secção 4.3: adicionada actualização a indicar que a afirmação "pré-existente" para 5 falhas estava incorrecta.
- Secção 5: corrigida a lista de falhas para reflectir que apenas `07-multi-feature` era preexistente; as falhas 01, 02, 03, 09 eram do código L1 órfão.
- Secção 6: adicionada nota de que a solução de P569 em L3 é suficiente e que o código L1 foi descartado.

---

## 5. Validação

### 5.1 Testes de layout_bidi

```bash
cargo test -p typst-infra layout_bidi
```

Resultado:

```text
test result: ok. 10 passed; 0 failed; 0 ignored
```

### 5.2 Snapshots P307b

```bash
cargo test -p typst-infra p307b_snapshot_tests
```

Resultado:

```text
test result: ok. 9 passed; 0 failed; 0 ignored
```

### 5.3 Código L1 removido

```bash
grep -rn "layout_space\|pending_space_width" 01_core/src/
```

Resultado: nenhuma ocorrência.

---

## 6. Próximos passos

1. Resolver o estado pendente de P568 (`03_infra/src/export/builder.rs`, `fonts.rs`, `shaper.rs`, `export/mod.rs` e `04_wiring/Cargo.toml`). O `Cargo.toml` foi temporariamente revertido para permitir o build durante P572; se P568 for mantido, o exemplo `test_lexer` precisa de ser criado ou o `Cargo.toml` ajustado.
2. Quando P568 estiver completo e consistente, regenerar os snapshots P307b se necessário.
3. Nenhuma acção adicional relacionada com o código L1 órfão — foi descartado.

---

## 7. Ficheiros alterados

| Ficheiro | Alteração |
|---|---|
| `01_core/src/engine/layout/cursor.rs` | Revertido para HEAD (removido `layout_space`). |
| `01_core/src/engine/layout/mod.rs` | Revertido para HEAD (removido `pending_space_width` e braço `Content::Space` alterado). |
| `01_core/src/engine/layout/text.rs` | Revertido para HEAD (volta a usar avanço directo de `cursor_x`). |
| `03_infra/fixtures/p307b/reference/*.pdf` | Revertidos para HEAD. |
| `00_nucleo/diagnosticos/paridade-producao-p569.md` | Corrigida a classificação das falhas. |
| `00_nucleo/diagnosticos/paridade-producao-p572.md` | Criado. |

---

## 8. Conclusão

O código L1 órfão `layout_space`/`pending_space_width` foi identificado como uma tentativa redundante e incompleta face à solução de P569. Foi descartado, os snapshots foram revertidos para o estado consistente com o histórico commitado, e o relatório P569 foi corrigido. A solução de P569 em L3 continua válida e suficiente sozinha.
