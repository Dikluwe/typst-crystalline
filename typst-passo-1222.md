# P1222 — fechar geometria SVG analítica e triar uma extensão upstream aberta

**Estado:** EXECUTADO — analytic focal GREEN; NO EXTENSION SELECTED; cluster PARTIAL  
**Predecessor causal:** P1221  
**Cluster de paridade:** `svg-morphology`, `PARTIAL`  
**Fragmento fechado:** rect/line com fill sólido e stroke rico  
**Fronteira:** ellipse/circle, rounded rect, path geral e transforms  
**Trilha adicional autorizada:** no máximo uma issue SVG upstream aberta,
reproduzida e documentada como extensão cristalina — nunca como paridade.
**Liberdade de engenharia:** preparar seams internos que facilitem issues
upstream é permitido quando o caminho padrão mantém paridade de saída.

## 1. Objetivo

Executar duas trilhas sequenciais e semanticamente separadas:

1. ampliar `svg-morphology` até decidir elipse/círculo, retângulo arredondado,
   paths gerais e transforms sem comparar bytes ou tags SVG;
2. depois da readjudicação de paridade, triar issues SVG abertas do Typst e,
   se houver uma melhoria pequena, reproduzível e compatível com a arquitetura,
   implementar **uma** extensão cristalina claramente identificada.
3. aproveitar decisões internas necessárias ao trabalho para deixar owners,
   dados e pontos de extensão aptos às issues triadas, sem alterar a saída
   padrão nem inventar API pública antecipadamente.

Resultado esperado:

```text
SVG ANALYTIC MORPHOLOGY GREEN — UPSTREAM EXTENSION SEPARATELY ADJUDICATED
```

Se nenhuma issue for adequada, a trilha 2 termina com `NO EXTENSION SELECTED`.

## 2. Baseline obrigatório

Antes de editar comparador, L0 ou código:

- congelar HEAD, horário, `git status --short` e `git diff HEAD --stat`;
- registrar working tree não commitado e todos os ficheiros alterados;
- registrar SHA-256 dos binários vanilla e cristalino;
- confirmar vanilla ratificado `upstream/main a51e02804`;
- registrar hashes do comparador, testes, mapa DSM, fila e artefatos P1221;
- congelar a saída da lente com `--mapa-correspondencia`;
- ler `00_nucleo/prompts/infra/export/svg.md` e os L0s dos owners que a
  medição realmente alcançar;
- não ler `00_nucleo/context/` nem `00_nucleo/materialization/`.

Toda contagem decisória deve declarar commit/working tree e horário.

## 3. Correção prévia do mapa para a lente

A lente atualizada aplicou sete correspondências, mas deixou
`svg-plain-page-paint` e `svg-solid-rect-line-stroke` como `unknown` por
cardinalidade incompatível. Antes de usar a lente como evidência:

1. verificar os paths reais emitidos pela lente;
2. repartir cada relação SVG em correspondências de cardinalidade válida;
3. não mudar `parcial` para `declarada-fechada` apenas para satisfazer o parser;
4. repetir a lente duas vezes e registrar o SHA-256 do mapa;
5. manter extensões cristalinas em entradas próprias.

## 4. Corpus de paridade geométrica

Medir no mínimo:

1. elipse com e sem stroke;
2. círculo e elipse de raios diferentes;
3. rounded rect com raio uniforme;
4. rounded rect com raios por canto, se publicamente construível;
5. linha horizontal, vertical e diagonal;
6. path aberto com `M/L/H/V`;
7. path com `C/S/Q/T`;
8. path com `A` em combinações de flags;
9. múltiplos subpaths e `Z`;
10. winding oposto sob `nonzero` e `evenodd`;
11. translate, scale, rotate, skew e matrix;
12. transforms aninhados não comutativos;
13. reflexão e determinante negativo;
14. dimensões zero e negativas quando aceites;
15. duas formas sobrepostas em ambas as ordens;
16. controles `Unknown`: gradient, tiling, clip/mask, image e link.

Cada fixture deve ser compilada pelo vanilla ratificado e pelo cristalino e
produzir operação canônica, hash do SVG e veredito.

## 5. Canonicalizador SVG

Estender `lab/parity/matrix/svg_morphology.py` para:

- tokenizar paths conforme repetição implícita SVG;
- suportar `M/L/H/V/C/S/Q/T/A/Z`, absolutos e relativos;
- converter primitivas analíticas em geometria canônica;
- aplicar transforms na ordem normativa, inclusive grupos aninhados;
- preservar subpaths, fechamento, orientação e ordem de pintura;
- comparar rect/rounded/ellipse/circle com paths equivalentes por regras
  matemáticas explícitas;
- nunca aproximar arco/Bézier por bbox ou raster;
- retornar `Unknown` para sintaxe, paint ou referência não modelada.

Para elipse representada por quatro Béziers, só declarar equivalência se os
pontos de controle seguirem a construção exata documentada pelo emissor. Uma
aproximação visual não prova identidade geométrica.

