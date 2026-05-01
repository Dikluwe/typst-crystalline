# Pesquisa — inventário de tipos do módulo introspection vanilla

## Contexto

Existe desenho arquitectural cristalino para Introspection com
13 tipos definidos (ver `desenho-introspection-fixpoint.md` ou
documento equivalente). Antes de avançar com implementação ou
refinar o desenho, é preciso saber **literalmente** que tipos
existem no módulo introspection do vanilla, para identificar
gaps reais (não inferidos).

Esta pesquisa não decide nada. Não propõe alterações ao desenho.
Não cria ADR. Apenas inventaria.

---

## O que fazer

Inspeccionar `lab/typst-original/crates/typst-library/src/introspection/`
e produzir inventário dos tipos definidos.

Output: ficheiro markdown único em
`00_nucleo/diagnosticos/inventario-tipos-introspection-vanilla.md`
com 3 secções definidas abaixo.

---

## Secção 1 — Lista de tipos vanilla

Tabela com uma linha por tipo `pub` definido no módulo. Colunas:

- **Tipo**: nome do tipo (ex: `Counter`, `Locator`).
- **Kind**: `struct` | `enum` | `trait` | `type alias`.
- **Ficheiro**: caminho relativo a `introspection/`.
- **Linhas**: tamanho do ficheiro inteiro (referência rápida).
- **Responsabilidade (1 linha)**: descrição literal extraída
  do doc-comment ou dos campos. Sem inferência sobre meta-pattern.

Critério para incluir:
- Tipos `pub` no module root.
- Tipos `pub` em sub-módulos directamente expostos.
- Excluir: tipos privados, módulos de tests, helpers internos
  (e.g. `XxxBuilder` se for `pub(crate)`).

Se houver ambiguidade sobre incluir ou não, incluir e marcar
"interno?" na coluna de Responsabilidade.

---

## Secção 2 — Cruzamento com desenho cristalino

Lista de tipos do desenho cristalino (13 tipos da §2.1 do
desenho):

1. `Location`
2. `Locator`
3. `Tag`
4. `Introspector`
5. `Counter`
6. `CounterRegistry`
7. `State`
8. `StateRegistry`
9. `MetadataStore`
10. `LabelRegistry`
11. `QueryEngine`
12. `DocumentInfo`
13. `WalkContext`

Para cada um destes, indicar **literalmente**:
- Existe equivalente em vanilla? (nome do tipo vanilla, ou
  "não — responsabilidade dispersa por X tipos", ou "não
  existe").
- Se existe, está no mesmo ficheiro/módulo ou disperso?

Tabela com 3 colunas: tipo cristalino | equivalente vanilla |
notas (ficheiro, dispersão).

---

## Secção 3 — Tipos vanilla sem equivalente no desenho

Lista de tipos da Secção 1 que **não** têm equivalente claro
nos 13 do desenho. Para cada:

- Nome do tipo vanilla.
- Ficheiro.
- Responsabilidade.
- Avaliação literal: este tipo existe porque vanilla precisa
  dele para arquitectura vtable, ou cobre conceito que cristalino
  precisará independentemente da arquitectura?

Esta é única secção que admite avaliação. Avaliação é literal,
não normativa: "vanilla usa para vtable dispatch" vs "cobre
conceito de localização" vs "não consegui determinar".

Se um tipo é conceptualmente necessário mas vanilla resolve
de forma específica que cristalino não vai imitar, marcar
"conceito necessário; forma vanilla não aplicável".

---

## Restrições

- **Não propor alterações** ao desenho cristalino.
- **Não criar ADR** sobre os achados.
- **Não classificar** tipos vanilla como "bons" ou "maus".
- **Não inferir** que cristalino "deve" ter equivalente — só
  marcar se equivalente existe ou não.
- **Não inflar linguagem**: sem "patamar", "cumulativo",
  "cross-domínio".
- **Não classificar como passo PNNN**: é pesquisa, não passo
  de migração.

---

## Critério de conclusão

- Ficheiro `inventario-tipos-introspection-vanilla.md`
  produzido.
- 3 secções presentes.
- Secção 1 lista todos os tipos `pub` do módulo introspection
  vanilla.
- Secção 2 cruza os 13 tipos cristalinos com vanilla.
- Secção 3 identifica tipos vanilla sem equivalente.
- Sem ADR criada.
- Sem propostas de alteração ao desenho.

---

## Estimativa de esforço

Inspecção de directório com ~13 ficheiros. Trabalho de leitura
+ tabulação. Sem necessidade de executar tests, builds, ou
análise estática. Provavelmente 30-60 minutos de leitura.

Se algum ficheiro for muito grande (>1000 linhas) e tiver
muitos tipos, listar todos mas com responsabilidade resumida
em palavras-chave em vez de prosa.
