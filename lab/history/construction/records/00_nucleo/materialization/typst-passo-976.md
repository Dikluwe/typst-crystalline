# Passo 976 — confirmar se o vanilla é determinístico entre execuções, antes de perseguir PDF byte-idêntico

**Precede este passo**: pedido do dono — gerar PDF literalmente idêntico byte a byte ao vanilla.
**Objeção registada, a confirmar com medição antes de prosseguir**: P950 documentou que o vanilla
usa prefixo de subset de fonte **aleatório** (`ABCDEF+NomeReal`), e que o cristalino optou
deliberadamente por um prefixo **determinístico** (`AAAAAA+`) por reprodutibilidade de build — se
isso for confirmado, o vanilla não produz o mesmo PDF byte a byte nem entre duas execuções dele
mesmo, o que tornaria "byte-idêntico" um alvo instável, não um alvo que falta alcançar.

**Este passo não implementa nada** — só confirma, com medição directa, se a premissa é real, antes
de decidir o que "paridade byte a byte" pode significar de facto.

**Pré-condição de árvore**: `git status`. Confirmar P975 (se já executado) presente.

---

## Fase A — testar determinismo do vanilla entre execuções

1. Compilar o mesmo `.typ` (o documento de 30 secções, ou um caso mínimo) com o binário vanilla
   real **três vezes seguidas**, sem tocar em nada entre as execuções.
2. Calcular o hash (`sha256sum`) dos três PDFs resultantes — confirmar se são idênticos ou
   diferentes.
3. Se diferentes: usar `qpdf --qdf`/diff binário para localizar exactamente onde divergem —
   confirmar se é o prefixo de subset de fonte (a hipótese de P950), o `DocumentID`/`InstanceID`
   XMP (mecanismo já conhecido deste projecto, P615/P617, que o cristalino já neutraliza via
   `CRYSTALLINE_PDF_FIXED_EPOCH`), ordem de iteração de alguma colecção interna, ou outra fonte.
4. Repetir o mesmo teste para o binário **cristalino**, três execuções seguidas do mesmo `.typ` —
   confirmar se o cristalino já é determinístico entre execuções (esperado que sim, dado o
   trabalho já feito em P615/617/950 para reprodutibilidade).

## Fase B — decidir o que "paridade byte a byte" pode significar

1. **Se o vanilla for determinístico** (contra a hipótese — confirmar antes de descartar): então
   byte-idêntico é tecnicamente um alvo válido, e vale desenhar um passo de implementação a sério
   (mapeando cada fonte de diferença: prefixo de subset, ordem de objectos PDF, parâmetros de
   compressão, e portando cada um literalmente) — registar isso como próximo passo.
2. **Se o vanilla não for determinístico** (confirmando a hipótese): byte-idêntico contra uma
   execução específica do vanilla não é alvo estável. Alternativas a apresentar ao dono:
   - **Byte-idêntico modulo as fontes conhecidas de não-determinismo do vanilla** (fixar/neutralizar
     o prefixo de subset e qualquer outra fonte encontrada na Fase A, e só depois comparar) — mais
     trabalhoso, mas pode ser o mais próximo de "byte a byte" que faz sentido.
   - **Sequência de operador idêntica, valores numéricos idênticos, mas não necessariamente a
     mesma codificação binária final** (o que P976 original, antes desta correcção, propunha —
     agrupamento de `BT…ET` igual ao vanilla) — mais realista, não persegue um alvo instável.
   - Outra formulação que o dono prefira, depois de ver a medição da Fase A.

## Resultado esperado

- Confirmação directa (hash + diff binário) se o vanilla é ou não determinístico entre execuções
  do mesmo `.typ`.
- Se não for: localização exacta de cada fonte de não-determinismo encontrada.
- Decisão do dono sobre qual formulação de "paridade byte a byte" perseguir, informada pela
  medição, não pela suposição inicial de nenhum dos dois lados.
