# ADR-0110 — Marcador de correspondência vanilla `@vanilla` (legível por máquina)

**Estado:** `PROPOSTO` (Passo 385; diagnóstico-primeiro — não materializa código).
**Decisão pendente:** confirmação do dono + execução do roadmap de sub-passos (§Roadmap).
**Histórico de numeração:** `00_nucleo/adr/` varrido — o maior em uso era 0109; **0110 é o
primeiro livre** (precedente P160A/P376: nunca assumir número sem varrer). ADR-0109 registou
que o slot 0110 chegou a ser inventado e corrigido; aqui 0110 é ocupado de propósito após varredura.
**ADRs relacionadas:** ADR-0026 (Content enum fechado, `*Elem`→`Content::*`), ADR-0033 (paridade
vanilla), ADR-0054 (scope-out graded), ADR-0107 (paridade é com a língua), ADR-0108 (medir antes
de decidir).

---

## Contexto

O Passo 385 mediu (script determinístico) o conjunto "só-vanilla" da lente — 10745 símbolos
vanilla sem par no cristalino. A decomposição (ver
`00_nucleo/diagnosticos/typst-decomposicao-so-vanilla-passo-385.md`) mostrou que **96% não é
dívida**: 58.3% mecânica do Rust (ADR-0107), 16.3% scope-out declarado, 3.6% renomes com registro.
O resíduo a julgar é ~21.8%, e encolhe sob julgamento.

O diagnóstico isolou a **causa de a leitura inflar**: a correspondência vanilla↔cristalino **já está
escrita** — na seção "Sobre paridade" de cada L0, nas entradas do Inventário 148, no corpo dos
prompts de elemento — mas **não num campo que a máquina leia**. A lente pareia por nome (chave K4);
o pareamento existe, só não é consumível. Evidência dura: dos 214 L0, só **33** têm seção "Sobre
paridade", e `entities/elements/` tem **0 de 66** — exatamente onde vive o renome dominante
`HeadingElem`→`Content::Heading`.

## Decisão (proposta)

Fixar um **marcador de correspondência vanilla legível por máquina**, `@vanilla`, que vive **ao lado**
da prosa "Sobre paridade" (não a substitui: a prosa é para o humano, o marcador é o que a ferramenta
lê). É o registro de construção promovido a **oráculo de correspondência** consumível — análogo ao
cabeçalho de linhagem `@prompt`/`@prompt-hash`, mas para a relação com o vanilla.

### As quatro formas

| Forma | Significado | Exemplo |
|-------|-------------|---------|
| `@vanilla <caminho::Símbolo>` | correspondência 1:1 (renome com registro) | `@vanilla typst_library::model::heading::HeadingElem` |
| `@vanilla-agrega [<A>, <B>, …]` | um construto cristalino agrega N vanilla | `@vanilla-agrega [UnderlineElem, OverlineElem, UnderbraceElem, …]` |
| `@vanilla-nenhum` | sem equivalente vanilla (construto cristalino próprio) | `@vanilla-nenhum` |
| `@scope-out <ADR>` | vanilla deliberadamente não perseguido | `@scope-out ADR-0026` (vtable) / `@scope-out ADR-0054` (graded) |

O marcador, uma vez semeado, dá à lente (ou a qualquer ferramenta) o campo que falta: o
pareamento deixa de depender de igualdade de nome e passa a ser **declarado**. Os baldes do
diagnóstico 385 tornam-se **calculáveis sem heurística**: balde 1 = soma dos `@vanilla`/`@vanilla-agrega`;
balde 2 = soma dos `@scope-out`; resíduo = símbolos vanilla sem nenhum marcador apontando para eles.

## Roadmap de materialização (sub-passos dedicados — NÃO neste passo)

1. **Semeadura.** Preencher `@vanilla*` nos L0 a partir das seções "Sobre paridade" e do registro
   de construção existentes (Haiku/Sonnet propõe a partir do corpo do prompt + Inventário 148; humano
   confirma). Prioridade: `entities/elements/` (0/66 — maior retorno: fecha o renome dominante).
2. **Regra de fecho.** Tornar o marcador obrigatório no fecho de ficheiros L1–L4 que materializem
   um símbolo com contraparte vanilla (análogo ao cabeçalho `@prompt`).
3. **Linter.** Adicionar check ao `crystalline-lint` (nova trava `V<n>`) que falha sem marcador onde
   ele é exigido. O script `lab/parity/tools/decompor_so_vanilla.py` do Passo 385 é o **esqueleto**
   deste check (extração + reconciliação fechada já existem).
4. **Ferramenta.** Só depois disto a lente consome o campo `@vanilla` direto — a contagem só-vanilla
   passa a refletir dívida real sem a decomposição heurística do Passo 385.

## Consequências

- **Positivas.** A leitura "o que falta migrar" passa a ser exata e re-rodável; o renome dominante
  fica legível por máquina; a literatura "Sobre paridade" ganha forma regular; o terceiro oráculo
  (registro de construção) fica consumível.
- **Custos.** Semeadura é trabalho distribuído por muitos L0 (mitigado: proposta automática +
  confirmação humana). Mais um marcador no fecho (mitigado: precedente `@prompt`, já interiorizado).
- **Risco.** Marcador semeado errado mente para a ferramenta. Mitigação: a semeadura propõe, o
  humano confirma; o linter verifica forma, não substância (o dono audita a substância — ADR-0108).

## Política "sem novas reservas"

Esta ADR **propõe** uma convenção e o seu roadmap; **não reserva** trabalho fora dele e não
materializa o marcador. A promoção a `EM VIGOR` é decisão do dono após (1) confirmar a convenção e
(2) ver a primeira semeadura. Até lá, `PROPOSTO`.
