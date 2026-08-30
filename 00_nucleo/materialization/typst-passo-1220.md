# P1220 — localizar e fechar a primeira divergência de `svg-morphology`

**Estado:** EXECUTADO — `plain.typ` MATCH FOCAL; SHAPES PERMANECEM `Unknown`  
**Predecessor causal:** P1219  
**Cluster:** `svg-morphology`, atualmente `PARTIAL`  
**Caso inicial:** `P1138-X-001`  
**Owner L0 vigente:** `00_nucleo/prompts/infra/export/svg.md`  
**Consumer:** `03_infra/src/export/svg.rs`

## 1. Objetivo

Reexecutar o caso SVG congelado, decompor a diferença da árvore em unidades
semânticas e identificar a **primeira divergência morfológica real** entre o
vanilla ratificado e o cristalino.

Depois da classificação, corrigir somente esse primeiro desvio se ele for
legitimado pelo L0 vigente ou por uma atualização interna de paridade no
próprio passo. Diferenças mecânicas devem ser normalizadas/documentadas;
diferenças ainda opacas permanecem `Unknown`.

O passo não promete fechar todo `svg-morphology`. Seu resultado preferido é:

```text
FIRST SVG MORPHOLOGY GAP GREEN — CLUSTER REMAINS MEASURED
```

## 2. Baseline e proveniência

Congelar antes de alterar harness, L0 ou exportador:

- vanilla ratificado `a51e02804` e SHA-256 do binário;
- HEAD, horário, `git status --short` e `git diff HEAD --stat` completos;
- SHA-256 do binário cristalino, runner, manifesto, fixture, L0, consumer,
  lente e mapa DSM;
- comando `python3 lab/parity/matrix/runner.py --case P1138-X-001`;
- os dois SVGs brutos, a árvore atual do runner e a saída JSON;
- fonte vanilla `lab/typst-original/crates/typst-svg/` necessária para
  explicar o primeiro nó divergente;
- artefatos P1138, P1210, P1213 e fila vigente.

Toda contagem usada para decidir deve registrar o estado exato da working
tree. Não reutilizar as contagens P1138 como prova atual: elas são apenas
hipóteses históricas.

## 3. Observável e política de `Unknown`

Paridade SVG é morfologia/render da linguagem, não igualdade de bytes, nomes
Rust nem passos do algoritmo (ADR-0107). O contrato deste passo observa:

- ordem de pintura quando altera composição visual;
- geometria, transforms, clipping e opacidade;
- fills/strokes e fundo da página;
- outlines e posicionamento dos glifos;
- conteúdo e ligação efetiva de `<use>`, `<image>`, links e recursos;
- dimensões/viewBox e estrutura que afeta o render.

São candidatos mecânicos, sujeitos a prova:

- whitespace, indentação e ordem irrelevante de atributos;
- prefixos de namespace equivalentes;
- nomes arbitrários de IDs, desde que referências sejam renomeadas de forma
  bijetiva e preservem o mesmo grafo;
- escolha equivalente entre atributo default omitido e explícito;
- precisão decimal somente quando render/geometry provar equivalência dentro
  de tolerância justificada.

Se não for possível provar que uma diferença é observável ou mecânica, marcar
`Unknown`. `Unknown` não fecha nem vira MATCH por default.

## 4. Corrigir primeiro o poder discriminatório do harness

O normalizador vigente remove `id`, mas ainda preserva referências como
`href="#glyph-..."`; portanto pode declarar DIFF por renome parcial do mesmo
grafo. Antes de diagnosticar o exportador, criar uma comparação SVG própria e
regenerável que:

1. parseie XML com namespaces;
2. faça renome alfa bijetivo de todos os IDs e referências locais
   (`href`, `xlink:href`, `url(#...)`, clip/mask/filter/paint servers);
3. preserve ordem dos filhos;
4. preserve tags, texto e atributos semanticamente relevantes;
5. não arredonde números silenciosamente;
6. produza path estrutural para a primeira diferença;
7. reporte diferenças adicionais por categoria sem deixar a primeira ser
   escolhida pela ordem de serialização de dict;
8. rejeite referências quebradas, colisões e renome não bijetivo.

Produzir:

```text
00_nucleo/diagnosticos/p1220-svg-normalization-contract.tsv
00_nucleo/diagnosticos/p1220-svg-diff-inicial.tsv
```

