# P1339 — sonda causal de show em compile

A nova sonda é executável nos quatro perfis e observa falhas semânticas do
antecedente nas 18 fixtures de selectors. As 108 células antes limitadas pelo
transporte `query --features` agora têm observação sucessora real via `compile`.
Os registros Unknown originais foram preservados integralmente: não foram
apagados, reescritos ou contados como sucesso.

Este é um recibo de medição exploratória independente, **executada sem atestação
de isolamento técnico**. Não constitui contrato, selo, implementação ou veredito
final de P1339. A skill `tekt-materializacao-segregada` orientou a separação de
entradas, o freeze anterior à execução, a revisão focal limitada e a manutenção
dos Unknown e limites de cobertura.

## Entradas, capacidades e causalidade

Executor `/root/p1312_tests`, reutilizado com contexto histórico P1312. As
funções, implementações e resultados P1312 desse contexto não foram usados
como entrada ou evidência desta sonda. Não foi lido L0 ou código produtivo,
candidato ou diff funcional P1339. Foram lidos somente o passo explicitamente
autorizado `00_nucleo/materialization/typst-passo-1339.md`, instruções do
repositório e skill, documentos de medição permitidos, fixtures listadas pelo
manifesto antigo e os artefatos desta sonda. Nenhuma pasta materialization/context
foi listada. `git status`, HEAD e diff/stat registram proveniência sem ler conteúdo
das alterações produtivas ou L0.

Escritas limitadas a `00_nucleo/diagnosticos/p1339-show-witness-*` por
`apply_patch`, além dos diretórios de saída PDF temporários dedicados em `/tmp`.
Python foi executado com `-B` e sem import de scripts anteriores de outro papel.
O filesystem compartilhado não impõe tecnicamente a allowlist declarada; não há
alegação de contexto limpo absoluto ou de atestação de isolamento.

