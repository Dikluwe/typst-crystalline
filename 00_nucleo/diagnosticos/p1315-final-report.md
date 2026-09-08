# P1315 — contar registros no erro CSV, não linhas físicas

Estado: implementado e validado — PASS restrito ao P1315, em 2026-09-08.

## Efeito para quem usa

Considere este CSV: o primeiro campo contém uma quebra de linha entre aspas,
e o registro seguinte contém apenas um campo:

```csv
"a
b",c
1
```

O erro anterior dizia `found 1 instead of 2 fields in line 3`.
O recorte P1315 exige `line 2`: trata-se do segundo registro, ainda que
comece na terceira linha física. A mesma regra vale para array e dictionary,
contando o cabeçalho e ignorando linhas vazias que o parser não retorna.
Não há mudança nos dados decodificados nem na rigidez do número de campos.

Isso **não** fecha o diagnóstico CSV completo: o vanilla também informa
`at 3:1` e aponta para a fonte; o cristalino conserva neste passo o formato
e as origens anteriores. Erros UTF-8, coerção Symbol, named desconhecido,
missing/excesso, I/O e ausência de csv.encode continuam fora deste recorte.
Não se deve apresentar igualdade do fragmento como igualdade dos produtos.

## Por que esta correção

A fonte ratificada `a51e02804`, em
`lab/typst-original/crates/typst-library/src/loading/csv.rs:54-77,150-153`,
declara explicitamente o workaround: enumerar registros para o número
diagnóstico, separando-o da posição física do parser. O baseline usava
Position.line em `01_core/src/compiler/stdlib/loading.rs:956-963`.

Medição anterior ao L0 e ao patch: `p1315-measurement.json`, SHA-256
`ee6535435927f5ea6bbe8dadcd05663636b8cb41324151bd3d59a2a0c4019fe0`.
Ela contém saídas integrais, argv, horários, fontes e identidades dos binários.
Os casos multiline e linhas vazias iniciais refutam a regra antiga; o controle
CRLF evita concluir que contar newlines seria equivalente.

O L0 substitui expressamente a derivação P787 e somente essa parcela das
preservações P1313/P1314. Revisão preliminar independente classificou fluxo
contínuo ADR-0127: paridade interna, sem API, cast, dependência ou fase nova.

## Proveniência e limites da verificação

