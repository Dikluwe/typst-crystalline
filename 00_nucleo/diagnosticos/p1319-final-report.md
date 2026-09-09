# P1319 — dizer qual arquivo CSV falhou sem tratar binário como texto

Estado: implementado e validado; PASS no recorte P1319, com limites abaixo.

## O que este passo resolve — e o que não resolve

Antes deste passo, uma falha de CSV carregado de arquivo perdia o caminho e
a origem do argumento. Para um buffer que contém UTF-8 inválido, P1319 acrescenta
`in caminho:linha:coluna` à causa e aponta para o argumento da chamada.
O caminho é virtual: não expõe o caminho físico do host; preserva raiz de
pacote e diretórios. A posição é a do parser, não o primeiro byte inválido.

Isso vale também quando o erro vencedor é quantidade incorreta de campos
e o byte inválido aparece depois. A validade do buffer inteiro decide a
apresentação, mas não pode antecipar nem trocar o erro do parser.

**CSV com texto UTF-8 válido não é corrigido aqui.** O vanilla aponta para
um range no arquivo externo; `World::read_path` devolve bytes sem FileId.
Usar `include_path` como atalho confundiria inclusão de Source com identidade
diagnóstica. Essa frente exige investigação/contrato próprios, não um sufixo
binário aplicado indiscriminadamente. I/O, coerções Symbol, validação de
unknown/missing/excesso e csv.encode ausente continuam dívidas separadas.

Outra dívida medida: Str com origem detached pode ser lido no cristalino,
mas vanilla responde `cannot access file system from here`; com excesso
posicional, vanilla rejeita antes do parsing. Os deltas desses casos não
podem ser apresentados como paridade integral. A resolução atual da string
no current_file não muda, nem a raiz/base já capturada em Value::Path.

## Commit e proveniência

Commit solicitado dos passos P1315–P1318:
`d31047d7b8af7837c84adae4ded3d2ff50c62093`,
`fix(csv): align parsing diagnostics for bytes (P1315-P1318)`.
Auditoria final P1318 foi reexecutada antes do commit, sem divergência de
fonte/L0, oráculos ou artefatos históricos. A árvore ficou limpa após o commit.
Não houve push, reescrita de histórico ou alteração de evidências anteriores.

Baseline P1319: esse commit, executável validado P1318
`/tmp/p1318-target.eAgQwp/release/typst`, SHA-256
`0bdb7c2ca80d7be17775d03d3fc7ac83ba4401bf4468dd84ad7711a93b5a585c`.
O binário foi construído antes do commit; identidade é o SHA e a cadeia
P1318, não sua string --version. Vanilla `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`,
upstream ratificado `a51e02804`.

`p1319-measurement.json`, SHA-256
`4396b098d5f5db30bfd832d210c1027eb9529d8120c18c5a63fd9bc16d8ae3bd`,
registra 52 observações, fonte/L0 integrais, bytes das fixtures, comandos,
cwd real das fixtures, UTC e estado antes/depois sobre o HEAD acima.
Não há alegação baseada no probe absoluto incorreto de etapas anteriores:
os arquivos desta medição foram efetivamente lidos nos dois produtos.

Fonte cristalina baseline `loading.rs:1374-1375` descarta caminho e retorna
erro puro; `read_path_value` já preserva RootedPath e bytes. Vanilla
`loading/csv.rs:138-157` fornece causa/posição; `diag.rs:851-855,895-923`
seleciona o ramo binário e compõe caminho/origem. `diag.rs:858-873` exige
FileId no ramo de texto válido. A leitura extra dessa fronteira impediu
generalizar a correção binária a todos os arquivos.

Target exclusivo `/tmp/p1319-target.VqXtmj`, cópia do cache P1318 sem hardlinks.
Foi usado /tmp por falta de espaço para o cache completo na RAM. Baseline
preservado, sem limpeza de temporários.

## Autoridades e gates

Skill `tekt-materializacao-segregada`, regime A/B sem atestação técnica de
isolamento. Root escreve obrigação, testes locais e implementação; testador
`/root/p1319_tests` recebe L0, vanilla, histórico A/B público e binários,
sem acesso declarado a código/testes/diff produtivo; revisor
`/root/p1319_review` julga sem editar produto ou oráculos. Filesystem
compartilhado: nomes distintos não são prova de isolamento. Sem selo,
mutation score ou paridade geral CSV. Unknown bloqueia, não vira sucesso.

Uma medição/freeze, correções focais antes do corpus final e candidato nas
ordens normal/repeat/reverse. Duas revisões sem ganho na mesma causa exigem
rever o desenho. Scripts e recibos novos usam serialização ASCII/linhas LF
para evitar a corrupção de U+2028 observada no writer P1318; o bruto antigo
permanece preservado como evidência, não foi reescrito.

