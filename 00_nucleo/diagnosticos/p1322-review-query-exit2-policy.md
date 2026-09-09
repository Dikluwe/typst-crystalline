# P1322 — distinguir rejeição pública de falha opaca de execução

Medição anterior à decisão: `p1322-transversal-r2.json` (SHA-256 `92be1cf09282fc96354a68deadaed4210fb78aa75b09bb1ef0f488efd6a6a5b3`) contém `P1137-I-001` nos perfis html/a11y/html+a11y. Ambos os argv usam a mesma fonte pinada, selector `heading`, formato JSON e o conjunto exato de features. O vanilla termina com exit 0, JSON e warning de depreciação integral; o cristalino termina com exit 2, stdout vazio e:

```text
error: unexpected argument '--features' found

  tip: to pass '--features' as a value, use '-- --features'

Usage: typst query --format <FORMAT> <INPUT> <SELECTOR>

For more information, try '--help'.
```

Conferência independente de fonte atual: `02_shell/src/cli.rs:313` define `QueryArgs` sem features; `:411` define `QueryIntent` também sem esse campo; `:515` transporta somente os campos existentes. O parser é clap, e a rejeição corresponde ao seu contrato público de argumentos. Não é evidência de crash, binário trocado, fixture errada, timeout ou saída não interpretável. O caminho de eval/compile já possui features; o caso query compara a disponibilidade pública dessa opção, não o valor interno de query depois de uma opção rejeitada.

Decisão de método D: este diagnóstico completo, com a identidade e o argv já autenticados, é `PUBLIC_CLI_ABSENT`/`DIFFERENCE` no observável capability. Não é MATCH de query, nem prova do resultado JSON no cristalino nesse perfil. O resultado JSON permanece não disponível precisamente porque a capacidade CLI falta. Analogamente à classificação VANILLA_ONLY por rejeição de membro, a ausência de um valor devido a uma rejeição pública interpretada não é execução opaca.

A regra do checker será estrita e focal: somente o exit 2 de query, com stdout vazio, a mensagem de opção `--features` rejeitada e Usage de query completos, e argv query contendo a opção, será reconhecido como esta rejeição pública. Exit 2 genérico, mensagem incompleta, outro comando, stdout opaco ou outro código desconhecido permanecem Unknown. O teste focal deve preservar estes negativos e não converter nenhum deles em sucesso. Esta extensão do parser de evidência não modifica o critério de igualdade: a célula continua divergente e a feature não é declarada implementada.

O plano congelado continua íntegro; esta política é recibo sucessor para a observação nova provocada pela correção real do adapter de perfil. Nenhum produto/L0/oráculo bilateral mudou. O próximo lote não pode ganhar prioridade ou owner único por esta leitura: a implementação de features em QueryIntent seria contrato público futuro com owner completo ainda não demonstrado.