O contrato do normalizador deve conter casos positivos, negativos e opacos.
Mudança no runner é harness, não correção de paridade do exportador.

## 5. Decomposição obrigatória da diferença

Classificar cada diferença encontrada, na ordem estrutural, em:

```text
id-graph
namespace
root-canvas
background
grouping
transform
clip-mask
paint
glyph-definition
glyph-use
path-geometry
numeric-precision
metadata
unknown
```

Para cada linha registrar:

```text
rank | tree_path | vanilla_node | crystalline_node | category |
language_or_mechanics | source_file_line | inference |
what_would_refute | disposition
```

Uma diferença de agrupamento só é gap se mudar herança, transform, clip,
opacidade, ordem de pintura, acessibilidade declarada ou render. Um `<g>`
neutro a mais não é automaticamente morfologia diferente.

## 6. Oráculos visuais e geométricos

Para a primeira diferença candidata, construir pelo menos três provas:

1. árvore normalizada focal;
2. render raster dos dois SVGs com o mesmo renderer e escala;
3. extração geométrica específica do nó afetado.

Executar também fixtures de controle mínimas, além de `plain.typ`:

- página vazia/fundo;
- texto de um glifo repetido;
- texto com pelo menos dois glifos distintos;
- forma sólida sem texto;
- grupo transformado ou clipado, se a primeira divergência tocar grupos;
- fundo transparente e fundo explícito, se tocar canvas/background.

Não usar igualdade pixel exata como único oráculo. Registrar dimensões,
bounding boxes, quantidade de pixels diferentes, delta máximo e imagem de
diff, com proveniência. Classificar tolerância somente depois da medição.

## 7. Gate L0 antes de código produtivo

Ler novamente `infra/export/svg.md` após a classificação.

- Se o L0 já determina a forma correta, escrever testes RED e implementar.
- Se o primeiro desvio exigir correção interna de paridade, atualizar o L0
  primeiro e seguir fluxo contínuo ADR-0127.
- Parar se exigir mudança de interface pública, default do produto, nova
  fase do pipeline, compatibilidade ou decisão nova sobre acessibilidade.

O L0 deve registrar somente comportamento perene. Métricas, paths da árvore,
contagens e estado da investigação ficam em `diagnosticos/`.

## 8. Testes RED

Antes do patch produtivo, confirmar RED para o primeiro gap com:

- um teste unitário no owner `03_infra/src/export/svg.rs` ou módulo dono
  descendente;
- um teste de pipeline público que compile fixture `.typ` para SVG;
- comparação contra o observável vanilla congelado;
- pelo menos um controle que impeça corrigir o caso focal quebrando outra
  morfologia já correta.

O RED deve falhar pela diferença classificada, não por IDs, whitespace,
ausência de fonte, path temporário ou ferramenta externa.

## 9. Implementação permitida

Aplicar a menor correção no owner real da primeira divergência:

- `03_infra/src/export/svg.rs` para serialização/morfologia SVG;
- pipeline/layout somente se a medição provar que o Frame/Page entregue ao
  exportador já diverge e o L0 desse owner for atualizado primeiro;
- runner somente para normalização e evidência, sem maquiar SVG produtivo.

Não:

- copiar integralmente `typst-svg`;
- perseguir igualdade byte a byte;
- fixar IDs vanilla literais;
- remover grupos que carregam transform/clip/opacidade;
- arredondar todos os números para fazer o diff desaparecer;
- converter paths em texto ou raster;
- alterar PNG/PDF junto com este gap sem causalidade provada;
- corrigir várias categorias independentes no mesmo passo;
- declarar o cluster inteiro RESOLVED a partir de `plain.typ`.

## 10. Ataques obrigatórios

Produzir:

```text
00_nucleo/diagnosticos/p1220-svg-ataques.tsv
```

Rejeitar, no mínimo:

1. remover `id` sem renomear referências;
2. considerar qualquer ID diferente como gap;
3. ordenar filhos e perder ordem de pintura;
4. ordenar transforms não comutativos;
5. ignorar `url(#...)` em fill/stroke/clip/mask/filter;
6. tratar referência quebrada como equivalente;
7. arredondar coordenadas sem tolerância medida;
8. ignorar fundo diferente;
9. tratar `<g>` neutro como divergência funcional;
10. tratar `<g transform>` como neutro;
11. comparar apenas quantidade de paths;
12. comparar apenas pixels;
13. usar renderer diferente para cada lado;
14. corrigir fixture em vez do exportador;
15. fechar o cluster com um único documento;
16. converter `Unknown` em MATCH.

