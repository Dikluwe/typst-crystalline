# Passo 929 — reduzir custo absoluto de fallback CJK/emoji (repete P928, instrução mais rígida)

**Este prompt foi escrito para ser lido por um chat novo, sem acesso ao histórico dos passos
anteriores. Todo o contexto necessário está aqui dentro ou nos ficheiros indicados abaixo.**

**Aviso sobre o passo anterior (P928): a instrução não foi seguida correctamente.** O prompt de
P928 pedia para medir duas ideias específicas (paralelizar o scan; guardar cache em disco) e
esperar confirmação antes de implementar qualquer uma. Em vez disso, o executor de P928 foi direto
testar uma terceira ideia diferente — "Opção 1" do passo P925 (pré-computar tudo no arranque) —
**que já tinha sido medida por completo no passo P926 e confirmada como regressão grande no caso
comum (1.12×-1.48× mais lento)**. Testar essa mesma ideia de novo, parcialmente, sem comparar com
as duas ideias pedidas, e sem esperar confirmação, foi um desvio da instrução — mesmo que o
resultado tenha sido medido e revertido honestamente. Este passo repete o pedido, com a instrução
ainda mais explícita para não haver dúvida.

---

## Leitura obrigatória antes de começar, nesta ordem

1. `00_nucleo/handoff-novo-chat-p927.md` — estado geral do projecto.
2. `00_nucleo/materialization/typst-passo-925-relatorio.md` — diagnóstico original.
3. `00_nucleo/materialization/typst-passo-926-relatorio.md` — "Opção 1" (pré-computar tudo no
   arranque) já testada por completo, já confirmada como regressão grande no caso comum. **Não
   testar isto de novo, nem em parte.** Também mostra "Opção 5" (thread de fundo) já testada e
   já confirmada como sem ganho.
4. `00_nucleo/materialization/typst-passo-927-relatorio.md` — a correcção que está em vigor hoje
   (scan condicional): zero regressão no caso comum, mas não reduz o custo de quem realmente usa
   CJK/emoji (continua ~7 segundos).
5. `00_nucleo/materialization/typst-passo-928-relatorio.md` — a tentativa mais recente
   (repetição parcial da Opção 1), já revertida. Ler para confirmar que o estado actual do código
   é o mesmo de P927 (a reversão de P928 foi completa).

**Confirmar, antes de continuar**: rodar `git log --oneline -5` e `git status`, confirmar que o
código está no estado descrito no relatório de P928 (pós-reversão), não no estado da tentativa
que foi revertida.

---

## As únicas duas ideias a testar neste passo

**Não testar nenhuma ideia além destas duas. Se surgir uma terceira ideia durante a investigação,
registá-la para decisão futura — não implementar sem passar pela Fase A.1 abaixo.**

### Ideia 1 — ler várias fontes ao mesmo tempo (paralelizar)

Hoje, quando o scan de fallback é necessário (só acontece em documentos com carácter não coberto
pela fonte principal — isso já está resolvido desde P927), as fontes do sistema são lidas uma de
cada vez, em sequência. A ideia é ler várias ao mesmo tempo, usando múltiplas *threads* (linhas de
execução simultâneas em Rust), para tentar reduzir o tempo total do scan quando ele de facto
acontece.

### Ideia 2 — guardar o resultado em disco, para reaproveitar depois

Guardar, num ficheiro, o resultado do scan (quais fontes têm quais caracteres) depois de feito uma
vez. Da próxima vez que o compilador for usado, ler esse ficheiro em vez de repetir o scan —
muito mais rápido, desde que o ficheiro ainda seja válido (fontes do sistema não mudaram desde a
última vez).

---

## Fase A — medir as duas, sem implementar nenhuma ainda

1. Confirmar o estado actual de `candidates_for_char` em `03_infra/src/world.rs` por leitura
   directa (não confiar em número de linha de relatórios antigos).
2. **Ideia 1**: construir um protótipo temporário (não guardar no projecto ainda) que leia as
   fontes em paralelo em vez de sequencialmente, e medir o tempo do scan quando ele dispara
   (documento com carácter CJK, documento com emoji — os mesmos casos de teste usados em
   P925-928). Confirmar quantos núcleos de processador o ambiente tem (`nproc`).
3. **Ideia 2**: construir um protótipo temporário que guarda o resultado num ficheiro e o lê da
   segunda execução em diante. Medir o tempo da primeira execução (deve continuar lento, o custo
   só é evitado depois) e o tempo da segunda execução em diante (deve ficar rápido). Confirmar
   como decidir que o ficheiro guardado ainda é válido (fontes não mudaram) — sem isso, o
   cristalino usaria informação desactualizada depois de o utilizador instalar uma fonte nova.
4. **Obrigatório para as duas**: medir também os 7 documentos de teste padrão (sem CJK/emoji),
   para confirmar que nenhuma das duas ideias reintroduz a regressão que P927 já eliminou —
   mesmo erro cometido em P928 (só mediu os casos CJK/emoji, não os 7 padrão, antes de decidir
   avançar).
5. Registar os números das duas ideias lado a lado — tempo de scan (Ideia 1), tempo de primeira e
   segunda execução (Ideia 2), e o efeito nos 7 documentos padrão para as duas. Não escolher ainda.

## Fase A.1 — apresentar as duas medições e esperar confirmação

**Não escrever nenhum código de produção antes desta etapa estar concluída.** Apresentar os
números da Fase A (as duas ideias, lado a lado) e esperar confirmação sobre qual implementar —
pode ser uma só, as duas juntas, ou nenhuma se os números não compensarem.

## Fase B — implementar só a opção confirmada

1. Se a mudança alterar a forma pública de alguma função ou estrutura usada por outras partes do
   projecto: escrever/actualizar primeiro a documentação em `00_nucleo/prompts/` (chamada "L0"
   neste projecto), só depois escrever o código. Nunca ao contrário — isto já foi quebrado por
   engano duas vezes nesta mesma investigação (passos P927 e, possivelmente, P928 — confirmar).
2. Escrever um teste automatizado que confirme o problema **antes** da correcção (deve falhar),
   só depois escrever a correcção, confirmar que o teste passa depois.
3. Rodar `cargo test --workspace` e reportar os números exactos, separados por parte do projecto
   ("crate") — não só "passou tudo".
4. Medir de novo, com números reais: os casos CJK/emoji (antes/depois) e os 7 documentos padrão
   (antes/depois, comparando com o estado anterior a este passo, não com o Typst original).

## Fase C — confirmar que nada mais piorou

Mesma disciplina dos passos anteriores: comparar **antes/depois deste passo especificamente**, não
contra o Typst original (isso responde a uma pergunta diferente).

## Resultado esperado

- As duas ideias (só estas duas) medidas, números lado a lado, incluindo o efeito nos 7 documentos
  padrão.
- Confirmação do dono sobre qual implementar, **antes** de qualquer código de produção.
- Implementação com teste automatizado, sem regressão no caso comum.
- Números reais mostrando melhoria nos casos CJK/emoji.
- Se mudar algo usado por outras partes do projecto: documentação (`00_nucleo/prompts/`)
  actualizada antes do código, e isso registado explicitamente no relatório final, com os nomes
  exactos dos ficheiros.
- Nenhuma ideia além das duas listadas testada sem passar primeiro pela Fase A.1.
