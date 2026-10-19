# L0 — Passo 1084: Levantamento de Candidatos para Retomar Fatiamento

**Gate**: nenhum — levantamento e triagem, sem executar nenhum fatiamento
neste passo.

**Base**: decisão do dono — Caminho 2.2 do P1079 (retomar fatiamento
planeado). Histórico de fatiamento anterior (documento de continuação, Bloco 3):
`eval::rules`, `eval::closures`, `eval::bindings`, `stdlib::structural`,
`stdlib::text`, `stdlib::foundations` já foram fatiados. `layout::metrics` foi
**recusado** (P1006, nasceu o critério-zero: é interface/trait, fatiar exigiria
gate para mover pouco).

---

## 1. Ler o método real antes de aplicar

Tenho só o resumo do método (documento de continuação), não o prompt completo.
Pedir: `00_nucleo/prompts/auditar-fatiamento.md`. Não reaplicar de memória os
critérios — reler antes.

## 2. Levantamento de tamanho — candidatos brutos

```bash
wc -l 01_core/src/compiler/**/*.rs 01_core/src/entities/**/*.rs \
  03_infra/src/**/*.rs 2>/dev/null | sort -rn | head -30
```

Esta conversa já deu sinais indirectos de que `01_core/src/compiler/layout/
mod.rs` é volumoso — dezenas de secções distintas (P250, P762, P785, P812,
P842, P864, P867, P887, P888, P891-908, P987, P992, entre outras) todas
referenciando o mesmo `layout/mod.rs`/`compiler/layout.md`. Isto é sinal
indirecto (densidade de conteúdo do prompt), não medição real de linhas —
confirmar com o `wc -l` acima antes de tratar como candidato.

`cursor.rs` já confirmado nesta conversa como grande (1119 linhas, visto
directamente num upload anterior) — candidato a considerar, mas já rejeitado
uma vez implicitamente por nunca ter tido um `.md` próprio (P1060/1062, decisão
já tomada: manter nomes, não fatiar por causa de colisão de nome — questão
diferente de fatiamento por tamanho, não misturar as duas decisões).

## 3. Aplicar critério-zero a cada candidato do levantamento

Antes de qualquer inventário de visibilidade: **é agregado (funções livres) ou
interface (trait)?** Se for interface, fatiar exige gate para mover pouco —
normalmente não compensa (precedente directo: P1006, `layout::metrics`
recusado por este exato motivo).

`layout/mod.rs`, por tudo que se viu nesta conversa, parece ser
predominantemente uma colecção de funções livres do `Layouter` (métodos
`impl<M, S> Layouter<M, S>`, não um trait) — mas confirmar isto lendo o
arquivo real antes de assumir que passa no critério-zero.

## 4. Não recomeçar do zero — cruzar com o que já foi decidido

Antes de propor qualquer candidato como "novo", verificar se já foi avaliado e
recusado antes (como `layout::metrics` em P1006) — evitar reabrir uma decisão
já tomada sem motivo novo.

## 5. Critérios de verificação deste passo

1. Lista real de tamanho por arquivo (não estimativa).
2. Prompt do método lido, critérios citados correctamente.
3. Critério-zero aplicado a pelo menos os 5 maiores candidatos do
   levantamento — classificado agregado/interface para cada um.
4. Nenhum candidato já recusado antes (P1006 ou outro) reproposto sem motivo
   novo.

## Critério de conclusão

- Levantamento de tamanho real obtido.
- Lista curta de candidatos (agregados, não interfaces) que passam o
  critério-zero — prontos para a próxima fase do método (inventário de
  visibilidade, co-mudança histórica), não decididos ainda.
- Nenhum fatiamento executado neste passo.
