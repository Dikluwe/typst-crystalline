# P1339 — decisão necessária após a retificação de ângulos

## Resultado delimitado

A autorização «Autrorizo» foi aplicada à correção `Angle → float` no passo,
preservando as dez rotas. Não autoriza mudar a política de `Unknown`, ampliar
entidades públicas ou corrigir aritmética fora do lote. A execução permanece
anterior ao contrato, selo e candidato; nenhum L0 ou código produtivo foi
alterado. Este diagnóstico não é certificado de paridade.

Regime da skill `tekt-materializacao-segregada`: ensaio executado sem
atestação de isolamento. A skill exige decisão humana quando interpretações
da intenção são incompatíveis; por isso a ambiguidade abaixo não foi resolvida
pela interpretação que permitiria avançar mais facilmente.

## Proveniência das medições usadas aqui

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, branch `Tekt`.
`git diff HEAD --stat` vazio durante as sondas: passo e artefatos P1339 são
não rastreados; não há working tree produtiva candidata. Os recibos abaixo
contêm `provenance` e `end_provenance`, status integral, identidade dos
binários, manifesto, comandos, UTC, exit, stdout e stderr por execução.
O manifesto de retomada é `p1339-authority-manifest-r1.json`; a verificação
do produto antecedente e dos hashes históricos está em `p1339-resume-r1.json`.

