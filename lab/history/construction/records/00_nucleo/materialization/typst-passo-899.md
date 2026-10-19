# Passo 899 — funções matemáticas nativas em falta (acentos, delimitadores, estilos, brackets)

**Precede este passo**: `typst-passo-894-relatorio.md` (achado original, ~26 ocorrências, 7+
secções) e `typst-passo-897-relatorio.md`/PDFs da secção 16 (confirmação visual de `bb`/`cal`/
`frak`/`mono`/`sans` renderizando sem estilo). Ler antes de começar.

**Escopo grande — dividir em sub-partes independentes, cada uma com o próprio ciclo TDD, para não
ficar um diff gigante difícil de rever.**

**Pré-condição de árvore**: `git status`. P893 continua parado no gate. P898 — confirmar se já
commitado.

---

## Catálogo (confirmar cada item na Fase A antes de implementar — alguns podem já existir com nome
diferente do esperado)

### Parte A — funções de acento
`hat(x)`, `tilde(x)`, `bar(x)`, `dot(x)`, `dot.double(x)` (nota: `dot.double` como **modificador de
símbolo solto** já foi corrigido em P895 — isto aqui é `dot(...)` como **função de acento**,
mecanismo diferente, já registado como distinto em P895 "item catalogado, não corrigido").

### Parte B — funções de delimitador/bracket
`abs(x)` → `|x|`, `norm(x)` → `‖x‖`, `floor(x)` → `⌊x⌋`, `ceil(x)` → `⌈x⌉`, `round(x)` → `⌊x⌉`.

### Parte C — funções "big" de chave
`underbrace(content, label)`, `overbrace(content, label)`, `underbracket(content)`,
`overbracket(content)`.

### Parte D — `binom`
`binom(n, k)` → coeficiente binomial com parênteses.

### Parte E — construtores de estilo de fonte em modo matemático
`bb(x)` (blackboard bold), `cal(x)` (caligráfico), `frak(x)` (fraktur), `mono(x)`, `sans(x)`.
Confirmado por P897 (secção 16): compilam sem erro mas devolvem o conteúdo **sem nenhum estilo
aplicado** (não é "função em falta" no sentido de erro de compilação, é "função presente mas
no-op").

## Fase A — diagnóstico por parte (confirmar antes de implementar cada uma)

1. Para as Partes A-D: confirmar se a função já existe com outro nome, ou existe mas com assinatura
   diferente do vanilla, antes de assumir "está totalmente ausente" — grep em
   `01_core/src/engine/stdlib/` e `01_core/src/rules/math/`. P895 já encontrou um caso deste tipo
   (`sect` existia com nome errado, deveria ser `inter`) — não presumir que todos os 26 casos são
   "adicionar do zero".
2. Para a Parte E: já se sabe que a função existe e compila — confirmar onde exatamente o estilo
   deveria ser aplicado (provavelmente troca de variante de glifo Unicode, mesmo mecanismo usado
   para itálico matemático automático de identificadores — `GlyphVariants`/mapeamento de estilo já
   mencionado em passos anteriores desta frente) e por que está a ser ignorado — é um `match` sem
   braço para estes casos, ou os casos existem mas devolvem o glifo base sem transformação?
3. Ler como o vanilla implementa cada função (`lab/typst-original/`) antes de replicar — confirmar
   assinatura, comportamento com argumentos inválidos/ausentes, e casos especiais (por exemplo,
   `bb`/`cal`/etc podem não ter variante Unicode para todo carácter — confirmar o que o vanilla faz
   quando não há glifo estilizado disponível).
4. Confirmar, para cada função, se é implementável como entrada de tabela simples (como os itens de
   P895) ou se precisa de lógica de layout própria (como as funções de acento, que têm de posicionar
   o símbolo sobre/sob o conteúdo, e as funções "big", que têm de desenhar a chave/colchete
   dimensionado ao conteúdo) — separar as duas categorias explicitamente no relatório, já que têm
   risco/esforço muito diferentes.

## Fase B — Implementação (protocolo de dois agentes de P898 para qualquer parte com geometria/
layout — acentos, big-brackets; TDD directo, sem necessidade de dois agentes, para as partes que
são só mapeamento de tabela — delimitadores simples, estilos de fonte, `binom` se for só composição
de fração+delimitador já existentes)

1. Implementar parte por parte (A, depois B, depois C, depois D, depois E — ou a ordem que fizer
   mais sentido dado o que a Fase A encontrar), cada uma com o próprio commit/teste, para poder
   isolar regressão se algo der errado.
2. Teste(s) por parte, ground-truth onde aplicável (mesmo padrão de P891/892/893 — não hardcoded
   quando o valor vem de uma tabela de fonte).
3. Suíte completa verde depois de cada parte, discriminada por crate.
4. Recompilar as secções relevantes do `.typ` de 30 secções (10 para acentos, 5-6-7 para
   delimitadores/binom, 16 para estilos, hash confirmado igual aos passos anteriores) e confirmar
   visualmente cada parte implementada.
5. `cargo run -- .` — zero violations, a cada parte.

## Fase C — Regressão

Uma vez no fim (depois de todas as partes implementadas), não a cada parte — benchmark completo, 7
cenários, `--warmup 5 -m 20` (ajuste que P898 já confirmou eliminar o falso-alarme de leitura
inicial).

## Resultado esperado

- Relatório por parte (A-E), cada uma com: veredicto da Fase A (existia com outro nome? mapeamento
  simples ou layout próprio?), implementação, testes, confirmação visual.
- Se alguma parte revelar-se muito maior que o esperado durante a Fase A (por exemplo, big-brackets
  precisarem de um mecanismo de dimensionamento que não existe para nenhum outro elemento ainda):
  registar isso explicitamente e considerar destacar essa parte para um passo totalmente separado,
  em vez de forçar caber aqui.
- Benchmark completo no fim.
