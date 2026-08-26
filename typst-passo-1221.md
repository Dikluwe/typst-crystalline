# P1221 — ampliar `svg-morphology` para shapes e localizar o primeiro gap real

**Estado:** EXECUTADO — gap focal RED→GREEN; cluster repartido e permanece PARTIAL  
**Predecessor causal:** P1220  
**Cluster:** `svg-morphology`, atualmente `PARTIAL`  
**Controle fechado:** `P1138-X-001`/`plain.typ` — MATCH focal  
**Fronteira atual:** shapes sólidos retornam `Unknown`  
**Owner produtivo provável:** `03_infra/src/export/svg.rs::render_shape`

## 1. Objetivo

Estender o modelo morfológico SVG de P1220 para comparar shapes sólidos por
operações geométricas e de pintura, independentemente de o vanilla emitir
sempre `<path>` e o cristalino escolher `<rect>`, `<ellipse>`, `<line>` ou
`<path>`.

Depois da extensão, executar um corpus de formas e identificar a primeira
divergência **real** em geometria, fill, stroke, transform ou ordem de pintura.
Corrigir somente esse primeiro gap produtivo se ele existir e estiver
legitimado pelo L0. Se todo o corpus focal for equivalente, fechar apenas o
subfragmento `svg-solid-shapes` e manter o cluster geral `PARTIAL` para
gradientes, tilings, clips, imagens e links.

Resultado preferido:

```text
SVG SOLID SHAPES MEASURED — FIRST REAL GAP GREEN OR FOCAL MATCH
```

## 2. Baseline e entradas protegidas

Congelar antes de alterar comparador, L0 ou exportador:

- vanilla ratificado `a51e02804` e SHA-256 do binário;
- HEAD, horário, `git status --short` e `git diff HEAD --stat` completos;
- binário cristalino, lente e mapa DSM;
- `lab/parity/matrix/svg_morphology.py`, testes, runner e manifesto;
- artefatos e certificado P1220;
- L0 `00_nucleo/prompts/infra/export/svg.md` e consumer
  `03_infra/src/export/svg.rs`;
- vanilla `typst-svg/src/shape.rs`, `path.rs`, `paint.rs` e owners de stroke;
- tipos cristalinos `ShapeKind`, `Stroke`, `PathItem`, cores e transforms;
- versões das ferramentas de render usadas como prova auxiliar.

Toda métrica decisória deve registrar commit ou working tree não commitado,
lista exata de ficheiros alterados e horário quando necessário.

## 3. Escopo focal de shapes

Medir, no mínimo:

1. retângulo sólido;
2. retângulo sem fill, só stroke;
3. retângulo com fill e stroke;
4. retângulo arredondado com raios simétricos;
5. retângulo arredondado com quatro raios diferentes;
6. elipse e círculo;
7. linha horizontal, vertical e diagonal;
8. curva aberta com segmentos line/cubic;
9. curva fechada;
10. shape traduzido;
11. shape sob escala, rotação e reflexão;
12. duas formas sobrepostas em ambas as ordens;
13. dimensões negativas ou degeneradas quando publicamente construíveis;
14. cores RGB com alpha;
15. stroke com espessura zero, positiva e default;
16. line cap, join, miter e dash quando expostos pelo contrato público.

Gradientes, tilings, clips/masks, imagens e links são controles `Unknown`
neste passo, salvo se forem necessários para explicar o primeiro gap.

## 4. Modelo geométrico canônico

O comparador não pode igualar strings `d`. Deve converter as representações
SVG suportadas para uma sequência canônica de paths absolutos:

```text
Move(x,y)
Line(x,y)
Cubic(c1x,c1y,c2x,c2y,x,y)
Quadratic(cx,cy,x,y)
Arc(rx,ry,rotation,large_arc,sweep,x,y)
Close
```

Requisitos:

- suportar comandos absolutos e relativos `M/L/H/V/C/S/Q/T/A/Z`;
- aplicar repetição implícita de parâmetros conforme SVG;
- expandir `<rect>`, `<ellipse>`, `<circle>`, `<line>`, `<polyline>` e
  `<polygon>` para geometria comparável;
- aplicar transforms na ordem correta, incluindo grupos aninhados;
- preservar orientação, winding e subpaths;
- não converter curvas em polilinhas nem usar aproximação raster como modelo;
- não considerar dois paths equivalentes apenas porque seus bounding boxes
  coincidem;
- retornar `Unknown` para sintaxe ou paint não suportado, nunca descartar.

