# Prompt — typst-passo-799 (achado P798 #13): `math::attach` — sub/superscript quebrado (prioridade alta)

**Origem**: P798 (lote 3 de triagem em lote, corrigido), tabela "Achados de P798, aguardando passo dedicado"
**Handoff**: `00_nucleo/handoff-novo-chat-p798.md`
**Módulo afectado**: `math::attach`
**Estado**: aguardando execução, ainda não corrigido

**Prioridade**: alta, marcada como tal no handoff — sub/superscript é uso comum em qualquer documento com matemática. O handoff recomenda tratar este achado antes de continuar a triagem de novos módulos.

---

## Achado (texto exacto do handoff)

> Posicionamento de sub/superscript com `_`/`^` completamente quebrado (`∫10x23` vs render correcto com bounds) — parece grave, candidato a prioridade alta

---

## Possíveis sobreposições a confirmar antes de corrigir (obrigatório, ver handoff §"Recomendação para o próximo chat")

1. **Achado #7 de P798 (typst-passo-800)** (`syntax::kind`): `#if true [Hello $x^2$]` tem ordem/conteúdo de output completamente diferente do vanilla. Pode ser o mesmo mecanismo de `attach` a falhar dentro de um `#if`. Confirmar antes de tratar como dois achados separados — se for a mesma causa, um único passo corrige os dois.
2. **Observação de P786 §7** (nunca virou passo): itálico matemático extrai texto plano em vez de estilizado (`αβ` vs `𝛼𝛽`). Pode sobrepor-se a este achado se a causa-raiz for a mesma rotina de renderização de conteúdo matemático. Confirmar antes de tratar como dois achados separados.

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" ou "mecanicamente correto" sem execução mostrada. Cada afirmação do relatório deste passo tem de vir acompanhada do comando exacto e da saída literal, comparando vanilla (`lab/typst-original/target/release/typst`) e cristalino (`./target/release/typst`). A contagem de testes da suíte `typst-core` tem de aparecer no relatório e bater com o número de testes novos declarados.

---

## Passo 1 — Sonda (obrigatória antes de qualquer alteração de código)

1. Compilar `$integral_1^0 x^23$` (ou a expressão exacta que produziu `∫10x23` no achado original — confirmar a fonte `.typ` usada em P798 antes de assumir) com os dois binários. Registar a saída literal de cada um.
2. Testar casos isolados adicionais: `$x^2$` sozinho, `$x_1$` sozinho, e a combinação `$x_1^2$`, para determinar se o problema é geral a qualquer `attach` ou específico da combinação sub+superscript.
3. Testar o caso do achado #7 (typst-passo-800) (`#if true [Hello $x^2$]`) e o caso de P786 §7 (itálico matemático, ex.: `$alpha beta$` estilizado) nos dois binários, para confirmar ou refutar as sobreposições listadas acima.
4. Localizar no código-fonte do vanilla (`lab/typst-original/`) o mecanismo de `attach` (sub/superscript) em modo matemático — como calcula posição, bounds e escala do script em relação ao núcleo.
5. Localizar no código do cristalino o mecanismo equivalente e identificar o ponto exacto da divergência (posicionamento, ordem de composição, ou ausência de bounds).
6. Registar os pontos exactos (vanilla e cristalino) no relatório antes de tocar em código, incluindo a confirmação ou refutação de cada sobreposição do Passo 1.3.

## Passo 2 — Implementação

Corrigir o mecanismo de `attach` no cristalino para calcular posição e bounds de sub/superscript replicando o vanilla. Se as sobreposições do Passo 1.3 forem confirmadas, a correcção deste passo deve fechar também o achado #7 (typst-passo-800) e/ou a observação de P786 §7 — documentar isso explicitamente no relatório, sem abrir passos separados para o mesmo mecanismo.

## Passo 3 — Validação

1. Recompilar o cristalino.
2. Repetir todos os comandos do Passo 1 (incluindo os casos isolados e os de sobreposição) e mostrar a saída literal de cada um, agora igual à do vanilla onde aplicável.
3. Adicionar casos de teste cobrindo subscript sozinho, superscript sozinho, e a combinação, incluindo pelo menos um caso com núcleo multi-caractere (o `10x23` do achado sugere que o problema pode envolver números multi-dígito).
4. Correr a suíte `typst-core` completa e mostrar o comando e a contagem de testes antes/depois.

## Passo 4 — Relatório

Produzir `00_nucleo/materialization/typst-passo-799-relatorio.md` com:
- Comando + saída literal do Passo 1 (antes da correcção), incluindo os três testes de sobreposição.
- Confirmação explícita, para cada uma das duas sobreposições, se foi a mesma causa ou não.
- Trecho do código vanilla e do código cristalino identificados no Passo 1.
- Diff da correcção.
- Comando + saída literal do Passo 3 (depois da correcção).
- Contagem de testes antes/depois.
- Se este passo fechou também o achado #7 (typst-passo-800) e/ou a observação de P786 §7, actualizar o handoff (`00_nucleo/handoff-novo-chat-p798.md`) removendo-os da lista de pendências.
