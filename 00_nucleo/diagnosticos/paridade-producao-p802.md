# Relatório de Verificação — Passo 802: `utils::listset` — warning de label não-anexada (achado P798 #5)

**Data:** 2026-07-21
**Status:** Concluído com Sucesso
**Proveniência da Medição:**
- **Commit Base:** `0661aef91` (HEAD)
- **Working tree na sonda "antes":** P799–P801 (zonas não relacionadas)
- **Working tree na validação "depois":** P799–P802
- **Hora da Medição:** 2026-07-21 ~16:20 (-0300)
- **Nota de localização:** este é o relatório canónico do passo (convenção: relatórios vivem em `00_nucleo/diagnosticos/`). O duplicado em `materialization/` foi removido por essa convenção.

---

## 1. O Problema Relatado

Achado #5 de P798: `<abc> Hello #context query(<abc>)` — o vanilla emite `warning: label `<abc>` is not attached to anything`; o cristalino não emite nenhum warning.

## 2. Diagnóstico e Medição

Sonda com a fonte original `temp/p798/5_listset.typ`: cristalino stderr vazio; vanilla com o warning (span da label). Mecanismo vanilla localizado em `crates/typst-eval/src/markup.rs:52-72`: ao avaliar `Value::Label` em markup, procura o último nó anexável na sequência; não havendo, avisa. Equivalente cristalino: braço `SyntaxKind::Label` de `eval_markup` (`eval/mod.rs`) — `parts.pop()` falhava e a label era ignorada **em silêncio** (e os espaços recolhidos perdiam-se).

Medições de enquadramento: `Hello <abc>` (label depois de texto) — vanilla **não avisa** e `query` = 1; cristalino não avisa mas query = 0 (divergência separada já registada em P791 §6 — label em nó de texto não indexado pelo introspector — **fora do âmbito**). `= Title <abc>`: ambos sem warning, query = 1.

## 3. A Solução Implementada

L0 `eval.md` (nova bullet "Warning de label órfã (P802)"); hashes corrigidos (`eval/mod.rs`, `eval/tests.rs` → `3f09960d`). No braço `SyntaxKind::Label`, caminho sem alvo: `engine.sink.warn_note(child.span(), "label `<{name}>` is not attached to anything", "")`; a label é descartada (como no vanilla) e os espaços recolhidos são re-inseridos. Formato de diagnóstico: o do projecto (uma linha `path:line:col: warning:`), conforme o passo. O warning vanilla "content labelled multiple times" ficou registado no L0 como fora de âmbito.

Validação depois: cristalino emite `...5_listset.typ:1:0: warning: label `<abc>` is not attached to anything` (mesmo texto do vanilla); controlo `Hello <abc>` sem warning ✓.

## 4. Testes Automatizados Persistidos (com nomeação explícita)

- `label_orfa_emite_warning` (novo): falhou antes em `tests.rs:5195`.
- `label_anexada_nao_emite_warning_orfa` (novo): controlo `Hello <abc>` e `= Título <abc>`.
- Helper novo `eval_for_test_keep_sink` (devolve o `Sink` para asserções sobre warnings).

## 5. Verificação de Sucesso do Workspace

```
Suite 'typst-core' (lib):  ANTES 4321 passed; 1 ignored → DEPOIS 4323 passed; 1 ignored (total 4324 = +2 ✓)
crystalline-lint . → exit 0
```

Verificação final do workspace completo (2026-07-21 ~18:00): typst-core 4336/1i, typst-infra 657/5i, typst-shell 33, CLI bin 2, cli.rs 29, crystalline_lint 2 — zero falhas.
