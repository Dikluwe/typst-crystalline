# P1315 — revisão do candidato, gates pendentes

Revisor `/root/p1315_review`, inspeção iniciada em
`2026-09-08T13:47:24Z`, HEAD `bc8213f36b7a29b4fdc30cfc74ddc23586117c64`,
working tree não commitado. Nenhuma edição de material verificado.

```text
 00_nucleo/prompts/compiler/stdlib/loading.md | 57 +++++++++++++++++-
 01_core/src/compiler/stdlib/loading.rs       | 86 ++++++++++++++++++++++++----
 2 files changed, 129 insertions(+), 14 deletions(-)
```

Código raw SHA-256
`4e2bfe511360bd09b4d8bf3fd98f3085018a06c1fd4ebd9eb2db58b360fbe208`;
L0 raw `b9c9b5f6f4d7835d75c91d952d45ed1130eb03f4a6504b8be7e8980be0291219`;
L0 normativo `9a67fcb0cd258ba661221b844e59d149061d4b673a52618bc62092054080b40a`,
igual ao freeze. Só a metadata Hash do Código mudou depois do freeze.

O diff produtivo enumera os registros antes de retirar o cabeçalho; assim,
o mesmo index+1 conta o registro rejeitado em array e dictionary. O parser
continua decidindo o que constitui registro; não há contagem manual de
newlines. O parâmetro novo pertence ao mapper privado local e somente o
braço UnequalLengths usa seu valor. O fallback dos demais erros mantém o
mesmo texto com o erro original. X/Y, construtores dos valores e err detached
não mudam.

Conferência automatizada somente leitura com Node crypto/fs e `git show
HEAD:01_core/src/compiler/stdlib/loading.rs` confirmou assinatura pública
decode_csv idêntica e todo o trecho de testes preexistentes, de
`fn p1314_csv_args` até o fim do módulo, intacto. O diff completo confirma
somente os testes locais novos, metadata e mudanças produtivas delimitadas.

Sem achados de código neste recorte. Esta revisão ainda não é PASS final:
depende de GREEN local, build, workspace, A/B congelado normal/repeat/reverse,
linhagem e lint. Mantém regime executado sem atestação de isolamento e não
declara paridade geral CSV.
