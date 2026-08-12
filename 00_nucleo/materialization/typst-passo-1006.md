# Passo 1006 — Fatiamento hub/nó: `layout::metrics` (fan-in 68, o mais alto de todo o `compiler/`)

**Tipo**: Aplicação do método (validado no Passo 1002) a um segundo caso real.
**Pré-condição**: `git status` limpo. **Depende do Passo 1005 já ter corrido** (este passo
usa os caminhos `compiler::` — se o rename ainda não aconteceu, ajustar todos os caminhos
abaixo de `compiler::layout::metrics` para `engine::layout::metrics` e
`01_core/src/compiler/` para `01_core/src/engine/`, e sinalizar a inversão de ordem no
relatório).
**Correcções ao método, herdadas do Passo 1002 (aplicar desde já, não redescobrir)**:
1. O inventário de funções tem de incluir `pub(crate) fn`, não só `pub fn`/`fn` de topo —
   o grep do Passo 1000 escondeu 4 de 13 funções em `operators.rs` por este motivo.
2. `V15` exige um `@prompt` por ficheiro — cada nó implica sempre um ficheiro `.rs`
   próprio, não é opcional.
3. O critério de co-mudança histórica (função ×  commit) é o mais informativo dos 4 —
   priorizar tempo nele.
4. Os critérios de isolamento de teste e pureza/estado **não discriminam** em ficheiro
   totalmente declarativo (foi o caso de `operators.rs`) — mas `layout::metrics` pode não
   ser totalmente declarativo (é layout, mede coisas). Confirmar por leitura, não
   presumir que vai repetir o padrão de `operators.rs`.

---

## Fase A — Confirmar por leitura, com os 4 critérios (mais o histórico corrigido)

1. Inventário completo de `compiler/layout/metrics.rs` — todas as funções, incluindo
   `pub(crate)`/privadas de topo de ficheiro:
   ```
   grep -n '^pub fn \|^fn \|^pub(crate) fn ' 01_core/src/compiler/layout/metrics.rs
   ```
2. Ler o ficheiro na íntegra — o Passo 1000 não detalhou o conteúdo interno de
   `metrics.rs`, só o tamanho e que é hub. Descobrir do zero que tipos/funções vivem lá
   (é de esperar algo como `FontMetrics`, cálculo de `em`/baseline/ascent-descent, mas
   confirmar, não presumir a partir do nome).
3. Critério 2 (pureza vs estado) — para cada função: toca `EvalContext`/`Engine<'a>`
   (contexto), ou é `Value`/medida → medida, sem estado externo? **Não presumir que é
   tudo declarativo como `operators.rs`** — medir função a função.
4. Critério 3 (co-mudança histórica) — mesma técnica do Passo 1002 (hash do corpo de cada
   função por commit, atravessando qualquer rename relevante incluindo o do Passo 1005).
   Este é o critério que deve dominar a decisão de corte, per a avaliação do método em
   P1002.
5. Critério 4 (vanilla) — já temos do Passo 1000/1003: o equivalente vanilla é a família
   `typst_library::layout::{length, abs, rel, em, axes, ratio}`, fragmentada em 6
   ficheiros pequenos. Usar como candidato inicial de fronteira, mas **confirmar com o
   critério 3** antes de aceitar — o Passo 1002 mostrou que a divisão "óbvia" por
   inspecção (a `coercion.md` hipotética) pode falhar quando confrontada com co-mudança
   real.
6. Explicitamente verificar: porque é que este módulo tem fan-in 68 (o mais alto de todo
   `compiler/`, mais que `eval` inteiro) — que módulos dependem dele e porquê. Isto pode
   revelar se o corte certo é "por tipo de medida" (paridade vanilla) ou "por quem
   consome" (um eixo diferente, se o padrão de consumo não bater com a divisão por tipo).

## Fase B — Materializar (só depois da Fase A fechar)

1. Hub `compiler/layout/metrics.md` — tabela de despacho, zero lógica.
2. Nós — número e fronteira decididos pela Fase A, não pré-definidos aqui (ao contrário
   do Passo 1002, onde a hipótese inicial de 5 nós era um bom ponto de partida; aqui não
   temos hipótese prévia validada, só o candidato vanilla de 6 ficheiros — tratar como
   ponto de partida a confirmar, não como plano fechado).
3. Cada nó: prompt L0 (sem referência a passo, regra já em vigor) + ficheiro `.rs` próprio
   (V15 obrigatório). Preencher o campo **Técnica** do template actualizado, se houver
   alguma técnica de CC nomeável por trás do mecanismo (ex.: se houver conversão de
   unidades com aritmética de ponto fixo/flutuante específica, ou normalização, nomear).
4. Preservação de comportamento: código cortado e colado, não reescrito, mesma disciplina
   do Passo 1002.

## Fase C — Validar

```bash
crystalline-lint .
cargo test --workspace
```
Zero regressão esperada. Comparar contagem de testes antes/depois.

## Fase D — Avaliação do método (mesma exigência do Passo 1002)

Registar explicitamente: os critérios 1/2 discriminaram desta vez (ou continuaram vácuos,
se `metrics.rs` também for todo declarativo)? O critério 3 confirmou ou corrigiu a divisão
candidata do vanilla? Alguma lacuna nova no método, como a do `pub(crate)` no Passo 1000?

## Resultado esperado

`compiler::layout::metrics` fatiado em hub + N nós, com evidência dos 4 critérios por
nó, código preservado, zero regressão, e uma segunda validação (ou correcção) do método
antes de o aplicar a `eval.md`/`bindings.rs` (os hubs maiores e mais arriscados, ainda por
tratar).
