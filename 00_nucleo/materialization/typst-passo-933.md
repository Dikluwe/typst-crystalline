# Passo 933 — confirmar correção do shaper (P932) e instrumentar o vanilla real para resolver a contradição de custo de arranque

**Precede este passo**: `typst-passo-932-relatorio.md` (shaper deixou de verificar
`face_covers_char` depois do filtro de `candidates_for_char`, ganho medido mas correção do glifo
não confirmada) e `typst-passo-925-relatorio.md` (leitura do código-fonte do vanilla, não
instrumentação do binário real).

**Duas partes independentes, mas a Parte B pode mudar a decisão da Parte A** (se o vanilla se
revelar não fazer parse eager de tudo, a "rede de segurança" que falta em P932 pode ter uma
resposta diferente da que parece à primeira vista).

**Pré-condição de árvore**: `git status`. Confirmar estado de P929-932 (protótipos revertidos,
código de produção = estado P932-lazy).

---

## Parte A — confirmar que o shaper de P932 não produz glifo errado

### O problema

P932 removeu a verificação `face_covers_char` depois do filtro por bitmap de `candidates_for_char`
— passou a confiar que qualquer candidato que sobrevive ao filtro tem mesmo o glifo. P930/931 já
mediram, com números, que esse filtro tem falsos positivos reais e não pequenos: bloco do grego
(`U+03B1`) devolve 866 candidatos por bitmap, a maioria sem o glifo exacto. P932 não confirmou o
que acontece quando o shaper escolhe um desses falsos positivos.

### Fase A.1 — reproduzir um caso de alta taxa de falso positivo

1. Compilar um documento com um carácter grego (`α`, `U+03B1`) ou símbolo comum (não CJK — CJK
   tem taxa baixa de falso positivo, 32 candidatos, não é o caso de risco) usando o binário actual
   (estado P932-lazy).
2. Extrair o PDF resultante e confirmar, via `mutool trace`, qual glifo foi de facto desenhado —
   comparar o `glyph_id` e a fonte de onde veio com o resultado do vanilla real, mesmo carácter,
   mesma fonte primária.
3. Se possível, forçar deliberadamente um caso onde o candidato escolhido pelo bitmap **não** tem
   o glifo exacto (por exemplo, usando `--font-path` para limitar as fontes disponíveis a um
   conjunto onde se saiba, por inspecção da fonte com `fontTools`, que o bitmap do bloco cobre mas
   o glifo exacto não existe) — isto é o teste que realmente prova o caso de falha, não só o caso
   comum que por acaso funciona.

### Fase A.2 — decidir com base no resultado

- **Se o glifo sair errado ou `.notdef`**: isto é um bug real introduzido por P932, não só um L0
  desactualizado. Corrigir antes de sincronizar qualquer L0 — reintroduzir alguma forma de
  verificação exacta, mas de forma barata (ideia a considerar: só verificar exactamente quando o
  bitmap devolver mais que N candidatos, evitando o custo médio mas cobrindo o caso de risco).
- **Se o glifo sair sempre certo**: confirmar por quê — pode haver uma camada de protecção que os
  relatórios anteriores não mencionaram (por exemplo, o shaper pode reordenar candidatos por
  alguma heurística que coloca fontes mais prováveis primeiro, reduzindo a chance prática de um
  falso positivo ser de facto escolhido antes de um candidato correcto). Documentar essa camada
  explicitamente antes de aceitar como "está certo por sorte".
- Só depois desta decisão: sincronizar `shaper.md` com o código real (ou corrigir o código para
  bater com o que o L0 já especifica, conforme a Fase A.2 concluir).

---

## Parte B — instrumentar o vanilla real, não só ler o código-fonte

### A contradição a resolver

P925 leu o código-fonte do vanilla e concluiu que `FontInfo::new` faz parse completo + iteração de
cmap de **todas** as fontes do sistema no arranque (`typst-library/src/text/font/info.rs`). Se isso
for literalmente verdade, o vanilla deveria ter um custo de arranque da mesma ordem de grandeza que
o cristalino mediu para a mesma operação (~756ms a ~1.8s, medido em P926/930/931/932-proto). Mas o
vanilla mede ~0.3s total para o mesmo documento. **Isto nunca foi reconciliado com medição directa
do binário — só com leitura de código.**

### Fase B.1 — instrumentar o binário vanilla real

1. Confirmar se `lab/typst-original/` tem símbolos de debug suficientes para instrumentação, ou
   recompilar localmente com `cargo build` (não `--release`, ou `--release` com
   `debug = true` no perfil) para permitir profiling.
2. Usar `strace -c` (contagem de syscalls, tempo por syscall) e/ou `perf record`/`perf report` no
   binário vanilla real, compilando o mesmo documento de teste (`utf8-cjk.typ` ou `utf8-emoji.typ`
   de P923/925) — confirmar quantos ficheiros de fonte são de facto abertos e lidos, e quanto tempo
   é gasto nisso, medido directamente, não inferido do código-fonte.
3. Confirmar se o vanilla, na prática, carrega **todas** as fontes do sistema em todo processo, ou
   se há alguma forma de scope/lazy/cache entre execuções que a leitura de código não capturou
   (por exemplo: `fontdb` pode ter algum mecanismo de cache próprio não mencionado em P925; ou o
   `typst-cli` pode limitar a descoberta a directórios específicos por defeito, não "todas as
   fontes do sistema" literalmente).
4. Se a contradição persistir mesmo depois de instrumentar (isto é: o vanilla realmente abre e
   parseia ~1100+ fontes e ainda assim é rápido): a diferença tem de estar na **eficiência do
   parse em si** — confirmar se `FontInfo::new`/o parse do vanilla é estruturalmente mais barato
   por fonte do que o do cristalino (`ttf_parser::Face::parse` + `extract_coverage`), por exemplo
   por ler menos tabelas, ou por uma implementação mais eficiente da iteração da cmap.

### Fase B.2 — decidir se há algo a copiar

- Se a Fase B.1 encontrar uma diferença mecânica real e portável (não "pré-computar tudo", que já
  sabemos ser lento no cristalino, mas *como* o vanilla pré-computa, se for isso que faz): registar
  como candidato a passo de implementação futuro, com a mesma disciplina de medição desta frente
  inteira (protótipo temporário, benchmark canônico completo antes de decidir).
- Se a Fase B.1 confirmar que o vanilla não faz o que P925 presumiu (por exemplo, se só carrega
  fontes relevantes ao documento, não todas as do sistema): isto muda a pergunta inteira — o
  cristalino pode estar a resolver um problema mais difícil do que o vanilla resolve (fallback
  contra todas as fontes do sistema vs. um conjunto mais restrito), o que mudaria a régua de
  comparação usada em toda esta frente (P925-932). Registar isso como achado importante,
  independentemente de haver ou não implementação a seguir.

---

## Resultado esperado

- Parte A: confirmação com número/prova de que o shaper de P932 produz o glifo correcto mesmo em
  scripts de alta taxa de falso positivo, ou achado de bug real corrigido antes de qualquer L0 ser
  sincronizado.
- Parte B: a contradição entre "vanilla faz parse eager de tudo" (leitura de código, P925) e
  "vanilla é rápido" (medição, ~0.3s) resolvida com instrumentação directa do binário real — não
  mais uma suposição por leitura de código-fonte sozinha.
- Se Parte B encontrar algo genuinamente portável: candidato a novo passo de implementação,
  registado com a mesma disciplina de protótipo+medição já estabelecida.
- L0 de `shaper.md` sincronizado só depois da Parte A estar resolvida, na direcção certa (código
  correcto e L0 batendo com ele, não o contrário).
