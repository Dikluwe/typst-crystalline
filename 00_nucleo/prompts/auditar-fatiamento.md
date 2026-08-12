# Workflow — `auditar-fatiamento.md`

**Propósito**: método para decidir se e como fatiar um ficheiro/módulo grande (hub) em
unidades menores (nós). Extraído de 4 aplicações reais — `operators.rs` (P1002),
`layout::metrics` (P1006, recusado por inteiro), `eval::bindings` (P1013),
`stdlib::structural` (P1014) — não escrito a priori. Cada regra abaixo tem proveniência.

**Não é**: um gate de aprovação automático. É um roteiro de investigação — o resultado
pode ser "não fatiar" (P1006) e isso é um resultado válido, não uma falha do método.

---

## Ordem obrigatória

### Passo 0 — Critério-zero: agregado ou interface?

Antes de qualquer outra coisa:
```
grep -n '^pub trait \|^trait ' <ficheiro>
```
Se o ficheiro define um `trait` público com múltiplos implementadores reais e a maior
parte do conteúdo são métodos desse trait: **é interface**. Fatiar exige mudar contrato
público (gate ADR-0127) para mover pouca lógica real — normalmente não compensa
(P1006: `layout::metrics`, trait com 13 implementadores, ~90 linhas de lógica, recusado).

Se é um agregado de funções livres (`fn`/`pub(...) fn`, qualquer visibilidade): prosseguir.

**Confirmado nas 2 vezes em que foi decisivo** (P1006 recusa, P1013/P1014 confirmam
"agregado" sem fricção) — vem sempre primeiro.

### Passo 1 — Inventário completo, com regex genérico de visibilidade

**Erro repetido três vezes, em três formas**: P1000 (grep sem `pub(crate)`, escondeu 4 de
13 funções de `operators.rs`), P1013 (regex tinha `pub(crate)` mas não `pub(super)`,
escondeu 18 de 43 funções de `bindings.rs`). A lacuna não foi fechada por corrigir um caso
específico — foi deslocada.

**Regex correcto, cobre qualquer visibilidade restrita**:
```
grep -nE '^(pub(\([a-z:) ]+\))? )?fn ' <ficheiro>
```

Contar também `enum`/`struct` de topo se relevantes ao domínio (P1013: `enum
ContentField` fazia parte do inventário e quase ficou de fora).

### Passo 2 — Os 4 critérios, aplicados nesta ordem de peso evidenciado

#### Critério 3 (co-mudança histórica) — o que decide, na prática

Nas 4 aplicações, foi o único critério que **sempre** produziu sinal accionável:
corrigiu a hipótese inicial em P1002 (`coercion.md` descartado), confirmou-a em P1013
(bateu commit a commit com os clusters vanilla), e corrigiu-a **por excesso** em P1014
(recusou fragmentar `table_grid`/`table_lines` apesar do vanilla os separar).

Ferramenta: `tools/analysis/cochange_metrics.py <ficheiro> <regex-de-caminhos-históricos>`
(cobrir renames conhecidos — `rules/`→`engine/`→`compiler/` — no regex). **Descontar
commits que só mudam `@prompt-hash`/`@updated`** (proposto no P1006, na ferramenta desde
P1013) — sem isto, resselos em massa (renames, `cargo fmt` global) mascaram o sinal real.

Um commit que toca muitas funções de clusters diferentes de uma vez (P1013: `b806f562d`,
16 funções, 3 clusters) é **ruído estrutural** (ex.: introdução de um mecanismo
transversal), não sinal de fronteira — identificar e descontar.

#### Critério 4 (correspondência vanilla) — hipótese a testar, nunca aceitar sozinho

Duas formas de errar, ambas já observadas:
- **Por defeito** (P1006): o mapeamento por nome apontava para o ficheiro errado
  (`layout::metrics` ≈ "medida" por semelhança de nome, não por conteúdo real).
- **Por excesso** (P1014): o vanilla fragmenta mais finamente do que a co-mudança
  sustenta (21 ficheiros vanilla para 50 funções cristalinas; a história só sustenta 9).

"Seguir o vanilla" nunca é a decisão final — é o candidato que o critério 3 aceita,
corrige, ou rejeita.

#### Critério 2 (pureza vs estado) — três classes, não duas

**Refinamento do P1012/P1013**: a presença de `Engine`/`EvalContext` na assinatura **não
decide sozinha**. Medir por `file:line` o uso *depois* do ponto onde o contexto é
recebido. Três classes observadas (P1013):

1. **Sem contexto nenhum** — assinatura não recebe `Engine`/`ctx`.
2. **Pass-through** — recebe, mas zero usos reais depois (só propaga para avaliar
   argumentos de forma que não introduz efeito). Classificar como funcionalmente puro
   apesar da assinatura.
3. **Contexto real** — usos medidos e citados (`file:line`) de leitura/escrita com efeito
   (I/O, sink, introspector, counters).

