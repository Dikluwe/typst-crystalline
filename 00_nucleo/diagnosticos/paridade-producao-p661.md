# Relatório de Paridade — P661

**Passo:** 661  
**Data:** 2026-07-09  
**Foco:** Verificar se a numeração automática de tabelas da extensão P459 (`table.numbering` + `caption`) responde a `#show figure.where(kind: table): ...`.  
**Dependências:** P459 (extensão `table.numbering`), P639 (confirmação de que é extensão, não paridade).

---

## 1. Sonda

### 1.1 Teste directo no cristalino

Documento de teste (`/tmp/p661-show-figure.typ`):

```typst
#show figure.where(kind: table): set figure.caption(position: top)

#set table(numbering: "1")
#table(
  columns: 2,
  caption: [Tabela via extensão P459],
  [A], [B],
)

#figure(
  table(columns: 2, [C], [D]),
  caption: [Tabela via figure normal],
)
```

Compilação com o binário release do cristalino:

```bash
./target/release/typst /tmp/p661-show-figure.typ /tmp/p661.pdf
pdftotext /tmp/p661.pdf -
```

Output:

```text
/tmp/p661-show-figure.typ:1:38: warning: set: target '' ainda não suportado
  hint: targets suportados: heading, page, figure, text, par
Table 1: Tabela via extensão P459
AB
CD
Tabela via figure normal
```

Observações:

- O cristalino ainda não suporta `set figure.caption(position: top)` (target vazio).
- A tabela numerada via P459 mantém a caption acima, independentemente da show rule.
- A tabela envolvida em `#figure(...)` também não teve a caption movida (porque o próprio `set figure.caption` falhou).

### 1.2 Testes de isolamento

Para confirmar que a tabela P459 não dispara show rules de `figure`, testaram-se variantes:

#### a) `figure.where(kind: table)` com conteúdo visível

```typst
#show figure.where(kind: table): [FIG-BEFORE]

#set table(numbering: "1")
#table(columns: 2, caption: [Tabela via extensão P459], [A], [B])

#figure(table(columns: 2, [C], [D]), caption: [Tabela via figure normal])
```

Output:

```text
Table 1: Tabela via extensão P459
AB
CD
Tabela via figure normal
```

Resultado: `FIG-BEFORE` não apareceu. A tabela P459 não disparou a show rule.

#### b) Show rule genérica sobre `figure`

```typst
#show figure: [FIG-BEFORE]

#set table(numbering: "1")
#table(columns: 2, caption: [Tabela via extensão P459], [A], [B])

#figure(table(columns: 2, [C], [D]), caption: [Tabela via figure normal])
```

Output:

```text
Table 1: Tabela via extensão P459
AB
FIG-BEFORE
```

Resultado: a show rule genérica de `figure` afectou apenas a segunda tabela (envolvida em `#figure(...)`).

#### c) Show rule sobre `table`

```typst
#show table: [TAB-BEFORE]
```

Resultado: erro de compilação:

```text
error: função 'table' não é um tipo de nó suportado como selector.
Tipos suportados: heading, figure, strong, emph, raw, underline, strike, overline, smallcaps, sub, super, link, quote, list, enum.
```

### 1.3 Código envolvido

- `01_core/src/rules/eval/rules.rs:871-889` — processa `#set table(numbering: ...)`.
- `01_core/src/rules/layout/table.rs` — lê `table.numbering` e renderiza a caption directamente no layout de `Content::Table`.
- `01_core/src/entities/elements/table.rs` — `TableElem` mantém `caption` como campo próprio; não há transformação para `figure`.

---

## 2. Decisão

A extensão P459 implementa a numeração de tabelas **dentro do próprio layout de `table`**, sem converter a tabela numa `figure`. Consequentemente:

- A tabela numerada **não responde** a `#show figure.where(kind: table): ...`.
- A tabela numerada **não responde** a `#show figure: ...`.
- Não é possível estilizá-la através do mecanismo unificado de `figure`.

Esta é uma **limitação conhecida e aceite** da extensão P459. Não é um bug de paridade com o vanilla, porque:

- No vanilla, a numeração/caption de tabelas pertence exclusivamente a `figure`.
- P459 foi introduzido como extensão do cristalino (confirmado em P639) para permitir numerar tabelas sem as envolver em `#figure(...)`.

Não se propõe, neste passo, estender a extensão para disparar show rules de `figure`. Fazer isso exigiria re-arquitecturar `TableElem` como um tipo que também seja reconhecido pelo selector/mechanismo de `figure`, o que está fora do scope de P661. Fica documentado como limitação.

---

## 3. Actualizações documentais

Foram actualizados os Prompts L0 relacionados com P459:

- `00_nucleo/prompts/rules/layout/table.md`:
  - Adicionada secção "§P661 — Limitação: fora do mecanismo `figure`".
  - Actualizado scope-out para mencionar a não integração no mecanismo `figure`.

- `00_nucleo/prompts/rules/eval/table.md`:
  - Adicionada secção "4. Limitação conhecida (P661)".
  - Adicionado item de verificação sobre show rules de `figure.where(kind: table)` não afectarem tabelas P459.

- `00_nucleo/prompts/entities/elements/table.md`:
  - Adicionada nota de que a tabela com `caption` numerada por P459 não responde a show rules de `figure.where(kind: table)`.

Os `@prompt-hash` nos ficheiros de código L1 correspondentes foram recalculados via `crystalline-lint --fix-hashes`:

- `01_core/src/entities/elements/table.rs` → `fe36c643`
- `01_core/src/rules/layout/table.rs` → `d0817664`

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

- [x] Testado directamente no cristalino; resultado confirmado.
- [x] Decisão registada: limitação aceite — tabelas numeradas por P459 ficam fora do mecanismo `figure`.
- [x] Documentação da extensão P459 actualizada nos Prompts L0.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p661.md`.
