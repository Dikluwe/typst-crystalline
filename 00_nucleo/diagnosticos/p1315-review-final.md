# P1315 — PASS restrito ao recorte congelado

Revisor `/root/p1315_review`. Veredito: **PASS** para a correção do ordinal
N de UnequalLengths CSV e as preservações congeladas. Nenhum achado pendente
no diff ou nas evidências desse recorte. Regime A/B executado sem atestação
de isolamento técnico, sem selo de refinamento ou alegação de paridade geral.

## Proveniência

Auditoria final em `2026-09-08T13:54:43.960Z`, HEAD
`bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, working tree não commitado:

```text
 00_nucleo/prompts/compiler/stdlib/loading.md | 57 +++++++++++++++++-
 01_core/src/compiler/stdlib/loading.rs       | 86 ++++++++++++++++++++++++----
 2 files changed, 129 insertions(+), 14 deletions(-)
```

Código SHA-256 `4e2bfe511360bd09b4d8bf3fd98f3085018a06c1fd4ebd9eb2db58b360fbe208`;
L0 raw `b9c9b5f6f4d7835d75c91d952d45ed1130eb03f4a6504b8be7e8980be0291219`;
L0 normativo `9a67fcb0cd258ba661221b844e59d149061d4b673a52618bc62092054080b40a`.
Candidato `/dev/shm/p1315-target.V6TWEF/release/typst`, SHA-256
`6a4a75787060ce8015ebde85ef2deb078f4b6a0b827b5533602785261ba890a1`.

O recibo independente `p1315-review-final-audit.json` tem SHA-256
`72931346521506bda1f719d34a74306e0765ec420828720c403ef6294d10b813`.
Ele contém UTC, HEAD, diff/stat, hashes integrais de todos os gates,
comandos, instantes antes/depois e resultados recontados. Reproduzir com
`node 00_nucleo/diagnosticos/p1315-review-audit.cjs` na raiz, em ambiente que
acesse os executáveis e fixtures host. Script SHA-256
`66c8a770750b5f26372ed750884f0c9c2db35e4c3ed3e8d4574e26375fd48f75`.
O script é somente leitura; não executa novamente compilação ou corpus.

## Base da decisão

A cadeia anterior ao candidato está registrada em `p1315-review-preflight.md`
e `p1315-review-prepatch.md`: fonte vanilla ratificada medida, L0 atualizado
antes do patch, RED real, freeze independente e GO anterior ao candidato.
O diff foi revisado em `p1315-review-candidate.md`: enumeração antes do consumo
do cabeçalho, parâmetro privado usado somente por UnequalLengths, assinatura
pública e testes preexistentes intactos. A classificação permanece correção
interna de paridade em fluxo contínuo ADR-0127.

Na auditoria final, todos os inputs do freeze, fixtures, baseline e vanilla
tiveram hashes revalidados. O L0 normativo permaneceu idêntico. Todos os
recibos de gates conservam os hashes atuais de código/L0 antes/depois e
apontam para o mesmo recorder. A identidade do executável candidato foi
verificada diretamente no namespace host.

| Gate | Resultado auditado |
|---|---|
| GREEN local | 3 passaram, 0 falharam |
| Build workspace release | exit 0 |
| Testes workspace release | 6648 passaram, 0 falharam, 3 ignorados |
| Lint | 0 erros, 240 warnings, 1137 infos; exit 0 |
| Linhagem dry-run | Nothing to fix |
| fmt e diff-check | exit 0 |
| A/B | 3480 comparações completas, 0 divergências, 0 Unknown |

A contagem workspace foi recomposta de todos os resumos do stdout, incluindo
os doctests ignorados; não foram somados apenas resultados selecionados.
A contagem do lint conserva os avisos e informações, sem chamar isso de
ausência total de diagnósticos.

As 3480 observações A/B foram confrontadas novamente com o freeze, sem
confiar somente no status PASS do comparator. Foram conferidos identidade
do candidato, argv integral com flags de perfil, cwd, produto, chaves únicas,
cobertura completa de normal/repeat/reverse e exit/stdout/stderr integrais.
O resultado foi zero divergências. São 1160 expectativas sobre 290 casos,
repetidas em três ordens, não 3480 expressões distintas.

Freeze SHA-256 `c2cd0c90e4ea09082980b7dba5ed960dc8c9415bc4930fbeb6c68c2566ddac4f`;
runs `7091b03795a741fd6c2120506e8322b6b871c43b8d91a76f30e8f920f96efc54`;
comparison `2ee04e8e626bb8d8032cbf3df66dbf4470e12096254184ff5de32fd1a164046d`.
As 112 expectativas RED tornam-se GREEN. Na revisão pré-patch já se
conferiu que cada expectativa ordinal conserva o baseline integral mudando
apenas N e coincide com o fragmento vanilla; históricos preservam P1314
literalmente. A calibração UTF-8 antecedeu o candidato e não alterou intenção.

O relatório `p1315-final-report.md`, versão SHA-256
`f77016fb65744e03405c08988cbfb6623a7e01c6a9812ef53983b9e7e29f9951`,
foi lido antes deste parecer; seus resultados e limites correspondem às
evidências auditadas. O recibo `p1315-ab-receipt.md` também foi lido. Somente
o fechamento documental para incorporar este PASS permanece ao coordenador.

## Limites e autoridade

O revisor não escreveu contrato, código, testes locais ou oráculos, nem
corrigiu artefatos julgados. Escreveu apenas `p1315-review-*`. Sua capacidade
de leitura abrangeu os artefatos de revisão e código; o filesystem compartilhado
não impõe isolamento técnico. O PASS não atesta essa propriedade.

O recorte não corrige formato geral de diagnósticos CSV, sufixos físicos,
spans vanilla, UTF-8, coerção Symbol ou demais dívidas fora do escopo. Os
controles mostram preservação nas entradas congeladas, sem provar equivalência
universal, determinismo universal ou novo selo dos corpora históricos.
Atualização posterior de código, L0 normativo ou entrada protegida exige
reabrir a parte afetada da verificação; mero fechamento do relatório não muda
o objeto deste parecer.
