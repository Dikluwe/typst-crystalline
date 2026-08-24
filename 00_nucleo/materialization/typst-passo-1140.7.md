# Passo 1140.7 — Estabilizar consumidores de `FrameItem::Semantic`

**Data de escrita:** 2026-08-24  
**Estado:** executado  
**Origem:** continuação corretiva do P1140.5  
**Baseline:** working tree não commitado sobre `ffd527c85dd7d547413d33cbc2d27a80e32a3f8c`

## 1. Problema medido

P1140.5 introduziu o envelope fechado:

```text
FrameItem::Semantic {
  kind: Formula,
  placement: Inline | Block,
  alt: Option<EcoString>,
  items: Vec<FrameItem>,
}
```

A compilação do workspace passou e os quatro testes próprios P1140.5 passaram.
Entretanto, `cargo test -p typst-core --lib`, executado em working tree não
commitado sobre o hash acima, produziu:

- 5106 testes aprovados;
- 38 falhas;
- 2 falhas P862 preexistentes;
- 36 falhas novas concentradas em testes de layout matemático.

As mensagens das falhas mostram a classe dominante: os testes procuram
`Text`, `Glyph`, `Shape`, `Group` ou coordenadas apenas no primeiro nível de
`Page.items`. O exemplo P994 imprime os mesmos filhos visuais dentro de
`Semantic.items`. Isso é evidência de incompatibilidade do consumidor de
inspeção, mas ainda **não prova** que todas as 36 falhas sejam somente testes.
Cada grupo deve ser reexecutado após o percurso recursivo; qualquer falha que
restar é tratada como possível regressão de produção.

## 2. Objetivo

Fazer todos os consumidores internos relevantes atravessarem
`FrameItem::Semantic` sem remover, achatar ou duplicar o envelope, restaurando
o baseline da suíte L1 para somente as duas falhas P862 conhecidas.

O passo também deve provar que:

- geometria visual antes/depois do envelope é invariável;
- fixups de página automática, centragem e numbering alcançam os filhos;
- shaping e export não desenham a fórmula duas vezes;
- testes deixam de depender da falsa topologia `Page.items = folhas visuais`;
- `alt` e placement continuam disponíveis depois de todas as passagens.

## 3. Auditoria L0 antes de código

Ler e confirmar os hashes vigentes de:

- `00_nucleo/prompts/entities/layout_types.md`;
- `00_nucleo/prompts/compiler/layout/equation.md`;
- `00_nucleo/prompts/compiler/layout.md`;
- `00_nucleo/prompts/infra/shaper.md`;
- `00_nucleo/prompts/infra/pipeline.md`;
- `00_nucleo/prompts/infra/export/render.md`;
- `00_nucleo/prompts/infra/export/svg.md`;
- `00_nucleo/prompts/infra/export/stream.md`.

Decisão esperada: esta estabilização não muda contrato público nem
comportamento por defeito. Correções de percurso interno seguem fluxo contínuo
do ADR-0127. Se for necessário alterar campos de `FrameItem::Semantic`, parar
no gate porque isso muda contrato público L1→L3.

## 4. Estratégia de correção

### 4.1 Uma travessia, duas intenções

Não criar um vetor achatado de produção nem retirar o envelope. Distinguir:

1. **Travessia estrutural:** visita `Semantic`, `Group` e `Link`, preservando
   entrada/saída de cada contentor. Usar `FrameVisitor`/`walk_frame_items` onde
   a intenção for leitura genérica.
2. **Travessia geométrica:** aplica deslocamento, bbox ou fixup aos filhos com
   a regra de coordenadas própria de cada contentor. Não assumir que Group e
   Semantic têm o mesmo espaço local.

`Semantic.items` mantém coordenadas dos filhos no espaço do pai. Logo,
deslocamentos X/Y e medições devem recursar diretamente sem somar uma origem
inventada. `Group.items` continua sujeito à matriz/origem local já existente.

### 4.2 Consumidores de produção

Auditar exaustivamente todos os matches de `FrameItem` em L1 e L3 com `rg`.
Para cada braço `Semantic`, classificar e testar:

- leitura textual/fontes/gradientes: recursão transparente;
- shaping: shape dos filhos, preservando um único envelope;
- deslocamentos/fixups: mutação recursiva das coordenadas dos filhos;
- bbox/métricas: união dos filhos, não largura/posição zero;
- raster/SVG/PDF: desenhar somente os filhos uma vez;
- tagging: continua fora de escopo, sem BDC/EMC, MCID ou StructureTree.

Não aceitar wildcard para silenciar exaustividade. Não duplicar filhos como
irmãos do envelope para satisfazer testes antigos.

### 4.3 Infraestrutura de testes

Criar, dentro do módulo de testes de layout, helpers pequenos e explícitos:

- percurso recursivo read-only de folhas visuais;
- procura recursiva por texto/glifo/shape;
- coleta recursiva de coordenadas e estilos;
- procura estrutural do próprio envelope semântico.

Migrar as asserções falhadas para esses helpers. Reutilizar helpers recursivos
já existentes no ficheiro em vez de criar variantes locais redundantes.
Preservar testes que verificam deliberadamente a topologia de `Group` ou
`Semantic`; esses não devem usar uma vista achatada.

## 5. Grupos RED a corrigir

Executar primeiro os grupos individualmente, mantendo o output RED como prova:

1. `p813_equacao_bloco`, `tests_inline_baseline`, `tests_limits`;
2. `p896`, `p944`, `p945_tests`, `p967_equacao_inline`;
3. `p987_tests`, `p994_tests`, `p997_tests`;
4. `eval_sqrt_layout_tem_overline` e `tests_align`;
5. `layout_equation_bloco_com_width_auto_nao_produz_infinito`.

