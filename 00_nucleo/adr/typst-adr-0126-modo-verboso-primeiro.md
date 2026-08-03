# ADR-0126 — Modo verboso primeiro, compacto depois, acessibilidade em eixo separado

**Estado:** `EM VIGOR`
**Data:** 2026-08-03
**Decisor:** dono — decisão tomada **em conversa directa**, fora do ciclo normal de
passo, entre P953 e P954 (proveniência real confirmada por varredura — ver §2).
**Registada no Passo 954; complemento de proveniência (varredura ao histórico
completo) no Passo 955 — ver §2.**
**EMENDA P956 (2026-08-03):** os rótulos "verboso"/"compacto" estavam
**invertidos** na redacção original (P954/P955) — correcção do dono em §1;
e o modo verboso **vira o caminho de produção padrão** (não é ferramenta de
diagnóstico descartável), com o compacto a passar a **flag opcional**. O texto
original fica preservado em §1 para histórico.

---

## 1. A decisão (formulação do dono, citada verbatim do cabeçalho de P954)

> "modo verboso primeiro (auditoria), modo compacto depois (validado pelo decalque
> contra o verboso), acessibilidade como eixo separado"

### ⚠️ EMENDA P956 (2026-08-03) — inversão dos rótulos + destino final dos modos

**Correcção do dono (1) — os rótulos estavam invertidos.** A redacção original
(§1 texto original abaixo, e o complemento P955 em §2) descrevia "modo verboso"
como "um `BT…ET` por item" — o formato *já existente* desde o Passo 20. Esse
formato é na verdade o mais **enxuto** dos dois em semântica de operadores
face ao vanilla. O correcto é:

1. **Modo verboso = espelhar a semântica do vanilla, operador a operador** —
   `Tm` em vez de `Td`, `q`/`cm`/`Q` a isolar cada bloco de texto, `cs`/`scn`
   declarado por transição de cor, `Tr` explícito. **É um modo novo, a
   construir** (não existe no exportador).
2. **Modo compacto = o formato actual do exportador**, existente desde o
   Passo 20 (P955) — `Td`, um `BT…ET` por item, sem `q`/`cm`/`Q` por bloco.

**Correcção do dono (2) — destino final dos modos.** O modo verboso **não é
só ferramenta de diagnóstico**: depois de implementado e validado
**directamente contra o vanilla** (comparação operador a operador), **vira o
caminho de produção padrão** — o comportamento por defeito do compilador. O
modo compacto **deixa de ser o único caminho** e passa a **flag opcional**
(para quem quer PDF menor, aceitando menos semântica/estrutura), **validada
por decalque contra o modo verboso** (agora o padrão) — preservada, não
removida. A hierarquia de validação fica: verboso ← validado contra o
vanilla; compacto ← validado por decalque contra o verboso. A ordem da
formulação original mantém-se ("verboso primeiro, compacto depois"): primeiro
constrói-se e valida-se o verboso; só depois se certifica o compacto contra ele.

O ponto 3 da redacção original (**acessibilidade como eixo separado**)
permanece **inalterado** pela emenda.

### Texto original (P954) — rótulos INVERTIDOS, preservado para histórico

> Aplicada à frente de exportação PDF, esta ordem de prioridade significa:
>
> 1. **Modo verboso primeiro.** Qualquer trabalho na emissão de content streams
>    mantém (ou produz primeiro) a forma verbosa — operadores explícitos e
>    auditáveis, um bloco `BT … ET` por item — porque é essa forma que serve de
>    **baseline de auditoria** contra o vanilla (`mutool trace`, `compare.py`,
>    overlays).
> 2. **Modo compacto depois, validado por decalque.** A compactação de operadores
>    (a frente 2 sondada em P884: ~199 vs ~18 blocos `BT … ET` por página face ao
>    vanilla) só entra **depois**, e a sua validação é o **decalque contra o modo
>    verboso** — mesmas posições de glifos, mesma tinta, operadores diferentes.
>    O modo compacto nunca se valida directamente contra o vanilla sem passar
>    pelo decalque com o verboso.
> 3. **Acessibilidade (PDF tagueado, `BDC`/`EMC`) é um eixo separado.** Não entra
>    nos passos de compactação, nem a compactação nos passos de acessibilidade.
>    São frentes independentes, com specs e passos próprios; a promoção do PDF
>    tagueado a prioridade activa continua a ser decisão separada do dono
>    (reconfirmado em P953 §4).

## 2. Proveniência (Fase A de P954 — varredura real, não presumida)

