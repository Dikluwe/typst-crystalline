# ⚖️ ADR-0034: Diagnóstico obrigatório antes de materializar tipo do vanilla

**Status**: `EM VIGOR`
**Data**: 2026-04-22

---

## Contexto

ADR-0033 estabelece paridade funcional com vanilla como invariante
arquitectural. A decisão sobre o que **é** paridade, porém, exige
conhecimento factual do comportamento vanilla. Decisões
arquitecturais tomadas sem verificação produzem um de dois
resultados:

- Divergência acidental não detectada — o cristalino comporta-se
  diferentemente sem intenção.
- Medo de refactor — divergências potenciais são sempre recusadas
  "para segurança", mesmo quando seriam legítimas.

Durante a série 84.x, múltiplos passos incluíram secções de
diagnóstico antes da execução (84.2 para DEBT-38, 84.3 para
DEBT-21, 84.5 para DEBT-36, 84.6 para DEBT-37). Inicialmente,
três ADRs (0022, 0023, 0025) incluíram o diagnóstico **dentro**
do próprio ADR como anti-padrão — o Passo 84.8d refactorou-os,
movendo o diagnóstico para `00_nucleo/diagnosticos/`.

Este ADR formaliza a regra que emergiu: **diagnóstico antes, ADR
depois, diagnóstico persistido separado**.

---

## Decisão

### Regra

**Qualquer passo que materialize em L1 um tipo do vanilla deve,
antes de escrever código, produzir e persistir diagnóstico
estruturado em `00_nucleo/diagnosticos/`.**

Esta regra é **vinculativa**, não sugestão.

### Escopo

"Materializar tipo do vanilla" inclui:
- Criar em L1 um tipo novo correspondente a tipo existente no
  Typst vanilla (ex: criar `Align2D` para corresponder a
  `Alignment` vanilla).
- Adicionar operador ou método significativo a tipo vanilla
  materializado (ex: adicionar `impl Add for Align2D` para
  reproduzir `alignment + alignment`).
- Refactorar tipo materializado de forma que altera semântica
  (forma estrutural nova, novos campos observáveis).

**Não** está no escopo:
- Correcções pontuais de bugs (ex: erro de cálculo num método
  existente).
- Optimizações internas (ex: converter campo `Vec<T>` para
  `Arc<[T]>` — não altera semântica observável).
- Tipos novos sem correspondência no vanilla (ex: tipos auxiliares
  internos da arquitectura cristalina).

### Conteúdo mínimo do diagnóstico

Cada diagnóstico em `00_nucleo/diagnosticos/` deve conter:

1. **Localização do tipo vanilla**: caminho(s) em
   `lab/typst-original/` onde o tipo está definido.
2. **Campos e variantes**: enumeração completa. Se enum, todas as
   variantes; se struct, todos os campos com tipos.
3. **Operadores e métodos públicos**: lista com assinatura.
4. **Dependências**: outros tipos referenciados (para entender
   ordem de materialização).
5. **Semântica de operadores-chave**: comportamento de `+`, `-`,
   `==`, conversões, casts — com exemplos concretos do vanilla.
6. **Mensagens de erro**: texto exacto dos erros que o tipo pode
   produzir no vanilla.
7. **Divergências propostas**: se a materialização vai divergir
   estruturalmente, descrever forma cristalina proposta e
   justificar paridade semântica (link para ADR-0033).

### Convenção de nome do ficheiro

`00_nucleo/diagnosticos/diagnostico-<contexto>-<slug>.md`

Onde `<contexto>` é um de:
- `adr-NNNN` — diagnóstico associado a ADR específico
  (convenção estabelecida no 84.8d para diagnósticos históricos).
- `passo-N.M` — diagnóstico associado a passo de materialização
  (convenção sugerida para futuros).
- `tipo-<nome>` — diagnóstico standalone sobre tipo vanilla
  (quando o diagnóstico precede a decisão de ADR ou passo).

Exemplos:
- `diagnostico-adr-0022-fontbook.md` (histórico, 84.8d).
- `diagnostico-passo-84.5-alignment.md` (convenção sugerida).
- `diagnostico-tipo-gradient.md` (antes de decidir passo).

### Cabeçalho obrigatório

```markdown
# Diagnóstico: <título>

**Tipo vanilla**: <nome do tipo principal>
**Localização vanilla**: `lab/typst-original/<path>`
**Data do diagnóstico**: YYYY-MM-DD
**Contexto**: <ADR ou passo que motivou; "standalone" se nenhum>

**Natureza**: registo factual do estado do vanilla na data acima.
Decisões arquitecturais derivadas deste diagnóstico ficam em
ADR/passo separados. Este ficheiro não contém decisões.
```

---

## Relação com outros ADRs

- **ADR-0033** (paridade vanilla): este ADR é mecanismo que torna
  ADR-0033 operacional. Sem diagnóstico, "paridade" seria
  afirmação sem base factual.

- **Anti-padrão corrigido no 84.8d**: os 3 ADRs (0022, 0023, 0025)
  continham diagnóstico dentro do ADR — era violação desta regra
  (ainda não formalizada na altura). O refactor do 84.8d moveu
  os diagnósticos para o directório canónico e os ADRs agora
  apenas referenciam via linha `**Diagnóstico prévio**: ver ...`.

---

## Consequências

**Positivas**:
- Decisões de paridade baseiam-se em factos documentados, não em
  memória.
- Futuros LLMs que peguem no projecto têm registo claro do que foi
  verificado antes de cada decisão.
- Diagnósticos persistidos permitem auditoria retrospectiva
  (como o Passo 83.6 fez com o 83.5).

**Negativas**:
- Passos de materialização tornam-se mais longos (diagnóstico
  precede execução).
- Pode haver tentação de "saltar o diagnóstico" para tipos
  aparentemente triviais. Resposta: os tipos tipográficos do
  vanilla listados em ADR-0029 **não** são triviais; cada um tem
  semântica particular (combinações, conversões, mensagens de
  erro específicas) que exigem verificação.

---

## Alternativas Consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| "Diagnóstico sugerido, não obrigatório" | Menos overhead para casos simples | Abre porta a "é trivial, não preciso" — volta ao estado pré-84.8d |
| "Diagnóstico dentro do ADR (como estava em 0022/0023/0025)" | Informação no mesmo sítio | Mistura decisão com execução; anti-padrão identificado no 84.7 |
| "Diagnóstico apenas para tipos novos, não para refactors" | Simplifica critério | Refactors são precisamente onde erros de paridade aparecem |
| **Decisão adoptada: obrigatório, persistido em directório canónico, referenciado pelo ADR/passo** | **Força rigor; preserva rastreabilidade; permite auditoria** | **Overhead real em cada materialização** |

---

## Referências

- ADR-0033 — Paridade funcional vanilla (regra que este ADR torna
  operacional)
- Passo 84.8d — Refactor do anti-padrão (criou `00_nucleo/diagnosticos/`)
- `00_nucleo/diagnosticos/` — directório canónico
- `00_nucleo/diagnosticos/diagnostico-adr-0022-fontbook.md`
  (exemplo de diagnóstico histórico)
- `lab/typst-original/` — fonte canónica para verificar
  comportamento vanilla
