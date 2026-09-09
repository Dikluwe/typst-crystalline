# P1328 — auditoria do freeze e integração antes de C

Revisor `/root/p1328_review`; regime A/B executado sem atestação técnica de
isolamento. Entradas recebidas antes de C; este revisor não escreve testes,
expectativas, comparador ou implementação. O RED ainda não estava disponível
ao concluir esta revisão e continua um gate separado.

## Identidade verificada

SHA-256 recalculados e iguais ao freeze:

- Snippet `p1328-ab-tests-r1.rs`:
  `fa971dded3237a1742905db2a45e3d94aef51a853fed17ca8046a0a8ba1cd353`.
- Runner `p1328-ab-cli.py`:
  `326235f263b308a59cb3d50726270ad6c262afe8a3591f1b2c23498fc3b0875d`.
- Expectativas `p1328-ab-cli-expected-r1.json`:
  `9d633ccf379d5a1124723ff1919f388b0603aab8cd4cd940456aa33144369623`.
- Medição válida `p1328-ab-cli-baseline-r1.json`:
  `8f9ddaead847be551b7eb779fbf6719ece3043e84714d872b03c278c9e78db5c`.
- Integração `p1328-test-integration.json`:
  `7fad50c6f00585775501b573efcafb84a21f55cadf0683bc1d7505b82eb50218`.

A integração registra UTC `2026-09-09T11:44:26.925771+00:00`, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado,
diff/stat e inventários integrais antes/depois. Fonte calc integrada
SHA-256 `d7c753a83243caa6e5eed929698b9cda4c54277c05c20a8e82cb2924ce22d1e7`.
Leitura atual confirmou esse hash e suffix exatamente igual ao snippet.
Removendo somente esse suffix e neutralizando a linha de linhagem
`@prompt-hash`, o prefixo é exatamente `original_owner` do baseline
canônico `p1328-baseline.json` SHA-256
`e049418db46ea039235b336254bfdbffd492253ac66c38782ac6bc56564f2ffb`.
Assim, a fonte auditada contém testes e resselo, sem mudança funcional C.

## Comparações e cobertura do artefato

Script de inspeção Node em leitura verificou cada expectativa contra a
medição válida identificada acima e a política pré-C de trace externo.
Há 112 chaves distintas caso/perfil, exatamente as 112 medições: 44 de
correção Content, 12 de correção com dívida do nome externo preservada,
12 de paridade numérica e 44 de dívida preservada. Nenhuma expectativa
diverge da construção normativa previamente registrada. Esses números
descrevem artefatos congelados, não execução de candidato ou cobertura geral.

O ramo `--freeze-from` constrói os literais antes de C; substitui somente
o nome do trace externo nos casos explicitamente nomeados, com guard de
ocorrência única no baseline/vanilla. O ramo `--candidate` lê os literais
e compara os campos integrais exit/stdout/stderr, sem normalização da
saída candidata. O corpo completo da observação permanece no recibo.
O verificador final deve validar os hashes e exigir que a lista de falhas
esteja vazia; o exit do runner sozinho não é veredito funcional.

O snippet cobre ambas representações públicas de content, distinção entre
span agregado/argumento/valor, ausência de origem e detached, prioridades
de guard, valores numéricos e saturação preservada, rotas math/markup/alias,
UTF-8/linhas, With/Args spread com origem externa, warnings e rejeições
fora de escopo. As asserções comparam erro completo/severidade/hints/trace
e range resolvido; os testes nativos comparam também a identidade integral
do Span. As construções de origem em fixtures são esperados de teste, não
recuperação de origem em produto.

## Veredito

Freeze e integração coerentes com o L0 e a segregação declarada. Não há
objeção para executar RED contra estes bytes. Compilação/harness inválidos
não satisfazem RED; a falha deve alcançar as asserções semânticas previstas.
O gate RED e a preservação dos hashes ainda precisam ser revisados antes
de C. Não há aprovação de fechamento nesta fase.
