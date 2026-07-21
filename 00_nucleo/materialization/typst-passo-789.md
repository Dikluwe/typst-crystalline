---
# P789 — Conflito de célula com header de tabela deve errar, não sobrepor silenciosamente

> **Passo:** 789
> **Data:** 2026-07-20
> **Foco:** P786 confirmou duas divergências em `utils::bitset` (grid/table): (1) `table.header` não se repete em páginas subsequentes quando a tabela quebra página (5 páginas no cristalino vs 7 no vanilla, header só na página 1); (2) uma célula com `rowspan` que colide com a região do header não é detectada — o vanilla erra (`error: cell would conflict with header also spanning row 0`, com hint), o cristalino aceita com exit 0 e produz um PDF com sobreposição visual.
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** M/L — repeat-across-páginas é o mesmo débito já registrado como scope-out em P772i (grid.header/footer); a detecção de conflito é um item separado, mais isolado.
> **ADR-0108 EM VIGOR** — confirmar se este passo reabre o débito de repeat-across-páginas de P772i ou é um achado novo mais restrito.
> **Prioridade:** Alta — o item 2 (conflito não detectado) é "aceita em silêncio quando deveria errar"; o item 1 é débito já conhecido.
> **Dependências:** P786 (achado, evidência em `temp/temp_p786/c_bitset_header.typ`, `c_bitset_conflict.typ`), P772i (débito de repeat-across-páginas já registrado e deferido).

---

## Passo 0 — Confirmar se o item 1 é o mesmo débito de P772i

```bash
grep -n -A10 "repeat-across-páginas\|repeat.*header" 00_nucleo/diagnosticos/paridade-producao-p772i.md 2>/dev/null
```

Se for exatamente o mesmo débito (header/footer renderiza só uma vez): confirmar se este passo deve implementá-lo agora (dado ter reaparecido como achado de alta prioridade em P786) ou se continua deferido — decisão a registrar, não assumida.

---

## Sonda — mecanismo exato do vanilla para detecção de conflito

```bash
grep -n "would conflict with header\|check_for_conflicting_cell_row" lab/typst-original/crates/typst-layout/src/grid/resolve.rs 2>/dev/null
```

Confirmar a lógica exata de detecção — provavelmente o mesmo `check_for_conflicting_cell_row` já citado como scope-out no achado #4 de P772i (item de `layout::grid::resolve` nunca implementado). Confirmar essa ligação antes de reimplementar do zero.

```bash
cat > /tmp/p789-conflict.typ <<'EOF'
#table(
  columns: 2,
  table.header[H1][H2],
  table.cell(rowspan: 2)[X], [Y],
)
EOF
lab/typst-original/target/release/typst compile /tmp/p789-conflict.typ 2>&1
```

---

## Decisão de âmbito

| Item | Decisão |
|---|---|
| Repeat-across-páginas (item 1) | Confirmar se implementa agora ou mantém deferido (P772i já avaliou como "esforço equivalente a um passo dedicado próprio") |
| Detecção de conflito (item 2) | Provavelmente implementável isolado — verificação de sobreposição de range antes de aceitar a célula, sem precisar do mecanismo de repetição |

---

## Implementação

Conforme a decisão — no mínimo, implementar a detecção de conflito (item 2), que é mais isolada e sempre "aceita em silêncio" (categoria de maior prioridade). Repeat-across-páginas só se a decisão do Passo 0 confirmar que vale fazer agora.

---

## Validação

```bash
./target/release/typst compile /tmp/p789-conflict.typ 2>&1
```

Confirmar erro idêntico ao vanilla.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Confirmado se item 1 é o mesmo débito de P772i.
- [ ] Decisão de âmbito registrada para os dois itens separadamente.
- [ ] Detecção de conflito de célula/header implementada, erro idêntico ao vanilla.
- [ ] Repeat-across-páginas implementado (se decidido) ou mantido deferido com justificativa atualizada.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p789.md`.

---

## Próximo passo

Show rule por string não aplicada em silêncio (grupo 4 de P786 §5), ou outro candidato da lista.
