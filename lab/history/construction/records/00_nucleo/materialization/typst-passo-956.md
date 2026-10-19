# Passo 956 — emendar ADR-0126 (inversão verboso/compacto) e construir o modo verboso como novo padrão de produção (compacto vira flag opcional)

**Precede este passo**: correção do dono sobre `ADR-0126` — o texto actual descreve "modo verboso"
como "um `BT…ET` por item" (o formato *já existente* desde o Passo 20, per P955), quando na
verdade esse formato é o mais **enxuto** dos dois comparado ao vanilla. A `ADR-0126` inverteu os
rótulos. Correcção: **"verboso" = espelhar a semântica do vanilla** (`Tm`, `q`/`cm`/`Q` por bloco,
`cs`/`scn` repetido, `Tr` explícito — a construir); **"compacto" = o formato actual** (já existe
desde o Passo 20).

**Correcção adicional do dono sobre o destino final dos dois modos**: o modo verboso **não é só
ferramenta de diagnóstico descartável** — depois de implementado com paridade ao vanilla, **vira o
caminho de produção padrão**. O modo compacto **deixa de ser o único caminho** e passa a ser uma
**flag opcional** (para quem quer PDF menor, aceitando menos semântica/riqueza de estrutura),
validada por decalque contra o modo verboso (agora o padrão), não descartada.

**Pré-condição de árvore**: `git status`. Confirmar `ADR-0126`/P955 presentes.

---

## Fase 0 — emendar `ADR-0126` (não reescrever, corrigir)

1. Corrigir a secção 1 da ADR: trocar a descrição de "modo verboso" para "novo modo, espelhando a
   semântica do vanilla operador a operador (`Tm` em vez de `Td`, `q`/`cm`/`Q` a isolar cada bloco
   de texto, `cs`/`scn` declarado por transição de cor, `Tr` explícito) — **torna-se o caminho de
   produção padrão** depois de validado"; e "modo compacto" para "o formato actual do exportador,
   existente desde o Passo 20 (P955) — **passa a flag opcional** para PDFs menores, validada por
   decalque contra o modo verboso, não removida".
2. Corrigir a ordem de validação: o modo verboso é validado **directamente contra o vanilla**
   (comparação operador a operador). O modo compacto, depois, é validado **por decalque contra o
   modo verboso** (agora o padrão) — mesma lógica de antes, só a hierarquia dos dois invertida:
   verboso não é mais o meio, é o fim; compacto não é mais o fim, é a opção.
3. Registar a correcção como emenda datada, preservando o texto original (mesmo padrão desta
   conversa inteira).

## Fase A — desenhar o modo verboso como novo padrão (gate obrigatório — mudança de contrato
público real: muda o comportamento por defeito do compilador)

1. Ler o `stream.rs`/`builder.rs` actual (Passo 20 em diante) e mapear, para cada operador que o
   vanilla usa e o cristalino não usa hoje (`Tm`, `q`/`cm`/`Q` por bloco, `cs`/`scn` repetido,
   `Tr`), onde exactamente ele entraria na função de emissão actual.
2. **Confirmado pelo dono**: o modo verboso não é modo alternativo temporário — depois de validado,
   substitui o comportamento por defeito. Desenhar a implementação já com isso em mente: o código
   do modo verboso é o candidato a ficar como está para sempre, não um protótipo a descartar.
3. Desenhar a flag do modo compacto (nome de CLI/API, comportamento por defeito quando ausente —
   verboso — e o que a flag activa exactamente). Confirmar se isto é uma mudança de interface
   pública do cristalino (linha de comando/API) que precisa de documentação própria, não só código.
4. Editar L0s afectados, sincronizar hashes, **parar para confirmação do dono antes da Fase B** —
   este gate é mais importante que o normal, porque muda o comportamento por defeito de todo
   documento compilado, não é feature nova opcional.

## Fase B — Implementação (protocolo de dois agentes de P898 — mudança de comportamento por
defeito, risco alto: afeta todo documento compilado sem flag nenhuma)

1. Agente A escreve testes cobrindo: `Tm` produzindo a mesma posição final que `Td` produzia
   (mesma matriz resultante); `q`/`cm`/`Q` isolando correctamente cada bloco (não vazando
   transformação para o bloco seguinte); `cs`/`scn` emitido na transição de cor; `Tr` emitido
   explicitamente; **e** que a flag do modo compacto, quando activada, continua a produzir o
   formato actual (Passo 20) inalterado — a mudança de padrão não pode quebrar quem já depende do
   formato compacto explicitamente.
2. Agente B implementa: modo verboso como caminho por defeito; formato actual movido para trás da
   flag de modo compacto, sem alteração de comportamento quando essa flag está activa.
3. Suíte completa verde, discriminada por crate — testes que hoje verificam o formato Passo 20 sem
   especificar modo precisam de decidir explicitamente qual modo esperam (verboso, o novo padrão,
   ou compacto, com a flag) — não deixar teste ambíguo sobre qual comportamento está a verificar.
4. `cargo run -- .` — zero violations.

## Fase C — Validar o modo verboso (agora padrão) directamente contra o vanilla

1. Recompilar o `.typ` de 30 secções, sem flag (modo verboso, o novo padrão), e comparar o content
   stream operador a operador contra o vanilla — comparação directa, sem heurística de
   emparelhamento estrutural complexa.
2. Rodar `compare.py` (P948/951) no par modo-verboso vs vanilla — usar a classificação
   `sistemático`/`pontual` para achar qualquer divergência geométrica real ainda não corrigida por
   P944-953. Corrigir as causas encontradas na camada de layout (não no exportador) — isso corrige
   o resultado visual dos dois modos ao mesmo tempo, já que ambos partilham o mesmo cálculo de
   posição a montante.
3. Benchmark do modo verboso (o novo padrão) contra os 7 cenários canónicos, `depois/antes` do
   estado pré-P956 — desta vez a comparação importa de verdade, porque é o caminho que todo
   utilizador vai usar por defeito.

## Fase D — validar o modo compacto (agora flag) por decalque contra o modo verboso

1. Com flag activada, recompilar o mesmo `.typ` e confirmar, via `compare.py`, que as posições dos
   glifos batem com o modo verboso (agora a fonte de verdade) — não precisa de bater operador a
   operador (é compacto de propósito), só precisa de produzir o mesmo resultado visual.
2. Confirmar o ganho de tamanho de arquivo que a flag compacta ainda oferece (é a razão de ela
   continuar a existir) — medir e documentar, para quem for decidir usar a flag saber a troca
   exacta (tamanho menor vs menos estrutura semântica).
3. Documentar a flag (nome, comportamento, quando usar) — isto é interface pública nova, precisa
   de ficar visível para quem usa o cristalino, não só registada internamente.

## Resultado esperado

- `ADR-0126` emendada: verboso = padrão de produção; compacto = flag opcional, validada por
  decalque.
- Modo verboso implementado, validado directamente contra o vanilla, virando o comportamento por
  defeito do compilador.
- Modo compacto preservado atrás de uma flag documentada, validado por decalque contra o verboso,
  não descartado.
- Qualquer divergência geométrica real encontrada durante a validação corrigida na camada de
  layout, beneficiando os dois modos.
- Benchmark do novo padrão (verboso) contra os cenários canónicos — desta vez o resultado importa
  para todo utilizador, não só para a investigação.
