# ADR-0127 — Gate de L0: paragem obrigatória só para contrato/comportamento; fluxo contínuo para correções internas

**Estado:** `EM VIGOR`
**Data:** 2026-08-04
**Decisor:** dono (resposta a pergunta sobre P962-964 — a distinção já era
praticada; ele aceitou-a como válida e pediu que virasse regra escrita).
**Registada no Passo 965.**

---

## 1. Contexto

A "Regra de Ouro" (`CLAUDE.md`, Protocolo de Nucleação) diz: a IA redige o L0
e **PARA**, só prosseguindo quando o humano confirmar. Na prática
(P893–P964), a paragem real só aconteceu em dois tipos de situação, e as
correções internas seguiram em fluxo contínuo (L0 editado primeiro, sem
parar). A redacção original não distinguia os dois modos — e foi essa
ambiguidade que permitiu o desvio de P952 (campos novos em `MathConstants` +
método novo em `FontMetrics` entraram **sem** paragem, corrigido por revisão
retroativa). Esta ADR formaliza o critério para que não dependa de
julgamento caso a caso.

## 2. Decisão — o critério

### PARAGEM OBRIGATÓRIA antes de escrever código (confirmar com o dono)

1. **Contrato público**: adicionar, remover ou alterar campo público de
   qualquer entidade (`entities/` — ex.: `MathConstants`, `GlyphVariant`);
   adicionar/alterar método de qualquer trait público (ex.: `FontMetrics`);
   alterar assinatura pública de função L1–L4.
2. **Comportamento por defeito do produto**: novo modo padrão, nova flag,
   troca de caminho por defeito do compilador (ex.: P956 — modo verbose como
   padrão de emissão), ou qualquer mudança deliberada de superfície
   utilizador-face. **Não** entram aqui correções de paridade (ponto 3
   abaixo).
3. **Mudança arquitectural de fase/ordem de pipeline** — mover lógica entre
   fases (ex.: eval ↔ layout, o tipo de decisão de P966), mesmo que a
   implementação final seja pequena.
4. **Compatibilidade**: qualquer mudança que, se revertida depois, quebraria
   código já escrito contra ela (binário/API).

### FLUXO CONTÍNUO (L0 editado primeiro + resselo de hash; SEM paragem)

1. Correção de fórmula/valor dentro de função existente, sem mudar a sua
   assinatura (P957, P959-fórmula, P963).
2. Adição/correção de entradas em tabelas de mapeamento — símbolos, nomes,
   shorthand (P958, P962, P964).
3. **Correções de paridade com o vanilla em geral**, mesmo quando mudam o
   output renderizado (P958: `Gamma(z)` → Γ(𝑧); P961: legenda reduzida) —
   o gate destas é o teste RED→GREEN + revalidação (visual/`compare.py`/
   benchmark), não a confirmação prévia. São o trabalho diário da frente;
   exigir paragem para cada uma paralisaria o fluxo sem ganho de revisão.
4. Qualquer mudança cujo "desfazer" não quebra nada fora do próprio módulo.

## 3. Verificação contra os passos executados (Fase A de P965)

| Passo | Tipo | Modo aplicado | Correcto? |
|---|---|---|---|
| P893/896/906/909/915/918/922/937 | contrato (campos/métodos) | paragem | ✓ |
| P956 | comportamento por defeito + assinaturas públicas | paragem | ✓ |
| P959 | contrato (2 campos `MathConstants`) | paragem | ✓ |
| P952 | contrato (campo + método de trait) | **sem paragem — desvio** | ✗ (corrigido por revisão retroativa) |
| P957 | fórmula interna | fluxo | ✓ |
| P958 | tabela + braço de fallback (paridade) | fluxo | ✓ |
| P961 | estilo de anotação (paridade) | fluxo | ✓ |
| P962 | registo com wrapper (paridade) | fluxo | ✓ |
| P963 | fórmula interna (`is_text_like`) | fluxo | ✓ |
| P964 | tabela (codepoints gregos) | fluxo | ✓ |

O critério separa limpo todos os casos executados — não ficou nenhum caso de
fronteira por cobrir. Caso de fronteira discutido e fechado na redacção:
correção de paridade que muda output visível (P958) parecia colidir com
"comportamento por defeito" — a distinção é **intenção de produto vs
convergência para o vanilla**: a primeira para; a segunda flui.

## 4. Não-colisão (varredura real de `00_nucleo/adr/`)

Varredura por `paragem|confirma|gate|PARAR`: nenhuma ADR existente decide
quando parar após editar L0. ADR-0114/0117 (sonda antes da spec) são de
método de investigação — **complementares**, não colidem. Número confirmado
por listagem real: último ficheiro `typst-adr-0126-*` → **slot livre =
0127**.

## 5. Consequências

- **Positiva:** a Regra de Ouro deixa de ser ambígua; o critério é
  verificável (assinatura pública? comportamento por defeito? fase do
  pipeline? — se nenhum, flui).
- **Positiva:** o desvio de P952 fica tipificado como violação do ponto 1 —
  detectável por inspecção do diff (campos/traits novos sem paragem
  registada).
- **Neutra/custo:** em caso de dúvida sobre a classe da mudança, a regra de
  bolso é **parar** — a paragem errada custa uma mensagem; a paragem em
  falta custa uma revisão retroativa (lição de P952).
- **Operacional:** a paragem regista-se no relatório do passo ("gate:
  confirmado pelo dono em <data>"), como já se fazia em P956/P959.

## 6. Referências

- `00_nucleo/materialization/typst-passo-965.md` (enunciado + proposta de
  critério do dono).
- `00_nucleo/diagnosticos/typst-passo-952-relatorio.md` §6 (o desvio que
  motivou a formalização).
- ADR-0114/0117 (sonda antes da spec — método), ADR-0108 (medir antes de
  decidir), ADR-0121 (proveniência de medições).
