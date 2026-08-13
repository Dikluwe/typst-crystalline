# Passo 1026 — Gate: dimensionamento de delimitadores de grelha (`cases`/`matrix`)

**Tipo**: Investigar → apresentar opções → gate (ADR-0127, categoria 2 — muda output de
produção) → implementar se aprovado.
**Depende do Passo 1025**: a Fase A desse passo confirma/corrige o binário de referência.
Se a Hipótese 2/3 dele se confirmar (resync necessário), **remedir aqui antes de decidir**
— os números abaixo, do Passo 1024, foram medidos contra o que na altura se pensava ser
0.15.0; confirmar que não mudam contra o hash ratificado antes de os usar para decidir.
**Achado (Passo 1024)**: `cases.md`/`matrix.md` afirmavam que a folga de altura dos
delimitadores "aplica a margem de 10% do vanilla" — falso em mecanismo e sentido, não só
em grau:

| | cristalino | vanilla (referência a confirmar via P1025) |
|---|---|---|
| operação | **multiplica** a altura da grelha | **subtrai** do alvo de esticamento |
| fórmula | `grid_height_pt = (ascent + descent) * 1.1` (`compiler/math/layout/mod.rs:390`) | `let short_target = target - short_fall;` (`typst-layout/src/math/fragment/glyph.rs:271`) |
| grandeza | 10% proporcional à altura | `DELIM_SHORT_FALL = Em::new(0.1)` — absoluto ao corpo (`typst-library/src/math/lr.rs:17`) |

Divergência cresce com a altura da matriz (não é erro constante) — já registado como o
achado mais grave da leva do Passo 1024, tratado com prioridade sobre o resto do Bloco 3.

**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1025.

---

## Fase A — Remedir contra o binário confirmado (per P1025)

Repetir a medição do Passo 1024 (fórmula e grandeza, tabela acima) contra o binário que o
Passo 1025 confirmou/corrigiu como referência. Se os números baterem com os já registados,
prosseguir. Se divergirem, actualizar a tabela antes de apresentar as opções.

## Fase B — Apresentar as duas opções (per o próprio Passo 1024), não decidir sozinho

**Opção 1 — Portar o short-fall absoluto (paridade estrutural)**

Substituir `(ascent + descent) * 1.1` por `target - Em::new(0.1).to_pt(font_size)` (ou
equivalente cristalino). Aproxima-se do mecanismo vanilla directamente.

- A favor: paridade de mecanismo, não só de resultado pontual — a divergência que cresce
  com a altura desaparece por construção.
- Contra: muda a geometria de **todas** as matrizes/cases existentes que já renderizam
  hoje com a fórmula proporcional — qualquer documento real com `cases`/`matrix` grande
  muda de aparência (mais estreito no delimitador do que hoje).

**Opção 2 — Manter a folga proporcional, com medição de output que a justifique**

Se houver razão para preferir a fórmula proporcional (ex.: comportamento melhor medido em
casos reais do corpus canónico, ou alguma vantagem visual documentável), mantê-la como
divergência **deliberada**, com a medição a favor registada no L0 (mesma disciplina já
usada para `eval::rules` eager vs multi-passe — divergência aceite com prova, não por
omissão).

- A favor: zero risco de regressão visual em documentos existentes.
- Contra: continua a divergir do vanilla, agora sabendo-se disso — deixa de ser bug
  desconhecido para ser decisão consciente, mas a divergência em si permanece.

## Fase C — L0 e critérios de verificação (depois do gate escolhido)

```
Dado cases/matrix com grelha de altura pequena (~14pt)
Quando renderizado
Então a folga bate com [opção escolhida], medida contra o vanilla confirmado

Dado cases/matrix com grelha de altura grande (~50pt)
Quando renderizado
Então a folga [opção 1: converge para short-fall absoluto; opção 2: mantém proporção,
  com a divergência documentada e aceite]
```

Não-regressão: todos os testes existentes de `cases`/`matrix`/delimitadores.

## Fase D — Implementar (só após decisão) e validar

```
crystalline-lint .
cargo test --workspace
```

Se Opção 1: decalque visual do corpus canónico antes/depois, confirmar que as mudanças de
aparência são as esperadas (delimitadores mais próximos do vanilla), não regressões
noutro eixo.

---

## Resultado esperado

Decisão explícita e implementada (uma das duas opções), com a divergência anterior
("aplica a margem de 10% do vanilla") corrigida na fonte partilhada
(`compiler/math/layout/_comum.md`, já feito no P1024) e o L0 dos nós `cases`/`matrix` a
reflectir a decisão final, não só o achado do bug.
