# Passo 981 — `"lr("` vaza como texto literal no PDF (seção 22, bug real de produção, não do oráculo)

**Precede este passo**: item catalogado desde `P944 §8.3.1`, nunca corrigido, agora confirmado
pela auditoria externa (2026-08-06) presente em **todas** as versões já testadas, desde a rodada 1
(2026-08-03) — a função `lr(...)` (redimensionamento de delimitador conforme o conteúdo) deixa o
texto literal da própria chamada (`"lr("`) vazar para o PDF renderizado, em vez de ser só
interpretada. `lr(())` aparece como `lr(())`, não como `()`. O vanilla nunca tem esse texto.

**Isto é bug de produção, não item de oráculo** — afeta qualquer usuário que use `lr()`, com ou
sem comparação com o vanilla. Vai para a saída principal (`stream.rs`/`eval`, o caminho normal),
não para `03_infra/src/export/oracle.rs`.

**Pré-condição de árvore**: `git status`. Confirmar P980 presente.

---

## Fase A — confirmar a causa

1. Reproduzir isoladamente: `$ lr((a/b)) $` — confirmar que o texto "lr(" aparece no PDF.
2. Ler o caminho de avaliação de `lr(...)` (`eval/math.rs` ou onde as funções nativas de
   delimitador foram implementadas, P899/P906) — confirmar se `lr` está a ser tratado como
   identificador desconhecido que cai em `Content::MathIdent`/`MathText` literal (mesma classe de
   bug já vista em P958 — nomes gregos com `Nome(args)` caindo em fallback literal antes da
   correção), em vez de ser reconhecido como a função nativa de redimensionamento.
3. Confirmar se `lr` está de facto implementado em algum lugar (o comportamento de
   redimensionamento parece funcionar — a auditoria não reportou delimitador do tamanho errado na
   seção 22, só o texto extra) — se o redimensionamento funciona mas o texto vaza, a causa é mais
   estreita: só o nome da função está a ser emitido por engano, não a função inteira a falhar.
4. Comparar com o mecanismo do vanilla para a mesma sintaxe.

## Fase B — Implementação (TDD directo se for correção pontual no caminho de avaliação)

1. Teste com `lr(())`, `lr([])`, `lr({})` e os outros delimitadores da seção 22 — confirmar que o
   texto "lr(" não aparece no PDF, e que o redimensionamento continua a funcionar (não regredir o
   que já funciona).
2. Implementar.
3. Suíte completa verde, discriminada por crate.
4. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Recompilar o documento de 30 secções, seção 22 — confirmar ausência do texto "lr(" nos oito
   casos.
2. `compare.py`/comparação por sequência de caractere (método mais robusto, per a nota
   metodológica da auditoria) — confirmar zero divergência de conteúdo na seção 22.
3. Benchmark completo, 7 cenários canónicos, `depois/antes`, zero regressão.

## Resultado esperado

- Texto "lr(" deixa de vazer para o PDF, em todos os oito delimitadores da seção 22.
- Redimensionamento de delimitador (a funcionalidade real de `lr()`) preservado.
- Benchmark sem regressão.