Vanilla `/usr/local/bin/typst`, upstream ratificado `a51e02804`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Antecedente `/tmp/p1338-target.vlNAmp/release/typst`, SHA-256
`f7c8085f8453ecb6b10841b698092d41e7fbec64d9d46f9341b9b9b538bfefa1`.
Os hashes foram verificados antes e depois de cada rodada. Todas as contagens
deste recibo derivam desses executáveis sobre HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`; a working tree contém alterações L0
em paralelo e documentos não commitados. HEAD, status integral, diff/stat e UTC
de cada início/fim estão nos manifestos e registros, sem atribuir essas
alterações ao executável antecedente já pinado.

## Desenho e única revisão focal

O manifesto anterior e seus resultados vanilla mostram `metadata("MATCH")`
para filtros vazios/equivalentes e ausência para mismatch. A nova fixture
conserva literalmente selector e sujeito e substitui somente o corpo do callback
por `panic("P1339_SHOW_WITNESS_<ID>")`. A ponte verifica mecanicamente essa
substituição. As expectativas de ocorrência/não ocorrência foram congeladas
antes de executar cada rodada e identificam a observação anterior usada.

O primeiro focal continha três casos strong: estático matching, estático
mismatch e show direto sem where. A expectativa inicial supunha aspas no
diagnóstico primário de panic; a observação vanilla mostrou que não há aspas.
Essa hipótese de formatação foi refutada antes de executar o baseline. Os
artefatos r0 permanecem imutáveis. A única revisão autorizada mudou esse literal;
a predição de matching e as expressões permaneceram as mesmas.

Na revisão focal R1, o vanilla produziu oito callbacks e quatro compilações
sem callback. O baseline executou o callback nos quatro controles diretos e
rejeitou os oito selectors estáticos com
`error: type function does not contain field "where"`. O focal foi validado
antes da expansão. A execução do callback exige o diagnóstico primário exato
`error: panicked with: P1339_SHOW_WITNESS_<ID>` com exit não zero; uma ocorrência
do marcador somente na fonte impressa por outro erro jamais conta como witness.

## Expansão e observações

O manifesto final contém 18 selectors strong/emph/text, nas formas estática e
ligada, com campos vazios/matching/mismatch, mais três controles show direto.
Foram executados quatro perfis (`default`, `html`, `a11y`, `html+a11y`) em ordem
normal e inversa: 168 execuções por produto, 336 no total. O target de saída
permanece PDF; habilitar a feature html não transforma isto em teste de target HTML.

Vanilla: 120 callbacks-sentinela e 48 mismatches sem callback, todas as predições
observadas. Baseline: 144 selectors rejeitados antes do callback; 16 callbacks
nos controles diretos strong/emph; oito rejeições no controle direto text.
Não houve Unknown de transporte nem instabilidade entre as ordens.

Os diagnósticos dos selectors no baseline foram:

- Estáticos: `type function does not contain field "where"`.
- Strong ligado: `selector where não suportado para strong`.
- Emph ligado: `selector where não suportado para emph`.
- Text ligado: ``function `text` does not contain field `where` ``.

O controle text direto revelou uma dívida semântica própria do baseline:
`função 'text' não é um tipo de nó suportado como selector`, com a lista completa
de tipos suportados registrada em stderr. Sua expectativa exploratória de
callback foi refutada; não é falha de transporte. Isso explica o booleano
conservador `direct_controls_observed_in_baseline: false` e o campo reutilizado
`focal_witness_valid: false` no resumo final; o focal strong anterior permanece
válido e os controles strong/emph demonstram o mecanismo de witness.

Conforme a delimitação posterior do operador, bare text/show deve ser tratado
como controle de dívida antecedente, com seu diagnóstico preservado fora das
18 rotas de selectors desta sonda; a medição não autoriza repará-lo. O manifesto
exploratório que fez a hipótese inicial permanece intacto. Não se exige que o
baseline execute um comportamento que ainda não possui nem se infere solução
interna para fazê-lo.

## Ponte e limites

`p1339-show-witness-bridge.json` mapeia as 144 células originais por ID, perfil e
ordem para índices exatos dos novos registros vanilla/baseline. As 108 células
que antes tinham Unknown de transporte recebem neste sucessor um diagnóstico
semântico real anterior ao callback. **Portanto todos aqueles 108 Unknown têm
agora observação sucessora; nenhum Unknown original foi substituído no registro.**

A sonda demonstra matching/no-matching no vanilla e a rejeição pública antecedente
dos selectors no baseline. Não demonstra matching bem-sucedido de where no
baseline, a morfologia de retorno do callback, preservação da metadata original,
igualdade PDF, número de invocações além da primeira, conservação de Args ou
compatibilidade geral de Selector. O não matching é inferido no fragmento
controlado pela combinação de compilação bem-sucedida sem erro com o caso
equivalente e os controles positivos; sucesso isolado de compile não seria prova.
O Unknown de Angle NaN permanece fora desta sonda e não foi resolvido.

## Artefatos e reprodução

Manifesto final: `p1339-show-witness-final-manifest.json`, SHA-256
`74bf9a08ff01bc19b36ccd36457daf0c2a64b3ce1b3147c66fa940e3a500f342`.
Vanilla: `p1339-show-witness-final-vanilla-runs.json`, SHA-256
`85ed33bd342081c5a995fad2523c207ffbbfabab43dba373b9eb8d631766fbaf`.
Baseline: `p1339-show-witness-final-baseline-runs.json`, SHA-256
`d61646c4c16054ff60a7727b5085e0e0dacf64ece5220aca94f5dbc77f2a92cd`.
Ponte causal: `p1339-show-witness-bridge.json`, SHA-256
`ed9c95202086c9f755b82d9fa7a395ad8b497f5d981503e8c0842279dc99fc6d`.

Os registros conservam argv, stdin vazio, stdout/stderr integrais, UTC,
identidade binária, fonte e hash, cwd, HEAD/status/diffstat e hashes dos PDFs
quando produzidos. A ponte contém hashes dos scripts e de todos os predecessores,
mais tempo/processo e estado Git. O runner é
`python3 -B 00_nucleo/diagnosticos/p1339-show-witness-probe-r1.py` com fases
`prepare`, `run`, `summarize`; os arquivos existentes são deliberadamente
imutáveis e o runner recusa sobrescrevê-los. Para reprodução pontual, usar os
argv registrados com outro caminho de saída PDF dedicado.
