# P1178 — auditar família HTML de documento

**Data:** 2026-08-25
**Estado:** `EXECUTADO — PARADO NO GATE ADR-0127`
**Baseline cristalina:** working tree P1173.1–P1177.1 GREEN, ainda não staged
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Gate:** ADR-0127 obrigatório antes de acrescentar bindings públicos

## Objetivo

Auditar e nuclear conjuntamente:

```text
html head body title
```

Medir primeiro assinatura, construção e morfologia no vanilla pinado. Separar
o contrato dos constructors da composição automática de documento feita pelo
exporter. Determinar como nós explícitos `html`/`head`/`body` interagem com
doctype, metadata e envelope existentes, e se `title` exige whitespace
escapable-raw em L3. Atualizar os L0s somente após a evidência e parar no gate
ADR-0127 antes de resselo, testes ou código público.

## 1. Proveniência

Registrar:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
git diff --cached --stat
git diff --cached --quiet
sha256sum /usr/local/bin/typst ./target/debug/typst
/usr/local/bin/typst --version
./target/debug/typst --version
```

Declarar a working tree acumulada, enumerar os paths e provar índice vazio.
Versão impressa não substitui o pin nem o hash do binário.

## 2. Fontes obrigatórias

Ler integralmente, sem acessar/listar `00_nucleo/context/` ou
`00_nucleo/materialization/`:

- `AGENTS.md` e `01_core/CLAUDE.md`;
- ADR-0107, ADR-0108, ADR-0109, ADR-0127 e ADR-0128;
- P1165–P1177.1 e diagnósticos HTML relevantes;
- L0s `compiler/stdlib/html.md`, `entities/html.md` e
  `infra/export/html.md`, com hashes vigentes;
- owners L1/L3 atuais;
- vanilla pinado: `typed.rs`, `tag.rs`, `property.rs`, `convert.rs`,
  `document.rs` e dados gerados das quatro entradas;
- inventário P1140.26 apenas como índice auxiliar, se materializado.

Se os assets gerados não estiverem presentes, triangular lockfile, fonte e
binário sem inventar linhas ou tipos.

## 3. Assinatura integral

Comparar cada função com `html.div` e registrar:

```text
tag | total | globais | específicos | ordem/casts | body/input |
void/raw/escapable-raw | display | decisão
```

Verificar especialmente:

- se `html`, `head` ou `body` possuem atributos específicos além dos 76
  globais;
- se `title` recebe body Content posicional opcional, apesar da categoria
  escapable-raw usada na conversão;
- chamada vazia e distinção `body: none`;
- ausência ou presença real de validação de parent, unicidade ou target no
  constructor.

Não inferir assinatura a partir da validade HTML nem confundir a classificação
escapable-raw do exporter com body string da API.

## 4. Sondas de linguagem

Por tag, medir no vanilla:

```typst
repr(type(html.TAG))
repr(html.TAG())
repr(html.TAG[X])
repr(html.TAG(id: "k", class: "c", hidden: true)[X])
```

Medir todos os específicos encontrados e sentinelas `href`, `value`, `start`,
unknown named, segundo body e body named. Construir cada tag isoladamente,
fora da hierarquia normativa e em target paged/HTML, para provar ou refutar
validação contextual.

## 5. Morfologia de documento

Comparar vanilla tipado com cristalino genérico em fixtures compactas e
formatadas:

1. cada tag isolada no topo, vazia e com texto;
2. `html[head[title[T]] body[B]]` completo;
3. `head`/`body` sem `html`, em ambas as ordens e repetidos;
4. `title` isolado, vazio e repetido;
5. espaços, newlines e markup dentro de `title`;
6. caracteres `& < > "`, linebreak e tags aninhadas em `title`;
7. attrs vazios, ordem e escaping nas quatro tags;
8. siblings antes/depois do documento explícito.

Responder com evidência:

- um `html` explícito substitui, aninha, funde ou é rejeitado pelo envelope
  automático?
