# Passo 1140.6 — PDF tagueado para fórmulas e `math.equation.alt`

**Data de escrita:** 2026-08-24  
**Estado:** executado  
**Predecessor funcional:** P1140.5  
**Estabilização independente:** P1140.7  
**Baseline:** working tree não commitado sobre `ffd527c85dd7d547413d33cbc2d27a80e32a3f8c`

## 1. Problema medido

O diagnóstico vigente é
`00_nucleo/diagnosticos/typst-p1140.5-alt-e-tagging.md`.

Contra o vanilla ratificado `a51e02804`, a medição P1140.5 confirmou:

- PDF tagueado é o comportamento por defeito;
- uma equação é representada por `StructElem` com `/S /Formula`;
- `alt: "texto"` chega como `/Alt` no elemento Formula;
- `--no-pdf-tags` desativa a estrutura tagueada;
- ausência, `none` e string vazia não satisfazem PDF/UA-1 para descrição da
  fórmula;
- o cristalino atual produz `Tagged: no`, sem `StructTreeRoot`, ParentTree ou
  associação MCID→estrutura.

P1140.5 já transporta `FrameItem::Semantic { kind: Formula, placement, alt,
items }` até L3. Portanto, este passo não precisa inferir semântica a partir de
glifos nem reler Content; deve consumir o envelope explícito.

## 2. Objetivo

Implementar a primeira fatia completa e verificável de PDF tagueado do
cristalino, fazendo cada `FrameItem::Semantic(Formula)` produzir:

- conteúdo marcado no stream com MCID;
- um `StructElem` `/Formula`;
- `/Alt` quando houver string, preservando Unicode e string vazia;
- ParentTree que associe página/MCID ao `StructElem`;
- `StructTreeRoot` alcançável pelo catálogo;
- MarkInfo coerente com o estado tagueado;
- opção pública para desativar tags, equivalente ao eixo
  `--no-pdf-tags` do vanilla.

O resultado deve ser estruturalmente válido antes de qualquer declaração de
PDF/UA. “Tem `/Alt`” e “é PDF/UA-1 conforme” são gates distintos.

## 3. Gate ADR-0127 obrigatório

Este passo altera comportamento por defeito do produto e contrato público de
exportação/CLI. A execução deve:

1. auditar os L0 donos;
2. escrever ou atualizar os L0 antes de código;
3. ressellar hashes;
4. **parar para confirmação humana**;
5. só depois escrever testes RED e implementação.

O gate deve apresentar explicitamente estas decisões:

- tagging ligado por defeito;
- nome e polaridade da opção de desativação;
- interação com `StreamMode::Verbose` e `Compact`;
- política para fórmulas sem descrição adequada;
- se conformidade PDF/UA é apenas medida ou prometida por uma opção pública.

## 4. Auditoria L0 obrigatória

Ler e confirmar os hashes vigentes de:

- `00_nucleo/prompts/entities/layout_types.md`;
- `00_nucleo/prompts/infra/export/mod.md`;
- `00_nucleo/prompts/infra/export/stream.md`;
- `00_nucleo/prompts/infra/export/builder.md`, se existir;
- `00_nucleo/prompts/infra/pipeline.md`;
- L0 da configuração pública de exportação PDF;
- L0 do CLI compile e suas opções;
- L0 de diagnóstico/erro se a validação PDF/UA emitir mensagens.

Também ler ADR-0126 para preservar a decisão de que Verbose é o padrão de
produção e Compact é flag separada. Tagging não pode ser acoplado à verbosidade
do stream.

## 5. Decisões arquiteturais a materializar no L0

### 5.1 Eixos independentes

Três eixos não podem ser confundidos:

1. `Verbose | Compact`: forma mecânica do stream, ADR-0126;
2. `Tagged | Untagged`: presença da estrutura lógica PDF;
3. `PDF/UA validado | não validado`: conformidade do artefato completo.

Todas as combinações tecnicamente suportadas devem ter teste. `--compact` não
desativa tags e `--no-pdf-tags` não força Compact.

### 5.2 Fonte única da semântica

L3 consome somente `FrameItem::Semantic`. É proibido:

- reconhecer fórmula pela fonte matemática;
- inferir alt por `plain_text`;
- procurar `Content::Equation` durante export;
- gerar Formula a partir de cada glifo individual;
- transportar callbacks ou `Value` de volta para L3.

### 5.3 Estrutura PDF mínima

Para cada página tagueada:

