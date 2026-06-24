# Correções pendentes — P389, P420, P421 e contagem de DEBT

Este documento trata os itens que ainda não foram corrigidos depois da
primeira rodada. O foco é P420 (as limitações e o cache colocado dentro
do struct de domínio). P389, P421 e a contagem de DEBT entram como
correções menores no fim.

Como antes: onde a correção exige ler o código, o comando de verificação
está escrito, mas o resultado tem de vir do repositório.

---

## P420 — cache de style dentro do struct de domínio

### O que aconteceu

O spec `typst-passo-420.md` decide manter `BibliographyElem` puro:

- A.1.3: "`BibliographyElem` inalterado (já tem `style: Option<EcoString>`)".
- A.1.5 passo 4: "O `BibliographyElem` armazena apenas a string original".
- A.1.5 passo 5 e B.2: o cache de styles vive no `World`
  (`StyleCache { styles: HashMap<EcoString, Style> }`).

A implementação real divergiu disto. Segundo o relatório, foi adicionado
um campo ao struct:

```rust
resolved_style: Option<Arc<IndependentStyle>>
```

A justificativa registada foi que `layout_with_introspector` não recebe
`World`, então não havia outro lugar para guardar o style resolvido.

### Por que isto é uma deriva, e não só uma escolha

Há dois problemas distintos. Separá-los importa, porque um pode ser
legítimo e o outro não é.

**Problema 1 — o campo pode não ter sido necessário.** O próprio spec, em
A.1.5 passo 4, diz que o `Style` resolvido é guardado no `Introspector`
(além do `World`). Se o `Introspector` está disponível em
`layout_with_introspector` — e o nome da função indica que está —, então
o style resolvido podia ter ido para o `Introspector`, como o spec
mandava, sem tocar no struct de domínio. A justificativa "World não está
disponível" responde à pergunta errada: a pergunta é se o `Introspector`
estava disponível. Se estava, o campo no struct foi um atalho. Se não
estava, o campo é defensável, mas aí a divergência face ao spec tem de
ser declarada com esse motivo exato.

**Problema 2 — o custo do campo não foi avaliado.** Independente de o
campo ser necessário ou não, ele introduz estado computado (um cache)
num struct de dados de domínio. `BibliographyElem` participa de
`PartialEq` e `Hash`. O relatório documentou que o struct mudou (contra o
L0 "struct inalterado"), mas não disse como `resolved_style` se comporta
nessas duas operações. Isto não é detalhe: a identidade de um
`BibliographyElem` é usada em deduplicação e memoização do `Introspector`.
Um campo de cache participar (ou não) da identidade muda o resultado
dessas operações.

A "preguiça" é o problema 2, e está em três lugares:

1. O campo foi adicionado sem declarar o comportamento em `PartialEq`/`Hash`.
2. O smell (cache em struct de domínio) não abriu DEBT.
3. O cache que o spec previa (no `World`) foi descrito como scope-out
   com a frase "se complexo demais, documentar 'sem cache; recarrega a
   cada eval'" — o que deixa a decisão por tomar em vez de tomada.

### Verificação no código (antes de escrever a correção final)

```bash
# 1. Como BibliographyElem deriva ou implementa PartialEq e Hash?
rg -n "BibliographyElem" 01_core/src/ -B2 -A6 | rg -n "derive|PartialEq|Hash|resolved_style|struct BibliographyElem"

# 2. IndependentStyle (hayagriva) implementa PartialEq/Hash?
rg -n "IndependentStyle" ~/.cargo/registry/src/*/hayagriva-*/src/ | rg -n "PartialEq|Hash|derive"

# 3. O Introspector está disponível em layout_with_introspector?
rg -n "fn layout_with_introspector" 01_core/src/rules/layout/mod.rs -A10
```

O resultado da verificação 1 diz qual dos dois cenários está em vigor:

- Se `BibliographyElem` usa `#[derive(PartialEq, Hash)]` sem atributo de
  exclusão, então `resolved_style` **participa** da identidade. Isto só
  compila se `Arc<IndependentStyle>` for comparável e hasheável — e mesmo
  que compile, está errado (ver "Decisão recomendada").
- Se há `impl PartialEq`/`impl Hash` manuais, ou um atributo que ignora o
  campo, então `resolved_style` está **excluído**. Isto é o correto, mas
  precisa de ser declarado e justificado.

A verificação 3 diz se o Problema 1 era real (Introspector indisponível)
ou não (campo foi atalho).

### Decisão recomendada

A identidade de um `BibliographyElem` deve vir das suas entradas (`path`,
`style`, `locale`, `title`), não do style resolvido, que é uma função
pura dessas entradas. Logo:

