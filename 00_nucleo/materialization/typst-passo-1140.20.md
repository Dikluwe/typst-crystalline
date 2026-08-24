# Passo 1140.20 — Nuclear as propriedades de página restantes

**Estado:** executado  
**Data:** 2026-08-24  
**Continua:** P1140.19  
**Prepara:** P1140.20.1–P1140.20.4 e P1140.21  
**Tipo:** auditoria, medição e divisão vinculante; sem código de produto  
**Relatório previsto:** `00_nucleo/diagnosticos/typst-p1140.20-propriedades-page.md`

## 1. Objetivo

Medir e nuclear as 15 propriedades de `page` que continuam ausentes ou
parciais depois de P1140.19, produzindo subconjuntos implementáveis sem aceitar
argumentos ignorados e sem concentrar geometria, composição visual,
introspecção e exportação num único passo.

P1140.20 não expõe `page(...)` e não acrescenta campos a entidades. Ele fecha
a decisão de divisão e escreve os passos executáveis seguintes. A
materialização pertence a P1140.20.1–P1140.20.3; o binding público continua em
P1140.21.

## 2. Proveniência inicial

- HEAD: `45b547073d7686cdd5d3e3030c82de3e22ec395f`;
- working tree não commitada;
- hora: `2026-08-24T14:04:01-03:00`;
- `git diff HEAD --stat`:
  `90 files changed, 1027 insertions(+), 511 deletions(-)`;
- vanilla: upstream/main ratificado `a51e02804`;
- fonte principal:
  `lab/typst-original/crates/typst-library/src/layout/page.rs:54-501`;
- diagnóstico de classificação:
  `00_nucleo/diagnosticos/typst-p1140.18-page-global.md`;
- fronteira lexical pronta:
  `00_nucleo/diagnosticos/typst-p1140.19-page-run.md`.

Estado estrutural cristalino medido:

- `PageConfig` possui apenas `width`, `height`, `margin`, `numbering` e
  `columns` (`entities/layout_types.rs:679-686`);
- `Content::SetPage` transporta o mesmo subconjunto
  (`entities/content.rs:516-532`);
- `PageRunElem` já isola esse subconjunto e body;
- `width`, `height` e `columns` são A/completos para a fronteira atual;
- `body` deixou de ser pré-requisito estrutural depois de P1140.19;
- restam `margin` e `numbering` parciais mais 13 propriedades ausentes.

## 3. Regra contra defaults empíricos

Os defaults impressos pelo inventário (`595.28pt`, `841.89pt`, `30%`, etc.)
são observações da build ratificada, não literais automaticamente autorizados.

Para cada default, o passo deve localizar sua definição normativa na fonte
vanilla, tabela de papel, tipo de domínio ou fórmula. Um valor medido pode ser
oracle de teste, mas só entra no código quando derivado de:

- constante semântica nomeada do domínio;
- tabela normativa com unidade e proveniência;
- fórmula estrutural documentada;
- valor fornecido pelo utilizador.

Não inserir números “calibrados” para reproduzir uma amostra.

## 4. Auditoria L0 obrigatória

Ler integralmente os L0s realmente afetados antes de classificar dependências:

1. `00_nucleo/prompts/entities/elements/page_run.md`;
2. `00_nucleo/prompts/entities/content.md`;
3. `00_nucleo/prompts/entities/layout_types.md`;
4. `00_nucleo/prompts/compiler/eval.md`;
5. `00_nucleo/prompts/compiler/layout.md`;
6. `00_nucleo/prompts/compiler/stdlib/layout.md`;
7. prompts de `Page`, `PageStore`, introspecção e export tocados pela medição;
8. ADR-0107, ADR-0108, ADR-0109, ADR-0126 e ADR-0127.

Confirmar hashes vigentes. P1140.20 só atualiza L0 se descobrir contradição ou
se a própria divisão precisar ser uma decisão perene; não editar por
proximidade nominal.

## 5. Matriz de medição por propriedade

Para cada propriedade abaixo, medir no vanilla ratificado:

1. tipos aceitos e rejeitados;
2. omissão, `auto` e `none`, quando aplicáveis;
3. efeito em `#set page(...)` e em `page(...)[body]`;
4. herança e restauração depois de um page-run;
5. interação com direção LTR/RTL;
6. efeito em página vazia e multipágina;
7. morfologia/introspecção pública disponível;
8. consumer final: layout, query, PDF, SVG/PNG ou acessibilidade;
9. diagnóstico de argumento inválido;
10. fonte `file:line`, inferências e o teste que as refutaria.

Propriedades:

| Propriedade | Estado P1140.18 | Pergunta central |
|---|:---:|---|
| `paper` | C | tabela normativa e precedência com width/height |
| `flipped` | C | troca física antes/depois de overrides explícitos |
| `margin` | B | fold, `inside`/`outside`, auto por lado |
| `binding` | C | resolução lógica por direção/paridade de página |
| `bleed` | C | geometria física, TrimBox e resolução percentual |
| `fill` | C | default por target e fundo da página inteira |
| `background` | C | camada atrás do body e referencial incluindo bleed |
| `foreground` | C | camada acima do body e referencial incluindo bleed |
| `numbering` | B | pattern e função, número atual/total |
| `supplement` | C | page refs e `auto`/`none`/content |
| `number-align` | C | ligação com header/footer automático |
| `header` | C | repetição, contexto e acessibilidade |
| `header-ascent` | C | length relativo à margem superior |
| `footer` | C | repetição, contexto e precedência da numeração |
| `footer-descent` | C | length relativo à margem inferior |

## 6. Sondas mínimas

### 6.1 Geometria lógica

- todos os nomes de papel aceitos e um inválido;
- paper posicional e `paper:` nomeado;
- paper com apenas width, apenas height e ambos;
- `flipped: true` com e sem dimensões explícitas;
- `binding: auto/left/right` sob LTR e RTL;
- margens `inside/outside` em páginas ímpares e pares;
- conflitos `left/right` versus `inside/outside`;
- fold de dois `#set page(margin: ...)` sucessivos;
- restauração desses valores após page-run.

### 6.2 Canvas e camadas

- bleed uniforme e por lado;
- MediaBox, TrimBox e BleedBox no PDF;
- bleed zero versus omitido;
- fill `auto`, `none` e paint explícito por target;
- background/foreground com conteúdo distinguível e ordem de desenho;
- percentuais no background relativamente à página com bleed;
- visibilidade em plain text/tagging/acessibilidade;
- isolamento e restauração em page-runs.

### 6.3 Running matter e referências

- numbering string de um e dois contadores;
- numbering função com aridade observada;
- `supplement: auto/none/content` em referência de página;
- componentes válidos/inválidos de `number-align`;
- header/footer `auto`, `none`, content e context;
- precedência de header/footer explícito sobre numbering;
- ascent/descent em valor absoluto e relativo;
- documento multipágina e page-run multipágina;
- restauração depois do run;
- morfologia acessível e conteúdo lido por tecnologia assistiva.

Não comparar bytes PDF; caixas e ordem de desenho são observáveis mecânicos
somente onde constituem o próprio contrato de exportação.

## 7. Divisão candidata a validar

### P1140.20.1 — geometria lógica

Escopo candidato:

- `paper`;
- `flipped`;
- `binding`;
- completar `margin`, incluindo `inside`/`outside` e fold.

Razão: os quatro valores determinam dimensões físicas e resolução dos lados
antes do layout do body. Não dependem de conteúdo de header/footer nem de
camadas de pintura.

### P1140.20.2 — canvas físico e camadas

Escopo candidato:

- `bleed`;
- `fill`;
- `background`;
- `foreground`.

Razão: exigem snapshot de página e consumers de render/export, incluindo
TrimBox e ordem de desenho. Devem compartilhar um modelo explícito de canvas,
sem assar frames no eval.

### P1140.20.3 — running matter, numeração e referências

Escopo candidato:

- completar `numbering`;
- `supplement`;
- `number-align`;
- `header` e `header-ascent`;
- `footer` e `footer-descent`.

Razão: formam um único sistema de repetição por página, contador atual/total,
precedência e referência. Separar header/footer de number-align sem contrato de
incompletude aceitaria propriedades sem consumer.

## 8. Critério para alterar a divisão

A divisão acima é hipótese, não decisão antecipada. A medição pode:

- mover `bleed` para P1140.20.1 se sua entidade for pré-requisito inevitável da
  resolução geométrica de background;