A formulação exacta **não consta de nenhum relatório de passo** (P944–P953
varridos por `verbos|compact|acessibil|BDC|EMC|tagged|eixo separado|ordem de
prioridade` — só P953 §4 menciona `BDC`/`EMC`, como scope-out), **nem de
nenhum prompt L0**, **nem de nenhuma ADR**. Os antecedentes mais próximos
registados são:

- **P953 §4** (`typst-passo-953-relatorio.md`): `BDC`/`EMC` mantido como
  scope-out deliberado; "se se quiser promover a prioridade activa, é uma
  decisão separada do dono".
- **P883/P884** (`typst-passo-883-relatorio.md`, `typst-passo-884-relatorio.md`
  §4): frente 2 — redução da verbosidade dos operadores PDF — sondada e
  adiada ("requer passo dedicado").

**Conclusão da Fase A:** a decisão foi tomada directamente na conversa com o
dono, fora do ciclo de passo. Esta ADR regista-a com essa proveniência real —
não se inventa um passo de origem.

**Complemento de proveniência (P955, varredura alargada a todo o histórico):**
a pergunta "de onde veio o formato actual do exportador" foi respondida por
varredura completa (não só P944–P953): o exportador nasceu no **Passo 20**
("export_pdf() e PDF mínimo válido", commit `f0f81549c` "Passo 10-23",
2026-03-28) já com o formato de hoje — um bloco `BT … ET` por item de texto,
posicionamento por `Td` (`typst-passo-20.md:212`), sem `q`/`cm`/`Q` por bloco
e sem conteúdo marcado. **Esse formato nunca foi decidido contra alternativas**:
o Passo 20 especifica-o directamente como "PDF mínimo válido", sem discussão de
`Tm`, sem discussão de blocos partilhados; as únicas menções posteriores a `Tm`
são inventário (P282 §A1.1, paridade top-level/local) e um sub-item adiado de
`y_offset` por glifo (P486 §B.3) — nenhuma reconsidera o formato base. Os
passos seguintes (P137 `Tc`, P139 `q … Q` para stroke, P281/282 emit unificado,
P483+ `TJ`/shaping) **estenderam** o formato, nunca o reavaliaram. Nenhuma ADR
antiga (0001–0055) trata da estrutura do content stream. Ou seja: o formato do
Passo 20 é a **implementação mínima viável da altura, nunca reconsiderada** —
o que é distinto de "formato simples escolhido conscientemente".
**[Emenda P956: a redacção original deste parágrafo identificava esse formato
como o "modo verboso" — rótulo invertido, corrigido em §1: o formato do Passo
20 é o modo COMPACTO; o modo verboso é o modo NOVO que espelha a semântica do
vanilla.]** A conclusão de não-tensão mantém-se com os rótulos corrigidos:
não existindo decisão antiga com razões registadas para `Td`/bloco-por-item,
a prioridade "verboso primeiro, compacto depois" não contradiz nada — o modo
verboso (novo) constrói-se sem conflito com o formato existente, que passa a
ser o modo compacto (flag opcional, Fase A de P956).

**Nota de interpretação (marcada como inferência, ADR-0108):** a ligação de
"modo verboso/compacto" à emissão de content streams PDF é a leitura
contextual mais forte (os dois antecedentes acima cobrem exactamente os dois
eixos: verbosidade de operadores e PDF tagueado). O que a refutaria: o dono
declarar que os modos se referem a outro artefacto — nesse caso esta ADR é
corrigida com a proveniência emendada, mantendo a ordem de prioridade.

## 3. Não-colisão com ADRs existentes (Fase B de P954 — varredura real)

Varredura de `00_nucleo/adr/` por `verbos|compact|acessibil|BDC|EMC|tagged|
decalque|L11|FlateDecode|content stream|compress` + leitura integral das
três ADRs aparentadas:

| ADR | Tema | Relação |
|-----|------|---------|
| ADR-0114 | Sonda A.0 antes da spec (gate duro) | Método de investigação — **não colide** (não decide ordem de implementação de modos de exportação) |
| ADR-0117 | Mecanismo operacional da sonda A.0 | Idem — **não colide** |
| ADR-0119 | Disciplina de verificação (+ verificação tipográfica via `mutool`) | Método de verificação — **complementar**, não colide: o "decalque" desta ADR é uma aplicação concreta da disciplina de ADR-0119 §5 |
| ADR-0120 | `TextShaped`/rustybuzz | Menciona acessibilidade só de passagem (`char_code` para ToUnicode) — **não colide** |
| ADR-0060 | `asset` com alt-text | Acessibilidade de conteúdo de imagem, não PDF tagueado — **não colide** |