Referência de linguagem: vanilla ratificado `a51e02804`, binário
`/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Antecedente cristalino: `/tmp/p1338-target.vlNAmp/release/typst`, SHA-256
`f7c8085f8453ecb6b10841b698092d41e7fbec64d9d46f9341b9b9b538bfefa1`.

## Medição: NaN no receiver de ângulo

Os recibos `p1339-full-boundaries-vanilla-runs.json` e
`p1339-full-boundaries-crystalline-before-runs.json` cobrem esta fronteira
em graus e radianos, nas ordens normal/inversa e nos perfis default, html,
a11y e html+a11y. Intervalos UTC completos desses recibos:
`2026-09-09T23:37:26.856085+00:00`–`23:37:28.924619+00:00` e
`23:37:28.975501+00:00`–`23:38:00.599217+00:00`.

Para `float("nan") * 1deg`, `0deg * float("inf")` e
`float("inf") * 1deg - float("inf") * 1deg`, a observação
`repr((type(a), a, a == 0deg, a / 1deg, (a / 1deg) == (a / 1deg)))`
devolve, em conteúdo da string:

- vanilla: `(angle, 0deg, true, 0.0, true)`;
- cristalino antecedente: `(angle, float.nan * 1deg, false, float.nan, false)`.

As construções equivalentes em radianos têm o mesmo resultado. Infinito é
construível nos dois binários; não pertence à limitação NaN.

Fonte que explica a fronteira, lida depois da sonda:
`lab/typst-original/crates/typst-library/src/layout/angle.rs:25,35,40`
transporta `Scalar`; `lab/typst-original/crates/typst-utils/src/scalar.rs:27-31`
documenta e implementa normalização de NaN para zero. Esse arquivo tem SHA-256
`e3b7bcf23af0ae7fedd312c74fa21b0a65cc14f1cfd62bec556155f75c902891`.
No cristalino, `01_core/src/entities/layout_types.rs:1277-1290` guarda f64,
e `01_core/src/compiler/eval/operators/arithmetic.rs:236-238,328-329`
constrói o resultado sem essa normalização.

Classificação: a diferença observável da construção é `Violated` e é dívida
aritmética antecedente, não falha de implementação das conversões ainda
ausentes. Já a comparação das conversões com um receiver NaN comum fica
`Unknown`: nenhuma construção bilateral desse receiver foi demonstrada.
Não chamar o zero vanilla de NaN, não transformar ausência de receiver em
teste aprovado e não corrigir essa aritmética incidentalmente.

## Decisão pendente sobre a obrigação

O passo corrigido, SHA-256
`817c3a1476897fb0a847c90183e9a9fe690994f60023126997c8022c4e8b86a9`,
em `:121-122`, condiciona NaN/infinito à construção bilateral e manda
registrar `Unknown` quando ela não existe. Em `:184-186`, bloqueia `Unknown`
obrigatório e admite `Unknown` em controles deliberados de opacidade.

Há duas leituras relevantes: a condição torna NaN não obrigatório quando
não construível, ou NaN continua uma obrigação bloqueante da lista mínima.
O texto não identifica previamente esse caso como controle de opacidade.
Não adotamos unilateralmente a primeira leitura nem declaramos impossível
todo o passo. A correção anterior não mudou essas cláusulas.

**Proposta para decisão do dono, ainda não aplicada:** explicitar que somente
o receiver Angle NaN sem construção bilateral demonstrada permanece
`Unknown` não bloqueante como limite de observabilidade; manter a divergência
de construção como dívida aberta fora do lote. Infinito, NaN de float, os
demais casos obrigatórios e as dez rotas continuam exigidos. Isso não seria
uma prova de paridade NaN de ângulos. Alternativamente, manter o bloqueio e
exigir uma construção pública bilateral válida antes de retomar.

## Medição distinta: o contrato de `function.where`

`p1339-full-where-early-{vanilla,crystalline-before}-runs.json` demonstra
seletores válidos para `strong`, `emph` e `text` no vanilla e rejeição no
antecedente cristalino. Filtro vazio tem morfologia `.where(:)`, não o nome
nu do elemento. As execuções começam em `2026-09-09T23:34:25.980546+00:00`
e terminam em `23:35:23.124431+00:00`.

`p1339-full-show-final-{vanilla,crystalline-before}-runs.json` e as fixtures
correspondentes verificam realização no vanilla: regras estáticas e ligadas
produzem metadata `MATCH` para filtro vazio/campo coincidente, e não produzem
esse metadata para campo divergente. É observação de aplicação de show, não
inferência de render a partir do `repr` de conteúdo não realizado. No
cristalino, somente default chegou à rejeição semântica; o transporte
`query --features` falhou nos outros perfis. Essas células são `Unknown`,
não prova de rejeição do seletor. A revisão focal com `eval --in` também
falhou no transporte CLI e foi preservada, não substituiu o recibo anterior.
Não é preciso nem legítimo declarar esse suplemento completo para expor
a lacuna pública. O relatório do autor `p1339-full-a2.md` e seu recibo de
entrega distinguem as execuções e limitações finais.

A checagem focal `p1339-full-show-compile-focal-*.json` já estava concluída
quando o operador pediu a interrupção de novas tentativas: apenas a fixture
`strong-static-match`, compilada nos perfis existentes, chegou à rejeição de
`function.where` no cristalino e compilou no vanilla. Essa evidência focal
não observa o valor do metadata, não substitui as células `Unknown` e não
fecha o suplemento inteiro. Nenhuma rodada posterior foi solicitada.

No produto vigente, `01_core/src/entities/selector.rs:28-61` não representa
essas identidades; `ElementKind` não contém Strong/Emph/Text. Filtros
`Where` só restringem uma base existente e não criam a identidade ausente.
`show::NodeKind` pertence a outro carrier. Ver o inventário e as alternativas
refutadas em `p1339-protocol-review-r1.md`.

Portanto a fase B precisará de desenho explícito de L0 e gate público
ADR-0127; a autorização para retificar conversões não é essa aprovação.
Não foi adotado um desenho nem editado L0 nesta retomada, pois a política
de observabilidade ainda requer decisão na fase A. A hipótese de revisão
`Selector::Element { function: Func, fields: Dict }` não é assinatura pronta:
o cristalino não tem entidade Rust `Dict`; `Value::Dict` usa `IndexMap`.
Representação, identidade, igualdade, grupo vazio, ordem e consumers devem
ser resolvidos no desenho, sem registry genérico nem sentinelas.

## Ponto de retomada

Resolver primeiro a obrigação NaN sem reescrever os recibos congelados.
Depois, preparar L0 da extensão pública de seletor, inventariar os consumers
e pedir a confirmação ADR-0127 sobre o desenho concreto antes do código.
A revisão demonstrou caminhos viáveis para mutantes pré-candidato e para
capacidade somente leitura do verificador; não são motivo para afrouxar
esses gates. Nenhum mutante foi compilado e não há mutation score observado.
Contrato, ataques, selo, implementação, build/testes finais e nova matriz
global continuam pendentes. Nenhum commit foi feito nesta retomada.