`p1319-review-preliminary.md` e `p1319-review-l0.md`: favoráveis ao ramo
binário, fluxo contínuo ADR-0127, sem API/trait/entidade nova. L0 atualizado
antes dos testes e do candidato. Preflight V15/V26 exit 0,
`p1319-lineage-preflight.json`, SHA-256
`c9a4f4e36a9e7a11fd44f968d2a5dfb66b26a7c4995a5c088e642c6242cb935d`.

Delta explícito antes do candidato: `p1319-test-delta.json`, SHA-256
`5856efb9c7f04ccf099095a72f266d8535090abb371f8216188f88e3659a5545`.
Dois testes antigos passam a esperar o sufixo de arquivo inválido, mantendo
controles puros e detached. Dois testes novos cobrem causas/raízes/origens,
normalização e uma única resolução/leitura. MockWorld registra chamadas;
nenhuma asserção antiga foi retirada. O manifesto verifica que o prefixo
produtivo ainda era idêntico ao HEAD, salvo linhagem.

RED confirmado: 65 testes passam e quatro falham pela ausência do novo
sufixo (os dois testes antigos atualizados e os dois novos). Não é falha
de compilação/infraestrutura. Recibo `p1319-unit-red.json`, SHA-256
`adbfe60839f87ac4087d15848755d8a2dc4de3da375a4b9dd1500032fe2c5e7a`,
exit 101. Fonte RED SHA-256
`9c647415dac61f26ac5a729bb9f5cfa80867bb85de83b76288d40e87f3a9152c`,
L0 bruto `5c33854b87f1af32a6bee7f62de0aa27f73ee6496f27e5662d253db2349aff75`;
os recibos preservam diff/stat e UTC. Normativo congelável
`da59f9964566dc99d3341145fe8200df15ff8df172158c79d91542d47015e823`
exclui somente a linha canônica Hash do Código, não obrigações.

Na calibragem A/B, sondas cross-file de eval encontraram erro de sandbox
antes do CSV no baseline. Esses resultados não são RED de parsing e
permanecem como controles da dívida de resolução, com novas sondas diretas
de subdiretório para medir caminho virtual. Nenhum candidato existia nessa
revisão focal; todos os replays P1318 permanecem obrigatórios.

Freeze `p1319-ab-freeze.json`, SHA-256
`61d7c5005e889ae0857230f27a4ca8db5ecc38734abfe8699418459c37a8908f`,
em `2026-09-08T16:09:27.147983+00:00`: 645 casos, quatro perfis,
2.580 expectativas. São 432 RED (400 diagnósticos vanilla integrais,
32 efeitos normativos detached/excesso) e 2.148 preservações literais.
Todos os 2.052 replays P1318 coincidiram antes dos deltas declarados de
oito casos históricos. A revisão focal anterior ao freeze conservou quatro
sondas de sandbox e acrescentou quatro sondas diretas de subdiretório,
com 32 processos adicionais. Brutos/scripts r0 foram preservados.

O A/B declara seus limites: Package e base capturada cross-file não são
atestados por esse corpus CLI. A identidade Package/Project e a ausência de
re-resolução Path são verificadas localmente e por revisão de código;
o P1319 não promete corrigir a resolução cross-file de Str.

GO pré-patch definitivo em `p1319-review-prepatch.md`, antes de qualquer
alteração produtiva. O candidato preserva um único parser; um contexto
privado distingue decoder puro, Bytes e arquivo. A rota de arquivo retém
RootedPath da leitura original; no erro, consulta validade integral e compõe
a apresentação binária apenas quando aplicável. Não usa include_path/source.

## Achado de linhagem: Nothing to fix não provou o hash reverso

A primeira execução GREEN passou os 69 testes, mas o linter deixou
`Hash do Código: 5249d235`, referente ao RED, depois da implementação.
O resultado `Nothing to fix` em `p1319-lineage-final.json` **não é prova de
par correto**. O diagnóstico independente encontrou B esperado `bf49e7b7`,
calculado por SHA-256 do source sem sua linha canônica `//! @prompt-hash`.
O hash completo é
`bf49e7b7a0874d81e24e2a0bdf23397fd504d4244099314a4cdaef0ee7a54edf`.

Fonte da ferramenta consultada somente leitura:
`/repos/Antigravity/tekt-linter/03_infra/hash_writer.rs:14-19` define B;
`02_shell/fix_hashes.rs:293-300` constrói plano somente de violações V5;
`04_wiring/main.rs:810-838` deriva os pares dessas entradas. Quando A já
coincide, não há entrada para reparar B. Isso contraria a suficiência
bidirecional exigida pela ADR-0129; não é uma colisão legítima de hash.