## 6. Contrato e ataques antes de medir o produto

Produzir antes de qualquer patch produtivo:

```text
00_nucleo/diagnosticos/p1222-svg-analytic-contract.tsv
00_nucleo/diagnosticos/p1222-svg-analytic-oraculos.tsv
00_nucleo/diagnosticos/p1222-svg-analytic-ataques.tsv
00_nucleo/diagnosticos/p1222-svg-analytic-diff.tsv
```

Atacar no mínimo:

- bbox igual com path diferente;
- círculo confundido com elipse;
- rounded rect confundido com rect;
- raio ou flag de arco alterado;
- controle refletido incorretamente;
- shorthand `S/T` expandido com controle errado;
- repetição implícita perdida;
- subpath reordenado;
- winding/fill-rule ignorado;
- transform order comutado;
- matriz aplicada duas vezes;
- stroke/fill/order/opacity alterados;
- path malformado aceito;
- construção `Unknown` promovida a MATCH.

Gate: todos os mutantes válidos rejeitados, `mutation_score = 1.0`.

## 7. Medir antes de decidir código produtivo

Com o comparador selado:

1. executar o corpus duas vezes;
2. classificar `Preserved`, `Violated` ou `Unknown`;
3. localizar o primeiro `Violated` por ordem estrutural;
4. comparar a entrada `FrameItem::Shape` antes do exportador;
5. decidir o owner por `file:line`;
6. corrigir somente o primeiro gap causal;
7. atualizar o L0 do owner primeiro e seguir ADR-0127.

Não corrigir o exportador se a divergência já estiver em stdlib/layout.

## 8. Trilha de issues upstream

Após a trilha de paridade, revalidar no GitHub o estado e o conteúdo destas
issues candidatas:

