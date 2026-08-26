# P1213 — descoberta assimétrica das lacunas

**Resultado:** `MISSING-FIRST BACKLOG FROZEN — CRYSTALLINE EXTRAS RECORDED SEPARATELY`  
**Data:** 2026-08-26  
**Regime:** protocolo completo em sessão única, **sem atestação de isolamento**.  
**Vanilla ratificado:** `a51e02804`.  
**Cristalino:** `dc47c9c32b8b6769a58622c98b885cb094337508` + working tree
documental P1212/P1213.  
**Lente:** `98d8f9e7a3882beb3fd0a294a95df9332058f76c` + working tree P0079;
binário SHA-256
`9b49489c4afbbe38199cf8025a386eac235a3435bb9657e781f3e97e0435fc18`.

## 1. Conclusão executiva

O P1213 congelou dois livros sem saldo líquido. O Livro A contém 22
responsabilidades vanilla examinadas: 5 `COVERED`, 6 `PARTIAL`, 5 `MISSING`,
2 `PRODUCT-GAP`, 2 `DIAGNOSTIC-GAP`, 1 `FEATURE-GATED` e 1 `HARNESS-GAP`.
O Livro B registra 10 extensões públicas cristalinas. Nenhuma extensão reduziu
a severidade ou a prioridade das lacunas.

A principal retificação foi HTML: com `--features html` nos dois lados, o
documento focal produziu exatamente o mesmo SHA-256. O `global-html` de P1210
não é ausência global; é capacidade feature-gated, e o caso da matriz contém
assimetria no harness. Isso não fecha todo HTML: CSS, MathML, posições,
constructors e morfologia ampla continuam fora do fragmento sondado.

As ausências pequenas mais claras são os métodos públicos de `bytes`:
`at`, `len` e `slice` existem no vanilla e o cristalino responde que o tipo não
possui esses métodos. O caso é o primeiro candidato de implementação porque o
tipo e constructor já existem, o owner é localizado e o RED é reproduzível.

## 2. Proveniência e estabilidade

Congelamento inicial em `2026-08-26T14:26:13-03:00`:

- mapa inicial SHA-256
  `8b45aaf1e6b84c706ffb02d4399c0aa546ffb030a029995b2bceff9cc0822877`;
- JSON sem mapa SHA-256
  `0881042f0e98038a3d19b946302291a2a35712cd0d5fab4ffa8c3ed6bda7688c`;
- JSON com mapa P1212 SHA-256
  `2000580683d0a7a050f24a167e590c630d1ae001bc216aca993e5e63154f91a9`.

Os dois JSON repetiram byte a byte os hashes de P1212. Após seis novas
correspondências:

- mapa SHA-256
  `a54023614fbb96869cec1ccf74c8bc29fdab76afb0ca1c2b68cc7ac5274f4f65`;
- JSON final SHA-256
  `e8237c13cde3f22567788d95750aec754e31942403a5de5652742922c657dd0d`.

A lente final manteve 24 módulos pareados, 18 arestas comuns e 1.906 itens K4
pareados. As ambiguidades caíram de 147 para 142 e itens cristalinos sem par de
3.709 para 3.708. Os 11.266 itens vanilla sem par não são 11.266 features:
helpers, tipos e responsabilidades agrupadas continuam no conjunto bruto.

Falhas de extração permaneceram: duas no vanilla e uma no cristalino. Nada
nessas crates foi classificado automaticamente como ausência.

## 3. O que falta primeiro

A fila nominal possui onze clusters:

1. métodos `bytes.at/len/slice`;
2. mensagens `bad_in` e comparação de comprimentos;
3. morfologia SVG;
4. configuração de smartquote (`alternative` e `quotes`);
5. linha com `start` não-zero;
6. `color.mix` com três ou mais cores;
7. attachment PDF;
8. serialização YAML no `eval` da CLI;
9. origem de transform, condicionada a sonda geométrica;
10. diferença raster de 11 pixels com delta máximo 1;
11. identidade de família reportada pelo PDF, condicionada a adjudicação do
    observável.

Os itens 7 e 8 carregam `ADR0127-GATE`: ampliam superfície pública/produto. Os
demais podem ser fluxo contínuo somente quando o L0 vigente for atualizado
primeiro e o passo de implementação preservar RED→GREEN.

## 4. Frentes conhecidas

### HTML

O runner vigente passou `--features html` apenas ao vanilla e obteve erro no
cristalino. A repetição simétrica gerou HTML byte-idêntico:

```text
c8049e71950f48ba1b3cbcdd0dd6ca75a6357a762dd6a3056670f75a73b35f92
```

Veredito focal: `FEATURE-GATED/COVERED-FOCAL`; veredito do runner atual:
`HARNESS-GAP`. Não há alegação de paridade HTML geral.

