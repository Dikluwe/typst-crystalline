# P1247 — fechar links internos e destinos SVG

**Estado:** FECHADO — MATERIALIZADO, SEGREGADO E CERTIFICADO  
**Predecessor:** P1246  
**Saída:** aresta, registro de destinos e API SVG classificados separadamente.

Auditar `render_link`, hoje com links internos em scope-out. Medir vanilla para
URL, posição/destino interno, escaping, conteúdo aninhado, transform e área
clicável. Verificar se entidades atuais carregam identidade suficiente.

Se o contrato atual bastar, atualizar L0 e implementar RED→GREEN no owner SVG.
Se faltar identidade pública, produzir diagnóstico e parar pelo ADR-0127. Não
fabricar IDs a partir de bytes, ordem incidental ou texto renderizado.

## Resultado saneado

`LinkTarget::Destination(Label)` já transporta distintamente a identidade da
aresta interna até L3. O `PagedDocument` também conserva página e posição dos
labels, mas `export_svg` recebe somente uma `Page`; portanto o exporter não
recebe o registro necessário para emitir o nó de destino e fechar `href → id`.

Separadamente, o construtor público `link()` continua restrito a URL e não
aceita label, location ou dicionário page/x/y. O universo de grafos SVG internos
completamente preservados permanece zero, mas por ausência de contexto/nó de
destino, não por perda da aresta.

Foram inventariadas oito fronteiras e cinco opções. O dono aprovou em
2026-08-28 a injeção de contexto imutável de destinos de página em L3, mantendo
política cross-page no caller de composição. Expandir o construtor público é
decisão ADR-0127 separada e não foi autorizado.

O invariant compartilhado foi nucleado em
`00_nucleo/prompts/_nuclei/export/svg-destination-context.toml` e pinado pelos
owners `infra/export/svg` e `infra/pipeline`. Os L0 foram atualizados antes de
código. Nenhum código, oracle ou mutante foi materializado: a implementação
aguarda contrato/oráculos/ataques independentes e preseal segregado válido.

## Fechamento do passo

P1247 fica fechado como saneamento arquitetural, não como alegação de paridade
implementada. O passo cumpriu o seu objetivo ao:

1. medir separadamente a aresta interna, o registro de destinos e a API SVG;
2. rejeitar `href` pendente, identidade sintetizada e duplicação por novo
   `FrameItem`;
3. obter a decisão do dono para contexto imutável page-local;
4. atualizar primeiro os owners L0 de SVG e pipeline;
5. extrair o invariant compartilhado para Núcleo Tekt com pins válidos;
6. reproduzir o runner byte a byte e passar V15, V26 e `git diff --check`.

### Materialização segregada (2026-08-28)

O débito produtivo foi materializado sob pré-selo Tekt revalidado:

- `SvgDestinationContext` imutável e page-local, com identidades atribuídas
  por label ordenada, independentes de posição e ordem incidental;
- wrappers antigos preservados por contexto vazio e variantes explícitas com
  contexto, com e sem fontes;
- nós de destino únicos e arestas internas fechadas na mesma página;
- destino ausente/cross-page mantém filhos e hit area, emite `Unknown` sem
  fragmento pendente;
- pipeline filtra a primeira página e exige posição homóloga antes de injetar
  o destino;
- área clicável explícita preserva `pos/size` do `FrameItem::Link`.

Verificação final independente: P1247 4/4; SVG 26/26; workspace build PASS;
V15/V26 zero; A01–A17 rejeitados, score `1.0`; `git diff --check` PASS.
Os warnings V5 remanescentes em `visualize.rs` e `tiling.rs` são externos ao
owner P1247.

Certificado:
`00_nucleo/diagnosticos/p1247-final-certificate.tsv`.

### Escopo que permanece fora

Cross-page/bundle e expansão pública de `link()` continuam fora desse escopo e
exigem decisões próprias. Até a materialização futura, links internos SVG
cross-page permanecem `Unknown`; P1247 declara preservado somente o grafo
same-page coberto pelo contexto certificado.

**Decisão de sequência:** o saneamento P1247 não bloqueia a auditoria do P1248.
