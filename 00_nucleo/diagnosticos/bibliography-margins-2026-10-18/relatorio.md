# A2 — prioridade das margens de blocos e títulos

Concluída a correção das margens relativas automáticas versus explícitas.
O objetivo **Faça todas as correções** e a bibliografia inteira continuam abertos.

## Resultado

Margem explícita agora prevalece sobre automática, mesmo menor ou zero.
Margens da mesma classe conservam o máximo. A presença causal de above/below/
spacing determina a classe; não o valor numérico. Heading usa sua margem
explícita no mesmo protocolo. O título da bibliografia e seu primeiro corpo
passam às posições do vanilla nos seis controles de spacing/estilo medidos;
a correção também vale para blocos comuns.

- Com par.spacing 2em e Block below0→Block auto, BODY passou de 37.064 para
  13.064pt, como o oracle. Below3 passa de 37.064 para 16.064pt.
- Heading→Block auto deixou de acrescentar 15pt indevidos; o corpo está em
  25.004pt. Block auto→Heading deixou de acrescentar 2.4pt; título em 33.3128pt.
- No controle IEEE 2em, título 35.7128→33.3128pt e primeiro corpo 71.9→54.5pt.
  No controle padrão 200pt da etapa anterior, foi removido o excesso de 5.4pt
  abaixo do título. O controle regional 2em/70pt passou de quatro para três
  páginas, coincidindo com o oracle, sem trocar a estratégia de quebra. Resíduos de métricas mistas não foram escondidos.

## Alterações e limites

Dois consumers existentes e seus dois L0s, mais um núcleo compartilhado.
Block é dono do helper puro privado collapse_margin; Heading o chama.
Não mudaram campos/assinaturas públicos, modos, flags, fases ou representação
pública. Não houve offset especial de bibliografia, supressão de lint ou
alteração de estratégias de quebra. Checkpoints e o protocolo de saída já
transportavam a classe; nenhum novo slot foi necessária.

L0s foram atualizados após medir, antes dos testes e da implementação.
A ADR0127 permite fluxo contínuo para esta correção interna de paridade.
O primeiro preparo de L0 tinha pin e ordenação incorretos, detectados pelo
V26 depois da escrita do patch: faltou executar esse gate antes da implementação.
O grafo foi corrigido antes de --fix-hashes, e a materialização final passou
no gate e na revalidação. A primeira tentativa não é apresentada como válida. A prova fonte→binário não depende de versão.
As três fontes vanilla consultadas foram comparadas byte a byte ao objeto
Git pinado; hashes em oracle-sources.json.

## Validação

**9011 testes passaram, zero falhas, 7 ignorados;
cinco doctests.** Os seis novos testes incluem três REDs funcionais antes do
código e três controles que já passavam; depois todos passaram. A compilação
inicial da fixture exigiu f64 e uma premissa de fonte 12 foi corrigida para
chain 11, antes do RED causal. Essas tentativas não são prova de defeito de
produção; snapshots e logs estão preservados em notas-fixtures.md.

Os 72 casos genéricos coincidem em texto, páginas, caixas e alturas com o oracle
(tolerância numérica 0.001pt de extração). Outros seis de bibliografia verificam
título e primeiro corpo, sem certificar linhas posteriores ou hanging.
Os 16 controles extras incluem nested/Box, regiões 70/150pt e títulos com
simple/optimized e IEEE/MLA. Nos 14 que não são MLA, a atribuição de página e a geometria passaram
pelo critério causal registrado em audit-method.md. Os dois MLA continuam falhas de paridade: a primeira auditoria
parou em Testing, já numa linha errada por falta do recuo francês. Remover 5.4pt
da origem melhora o início do corpo, mas aumenta a distância y absoluta dessa
palavra mal quebrada. A classificação posterior exige origem inicial coincidente
e translação única sem mudar nenhuma posição relativa, palavra ou página
interna. Isso foi provado, não é fechamento de hanging nem melhora universal
por palavra. first-audit.py/log e mla-residual-classification.json preservam
a falha e a análise. Nenhum código produtivo foi modificado após essa coleta.

A primeira suíte ampla teve 40 falhas de harness em cinco targets: 34 exigiam
TMPDIR explícito e seis exigiam TYPST_VANILLA. Faltaram essas entradas no comando
de validação; não foram alteradas expectativas. As falhas observadas ocorreram
antes dos asserts do produto afetados.
Os cinco targets foram repetidos com oracle verificado e TMPDIR em /dev/shm:
46 testes passaram. O total acima substitui os resultados desses targets,
reaproveitando os outros 8965 testes aprovados sobre os mesmos hashes congelados,
sem dupla contagem. workspace.log preserva a rodada inicial;
workspace-retry.log, harness-config.json e finish-results.json guardam a correção.

Build workspace, Clippy all-targets com warnings/expectativas negados, Rustdoc
com warnings negados, fmt, V1/V5/V15/V26, reversa 988 e diff-check passaram.
Os 2099 arquivos produtivos/L0 foram congelados durante toda
a validação. O manifesto predecessor foi verificado contra artefatos imutáveis
e, para fontes alteradas, contra os bytes iniciais deste lote.

| Tekt | Antes | Depois |
| --- | ---: | ---: |
| V16 erro | 105 | 105 |
| V19 nota | 1122 | 1122 |
| V20 nota | 876 | 876 |

No binário do pin do CI, nenhum erro/warning novo: 105 V16 anteriores continuam
bloqueando o gate. Uma execução pelo PATH global usou outro binário e apontou
114 V16. Os nove achados adicionais estão em consumers idênticos ao estado
inicial deste lote; não são introduções do patch. Isso não os descarta: a
lista com L0s e hashes está em global-linter-findings.json, para investigação
própria. A revisão-fonte do executável global não foi provada. O diferencial
final foi repetido explicitamente com o binário pinado já verificado na etapa
anterior, sem alterar o pin do CI ou suprimir os achados globais. Zero violations
global não foi alcançado nem alegado.

## Pendências

O recuo francês (hanging-indent) continua sem aplicação física, assim como
seus overrides causais em par/set. Persistem diferenças de métricas mistas,
continuação, múltiplas bibliografias e clusters de citações. Não é prova de
paridade universal da bibliografia. R3-T1 permanece aguardando a aprovação
já solicitada. Os demais E01–E20, contratos públicos e V16 permanecem no escopo.
A ordem está em [proximos-passos.md](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-margins-2026-10-18/proximos-passos.md).

## Recursos e proveniência

Pico dos jobs limitados: **3356434432 bytes (3.126 GiB)**.
MemoryMax 12 GiB/High 8 GiB/swap 0/jobs 1/debug 0/incremental 0; nenhum evento
high/max/OOM/OOM-kill. Não é uso total da máquina ou benchmark comparado.
Comandos pesados sequenciais; recursos/UTC e comandos em *-resources.json e
results.json. A medição inicial usa o binário congelado bin/typst-before.

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, branch `Tekt`, working tree não commitado.
before-sources.json.gz e after-sources.json.gz guardam todos os fontes/L0s;
status/diffstat/UTC, freeze.json.gz, change.patch.gz e final-state.json.gz
registram a árvore exata. Fontes/argv, textos e caixas antes/depois constam dos
JSONs gzip; PDFs anteriores foram arquivados antes da nova coleta.

Oracle bd5ebec285505889e8ff1d4f68804ab147a699f3, árvore
cac836c07e4dec2a89219370428292393a3216b5, binário
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. Tekt final
`2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095`.
Sem commit, push, merge, publicação ou release.
