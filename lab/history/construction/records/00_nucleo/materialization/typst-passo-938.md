# Passo 938 — coverage exata, mas extraída lazy (combinar P927 + P937)

**Precede este passo**: `typst-passo-937-relatorio.md` — coverage exata (runs de codepoints, sem
falsos positivos) implementada, fallback pesado melhorou muito (CJK 6.4×→4.0×, emoji 22×→5.7×
distância ao vanilla real), **mas o caso comum regrediu ~5.5× (88ms→480ms)** porque a extração
ficou eager para todas as ~1086 fontes no arranque, mesmo em documentos que nunca precisam de
fallback. O próprio relatório de P937 (seções 6.3 e 7) já aponta a solução: manter a
representação exata, extrair lazy.

**Antes de qualquer coisa**: confirmar com o dono se o estado de P937 foi commitado ou fica em
working tree — se commitado, este passo parte dele; se não, confirmar que ainda está presente
antes de continuar.

**Pré-condição de árvore**: `git status`.

---

## A ideia — reunir o que já funciona, cada peça já validada separadamente

1. **De P927**: não disparar nenhum scan caro se o documento não tiver carácter fora da cobertura
   da fonte primária/embutida (scan do source bruto, já implementado e validado, zero regressão
   no caso comum).
2. **De P937**: quando o scan **de facto** dispara, a coverage extraída é exata (runs de
   codepoints), não aproximada por bloco — elimina os falsos positivos que forçavam abertura de
   faces extra durante o shaping.

A combinação: manter o gatilho condicional de P927 (só entra na Fase de fallback se o source tiver
carácter não coberto), mas quando entrar, extrair a coverage **exata** por fonte candidata, de
forma lazy (por fonte, à medida que é consultada, não todas as ~1086 de uma vez) — usando o mmap já
implementado em P937 para tornar cada extração individual barata.

## Fase A — confirmar que isto é possível sem reintroduzir o problema de P933-fixed

0. **Investigar a diferença residual de eficiência, independente de eager/lazy.** P937 mediu que,
   abrindo o mesmo número de arquivos (~2200, `strace`), o cristalino levou 0.73s contra 0.57s do
   vanilla — uma diferença de ~150-200ms que sobra mesmo quando o mecanismo de I/O é idêntico
   (mmap dos dois lados). Isto significa que há um custo residual **por fonte processada**, não
   só um custo de "quando" acessar os bytes. Antes de implementar lazy (que só reduz *quantas*
   fontes são processadas no caso comum, não *quanto custa* processar cada uma), perfilar onde
   esse tempo vai:
   - Usar `perf record`/`perf report` (ou instrumentação manual com `std::time::Instant` em
     pontos específicos, revertida depois) no cristalino e no vanilla, processando o **mesmo**
     conjunto de fontes, medindo tempo de CPU gasto em: descoberta/indexação inicial (antes da
     extração de coverage), construção de `Coverage::from_codepoints` por fonte, e qualquer outro
     passo que aconteça por fonte durante o arranque.
   - Confirmar se a descoberta inicial do cristalino usa `fontdb` com parse parcial
     (`RawFace`, mais barato, per P925) ou se já faz parse mais completo que o necessário só para
     indexar — se for isto, é uma correção separada e mais simples que mudar eager/lazy.
   - Confirmar se a construção da estrutura de runs de `Coverage` no cristalino tem alguma
     ineficiência óbvia (realocações repetidas, cópias desnecessárias) comparada com a
     implementação do vanilla, lida linha a linha, não só "porta directa" presumida.
   - Registar o resultado desta investigação **antes** de decidir se a Fase B deste passo
     (lazy) sozinha é suficiente, ou se há uma segunda correção (eficiência por-fonte) a fazer
     junto ou depois.

1. Confirmar que a extracção de coverage exata **por fonte individual** (não para todas de uma
   vez) é rápida o suficiente para não recriar o custo por-candidato que P933-fixed tinha — a
   diferença chave é que aqui a extracção é exata desde a primeira vez (sem precisar de
   verificação adicional depois), enquanto P933-fixed tinha um filtro aproximado seguido de
   verificação cara. Confirmar com número: tempo de extrair coverage exata de **uma** fonte via
   mmap (não das 1086 juntas).
2. Confirmar onde cachear o resultado por fonte — uma vez extraída a coverage exata de uma fonte
   candidata, guardar para não repetir se o mesmo documento (ou outro carácter do mesmo fallback)
   consultar essa fonte de novo.
3. Confirmar se o scan condicional de P927 (varredura do source bruto) precisa de ajuste para
   funcionar com a nova representação de coverage — ou se continua igual, só trocando o que
   acontece depois de disparar.

## Fase B — Implementação (TDD directo se for só reconectar mecanismos já validados
separadamente; protocolo de dois agentes se a integração revelar necessidade de mudança
estrutural nova, não só reconexão)

1. Teste com medição real (contagem de aberturas de fonte, mesmo padrão de sempre): documento
   latino puro não deve extrair coverage de nenhuma fonte além da primária/embutida (mesmo
   comportamento de P927); documento CJK/emoji deve extrair coverage exata só das fontes
   candidatas consultadas, não das ~1086 do sistema inteiro.
2. Implementar.
3. Suíte completa verde, discriminada por crate.
4. `cargo run -- .` — zero violations.

## Fase C — Medição completa (attestation, `L11`, `--min-runs 10`+, vanilla confirmado por string)

1. 7 cenários canônicos, `depois/antes` contra P933-fixed **e** contra P937 — confirmar que o
   caso comum volta a ~88-100ms (não os ~480ms de P937), sem perder o ganho de fallback de P937.
2. 5 casos UTF-8, `depois/antes` e `cristalino/vanilla-real` — confirmar que a melhoria de
   fallback de P937 (CJK 4.0×, emoji 5.7×) se mantém ou melhora.
3. `strace`/contagem de aberturas de fonte, mesmo formato de P936/937 — confirmar que o caso
   comum não abre nenhuma fonte extra, e o caso de fallback abre só as fontes de facto candidatas,
   não todas as 1086.

## Resultado esperado

- Caso comum de volta à banda de P933-fixed (ou melhor), sem a regressão de 5.5× de P937.
- Ganho de fallback de P937 preservado ou melhorado.
- Contagem de aberturas de fonte confirmando o comportamento condicional (zero no caso comum,
  só candidatos reais no caso de fallback).
- Se ainda sobrar distância ao vanilla real depois disto: registar quanto, não forçar fechamento
  total.
