# Estado do lote A2-Structure — trabalho em andamento

Não marcar o objetivo Faça todas as correções completo/pausado/bloqueado.
HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, branch Tekt, grande árvore anterior
preservada. 12GiB/High8GiB/swap0/jobs1/debug0/incremental0, pesados sequenciais.
Sem commit/push/release. Nenhum agente delegado ou skill aplicado neste lote.

## Estado autoritativo

O predecessor bibliography-indent-2026-10-18 foi concluído com8994 testes,5doc,
105V16/1120V19/876V20, pico4.06455GiB. Seu relatório permanece histórico.
Este lote tem before-sources.json.gz completo, primeiro before.json.gz12pares,
mais6 paginação e3 extras (RTL, prefix-only, prefix-formatado). Oracle atual
bd5ebec285505889e8ff1d4f68804ab147a699f3, bin9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9.

Novo plano privado em bib_csl/plan.rs (Columns/Hanging/Plain e entriesprefix/body),
fachadas BibRenderCache/build_cache/build_cache_with_style públicas preservadas.
Um Arc privado é preparado uma vez e compartilhado entre tentativas/checkpoints.
Plano conserva hanging mas o ramo Hanging ainda usa Block sem recuo: continua
A2-HangingTransport/A2-ParIndent. Não declarar A2 inteiro encerrado.

Grid inicial acertou colunas/gap, mas90pt deu3páginas vs2. Arquivado bin/typst-grid
7c01fa4a9d183b371af3efcc0ea3952b4ca386969436c7fa6a37aa968c703434,
grid-freeze/sources.json.gz e grid-artifacts.tar.gz. Fontes Grid reconstruídas
por bytes e TODOS2096 hashes conferidos, ver archive-grid.py.
Block/Place antecipado recuperou2páginas mas perdeu inset nas continuações(x8
vs29.924) e podia deixar prefixo na página anterior à admissão do corpo.
Arquivado bin/typst-block-before c1c222ae54ab3b67487471326012a7f6c4ed86c1e6bd61b1830ec6a987c2ccea,
block-freeze.json.gz completo, block-artifacts e pagination-block/extra-block.

Foram implementados PendingPrefix privado, emitido via word_layout::push_text
ou Text vazio APÓS admissão efetiva, e InlineRegion de Block conservando offset/
width em página/coluna; checkpoint e sub_frame capturam/isolam/restauram.
Três novos núcleos e11 owners L0 (10 existentes+plan) estão em prescription.json.
seal.py calcula escopo exato; V1/V5/V15/V26+reversa988 passaram, green3 deu25/25.
L0s e códigos em layout/{bib_csl,bibliography,mod,checkpoint,convergence,block,
region_progress,sub_frame,text,word_layout}.rs e novo bib_csl/plan.rs.

## Bloqueio de validação atual (não é blocker do objetivo)

validate.py executou build e127? pares (contagens exatas nos JSONs), mas a
primeira auditoria estrita falhou em ieee-none-size (18pt): 2020 termina a
linha e ponto isolado na próxima; oracle mantém2020. junto. NÃO relaxar auditor.
Todos resultados/pdfs/sources/binários foram preservados em attempt1-artifacts,
admission-sources.json.gz (2098 fontes completas conferidas),
bin/typst-admission-before. O executável rootdebug ainda é esse candidato.
validate.py não chegou a Clippy/workspace/docs nesta tentativa.

cache-shape.txt via /tmp/tekt_bib_cache_probe mostra ContentText2020 adjacente
ContentText., sem separador. A raiz Sequence que contém Biblio recusa preparo e
seta paragraph_planning_suppressed=true em todos filhos; Block não reabre o
próprio escopo. L0 Block já atualizado ANTES de teste/código para salvar flag,
instalar false só durante seu body e restaurar após flush. Produto AINDA não
implementou esse reset. Próxima mudança necessária está limitada a Block.