- separar numbering funcional de pattern se o callback exigir uma fase de
  fixpoint ainda ausente;
- dividir P1140.20.3 em passos menores se cada subconjunto for rejeitado
  explicitamente até seu consumer existir;
- criar pré-requisito de export antes de transportar uma propriedade.

Qualquer divisão final deve manter o L0 inicial explicitamente incompleto e
nomear o passo que completa cada subconjunto, conforme a regra do repositório.

## 9. Atomização a auditar

Para cada subconjunto, identificar donos antes de propor arquivos:

- tipos de domínio em unidade própria dentro de `entities`, sem transformar
  `layout_types.rs` em depósito automático;
- payload de `PageRunElem` e `SetPage` somente com tipos fechados;
- parsing/validação na unidade stdlib/eval dona;
- aplicação de configuração em `compiler/layout` por free functions forma B;
- composição de header/footer/background/foreground em arquivos da feature;
- caixas PDF e pintura em L3, sem importar L3 em L1;
- wiring somente para composição, sem lógica.

Não usar `PropMap`, strings para dispatch, `dyn`, vtable ou import reverso.
Antes de criar cada arquivo, medir se o módulo atual já é dono coerente e se
precisa ser atomizado.

## 10. Gate e fluxo dos subpassos

P1140.20 é diagnóstico e não precisa de gate de código. Cada subpasso que
adicionar campos públicos, alterar defaults ou tocar fase de pipeline deve:

1. medir seu subconjunto;
2. atualizar L0;
3. registrar hashes;
4. parar no gate ADR-0127;
5. aguardar confirmação explícita;
6. escrever RED;
7. implementar e validar;
8. registrar lacunas restantes sem aceitar argumentos silenciosamente.

Não usar uma única confirmação para autorizar automaticamente os três
subpassos; seus contratos e blast radius são diferentes.

## 11. Entregáveis

1. diagnóstico `typst-p1140.20-propriedades-page.md`;
2. matriz completa com tipo/default/efeito/consumer/fonte/classificação;
3. mapa de dependências entre entidade, eval, layout, introspecção e export;
4. decisão final de divisão;
5. passos materializados P1140.20.1–P1140.20.3, ou numeração revista se a
   medição exigir mais unidades;
6. registro das contradições L0 encontradas;
7. fila explícita até P1140.21.

## 12. Aceitação

P1140.20 fecha quando:

1. as 15 propriedades foram medidas, não apenas listadas;
2. nenhum default empírico foi promovido a regra sem fonte normativa;
3. cada propriedade possui owner e consumer final identificados;
4. dependências e fases de pipeline estão explícitas;
5. a divisão final foi decidida depois da medição;
6. cada constructor intermediário declara incompletude e rejeições;
7. nenhum código de produto ou campo público foi escrito neste passo;
8. os passos executáveis seguintes foram escritos em `materialization/`;
9. o diagnóstico registra HEAD, hora e working tree de todos os números;
10. `crystalline-lint .` e `git diff --check` passam após eventuais correções
    documentais.

## 13. Fora de escopo

- implementar qualquer propriedade;
- expor `page` ou `std.page`;
- alterar `location.page()` ou `show page`;
- resolver tagging PDF completo;
- copiar estruturas internas do vanilla como requisito;
- usar valores medidos como constantes sem derivação normativa;
- fechar P1140.21 antes de todos os argumentos públicos terem comportamento ou
  rejeição explícita.

## 14. Resultado da execução

Executado em 2026-08-24. A medição confirmou P1140.20.1 e P1140.20.2, mas
refutou a concentração proposta em P1140.20.3: `supplement` termina em
introspecção/referências, enquanto numbering/header/footer terminam na
composição marginal de cada página. A divisão vinculante passou a quatro
unidades:

1. P1140.20.1 — papel, orientação, binding e margens lógicas;
2. P1140.20.2 — bleed, fill, background e foreground;
3. P1140.20.3 — numbering, number-align, header/footer e seus offsets;
4. P1140.20.4 — supplement, introspecção e referências de página.

O diagnóstico completo está em
`00_nucleo/diagnosticos/typst-p1140.20-propriedades-page.md`. Os quatro passos
executáveis foram escritos em `materialization/`. Nenhum L0 ou código de
produto foi alterado nesta execução.
