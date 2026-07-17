# Relatório de Paridade — P639

**Passo:** 639  
**Data:** 2026-07-09  
**Foco:** Determinar se `table.numbering` no cristalino é código morto, uma extensão genuína, ou um erro de paridade face ao vanilla.  
**Dependências:** P636 (validação de tipo de `table.numbering`), P638 (descoberta de que o vanilla não tem a propriedade).

---

## 1. Sonda

### 1.1 Onde `table.numbering` é definida e lida no cristalino

```bash
grep -n "table.*numbering\|numbering.*table" 01_core/src/engine/eval/rules.rs 01_core/src/engine/layout/*.rs 01_core/src/entities/elements/table*.rs 2>/dev/null
```

Resultado:

- `01_core/src/engine/eval/rules.rs:871-889` — processa `#set table(numbering: ...)` e guarda o valor na style chain como `custom("table.numbering")`.
- `01_core/src/engine/layout/table.rs:27-30` — lê `table.numbering` da style chain e, em conjunto com `caption`, computa o prefixo numerado (`Table 1: ...`).
- `01_core/src/engine/layout/tests.rs` — testes P459 confirmam o comportamento.
- `01_core/src/entities/elements/table.rs` — documentação da extensão P459 (`caption` opcional para numeração automática via `table.numbering`).

### 1.2 Teste directo no cristalino

Sem `caption`:

```typst
#set table(numbering: "1")
#table(
  columns: 2,
  [A], [B],
)
```

Output (`pdftotext`):

```text
AB
```

Com `caption`:

```typst
#set table(numbering: "1")
#table(
  columns: 2,
  caption: [My table],
  [A], [B],
)
```

Output:

```text
Table 1: My table
AB
```

Conclusão: `table.numbering` **tem efeito real** no cristalino quando a tabela tem `caption`. Não é código morto.

### 1.3 Equivalente no vanilla

No vanilla, `table` não tem propriedade `numbering`:

```bash
grep -n "figure.*numbering\|numbering.*figure" lab/typst-original/crates/typst-library/src/model/figure.rs
```

A numeração pertence a `figure`. Teste directo:

```typst
#figure(
  table(
    columns: 2,
    [A], [B],
  ),
  caption: [My table],
)
```

Output no vanilla:

```text
A B
Table 1: My table
```

Tentar `#set table(numbering: 123)` no vanilla produz:

```text
error: unexpected argument: numbering
```

E tentar passar `caption` a `table` no vanilla produz:

```text
error: unexpected argument: caption
```

---

## 2. Decisão

`table.numbering` é uma **extensão genuína do cristalino**, introduzida em P459, que permite numerar tabelas directamente sem as envolver em `#figure(...)`. A propriedade:

- é guardada na style chain;
- é lida durante o layout de `table`;
- tem efeito observável quando a tabela tem `caption`.

Portanto, **mantém-se**. A validação de tipo introduzida em P636 (`expected string or none, found {tipo}`) continua correcta dentro do cristalino.

A única mudança é de enquadramento: em vez de ser tratada como uma correcção de paridade com o vanilla, `table.numbering` é registada como capacidade extra do cristalino. Os relatórios de P633 e P636 foram actualizados com essa nota.

Não houve alterações de código.

---

## 3. Actualizações documentais

- `00_nucleo/diagnosticos/paridade-producao-p633.md`:
  - Entrada 11 da tabela de falhas silenciosas: adicionada nota de que `table.numbering` é extensão cristalina.
  - Secção 5.2.1 (Confirmado): adicionada nota contextual sobre P639 e a diferença para o vanilla.

- `00_nucleo/diagnosticos/paridade-producao-p636.md`:
  - Secção "Sonda": separada a menção a `table.numbering`, com nota sobre extensão P459.
  - Secção "Implementação": `table.numbering` destacado como extensão cristalina.
  - Tabela de mensagens de erro: `table.numbering` marcado como extensão cristalina.

---

## 4. Validação

```bash
cargo test --workspace
crystalline-lint .
```

Resultado:

- `cargo test --workspace` — todos os testes passaram.
- `crystalline-lint .` — `✓ No violations found`.

---

## 5. Estado de fecho

- [x] Confirmado que `table.numbering` no cristalino tem efeito real (não é código morto).
- [x] Confirmado que, no vanilla, a numeração de tabelas pertence a `figure`, não a `table`.
- [x] Decisão registada: manter `table.numbering` como extensão cristalina.
- [x] Relatórios de P633 e P636 actualizados com a decisão final.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p639.md`.