- `resolved_style` deve ser **excluído** de `PartialEq` e `Hash`.
- A exclusão é sólida **sob a invariante** de que `resolved_style` é
  sempre função pura de (`path`/`style`/`locale`). Se essa invariante não
  vale (o campo pode ser preenchido de forma inconsistente), a exclusão
  esconde a inconsistência e vira bug.

Há duas formas de fechar, em ordem de preferência:

**Forma A (preferida) — tirar o campo do struct.** Rotear o style
resolvido pelo `Introspector` (como o spec A.1.5 passo 4 já mandava) ou
passá-lo como parâmetro da função de layout. O struct de domínio volta a
ser puro. Só é viável se a verificação 3 mostrar que o `Introspector`
está disponível no ponto de layout. Esta forma elimina o smell em vez de
o documentar.

**Forma B (se A for inviável) — manter o campo, mas declarar tudo.**
Manter `resolved_style` no struct, com:

- exclusão explícita de `PartialEq` e `Hash`;
- a invariante "resolved_style é função pura das entradas" escrita;
- uma DEBT que rastreia o smell e o custo de o manter consistente.

### Texto a inserir

**1. No relatório P420 (criar a secção se o relatório ainda não existe),
secção "Notas epistêmicas" ou "Divergência face ao L0":**

Para a Forma A:

```
Divergência resolvida (correcção de deriva): a implementação inicial
adicionou resolved_style: Option<Arc<IndependentStyle>> a BibliographyElem.
O struct de domínio passou a guardar estado computado, o que diverge do L0
("struct inalterado"). A correcção remove o campo: o style resolvido é
guardado no Introspector (per spec A.1.5 passo 4) / passado como parâmetro
de layout. BibliographyElem volta a ser puro; a sua identidade (PartialEq,
Hash) é definida apenas pelas entradas (path, style, locale, title).
```

Para a Forma B:

```
Divergência declarada (correcção de deriva): BibliographyElem ganhou
resolved_style: Option<Arc<IndependentStyle>> porque o Introspector não
estava disponível no ponto de layout [confirmar com a verificação 3].
Comportamento em identidade: resolved_style é EXCLUÍDO de PartialEq e Hash.
Dois BibliographyElem com as mesmas entradas (path/style/locale/title) são
iguais mesmo que um tenha o style já resolvido e o outro não. Invariante
que torna a exclusão sólida: resolved_style é função pura das entradas; é
preenchido apenas a partir de (path/style/locale), nunca de forma
independente. Custo de manutenção rastreado em DEBT-XX.
```

**2. Nova entrada na Secção 1 do `00_nucleo/DEBT.md` (só na Forma B):**

```
## DEBT-XX — Cache de style (resolved_style) dentro de BibliographyElem — EM ABERTO (Passo 420)

**Origem**: P420 adicionou resolved_style: Option<Arc<IndependentStyle>>
a BibliographyElem porque o Introspector/World não estava acessível no
ponto de layout. Um struct de dados de domínio passou a guardar estado
computado (cache do style CSL resolvido).

**Risco**: resolved_style está excluído de PartialEq e Hash. A exclusão é
sólida só enquanto o campo for função pura de (path/style/locale). Se um
passo futuro preencher o campo por outro caminho, dois BibliographyElem
iguais nas entradas mas com styles resolvidos diferentes serão tratados
como iguais, escondendo a divergência na deduplicação/memoização do
Introspector.

**Critério de fecho**: mover o style resolvido para fora do struct de
domínio (Introspector ou parâmetro de layout), repondo BibliographyElem
puro; OU provar e fixar em teste a invariante "resolved_style é função
pura das entradas" para que a exclusão permaneça sólida.

**Magnitude**: S-M (refactor do ponto de layout).
```

