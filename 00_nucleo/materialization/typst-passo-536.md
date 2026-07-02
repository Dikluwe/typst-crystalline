---
# P536 — Metadados do documento (`#set document(title:, author:)`)

> **Passo:** 536
> **Data:** 2026-07-02
> **Foco:** P531 Grupo 4.4 confirmou que `#set document(title: ..., author: ...)` produz um aviso ("target 'document' ainda não suportado") e o PDF final não tem Title nem Author. O vanilla escreve estes campos no dicionário `/Info` do PDF, e também numa stream de metadados XMP.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — sonda antes de qualquer edição.
> **Dependências:** P531 (Grupo 4.4). Não depende de P534 nem P535 — área isolada, só exportação.

---

## Contexto

`#set document(...)` é a forma de o utilizador definir informação sobre o documento inteiro — título, autor, palavras-chave, data. O cristalino reconhece a sintaxe mas emite aviso de "não suportado" e descarta a informação. Nada disto chega ao PDF final.

O vanilla escreve esta informação em dois sítios do PDF, que não são a mesma coisa:

1. **`/Info`** — dicionário simples, antigo, com campos como `/Title`, `/Author`, `/CreationDate`. Todos os leitores de PDF lêem isto.
2. **Stream de metadados XMP** — formato baseado em XML, mais recente, usado por ferramentas de arquivo e catalogação. Contém a mesma informação, escrita de outra forma, e pode ter campos adicionais.

---

## Sonda

```bash
grep -rn "\"document\"\|set.*document\|SetDocument" 01_core/src/rules/eval/rules.rs 01_core/src/entities/content.rs --include="*.rs"
grep -rn "/Info\|DocumentInfo\|/Title\|/Author\|XMP" 03_infra/src/export/ --include="*.rs"
```

Perguntas, com `file:line`:

1. Onde exactamente é gerado o aviso "target 'document' ainda não suportado"? Isso confirma o ponto exacto onde a propriedade é descartada.
2. Existe algum `Content::SetDocument` ou equivalente, ou a sintaxe é reconhecida e ignorada de imediato?
3. O `/Info` do PDF já é escrito com algum valor por defeito (mesmo que vazio), ou não existe de todo?

### Critério de fecho da sonda

- [ ] Localizado o ponto exacto onde `#set document(...)` é descartado.
- [ ] Confirmado se `/Info` já existe no export com valores por defeito.

---

## Implementação

Seguir o mesmo padrão já usado em P532 para `#set page(numbering:)`: adicionar os campos a uma estrutura de conteúdo (`Content::SetDocument` ou equivalente), transportar essa informação da avaliação até ao layout/pipeline, e escrevê-la no export.

### Sub-tarefa A — `/Info`

Campos mínimos: `/Title`, `/Author`. Vanilla também escreve `/CreationDate` e `/Creator` — confirmar se o cristalino já tem acesso a uma data (relógio do sistema no momento da compilação) antes de decidir se inclui isto agora ou deixa para depois.

### Sub-tarefa B — XMP

Decidir, depois da sonda, se a stream XMP entra neste passo ou fica para um passo seguinte. Critério: se `/Info` já resolve o caso de uso comum (a maioria dos leitores só olha para `/Info`), a stream XMP pode ficar como scope-out registado, não escondido, para não aumentar o tamanho deste passo sem necessidade.

### Critério de fecho da implementação

- [ ] `#set document(title: ..., author: ...)` deixa de emitir aviso de "não suportado".
- [ ] `/Info` no PDF final tem `/Title` e `/Author` correctos.
- [ ] Decisão sobre XMP registada (incluído neste passo, ou scope-out explícito para depois).

---

## Validação

```bash
cat > /tmp/test-metadata.typ <<'EOF'
#set document(title: "Documento de Teste", author: "Autor Teste")
Conteúdo do documento.
EOF
./target/release/typst /tmp/test-metadata.typ /tmp/metadata.pdf
pdfinfo /tmp/metadata.pdf
```

Esperado: `Title: Documento de Teste` e `Author: Autor Teste` na saída de `pdfinfo`.

Comparar com vanilla 0.15.0 para o mesmo documento.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa antes de código.
- [ ] `/Info` com Title e Author correctos, confirmado com `pdfinfo`.
- [ ] Decisão sobre XMP registada, não escondida.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p536.md`.

---

## Próximo passo

P537 — Notas de rodapé em layout de duas colunas. Último dos seis itens da sequência antes do passo de confirmação (P538).