- `head` e `body` são serializados no fluxo normal ou extraídos para o
  documento?
- metadata automática (`charset`, viewport, idioma) permanece, muda de ordem
  ou é suprimida?
- `title` preserva whitespace como pre/raw, escapa texto e admite markup?
- nós estruturais atuam como boundaries top-level?
- o exporter precisa de regra interna nova ou a representação atual basta?

Byte-identidade só é critério quando o próprio HTML emitido é o observável;
constructor/repr continuam avaliados no nível da linguagem.

## 6. Baseline cristalina

Com feature off, preservar o diagnóstico gated. Com feature on:

- provar as quatro funções ABSENT antes de código;
- provar os 58 bindings até P1177.1 presentes;
- medir `html.elem` para as mesmas tags sem promover equivalência genérica a
  MATCH de assinatura;
- classificar semântica, sintaxe e morfologia separadamente por tag.

## 7. Decisão somente após a medição

Classificar cada tag individualmente. Uma divergência de assinatura remove
somente a afetada; uma regra L3 deve ser especificada separadamente. Não
presumir que toda a família é global-only nem que o envelope cristalino atual
é legitimado pelo comportamento observado.

Hipóteses a testar, não decisões:

- os quatro constructors reutilizam `native_typed_html` com metadados por tag;
- `title` mantém body Content no eval e recebe tratamento escapable-raw apenas
  na conversão;
- `html`/`head`/`body` podem exigir composição documental L3 distinta da
  serialização recursiva comum.

Refutadores: body string/obrigatório, atributos específicos não representados,
validação contextual, extração/merge documental, erro próprio, estado novo ou
mudança de fase.

Continuam fora deste passo:

```text
caption table tbody tfoot thead tr
```

Também ficam fora outras tags, void/raw `script`/`style`, frame, CSS, MathML e
positions.

## 8. Diagnóstico, L0 e gate

Criar:

```text
00_nucleo/diagnosticos/typst-p1178-auditoria-html-familia-documento.md
```

Após medir:

1. atualizar `compiler/stdlib/html.md` somente com os constructors
   sobreviventes e suas assinaturas integrais;
2. atualizar `infra/export/html.md` com cada observável novo de composição,
   envelope ou whitespace;
3. atualizar `entities/html.md` somente se a representação atual for
   insuficiente;
4. não ressellar e não escrever testes/código L1–L4.

Parar e solicitar aprovação ADR-0127 para exatamente os bindings
sobreviventes e qualquer contrato público adicional realmente necessário. A
aprovação não cobre tabela, raw `script`/`style`, outros constructors,
entidade, default ou pipeline não explicitamente nucleados.

## Critérios de aceitação

- assinatura integral e categorias das quatro tags medidas;
- isolamento e hierarquia inválida testados;
- envelope, metadata, nesting, repetição e boundaries comparados;
- whitespace/escaping/morfologia de `title` caracterizados;
- 58 bindings anteriores e feature gate preservados;
- decisão posterior à evidência, com inferências e refutadores marcados;
- L0s atualizados sem resselo ou código;
- `git diff --check` limpo e índice vazio;
- parada efetiva no gate ADR-0127.

## Próximo passo após aprovação

Escrever P1178.1 para resselo, RED e materialização GREEN somente das tags e
correções internas explicitamente aprovadas.

## Resultado da execução

As quatro tags sobreviveram à auditoria com 76 globais e body Content
posicional opcional. Não há específicos nem validação contextual no
constructor. A medição encontrou composição L3 própria: `html` único substitui
o envelope, `body` único é adotado pelo envelope, ambos exigem exclusividade e
`head` permanece nó comum. `title` exige whitespace pre, filhos somente
textuais e escape escapable-raw. Os L0s L1/L3 foram atualizados sem resselo; a
entidade não mudou. Nenhum teste ou código L1–L4 de P1178 foi escrito.
