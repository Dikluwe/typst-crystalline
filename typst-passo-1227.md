# P1227 — adjudicar paint servers SVG com isolamento Tekt atestado

**Estado:** EXECUTADO — PARIDADE PARCIAL DOCUMENTADA  
**Predecessor causal:** P1226  
**Cluster:** `svg-morphology`, `PARTIAL`  
**Fresta:** gradients, tilings e referências `url(#…)` ainda são opacos para a
lente e não possuem alegação pública de paridade no mapa DSM.

## 1. Objetivo

Medir a superfície Typst pública de paint servers no vanilla ratificado e no
cristalino, ampliar o contrato morfológico SVG sem comparar IDs mecânicos e
isolar qualquer gap do produto. A execução deve usar o protocolo completo de
materialização segregada com separação verificável de entradas, escritas,
ordem e contexto.

Resultado esperado:

```text
SVG PAINT SERVERS ADJUDICATED — SEGREGATED AND ATTESTED
```

Se o ambiente não permitir o isolamento descrito neste passo, executar apenas
como ensaio e terminar com:

```text
EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO
```

## 2. Gate de entrada: baseline imutável

A árvore após P1226 está atualmente não commitada. Portanto é proibido criar
worktrees a partir de `HEAD` e alegar que elas representam o produto medido.
Antes de distribuir papéis, produzir uma destas identidades imutáveis:

1. commit explícito do dono contendo o estado aprovado; ou
2. snapshot somente leitura com manifesto de todos os paths e SHA-256, mais
   receita reproduzível para restaurá-lo.

Registrar:

- identidade do snapshot/commit;
- `git diff HEAD --stat` e lista de ficheiros não rastreados no instante do
  congelamento;
- horário ISO-8601;
- vanilla `a51e02804` e SHA-256 do binário;
- SHA-256 do cristalino, lente DSM e comparador SVG;
- hashes do passo, L0 SVG, mapa DSM e fila de implementação.

Mudança em qualquer entrada protegida invalida todos os selos descendentes.
Não ler `00_nucleo/context/` nem `00_nucleo/materialization/`.

## 3. Topologia obrigatória de isolamento

Criar ambientes independentes a partir do mesmo baseline imutável. Cada papel
recebe contexto conversacional novo e somente a allowlist declarada.

| Papel | Ambiente | Pode ler | Pode escrever | Não pode receber |
|---|---|---|---|---|
| C — contrato | worktree/sandbox `p1227-contract` | passo, baseline SVG vanilla, L0 vigente, ADR-0107/0108 | somente contrato e manifesto | patch candidato |
| O — oráculos | `p1227-oracles` | passo + contrato candidato selado | fixtures e resultados vanilla | código cristalino candidato |
| A — adversário | `p1227-attacks` | passo, contrato, baseline | mutantes e testemunhas | solução do implementador |
| I — implementador | `p1227-impl` | passo, L0 e contrato já selado | L0/produção/testes próprios permitidos | oráculos privados e mutantes antes do patch |
| T — A/B independente | `p1227-ab` | passo congelado + dois binários finais | recibos A/B | diff/patch da implementação |
| V — verificador | ambiente somente leitura | manifesto, selo, outputs e hashes | certificado novo, fora dos artefatos julgados | permissão para corrigir entradas |

Fusões proibidas:

- C + I + V sob a mesma autoridade de escrita;
- I com escrita em contrato, fixtures independentes ou mutantes;
- T com acesso ao patch antes de congelar suas sondas;
- V com escrita nos artefatos que verifica;
- troca de mensagens livres entre papéis depois do congelamento.

Comunicação entre papéis ocorre exclusivamente por artefatos canônicos com
hash. Nomes diferentes de agentes sem separação de capacidades não contam como
isolamento.

## 4. Manifesto de capacidades

Antes de qualquer contrato, produzir
`p1227-tekt-manifesto.tsv` contendo para cada papel:

- executor e identificador do ambiente;
- paths legíveis e graváveis;
- contexto herdado (`none` deve ser preferido);
- hashes das entradas;
- instante de início e predecessor causal;
- comandos disponíveis;
- rede habilitada ou desabilitada;
- artefatos esperados.

O manifesto é selado antes do papel C começar. Capacidade adicional não
prevista invalida a alegação de isolamento.

