# Passo 982 — oráculo: aprofundar convergência `TJ`/`Tj` e investigar `q`/`Q` descasado de `cm`

**Precede este passo**: auditoria externa (2026-08-06) sobre a saída actual do oráculo (P980) —
dois itens específicos do oráculo, não da saída principal.

**Pré-condição de árvore**: `git status`. Confirmar P981 presente.

---

## Parte A — `TJ`/`Tj` ainda em 20.3%/79.7%, vanilla em 7.5%/92.5%

P980 já colapsou `TJ` sem ajuste real para `Tj` (31.2%→79.7% de `Tj`, medido então; a auditoria
agora confirma 79.7%/20.3%, mesmo estado). O residual são `TJ` com ajuste **real** que o vanilla
não precisa nesses mesmos pontos — ou seja, o delta model do cristalino está a gerar mais ajustes
de posição do que o vanilla considera necessários.

### Fase A.1 — confirmar a causa dos ajustes residuais

1. Localizar, no documento de 30 secções, casos onde o oráculo emite `TJ` com ajuste não-zero mas
   o vanilla, no mesmo ponto, emite `Tj` simples — comparar os valores de ajuste, confirmar se são
   pequenos (arredondamento/tolerância) ou reais (kerning/tracking genuíno).
2. Se forem pequenos (abaixo de uma tolerância razoável, por exemplo <1/1000 em): considerar
   estender `collapse_trivial_tj` (P980) para tratar "quase zero" como zero, com uma tolerância
   confirmada contra o comportamento do vanilla, não escolhida arbitrariamente.
3. Se forem reais: confirmar por que o cristalino precisa desse ajuste onde o vanilla não precisa
   — pode ser uma diferença genuína no delta model (P485/P520/P548) que produz ajustes que o
   vanilla evita por outro mecanismo (por exemplo, um kerning já embutido na fonte que o vanilla
   deixa o rasterizador aplicar via GPOS, e o cristalino calcula explicitamente e escreve como
   ajuste).

### Fase B — Implementação

TDD directo se for ajuste de tolerância no colapso; investigação mais profunda (possível protocolo
de dois agentes) se for diferença genuína no delta model. Continua restrito ao oráculo
(`03_infra/src/export/oracle.rs`), não à saída principal — mesma decisão de P980.

## Parte B — `q`/`Q` (1361) descasado de `cm` (1293), delta de 68 = número de traços vetoriais

### Fase A — confirmar a causa

1. Confirmar se os 68 `q`/`Q` extra envolvem especificamente os traços vetoriais (`w`/`m`/`l`/`S`,
   linhas de fração/matriz avançada) separadamente do `q`/`cm`/`Q` de texto — ler o código de
   emissão de traço vetorial (`stream.rs`, o caminho que desenha barras de fração/linhas de
   matriz) e confirmar se cada traço abre o seu próprio `q`/`Q` sem `cm` correspondente.
2. Confirmar se isto é comportamento intencional (isolamento de estado gráfico para o traço, sem
   precisar de transformação) ou um resíduo/bug — comparar com o padrão do vanilla para o mesmo
   tipo de traço.

### Fase B — decidir se há algo a corrigir

Se for comportamento correto (só `q`/`Q` sem `cm` porque o traço não precisa de transformação):
registar como explicado, sem código. Se for resíduo/duplicação: TDD directo, mesma disciplina.

## Resultado esperado

- Parte A: causa do residual `TJ` confirmada (tolerância vs diferença genuína de delta model);
  proporção `Tj` do oráculo mais próxima dos 92.5% do vanilla, se a Fase A permitir.
- Parte B: descasamento `q`/`Q` vs `cm` explicado ou corrigido.
- Ambas restritas ao oráculo — saída principal inalterada, mesma prova de P980 (byte-idêntica sem
  a flag).