Um nó pode ser correctamente classificado "declarativo" **por razão mecânica** (é
pass-through) — nunca por "propósito" ou "o que o código pretende fazer" (erro do P1011
com `font_dict`, corrigido no P1012 com evidência).

Quando este critério discrimina (P1013 — 3 classes reais), é decisivo. Quando o ficheiro
inteiro é puro ou inteiro impuro (P1002, P1006, P1014 — vácuo 3 de 4 vezes), não
descarta nem confirma nada sozinho — mas **ainda tem de ser medido**, porque o vácuo em
si é informação (confirma que a fronteira de estado não é o eixo relevante aqui).

#### Critério 1 (isolamento de teste) — proposta: retirar ou substituir

**Vácuo em 4 de 4 aplicações**, e em P1014 chegou a apontar **contra** o fatiamento
correcto (os 56 testes partilham harness único; dividi-los criaria 9 cópias). Este
projecto testa E2E por código Typst, num ficheiro por módulo — a granularidade de teste
nunca vai coincidir com a granularidade de função.

**Proposta, ainda não aplicada, para confirmar na próxima aplicação**: substituir por
"quem chama" — fan-in por símbolo *dentro* do módulo/domínio (quantos consumidores
distintos usam cada função), como proxy de coesão, em vez de tentar isolar testes que
não se isolam nesta base de código.

### Passo 3 — Verificar órfãos antes de escrever L0 de raiz

**Confirmado 2 vezes** (P1002 com `decimal-arithmetic.md`, P1014 com os 4 de `stdlib/
grid_*.md`/`table_*.md`): um prompt órfão existente é uma hipótese de fronteira já
escrita por alguém, frequentemente certeira. Ler antes de escrever, confrontar com o
código actual (pode ter derivado), absorver o que estiver correcto.

**Verificar também `crystalline.toml`** — excepções de órfão apontando para ficheiros já
removidos por um fatiamento anterior (achado lateral do P1014: `decimal-arithmetic.md`
tinha sido apagado em P1002, a excepção sobreviveu).

### Passo 4 — Materializar

- `V15`: um `@prompt` por ficheiro `.rs`. Cada nó implica sempre um ficheiro próprio.
- Preservação de comportamento: corte e cola, não reescrita. Provar item a item
  (contagem antes/depois, corpos comparados) — não só "os testes passam".
- Visibilidade: ao mover função de um agregado plano para um nó, respeitar a visibilidade
  original — `pub(super)` no ficheiro velho pode exigir `pub(in caminho::completo)` no nó
  novo, reexportado por `pub(super) use` no hub (E0364 se o hub tentar reexportar mais
  largo do que o item permite — P1013).
- **O hub nem sempre é uma tabela de despacho.** Se a unidade original era um agregado
  plano (chamadas directas, sem dispatcher), o hub correcto é só fronteira de
  reexportação — registar isto explicitamente no L0 para não parecer "hub por preencher"
  (P1013).
- **Testes com harness partilhado ficam no hub**, não nos nós — decisão tomada em P1014,
  proposta como regra geral daqui para a frente. Registar no L0 do hub que ele aloja a
  suite, não é descuido.

### Passo 5 — Validar

```
crystalline-lint .
cargo test --workspace
```
Comparar contagem de `#[test]` antes/depois — tem de bater exactamente.

### Passo 6 — Avaliação do método (obrigatória em cada aplicação)

Mesmo quando o resultado é "não fatiar" (P1006). Registar: que critério decidiu, que
critério foi vácuo, se alguma hipótese inicial (candidato vanilla, divisão por inspecção)
foi corrigida pela evidência, e se a cláusula de guarda de um passo anterior (ex.: "se
este fatiamento tocar X, confirmar Y") foi ou não activada — **declarar explicitamente
qual dos dois, nunca deixar a ausência de menção implicar a resposta** (lição do P1014 —
a confirmação de `table_counter` teve de ser pedida depois, porque o relatório não disse
"a cláusula não se activou").

---

## Resumo de proveniência

| Regra | Origem |
|---|---|
| Critério-zero primeiro | P1006 |
| Regex de visibilidade genérico | P1000 → P1013 (erro repetido, agora fechado aqui) |
| Descontar ruído de resselo em co-mudança | P1006 (proposto) → P1013 (na ferramenta) |
| Vanilla nunca aceite sozinho | P1006 (defeito) + P1014 (excesso) |
| 3 classes de pureza, medidas por `file:line` | P1012 (`font_dict`) + P1013 (refinado) |
| Verificar órfãos antes de escrever | P1002 + P1014 |
| Verificar `crystalline.toml` por excepções mortas | P1014 |
| Hub sem dispatcher = só reexportação | P1013 |
| Testes com harness partilhado ficam no hub | P1014 |
| Declarar explicitamente se guarda activou ou não | P1014 (esta consolidação) |
| Critério 1 — candidato a retirar/substituir | P1002, P1006, P1013, P1014 (4/4 vácuo) |
