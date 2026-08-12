# Passo 1008 — Triagem: critério-zero nos 5 candidatos restantes do Passo 1003

**Tipo**: Triagem barata — só o critério-zero (agregado ou interface?) + confirmação de
granularidade do fan-in. **Não** aplica os 4 critérios completos do P1002 a nenhum
candidato ainda. Objectivo: filtrar antes de comprometer esforço, não decidir o corte.
**Motivo**: o Passo 1006 mostrou que um candidato bem cotado pelo DSM (`layout::metrics`,
fan-in 68) era, na prática, uma interface (`trait`) com 90 linhas de lógica real — um
fatiamento completo teria descoberto isto só depois de escrito o L0 e medido co-mudança.
Este passo faz essa pergunta primeiro, para todos os candidatos de uma vez, barato.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1007. Caminhos em `compiler::` (pós
Passo 1005).

---

## Candidatos (Fase D do Passo 1003, por ordem original)

1. `compiler::eval::bindings`
2. `compiler::eval::rules`
3. `compiler::eval::closures`
4. `compiler::stdlib::structural`
5. `compiler::stdlib::text`

(`compiler::eval` em si e `compiler::layout::metrics` já foram tratados — P1002 e P1006,
respectivamente. Não repetir.)

---

## Para cada candidato, responder só isto

### A — Critério-zero: agregado ou interface?

```
grep -n '^pub trait \|^trait ' 01_core/src/compiler/eval/bindings.rs \
  01_core/src/compiler/eval/rules.rs 01_core/src/compiler/eval/closures.rs \
  01_core/src/compiler/stdlib/structural.rs 01_core/src/compiler/stdlib/text.rs
```

Se o ficheiro **define** um `trait` público com múltiplos implementadores reais (não só
um blanket/teste) e a maior parte do conteúdo são métodos desse trait: **é interface**,
suspender aqui, registar como "mesma classe de `metrics.rs`", não avançar para B.

Se o ficheiro é um conjunto de `fn`/`pub(crate) fn` livres (mesmo que grande): **é
agregado**, avançar para B.

Se for misto (algumas funções livres, mais um trait pequeno incidental): registar como
misto, avançar para B só sobre a parte agregada, sinalizar a parte de interface à parte.

### B — Confirmar granularidade do fan-in (só se passou o critério-zero como agregado)

Repetir a distinção que o P1006 fez para `metrics`: o fan-in alto do P1003 é de **módulo**
(outros ficheiros importam este módulo) ou de **símbolo** (um tipo/trait/const definido
aqui é usado como bound genérico ou re-exportado, inflacionando a contagem sem reflectir
acoplamento real de módulo)?

```
grep -rn 'use.*compiler::eval::bindings\|super::bindings' 01_core 02_shell 03_infra 04_wiring | wc -l
```
(ajustar por candidato — contar imports directos do módulo, não menções ao símbolo mais
usado lá dentro)

Comparar esta contagem com o fan-in reportado pelo P1003. Se forem próximos: fan-in é de
módulo, sinal real. Se o fan-in do P1003 for muito mais alto do que os imports directos
do módulo: há um símbolo específico a inflacionar — identificar qual, e se esse símbolo é,
ele próprio, um `trait`/tipo cujo corte implicaria gate (mesma armadilha do P1006).

### C — Veredicto por candidato (uma linha)

- **Prosseguir para P1002-completo** — passou A (agregado) e B (fan-in é de módulo, não
  de símbolo inflacionado).
- **Suspender, mesma classe de `metrics.rs`** — falhou A (é interface) ou B revelou que o
  fan-in real é de um trait/tipo, não do módulo.
- **Investigar mais antes de classificar** — resultado ambíguo, registar o que falta.

---

## Output (tabela única, todos os 5 candidatos)

| Candidato | A: agregado/interface | B: fan-in confirma módulo? | Veredicto | Nota |
|---|---|---|---|---|

## O que este passo NÃO faz

- Não aplica os 4 critérios completos a nenhum candidato.
- Não escreve nenhum L0 nem move código.
- Não decide qual candidato tratar primeiro entre os que passarem — só filtra os que não
  vale a pena tentar.

## Resultado esperado

Lista curta (0 a 5) de candidatos que valem um P1002-completo, com os restantes já
descartados ou marcados para investigação futura — sem ter gasto o esforço de um
protótipo inteiro em nenhum deles primeiro.