**3. No spec `typst-passo-420.md`, secção A.1.8 (limitações) — tornar os
scope-outs explícitos em vez de pendentes.** O cache é o caso principal.
Substituir a frase pendente do A.1.5 passo 5 / B.2 ("se complexo demais,
documentar 'sem cache; recarrega a cada eval'") por uma decisão tomada:

```
Cache de styles: DECISÃO (não pendente). O cache vive [no World, per B.2]
OU [não existe; recarrega a cada eval]. Não deixar como "depende".
Se o caminho real foi resolved_style no struct, isso NÃO é o cache previsto
aqui — é um campo por elemento, rastreado em DEBT-XX, e não substitui a
decisão sobre o StyleCache do World.
```

### As outras limitações de P420 (já listadas, mas não fechadas nos critérios)

O spec A.1.8 lista seis scope-outs: CSL via URL, CSL em diretório de
sistema, múltiplos styles simultâneos, hot-reload, validação completa de
schema CSL, e cache cross-evaluation. Eles estão no texto mas não nos
critérios de fecho (a lista de checkboxes da Fase C não os menciona). A
correção é levá-los aos critérios de fecho como itens marcados
explicitamente como scope-out, para que nenhum passe por esquecimento:

```
- [scope-out] CSL via URL — só paths locais
- [scope-out] CSL em diretório de sistema (~/.csl/) — só path explícito
- [scope-out] Múltiplos styles simultâneos — primeiro BibliographyElem governa
- [scope-out] Hot-reload de CSL em runtime
- [scope-out] Validação completa de schema CSL (relaxNG) — hayagriva faz a básica
- [scope-out] Cache cross-evaluation persistente — P420.X
```

### Critério de fecho de P420 (correção)

- A verificação 1 foi executada; o comportamento de `resolved_style` em
  `PartialEq`/`Hash` está declarado no relatório.
- Foi escolhida a Forma A ou a Forma B, com o motivo registado.
- Na Forma B, existe DEBT-XX e a invariante está escrita.
- O cache do `World` deixou de ser "depende" e virou decisão.
- Os seis scope-outs estão nos critérios de fecho.

---

## P421 — nota de reclassificação S→M ausente no spec

### O que falta

A ADR-0114 regista que a sonda de P421 revelou que `native_repr` não
existia e que o passo foi reclassificado de S para M. Mas o spec
`typst-passo-421.md` continua a dizer, no cabeçalho, "Tipo:
Materialização (S)" e, nos bloqueadores, "`native_repr` existe como
infraestrutura". O spec contradiz a ADR-0114.

### Correção

Inserir uma nota de reclassificação no topo do spec P421, logo após o
título, no mesmo formato usado em P409/P413/P416:

```
> **Reclassificação retroativa (S→M):** a sonda A.0 revelou que native_repr
> não existia como infraestrutura completa; o passo foi reclassificado de S
> para M. O cabeçalho e a linha de bloqueadores abaixo descrevem o estado
> assumido antes da sonda. Ver ADR-0114 (gate sonda-antes-da-spec).
```

Corrigir também a linha de bloqueadores: trocar "`native_repr` existe como
infraestrutura" por "`native_repr` parcial/ausente — confirmado pela sonda
A.0; ver ADR-0114".

---

## Contagem de DEBT "10" na entrada P407

### O que está errado

A entrada P407 no `DEBT.md` diz "Contagem de DEBTs abertos: inalterada
(**10**)". A auditoria do Passo 275, no mesmo ficheiro, reconciliou a
contagem para **8** e marcou o 10 como herança desatualizada. Depois
disso, DEBT-62 fechou em P398. O número na entrada P407 é o velho.

### Correção

A correção exige confiar na contagem real do `DEBT.md` na data de P407.
A partir do que está no ficheiro: 8 abertos pós-P275, menos DEBT-62
(fechado P398) = 7, salvo DEBTs abertos entre P275 e P407 que eu não
tenha visto. Verificar e then corrigir a frase. Texto sugerido (ajustar o
número ao real):

```
Contagem de DEBTs abertos: inalterada por este passo. Nota: o número
correcto pós-auditoria P275 é 8 (não 10); após o fecho de DEBT-62 em P398
o saldo é [N]. A referência a "10" em relatórios desta série herda a
contagem anterior à auditoria P275 e não deve ser propagada.
```

Se houver outras entradas da série P400+ que repetem "10", aplicar a mesma
correção, porque é exatamente o erro que a auditoria P275 mandou parar de
cometer.

---

## P389 — output da sonda ainda ausente

### O que falta

O ficheiro enviado é o spec da sonda (`typst-passo-389.md`), que descreve
o que a sonda deve medir e manda produzir
`typst-sonda-ausentes-ordem-passo-389.md`. Esse output, que P390–P395
citam como §2D (fonte das âncoras `file:line`), continua sem aparecer.

### Correção

Localizar o output em `00_nucleo/diagnosticos/`:

```bash
ls 00_nucleo/diagnosticos/ | rg "sonda-ausentes|passo-389"
```

Se existir, está só fora dos materiais enviados — confirmar o caminho nos
passos que o citam. Se não existir, reconstruí-lo rodando a sonda descrita
no spec e gravando como ficheiro imutável (paridade ADR-0085). Sem ele, as
âncoras §2D que P390–P395 citam não são verificáveis.

### Critério de fecho

O ficheiro existe em `00_nucleo/diagnosticos/` e cada `file:line` citado
por P390–P395 corresponde a uma linha presente nele.
