# Paridade de Produção — P600

**Data do relatório:** 2026-07-07
**Passo:** 600
**Foco:** Verificar se `/Producer` no `/Info` do PDF deve ser preenchido.

---

## Resumo executivo

A premissa do passo era que o vanilla 0.15.0 preenche `/Producer` no dicionário `/Info` e que o cristalino o tinha deixado de fora em P536. A sonda directa ao PDF gerado pelo vanilla refuta essa premissa: **o vanilla não escreve `/Producer`**. O campo que o vanilla preenche é `/Creator` com o valor `(Typst 0.15.0)`.

O cristalino já preenche `/Creator` com `(typst-crystalline)` (em `03_infra/src/export/builder.rs:1229`) sempre que o dicionário `/Info` é emitido — isto é, quando o documento tem pelo menos `title`, `author` ou `keywords`.

Conclusão: não há implementação a fazer para `/Producer`. A disparidade documentada era um mal-entendido. As listas de disparidades foram actualizadas para refletir que `/Producer` não é campo usado pelo vanilla e que `/Creator` já está preenchido.

---

## Proveniência

- **Hash base:** `1c6aae8818051c6ac9e9efcc0e6714e54d42bdf5`
- **Data/hora:** 2026-07-07T20:34:13-03:00
- **Binários usados:**
  - Cristalino: `./target/release/typst`
  - Vanilla 0.15.0: `lab/typst-original/target/release/typst compile`
- **Ferramentas auxiliares:** `mutool show`, `strings`

---

## Sonda

### Documento sem metadados

```typst
Texto de teste.
```

**Vanilla 0.15.0** — objecto `/Info`:

```text
17 0 obj
<<
  /Creator (Typst 0.15.0)
  /ModDate (D:20260707204412-03'00)
  /CreationDate (D:20260707204412-03'00)
>>
endobj
```

**Cristalino** — não emite `/Info` para documentos sem metadados (comportamento actual de `emit_info`).

### Documento com metadados

```typst
#set document(title: "Documento de Teste", author: "Autor Teste")
Texto.
```

**Vanilla 0.15.0** — objecto `/Info`:

```text
17 0 obj
<<
  /Title (Documento de Teste)
  /Author (Autor Teste)
  /Creator (Typst 0.15.0)
  /ModDate (D:20260707204557-03'00)
  /CreationDate (D:20260707204557-03'00)
>>
endobj
```

**Cristalino** — objecto `/Info`:

```text
10 0 obj
<<
  /Title <FEFF...>
  /Author <FEFF...>
  /CreationDate (D:20260707234627)
  /Creator (typst-crystalline)
>>
endobj
```

Em ambos os casos, **não existe `/Producer`** no PDF do vanilla. O campo semanticamente equivalente é `/Creator`, que o cristalino já preenche.

### Busca por `/Producer` nos bytes

```bash
strings /tmp/p600-vanilla.pdf | grep -i producer
# (nenhum resultado)

strings /tmp/p600-cristalino.pdf | grep -i producer
# (nenhum resultado)
```

---

## Decisão

Não se implementa `/Producer`. A premissa do passo não se sustenta face à medição directa do vanilla 0.15.0. O campo usado pelo vanilla para identificar o produtor é `/Creator`, e o cristalino já o preenche com `(typst-crystalline)`.

**Observação consciente:** o cristalino só emite `/Info` quando o documento tem `title`, `author` ou `keywords`. O vanilla emite `/Info` sempre, mesmo sem metadados do utilizador. Esta é uma disparidade menor, mas não é o foco de P600 e não foi corrigida neste passo.

---

## Actualização das listas de disparidades

- `00_nucleo/diagnosticos/inventario-decisoes-pendentes.md`:
  - `/Producer` em `/Info`: estado alterado de **Aberto (scope-out)** para **Fechado (P600)**.
  - Secção 2.3: `/Producer` marcado como fechado em P600.

- `00_nucleo/diagnosticos/estado-disparidades-vanilla-p593.md`:
  - `/Producer` removido da secção "Scope-out deliberado".
  - `/Producer` adicionado à secção "Corrigido ao longo desta conversa" com a razão de P600.

---

## Validação

- `cargo test --workspace` → 0 falhas.
- `crystalline-lint .` → `✓ No violations found`.

---

## Critérios de fecho do passo

- [x] Valor exacto do vanilla confirmado por leitura directa do `/Info`.
- [x] Confirmado que o cristalino não escreve `/Producer` (porque o vanilla também não o faz).
- [x] Confirmado que o cristalino já preenche `/Creator`.
- [x] Listas de disparidades actualizadas.
- [x] Relatório escrito com hash do commit e proveniência.

---

## Ligações

- `00_nucleo/materialization/typst-passo-600.md` — passo que originou esta sonda.
- `00_nucleo/diagnosticos/paridade-producao-p536.md` — relatório P536 onde `/Producer` foi registado incorrectamente.
- `00_nucleo/diagnosticos/inventario-decisoes-pendentes.md` — lista de decisões actualizada.
- `00_nucleo/diagnosticos/estado-disparidades-vanilla-p593.md` — estado das disparidades actualizado.
- `03_infra/src/export/builder.rs:1229` — preenchimento de `/Creator` no cristalino.