### SVG, PNG e PDF

- SVG: ambos exportam, mas a árvore semântica focal diverge (`PARTIAL`);
- PNG: dimensões iguais; 11 de 2.005.644 pixels divergem, delta máximo 1
  (`PARTIAL`, sem tolerância inventada);
- PDF: páginas, caixa e texto iguais; `pdffonts` reporta
  `LibertinusSerif-Regular-Identity-H` versus `LibertinusSerif-Regular`.
  Continua `PARTIAL` até decidir se essa identidade é observável de paridade.

### Identidades antes ambíguas

- `BinOp` e `UnOp`: responsabilidade sintática dividida entre tipo do parser e
  accessor da AST; fragmento sintático congelado `COVERED`;
- `Selector`: responsabilidade vanilla dividida entre query e show;
  `PARTIAL`, pois variantes/stubs ainda precisam de corpus;
- `Bytes`: o tipo público corresponde a `entities::bytes::Bytes`;
  `world_types::Bytes` é transporte do World, não segundo candidato público;
- `eval`: entrypoint do motor e função pública `eval(source)` são duas
  responsabilidades diferentes, ambas explicitadas no mapa.

## 5. Ausências adicionais demonstradas

Sondas diferenciais produziram RED para:

- `bytes.at`, `bytes.len`, `bytes.slice`;
- `eval --format yaml`;
- attachment PDF válido com dados em memória;
- `smartquote(alternative:)` e `smartquote(quotes:)`;
- `line(start: não-zero, end:)`;
- `color.mix(red, green, blue)`.

`rotate(origin:)` preserva o campo no repr vanilla e aparece como transform
genérico no cristalino. Sem sonda geométrica com corpo assimétrico, permanece
`PARTIAL`, não `MISSING`.

Quatro scope-outs antigos de color foram refutados como estado atual:
`components`, `space`, `rotate` e `saturate` produziram os mesmos valores
focais. Foram classificados `COVERED`; a documentação histórica deve ser
limpa depois, sem reabrir implementação.

## 6. Excedentes cristalinos

Dez bindings foram revalidados como funções cristalinas e como nomes ausentes
no vanilla ratificado: `accent`, `bb`, `bold`, `cal`, `calc.deg`,
`calc.log10`, `calc.rad`, `counter_at`, `linear_rgb` e `lof`.

Eles estão registrados como `CRYSTALLINE-EXTENSION`. O P1213 não decide
mantê-los nem removê-los. Riscos possíveis — principalmente colisão futura de
namespace ou exposição de vocabulário interno — ficam preservados no Livro B.
Não existe crédito positivo de paridade por esses dez itens.

## 7. Ataques

| Ataque | Resultado |
|---|---|
| falso ausente por feature | refutou `global-html`; feature simétrica fecha o caso focal |
| falso ausente por renome/divisão | retirou as ambiguidades de operadores, selector, bytes e eval |
| falso coberto por nome | `Selector` permaneceu `PARTIAL`; nome não fechou variantes |
| falso extra | `world_types::Bytes` foi classificado como transporte, não extensão da linguagem |
| extensão incompatível | dez bindings extras preservam risco nominal; nenhum foi declarado seguro universalmente |
| compensação indevida | Livro B não aparece na fila nem altera contagens do Livro A |
| contagem enganosa | itens DSM não foram convertidos em percentual nem em número de features |
| scope-out desatualizado | métodos color foram re-medidos e marcados `COVERED` |

Não houve campanha externa de mutation testing. Os ataques são refutações
dirigidas; não há mutation score atestado.

## 8. Separação e limites

As fases foram mantidas em ordem causal — censo, L0/oráculos, execução,
ataques, adjudicação — mas uma única sessão teve acesso a todas. Classificação:
`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`.

Nenhum `.rs` ou Prompt L0 foi alterado. Os REDs são saídas de descoberta e não
foram corrigidos. O mapa foi alterado somente para relações adjudicadas.

Gates finais:

- repetição da lente final: byte-idêntica, exit 0;
- `cargo build --workspace --quiet`: exit 0, com avisos preexistentes;
- `crystalline-lint .`: exit 0, com avisos informativos/preexistentes;
- `git diff --check`: exit 0;
- cardinalidade das cinco tabelas TSV: exit 0;
- quatro casos P4 do harness e sondas focais: executados; os REDs nominais
  permanecem nos livros, como exige este passo de descoberta.

## 9. Estado terminal

```text
MISSING-FIRST BACKLOG FROZEN — CRYSTALLINE EXTRAS RECORDED SEPARATELY
```

Próximo cluster recomendado: `bytes.at/len/slice`, com atualização prévia do
L0 proprietário, teste RED focal e implementação restrita ao dispatch de
métodos do valor público.
