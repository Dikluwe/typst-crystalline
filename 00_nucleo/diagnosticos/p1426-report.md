# P1426 — relatório final de verificação

## Veredito

**PASS**, executado sem atestação de isolamento.

O candidato materializa somente o fragmento selado de `math.root`: binding
qualificado, assinatura posicional, três estados causais do índice, convergência
com a sintaxe bare, walkers, relação observacional, `repr`, reconstrução e
inércia de layout. Nenhum achado bloqueante permaneceu.

## Estado causal e integridade

A verificação foi medida em `2026-09-16T17:18:27-03:00`, sobre HEAD
`a9897753233fdfd4f9bce01da71bf47b335b7864` e tree comprometida
`9e682b911c76da6d1969040eb04e744d706d07d2`, com candidato não commitado de
16 caminhos: oito consumers produtivos e os oito L0s proprietários.

Manifesto, contrato, oráculos, mutantes, recibo RED, selo, quatro testes
independentes e quatro L0s de teste permaneceram byte-idênticos aos hashes do
selo R1. Os oito L0s proprietários diferem do estado selado exclusivamente na
linha gerada `Hash do Código`, exceção explicitamente autorizada. O linter
confirmou a linhagem final e deixou V5, V15 e V26 livres.

Os 14 callsites antigos de `Content::math_root` na suíte de layout continuam
presentes; o reparo foi restrito ao terceiro argumento causal e à expansão de
formatação correspondente.

## Execução independente

Foram executadas três ordens em processos novos:

1. `forward`: núcleo e depois integração;
2. `forward_repeat`: repetição da ordem anterior;
3. `reverse`: integração e depois núcleo.

Cada ordem produziu o mesmo vetor: 12/12 testes focais de `typst-core` e 4/4
testes públicos de `typst-infra` aprovados. Assim, P01–P13 e N01–N08 foram
classificados `Preserved` sem `Unknown` obrigatório.

Os controles opacos U01–U04 permaneceram `Unknown`, como exigido, e receberam
zero crédito. Nenhum `Unknown` foi promovido a sucesso.

## Poder discriminatório

As 18 famílias de mutação canônicas foram auditadas contra testemunhas
executáveis seladas e rejeitadas nas três ordens. Resultado: 18/18
`Violated`, zero sobreviventes, zero `Unknown` entre mutações válidas e
`mutation_score = 1.0`.

O resultado cobre separadamente ausência do binding, alias com `sqrt`, aridade,
casts, named arguments, os três estados do índice, ambas as rotas, as três
formas de `repr`, igualdade/hash/walkers, `ObservationEq`, duas reconstruções,
inércia geométrica e coerção de radicando `none`.

## Gates finais

- `cargo test -p typst-core p1426_ -- --nocapture`: 12 aprovados em cada ordem;
- `cargo test -p typst-infra --test p1426_math_root -- --nocapture`: 4 aprovados em cada ordem;
- regressão `layout_root`: 6 aprovados;
- regressão `sqrt`: 21 aprovados;
- `cargo fmt --all -- --check`: PASS;
- `cargo build`: PASS;
- `crystalline-lint .`: PASS, somente informações preexistentes;
- `git diff --check HEAD`: PASS.

## Limite da alegação

Este PASS atesta somente o fragmento P1426 e as 18 famílias de mutação
congeladas. Não prova paridade matemática geral, igualdade de bytes de render,
mudança de geometria de radicais nem as outras 20 rotas `math.*` residuais.
Como o workspace foi compartilhado e as capacidades foram aplicadas por
procedimento, a formulação correta permanece **executado sem atestação de
isolamento**.
