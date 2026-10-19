# Passo 1036 — numeração hierárquica de heading, e a premissa do enunciado refutada

**Data**: 2026-08-13
**Estado**: **fechado**. O corpo passa a numerar hierarquicamente com qualquer pattern; a
afirmação do enunciado de que "o outline acerta" foi medida e é falsa fora de `"1."`, com o
achado registado e escalado.

---

## Proveniência

`HEAD = 0c8b64a41` (P1033); árvore de trabalho não commitada, com as alterações deste passo
(10 ficheiros — ver §"O que ficou escrito"). Vanilla `/usr/local/bin/typst`, md5
`36da18895eeb5e0136c068a7634e3f82`, idêntico a `lab/typst-original/target/release/typst`
(baseline ratificado `a51e02804`). Cristalino `target/release/typst` reconstruído após cada
alteração. Medições 17:30–17:50 -03:00.

---

## O que a medição mostrou

O enunciado dava uma tabela de duas linhas (corpo `1./1./1.` vs vanilla `1./1.1./1.1.1.`) e
uma premissa: *"o outline acerta … o mecanismo correcto existe algures no código"*. As duas
metades precisavam de ser testadas com **mais do que um pattern**. Resultado, mesmo
documento (`= / == / === / =`), quatro patterns:

| pattern | sítio | vanilla | cristalino (antes) |
|---|---|---|---|
| `1.` | corpo | `1.` / `1.1.` / `1.1.1.` | `1.` / `1.` / `1.` ❌ |
| `1.` | outline | `1.` / `1.1.` / `1.1.1.` | igual ✅ |
| `I.` | corpo | `I.` / `I.I.` / `I.I.I.` | `I.` / `I.` / `I.` ❌ |
| `I.` | outline | `I.` / `I.I.` / `I.I.I.` | `1.` / `1.1.` / `1.1.1.` ❌ |
| `1.1` | corpo | `1` / `1.1` / `1.1.1` | `1.` / `1.1` / `1.1` ❌ |
| `1.1` | outline | `1` / `1.1` / `1.1.1` | `1.` / `1.1.` / `1.1.1.` ❌ |
| `A.1.a` | corpo | `A` / `A.1` / `A.1.a` | `1.` / `1.1.` / `A.1.a` ❌ |
| `A.1.a` | outline | `A` / `A.1` / `A.1.a` | `1.` / `1.1.` / `1.1.1.` ❌ |

**A premissa do enunciado é falsa.** O outline não "acerta": ignora o pattern por completo.
Coincide com o vanilla só em `"1."`, porque `formatted_counter_at`
(`compiler/introspect/heading.rs:34,56`) junta os níveis com `"."` em arábico e acrescenta
`"."` — o que por acaso *é* `"1."` aplicado. Não havia, portanto, "mecanismo correcto a
copiar do outline". O mecanismo correcto estava noutro sítio: `format_pattern`
(`compiler/stdlib/numbering.rs:100`), que alimenta `#numbering()` e que verifiquei bater com
o vanilla em 9 casos (`1.2.3`, `1`, `1.1.1.`, `A.1.a.a`, `(a(b)`, `II.3`, `1.2`, `A`, `A.2`).

## A causa, em duas metades

Ambas do lado do corpo, e ambas dentro de funções existentes:

1. **`entities/counter_format.rs::format_counter`** iterava o **pattern** e consumia um
   valor por token — logo `"1."` (um token) consumia um só nível, quaisquer que fossem os
   valores. E `values.get(level)?` devolvia `None` quando havia mais tokens do que valores,
   empurrando o caller para um fallback com formatação diferente. A regra em falta
   (*"the last counting symbol with its prefix is repeated"*) **já estava registada como
   lacuna documentada** no próprio L0, por P1031 — este passo fecha-a.
2. **`compiler/layout/heading.rs`** escolhia o separador por uma heurística (`used_pattern`,
   que recomputava `values.len() >= nº de tokens`) e emitia `". "` quando ela falhava. Com
   `"1.1"` e um nível, isso dava `1. Alpha` onde o vanilla dá `1 Alpha`: o pattern já
   transporta a pontuação. A heurística desapareceu — **há pattern → verbatim + `" "`; não
   há pattern → legado + `". "`**.

## Não-regressão

`cargo test --workspace` → **5860 passed; 0 failed**. Um único teste falhou no caminho e
merece registo:

**`f339t_caracterizacao_saida_preservada` caracterizava o defeito.** Fixava `"1. A 2. B"`
para `#set heading(numbering: "1.1")` + `= A` / `= B`. Medido no vanilla: `1A` / `2B` — sem
ponto (o `0.3em` do vanilla é estreito de mais para o `pdftotext` separar palavras). A
expectativa passou a `"1 A 2 B"`, com a medição no comentário. Não é regressão: era o
comportamento errado que estava fixado.

A guarda do outline vale: o outline não partilha o caminho corrigido, logo `"1."` continua
a bater com o vanilla, tal como antes.

## O que ficou escrito

- **`entities/counter_format.md` §P1036** — as duas regras de excedente com a tabela medida;
  débito nomeado: `format_pattern` e `format_counter` são **duas** implementações da mesma
  regra da linguagem, agora concordantes mas ainda duplicadas (unificar exige mover
  `format_pattern` para `entities/` e resolver a sua dependência de `Engine`, usada só para
  os warnings de `א`/`①` — passo próprio).
- **`compiler/layout/heading.md` §P1036** — a regra do separador, o achado escalado do
  outline, e um residual medido (abaixo).
- Código: `entities/counter_format.rs` (reescrita de `format_counter` + 3 testes novos, 11
  no total), `compiler/layout/heading.rs` (heurística removida), `compiler/eval/tests.rs`
  (caracterização corrigida).

## Achados escalados / residuais (medidos, não corrigidos)

1. **O outline ignora o pattern.** Para o corrigir, o pattern tem de chegar ao walk de
   introspecção: `ElementPayload::Heading` transporta `numbering_active: bool` mas não o
   pattern, e o bake da chain (`compiler/introspect.rs:1105-1108`) só lê o `Bool`. Campo
   novo em `entities/element_payload.rs` → contrato público, **gate ADR-0127 ponto 1**,
   passo próprio.
2. **O separador número→título é um espaço literal; no vanilla é `0.3em` fraco.** Medido por
   `pdftotext -bbox`: vanilla 4.62pt a 15.4pt e 3.96pt a 13.2pt (= `0.3 ×` tamanho nos dois
   níveis, o que identifica a regra); cristalino 9.08pt e 10.98pt. **Pré-existente** — o
   caminho de nível 1 com `"1."` é idêntico antes e depois deste passo. É geometria, não
   numeração: passo próprio.

## Validação

```
crystalline-lint .       → 0 erros; 0 V5; 3 avisos V7 pré-existentes (prompts órfãos)
cargo test --workspace   → 5860 passed; 0 failed  (4989 + 789 + 41 + 2 + 37 + 2)
```