Depois da migração de inspeção, qualquer teste ainda RED deve ser diagnosticado
antes de editar expectativas. Não atualizar número/coordenada esperada apenas
para obter GREEN; comparar as folhas visuais e a geometria com a execução sem
envelope ou com o baseline anterior reproduzível.

## 6. Testes novos obrigatórios

Adicionar regressões próprias que cubram:

- walker visita folhas sob `Semantic → Group → Link` e sob
  `Group → Semantic → Link`;
- `Page::plain_text` atravessa Semantic sem usar `alt` como texto visual;
- shift X/Y move todas as folhas uma vez;
- bbox de Semantic equivale à união de seus filhos;
- shaping troca `Text` por `TextShaped` dentro do envelope e preserva
  `kind`, placement e `alt`, incluindo string vazia;
- raster/SVG/PDF processam filhos uma vez e não materializam `alt` visualmente;
- fórmula inline e block preservam baseline, centragem e numbering;
- ausência de `alt`, `none`, `""` e Unicode chegam intactos à fronteira L3.

## 7. Sequência de execução

1. Registrar hora, HEAD e `git diff HEAD --stat` antes do primeiro RED.
2. Auditar L0 e ressellar apenas se houver correção documental necessária.
3. Reproduzir e registrar os cinco grupos RED da seção 5.
4. Auditar matches de produção e corrigir percursos incompletos.
5. Criar/consolidar helpers recursivos de teste.
6. Migrar as 36 asserções sem alterar expectativas geométricas.
7. Executar os grupos até GREEN.
8. Executar a suíte L1 completa e confirmar apenas as duas P862 conhecidas.
9. Executar a suíte L3 completa e separar eventual falha ambiental conhecida.
10. Executar todas as travas finais e atualizar o diagnóstico P1140.5.

## 8. Travas finais

```text
cargo fmt --check
cargo test -p typst-core --lib
cargo test -p typst-infra --lib
cargo build --workspace
crystalline-lint .
git diff --check
```

Critério numérico L1: resultado esperado = somente as duas falhas P862 já
registradas. Uma terceira falha mantém este passo aberto.

## 9. Critérios de aceitação

- [ ] Todos os matches de produção de `FrameItem` tratam Semantic conforme a intenção.
- [ ] Nenhum filho visual é duplicado ou retirado do envelope.
- [ ] As 36 falhas novas ficam GREEN sem relaxar valores esperados.
- [ ] Somente as duas falhas P862 permanecem na suíte L1.
- [ ] Suíte L3 não apresenta regressão de Semantic.
- [ ] `alt` permanece metadata, nunca texto visual ou fallback inventado.
- [ ] PDF continua sem tagging parcial; P1140.6 permanece dono desse eixo.
- [ ] `cargo build --workspace` passa.
- [ ] `crystalline-lint .` termina com zero violations.
- [ ] Proveniência de todas as contagens e medições fica registrada.
- [ ] Diagnóstico P1140.5 e este passo são atualizados com o resultado final.

## 10. Fora de escopo

- implementar PDF/UA, MCID, ParentTree ou StructTreeRoot;
- remover `FrameItem::Semantic` para recuperar compatibilidade mecânica;
- duplicar folhas no primeiro nível de `Page.items`;
- corrigir P862;
- mudar algoritmos matemáticos, métricas ou coordenadas sem uma regressão real
  demonstrada depois da travessia recursiva;
- criar uma abstração genérica de árvore que apague as diferenças geométricas
  entre Semantic, Group e Link.

## 11. Resultado esperado

P1140.5 volta a cumprir as travas do repositório: o envelope Formula permanece
estruturalmente observável para acessibilidade futura, todos os caminhos
visuais continuam equivalentes e a suíte deixa de confundir folhas visuais com
filhos imediatos da página. Se alguma das 36 falhas sobreviver à correção dos
walkers, o passo a isola como regressão real e a corrige antes do fechamento.

## 12. Resultado da execução

Execução concluída em `2026-08-24T10:16:37-03:00`, sobre working tree não
commitado baseado em `ffd527c85dd7d547413d33cbc2d27a80e32a3f8c`. O estado
exato continua representado por `git diff HEAD --stat`; no instante da medição,
o diff continha 76 ficheiros, 2463 inserções e 199 remoções, incluindo o trabalho
acumulado de P1140.4/P1140.5 e esta estabilização.

Resultados:

- `cargo test -p typst-core`: 5142 aprovados, somente as 2 falhas P862
  preexistentes;
- as 36 falhas L1 novas foram eliminadas sem alterar expectativas geométricas;
- `cargo test -p typst-infra --lib`: 824 aprovados, somente 1 falha ambiental
  preexistente em `custom_ca_autoriza_cadeia_local_mas_nao_hostname_incorreto`
  (`Operation not permitted` ao abrir o recurso local do teste);
- a primeira passagem L3 também encontrou 22 inspeções rasas e uma regressão
  real: `stack` não atravessava `Semantic` ao medir baseline/ascent. Depois da
  correção, a altura P1132b voltou de 130.450113pt para o valor esperado
  125.236pt;
- `cargo build --workspace` passou;
- `crystalline-lint .` terminou sem violations (apenas warnings/info já
  reportados pelo projeto);
- `cargo fmt --all` e `git diff --check` passaram.

O envelope continua preservado; nenhum filho foi duplicado ou promovido para
`Page.items`. P1140.6 permanece o dono exclusivo de tagging PDF.