## 5. Sonda pública congelada

O papel O mede, sem conhecer o patch candidato:

1. gradient linear em fill;
2. gradient radial em fill;
3. gradient conic quando exportável para SVG;
4. gradient em stroke;
5. dois elementos reutilizando o mesmo paint;
6. paints estruturalmente iguais construídos separadamente;
7. stops coincidentes e stops fora de ordem quando a linguagem aceitar;
8. alpha nos stops;
9. espaços sRGB, linear-rgb, luma, oklab/oklch, HSL/HSV e CMYK conforme
   aceitação pública;
10. transform do elemento e transform do gradient;
11. relative `auto`/self/parent quando observável;
12. tiling em fill e stroke;
13. nesting e clipping quando surgirem no corpus;
14. referência quebrada e ciclo sintético, marcados `harness-only`.

Para cada fixture registrar comando, stdout, stderr, exit, paths do artefato e
SHA-256. Executar duas vezes antes de fornecer qualquer resultado ao papel I.

## 6. Contrato morfológico do papel C

O contrato não compara IDs, ordem textual de `<defs>`, whitespace ou nomes de
símbolos. Deve resolver cada `url(#id)` para uma definição local e representar:

- tipo: linear, radial, conic aproximado explicitamente identificado, pattern;
- coordenadas e unidades;
- transform composta na ordem correta;
- spread/repeat;
- stops ordenados com offset, cor e alpha;
- geometria e conteúdo de tiling/pattern;
- identidade de compartilhamento somente quando ela for observável;
- cadeia de `href` resolvida, com detecção de referência quebrada e ciclo.

Estados:

- `Preserved`: paint resolvido e aplicação à geometria concordam no nível da
  linguagem dentro da tolerância selada;
- `Violated`: tipo, coordenada, transform, stop, alpha, repeat ou aplicação
  diverge;
- `Unknown`: paint server não modelado, ciclo, referência externa, conteúdo
  opaco ou equivalência que dependeria apenas de aproximação visual.

Conic SVG não pode ser declarado igual a mesh/pattern por inspeção visual.
Exige contrato de erro geométrico/cromático separado ou permanece `Unknown`.

## 7. Gate discriminatório antes da implementação

O papel A entrega no mínimo estes mutantes:

1. comparar `url(#id)` literalmente;
2. aceitar referência quebrada;
3. não detectar ciclo de `href`;
4. inverter stops;
5. ordenar stops que a linguagem preserva;
6. remover stop coincidente;
7. zerar alpha;
8. converter espaço de cor sem contrato;
9. trocar linear por radial;
10. trocar focal point radial;
11. ignorar gradient transform;
12. comutar transforms;
13. aplicar transform duas vezes;
14. ignorar units/relative;
15. trocar repeat por pad;
16. confundir fill e stroke;
17. fundir paints iguais quando compartilhamento é observável;
18. separar paints compartilhados quando identidade é observável;
19. aceitar pattern com conteúdo opaco;
20. promover `Unknown` a `Preserved`.

O verificador pré-implementação executa positivos, negativos e opacos em ordem
aleatória e repetida. Gate de selo:

```text
mutation_score = mutantes_válidos_rejeitados / mutantes_válidos = 1.0
```

Mutante inviável sai do denominador somente com justificativa individual.
Sobrevivente bloqueia o selo e a implementação não começa.

## 8. Selo do contrato

Produzir recibo contendo hashes de:

- manifesto;
- passo;
- baseline;
- contrato;
- oráculos positivos e opacos;
- conjunto adversarial;
- runner;
- resultado do gate e mutation score.

Após o selo, contrato, fixtures e ataques tornam-se somente leitura para I.
Qualquer alteração reinicia a cadeia a partir do primeiro artefato afetado.

## 9. Implementação condicionada à medição

O papel I recebe somente passo, L0 e contrato selado.

- Se o corpus público já for `Preserved`, não alterar L1–L4; ampliar somente a
  lente e documentação.
- Se houver `Violated`, localizar a primeira perda entre eval, layout e export.
- Ler o L0 proprietário e medir `file:line` antes de decidir.
- Atualizar L0 antes do código.
- Acionar novo gate humano ADR-0127 para contrato público, comportamento por
  defeito, mudança de fase ou incompatibilidade.