O commit solicitado foi criado antes deste passo:
`bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, incluindo o trabalho e evidências
pendentes P1309–P1314. A tentativa inicial de stage no sandbox foi recusada
por filesystem read-only; a execução autorizada no host criou o commit.
Não houve push. P1315 é working tree não commitado sobre esse commit;
recibos de comando contêm a lista exata de alterações via diff/stat e SHA
do owner/L0, antes e depois da execução.

Baseline executável P1314: `/dev/shm/p1314-target.cswujn/release/typst`,
SHA-256 `cf9b997ac399cfb95cf15063cc417484ed28a75be18f6e7d55a28300d792114f`.
Vanilla `/usr/local/bin/typst`, upstream `a51e02804`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
A string --version não é usada como prova de identidade.
Target novo `/dev/shm/p1315-target.V6TWEF`, cópia independente do cache,
sem hardlinks e sem sobrescrever baseline. RAM host exige o namespace
autorizado, distinto do sandbox padrão. Nenhum artefato histórico é reescrito.

Regime A/B **executado sem atestação de isolamento técnico**. Root escreve
L0, testes locais e código. `/root/p1315_tests` recebe L0/vanilla/baseline
e congela oráculos sem ler código candidato ou testes locais.
`/root/p1315_review` lê e julga artefatos, sem corrigi-los. Filesystem é
compartilhado; os limites de capacidades são procedimentais. Não há selo de
refinamento nem mutation score. Unknown obrigatório impede aprovação.

## Resultados

RED local: `cargo test -p typst-core --release p1315 --lib` terminou com
duas falhas exatamente em `line 3` versus `line 2`, e um controle de valores
válidos passou. Não foi falha de compilação. Recibo `p1315-unit-red.json`,
SHA-256 `ba7177838dcae34e4e759c0c3eb562553f920c50efa30a375f3712bd0580b505`,
com estado before/after idêntico e patch limitado ao L0/testes.

GREEN local: os três testes novos passaram sobre o candidato final resselado,
sem mudança de source durante a execução. Recibo `p1315-unit-green.json`,
SHA-256 `0dfa3398fd915a6ea592f479e8c56818136cf462da669c6b651a3354563213d2`.

Identidades finais de código e contrato:

- Owner SHA-256 `4e2bfe511360bd09b4d8bf3fd98f3085018a06c1fd4ebd9eb2db58b360fbe208`.
- L0 SHA-256 `b9c9b5f6f4d7835d75c91d952d45ed1130eb03f4a6504b8be7e8980be0291219`.
- L0 normativo congelado `9a67fcb0cd258ba661221b844e59d149061d4b673a52618bc62092054080b40a`;
  exclui somente a linha canônica Hash do Código. Linhagem A `14579ef7`, B `9bd50301`.

Lint: zero erros, 240 warnings e 1.137 infos, exit 0. Recibo
`p1315-lint.json`, SHA-256
`7e21449cc15eba0a2de38b7db7252a6e363410c3117490ce6bce000770f2e82c`.
`p1315-lineage-final.json` registra `Nothing to fix`; fmt e diff-check
também passaram. Zero erros não significa ausência de avisos.

Freeze independente `p1315-ab-freeze.json`, SHA-256
`c2cd0c90e4ea09082980b7dba5ed960dc8c9415bc4930fbeb6c68c2566ddac4f`:
290 casos, quatro perfis, 1.160 expectativas completas, 112 RED.
Dos casos, 234 são replay literal P1314 no cwd original e 56 são novos.
Não se conta replay histórico como suíte nova. Houve uma correção de
construção de quatro controles UTF-8 antes do candidato: soma de Bytes
falhava antes do CSV no baseline; tuplas dos mesmos octetos atingiram o
parser. Recorte de 32 execuções passou antes da nova medição final.
Esses incidentes e custos estão preservados no freeze, não apagados.

O patch só enumerou o iterador já existente e passou index+1 ao mapper
privado. O fallback dos demais erros ficou literal, assim como todos os
testes anteriores. O revisor conferiu freeze e RED antes de emitir
`GO_PREPATCH` em `p1315-review-prepatch.md`; a inspeção do candidato está
em `p1315-review-candidate.md`, sem achados.

Build workspace release: exit 0, recibo `p1315-build.json`, SHA-256
`36430daaed73064e9a783cb66f8b6b021822939da86b04566df7b5eb24598b46`.
Binário candidato `/dev/shm/p1315-target.V6TWEF/release/typst`, SHA-256
`6a4a75787060ce8015ebde85ef2deb078f4b6a0b827b5533602785261ba890a1`.

`cargo test --workspace --release --no-fail-fast`: **6.648 passaram,
zero falharam, três ignorados**, exit 0. Os ignorados são os três doctests
preexistentes de layout/introspector; nenhum teste foi desativado. Recibo
`p1315-workspace-tests.json`, SHA-256
`beb222a6b7ccaf6d10c725ee850d0395038483faea51fe97440953cef391661a`.
Owner e L0 têm os mesmos hashes antes/depois de cada execução. Não foi
necessário repetir a suíte por falha intermitente.

**A/B: 3.480 comparações aprovadas, zero falhas e zero Unknown**, nas ordens
normal/repeat/reverse. São 1.160 expectativas repetidas nas três ordens,
não 3.480 expressões distintas. As 936 expectativas históricas P1314
(234 casos nos quatro perfis) permanecem literais em cada ordem; as 112
expectativas RED passaram com alteração exclusivamente do número N.
Isso verifica estabilidade no ensaio, não prova universal de determinismo.

Saídas integrais em `p1315-ab-candidate-runs.json`, SHA-256
`7091b03795a741fd6c2120506e8322b6b871c43b8d91a76f30e8f920f96efc54`.
Comparação `p1315-ab-comparison.json`, SHA-256
`2ee04e8e626bb8d8032cbf3df66dbf4470e12096254184ff5de32fd1a164046d`.
Os demais corpora históricos não foram reexecutados integralmente neste
recorte; não há novo selo geral P1308 nem alegação de quitar suas dívidas.

**Parecer final independente: PASS restrito ao P1315**, sem achados
pendentes. `p1315-review-final.md`, SHA-256
`799177233bd55746d89617e197ce9fb7be1a06320ffdad0f22b4556c4f022372`.
Auditoria própria reproduzível por `p1315-review-audit.cjs`; recibo
`p1315-review-final-audit.json`, SHA-256
`72931346521506bda1f719d34a74306e0765ec420828720c403ef6294d10b813`.
Recontou saídas integrais, argv/perfis/cwd, cobertura, duplicatas e hashes
de inputs/fixtures/binários, além dos gates, sem discrepâncias.

Recibo do testador `p1315-ab-receipt.md`, SHA-256
`773bdf0b69eeb3bd0d332aca22fa2bc8c80d68b1458f9fc6df1c23a9a720568d`,
detalha execução, reprodução e limites. Após o parecer, só este fechamento
e o estado tático do passo foram atualizados. P1315 entregue sem stage ou
commit adicional; o commit solicitado do trabalho anterior permanece
`bc8213f36b7a29b4fdc30cfc74ddc23586117c64`.