Exigir `mutation_score = 1.0` para mutantes válidos do normalizador e da
primeira correção.

## 11. A/B final

Produzir duas rodadas independentes:

```text
00_nucleo/diagnosticos/p1220-svg-resultados.tsv
```

Incluir:

- `P1138-X-001` bruto e normalizado;
- fixtures mínimas da seção 6;
- primeiro path divergente antes/depois;
- render e geometria do nó focal;
- controles P1219 para provar que o trabalho de infra não alterou eval;
- pelo menos uma fixture que deve continuar DIFF por uma divergência posterior,
  demonstrando que o harness não foi enfraquecido.

Registrar hashes dos SVGs, árvores normalizadas, renders e imagens de diff.
Repetir a lente duas vezes.

## 12. Mapa DSM e fila

Adicionar uma correspondência específica somente quando a fonte vanilla e o
owner cristalino do primeiro gap estiverem medidos. Exemplo de forma, não de
conteúdo presumido:

```toml
[[correspondencia]]
id = "svg-<responsabilidade-medida>"
antes = ["typst_svg::<owner-real>"]
depois = ["typst_infra::export::svg::<owner-real>"]
relacao = "um-para-um" # ou divisão/consolidação conforme medido
alegacao = "parcial"   # promover só com contrato focal completo
```

Para `svg-morphology`:

- manter `PARTIAL` se qualquer categoria morfológica continuar aberta;
- atualizar `red_evidence` com a primeira divergência restante;
- marcar `RESOLVED` somente após corpus amplo, não neste passo por default;
- documentar extensões cristalinas adicionais sem tratá-las como faltas.

Se a primeira diferença real estiver fora do exportador, abrir cluster
nominal para o owner causal e não escrever um remendo no SVG.

## 13. Artefatos e laudo

Produzir:

```text
00_nucleo/diagnosticos/p1220-svg-normalization-contract.tsv
00_nucleo/diagnosticos/p1220-svg-diff-inicial.tsv
00_nucleo/diagnosticos/p1220-svg-ataques.tsv
00_nucleo/diagnosticos/p1220-svg-resultados.tsv
00_nucleo/diagnosticos/typst-p1220-svg-morphology.md
```

O laudo deve separar claramente:

- diferenças removidas por normalização mecânica comprovada;
- primeiro gap de linguagem/morfologia corrigido;
- diferenças `Unknown`;
- primeira divergência real ainda aberta;
- alcance exato da alegação DSM.

## 14. Gates finais

```text
python3 lab/parity/matrix/runner.py --case P1138-X-001
cargo test -p typst-infra p1220
cargo test -p typst-wiring p1220
cargo test -p typst-core p1219
cargo test --workspace
cargo build --workspace --quiet
cargo build --release --bin typst
crystalline-lint --fix-hashes .
crystalline-lint .
cargo fmt --all -- --check
git diff --check
lente --comparar --antes lab/typst-original --depois . \
  --mapa-correspondencia \
  00_nucleo/diagnosticos/dsm/typst-correspondencias-v1.toml
```

Se o runner manter `expected_state = DIFF`, o gate passa somente quando o JSON
provar que a primeira divergência foi removida e uma divergência posterior
explica o DIFF restante. Não trocar `expected_state` para MATCH antes disso.

## 15. Separação de autoridades

Usar protocolo Tekt completo:

- A congela observável, baseline e política de `Unknown`;
- B escreve contrato do normalizador e mutantes;
- C classifica diferenças sem editar o exportador;
- D atualiza L0 e implementa somente após o selo;
- E executa A/B e produz evidência sem editar o mapa;
- F readjudica fila e correspondência DSM.

Numa única sessão, declarar:

```text
EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO
```

## 16. Continuação

Se restarem diferenças SVG, o próximo passo deve fechar a primeira categoria
morfológica ainda aberta registrada por P1220. Somente quando o cluster SVG
for realmente fechado a fila avança para `smartquote-config`.
