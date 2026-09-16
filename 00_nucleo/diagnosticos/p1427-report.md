# P1427 — relatório final de verificação

## Veredito

**BLOCKED**, executado sem atestação de isolamento.

O candidato passa integralmente os oráculos independentes de L1 e os gates
arquiteturais, mas o protocolo não permite um certificado PASS enquanto quatro
casos públicos obrigatórios permanecem `Unknown`. A suíte pública selada também
continua vermelha em duas asserções nas três ordens exigidas.

## Integridade e escopo

A medição final foi feita em `2026-09-16T18:37:30-03:00`, sobre HEAD
`5686fdeb09ee3da33bbcb3b54ab009a4a55a2be5`, com candidato não commitado.
Manifesto, contrato, oráculos, mutantes, recibo RED, selo, quatro L0s de teste e
quatro testes independentes permaneceram byte-idênticos aos hashes selados.

A allowlist final está correta: os 12 consumers produtivos declarados estão
presentes, sem consumer extra. O diff candidato tem 23 caminhos: 12 consumers e
11 L0s proprietários cujo único delta é o `Hash do Código`; o owner de
`math/layout/tests.rs` permaneceu byte-idêntico e o consumer recebeu apenas o
reparo mecânico do hash de linhagem. `crystalline-lint .` confirmou a relação
1:1 e deixou V5, V15 e V26 livres.

## Execução nas três ordens

Foram executados processos novos em `forward`, `forward_repeat` e `reverse`.
Em todas as ordens, `typst-core` aprovou 15/15 testes P1427. A suíte
`typst-infra --test p1427_math_frac` repetiu exatamente 4 aprovações e 2 falhas.

O vetor obrigatório auditável ficou em 35 `Preserved`, zero `Violated` e quatro
`Unknown`: P13, P14, P15 e P21. `Unknown` não recebeu crédito.

## Investigação das duas falhas públicas

A falha de `repr` não demonstra divergência semântica do candidato. O HTML
serializado contém `style: &quot;vertical&quot;`, cujo texto decodificado é exatamente
`style: "vertical"`; o oracle selado procura aspas cruas dentro da serialização
HTML. O oracle direto de L1, também selado e byte-idêntico, aprova as três formas
`vertical`, `skewed` e `horizontal`. Mesmo assim, a asserção pública congelada
falha e o gate de testes não pode ser declarado verde.

A falha causal é opaca de modo material. As quatro fixtures exatas — sem bits,
bit do numerador, bit do denominador e ambos — geram o mesmo SVG, contendo só o
fundo e um grupo traduzido vazio. Os testes diretos de layout distinguem as
quatro morfologias, mas a saída pública registrada não expõe nenhum item que
permita decidir P13, P14, P15 ou P21. Pela política selada, esses casos são
`Unknown`, não `Preserved`.

## Mutações

As 28 famílias negativas não foram admitidas para pontuação final. Depois de a
mesma falha obrigatória ocorrer nas três ordens, prosseguir não poderia produzir
um certificado válido e violaria a regra de não converter opacidade em kill.
Assim, o recibo registra 28 `Unknown`, `mutation_score = null` e nenhum crédito.
O score exigido continua 1.0.

## Gates finais

- `cargo test -p typst-core p1427 -- --nocapture`: 15 aprovados em cada ordem;
- `cargo test -p typst-infra --test p1427_math_frac -- --nocapture`: 4 aprovados
  e 2 falhos em cada ordem;
- `cargo fmt --all -- --check`: PASS;
- `cargo build`: PASS;
- `crystalline-lint .`: PASS, apenas avisos/informações não bloqueantes;
- `git diff --check HEAD`: PASS;
- allowlist, hashes selados e corpos dos testes: PASS.

## Condição para reabrir

A autoridade de obrigação precisa revisar e ressellar a fronteira pública:
decodificar o texto HTML antes de comparar `repr` e usar uma fixture/extrator SVG
que materialize de fato o conteúdo matemático. Como isso altera oráculos
protegidos, não é um reparo que o verificador possa fazer silenciosamente.

Este veredito cobre somente o fragmento P1427. Não alega divergência funcional
geral, não invalida a evidência L1 aprovada e não prova paridade matemática fora
dos observáveis congelados.