- atribuir MCIDs determinísticos por ordem de pintura semântica;
- envolver os operadores visuais da fórmula com BDC/EMC;
- criar `StructElem` `/S /Formula`, `/P` apontando ao pai e `/Pg` à página;
- registrar `/K` com o MCID correspondente;
- montar ParentTree/Nums coerente;
- ligar `StructTreeRoot` ao catálogo;
- definir MarkInfo `/Marked true` somente quando a estrutura estiver completa.

Elementos não semânticos não devem receber tags inventadas neste passo. Se a
estrutura PDF exigir um pai Document para validade, criar somente o esqueleto
mínimo legitimado no L0 e registrar o scope-out dos demais papéis.

### 5.4 Política de `alt`

- `Some("texto")`: emitir `/Alt` como PDF string Unicode correta;
- `Some("")`: preservar e emitir vazio; não converter em ausência;
- `None`: omitir `/Alt`;
- nunca usar texto visual como fallback;
- nunca recusar export PDF comum apenas por falta de alt.

PDF/UA-1 pode diagnosticar ausência/vazio separadamente, mas essa validação não
deve alterar silenciosamente a semântica do PDF comum.

### 5.5 Opção pública

O L0 deve decidir uma representação tipada, evitando booleanos ambíguos, por
exemplo `PdfTags::{Enabled, Disabled}` na configuração de exportação. O CLI
espelha o vanilla com `--no-pdf-tags`; ausência da flag seleciona Enabled.

O nome concreto e a assinatura são contrato público: escrever L0 e parar no
gate antes de adicioná-los.

## 6. Testes RED antes da implementação

### 6.1 Unidade do writer/builder

- catálogo contém `StructTreeRoot` quando tags estão ligadas;
- MarkInfo declara `/Marked true` somente no modo tagueado;
- ParentTree resolve page/MCID para o `StructElem` correto;
- Formula tem `/Alt` Unicode, vazio ou omitido conforme a entrada;
- dois envelopes na mesma página recebem MCIDs distintos e determinísticos;
- fórmulas em páginas distintas não colidem;
- nesting visual em Group/Link não perde a associação semântica;
- conteúdo da fórmula fica entre BDC e EMC exatamente uma vez.

### 6.2 Matriz dos eixos

Testar:

| Stream | Tags | Resultado esperado |
|---|---|---|
| Verbose | Enabled | estrutura + stream verbose |
| Compact | Enabled | estrutura + stream compacto |
| Verbose | Disabled | sem estrutura, verbose |
| Compact | Disabled | sem estrutura, compacto |

### 6.3 Integração pública

- compilação default de fórmula com alt gera PDF `Tagged: yes`;
- `--no-pdf-tags` gera `Tagged: no`;
- ausência/none/vazio/string/Unicode preservam a distinção;
- geometria, páginas e texto visual são invariantes tags on/off;
- PDF abre e é parseável nos dois modos;
- múltiplas fórmulas mantêm ordem estrutural igual à ordem de pintura.

### 6.4 Validação externa

Quando as ferramentas estiverem disponíveis, executar e registrar versões:

- `pdfinfo` para Tagged yes/no;
- `qpdf --check` para integridade;
- `mutool show` ou parser equivalente para Catalog/StructTreeRoot/ParentTree;
- verificador PDF/UA para medir, sem transformar resultado parcial em alegação
  de conformidade.

Se uma ferramenta externa não estiver disponível, registrar limitação e manter
o critério correspondente aberto; não substituir por busca textual cega em
bytes comprimidos.

## 7. Sequência de implementação após o gate

1. Registrar HEAD, hora e `git diff HEAD --stat` do RED.
2. Adicionar configuração tipada de tags na API pública.
3. Threading da configuração por CLI → wiring → pipeline → export.
4. Criar modelo interno de nós estruturais/MCID em L3.
5. Fazer o stream writer envolver Semantic Formula com BDC/EMC.
6. Fazer o builder emitir StructElem, ParentTree, StructTreeRoot e MarkInfo.
7. Implementar `--no-pdf-tags` sem acoplamento a Compact.
8. Passar testes unitários e matriz dos eixos.
9. Executar integração e validadores externos.
10. Medir invariância visual tags on/off.
11. Atualizar diagnóstico P1140.5/P1140.6 com proveniência completa.

## 8. Interação com P1140.7

P1140.7 corrige walkers e testes que ainda assumem `Page.items` plano. Ele não
é pré-requisito arquitetural para escrever o L0 deste passo, mas deve estar
GREEN antes do fechamento conjunto, porque o export tagueado depende de uma
travessia correta de Semantic.

