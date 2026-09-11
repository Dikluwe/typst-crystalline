# P1342 — reemissão R4 dos oráculos para o contrato R3

Regime: **executado sem atestacao de isolamento**. Papel: autor independente
de oráculos, sem leitura de candidato P1342. Veredito:
**ORACLE_REISSUED_NOT_VERIFIED_NOT_SEALED**.

R1–R3 permanecem intactos como histórico. O contrato/binding R3 tornou a cadeia
anterior inapta para julgar um candidato e esta R4 a recompõe sem editar
contrato, L0, código, testes, adversário ou pré-selo.

## Composição e trust boundary

O checker R4 carrega os checkers R3/R2 congelados e conserva todas as
obrigações já fechadas: schema/tipos, 19 rows e 18 hooks aplicáveis, spans,
Dicts ordenados, `SyntaxNode`, challenge/IDs, recibos SHA-256, raw freeze,
carriers, Location/snapshots, attempt-kind/R, ordem causal, endpoints e
precedência de `Unknown`. A01–A21 e X01–X22 continuam casos válidos.

A nova fronteira separa três objetos:

1. DTO e raw runtime, que não podem autenticar source;
2. baseline candidate-free histórico, referenciado por um pré-selo;
3. evidência pós-patch externa, recebida por path+SHA-256 controlados pelo
   verificador.

O checker rehasha os bytes do fixture candidato exclusivamente contra
`candidate_file_sha256` externo. Não compara o hash live com
`baseline_file_sha256`. O baseline só é ligado à atestação histórica pinada.
Depois valida as 19 resoluções Rust-aware, limites cfg-only, igualdade fora da
união, writer único, projeção read-only e coverage conectado ao raw.

Como ainda não existe candidato nem pré-selo R3 reemitido, o corpus não usa
arquivos produtivos live. Ele materializa source, baseline, preseal e evidence
como fixtures sintéticos, fechados e independentes. Esses fixtures calibram o
predicado; não atestam checkout, candidato ou baseline reais.

## Controles Y

- Y01: hash candidato diferente do baseline, igual ao
  `candidate_file_sha256`, com binding/diff/coverage válidos → `Preserved`.
- Y02: hash candidato igual ao baseline, mas coverage ausente → `Violated` por
  coverage, sem crédito ou rejeição decorrente da igualdade histórica.
- Y03: fixture de checker que compara live ao baseline → `Violated` na
  meta-calibração; o checker R4 não executa essa comparação.
- Y04: evidence disponível apenas no DTO/fachada → `Violated`.
- Y05: path/hash do canal externo adulterado → `Violated`.
- Y06: mudança fora da boundary permitida → `Violated`.

## Calibração e resultado

O primeiro passe focal dos seis Y classificou todos corretamente, mas Y02 foi
interceptado por um range sintético encurtado antes de alcançar a causa
pretendida. Uma revisão focal ajustou somente esse range. O segundo passe focal
obteve Y01 `Preserved` e Y02–Y06 `Violated`, cada qual pela fronteira esperada.
Isso consumiu uma revisão da classe; nenhum vetor sobreviveu duas vezes.

Depois do focal, houve uma única execução completa final. Nos três percursos
`normal`, `repeat` e `reverse`, os 51 casos concordaram: 48/48 negativos
`Violated`, P01 e Y01 `Preserved`, P02 `Unknown`, score `1.0` e nenhum
sobrevivente. O delta discriminatório sobre R3 é cinco novos negativos
rejeitados e um novo controle positivo preservado, sem regressão dos 45 casos
anteriores.

Este é um resultado de autoria/calibração. Não é verificação independente,
pré-selo, autorização de implementação ou certificado, e não fecha P1340,
NT01–NT06, retenção/invalidação, política terminal ou equivalência geral.
