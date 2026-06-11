# Tarefa P317-c — Correções no modelo de lote + DEBT dos primitivos de AST

**Repositório de trabalho**: typst-crystalline (raiz).
**Tipo**: manutenção de artefatos de processo (modelo + DEBT + realocação de
L0 deferidos). **Zero código de produto.** Tarefa pequena; pode rodar antes,
durante ou depois da Fase B do Lote 2 — não conflita (toca arquivos que a
Fase B não toca, exceto a checagem final de lint, que deve rodar num tree
consistente).
**Fonte**: revisão externa do `modelo-lote-migracao-d.md` (2026-06-10),
quatro ajustes aprovados pelo dono.
**Commit**: um, `Passo 317c — corrige modelo de lote + DEBT primitivos`.

---

## Ajuste 1 — Resolver a contradição interna do modelo

Em `00_nucleo/modelo-lote-migracao-d.md`, a última regra permanente diz
"Primitivos de AST não entram em lote D" (absoluta), contradizendo o bloco
de elegibilidade ("default: excluir; a composição é decisão do dono por
lote e pode incluir primitivos, com ressalva e medição"). Uma sessão futura
que leia só a lista de regras recusaria um override legítimo.

Substituir a regra final por:

> Primitivos de AST não entram **por default**; inclusão só por decisão
> explícita do dono, com a ressalva no L0 da variante e a medição que a
> justifique (ver critério de elegibilidade acima).

## Ajuste 2 — Regra do L0 deferido (e aplicá-la aos 3 do Lote 2)

**A regra** (adicionar às regras permanentes do modelo):

> L0 redigido para variante deferida **não fica em `00_nucleo/prompts/`**
> (geraria V7 órfão permanente e quebraria o "zero violations"). L0 deferido
> é anexado à entrada de DEBT que registra o deferimento, e volta a
> `prompts/` no passo que o materializar.

**A aplicação**: mover `entities/elements/math_sequence.md`,
`math_text.md` e `math_ident.md` (os 3 deferidos no checkpoint do Lote 2)
para junto da entrada de DEBT do Ajuste 3 — sugestão:
`00_nucleo/debt-anexos/primitivos-ast/` (criar o diretório; confirmar que o
linter não varre `debt-anexos/` — se varrer, usar localização que não varra
e registrar qual). `git mv` para preservar a história.

## Ajuste 3 — DEBT dos primitivos de AST (a âncora durável)

Entrada nova no `00_nucleo/DEBT.md` (próximo número livre, convenção do
arquivo): **"Primitivos de AST fora do modelo D — decisão pendente"**.

Conteúdo mínimo:

- **Conjunto**: `MathSequence`, `MathText`, `MathIdent` (deferidos no P317,
  L0 anexados), `Sequence`, `Empty`, `Block`, e os wrappers estruturais
  (`Styled`, `Boxed`, `Labelled`) **se** a triagem confirmar que são
  primitivos e não element-shaped — a triagem de cada um é parte do fecho.
- **As três saídas possíveis**, sem ordem de preferência: (a) migrar para D
  **com medição de performance antes** (o risco registrado: `Arc` em folha
  quente — `MathIdent` é construída por identificador em eval/layout;
  ADR-0029/0030 fazem de performance de RAM domínio de L1); (b) manter
  inline **por design**, gravado em nota na ADR-0105 (deixa de ser pendência
  e vira forma); (c) forma terceira a desenhar (ex.: payload inline pequeno
  sem `Arc`).
- **Critério de fecho**: cada variante do conjunto com destino decidido e
  gravado (ADR ou migração executada); nenhuma "deferida" sem dono.
- **Gatilho de decisão**: o fim dos lotes element-shaped (quando a tabela de
  largura do P317 esgotar os elegíveis), ou antes se algum passo precisar
  tocar um primitivo.
- **Referências**: relatório P317 (tabela de largura; ressalva do
  checkpoint), modelo de lote (critério de elegibilidade), ADR-0104/0105,
  os 3 L0 anexados (Ajuste 2).

## Ajuste 4 — Duas regras aprendidas que faltam no modelo

Adicionar às regras permanentes:

> - Se a Fase B do lote precisar **editar** um prompt grosso
>   (`rules/eval.md`, `rules/parse.md`, `rules/layout.md` ou outro com
>   linhagem larga), o imposto morde: **fatiar primeiro** pela receita do
>   P314 (partição content-preserving, `_comum.md` por área, `git rm` do
>   prompt velho — a trilha fica no git, decisão do dono no P316).
> - **Um commit isolável por lote** (pré-tarefas separadas do lote
>   principal). Lição medida no diagnóstico 313: o P298 não era isolável no
>   histórico e custou à medição.

---

## Validação e relatório

- `crystalline-lint .` → **ZERO violations** num tree consistente (em
  particular: os 3 L0 movidos não aparecem mais como órfãos; se a Fase B do
  Lote 2 estiver no meio, rodar a checagem após o ponto de sincronização e
  registrar o momento).
- `git status` limpo após o commit; `git log --oneline -1`.
- Resumo no chat: os 4 ajustes aplicados (diff de uma linha cada onde
  couber), o número da entrada de DEBT criada, a localização final dos 3 L0,
  e a confirmação do lint.

## Fora de escopo

- A Fase B do Lote 2 (corre pelo modelo, em paralelo ou em sequência).
- Decidir o destino dos primitivos (o DEBT registra; a decisão tem gatilho).
- Qualquer edição nos L0 das 11 variantes confirmadas.
