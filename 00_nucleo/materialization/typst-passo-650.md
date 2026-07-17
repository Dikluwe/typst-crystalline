---
# P650 — Segunda ronda de auditoria de falhas silenciosas

> **Passo:** 650
> **Data:** 2026-07-09
> **Foco:** P633 varreu seis padrões específicos (`.ok()`, wildcard arms, `let _ =`, `unwrap_or_default`/`unwrap_or_else`, `if let Ok(...)`, `fn -> Option<T>`) e encontrou 23 falhas reais. Este passo varre padrões diferentes, não sobrepostos, e confirma se algum directório ou tipo de código ficou fora da primeira varredura.
> **Tipo:** Sonda directa, ampla. Sem implementação neste passo.
> **Tamanho:** L.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P633 (padrões e método já estabelecidos), P638 (extensão ao vanilla, mesmo método).

---

## Parte 1 — Confirmar cobertura da primeira ronda

### Directórios e tipos de ficheiro

P633 excluiu explicitamente `tests/`, `benches/`, e fixtures — razoável, mas confirmar se ficou mais alguma coisa de fora.

```bash
find 01_core/src 03_infra/src 02_shell/src 04_wiring/src -name "*.rs" | wc -l
find 01_core/src 03_infra/src 02_shell/src 04_wiring/src -name "*.rs" -newer 00_nucleo/diagnosticos/paridade-producao-p633.md | wc -l
```

O segundo comando confirma quantos ficheiros foram alterados ou criados **depois** de P633 — todo o código escrito desde P634 (o próprio mecanismo de `FlowEvent`, `layout_errors`, `SyntaxErrorKind`, etc.) nunca passou pela auditoria original, porque não existia ainda.

### Critério de fecho da Parte 1

- [ ] Confirmado quantos ficheiros/linhas foram adicionados desde P633.
- [ ] Esses ficheiros novos incluídos na varredura da Parte 2, não assumidos como seguros só por terem sido escritos com mais cuidado.

---

## Parte 2 — Padrões novos, não cobertos por P633

### 7. `unwrap_or(valor)` genérico (não só `_default`/`_else`)

```bash
grep -rn "\.unwrap_or(" 01_core/src/ 03_infra/src/ --include="*.rs" | grep -v "test" | grep -v "unwrap_or_default\|unwrap_or_else"
```

Um valor de recurso fixo, diferente do default do tipo, também pode esconder um problema — por exemplo, `.unwrap_or(0)` quando `0` é um valor com significado, não neutro.

### 8. `saturating_*` e `clamp` sem aviso

```bash
grep -rn "saturating_sub\|saturating_add\|\.clamp(" 01_core/src/ 03_infra/src/ --include="*.rs" | grep -v "test"
```

Truncar ou limitar um valor silenciosamente, quando o valor original estava fora do esperado, pode esconder um erro do utilizador (por exemplo, um valor negativo que devia ter sido rejeitado, não ajustado).

### 9. `#[allow(dead_code)]` e código morto marcado

```bash
grep -rn "#\[allow(dead_code)\]" 01_core/src/ 03_infra/src/ --include="*.rs"
```

Código morto marcado como tal pode ser inofensivo, ou pode ser um sinal de uma funcionalidade a meio, nunca ligada ao resto — o mesmo tipo de situação já encontrada em P640 (`eval_counter_method`).

### 10. `Vec::retain`/`filter` que descartam elementos sem contagem nem aviso

```bash
grep -rn "\.retain(\|\.filter(" 01_core/src/rules/ 03_infra/src/export/ --include="*.rs" | grep -v "test"
```

Cada uso confirma se os elementos descartados são genuinamente irrelevantes, ou se representam informação do utilizador a desaparecer (o mesmo padrão do caso de bibliografia, P644, mas potencialmente noutros sítios).

### 11. Mensagens só em `eprintln!`/logging, nunca no `Sink` do utilizador

```bash
grep -rn "eprintln!\|log::warn!\|log::error!" 01_core/src/ 03_infra/src/ --include="*.rs" | grep -v "test"
```

Um aviso que só aparece no terminal de quem desenvolve o compilador, não no diagnóstico que o utilizador final vê, é uma falha silenciosa do ponto de vista de quem usa o Typst normalmente.

### 12. `panic!`/`unreachable!()` em código alcançável

```bash
grep -rn "unreachable!()\|panic!(" 01_core/src/rules/ 03_infra/src/ --include="*.rs" | grep -v "test"
```

Não é o mesmo tipo de falha silenciosa (um `panic!` não é silencioso, trava o programa) — mas um `unreachable!()` que na verdade é alcançável é uma suposição errada sobre o código, e vale a pena confirmar se cada um destes é mesmo inalcançável, ou se pode ser accionado por um documento do utilizador, produzindo um crash em vez de um erro tratado.

### Critério de fecho da Parte 2

- [ ] Os seis padrões novos varridos no código de produção completo, incluindo ficheiros escritos desde P633.
- [ ] Cada ocorrência classificada em Inofensivo/Suspeito/Confirmado, com a mesma disciplina de P633.
- [ ] Casos suspeitos testados directamente, não deixados como suposição.

---

## Decisão

Mesma estrutura de P633: este passo não corrige nada, produz uma lista priorizada. Cada item "Confirmado" vira o seu próprio passo de correcção, com sonda-causa-correcção, não remendado às pressas aqui.

---

## Critério de fecho do passo

- [ ] Parte 1: cobertura confirmada, ficheiros novos desde P633 incluídos.
- [ ] Parte 2: seis padrões novos varridos, classificados.
- [ ] Casos suspeitos testados.
- [ ] Lista final priorizada.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p650.md`, com a lista completa.
