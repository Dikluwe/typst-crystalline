# L0 — Passo 1095: Investigação — Deslocamento Vertical Constante em `oracle` Antes da Secção 30

**Gate**: `ADR-0127` se confirmar necessidade de correcção. Este passo é
investigação.

**Base**: reconfirmação com arquivos novos (2026-08-12, mesmo dia do achado
original). `crystalline` confirmado corrigido pela cadeia P1086-1094 (Δ máx
0.09pt, paridade essencialmente perfeita). `oracle` mudou de natureza de
divergência: não é mais o padrão crescente/trocando sinal já corrigido — é um
deslocamento **constante** de ~1.83-1.92pt em todos os 13 pontos, sugerindo
causa **antes** da secção 30 (fora do recorte desta página), não dentro dela.

---

## 0. Esclarecer o que é `oracle` antes de investigar

Não tenho, nesta conversa, definição clara do que `oracle` representa em
relação a `crystalline` — a nota original tratava os dois como tendo "mesmo
bug/estado" antes da correcção, e agora divergem de forma diferente. Antes de
prosseguir, confirmar: `oracle` é um binário/branch separado (build de
referência ou de teste, talvez anterior à correcção P1086-1094 aplicada só a
`crystalline`), ou é o mesmo binário com configuração diferente de
documento/entrada? Isto muda o que faz sentido investigar — se for o mesmo
código-base sem a correcção aplicada, a causa pode já estar resolvida e só
precisar de re-build; se for código genuinamente diferente, precisa de
auditoria própria.

## 1. Pedir o documento completo, não só o recorte da secção 30

A nota já identifica a limitação: "algo antes da secção 30 (não capturado
neste recorte de página única)". Sem o documento desde o início, não há como
localizar a causa. Pedir o `.typ` fonte completo (ou pelo menos até ao fim da
secção 29) e os PDFs correspondentes gerados por `oracle` e vanilla.

## 2. Resolver a discrepância 4.06pt (altura) vs 1.85pt (deslocamento) antes de investigar a causa

Estes dois números não precisam de ser iguais — mas a nota já assinala que a
relação entre eles "ainda não está explicada com os dados que tenho", o que é
honesto e correcto não presumir. Duas hipóteses a testar quando o documento
completo estiver disponível:

1. O deslocamento de ~1.85pt acontece **uma vez**, antes da secção 30
   (ex.: um bloco/margem a mais algures nas secções 1-29), e o crescimento de
   4.06pt na altura da página é number de **outra** coisa, não relacionada
   directamente (ex.: espaço extra no fim do documento, ou a soma de vários
   pequenos ajustes que não se manifestam como deslocamento visível na
   secção 30 especificamente).
2. Os dois têm a mesma causa, mas a relação não é 1:1 directa — por exemplo,
   se o deslocamento acontece a meio do documento e há mais conteúdo depois
   da secção 30 que também é empurrado, a altura total cresce mais do que o
   deslocamento medido na secção 30 sozinha.

Não escolher uma das duas sem ver o documento completo.

## 3. Localizar a origem do deslocamento constante

Uma vez com o documento completo: comparar página a página (ou secção a
secção) `oracle` vs vanilla, procurando o ponto exacto onde o deslocamento de
~1.85pt aparece pela primeira vez — antes desse ponto, os dois devem
coincidir; depois, a diferença deve ser constante e igual a ~1.85pt em toda a
extensão restante (confirmar que é mesmo constante ao longo de todo o
documento, não só nos 13 pontos da secção 30 já medidos).

## 4. Confirmar se `oracle` já tem ou não a correcção de P1086-1094

Dado o esclarecimento pedido em §0: se `oracle` for uma build sem a
correcção aplicada, o deslocamento constante pode ser uma manifestação
diferente do mesmo tipo de causa (margem/espaçamento mal calculado), não um
problema novo — confirmar antes de tratar como achado independente.

## Critério de conclusão

- §0 respondido — natureza de `oracle` esclarecida.
- Documento completo obtido, não só o recorte de página.
- Ponto de origem do deslocamento localizado.
- Relação entre os 4.06pt e 1.85pt explicada com dados reais, não hipótese.