Para formas semanticamente equivalentes com parametrizações diferentes — por
exemplo rect path versus `<rect>` ou elipse Bézier versus `<ellipse>` — usar
uma regra explícita e testada. Não aproximar arcos/Béziers sem erro máximo
declarado e prova visual/geométrica.

## 5. Modelo canônico de pintura

Cada shape deve produzir uma operação ordenada:

```text
ShapePaint {
  geometry,
  transform,
  fill,
  fill_rule,
  stroke: {
    paint,
    width,
    cap,
    join,
    miter_limit,
    dash_array,
    dash_offset,
  }?,
  opacity,
}
```

Normalizar defaults SVG somente quando a especificação e um teste provarem
equivalência. Distinguir obrigatoriamente:

- fill ausente/default preto versus `fill="none"`;
- stroke ausente versus transparente ou largura zero;
- `nonzero` versus `evenodd`;
- butt/round/square;
- miter/round/bevel;
- dash order, phase e unidades;
- alpha no paint versus opacidade do elemento/grupo;
- ordem de pintura entre shapes e glyphs.

Paint server `url(#...)` deve preservar o grafo de referências e permanecer
`Unknown` enquanto o conteúdo do server não estiver modelado.

## 6. Contrato e artefatos antes da implementação

Produzir antes do candidato produtivo:

```text
00_nucleo/diagnosticos/p1221-svg-shape-contract.tsv
00_nucleo/diagnosticos/p1221-svg-shape-oraculos.tsv
00_nucleo/diagnosticos/p1221-svg-shape-diff.tsv
```

Contrato mínimo:

```text
Preserved: path-rect equivalente, defaults equivalentes, transforms compostos
Violated: coordenada, fill, stroke, winding, order ou transform diferente
Unknown: paint server, clip/mask ou comando não modelado
```

Cada oráculo deve registrar fixture, comando, hashes dos SVGs, operação
canônica vanilla/cristalina, render auxiliar, source `file:line`, inferência e
o que a refutaria.

## 7. Gate discriminatório do comparador

Antes de medir o exportador, adicionar testes que rejeitem, no mínimo:

- mudar uma coordenada preservando bbox;
- inverter winding sob fill `nonzero`;
- trocar `nonzero` por `evenodd`;
- trocar fill por stroke;
- omitir stroke width;
- trocar cap/join/miter/dash;
- inverter ordem de duas formas sobrepostas;
- trocar ordem de transforms não comutativos;
- aceitar path malformado;
- aceitar referência de paint quebrada;
- igualar círculo a elipse de raios diferentes;
- igualar rounded rect a rect sem raio;
- arredondar diferença acima da tolerância;
- converter construção opaca em Preserved.

Somente selar o contrato com `mutation_score = 1.0` para mutantes válidos.

## 8. Medição antes da decisão produtiva

Executar o corpus com o comparador selado e classificar, em ordem estrutural:

```text
rank | fixture | paint_index | vanilla | crystalline | category |
Preserved/Violated/Unknown | source_file_line | inference |
what_would_refute | next_action
```

Para o primeiro `Violated`, confirmar causalidade:

1. comparar `Page`/`FrameItem::Shape` entregue ao exportador;
2. verificar vanilla `convert_geometry_to_path`/`write_fill`/`write_stroke`;
3. verificar cristalino `render_shape`/`write_shape_attrs`;
4. renderizar ambos com o mesmo renderer em pelo menos duas escalas;
5. extrair bounding boxes e pixels, sem usar pixels como único oráculo.

Se a entrada `FrameItem` já divergir, o owner não é o SVG: abrir cluster para
layout/stdlib e não remendar a serialização.

## 9. Gate L0

Após localizar o primeiro gap, reler `infra/export/svg.md`:

- se o L0 já exige o comportamento, escrever RED e implementar;
- se for correção interna de paridade, atualizar o L0 primeiro e seguir fluxo
  contínuo ADR-0127;
- parar se exigir interface pública, default de produto, nova fase,
  compatibilidade ou nova política de acessibilidade.

Se a causa estiver fora do exportador, ler o L0 explícito do owner causal
antes de propor qualquer alteração.

## 10. Testes RED produtivos

Antes do patch L3/L1:

- teste unitário no owner exato;
- teste público `.typ → SVG` para o primeiro gap;
- A/B contra a operação canônica congelada;
- controle da forma imediatamente anterior já Preserved;
- controle `plain.typ` P1220;
- controle que permaneça `Unknown` por estar fora do escopo.

Confirmar e registrar RED. Falha do harness, fonte ausente ou renderer externo
não conta como RED produtivo.

