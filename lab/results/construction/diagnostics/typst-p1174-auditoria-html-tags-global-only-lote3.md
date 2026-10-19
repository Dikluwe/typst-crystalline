# P1174 — auditoria do terceiro lote HTML global-only

**Data:** 2026-08-25
**Estado:** concluída; parada no gate ADR-0127
**HEAD:** `ce49041de76eb64c011e990e9774a0339f979211`
**Início:** `2026-08-25T15:04:33-03:00`
**Árvore:** working tree não commitado P1173.1; índice vazio
**Vanilla:** `a51e02804`; SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`

## Proveniência

No início havia cinco ficheiros de produção/L0 modificados, o passo P1173.1
modificado e dois documentos não rastreados; `git diff HEAD --stat` reportou 6
ficheiros, 161 inserções e 19 remoções porque o diagnóstico e P1174 ainda eram
untracked. `git diff --cached --quiet` terminou com exit 0. O cristalino medido
foi `./target/debug/typst`, SHA-256
`c431bd7cab232b4dc256975076d119e741487c60c5c43713c787ae6ecfe7f855`.

## Medição da composição

Uma comparação mecânica no inventário P1140.26 usou `html.div` como baseline.
Cada candidata teve 77 parâmetros, igualdade integral de nomes, ordem e
metadados, zero extras, zero ausentes e estado cristalino `present=false`.

| tags | específicos | body | void/raw | display | contexto validado no constructor |
|---|---:|---|---|---|---|
| `dd dl dt` | 0 | content opcional | não | block | não |
| `figcaption figure` | 0 | content opcional | não | block | não |
| `footer header hgroup legend main menu` | 0 | content opcional | não | block | não |
| `mark` | 0 | content opcional | não | inline/phrasing | não |

Na fonte pinada, `typed.rs:73-114` constrói parâmetros dos atributos da entrada
e adiciona body; `:125-158` constrói a mesma entidade. `tag.rs:125-158` prova
que nenhuma candidata é void/raw/escapable-raw. `property.rs:82-120` prova as
onze block, enquanto `property.rs:194` e `tag.rs:298-335` provam `mark`
inline/phrasing. A fonte gerada de `typst-assets` não está materializada; o pin
permanece provado pelo lockfile, e os atributos foram triangulados com o
inventário e o binário, sem inventar linhas de `data.rs`.

## Sondas de linguagem

Para cada uma das 12 tags, o vanilla retornou:

- `repr(type(html.TAG))` → `function`;
- chamada vazia → `elem(tag: "TAG", body: none)`;
- `id: "k", class: "c", hidden: true` → attrs `id`, `class`, `hidden: ""`
  nessa ordem, com body `[X]`;
- `href`, `value` e `start` → exit 1, `unexpected argument: NAME`.

Em representantes block (`figure`) e phrasing (`mark`), named desconhecido e
body named produziram `unexpected argument: NAME`; um segundo body posicional
produziu `unexpected argument`. A aceitação de `dd` e `legend` isolados refuta
a hipótese de validação parental no constructor tipado.

No cristalino pré-código, as 12 sondas terminaram com exit 1 e `module 'html'
does not contain field "TAG"`: assinatura ABSENT. Feature off continuou com o
diagnóstico gated vigente. A construção genérica por `html.elem` permaneceu
disponível, mas não substitui a sintaxe pública tipada ausente.

Como controle de regressão do mesmo estado, `repr(type(html.TAG))` retornou
`function` para os 28 bindings materializados até P1173.1, com zero ausentes.

## Morfologia e export

Duas fixtures foram compiladas com `--format html --features html`, usando
constructors tipados no vanilla e `html.elem` no cristalino:

1. nesting `dl/dt/dd`, `figure/figcaption`, `header`, `hgroup`, `main`,
   `legend`, `menu`, `mark` e `footer`;
2. topo com `mark`, `dd` e `legend` isolados.

Ambos os pares terminaram com `cmp` exit 0. O observável mostrou `<p><mark>M`
`</mark></p>`, seguido por `<dd>D</dd><legend>L</legend>` sem wrappers. Espaço
entre irmãos block foi removido e o phrasing interno foi preservado. Logo as
tabelas L3 materializadas até P1173.1 já cobrem o lote; nenhum L0 de entidade
ou exporter precisa mudar.

A igualdade de bytes é evidência da morfologia observada nestas fixtures, não
um requisito de copiar a mecânica Rust.

## Classificação e decisão

As 12 tags são ABSENT em sintaxe/assinatura pública e MATCH aproximado em DOM
somente pela construção genérica. Todas sobrevivem ao lote global-only. A
implementação proposta é mecânica: wrappers estáticos reutilizando
`native_typed_html(tag, &[], args)` e entradas em `TYPED_TAGS`.

É contrato público novo, portanto P1174 atualizou somente o L0
`compiler/stdlib/html.md` e para agora no gate ADR-0127. Não houve resselo,
teste, código L1–L4, staging ou commit. A aprovação solicitada cobre exatamente
os 12 bindings, cada um com 76 globais e body content opcional; não cobre
qualquer outra tag, específico, entidade, exporter, default ou fase.