Teste novo a2_structure_block_owns_preparation_after_outer_flow_rejection
foi acrescentado ao módulo a2_inline_region_tests no block.rs. width54pt pode
ser limiar ruim para FixedMetrics; se passar, medir/calibrar para52pt (xx+space+
2020 cabe, ponto não), preservar tentativa e só contar RED funcional.
O job red-block-planning tem handle exec17466 (verificar se já terminal,
não reiniciar por timeout); recibo/run.py segue12GiB. Depois de RED, implementar
save std::mem::replace(&mut paragraph_planning_suppressed,false) antes do body,
restore após flush. Resselo via seal.py antes de GREEN. Green-check.py roda
seal e cargo test --lib a2_ sequenciais. São26 testes focados e total workspace
esperado9005/0/7, mas só reportar após medição. Defaults/assinaturas/fase não
mudam: correção interna/paridade ADR0127 fluxo contínuo.

## Fechamento ainda obrigatório

Após GREEN, executar validate.py novamente (primeira tentativa está arquivada),
medindo todos PDFs e corpus: probe12, paginação6, extra3, legacy60, textual44,
custom6, colors3; audit.py deve passar SEM enfraquecer pontuação. Clippyestrito,
workspace completo TMPDIR/dev/shm,doctests5,Rustdocestrito4índices incluindo typst,
fmt/lineage/reversa988/SARIF diferencial sem novo erro/warning/diff-check.
Se falha só no harness, fontes congeladas intactas e comandos anteriores
passaram, retomar gates sem repetir custos à toa; se source muda repetir provas
relevantes. Complete-freeze/results/summary e recibos dão UTC/head/status/stat.

Escrever close/manifest/relatório deste lote (ainda NÃO existem). Verificar
changed+new contra before-sources:20 existentes (10fontes/10L0)+5novos
(plan.rs/plan.md e3núcleos). Contagens novas confirmadas via source freeze.
Guardar fontes/L0 finais completos e patch. Verificar manifesto predecessor:
artefatos sob pasta antiga intactos; entradas que apontam a código agora
intencionalmente alterado devem comparar com before-sources, NÃO com root atual.
Manifest após wrapper close terminar, para não selar logs enquanto mudam.
Não editar artefatos selados do predecessor ou proposta R3T1.

## Pendências reais que não podem desaparecer do relatório/objetivo

Hanging/default/overrides ainda não aplicados, parágrafos genéricos comhanging,
margem entre título e body (+5.4pt), A2-Multi, clusters de citações adjacentes
(Tekt MLA(Silva)(Doe) vs(Silva;Doe)), diferença de extração RTL de colchetes
preexistente (precisa classificação visual; não assumir defeito de língua só
por pdftotext), alias-style bibliográfico preto vsoraclevermelho(R3T1).
Pós-contexto de continuação IEEE90 já acertoux29.924 masprimeiray=5.012 vs5.167996;
ver se o preparo do body resolve também, caso contrário manter diferença aberta.
Não afirmar paridade universal nem gain RAM só porque pico ficoubaixo.
E01–E20 e frentes públicas RM1/RG/FG/Paint/Symbol continuam objetivo amplo.

R3T1 aprovação ainda pendente, manifesto47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302.
Não pedir duplicado. O antigo r3-t1-overlay.json em bibliography-indent explica
como preservar adendo L0 de testes ao integrar. Este lote não mexe nesses owners.
Proposta e10bases continuam intactas, testsL0base era único overlay da etapaanterior.

## Atualização posterior

red-block-planning com54pt passou (não foi RED); probe público de larguras
confirmou52pt, teste red-block-planning2 falhou como exigido. Produto Block
foi corrigido: salva paragraph_planning_suppressed, instala false durante body,
restaura após flush. green4 confirmou26/26 testes +selagem V1/V5/V15/V26 e
reversa988. Validação completa2 está rodando no handle exec retornado na sessão
40776 anterior já terminou com audit1; NÃO confundir com job atual complete2.
O job atual tem seus recibos complete2-resources.json somente ao terminar.
Scripts close.py/manifest.py estão preparados mas ainda NÃO executados; exigem
9005/0/7 +doc5 e auditoria preservada. Arquivos produtivos25 (20delta+5new).

Handle confirmado da validação complete2: exec95300. Não reiniciar porque
observação expirou; esperar/pollar o mesmo handle ou recibo terminal.
