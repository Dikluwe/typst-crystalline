# P1332 — parecer sobre L0 e manifesto antes de C

**Veredito: favorável ao L0 e ao manifesto para a fase A/B pré-C.**
Nenhum candidato, RED ou suite congelada foi aprovado por este parecer.
Revisor `p1332_review`, sem editar os inputs julgados; execução sem atestação
técnica de isolamento e sem selo de refinamento.

## Entradas e verificação

O L0 calc atualizado foi lido integralmente. Norma canônica:
`ac26a9a0492f250e4a59175256b8b1b7a4a173546dc483131a704f01262b84ad`.
Manifesto `p1332-manifest.json`:
`b32fa0ac39b79d7a1b7f23f62540011b58ac68e20f0c850009f688b68b412084`.

O script read-only `p1332-review-l0-audit.cjs`, executado com
`node 00_nucleo/diagnosticos/p1332-review-l0-audit.cjs`, terminou com exit 0
em `2026-09-09T14:35:10.626Z`. Recomputou os hashes do manifesto, norma,
baseline privado/público, sondas públicas de consumidores e fechamento
P1331. Todos os bindings conferiram. Comparou o texto do L0 com
`original_prompt` do baseline: somente a seção P1332 foi acrescentada.
Comparou integralmente o owner após remover apenas `@prompt-hash`:
igual ao baseline. Inventário produtivo: somente calc.md e o header de
calc.rs diferem. Logo a leitura do registro vigente não é leitura de C.

Proveniência do baseline `p1332-baseline.json`:
`1e9b77f550d5e34c4d3b52df5306eae3588122a35262d19128561532c33a7d97`,
UTC `2026-09-09T14:30:30.883093+00:00`, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado,
com diff/stat/inventário. O diff inicial está também em
`p1332-review-scope.md`; a alteração P1332 do estado auditado é a adição
normativa e o header descritos acima. Os recibos públicos conservam argv,
instantes individuais, exit e stdout/stderr integrais.

Baseline público: `559b83db56c457f13aa8969a3260f99fec3b90f3b759c0e844a0749ce547e1ed`.
Sondas de consumidores: `2d8ea9bb87879b307d1a7bd35e35c72feb02f913470b935aa8355207f8002e48`.
BASE P1331: `/tmp/p1331-target.rtY0la/release/typst`,
`a8d6e2f4472fefc9e63a123783feac852445191dcb82ac269d366a6419ae1a47`.
Vanilla ratificado `a51e02804`: `/usr/local/bin/typst`,
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Nenhum binário foi executado pelo revisor nesta fase; as observações são
auditadas dos recibos, não apresentadas como novas execuções.

## Substância da norma

`calc.md:332-402` atende a medição anterior à decisão e delimita a
obrigação: nome intrínseco abs, chave calc.abs acessível, mesmo callable,
assinatura, resultados, guards, origem e demais componentes do diagnóstico.
A nova seção substitui expressamente a antiga preservação do nome nas
seções anteriores. Não autoriza alterar dispatcher, Func, repr ou outros
registros. A posição histórica “dívida do dispatcher” não é usada para
atribuir ownership incorreto ao novo código.

O baseline público distingue adequadamente três efeitos:

- With/arguments de conteúdo, Length misto, overflow e fallback possuem
  diagnóstico primário já equivalente, com diferença no nome do trace.
- Named/aridade continuam divergentes no diagnóstico primário e origem;
  corrigir nome do trace não significa equivalência integral. A string
  `calc.abs()` na mensagem de aridade permanece protegida.
- Repr de lookup/import/alias/With e igualdade na linguagem já coincidem
  nas sondas registradas; são obrigações de preservação, não ganho P1332.

As sondas de consumidores incluem gradient linear/radial/conic e show.
Elas confirmam interpolação do nome em mensagens que já divergem do vanilla.
A norma permite apenas o efeito transitivo do nome e exige conservar os
demais campos; não reivindica corrigir esses owners. `.where()` permanece
dívida explícita e já imprime o nome curto. A aceitação não se limita a
normalizar o stderr nem a comparar substrings.

Igualdade/hash de nativas por nome foram reconhecidos, sem promover valores
de hash Rust a observáveis da linguagem nem modificar Func::PartialEq.
A hipótese de suficiência do registro se apoia na ausência de homônimo
produtivo encontrada na inspeção e exige preservar identidade por
lookup/import/alias e distinção de outras funções. Colisão acessível
refutaria essa hipótese. Isso resolve o principal risco identificado no
parecer de escopo sem inventar exigência de editar outro consumer.

## Regime e próximos gates

ADR-0127 continua em fluxo contínuo: correção de nome para paridade, sem
contrato público ou fase novos. O manifesto nomeia root, autor B e revisor,
suas entradas e outputs; assume a limitação de isolamento. Congela norma,
baseline e efeitos observáveis, impõe Unknown bloqueante e limita revisões
focais sem ganho. Os paths de testes/expectativas serão fixados no freeze
do autor B; a ausência desses outputs nesta fase não é um selo incompleto
de refinamento, pois esse regime foi explicitamente excluído.

Antes de C ainda é preciso auditar o freeze B, sua migração restrita de
nomes históricos e o RED compilado. Depois de C serão necessários GREEN,
CLI integral nos ordenamentos/perfis exigidos, gates arquiteturais e
preservação de artefatos. Este parecer não antecipa esses resultados nem
transforma a hipótese de suficiência em prova de equivalência geral.

O path do passo aparece no manifesto, mas seu arquivo não foi aberto nem
usado como L0. Todos os writes desta revisão são `p1332-review-*`.
