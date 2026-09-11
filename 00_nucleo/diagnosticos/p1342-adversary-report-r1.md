# P1342 — ataque adversarial independente ao contrato/oráculo R1

Regime: **executado sem atestacao de isolamento**. Papel: adversário
independente, sem acesso a candidato futuro P1342. Veredito:
**BLOCKER_NOT_SEALED**.

Este parecer cobre somente o poder discriminatório do fragmento focal de
binding real P1342. Não é selo, autorização de implementação, certificado
final, fecho lifecycle/profile, NT01–NT06 ou política terminal.

## Resultado mensurado

O checker R1 primeiro reproduziu o próprio corpus: 23 casos, dois positivos
`Preserved`, vinte negativos `Violated`, um opaco `Unknown`, com agreement
verdadeiro. Isso confirma a execução do baseline, mas não mede ataques novos.

O harness adversarial aplicou 36 mutações negativas válidas a cópias em memória
do DTO/manifesto e chamou, sem edição, `validate_static_manifest` e
`validate_dto` do checker pinado. Resultado idêntico em `normal`, `repeat` e
`reverse`:

- 16/36 rejeitadas como `Violated`;
- 19/36 sobreviveram como `Preserved`;
- 1/36 sobreviveu como `Unknown`;
- mutation score: **0.4444444444444444** (16/36), abaixo de 1.0;
- estabilidade entre ordens: verdadeira.

Estabilidade não salva um classificador que aceita negativos. Uma única
mutação válida sobrevivente já impediria o selo; aqui sobreviveram vinte.

## Blockers reproduzíveis

### B01 — binding estático textual não prova alcance

O checker exige hash e uma ocorrência textual da âncora, mas não valida a
semântica de `anchor_scope`, `stable_role`, `species`, `fixture_reach`,
`cardinality_per_attempt`, `baseline` ou `forbidden_bindings`. Assim:

- H06 foi ligado ao braço `"display"`, que a fixture focal não percorre;
- H15 foi ligado ao `else` `Value::None`, impossível para o CodeBlock medido;
- H08 foi ligado a uma função sob `#[cfg(test)]`;
- metadados de papel/espécie/alcance/cardinalidade e baseline foram adulterados;
- um valor inexistente de `anchor_scope` foi aceito.

Todos A01–A05 classificaram `Preserved`. Logo, âncora única e hash não provam
hook produtivo ou alcançado.

### B02 — continuidade causal do carrier e endpoints é parcial

O mesmo carrier não é exigido no dispatch do wrapper, aresta With, dispatch do
inner, dispatch da closure ou entrada/saída do corpo. Mudar o carrier
separadamente em cada um desses pontos (A08–A13) continuou `Preserved`.

Também sobreviveram Location diferente no replay-exit (A14) e snapshot-post
fabricado (A15). O checker só exige que esse snapshot tenha prefixo correto e
seja diferente do pre-snapshot; não prova o snapshot real produzido.

O controle C12, que perde o carrier no walk atualmente comparado, foi
`Violated`. Isso localiza a fraqueza: não é ausência total de verificação, é
uma cadeia incompleta.

### B03 — DTO final não prova append-only nem origem produtiva

A07 regenerou depois do fato todos os IDs de evento, predecessores e ordinais;
o resultado permaneceu `Preserved`. Uma sequência final coerente não prova que
ela foi apendada nos callsites declarados.

A16 inseriu um `dict-produced` extra com papel não pertencente ao modelo
fechado. Ele foi aceito porque o checker valida payload/span somente quando o
papel é exatamente `dict-prebound` ou `dict-witness`. Pela mesma lacuna, A18
aceitou JSON `true` como `Int` nominal e A19 aceitou span trocado num evento
ignorado. A17 mostra o controle: duplicar o papel conhecido `dict-witness` é
rejeitado.

A20 manteve uma cadeia R=1 integralmente declarada pelo DTO e apenas trocou a
tentativa para `discovery`; continuou `Preserved`. Portanto `R` não vem de uma
evidência externa de hook hit: é contado a partir dos próprios eventos que o
DTO afirma conter.

### B04 — identidade runtime não tem prova de proveniência

A06 reutilizou exatamente os mesmos IDs concretos em normal, repeat e reverse,
sem remap, e passou nas três execuções. Prefixos de domínio impedem alguns
aliases, mas não distinguem identidade runtime real de constante fixada. A15 e
A20 reforçam a mesma lacuna para snapshot e R.

### B05 — `Unknown` antes de todos os predicados não-payload

A21 tornou somente o witness opaco, mas também perdeu o carrier na entrada do
corpo. Como esse elo não é verificado, o checker devolveu `Unknown` nas três
ordens. Pela política congelada, carrier perdido deveria ser `Violated` antes
de considerar opacidade. Esse vetor sozinho impede o selo.

## Controles rejeitados corretamente

O checker rejeitou spans protegidos trocados, Dicts protegidos trocados ou
alterados, `syntax-body` falsamente `Func`, alias cross-domain, remoção,
duplicação e reordenação dos eventos obrigatórios, inversão/desconexão do With,
carrier perdido no walk, campo `R` explícito, bool no Int protegido e opacidade
do Dict pré-ligado. A matriz completa, com a mesma classificação nos três
modos, está no JSON deste parecer.

## Reprodução e proveniência

Com HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não
commitada, executar:

```text
python3 -B 00_nucleo/diagnosticos/p1342-adversary-runner-r1.py \
  --checker 00_nucleo/diagnosticos/p1342-oracle-checker-r1.py \
  --contract 00_nucleo/diagnosticos/p1342-contract-r1.json \
  --manifest 00_nucleo/diagnosticos/p1342-contract-binding-manifest-r1.json \
  --corpus 00_nucleo/diagnosticos/p1342-oracle-corpus-r1.json
```

Pins decisivos:

- checker: `99735c9ba6bab67a86bba897f7765796dcaf92c43f9198dd62d0cb06d354ce61`;
- contrato: `de5a23a6053ce9fae7d8a377044bf5a43381bd8fbc5a362dd159bdd33567e783`;
- manifesto: `57abe32b9dd0e09d12235fe26d4f02c75a67c4e837736ac4c08b19174d8598bb`;
- corpus: `880978de128ca3582bfc6e59da0ed8753734a5af34687a8f03b8ba28ed0eb3da`;
- harness: `1d5a2ef7cb12b8fe37298edcf5794c68b3ed73b3741fa0d098e3fbe42ce8d236`.

Medição em `2026-09-10T21:12:25Z`: SHA-256 de
`git status --porcelain=v1 -z` =
`c2f6c9f94a9b72644df73854104152b27f250803436f5e3d4a66d24c616d3344`;
SHA-256 de `git diff --binary HEAD` =
`ff4dfff0c1401e48855b4116c5e11ce895e4c21fafc9977d8af5c218f6338a86`;
stat = 72 ficheiros, 10881 inserções, 830 remoções.

## Decisão de parada

O gate exige score 1.0. Como há negativos `Preserved` e um negativo `Unknown`,
o resultado obrigatório é **BLOCKER_NOT_SEALED**. Este papel não corrige o
checker julgado; devolve os vetores às autoridades de contrato/oráculo. Nenhum
candidato futuro foi lido, nenhum artefacto julgado foi alterado e nenhuma
alegação de isolamento técnico é feita.