- Correções internas de paridade seguem RED→GREEN em fluxo contínuo.

É proibido modificar a emissão para facilitar o comparador, copiar IDs do
vanilla ou reduzir gradients a uma cor sólida para obter igualdade aparente.

## 10. Testador A/B independente

O papel T congela seus testes antes de receber os binários finais. Ele conhece
o passo, mas não o patch. Após congelamento recebe somente:

- binário vanilla ratificado;
- binário cristalino candidato;
- hashes e comandos de execução.

T executa o corpus duas vezes, embaralha a ordem e publica apenas recibos. Não
faz correções. Resultado instável ou `Unknown` inesperado não é sucesso.

## 11. Integração e verificação somente leitura

O integrador aplica o patch I sem permitir que I altere os artefatos selados.
O papel V verifica:

1. hashes do selo ainda idênticos;
2. capacidades reais iguais ao manifesto;
3. ausência de leitura proibida documentada;
4. `mutation_score = 1.0`;
5. repetição A/B determinística;
6. alegação DSM limitada ao corpus público;
7. `Unknown` preservado nos casos opacos;
8. nenhuma extensão cristalina misturada à paridade vanilla;
9. L0/consumer 1:1, núcleos e hashes válidos.

V não pode editar o material julgado. Falha produz certificado `FAIL`, nunca
reparo silencioso.

## 12. Artefatos

```text
00_nucleo/diagnosticos/p1227-tekt-manifesto.tsv
00_nucleo/diagnosticos/p1227-svg-paint-contract.tsv
00_nucleo/diagnosticos/p1227-svg-paint-oraculos.tsv
00_nucleo/diagnosticos/p1227-svg-paint-fixtures.tsv
00_nucleo/diagnosticos/p1227-svg-paint-ataques.tsv
00_nucleo/diagnosticos/p1227-svg-paint-resultados.tsv
00_nucleo/diagnosticos/p1227-svg-paint-capacidades.tsv
00_nucleo/diagnosticos/p1227-svg-paint-selo.tsv
00_nucleo/diagnosticos/p1227-svg-paint-ab-recibos.tsv
00_nucleo/diagnosticos/p1227-svg-engineering-choices.tsv
00_nucleo/diagnosticos/p1227-tekt-certificado.tsv
00_nucleo/diagnosticos/typst-p1227-svg-paint-servers.md
```

## 13. DSM e fila

- adicionar `svg-paint-servers` somente após A/B público GREEN e certificado
  `PASS`;
- separar `public` de `harness-only` em cada evidência;
- limitar a alegação aos tipos, espaços e relative modes medidos;
- manter conic/pattern opacos como `Unknown` quando faltar contrato;
- atualizar a fila com a primeira fresta material restante.

## 14. Gates finais

```text
python3 -m unittest discover -s lab/parity/matrix -p 'test*.py'
python3 lab/parity/matrix/runner.py --case P1138-X-001
cargo test -p typst-core p1227
cargo test -p typst-infra p1227
cargo test -p typst-wiring p1227
cargo test --workspace
cargo build --workspace
cargo fmt --all -- --check
git diff --check
crystalline-lint --checks v5,v15,v26 --fail-on warning .
crystalline-lint .
../tekt-cargo-dsm/target/release/lente --comparar \
  --antes lab/typst-original --depois . \
  --mapa-correspondencia 00_nucleo/diagnosticos/dsm/typst-correspondencias-v1.toml
```

Executar cada gate determinístico duas vezes. Registrar comando, ambiente,
entradas, exit e hashes dos outputs.

## 15. Critério de encerramento

O passo só recebe `SEGREGADO E ATESTADO` quando:

- baseline e capacidades foram congelados antes do contrato;
- C/O/A não viram o patch candidato;
- I não escreveu contrato/oráculos/ataques;
- T congelou testes sem ler o patch;
- V operou somente leitura;
- mutation score válido foi `1.0`;
- entradas seladas permaneceram byte-idênticas;
- o certificado limita explicitamente o fragmento observado.

Qualquer violação de capacidade, contexto ou ordem rebaixa obrigatoriamente o
resultado para `EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`, ainda que todos os
testes funcionais passem.
