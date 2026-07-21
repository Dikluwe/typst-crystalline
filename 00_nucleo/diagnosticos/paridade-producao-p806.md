# Relatório de Verificação — Passo 806: `model` — `#par[...]` como função (achado P798 #12)

**Data:** 2026-07-21
**Status:** Concluído com Sucesso
**Proveniência da Medição:**
- **Commit Base:** `0661aef91` (HEAD)
- **Working tree na sonda "antes":** P799–P805a (zonas não relacionadas)
- **Working tree na validação "depois":** P799–P806
- **Hora da Medição:** 2026-07-21 ~17:40 (-0300)
- **Relatório de materialização:** `00_nucleo/materialization/typst-passo-806-relatorio.md`

---

## 1. O Problema Relatado

Achado #12 de P798: `#par[conteúdo de teste]` — cristalino `error: unknown variable: par`; vanilla aceita e renderiza.

## 2. Diagnóstico e Medição

Vanilla (`crates/typst-library/src/model/par.rs`): `ParElem` invocável com `leading`, `spacing`, `justify`, `justification-limits`, `linebreaks`, `first-line-indent`, `hanging-indent` e `body` posicional obrigatório. Medições de erro vanilla: `#par()` → `missing argument: body`; `#par(5)` → `expected content, found integer`.

Cristalino: `Content::Par` **não existia** — parágrafos são implícitos (texto plano em `Sequence`; `element_kind.rs:63`). `#set par(leading:)` já existia (canal custom `"par.leading"`, F-5b/P373). O nome `par` não estava registado na `Scope` global — daí o `unknown variable`. **Nota pedida pelo passo**: `Content::Par` teve de ser criada? **Não** — a arquitectura de parágrafos implícitos torna o body-devolvido-directamente equivalente no caso standalone; `Content::Par` fica candidato futuro apenas para o caso mid-paragraph (block-level break do vanilla), registado no L0.

## 3. A Solução Implementada

L0 `stdlib/structural.md` (nova secção `native_par(body, leading:?)` — P806). Nova `native_par` em `structural.rs`: body `Content`/`Str`; `missing argument: body` (literal vanilla medido); `expected content, found {type_name()}`; `leading:` Length → `Content::Styled` com custom `"par.leading"` (mesmo canal do `#set par`); `justify`/`spacing`/`linebreaks`/`first-line-indent`/`hanging-indent`/`justification-limits` aceites e ignoradas (funções nativas não têm `Sink` — limitação registada); named desconhecido → erro. Registo em `make_stdlib` (`eval/mod.rs`) e re-export (`stdlib/mod.rs`).

**Nota de processo (disciplina):** a implementação precedeu os testes unitários por minutos (inversão); a prova "antes" E2E já existia (`unknown variable: par`, medido na sonda) e os 7 testes cobrem a função nova. Registado.

Validação depois: `#par[conteúdo de teste]` → `conteúdo de teste` == vanilla ✓; `#par(justify: true)[...]` mid-document == vanilla ✓; `#par()` → `missing argument: body` ✓; `#par(5)` → `expected content, found int` (vanilla diz "integer" — divergência pré-existente de `type_name()`, registada). Erros saem com local `<detached>` (convenção actual das nativas) — registado.

## 4. Testes Automatizados Persistidos (com nomeação explícita)

7 novos em `structural.rs`: `p806_par_body_content_devolvido_directamente`, `p806_par_body_str_convertida`, `p806_par_leading_embrulha_styled_custom`, `p806_par_justify_aceite_e_ignorado`, `p806_par_sem_body_erro_literal_vanilla`, `p806_par_tipo_errado_erro_expected_content`, `p806_par_named_desconhecido_erro`.

## 5. Verificação de Sucesso do Workspace

```
Suite 'typst-core' (lib):  ANTES 4329 passed; 1 ignored → DEPOIS 4336 passed; 1 ignored (total 4337 = +7 ✓)
crystalline-lint . → exit 0
```

Verificação final do workspace completo (2026-07-21 ~18:00): typst-core 4336/1i, typst-infra 657/5i, typst-shell 33, CLI bin 2, cli.rs 29, crystalline_lint 2 — zero falhas.