Checkout da ferramenta `49f46885fd03f753e9f1cf37e271d6baf30ed725`, limpo;
binário instalado identificado separadamente, SHA-256
`e47974fe903b225e96fec580040604d80380edda054a7b2187bbf5b18da9a30b`.
Não foi alterado o linter nem outro repositório. O reparo local foi somente
metadata B via apply_patch, depois de terminar o primeiro GREEN; nenhuma
obrigação, teste ou expectativa A/B mudou. A norma congelada continuou igual.

`p1319-lineage-check.py` reproduz a checagem explícita de B, compara norma
com freeze e exige V5/V15/V26. Antes do reparo: exit de checker 1 apesar de
exit de linter 0, `p1319-lineage-before-repair.json`, SHA-256
`3752b30ed395dfd2f5b4b99fcc8da3b2e5f6ecb9167d057a1b14ec2a11cde417`.
Após reparo: PASS em `p1319-lineage-verified-r1.json`, SHA-256
`70f95c86e4175d610a074f03a1b2a03b55e5b749bc541a2f8bc8dbd4823a1570`.
Recibos preservam hashes de ferramenta/fontes, estados, UTC e comando.
O parecer `p1319-review-candidate-and-lineage.md` aceita essa validação
explícita para P1319; a deficiência da ferramenta permanece uma dívida real.

GREEN após metadata: `p1319-unit-green-r1.json`, SHA-256
`dfb6b5c94b35d541917bf10d986208b39cbbe82601f10e72913ceb583a7aade1`,
69 testes passam, zero falhas. Fonte inicial candidata
`4518a5531a69163cf9f6474ca04cd22ac0906cd8e10fd98297cf93b9e9ba890e`,
L0 bruto reparado `181f748041e4f8bec6f80a724930ffbeee5bb98d1ec52068047a296b00566c7d`.
Todos os estados são working tree não commitado sobre o HEAD P1319 acima.

O lint desse candidato apontou um warning V16 novo no wildcard do contexto
privado. Antes de liberar binário para A/B, os ramos Pure e File válido foram
explicitados para não silenciar variantes futuras. A revisão reproduziu o
source anterior substituindo somente esses braços em memória. Norma e testes
congelados permanecem; os recibos iniciais não foram sobrescritos.

## Candidato final r2

Identidades sobre working tree não commitado do HEAD P1319 acima:

- Source SHA-256 `15078b5441b514a200c54039589521cb4755b99a99a68b5a37983c247f759800`.
- L0 bruto SHA-256 `1bd9633f0cdfb5a1ef7ca0265b55ac7ea8ad5b28c41035a2aaaa3572edff0dd1`.
- A `94cfda0e`, B `ed0ff72c`; norma congelada `da59f9964566dc99d3341145fe8200df15ff8df172158c79d91542d47015e823`.
- Binário `/tmp/p1319-target.VqXtmj/release/typst`, SHA-256
  `37a8a23d6e2b5daf355d90d510bca7efb399547730e1d807e8ae871be75748bd`.

Build `cargo build --workspace --release`, exit 0, entre
`2026-09-08T16:23:23.215992+00:00` e `2026-09-08T16:24:01.896571+00:00`.
Somente esse binário r2 foi liberado ao testador; o candidato inicial não
teve execução do corpus A/B. Baseline e vanilla conservam suas identidades.

Gates r2 com fonte/L0 antes e depois idênticos aos acima:

| Recibo | Resultado | SHA-256 |
|---|---|---|
| `p1319-build-r2.json` | build exit 0 | `7117debf0c6486f087e4426a0837a4dc904ec3ac15c217ea62cd5b778c0d06d4` |
| `p1319-lineage-verified-r2.json` | B recalculado, norma intacta, V5/V15/V26 aprovados | `425a8c1b27e8178177940e6a513a8c33ac58091737367cc1c868d6c929acde29` |
| `p1319-lineage-final-r2.json` | dry-run Nothing to fix, apenas complemento da prova acima | `cca82e6fdcea79736e222efec6a60219837f271f48ec7873e2ca5e4b72c3c657` |
| `p1319-lint-r2.json` | 0 errors, 240 warnings, 1.137 infos | `b2bbff72cf9a13b27a1595e63b099927b31f9d7076a6b4f9095e046413e237bb` |
| `p1319-fmt-r2.json` | cargo fmt --all --check, exit 0 | `734b321dd10a4a59dcff3d9f20a553107adbc3f8d64bfb591401f68bcbb018b5` |
| `p1319-diff-check-r2.json` | git diff --check, exit 0 | `4dc416e2eab6898149da243bd81eb5a6b56567fab8488314f42cc8d03f985911` |

