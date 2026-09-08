# P1316 — revisão independente anterior ao candidato

Executor: `/root/p1316_review`, ambiente compartilhado
`/repos/Antigravity/typst-crystalline`. Regime A/B, executado sem atestação
técnica de isolamento. O papel revisor lê spec, baseline, oráculos congelados
e recibos; escreve somente `00_nucleo/diagnosticos/p1316-review-*`.
Não escreve código, L0 nem oráculos. Contexto herdado: instruções do repositório
e pedido delimitado do coordenador; nenhum patch candidato P1316 recebido.

## Evidência anterior à classificação

Entrada `p1316-measurement.json`, SHA-256
`b5d25936d78b8584cec7469fdde2a2cd5130da2f52936bcdd9f103efc28154d7`,
confirmado diretamente. Contém 24 execuções entre
`2026-09-08T14:23:00.074400+00:00` e
`2026-09-08T14:23:01.374796+00:00`, HEAD
`bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, com P1315 não commitado.
O `diff_stat` registrado identifica exatamente:

```text
 00_nucleo/prompts/compiler/stdlib/loading.md | 57 +++++++++++++++++-
 01_core/src/compiler/stdlib/loading.rs       | 86 ++++++++++++++++++++++++----
 2 files changed, 129 insertions(+), 14 deletions(-)
```

Fonte baseline SHA-256
`4e2bfe511360bd09b4d8bf3fd98f3085018a06c1fd4ebd9eb2db58b360fbe208`;
L0 baseline SHA-256
`b9c9b5f6f4d7835d75c91d952d45ed1130eb03f4a6504b8be7e8980be0291219`.
Binário cristalino `/dev/shm/p1315-target.V6TWEF/release/typst`, SHA-256
`6a4a75787060ce8015ebde85ef2deb078f4b6a0b827b5533602785261ba890a1`.
Vanilla `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`,
upstream ratificado `a51e02804`.

Leitura direta da fonte ratificada:
`lab/typst-original/crates/typst-library/src/loading/mod.rs:79-110`
conserva o span da fonte em Loaded e distingue Bytes de Path;
`lab/typst-original/crates/typst-library/src/diag.rs:845-925`
ancora erros de texto de arquivo no arquivo, mas erros de Bytes no argumento.
A medição confirma isso para `csv(bytes("a,b\n1"))`, With, spread de
arguments e named anterior. A transformação por arguments.map permanece
detached. Path/Str aponta para o arquivo no vanilla e continua divergente
no baseline. Texto de UTF-8 e sufixos de posição também divergem.

O L0 vigente, seção P1315, preserva spans detached. P1316 precisa substituir
explicitamente essa obrigação somente para parsing na chamada CSV com Bytes;
a API pública `decode_csv` e os caminhos Path/Str continuam preservados.

## Classificação e limites

ADR-0127, seção 2, fluxo contínuo item 3: correção de paridade com o vanilla
pode mudar saída observável, com L0 primeiro, RED→GREEN e revalidação.
O escopo proposto corrige origem diagnóstica interna, não introduz API,
campo, trait, flag, modo padrão ou fase. Portanto a classificação proposta
é fluxo contínuo. A origem e os diagnósticos pertencem ao observável da
linguagem, conforme ADR-0108; não se pede copiar mecânica interna do vanilla.

É inferência que os carriers Args existentes bastam. Origem conservada no
vanilla, mas indisponível neste consumer, refutaria o fechamento local.
Generalizar a ancoragem Bytes para Path/Str seria alteração fora do escopo,
refutada pela distinção explícita na fonte ratificada.

Skill e ambas as referências operacionais foram lidas integralmente.
Nenhuma ADR de segregação foi localizada na coleção de ADRs permitida.
Sem consulta às pastas materialization/context.

Estado: classificação aceita para o escopo descrito; GO de implementação
pendente de L0 atualizado, freeze A/B e inspeção dos artefatos congelados.
Sem veredito final ou alegação de paridade CSV integral.