## 11. Implementação permitida

Aplicar a menor correção legitimada:

- atributos de shape em `03_infra/src/export/svg.rs` se a entrada estiver
  correta;
- helper privado de path/stroke somente dentro do owner L3;
- owner de layout/entidade apenas se a medição provar causalidade e o respetivo
  L0 for atualizado primeiro.

Não:

- forçar tudo a `<path>` apenas para imitar o vanilla;
- perseguir bytes ou whitespace;
- trocar geometria analítica por raster;
- implementar gradientes/clips junto sem causalidade;
- ignorar cap/join/dash porque o caso simples não usa;
- corrigir múltiplos gaps independentes;
- declarar shapes completos a partir de rect;
- marcar `svg-morphology` RESOLVED com corpus focal.

## 12. Ataques obrigatórios

Produzir:

```text
00_nucleo/diagnosticos/p1221-svg-shape-ataques.tsv
```

Rejeitar, no mínimo, 20 mutantes:

1. path e rect sempre equivalentes;
2. bbox igual implica path igual;
3. ignorar winding;
4. ignorar fill-rule;
5. default fill sempre none;
6. stroke transparente igual a ausente;
7. ignorar stroke width;
8. ignorar cap;
9. ignorar join;
10. ignorar miter limit;
11. ordenar dash array;
12. zerar dash offset;
13. ignorar alpha;
14. ordenar paints;
15. comutar transforms;
16. arredondar todas as coordenadas;
17. aceitar path malformado;
18. aceitar paint reference quebrada;
19. tratar paint server como solid;
20. converter `Unknown` em MATCH.

## 13. A/B final

Produzir duas rodadas independentes:

```text
00_nucleo/diagnosticos/p1221-svg-shape-resultados.tsv
```

Executar:

- corpus integral da seção 3;
- `P1138-X-001` P1220;
- shape focal antes/depois;
- renders comuns em duas escalas;
- pelo menos um `Violated` sintético e um `Unknown` sintético;
- controle P1219 para ausência de regressão transversal;
- runner integral ou subconjunto relevante do manifesto.

Registrar hashes de SVG bruto, operações canônicas, PNGs e diff. As duas
rodadas devem produzir o mesmo veredito por caso; timestamps não entram na
comparação.

## 14. DSM e fila

Se o subfragmento fechar, adicionar/promover relação específica:

```text
svg-solid-shapes
```

com owners reais e alegação limitada às formas/paints medidos.

Para `svg-morphology`:

- manter `PARTIAL` enquanto gradientes, tilings, clips, imagens, links ou
  outra forma permanecerem `Unknown`/Violated;
- atualizar a fila com a primeira categoria ainda aberta;
- se surgir causa externa ao SVG, criar cluster nominal antes de continuar;
- extensões cristalinas adicionais devem ser documentadas, não apagadas.

## 15. Laudo

Produzir:

```text
00_nucleo/diagnosticos/typst-p1221-svg-solid-shapes.md
```

O laudo deve declarar separadamente:

- escopo Preserved;
- primeiro gap corrigido, se houve;
- `Unknown` restantes;
- diferenças mecânicas normalizadas;
- estado do cluster e próximo gap;
- alcance exato da alegação DSM.

## 16. Gates finais

```text
python3 -m unittest discover -s lab/parity/matrix -p 'test*.py'
python3 lab/parity/matrix/runner.py --case P1138-X-001
python3 lab/parity/matrix/svg_morphology.py <vanilla.svg> <crystalline.svg>
cargo test -p typst-infra p1221
cargo test -p typst-wiring p1221
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

Repetir corpus e lente duas vezes, com hashes e proveniência.

## 17. Separação de autoridades

Usar protocolo Tekt completo:

- A congela spec, baseline e corpus;
- B escreve parser/modelo canônico e mutantes;
- C verifica poder discriminatório e sela;
- D mede/classifica sem editar código produtivo;
- E atualiza L0 e implementa somente o primeiro gap;
- F executa A/B sem editar mapa;
- G readjudica DSM e fila.

Numa única sessão, declarar:

```text
EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO
```

## 18. Continuação

Se shapes sólidos fecharem, o próximo subcluster SVG deve ser escolhido pela
primeira fronteira `Unknown` observada, preferindo `stroke-complex` se cap,
join/dash ainda estiverem abertos; depois gradientes/tilings, clips/masks,
imagens e links. `smartquote-config` só volta ao topo quando
`svg-morphology` estiver realmente fechado ou formalmente repartido.
