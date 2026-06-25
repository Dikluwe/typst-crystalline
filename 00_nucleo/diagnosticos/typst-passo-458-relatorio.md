# Relatório — Passo 458

> **Passo:** 458 — Sonda DEBT-2: Verificação de premissa + consolidação do oráculo de introspecção  
> **Data:** 2026-06-25  
> **Executor:** IA (Kimi Code CLI)  
> **Tipo:** Sonda / Investigação / Decisão arquitectural  

---

## 1. Objetivo

Verificar empiricamente a premissa do **DEBT-2**: que o Typst vanilla usa
*lazy capture* de closures, ou seja, que
`#let x=1; #let f()=x; #let x=2; f()` devolveria `2` no vanilla vs `1` no
cristalino. Se a premissa se confirmasse, o passo deveria consolidar o plano de
oráculo de introspecção; se fosse refutada, o DEBT-2 deveria ser encerrado sem
código.

## 2. Metodologia

Compilar o mesmo snippet minimal em ambos os compiladores, renderizar para PDF
e extrair o texto renderizado com `pdftotext`.

### 2.1 Snippet de teste

```typst
#let x = 1
#let f() = x
#let x = 2
#repr(f())
```

### 2.2 Ambientes

| Ambiente | Binário | Versão / origem |
|----------|---------|-----------------|
| Typst vanilla (referência do projecto) | `lab/typst-original/target/debug/typst` | Workspace `lab/typst-original` — versão tipada nos manifests do projecto |
| Cristalino | `target/debug/typst` | Workspace cristalino, branch `main` actual |

### 2.3 Comandos executados

```bash
# Vanilla
lab/typst-original/target/debug/typst compile /tmp/p458/test_vanilla_full.typ /tmp/p458/out.pdf
pdftotext /tmp/p458/out.pdf -

# Cristalino
target/debug/typst /tmp/p458/test_vanilla_full.typ /tmp/p458/out_crist.pdf
pdftotext /tmp/p458/out_crist.pdf -
```

## 3. Resultados

| Ambiente | Saída de `pdftotext` | Interpretação |
|----------|----------------------|---------------|
| Typst vanilla | `1` | Eager capture (snapshot do scope na definição de `f`) |
| Cristalino | `1` | Eager capture (snapshot do scope na definição de `f`) |

## 4. Decisão: Caso 1

A premissa "vanilla é lazy" é **falsa**. Ambos os compiladores são eager para
esta construção. Não existe divergência semântica entre cristalino e vanilla no
DEBT-2.

Consequências:

- **DEBT-2 é encerrado sem código.**
- A infraestrutura `comemo`/`TrackedWorld` **não** é necessária para fechar este
débito.
- O oráculo de introspecção (duas passagens) continua a ser necessário para
outras funcionalidades — TOC, `ref`, bibliography Fase 2, counters — mas essas
já têm os seus próprios rastreadores no roteiro.

## 5. Documentos actualizados

- `00_nucleo/diagnosticos/debt/DEBT.md` — secção DEBT-2 reclassificada como
  **ENCERRADO (Passo 458)** com a medição e a lição arquitectural.
- `00_nucleo/roteiro-conclusao-typst-cristalino.md` — Trilha 9 marcada como
  encerrada; removido o bloqueio fictício a outras trilhas.

## 6. Validação

### 6.1 Testes de closure no cristalino

```bash
cargo test -p typst-core --lib eager_capture
```

Resultado: `test rules::eval::tests::tests::eager_capture_isolada_do_scope_pai ... ok`

### 6.2 Linter

```bash
crystalline-lint .
```

Resultado: zero violações. Apenas warnings preexistentes V7 (prompts órfãos não
referenciados por código L1–L4):

- `00_nucleo/prompts/adr/adr-stub-vs-fallback.md`
- `00_nucleo/prompts/rules/show-regex.md`

### 6.3 `cargo test --workspace`

O comando completo falha num teste **preexistente e não relacionado** ao P458:

```
thread 'rules::eval::tests::tests::p350c_flag_on_nao_convergente_classifica' has overflowed its stack
fatal runtime error: stack overflow, aborting
```

Este stack overflow já ocorria antes das alterações deste passo (nenhum ficheiro
Rust foi modificado). Fica registado como falha preexistente a tratar fora do
escopo de P458.

## 7. Lição central

> **Nunca construir infraestrutura grande sobre uma suposição não medida.**

O DEBT-2 carregava a suposição de que o vanilla era lazy, o que levaria a uma
infraestrutura XL (`comemo`/`TrackedWorld`). A medição directa refutou a
suposição e mostrou que não havia débito semântico a corrigir. O fecho foi
administrativo, não técnico.

---

**Fim do relatório.**