Os warnings não foram apresentados como zero dívida. A falha reverse-hash
da ferramenta continua registrada e não foi corrigida fora do escopo.
GREEN r2: 69 testes passam, zero falhas/ignorados, exit 0;
`p1319-unit-green-r2.json`, SHA-256
`5ed8f4f90efb122ee5ae3c5657af623a20cae44ad8909f0e1a9703f70d765513`.
O módulo inteiro de testes permanece idêntico ao RED reconstruído pela
revisão, SHA-256 `9d89b7d2441cc4f5f76b5222abf9115f2ead0a8a2be081bdf3c6674ac7b22101`.
Workspace r2: `cargo test --workspace --release --no-fail-fast`, exit 0,
6.661 testes passam, zero falhas, três doctests ignorados. Recibo
`p1319-workspace-tests-r2.json`, SHA-256
`0a420e3668df1b5f2cdd2ee7a7b77f1473fdc0987d5d67e082c2c34da44c47d8`.
Execução entre `2026-09-08T16:23:32.864799+00:00` e
`2026-09-08T16:30:28.382771+00:00`, fonte/L0 finais estáveis.

## A/B final e parecer independente

O candidato r2 foi executado entre `2026-09-08T16:25:01.381970+00:00` e
`2026-09-08T16:29:50.461642+00:00`, sobre o working tree e as identidades
finais acima. Foram 645 casos em quatro perfis: 2.580 expectativas congeladas,
exercitadas em ordem normal, repetida e inversa. As 7.740 comparações passaram,
sem falhas nem Unknown. Das expectativas, 400 exigem correção vanilla integral,
32 exigem correção normativa com dívidas explícitas e 2.148 preservam o resultado
literal anterior. Isso não equivale a paridade geral CSV.

Proveniência reproduzível:

- Freeze `p1319-ab-freeze.json`, SHA-256
  `61d7c5005e889ae0857230f27a4ca8db5ecc38734abfe8699418459c37a8908f`.
- Execuções `p1319-ab-candidate-runs.json`, SHA-256
  `903ad38bc4e1306e8637d2e7f801ae044d6f65fe5c3a25ce02d2c46c92ace31d`.
- Comparação `p1319-ab-comparison.json`, SHA-256
  `69219e4762054c05b3f04a861b4edca450e5f2e69aad67e044579b64ce44d5dc`.
- Recibo explicativo `p1319-ab-receipt.md`, SHA-256
  `ce1b9e415d7b1908e42cbd0ecd0161f885c60fa4f1c5e2a8e88b51f496c2ed61`.

A calibração anterior ao patch preservou as sondas de import que falham antes
de CSV como controles de dívida e acrescentou sondas diretas de subdiretório.
Os registros r0/r1 permanecem; nenhuma expectativa mudou após o freeze final.
Essas sondas CLI não provam base capturada entre arquivos. Package e chamadas
World têm evidência local; o parser único e os fallbacks de posição também
foram examinados no código pelo revisor.

Parecer independente `p1319-review-final.md`, SHA-256
`1557870bfbc75c3b3b0e2ebbb2817890ebb4108e49ce0fc303470df743da3895`:
PASS no recorte, sem bloqueios restantes. Sua auditoria reproduzível
`p1319-review-final-audit.json`, SHA-256
`43972448740d8373dbf50c45d75baea66ac258e45bc334e008ebbd243c563d45`,
registra UTC, comandos, estados e hashes dos recibos. Recalcula cada comparação,
a identidade do módulo de testes RED/GREEN, a norma e a linhagem bidirecional
incluindo o núcleo; confirma integridade dos inputs congelados. Não depende
apenas do PASS declarado pelo testador ou pelo linter.

## Encerramento e próximo limite

P1319 encerra a localização do diagnóstico de parsing para arquivos com buffer
UTF-8 inválido. O uso da skill `tekt-materializacao-segregada` separou autoria
do candidato, produção do oráculo A/B e julgamento final, sem alegar isolamento
técnico atestado. A medição manteve o ramo UTF-8 válido fora da correção; a
revisão exigiu prova explícita para compensar o falso-negativo do linter.

Continuam abertos o diagnóstico externo de CSV UTF-8 válido, as dívidas de
resolução/validação descritas no início e o reparo da ferramenta de linhagem.
Não são pendências ocultas deste recorte nem foram corrigidos por este passo.
O commit solicitado integra P1315–P1318; os arquivos novos e a implementação
P1319 permanecem não commitados para revisão. Sem push ou limpeza de temporários.
