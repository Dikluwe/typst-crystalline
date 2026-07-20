# Prompt L0 — `engine/eval/fields` — Acesso a Campos Nativos e Diagnósticos de Campo
Hash do Código: 3624668a

**Camada**: L1 · **Alvo**: `01_core/src/engine/eval/bindings.rs`
**Origem**: P785b (campos nativos em `RelativeLength`, `Alignment`, `Length`, `Stroke`).
**ADRs Aplicadas**: ADR-0107 (paridade de semântica com a linguagem), ADR-0108 (disciplina anti-deriva).

---

## 1. Contexto e Objetivo

O Typst Vanilla expõe campos nativos pré-definidos em diversos tipos de valor (`Length`, `RelativeLength`, `Alignment`, `Stroke`, `Version`), além de dicionários (`Dict`) e elementos (`Content`).

Este módulo L1 implementa a avaliação de `eval_field_access` para todos os tipos com suporte a campos nativos e padroniza as mensagens de erro para campos ausentes ou tipos sem suporte a campos.

---

## 2. Tabela de Campos por Tipo de Valor

| Tipo | Campo | Tipo de Retorno | Exemplo / Comportamento |
|---|---|---|---|
| `RelativeLength` (`Value::Relative`) | `"ratio"` | `Value::Ratio` | `(10pt + 50%).ratio` $\to$ `50%` |
| `RelativeLength` (`Value::Relative`) | `"length"` | `Value::Length` | `(10pt + 50%).length` $\to$ `10pt` |
| `Alignment` (`Value::Align`) | `"x"` | `Value::Align` ou `Value::None` | `(top + left).x` $\to$ `left`, `top.x` $\to$ `none` |
| `Alignment` (`Value::Align`) | `"y"` | `Value::Align` ou `Value::None` | `(top + left).y` $\to$ `top`, `left.y` $\to$ `none` |
| `Length` (`Value::Length`) | `"em"` | `Value::Float` | `(10pt + 2em).em` $\to$ `2.0` |
| `Length` (`Value::Length`) | `"abs"` | `Value::Length` | `(10pt + 2em).abs` $\to$ `10pt` |
| `Stroke` (`Value::Stroke`) | `"paint"` | `Value::Color` / `Value::Paint` | `stroke(red).paint` $\to$ `rgb("#ff4136")` |
| `Stroke` (`Value::Stroke`) | `"thickness"` | `Value::Length` | `stroke(2pt).thickness` $\to$ `2pt` |
| `Stroke` (`Value::Stroke`) | `"cap"`, `"join"`, `"dash"`, `"miter-limit"` | Atributos do stroke | Devolve o atributo correspondente |

---

## 3. Mensagens de Diagnóstico de Erro

1. **Campo inexistente em tipo com suporte a campos**:
   - Formato: `"<tipo> does not contain field \"<nome>\""`
   - Exemplo: `length does not contain field "invalid"`
   - Exemplo: `relative length does not contain field "invalid"`
   - Exemplo: `alignment does not contain field "invalid"`
   - Exemplo: `stroke does not contain field "invalid"`

2. **Chave inexistente em dicionário**:
   - Formato: `"dictionary does not contain key \"<nome>\""`

3. **Campo inexistente em elemento de conteúdo**:
   - Formato: `"<nome_do_elemento> does not have field \"<nome>\""`

4. **Acesso a campo em tipo que não possui campos**:
   - Formato: `"cannot access fields on type <tipo>"`
   - Exemplo: `cannot access fields on type integer`
   - Exemplo: `cannot access fields on type string`
