# P1428 — relatório final de verificação

## Veredito

**PASS**, executado sem atestação de isolamento.

O certificado é proporcional ao fragmento selado: a recalibração dos
observadores públicos de `math.frac` passou integralmente, sem qualquer mudança
produtiva desde o candidato P1427. O resultado não atesta isolamento do
workspace nem equivalência funcional geral.

## Integridade e proveniência

A medição foi concluída em `2026-09-16T19:51:05-03:00`, sobre HEAD limpo
`d626d530b86d92e4001203b5d0e3267cd85b86c2`, árvore
`9ecbfe69bc9b8ba646e2648daeb56ed535787365`.

O passo, manifesto, contrato, oráculos, mutantes, recibos de mutação e ordem,
RED, L0/teste independente, preseal R3 e selo coincidem com os hashes
congelados. A linhagem do teste P1428 coincide em `a8451345`. Os 12 ficheiros
produtivos listados no manifesto são byte-idênticos aos objetos do commit
`22d7795d48786e590e12c1d6c02280673c657a8c`; o teste público P1427 também
permanece byte-idêntico (`708a39bc…`).

## Ordens e classificações

Três processos novos executaram o runner exato de 17 casos:

- `forward`: 13 `Preserved` e quatro `Unknown` opacos;
- `forward_repeat`: o mesmo vetor por ID;
- `reverse`: o inverso exato, com o mesmo vetor por ID.

Nenhum caso obrigatório ficou `Unknown`. Apenas U02–U05, declarados para testar
opacidade, permaneceram `Unknown`, sempre com crédito zero. O conjunct causal de
layout P1427, `p1427_horizontal_restaura_protecao_causal`, aprovou 1/1.

## Mutações

M01–M12 foram executados em 12 processos novos. Todos terminaram com a falha
atribuível esperada (`exit 101`), sem sobrevivente e sem `Unknown`. Portanto,
para os doze ataques congelados do observador/classificador,
`mutation_score = 12/12 = 1.0`.

O recibo dedicado preserva a testemunha de cada mutante. Esse score não é
generalizado para fora da fronteira P1428.

## Gates finais

- `cargo fmt --all -- --check`: PASS;
- `cargo build`: PASS, com avisos preexistentes não bloqueantes;
- `crystalline-lint .`: PASS (`exit 0`; avisos e informações não bloqueantes,
  com V5/V15/V26 livres);
- `git diff --check HEAD`: PASS antes dos recibos;
- `git status --short`: vazio antes dos recibos;
- allowlist, hashes, linhagem e ausência de mudanças produtivas: PASS.

## Limite da afirmação

O PASS atesta somente a transformação registrada: texto semântico de `repr`
após uma decodificação de entidades, extração de um único `MathFrac`, vetor
exato dos quatro bits, conjunct com o oracle horizontal P1427 e preservação dos
controles de rota, diagnóstico, constantes OpenType e `binom`. Não afirma
correção geral do backend SVG, paridade matemática geral, igualdade de bytes de
render ou isolamento técnico do ambiente partilhado.