Se P1140.7 encontrar regressão real no envelope, corrigir primeiro a fonte em
L1/L3 e repetir os testes RED deste passo. Não contornar a regressão no writer
PDF.

## 9. Travas finais

```text
cargo fmt --check
cargo test -p typst-core --lib
cargo test -p typst-infra --lib
cargo test -p typst-shell --lib
cargo build --workspace
crystalline-lint .
git diff --check
```

Além disso, guardar comandos, versões e outputs resumidos dos validadores PDF,
sempre com HEAD/working-tree e hora da medição.

## 10. Critérios de aceitação

- [ ] L0 atualizado e gate ADR-0127 confirmado antes do código.
- [ ] Tagging é configuração tipada e ligado por defeito.
- [ ] `--no-pdf-tags` desativa somente tagging.
- [ ] Verbose/Compact e Tagged/Untagged são eixos independentes.
- [ ] Cada Semantic Formula produz um StructElem Formula e MCID coerente.
- [ ] ParentTree, StructTreeRoot e MarkInfo são estruturalmente válidos.
- [ ] Alt Unicode/vazio/ausente preserva distinções sem fallback inventado.
- [ ] Conteúdo visual é emitido exatamente uma vez.
- [ ] Geometria e paginação são invariantes tags on/off.
- [ ] `pdfinfo` reporta Tagged yes no default e no com a flag de desativação.
- [ ] `qpdf --check` passa nos dois modos.
- [ ] Resultado PDF/UA é declarado somente conforme verificação real.
- [ ] P1140.7 e suítes L1/L3 estão no baseline aceito.
- [ ] `crystalline-lint .` termina com zero violations.
- [ ] Diagnóstico final registra proveniência reproduzível.

## 11. Fora de escopo

- inferir descrição da fórmula;
- exigir alt para export PDF comum;
- taguear integralmente headings, listas, tabelas, links e figuras;
- declarar PDF/UA apenas porque Formula/Alt existe;
- misturar tagging com o modo Verbose/Compact;
- usar igualdade de bytes como critério de paridade de linguagem;
- corrigir P862;
- remover ou achatar `FrameItem::Semantic`.

## 12. Resultado esperado

O PDF default do cristalino passa a transportar a semântica Formula já
preservada por P1140.5, com MCID e árvore estrutural verificáveis. A opção
`--no-pdf-tags` mantém um caminho explicitamente não tagueado, e Compact
continua sendo uma decisão ortogonal. O projeto terá uma base real para
expandir acessibilidade a outros elementos sem chamar uma primeira fatia de
Formula de conformidade PDF/UA completa.

## 13. Resultado da execução

Gate ADR-0127 confirmado pelo dono em 2026-08-24. Execução medida em
`2026-08-24T10:37:11-03:00`, sobre working tree não commitada baseada em
`ffd527c85dd7d547413d33cbc2d27a80e32a3f8c`; `git diff HEAD --stat` ao fim
continha 89 ficheiros, 3248 inserções e 307 remoções, incluindo a frente
P1140.4–P1140.7 acumulada.

Implementado:

- `PdfTags::{Enabled, Disabled}`, default `Enabled`;
- `--no-pdf-tags` em compile/watch e tradução tipada em L4;
- threading separado de `StreamMode` pelas entry points PDF;
- BDC/EMC com MCID determinístico por página para cada Formula;
- StructTreeRoot → Document → Formula, `/Pg`, `/K`, ParentTree `/Nums`,
  `/StructParents` e MarkInfo;
- `/Alt` UTF-16BE para Unicode, preservando vazio e omitindo ausência;
- caminho Disabled sem estrutura nem operadores marcados;
- suporte equivalente no caminho do oráculo PDF.

Validação externa nos quatro quadrantes Verbose/Compact × Enabled/Disabled:

- `pdfinfo`: `Tagged: yes` em Enabled e `Tagged: no` em Disabled;
- `qpdf --check`: sem erros de sintaxe ou codificação nos quatro artefatos;
- `mutool show`: catálogo alcança StructTreeRoot e ParentTree;
- testes P1140.6: 3/3 export + 1/1 CLI aprovados;
- L3: 827 aprovados; somente a falha ambiental preexistente do teste de CA
  local (`Operation not permitted`);
- L1: 5142 aprovados; somente as duas falhas P862 preexistentes;
- L2: 53/53; L4: 55/55 + 2/2 lint tests;
- `cargo build --workspace`, `crystalline-lint .` e `git diff --check`
  aprovados.

Não se declara PDF/UA: esta entrega implementa estrutura lógica de Formula e
mede validade PDF, mas não tagueia integralmente os demais papéis do documento.
