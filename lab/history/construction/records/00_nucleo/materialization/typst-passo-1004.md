# Passo 1004 — Auditoria: `engine::stdlib`, facade deliberada ou acumulação sem decisão?

**Tipo**: Auditoria — entender, catalogar, **não corrigir nem propor fatiamento ainda**.
**Motivo**: `engine::stdlib` tem fan-out 32 (Passo 1003) e não tem equivalente estrutural
no vanilla — cada módulo de `typst_library` exporta as suas nativas directamente, sem
facade central. Antes de decidir se isto é candidato a fatiamento, preciso de saber **por
que existe** — se foi uma decisão registada, o fatiamento pode ter de a respeitar ou
revisitá-la formalmente (gate); se foi acumulação sem decisão, o fatiamento é mais livre.

---

## Fase A — Ler o L0 actual, literalmente

```
view 00_nucleo/prompts/engine/stdlib.md
```
(ou o caminho real, confirmar primeiro com `find 00_nucleo/prompts -iname 'stdlib*'`)

Registar: o que o Contexto/Instrução diz sobre *por que* existe uma facade central, se diz
alguma coisa. Citar literalmente, não resumir de memória.

## Fase B — Procurar a decisão de origem

```
grep -rln 'stdlib' 00_nucleo/adr/ | xargs grep -l 'facade\|central\|agregador'
```
Se existir um ADR que discuta a decisão de ter uma facade central para funções nativas
(em vez de exportar por módulo, como o vanilla), citá-lo e resumir a razão registada.

Se **não** existir nenhum ADR: procurar no histórico de passos (via `conversation_search`
ou `project_knowledge_search`, conforme disponível) o momento em que `stdlib.rs`/
`engine/stdlib/` nasceu como estrutura — foi uma decisão explícita do primeiro passo que a
criou, ou cresceu organicamente porque cada `native_*` precisava de viver nalgum lado e
este foi o sítio mais óbvio?

## Fase C — Mapear o que está lá dentro, por natureza

Para cada submódulo de `engine/stdlib/` (`foundations`, `structural`, `text`, `state`,
`counter`, `layout`, `eval`, etc. — confirmar lista real primeiro):

1. O que agrupa (que funções nativas, que domínio da linguagem)?
2. É **Declarativo** ou **Imperativo**, pelo mesmo critério já usado no Passo 1002 (toca
   `EvalContext`/scope/estado, ou é `Value → Value` puro)?
3. Tem correspondência directa a um módulo de `typst_library` no vanilla (já parcialmente
   mapeado no Passo 1003, Fase C — reaproveitar, não remedir do zero)?

## Fase D — A pergunta central

Com a Fase A/B respondidas, classificar `engine::stdlib` como:

- **Decisão deliberada, registada** — então o fatiamento futuro precisa de manter a facade
  como está (só reorganizar o que está por trás dela), ou revisitar a decisão formalmente
  (gate ADR-0127, se a mudança alterar contrato público).
- **Acumulação sem decisão registada** — o fatiamento é mais livre; a facade pode
  desaparecer (cada módulo exporta directamente, como o vanilla) sem precisar de reverter
  uma decisão formal, só de reescrever os pontos que hoje dependem da facade existir.

## Fase E — Catálogo (output, sem decidir o fatiamento em si)

- Classificação da Fase D, com evidência.
- Lista de submódulos com o eixo Declarativo/Imperativo aplicado.
- Se acumulação sem decisão: nota de que isto é mais um caso da mesma classe já vista
  (`content.md`, os órfãos do Passo 1001) — estrutura que existe sem que ninguém tenha
  decidido a forma dela, só foi crescendo.

---

## O que este passo NÃO faz

- Não decide fatiar `engine::stdlib`.
- Não remove a facade.
- Não presume a resposta da Fase D antes de procurar.
