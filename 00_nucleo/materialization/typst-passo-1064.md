# L0 — Passo 1064: Auditoria V21 Categoria 1 (40 casos de `/2.0` de centragem geométrica)

**Escopo**: só auditoria e classificação. Nenhuma anotação de silêncio, nenhuma
mudança de código, nenhuma alteração à regra V21 do `tekt-linter` neste passo.

**Gate**: nenhum — não há mudança de comportamento nem de código.

**Base**: pendência registada no documento de continuação desta conversa —
reconciliamento P1054/P1055, Categoria 1 nunca confirmada por amostra. Hipótese a
testar: `/2.0` para centrar conteúdo é geometria universal (não precisa de citação
externa, ao contrário de `0.65em`/`1.2em`, que vêm de decisões específicas do
vanilla).

---

## Parte 0 — Ler a regra real antes de assumir critério

Per o protocolo do manifesto (prompt antes de código, e aqui, antes de julgamento):
localizar e ler o prompt L0 da regra V21 (`unsourced_constant`) no `tekt-linter`
antes de decidir se "geometria universal sem citação" já é um critério formal
existente ou se está a ser inventado agora.

```bash
find 00_nucleo/prompts -iname "*unsourced*" -o -iname "*v21*"
```

Ler o arquivo encontrado (nome exacto não presumido) por inteiro antes de avançar
para a Parte 1. Se já existir uma categoria de excepção para expressões puramente
aritméticas/geométricas, este passo só confirma que os 40 casos cabem nela — não
inventa critério novo.

## Parte 1 — Levantar o inventário completo dos 40 casos

```bash
grep -rn "/ 2\.0\|/2\.0" 01_core/src/compiler/layout/columns.rs \
  01_core/src/compiler/layout/cursor.rs \
  01_core/src/compiler/math/layout/attach.rs \
  01_core/src/compiler/math/layout/frac.rs
```

Confirmar que a busca acima cobre os arquivos correctos — os quatro citados na
pendência (`columns.rs`, `cursor.rs`, `attach.rs`, `frac.rs`) podem não ser a lista
completa; os 40 casos podem estar espalhados por mais arquivos. Rodar também:

```bash
grep -rln "/ 2\.0\|/2\.0" 01_core/src/compiler/
```

e reconciliar contra o número 40 já citado (P1054/P1055) — se o total não bater,
isso é achado a registar, não a ignorar.

## Parte 2 — Amostragem, não os 40 de uma vez

Escolher uma amostra representativa — pelo menos 3 casos por arquivo dos quatro já
identificados (mínimo 12, não os 40 completos neste passo) — e para cada caso:

1. Transcrever a linha de código exacta e o contexto imediato (função/bloco).
2. Confirmar que a fórmula é genuinamente `(espaço_disponível - tamanho_conteúdo) / 2.0`
   (ou equivalente algébrico) — centragem — e não outra coisa que só parece `/2.0`
   de relance (ex.: meia-largura de um traço, ponto médio de um intervalo com outro
   significado).
3. Verificar se o vanilla, no ponto correspondente, também usa uma divisão por 2
   genérica (sem constante nomeada nem citação de decisão de design) — se sim,
   reforça a hipótese de geometria universal; se o vanilla usa uma fórmula diferente
   ou uma constante com nome próprio nesse ponto, a hipótese falha **para esse caso
   específico**, mesmo que valha para os outros.

## Parte 3 — Classificar, não decidir sozinho o que fazer a seguir

Para cada caso da amostra, produzir um veredicto:

- **Confirma hipótese** — centragem genuína, vanilla também usa divisão genérica.
- **Não confirma** — motivo específico (fórmula não é centragem, ou vanilla usa algo
  com proveniência própria).
- **Inconclusivo** — vanilla não tem ponto correspondente directo, ou a comparação
  não é possível sem medição adicional.

Se a amostra confirmar a hipótese em todos os casos: propor ao dono, como
recomendação (não execução automática), estender a conclusão aos 40 casos e
anotá-los como classe legítima — mas essa anotação é passo seguinte, não este.

Se algum caso da amostra **não confirmar**: não generalizar — os outros 40 (menos a
amostra) precisam da mesma verificação individual antes de qualquer conclusão.

## Critério de conclusão

- Resposta da Parte 0 (critério da regra já existe ou não).
- Inventário completo (Parte 1) com contagem reconciliada contra "40".
- Pelo menos 12 casos auditados individualmente (Parte 2), com veredicto (Parte 3).
- Nenhuma anotação de código, nenhuma mudança em `.rs` ou nas regras do
  `tekt-linter`.
