# Relatório de Paridade — P663

**Passo:** 663  
**Data:** 2026-07-09  
**Foco:** Auditoria por divergências de linguagem introduzidas sem confirmação contra o vanilla.  
**Dependências:** P660/P662 (onde o problema foi descoberto e corrigido), P459/P639 (extensão `table.numbering`), P617 (`--document-id`).  
**Hash do commit com as alterações:** `PENDING`

---

## 1. Contexto

P660 implementou `font: (family: "...", variant: (eixo: valor))` e P662 reverteu-a porque o vanilla de referência rejeita essa sintaxe. A distinção central é:

- **Diferença de implementação** (aceitável): caches, algoritmos, estruturas de dados, passos internos — tudo o que não altera o conjunto de documentos `.typ` válidos.
- **Diferença de linguagem** (inaceitável): sintaxe ou semântica que o cristalino aceita e o vanilla rejeita, ou vice-versa. Quebra a portabilidade dos documentos.

P663 audita os passos da conversa à procura de outras diferenças de linguagem introduzidas sem confirmação directa contra o vanilla.

---

## 2. Método

1. Varreu-se `00_nucleo/diagnosticos/*.md` por menções a "extensão", "capacidade nova", "além do vanilla", "não é paridade".
2. Para cada funcionalidade encontrada, verificou-se se a confirmação contra o vanilla foi feita antes ou depois da implementação.
3. Para os itens não confirmados, testou-se directamente contra o binário vanilla de referência (`lab/typst-original/target/release/typst`).
4. Classificou-se cada item conforme a tabela abaixo.

---

## 3. Tabela de funcionalidades novas

| Item | Passo | Confirmado contra vanilla antes de implementar? | Classificação | Decisão |
|---|---|---|---|---|
| `table.numbering` e `caption` em `table` | P459 (original), P639 (confirmação) | **Não** — P639 confirmou depois que o vanilla não tem `numbering` nem `caption` em `table`. | Extensão de linguagem consciente (confirmada posteriormente) | **Manter**. O efeito só é observável quando `caption` está presente; documentada como extensão cristalina. |
| `--document-id` / `CRYSTALLINE_DOCUMENT_ID` | P617 | **Sim** — P617 verificou explicitamente que o vanilla não tem flag equivalente e decidiu introduzi-la como capacidade de CLI. | Capacidade nova de CLI (não entra no documento `.typ`) | **Manter**. Não é diferença de linguagem; é uma bandeira de invocação. |
| `variant: (eixo: valor)` em dicionário de fonte | P660 | **Não** — P660 confirmou depois que o vanilla rejeita (`unexpected key 'variant'`). | Extensão de linguagem por erro | **Revertida em P662**. |

### 3.1 Reconfirmação directa no vanilla

`table.numbering`:

```text
error: unexpected argument: numbering
```

`table(caption: ...)`:

```text
error: unexpected argument: caption
```

`font: (..., variant: (...))`:

```text
error: unexpected key 'variant', in dict</text>
```

### 3.2 Itens revistos e descartados

- **P598 (`margin_is_auto`)**: extensão interna de representação para preservar margens do utilizador; não expõe sintaxe nova.
- **P614 (RTL)**: reescrita do eixo de referência do motor de texto; não altera a linguagem.
- **P638/P650 (grid `unwrap_or_default`)**: melhoria de diagnóstico além do vanilla; não introduz sintaxe nova.

Nenhum outro caso de extensão de linguagem foi encontrado nos diagnósticos revistos.

---

## 4. Decisões

### 4.1 `table.numbering` (P459)

- Mantém-se como extensão cristalina consciente.
- Os Prompts L0 e relatórios de P633/P636/P639/P661 já documentam claramente que não existe no vanilla.
- Recomenda-se manter um aviso nos documentos de utilizador: documentos que usem `#set table(numbering: ...)` ou `table(caption: ...)` não são portáveis para Typst real.

### 4.2 `--document-id` (P617)

- Mantém-se como capacidade nova de CLI.
- Não afecta a linguagem Typst; afecta apenas metadados XMP do PDF gerado.

### 4.3 `variant: (eixo: valor)` (P660)

- Já revertida em P662.
- A nota póstuma em `00_nucleo/diagnosticos/paridade-producao-p660.md` deixa registado que a sintaxe foi introduzida por erro de verificação.

---

## 5. Recomendação formal

A distinção implementação vs. linguagem devia ser explícita na disciplina de verificação do projecto. Recomenda-se adicionar uma regra nova em `00_nucleo/adr/` ou `AGENTS.md` com o seguinte conteúdo mínimo:

> Antes de introduzir sintaxe ou semântica nova num documento Typst, confirmar com o binário vanilla de referência que a construção existe e comporta-se da mesma forma. Se o vanilla não a suportar, a introdução só pode proceder como **decisão consciente de extensão de linguagem** registada por escrito; nunca por omissão.

A presente auditoria não escreve essa regra — fica como próximo passo sugerido.

---

## 6. Estado de fecho

- [x] Todos os relatórios de diagnóstico revistos quanto a extensões/capacidades novas.
- [x] Tabela completa com confirmação contra vanilla e classificação.
- [x] Itens não confirmados testados directamente no vanilla.
- [x] Caso de erro (`variant`) já revertido em P662; não há novas reversões a propor.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p663.md`.
