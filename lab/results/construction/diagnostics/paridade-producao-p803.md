# Relatório de Verificação — Passo 803: `visualize::curve` — mensagem de erro da API diverge (achado P798 #8)

**Data:** 2026-07-21
**Status:** Concluído com Sucesso
**Proveniência da Medição:**
- **Commit Base:** `0661aef91` (HEAD)
- **Working tree na sonda "antes":** P799–P802 (zonas não relacionadas)
- **Working tree na validação "depois":** P799–P803
- **Hora da Medição:** 2026-07-21 ~16:35 (-0300)
- **Nota de localização:** este é o relatório canónico do passo (convenção: relatórios vivem em `00_nucleo/diagnosticos/`). O duplicado em `materialization/` foi removido por essa convenção.

---

## 1. O Problema Relatado

Achado #8 de P798: `#curve((0pt, 0pt), (10pt, 10pt))` — vanilla `error: expected content, found array` (×2, um por argumento); cristalino `error: curve(): segmento 0: primeiro elemento deve ser string (kind)`.

## 2. Diagnóstico e Medição

Vanilla: os argumentos posicionais de `curve` são componentes (content de `curve.move`/etc.); um array bare falha a validação **genérica** de tipo (`expected X, found Y`). Cristalino: `native_curve` (`01_core/src/engine/stdlib/shapes.rs`) assumia que todo argumento não-`Content::Curve` era tuplo legado (`("move", ...)`) e emitia mensagens próprias.

**Causa local ou genérica?** Local a `curve` — a validação genérica do projecto já usa o padrão `expected X, found {type_name()}` noutros pontos (`foundations.rs:102`, `shapes.rs:478`); era `native_curve` que a contornava. Correcção em `native_curve`, não na origem comum.

## 3. A Solução Implementada

L0 `stdlib/curve.md` (nota P803 na secção Consumo); lint sem hashes pendentes, exit 0. Guarda do tuplo legado passa a exigir array não-vazio **com primeiro elemento string**; tudo o resto → `expected content, found {val.type_name()}`. Erros internos do tuplo legado (aridade, coordenada, kind desconhecido) inalterados — testes P293/P294 existentes passam.

Validação depois: `...8_curve.typ:1:6: error: expected content, found array` == mensagem do vanilla ✓. Observação registada (fora de âmbito): o vanilla emite 2 erros (acumula diagnósticos por argumento); o cristalino pára no primeiro `Err`.

## 4. Testes Automatizados Persistidos (com nomeação explícita)

- `p803_curve_arg_nao_content_erro_paridade_vanilla` (novo): array bare de lengths → "expected content, found array"; controlo `Value::Int` → "expected content, found int". Falhou antes em `stdlib/mod.rs:5140`.

## 5. Verificação de Sucesso do Workspace

```
Suite 'typst-core' (lib):  ANTES 4323 passed; 1 ignored → DEPOIS 4324 passed; 1 ignored (total 4325 = +1 ✓)
crystalline-lint . → exit 0
```

Verificação final do workspace completo (2026-07-21 ~18:00): typst-core 4336/1i, typst-infra 657/5i, typst-shell 33, CLI bin 2, cli.rs 29, crystalline_lint 2 — zero falhas.