- [#4702 — texto real/selecionável no SVG](https://github.com/typst/typst/issues/4702);
- [#6035 — texto ausente ao reimportar SVG em PDF](https://github.com/typst/typst/issues/6035);
- [#6858 — SVG aninhado em `<image href>`](https://github.com/typst/typst/issues/6858);
- [#8678 — metadados de documento em PNG/SVG](https://github.com/typst/typst/issues/8678).

Não selecionar issue apenas por estar aberta. Para cada candidata registrar:

```text
issue | estado/data | reprodução upstream atual | reprodução a51e02804 |
cristalino | linguagem/produto/mecânica | owner | custo | risco |
compatibilidade | decisão | o_que_refutaria
```

Produzir:

```text
00_nucleo/diagnosticos/p1222-svg-upstream-issues.tsv
```

## 9. Regra de seleção da extensão

Uma issue só pode virar extensão neste passo se:

- continuar aberta no momento da execução;
- tiver reprodução local mínima e determinística;
- o comportamento vanilla ratificado estiver congelado;
- não exigir rede em runtime;
- não enfraquecer segurança, sanitização ou portabilidade;
- não quebrar o output vanilla por defeito;
- tiver owner e L0 claros;
- puder ser implementada e atacada isoladamente.

Preferência, se todas forem viáveis:

1. correção interna que preserve dados já disponíveis;
2. opção opt-in sem alterar o default;
3. metadado puramente declarativo;
4. embedding/sanitização de SVG externo;
5. mudança ampla de estratégia de texto.

Logo, #4702 não deve ser escolhida automaticamente: texto como `<text>` muda
portabilidade, seleção, dependência de fontes, tamanho e contrato público. #6858
exige auditoria de recursos aninhados e sanitização. #6035 pode pertencer ao
pipeline de importação/PDF, não ao exportador SVG. #8678 altera superfície de
produto e metadados. A medição decide.

## 10. Gate ADR-0127 para extensão

A autorização deste passo permite investigar e propor a extensão, mas não
remove o gate arquitetural:

- nova flag CLI, novo campo público, mudança de default, nova fase ou quebra de
  compatibilidade: redigir/atualizar L0 e **parar para confirmação**;
- correção interna opt-in já legitimada pelo contrato: fluxo contínuo;
- se houver dúvida sobre a classe: parar.

A extensão escolhida deve receber identidade separada, por exemplo:

```text
svg-extension-upstream-4702
```

Nunca marcar uma extensão como evidência de paridade vanilla.

## 10.1. Escolhas de engenharia que preservam a saída

Durante a correção de paridade, é permitido escolher uma implementação interna
que também reduza o custo das issues upstream, desde que todas estas condições
sejam satisfeitas:

- o output padrão permaneça semanticamente equivalente ao vanilla ratificado;
- o corpus de paridade anterior e posterior permaneça `Preserved`;
- a escolha pertença ao owner causal já afetado, sem dependência reversa;
- não adicione flag, campo público, target, fase ou comportamento default;
- não carregue I/O para L1 nem mova sanitização para camada imprópria;
- não generalize tipos apenas por possibilidade futura sem consumidor medido;
- a abstração tenha uso imediato no gap atual e teste discriminatório próprio;
- o laudo declare qual issue futura ficou facilitada e qual trabalho ainda falta.

Exemplos admissíveis, se causais:

- separar a operação canônica de shape da escolha mecânica de tag SVG;
- centralizar escrita de paint/stroke sem alterar atributos emitidos;
- preservar no modelo interno metadado já disponível que antes era descartado,
  sem ainda serializá-lo no caminho padrão;
- criar helper privado para IDs/referências/sanitização usado imediatamente pelo
  fragmento atual;
- manter estratégia de texto atrás de uma interface interna estática, com a
  estratégia vanilla como única seleção produtiva por defeito.

Exemplos proibidos neste passo:

- criar uma flag CLI “para usar depois”;
- emitir `<text>`, metadados ou SVG aninhado no caminho padrão antes do contrato;
- introduzir `dyn`, registry global ou plugin system para hipotéticas extensões;
- preservar XML externo sem limites ou sanitização;
- alterar bytes/whitespace apenas para aproximar o vanilla;
- chamar preparação arquitetural de paridade fechada.

Quando duas soluções corrigirem igualmente o gap, preferir a que:

1. preserva mais informação sem mudar o observável padrão;
2. mantém a decisão de serialização em L3;
3. deixa política pública fora do domínio puro;
4. permite ativação futura explícita e testável;
5. tem menor superfície e menos estados inválidos.

Produzir uma tabela de decisão:

```text
00_nucleo/diagnosticos/p1222-svg-engineering-choices.tsv
```

com:

```text
choice | current_gap | output_before | output_after | upstream_issue_helped |
immediate_consumer | public_surface_change | rejected_alternative | verdict
```

O gate desta tabela é: `public_surface_change = none` e comparação padrão
`Preserved`. Caso contrário, a decisão passa para o gate ADR-0127.

## 11. Testes da extensão escolhida

Antes da implementação:

- reprodução RED do caso upstream;
- controle vanilla ratificado;
- teste de compatibilidade do output padrão;
- teste de sanitização e referência quebrada;
- teste de determinismo;
- teste negativo que impeça ativação acidental;
- pelo menos dez ataques específicos à feature.

Implementar no máximo uma issue. As demais ficam `candidate`, `rejected`,
`blocked` ou `not-reproduced`, com justificativa.

## 12. DSM, fila e laudo

Atualizar o mapa com relações de cardinalidade válida:

- promover apenas os fragmentos SVG comprovados;
- manter `svg-morphology` `PARTIAL` enquanto houver `Unknown` relevante;
- registrar a extensão numa correspondência independente e `parcial` ou
  `fora-de-escopo`, conforme o caso;
- documentar features extras do cristalino sem apagá-las.

Produzir:

```text
00_nucleo/diagnosticos/p1222-svg-analytic-resultados.tsv
00_nucleo/diagnosticos/p1222-svg-extension-resultados.tsv
00_nucleo/diagnosticos/p1222-svg-engineering-choices.tsv
00_nucleo/diagnosticos/p1222-tekt-certificado.tsv
00_nucleo/diagnosticos/typst-p1222-svg-analytic-and-extension.md
```

O laudo separa obrigatoriamente:

- paridade comprovada;
- diferenças mecânicas normalizadas;
- gaps vanilla→cristalino ainda abertos;
- extensão cristalina além do vanilla;
- preparação interna incorporada sem mudança da saída padrão;
- issues upstream não selecionadas;
- próximo `Unknown` nominal.

## 13. Gates finais

```text
python3 -m unittest discover -s lab/parity/matrix -p 'test*.py'
python3 lab/parity/matrix/runner.py --case P1138-X-001
cargo test -p typst-core p1222
cargo test -p typst-infra p1222
cargo test -p typst-wiring p1222
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

Repetir corpus, extensão escolhida e lente duas vezes. Registrar hashes e
proveniência. Falha transitória só pode ser classificada após repetição
isolada e repetição integral.

## 14. Separação de autoridades

Usar protocolo Tekt completo:

- A congela baseline, issues e contrato;
- B implementa canonicalizador e mutantes;
- C sela o poder discriminatório;
- D mede paridade sem editar produção;
- E corrige somente o primeiro gap produtivo;
- F tria issues upstream sem editar produção;
- G escreve L0/testes/implementação de no máximo uma extensão;
- H executa A/B final sem editar mapa;
- I readjudica DSM e fila.

Em sessão única declarar:

```text
EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO
```

## 15. Condição de encerramento

O passo encerra quando:

- ellipse/rounded/path/transforms tiverem veredito honesto;
- o primeiro gap de paridade estiver GREEN ou não houver `Violated` focal;
- as relações SVG da lente tiverem cardinalidade válida;
- uma issue upstream tiver sido implementada separadamente **ou** houver
  decisão explícita `NO EXTENSION SELECTED`;
- escolhas facilitadoras tiverem consumidor imediato e prova de que a saída
  padrão permaneceu `Preserved`;
- `svg-morphology` permanecer `PARTIAL` se qualquer fronteira material seguir
  `Unknown`.
