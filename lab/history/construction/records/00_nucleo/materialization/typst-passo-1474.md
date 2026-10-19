# Passo 1474 — adjudicar as 42 rotas cristalinas extras

**Estado:** ESCRITO — DIAGNÓSTICO PRIMEIRO; NENHUMA REMOÇÃO AUTORIZADA

**Data da auditoria:** 2026-09-20T17:40:24-03:00

**Baseline de redação:** `306ca50022104619447fae2625090b8d9c7b0a35`,
tree `0f7f307e8ffda460270714cab0ae525237dc3433`, working tree não commitada
descrita no P1467.

**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`

## 1. Objetivo

Classificar cada rota `CRYSTALLINE_ONLY` normalizada como:

- extensão deliberada a preservar;
- alias histórico a depreciar;
- vazamento de implementação a remover;
- falso positivo do inventário;
- `Unknown` que exige nova sonda.

Este passo não recebe autorização para remover todas as rotas em lote. Ele
produz decisões por família e somente então abre passos de quebra compatível,
se necessários.

## 2. Limpeza prévia do número

O catálogo bruto reporta 48 extras no default e 49 nos perfis HTML. Sete são
falsos positivos: campos default acessíveis bilateralmente sob `format`:

```text
format.svg.pretty=false
format.png.ppi=144.0
format.pdf.pages=none
format.pdf.standard=("1.7",)
format.pdf.tagged=auto
format.pdf.pretty=false
format.html.pretty=false   # apenas perfis HTML
```

Depois da normalização restam exatamente 42 rotas:

```text
accent
asset
bb
bold
cal
calc.deg
calc.log10
calc.rad
cancel
counter_at
counter_display
counter_final
counter_step
display
frak
grid_cell
grid_footer
grid_header
inline
italic
lof
lot
math.registered
mono
op
replace
sans
scr
script
serif
sscript
state_at
state_display
state_final
state_update
state_update_with
sym.registered
table_cell
table_footer
table_header
underover
upright
```

As contagens vieram do baseline acima, com binário cristalino SHA-256
`9c94277e84e859bcee1b9691fcba3a6806d34f7d581551c7ea3083c04c5752a6`
e vanilla `/usr/local/bin/typst` SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.

## 3. Adjudicação por família

Criar uma tabela reproduzível, uma linha por rota, com:

```text
rota | kind/repr cristalino | resultado vanilla | origem file:line |
primeiro commit | uso interno | fixture real | decisão | refutação
```

Examinar nesta ordem:

1. estilos/math globais: `accent`, `bb`, `bold`, `cal`, `cancel`, `display`,
   `frak`, `inline`, `italic`, `mono`, `op`, `sans`, `scr`, `script`, `serif`,
   `sscript`, `underover`, `upright`;
2. aliases `calc`: `deg`, `log10`, `rad`;
3. aliases de counter/state;
4. aliases globais de grid/table;
5. `asset`, `lof`, `lot`, `replace`;
6. `math.registered` e `sym.registered`.

Para cada família, medir também se documentos do próprio repositório e suites
externas conhecidas usam o nome. Ausência no vanilla prova divergência de
superfície, mas não decide sozinha se uma extensão documentada deve ser
removida.

## 4. Regras de decisão

- Binding necessário apenas como implementação interna deixa de ser global;
  o owner continua acessível por caminho privado/estático.
- Alias público deliberado só permanece se o L0 o declarar como extensão e os
  catálogos o excluírem do numerador de paridade.
- Alias histórico a remover exige plano de depreciação ou quebra explícita;
  não basta apagar `scope.define`.
- `registered` não pode ser removido se for mecanismo de introspecção
  cristalino ainda contratado; deve ser marcado como extensão sem crédito.
- P1472 deve terminar antes da decisão sobre estilos globais, para não remover
  o único caminho funcional durante a migração.
- Não converter extra em membro ausente do vanilla nem esconder rotas no
  inventoriante.

## 5. Gate ADR-0127 e divisão posterior

Qualquer remoção/renomeação quebra compatibilidade. Ao fim da tabela:

1. atualizar os L0s donos apenas das decisões aprovadas;
2. agrupar remoções por owner e dependência, não por conveniência numérica;
3. criar um passo de materialização por família com RED de ausência e
   regressão do caminho canônico;
4. ressellar hashes, validar V5/V15/V26 e **parar para confirmação humana**.

P1474 pode fechar como diagnóstico sem remover nada. Extensões preservadas
devem continuar visíveis como extras deliberados; “paridade total” deve
reportar separadamente paridade vanilla e extensões.

## 6. Gates do diagnóstico

- regenerar os oito catálogos duas vezes e confirmar determinismo;
- provar a normalização 48/49 → 42 sem subtrair rota real;
- nenhuma linha sem `file:line`, estado Git e evidência bilateral;
- zero alteração produtiva neste passo;
- `git diff --check` e V5/V15/V26 verdes se algum L0 diagnóstico for tocado;
- registrar no próprio passo a tabela final e os números recalculados.

## 7. Resultado (2026-09-22) — fecha como diagnóstico

Diagnóstico completo em
`00_nucleo/diagnosticos/typst-p1474-adjudicacao-42-extras.md`. Resumo:

- Não foi possível regenerar o catálogo automatizado P1140.x completo
  nesta sessão (ferramenta não disponível como script executável neste
  checkout); as 42 rotas foram revalidadas **individualmente** por sonda
  bilateral direta (`typst eval "type(<rota>)"`), com proveniência por
  linha — método mais lento, mas cumpre a exigência do passo de
  `file:line`/evidência bilateral por rota.
- As 42 rotas continuam todas presentes e com o mesmo kind desde a
  redação original (2026-09-20); nenhuma desapareceu com P1467-P1473.
  P1472 tornou 17 das 18 rotas de "estilos globais" redundantes com uma
  rota `math.*` qualificada equivalente (confirmado por identidade de
  função, `Func::PartialEq`).
- Classificação final: **10** extensões deliberadas já decididas em L0
  (grid/table flat aliases via P1293; `lof`/`lot`; `math.registered`/
  `sym.registered` via P1283) — nenhuma ação; **18** candidatas a
  depreciar (17 estilos globais + `calc.log10`, todas com caminho
  canônico equivalente confirmado) — passo futuro por família; **1**
  lacuna real não decisão (`underover` sem `math.underover`); **1**
  falso positivo do inventário (`asset` — existe no vanilla atrás de
  `Feature::Bundle`, gate ausente no cristalino, não é extra); **12**
  requerem nova sonda antes de qualquer decisão (`counter_*`/`state_*`
  — comportamento `None` vs `init` documentado e distinto, não
  puramente redundante; `calc.deg`/`calc.rad`/`replace` — uso real não
  investigado a fundo).
- Nenhuma remoção, renomeação, L0 produtivo ou código foi tocado nesta
  sessão — conforme autorizado explicitamente pelo próprio passo (§Estado:
  "P1474 pode fechar como diagnóstico sem remover nada"). `git status
  --short` mostra só o diagnóstico e este ficheiro. `git diff --check`:
  sucesso. Nenhum L0 (`00_nucleo/prompts/`) foi tocado, logo V5/V15/V26
  não se aplicam (nada para resselar).

