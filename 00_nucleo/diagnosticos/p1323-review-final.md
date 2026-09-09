# P1323 — veredito separado final

Veredito: aprovado para fechamento do recorte do warning HTML plain, com
as limitações abaixo. A objeção `p1323-review-process-r0.md` foi resolvida
pela medição focal e pelo comparador sucessor R2, sem reescrever ou absolver
o receipt inicial. O relatório `p1323-final-report.md` pode substituir suas
duas marcações pendentes por este veredito e pelos resultados R2.

Revisor `/root/p1323_review`; autor de intenção/implementação `/root`;
autor dos testes `/root/p1323_tests`. Regime A/B executado sem atestação
técnica de isolamento, sem selo de refinamento. O revisor escreveu apenas
seus artefatos e não corrigiu produto, norma ou oráculos que julgou.

## Produto e causalidade

O L0 foi aprovado antes da preparação mecânica e da correção. O teste
independente foi integrado exatamente como recebido; RED é uma assertiva
semântica executada, e o mesmo teste fica GREEN após completar a constante.
A preparação com a headline antiga conservou as saídas do baseline nos
quatro perfis medidos. As revisões anteriores registram os respectivos
horários, comandos, estados e hashes.

A auditoria independente `p1323-review-final-audit.json`, SHA-256
`0e7377ceb50af24504f839c51353dad809bf04678c6fa4cea999eec91b95d488`,
registra HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree
não commitado e diff/stat. Ela verifica que o inventário produtivo do
baseline só difere no par wiring. As alterações preexistentes em
call_dispatch/loading L0/Rust permanecem intactas. Removendo exatamente
constante e snippet e restaurando emissão/header anteriores, o owner
coincide byte a byte com `original_owner` do baseline.

Logo o único delta semântico produtivo deste passo é a complementação do
warning. A condição Feature::Html, posição antes da compilação, canal stderr,
dispatch dos formatos, modos e infraestrutura de cores estão preservados.
O teste unitário sozinho não provaria a ligação produtiva; os processos A/B
exercitam essa ligação, inclusive erro HTML após o warning e ausência sem
feature. A classificação ADR-0127 de fluxo contínuo permanece válida.

## A/B e retificação PDF

Li integralmente ambos os runners. O R2 mantém casos/argv, warning e entradas
protegidas do R1; altera apenas comparação PDF e retenção dos artefatos.
Seu freeze está pinado antes da nova execução integral. O focal reteve
amostras dos mesmos binários, identificou diferenças somente no payload
XMP InstanceID e mostrou igualdade das demais observações. Conferi também
diff direto dos PDFs originais baseline repetido e baseline/candidato.

A exclusão é estreita: somente o payload único, base64 canônico de 16 bytes;
todos os demais bytes continuam discriminatórios. A calibração rejeita
mudanças de MediaBox, fonte e DocumentID, preserva a alteração exclusiva
do InstanceID e mantém opacidade/ausência/duplicação como Unknown. Não há
normalização geral de metadados nem mutação do exporter. Esta retificação
é adequada ao controle de preservação medido e à ADR-0107, sem transformar
o controle em prova de paridade PDF.

O receipt R2, SHA-256
`55e316ba86a7681bd01cef3cd67c3df6e6fed3b43e11cacceb06ae30e9193e9e`,
contém quinze casos em cada uma das ordens normal, repetida e inversa:
45 verificações de warning e 45 de preservação passam; 60 comparações de
estabilidade são verdadeiras. A auditoria final verificou esses estados,
recalculou hashes dos artefatos retidos e comparou independentemente os PDFs
fora do único payload. Nenhum Unknown obrigatório foi convertido em sucesso.
Ausência/rejeição nos perfis sem HTML é controle de gate, não crédito de
paridade HTML exercitada.

Os PDFs originais R1 foram apagados pelo runner R1. Isso continua uma
limitação: a causa foi demonstrada nas amostras focais novas, não recuperada
retroativamente nas amostras perdidas. O receipt R1 continua vermelho e
imutável. As calibrações usam cópias do observável, não mutantes produtivos;
portanto não há alegação de mutation score produtivo ou selo de refinamento.

## Linhagem e gates

Recalculei a linhagem a partir das funções canônicas do tekt-linter
`hash_writer.rs:16`, `prompt_io.rs:151–235` e `nucleus.rs:173–218`.
O hash efetivo do L0 é `dd1ca444` e coincide com o header; os três pins dos
Núcleos conferem pelo framing canônico. O hash do source sem sua linha
recíproca é `136cabde8c1b7c0a0b8f51cf2ac6f8628fb1bf6b4a4e7effa38c3f483ad6def8`.

A inspeção detectou metadado recíproco stale apesar do dry-run `Nothing to
fix`. O autor corrigiu somente `Hash do Código: 136cabde`, com receipt
`p1323-reciprocal-correction.json` preservado. O corpo normativo, fonte,
binário e testes não mudaram; não era necessário repetir os gates funcionais
por essa linha. V5/V15/V26 e diff-check foram repetidos depois da correção.

O audit final pina os receipts de build workspace release locked, GREEN,
workspace tests, formatação, lint, linhagem e diff-check. Todos têm exit zero.
A contagem independente dos grupos do receipt workspace confirma 6.667
testes passados, nenhum falhado e três ignorados. O lint completo contém
zero erros, 240 warnings e 1.138 infos; V5/V15/V26 não têm violações mesmo
com `--fail-on warning`. Assim, não se deve escrever “lint integral sem
violações/avisos”: o sucesso é do gate executado e o passivo de warnings
continua explícito.

## Identidade final e limites de fechamento

- L0 integral: `83d7a1364b136a7d198d3fd84c601336ef2fff687b34987c904f637e8173870d`.
- Corpo normativo: `33ca7a522ae37d8fc9e206c6f5cc592638adbf6d182b5eac189c138e197b4926`.
- Owner integral: `0ee273c4411d9752f14900982f02f481c2bc50f125944296111fbe760d5fae9a`.
- Binário candidato: `f0b251746274d537c63929ab357b0e84e0e0955c7986d58621d83a0ce3a4e5ee`.
- Suite R2: `d98a1b73c0dcc5167f145c1195db6ccb9a84294dfce08b2c731c69b9f2248cb1`.

Estas identidades e os pins do audit delimitam o veredito. Não há paridade
ANSI, HTML geral, PDF geral ou linguagem geral atestada. Os débitos P1322 não
se fecham por estes controles. Alterações posteriores em entradas protegidas
exigem revisão sucessora; correções documentais no relatório/fechamento podem
referenciar este parecer sem modificar os artefatos avaliados.

O script de auditoria inicial encontrou a peculiaridade do ambiente Node:
spawn retornou EPERM com processo git efetivamente concluído, status zero e
stdout presente. O sucessor r1 tornou esse caso explícito; r2 acrescentou
verificação de R2/linhagem. Os scripts predecessores estão preservados. Isso
não alterou o produto nem foi usado para ignorar falha de gate.
