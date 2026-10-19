# P1171 — auditar `html.br`, void e whitespace HTML

**Data:** 2026-08-25  
**Estado:** `EXECUTADO — PARADO NO GATE ADR-0127`  
**Baseline cristalina:** working tree P1170.1 GREEN, ainda não staged  
**Baseline de linguagem:** vanilla ratificado `a51e02804`  
**Gate:** ADR-0127 obrigatório para o binding público `html.br` e para qualquer
mudança de fase/contrato descoberta

## Objetivo

Auditar três eixos relacionados, mas não equivalentes:

1. assinatura pública e morfologia de `html.br`;
2. classificação declarativa das 13 tags HTML void e serialização sem end tag;
3. whitespace entre expressões HTML em markup formatado.

Medir cada eixo antes de decidir, atualizar somente os L0s necessários e
**parar no gate ADR-0127** antes de escrever Rust, testes RED, resselo ou código
de produção. Não assumir que corrigir `br` corrige whitespace nem que a tabela
void deve morar na entidade.

## 1. Proveniência obrigatória

Registrar antes da primeira sonda:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
git diff --cached --stat
sha256sum /usr/local/bin/typst ./target/debug/typst
/usr/local/bin/typst --version
./target/debug/typst --version
```

Declarar working tree não commitado P1168–P1170.1, listar os paths alterados e
confirmar índice vazio. Preservar todos os hunks. O hash do binário e o pin
provam a referência; `--version` sozinho não prova.

## 2. Fontes obrigatórias

Ler integralmente, sem acessar/listar `00_nucleo/context/` ou
`00_nucleo/materialization/`:

- `AGENTS.md`, `01_core/CLAUDE.md`;
- ADR-0107, ADR-0108, ADR-0109, ADR-0127 e ADR-0128;
- P1168–P1170.1 e diagnósticos correspondentes;
- L0s `compiler/stdlib/html.md`, `entities/html.md`, `entities/content.md` e
  `infra/export/html.md`;
- `01_core/src/compiler/stdlib/html.rs`;
- `03_infra/src/export/html.rs`;
- vanilla `typst-html/src/typed.rs`, `tag.rs`, `encode.rs`, `fragment.rs` e
  regras de realização relevantes;
- entrada `br`, globais e dados de void no `typst-assets` pinado `94dcb99`, se
  a fonte estiver materializada; caso contrário, registrar a limitação e usar
  lockfile + binário ratificado sem inventar `file:line`.

## 3. Medir a assinatura de `html.br`

Na fonte e no vanilla com `--features html`, medir:

```typst
repr(type(html.br))
repr(html.br())
repr(html.br(id: "b"))
repr(html.br[content])
repr(html.br(none))
```

Confirmar ou refutar:

- `html.br` é função;
- aceita os mesmos 76 globais P1168;
- não possui atributo específico;
- por ser void, não possui parâmetro body;
- chamada vazia não expõe `body: none` se a função não cria esse field;
- qualquer posicional é `unexpected argument` e não cast de content;
- named desconhecido e `data-*` continuam rejeitados;
- attrs globais preservam casts, omissões, ordem e diagnósticos existentes.

Medir `repr`, `.fields()` se observável, igualdade de morfologia relevante e
nesting em sequência. Não copiar a forma Rust do vanilla.

## 4. Matriz void completa

Revalidar em `tag.rs:123-141` as 13 tags:

```text
area base br col embed hr img input link meta source track wbr
```

O número acima deve ser recontado na execução e acompanhado de proveniência;
se a lista ou contagem divergir, prevalece a fonte. Para cada tag, medir no
vanilla por `html.elem`, distinguindo:

- body omitido;
- body vazio explícito;
- body com texto;
- serialização compacta;
- repr antes da expansão;
- diagnóstico, descarte ou preservação do body.

Medir especialmente se `html.elem("br")[X]` é rejeitado no constructor, na
realização ou no exporter. Não inferir a política de filhos apenas de
`is_void`.

Classificar onde a tabela deve ser consumida:

- L1, se valida construção/morfologia pública;
- L3, se decide apenas sintaxe de serialização;
- ambos através de dado puro compartilhado em L1, somente se duas medições
  provarem consumidores reais.

Não adicionar flag pública a `HtmlElem` nem duplicar listas por camada sem
decisão explícita. A classificação é propriedade da tag, não estado por nó.

## 5. Serialização void

Medir no vanilla os fragmentos compactos:

```typst
#html.div[A#html.br()B]
#html.div[#html.br(id: "b")]
#html.elem("br")
#html.elem("br", attrs: (hidden: ""))
```

Confirmar abertura `<br...>` sem `</br>` e sem slash XHTML. Medir escaping,
atributo vazio, ordem e adjacência textual. Repetir no cristalino P1170.1:

- `html.br` deve estar ABSENT;
- `html.elem("br")` atualmente deve revelar o gap `<br></br>`;
- `Content::Linebreak` já emite `<br>` e deve permanecer regressão protegida.

Separar o binding público (gate) da correção interna da tabela/serialização
(fluxo contínuo ADR-0127 depois de L0), salvo se a solução mudar contrato,
default ou fase.

## 6. Whitespace — matriz diferencial

O gap observado anteriormente ocorre em fontes formatadas entre expressões.
Medir vanilla e cristalino, tanto `eval repr` quanto HTML compacto, para:

1. expressões adjacentes na mesma linha;
2. quebra simples entre duas expressões;
3. quebra + indentação;
4. linha vazia/parbreak;
5. espaço explícito em markup;
6. texto antes/depois de `br`;
7. `br` entre tags inline (`span`, `strong`, `a`);
8. tags block (`div`, `p`, `ol`) em linhas separadas;
9. expressão dentro de body `[...]` versus no topo;
10. comentários entre expressões.

Fixtures mínimas devem variar um único fator. Registrar repr de Content antes
do export para localizar a origem:

```text
parser/eval já criou Space/Text? → eixo L1 de linguagem
Content coincide, HTML diverge? → realização/export L3
diferença só em pretty output? → encoder, não morfologia compacta
```

Não usar `.trim()` adicional como correção presumida. Demonstrar quando
whitespace é sintaxe significativa e quando o vanilla o suprime por realização
HTML. Uma mudança eval ↔ realização ↔ export é gate ADR-0127 de fase.

## 7. Interação com agrupamento phrasing

Revalidar o achado P1170:

- anchor isolado no topo: vanilla envolve em `<p>`;
- anchor aninhado em `div`: decalque já MATCH;
- `br` isolado e em sequência de texto podem participar do mesmo agrupamento.

Determinar se whitespace e wrapper `<p>` têm a mesma causa de realização ou
apenas sintomas adjacentes. Não ampliar P1171 para “corrigir toda realização
phrasing” sem matriz e gate explícitos. Se a causa for comum, apresentar dois
recortes aprováveis: mínimo `br`/void e realização phrasing completa.

## 8. Diagnóstico e L0

Criar
`00_nucleo/diagnosticos/typst-p1171-auditoria-html-br-void-whitespace.md`
com:

```text
eixo | sonda | vanilla | cristalino | camada de origem |
língua/mecânica | MATCH/PARTIAL/ABSENT | evidência | decisão
```

Somente depois das medições:

1. atualizar `compiler/stdlib/html.md` com a assinatura completa de `html.br`;
2. atualizar `infra/export/html.md` com a tabela void e sintaxe medida, se
   confirmadas;
3. atualizar `entities/html.md` somente se construção/representação exigir
   contrato novo — a preferência inicial é classificação derivada da tag;
4. atualizar o L0 owner de parser/eval/realização se o whitespace nascer antes
   do exporter;
5. não alterar `entities/content.md` sem variante/campo realmente novo;
6. declarar restantes tags, raw, frame, CSS, MathML e positions fora.

Não ressellar hashes, não escrever testes e não alterar L1–L4 neste passo.

## 9. Gate ADR-0127

Parar e pedir decisões separadas, conforme a evidência:

### Gate A — contrato mínimo

- exatamente um binding público `html.br`;
- exatamente 76 globais existentes;
- zero atributos específicos e zero body;
- reutilização da representação atual, se suficiente.

### Gate B — apenas se necessário

- qualquer campo/tipo público novo para classificação void;
- qualquer mudança de fase para whitespace/realização phrasing;
- qualquer comportamento de agrupamento `<p>` além do caso mínimo medido.

A tabela interna de serialização void pode seguir fluxo contínuo somente se
não alterar contrato público, default ou fase. Em dúvida, incluí-la no gate.

## Critérios de aceitação

- assinatura `br` medida integralmente;
- lista void recontada e citada da fonte;
- `html.elem("br")` com/sem body medido, não presumido;
- `<br>` sem end tag e regressão de `Linebreak` cobertos;
- matriz de whitespace localiza a camada causal;
- agrupamento phrasing não é misturado silenciosamente;
- L0 atualizado antes de código e sem resselo;
- `git diff --check` limpo e índice vazio;
- parada efetiva nos gates ADR-0127.

## Próximo passo após aprovação

Escrever P1171.1 para o recorte mínimo aprovado (`html.br` + void). Se
whitespace/realização exigir mudança de fase ou escopo maior, escrever passo
fracionário separado em vez de acoplá-lo à materialização do binding.
