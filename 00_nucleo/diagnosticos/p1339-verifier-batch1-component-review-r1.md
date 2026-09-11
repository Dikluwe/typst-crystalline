# P1339 — auditoria estrutural de componentes do lote 1

Verificador `/root/p1311_review`, 2026-09-10. Executado sem atestação de
isolamento. Somente leitura e checagem mecânica de hashes/definições; zero
execuções semânticas novas. Não é manifesto de lote, gate discriminatório,
selo, RED ou veredito final. APIs futuras permanecem NOT_EXECUTED_PRESEAL,
com zero crédito de runtime.

## Entradas identificadas

- Manifesto r2: `842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b`.
- Contrato r3: `c0cd1826679ddaf77e937a677839c5cbe64fdae6ec71bfa34e635d584d9c3e17`.
- Budget: `4bdd984d7b9d482d68ebb787e98eeed8333a1d4447c637615d4ceb9c78ed6c19`;
  aceite: `0552cdbfb02e18496ac3c07121e0640fad2a772aa32e678a0b7062344286bafd`.
- `p1339-mutant-closed-state-inventory-supplement.json`:
  `87f07ec2951b5d16843161ea08054f08d974f72ef749b34e171854384aa23390`.
- `p1339-ab-batch1-retention-lifecycle-fixtures.json`:
  `f7765b7b7ad42dfe29f646fb8ec08a543d32e39b05b7e121f0bbab1fb46c96c3`.
- `p1339-ab-batch1-public-opaque-fixture.json`:
  `bc6fc67a4b8a9f49a367582fc34c6d125e0478e6bb12a7de8047714025961752`.
- `p1339-mutant-closed-state-relation-harness.rs`:
  `320299b464c2c8ab47a206a7ff15e806afd595d7bdb1dc7488432d856ab6b289`.
- `p1339-implementation-observer-authority.json`:
  `157e6aeee5ba8caec259b861c4c0fc171053091c13a0bc071cd99fbbe7986a05`.

Todos os hashes acima conferidos; fontes contidas nos 12 cenários têm os
hashes declarados. Todos os 36 hashes de fonte do suplemento nominal também
conferem. Números descrevem somente estes artefatos, não cobertura geral.

## Autoridade prospectiva do observador — aceita

O suplemento concreto preserva as condições do aceite prospectivo anterior:
executor `/root/p1312_review`, contexto P1312 não relacionado, escrita somente
em eval/mod.rs e diagnósticos próprios após selo válido que o pine e RED
real, sem contrato/oráculos/veredito/L0 semântico, sem concorrência de escrita
do root no arquivo. Nenhum owner ou módulo novo. Aceito o documento para
pin no selo; autoridade continua dormente. Não há alegação de isolamento
tecnicamente atestado ou independência dos testes locais do implementador.

## Inventário nominal — avanço, não exaustividade

As qualificações distinguem Engine real/stub, Selector público/show,
NodeKind show/syntax, Bytes público/World e Date/time versus CSL. Li as 36
declarações e os aliases. `Route` possui dois métodos no bloco comemo track,
contains e within, ambos &self; o macro efetivamente seleciona
ImmutableConstraint<__ComemoCall> e variantes dos argumentos. Isso sustenta
essa resolução nominal, não torna o controle mutável da Route uma folha
imutável da linguagem.

As declarações de traits são projeções de assinatura: onde havia corpo
default o extrator substituiu `{...}` por `;`. Não são cópias literais desses
corpos nem prova de sua pureza. As demais linhas registradas conferem com
as fontes pinadas. DynElement/PluginHost/World/metrics continuam fronteiras
de recurso; endereço Arc, token u64, Debug ou dyn_eq não provam causalidade.

Ainda falta o mapa completo de variantes/campos/transitividade e testemunhas
estruturais. A coordenação informou que nove payloads Value e HtmlElem ainda
faltam por colisão nome-variante; não se considera o primeiro suplemento
completo. Toda nova folha exposta exige classificação/teste/testemunha antes
do selo, sem preencher por wildcard Same.

## Retenção e ciclo — dois gaps específicos ainda abertos

Os 12 cenários e DTOs foram lidos integralmente. Eles fixam fontes reais,
recursos por compilação, hooks passivos, identidades reais, A1–A5, D5/I4,
retenção de closure, invalidação de descendentes, erros provisórios/finais e
sinks. São definições prospectivas úteis, não execução. A tradução deverá
transformar todos os predicados exigidos em assertions verificáveis e falhar
em campo/evento/identidade ausente; não basta imprimir textos esperados.

1. `lifecycle-request-chain-points` tem dois filhos com uma cadeia por filho.
   Não discrimina duas capturas diferentes **no mesmo corpo/registro** de
   uma implementação que usa sempre a cadeia final daquele corpo. Exigir
   cenário concreto dessa fronteira ou testemunha estrutural específica
   que rejeite a substituição pela cadeia final; a frase global não basta.
2. O orçamento compartilhado com paginação/posição aparece como predicado
   textual de increasing/oscillating, mas essas fontes não exigem uma
   invalidação real de paginação/posição combinada ao ciclo semântico.
   Mapear caso concreto/hook/assert ou testemunha estrutural exata sob F08.
   Não citar a frase como execução de interação que o programa não produz.

Ambos estão dentro da lista fechada do lote 1 e foram comunicados antes do
manifesto. Não há nova execução nem consumo de segunda matriz/lote por
esta leitura. O ledger deve conservar as publicações e correções internas.

## Opaco público e relação privada

O novo opaco constrói dois Callout reais independentes; query(metadata)
captura o valor anterior à demanda filtrada e o overlay troca somente o
elemento no mesmo Location. O pareamento exige as mesmas instâncias reais
para observar Unproven privado e não certificação pública separadamente.
Ok(false) público não é recodificado como Unknown. Erro público conhecido
permitido não dispensa o discriminante privado nem autoriza warning vanilla
fabricado. Admissível como desenho, condicionado à tradução e ao binding
produtivo auditado; nenhum crédito C.

Li integralmente o harness de relação: constructors reais, ordem/presença
Args, spans reais com falha em ambiguidade, bits IEEE preservados, scope
retido e Function allocation apenas como observação. O port não implementa
comparador. O relatório retorna failures, portanto o chamador final deverá
assertar sua ausência e conferir cardinalidade/IDs/ordens; a simples execução
da função ou seu retorno não é GREEN. O controle público pendente não pode
sumir ao agregar a suíte. Compile/binding efetivos ainda são futuros.

## Proposta causal CLI empty-vs-bare

É admissível o autor manter o ancestor original e medir controle que troca
somente o primeiro strong.where() por none + padding do mesmo comprimento
em bytes. O controle baseline isola o erro bare posterior no mesmo offset.
Na derivação pré-candidato do expected, exigir exatamente uma linha de
fonte renderizada correspondente ao controle e trocar somente essa linha
pela original; preservar exit/stdout/restante stderr/span integrais e
guardar mapa, raw e expected derivado. Nenhuma normalização da saída do
candidato. Substituição ambígua ou contexto adicional exige falha fechada.

Conjugar bare-selector baseline, strong.where vanilla e heading empty/bare
vanilla separadamente. Não é ponte same-read-graph nem paridade global.
Esse aceite é da proposta; inputs concretos e medição focal do lote ainda
precisam ser auditados.
