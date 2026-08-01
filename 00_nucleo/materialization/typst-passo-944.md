# Passo 944 — regressão visual confirmada em matrizes e `lim`: causada pela reescrita de `covering()`/`Coverage` (P937-942)?

**Precede este passo**: confirmação visual direta pelo dono, dois PDFs (`test_crystalline.pdf`/
`test_vanilla.pdf`, mesmo `.typ` de 30 seções usado desde P894) comparados lado a lado nesta
conversa. **Dois defeitos confirmados visualmente, não por texto extraído**:

1. **Seção 4, segundo `lim` (`lim_(x→∞) 1/x = 0`)**: sobreposição real de glifos — `x→∞` colide
   com conteúdo adjacente, não fica limpo como subscrito abaixo de `lim`. O primeiro `lim`
   (`x→0`) está correto.
2. **Seções 5/21 (matrizes)**: o parêntese/colchete/chave não estica para cobrir a matriz inteira
   com folga simétrica como o vanilla — fica curto/apertado, colado à última linha em vez de
   abraçar todas as linhas.

**Suspeita a confirmar, não fato**: este `.typ` não foi reverificado desde antes da frente de
performance de fontes (P925-943), que reescreveu por completo `Coverage`/`covering()` (P937:
coverage exata; P938: extração lazy; P942: removida a reverificação redundante de `glyph_index`).
`stretchy.rs` (escala de delimitador) e `attach.rs`/`font_metrics` (kerning, `text_ink_bounds`)
dependem desse mesmo mecanismo. Nenhum teste da frente de performance (focados em CJK/emoji/
latim/grego) exercitava matriz ou `lim` — uma regressão aí não seria pega por nenhum benchmark já
rodado.

**Pré-condição de árvore**: `git status`. Confirmar estado P943 presente.

---

## Fase A — confirmar se é regressão nova, e desde quando

1. **Bisecção por binário**: se ainda existirem binários preservados de checkpoints da frente de
   geometria (fim de P924, antes de P925 começar) e da frente de performance (P937, P938, P942),
   recompilar o mesmo `.typ` de 30 seções em cada um e comparar visualmente as seções 4 e 5/21 —
   confirmar em qual passo exato a regressão apareceu. Se os binários não existirem mais,
   recompilar a partir dos commits correspondentes (`git checkout <commit>` num worktree
   temporário, mesma técnica já usada em P909/918/936).
2. Se a regressão for confirmada como tendo começado durante P937-942: isso aponta directamente
   para a reescrita de `Coverage`/`covering()` como causa — confirmar especificamente:
   - Para a matriz: se `layout_stretchy_delimiter` (`stretchy.rs`) está a escolher uma variante
     diferente (ou nenhuma, caindo em fallback de tamanho fixo) depois das mudanças de P937/938/
     942 — instrumentar e comparar a variante escolhida antes/depois.
   - Para o `lim`: se `math_kern`/`text_ink_bounds` (consumidos por `attach.rs` para posicionar
     `x→∞` como limite) estão a receber métricas de uma fonte diferente da que recebiam antes —
     `covering()` mudou de comportamento (coverage exata, sem reverificação de `glyph_index`) e
     isso pode ter mudado qual fonte é escolhida para glifos específicos (`→`, por exemplo).
3. Se a regressão **não** for nova (já existia antes de P925, só nunca foi vista porque o `.typ`
   de 30 seções não foi revisitado depois de P918/919): registar isso explicitamente — muda a
   causa provável (não é a reescrita de performance, é algo mais antigo nunca fechado).

## Fase B — corrigir a causa confirmada (só depois da Fase A)

Protocolo de dois agentes (mesmo de sempre para geometria/font_metrics), TDD, gate se mudar
contrato público. Não implementar sem a Fase A confirmar a causa exata — não presumir que é
`covering()` só porque é a suspeita mais provável.

## Fase C — revalidar o `.typ` de 30 seções inteiro, não só os dois casos confirmados

Depois da correção, recompilar o documento completo e conferir visualmente (não só as duas seções
que motivaram este passo) — outras seções podem ter o mesmo tipo de regressão silenciosa, ainda
não notada. Comparar com o vanilla real seção a seção onde houver dúvida.

## Resultado esperado

- Confirmação de quando a regressão começou (bisecção), não suposição.
- Causa exacta confirmada (mudança em `covering()`/`Coverage`, ou outra coisa).
- Correção implementada, com prova geométrica (`mutool trace`/visual) para os dois casos
  confirmados.
- Revalidação visual do documento de 30 seções inteiro, registando qualquer achado novo
  encontrado durante essa revisão, não só os dois já conhecidos.
- **Lição de processo a registar**: o `.typ` de 30 seções deveria ter sido revalidado depois de
  qualquer passo que mexesse em `font_metrics`/`covering()`/`Coverage`, mesmo em passos
  classificados como "performance" — a frente de P925-943 nunca rodou esse documento específico,
  só os benchmarks canónicos e os casos UTF-8 isolados.
