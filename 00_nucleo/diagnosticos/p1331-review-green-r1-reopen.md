# P1331 — candidato C1 e reabertura da fixture R1

Revisor `/root/p1331_review`, A/B sem atestação de isolamento, sem refinamento.
Nenhum input julgado foi editado pelo revisor.

## Evidência e causa

`p1331-unit-green.json`, SHA-256
`ff1d4d7dab98404c82f0c9cf0a3f09bf6e6625a543743c80192d80d8cc813fd1`,
registra a working tree não commitada sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, argv, horários e inventários
before/after. O resultado compilado é 29 testes passados e um falhado.
Não é GREEN válido nem fundamento de fechamento.

A falha está no subcaso `#calc.abs(path("p1331.typ"))` do teste de famílias
públicas: o erro vem da construção anterior à chamada. `p1331-ab-tests.rs:33-72`
declara TestWorld sem sobrescrever `World::resolve_path`.
`01_core/src/contracts/world.rs:49-55` documenta resolução sem I/O e define
o default como erro incondicional `cannot access file system from here`.
`01_core/src/compiler/stdlib/foundations/path.rs:54-60` chama esse método
para a string antes de produzir Value::Path. `compiler/eval/mod.rs:348-349`
também delega a resolução ao World original.

Portanto a causa não é simplesmente Source::detached: mudar só a Source
ou seu FileId não altera o default incondicional. Falta à fixture um contexto
virtual de resolução coerente com o caso que pretende testar. O corpus CLI
congelado já demonstra a construção válida de Path; o teste nativo de Path
construído também passa. A observação inválida de R1 é Unknown para abs(Path),
não prova de defeito no fallback nem autorização para mudar o construtor.

O RED R1 interrompeu esse teste denso num tipo anterior e por isso não
alcançou o subcaso Path. A limitação foi registrada no parecer RED, mas o
pré-C não detectou a insuficiência da fixture. Corrigir a cadeia exige
reabrir o primeiro artefato afetado, sem apagar essa limitação histórica.

## Delta C1 revisado

Comparei o owner completo com o pre-C reconstruído. C1 muda exclusivamente
o corpo do braço `[other]`: nomes locais Str/string e Bool/boolean, demais
nomes por type_name, primeiro value_span posicional e diagnóstico Error.
Guards, demais braços, helpers e snippets congelados permanecem iguais.
Hash B sem header: `35e5681ac54d5207d71329d0da0560bfdeda83f0026030474f85e8c100244c7e`.
Norma L0 permanece `ef406128181e4cfa5b802b86ec0a9c58370cc4f7dab09365a088eef74523956a`.
Esse julgamento do delta não torna o resultado R1 aceitável.

## Primeira fase afetada e próximo gate

REOPEN_AB_FIXTURE. A primeira fase afetada é autoria/congelamento da fixture
A/B, não a intenção L0 nem a expectativa CLI. É correta a sequência proposta:

1. Preservar todos os artefatos R1/C1 e seus recibos.
2. Restaurar somente o runtime fallback pré-C, com preimagem comprovada.
3. Autor A/B recebe a insuficiência pública da API World, sem patch de C1
   ou assertions privadas, e prepara fixture R2 com resolução virtual válida.
4. Preservar caso Path, diagnóstico completo e demais asserções. Não aceitar
   erro do construtor, remover Path ou enfraquecer sua obrigação.
5. Congelar R2, integrar cegamente, obter novo RED compilado e só então
   reaplicar candidato, avaliando os mesmos testes R2 em GREEN.

O autor deve demonstrar focalmente que a construção de Path é válida no
contexto da fixture, antes do novo corpus completo. A implementação da API
pública existente de resolução pode ser inteiramente em memória; não exige
I/O nem alteração de produto. A escolha concreta pertence ao autor A/B.

Norma e CLI podem permanecer iguais. Alterar snippet protegido invalida o
freeze R1; o novo freeze deve identificar R2 e a causa desta revisão. Esta é
a primeira correção dessa causa, sem motivo para repetir gates completos
antes da checagem focal. Não há autorização de fechamento ou paridade geral.