Nenhuma ADR existente trata de modos de verbosidade do export nem de PDF
tagueado. Número confirmado por listagem real do directório: último ficheiro
`typst-adr-0125-*` → **slot livre = 0126**.

## 4. Alternativas consideradas

1. **Compacto directo, sem modo verboso preservado** — rejeitada: perde a
   baseline de auditoria; sem a forma verbosa não há decalque possível e a
   verificação ficaria reduzida a "parece igual" (a classe de erro que
   ADR-0119/L11 proíbem fechar sem recibo).
2. **Acessibilidade no mesmo eixo da compactação** — rejeitada: a compactação
   muda operadores de texto; o tagging muda a estrutura lógica do documento
   (`BDC`/`EMC`, structure tree). Misturá-los no mesmo passo torna impossível
   atribuir regressões a uma das frentes (mesma lição de P952: "cinco bugs
   distintos a somar-se").
3. **Registo leve (nota em L0) em vez de ADR** — rejeitada: ainda não há
   código desta frente, logo não há L0 a que a nota se possa pendurar; a
   decisão é de produto/prioridade e perene — o veículo é ADR.
4. **Extensão de ADR-0119 (ou 0114/0117)** — rejeitada: essas ADRs são de
   método (como investigar/verificar); esta é uma decisão de produto (o que se
   constrói primeiro e como se valida). Assuntos distintos, ficheiros
   distintos.

## 5. Consequências

**[Emenda P956: esta secção foi escrita com os rótulos invertidos; lê-se em
baixo a versão corrigida. O texto original segue depois, preservado.]**

Versão corrigida (P956):

- **Positiva:** o modo verboso (vanilla-espelhado) torna-se o **caminho de
  produção padrão** — todo o documento compilado sem flag passa a emitir a
  semântica completa de operadores do vanilla.
- **Positiva:** o modo compacto (formato Passo 20) fica preservado atrás de
  uma **flag opcional documentada**, validado por decalque contra o verboso.
- **Positiva:** PDF tagueado fica com fronteira limpa para uma spec própria,
  sem herdar restrições dos modos de emissão.
- **Negativa/custo:** o PDF por defeito (verboso) fica **maior** que o actual
  (mais operadores por bloco) — a troca exacta (tamanho vs semântica) é medida
  e documentada na Fase D de P956, para quem decidir usar a flag compacta
  saber o que troca.
- **Operacional:** o modo verboso é validado **directamente contra o vanilla**
  (operador a operador); a flag compacta é validada **por decalque contra o
  verboso** — ambos com comando + recibo (ADR-0119/ADR-0121).

Texto original (P954, rótulos invertidos — preservado):

- **Positiva:** a frente de compactação de streams (quando for promovida) tem
  a ordem e o critério de validação fechados à partida — não se reabre a
  discussão em cada passo.
- **Positiva:** o modo verboso passa a ser artefacto protegido: refactors do
  export não o podem destruir sem alternativa de auditoria equivalente.
- **Positiva:** PDF tagueado fica com fronteira limpa para uma spec própria,
  sem herdar restrições da compactação.
- **Negativa:** enquanto não houver modo compacto, os PDFs continuam maiores
  que o vanilla (estado medido em P884 — adiamento consciente, já registado).
- **Operacional:** qualquer passo da frente de compactação tem de declarar no
  seu plano o decalque contra o modo verboso como gate de aceitação (comando
  + recibo, per ADR-0119/ADR-0121).

## 6. Referências

- `00_nucleo/materialization/typst-passo-954.md` (cabeçalho — formulação do dono).
- `00_nucleo/materialization/typst-passo-956.md` (cabeçalho — correcção dos
  rótulos e do destino final dos modos; origem da emenda).
- `00_nucleo/materialization/typst-passo-20.md` (origem do formato do
  exportador — identificado em P955).
- `00_nucleo/diagnosticos/typst-passo-955-relatorio.md` (varredura completa do
  histórico — complemento de proveniência).
- `00_nucleo/diagnosticos/typst-passo-953-relatorio.md` §4 (scope-out `BDC`/`EMC`).
- `00_nucleo/diagnosticos/typst-passo-884-relatorio.md` §4 (sonda da frente 2).
- ADR-0107 (paridade com a linguagem), ADR-0108 (medir antes de decidir),
  ADR-0114/0117 (sonda antes da spec), ADR-0119 (disciplina de verificação),
  ADR-0121 (proveniência de medições).
